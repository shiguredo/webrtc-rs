# whip/whep の接続処理にタイムアウトを設ける

- Priority: Medium
- Polished: 2026-06-05
- Created: 2026-06-05
- Model: Opus 4.8

## 目的

whip / whep の接続処理にタイムアウトを導入し、相手が応答しない場合でも一定時間で打ち切れるようにする。現状は接続完了の待機とシグナリングの同期 I/O がいずれもタイムアウトを持たず、応答がないと永久にハングする。これを修正する。

## 優先度根拠

Medium とする。応答がない相手やネットワーク断の状況で、接続待機スレッドやシグナリングスレッドが無期限にブロックし、プロセスが復帰できなくなる。クラッシュやメモリ破壊ではないため High ではないが、運用上ハングは深刻で、タイムアウトの欠如は明確な実装上の欠陥であるため Medium とする。

## 現状

### whip.c の接続待機 (`webrtc/src/whip.c`)

`SignalingWhip_WaitForConnect` は `pthread_cond_wait` をタイムアウトなしで使用しており、`state` が `SIGNALLING_WHIP_STATE_CONNECTING` から変わるまで無期限に待つ (`webrtc/src/whip.c:1504-1513`)。

```c
int SignalingWhip_WaitForConnect(struct SignalingWhip* self) {
  RTC_LOG_INFO("SignalingWhip_WaitForConnect");
  pthread_mutex_lock(&self->mutex);
  while (self->state == SIGNALLING_WHIP_STATE_CONNECTING) {
    pthread_cond_wait(&self->cond, &self->mutex);
  }
  int connected = (self->state == SIGNALLING_WHIP_STATE_CONNECTED);
  pthread_mutex_unlock(&self->mutex);
  return connected;
}
```

シグナリングや接続が失敗して状態を遷移させる経路が走らない限り、この関数は戻らない。

### whip.cpp のブロッキング同期 I/O (`webrtc/src/whip.cpp`)

シグナリングのリクエスト送信は `SendRequest` で行われ、これは CreateOffer のコールバック内から呼ばれる (`webrtc/src/whip.cpp:754`)。このコールバックはシグナリングスレッド上で実行される。

`SendRequest` は素の `getaddrinfo` / `socket` / `connect` と OpenSSL を用いた同期ブロッキング I/O であり、ソケットに送受信タイムアウト (`setsockopt` の `SO_RCVTIMEO` / `SO_SNDTIMEO` 等) を一切設定していない (`webrtc/src/whip.cpp:952-1047`)。

`connect` はブロッキングのまま実行され (`webrtc/src/whip.cpp:987`)、`SSL_connect` (`webrtc/src/whip.cpp:1026`)、`SSL_write` (`webrtc/src/whip.cpp:1031`) も同様である。とくに受信ループは `SSL_read` が `0` 以下を返すまで回り続ける (`webrtc/src/whip.cpp:1039-1045`)。

```cpp
    std::string resp;
    resp.reserve(4096);
    char buf[4096];
    for (;;) {
      int n = SSL_read(ssl, buf, sizeof(buf));
      if (n <= 0) {
        break;
      }
      resp.append(buf, n);
    }
    response_body = resp;
```

相手が `connect` 後に無応答だったり、レスポンスを返さずに接続を維持し続けたりすると、シグナリングスレッドがここでブロックし続ける。これがシグナリングスレッドを占有するため、結果として状態遷移が進まず `WaitForConnect` 側も戻れなくなる。

### whep 側

whep でも同様に、シグナリングのレスポンスを待つ同期処理がタイムアウトを持たない。whep.c の接続待機・whep.cpp のレスポンス受信ループも、相手が応答しない場合に打ち切る仕組みがない。

## 設計方針

- 接続完了の待機 (`SignalingWhip_WaitForConnect` および whep の対応箇所) を、タイムアウト付きの待機 (例: `pthread_cond_timedwait`、C++ 側は `cv_.wait_for`) に置き換え、所定時間を超えたら接続失敗として打ち切る。
- 同期ブロッキング I/O (`SendRequest` および whep の対応箇所) に、接続・送信・受信それぞれのタイムアウトを設定する。`connect` / `SSL_read` / `SSL_write` がタイムアウトで戻るようにし、超過時はエラーとして `on_response(std::nullopt)` 相当でコールバックを終え、状態を失敗へ遷移させる。
- タイムアウト値は固定でよいが、超過時には英語のログメッセージでタイムアウトした旨を出力する。

## 完了条件

- 相手が応答しない場合でも、接続待機と同期 I/O が一定時間で打ち切られ、無期限にハングしない。
- タイムアウト発生時は接続失敗として状態が遷移し、`WaitForConnect` 相当が戻る。
- whip / whep の両方でタイムアウトが効く。
