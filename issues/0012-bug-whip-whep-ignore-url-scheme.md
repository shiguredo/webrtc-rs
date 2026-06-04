# whip/whep で URL scheme を尊重して接続方式を切り替える

- Priority: High
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/fix-whip-whep-ignore-url-scheme

## 目的

whip/whep のシグナリング URL には `http://` または `https://` の scheme が指定される。
しかし現状の実装は URL から scheme をパースしているにもかかわらず、接続時には scheme を見ずに
無条件で TLS ハンドシェイク（`SSL_connect`）を行う。このため `http://` を指定すると平文ポートへ
TLS を送ってしまい接続できない。scheme を尊重して TLS／平文を切り替えられるようにする。

## 優先度根拠

scheme を無視する実装は `http://` 指定時に接続不能となる機能上の欠陥であり、設定例も `http://` を
与えているため、現状の設定では接続が成立しない。利用者が指定したプロトコルが反映されないことは
影響が大きいため、優先度は High とする。

## 現状

`URLParts_Parse`（whip.c）/ `URLParts::Parse`（whip.cpp）で URL から scheme を取得しているが、
`SendRequest` 内ではその scheme を参照せず、常に `SSL_CTX_new(TLS_client_method())` から
`SSL_connect` までを実行する。平文 TCP へのフォールバック経路は存在しない。

webrtc/src/whip.c:953-957 では `SendRequest` は host と port のみを受け取り、scheme を受け取って
いない。

```c
static void whip_SendRequest(const char* host,
                             const char* port,
                             const char* req,
                             void (*on_response)(char* resp, void* user_data),
                             void* user_data) {
```

webrtc/src/whip.c:1026-1034 では scheme に関係なく `SSL_connect` する。

```c
  SSL_set_fd(ssl, sock);
  if (SSL_connect(ssl) != 1) {
    RTC_LOG_ERROR("SSL_connect failed");
    SSL_free(ssl);
    SSL_CTX_free(ctx);
    close(sock);
    on_response(NULL, user_data);
    return;
  }
```

webrtc/src/whip.cpp:952-956 でも `SendRequest` は host と port のみを受け取る。

```cpp
  static void SendRequest(
      const std::string& host,
      const std::string& port,
      const std::string& req,
      std::function<void(std::optional<std::string>)> on_response) {
```

webrtc/src/whip.cpp:1025-1029 でも scheme に関係なく `SSL_connect` する。

```cpp
    SSL_set_fd(ssl, static_cast<int>(sock));
    if (SSL_connect(ssl) != 1) {
      RTC_LOG(LS_ERROR) << "SSL_connect failed: ec=" << ERR_get_error();
      return;
    }
```

設定例 webrtc/src/whip.cpp:1116 は `http://` スキームを与えている。

```cpp
    config.signaling_url = "http://192.0.2.1/whip";
```

ポート決定ロジック webrtc/src/whip.c:276-285（`URLParts_GetPort`）は scheme が `wss` / `https`
のとき 443、それ以外は 80 を返す。

```c
static const char* URLParts_GetPort(struct URLParts* parts) {
  if (parts->port != NULL && parts->port[0] != '\0') {
    return parts->port;
  }
  if (parts->scheme != NULL && (strcmp(parts->scheme, "wss") == 0 ||
                                strcmp(parts->scheme, "https") == 0)) {
    return "443";
  }
  return "80";
}
```

whep.c / whep.cpp も同一構造で、scheme を無視して `SSL_connect` する。webrtc/src/whep.cpp:882 の
設定例も `config.signaling_url = "http://192.0.2.1/whep";` となっている。

## 設計方針

- `SendRequest` に scheme を伝え、scheme が `https` のときは TLS、`http` のときは平文 TCP で
  接続するように分岐する
- デフォルトポートは scheme から決定する（`http` は 80、`https` は 443）。`URLParts_GetPort` の
  既存ロジックと整合させる
- whip.c / whip.cpp / whep.c / whep.cpp の 4 ファイルに同じ方針を適用する

## 完了条件

- `http://` 指定時は平文 TCP で接続できる
- `https://` 指定時は TLS で接続できる
- scheme に応じてデフォルトポート（http は 80、https は 443）が選択される
