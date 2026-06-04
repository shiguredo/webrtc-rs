# whip/whep で Content-Length と chunked エンコーディングを解釈する

- Priority: High
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/fix-whip-whep-content-length-chunked-ignored

## 目的

WHIP / WHEP のレスポンス受信処理が、`Content-Length` ヘッダも `Transfer-Encoding: chunked` も一切解釈しておらず、ソケットが読み切る（`SSL_read` が 0 以下を返す）まで読み続ける実装になっている。これはサーバが `Connection: close` で接続を閉じてくれることに暗黙に依存しており、レスポンスのメッセージ境界をプロトコル的に確定していない。`Content-Length` を尊重すればボディ長を正しく確定でき、`chunked` を解釈すればチャンク化されたボディを正しく復元できる。これにより、ボディの途切れ・読み過ぎ・接続が閉じられない場合のブロッキングを避け、シグナリングの堅牢性を高める。

## 優先度根拠

High。HTTP のメッセージ境界をプロトコルに従って確定していないため、サーバが接続を閉じない構成（keep-alive）や chunked でボディを返す構成では、レスポンスを正しく受信できない。ボディの途切れや読み過ぎは SDP 解析の失敗に直結し、接続が閉じられない場合は読み取りループでブロックし続ける危険がある。受信処理の正しさに直結するため優先度を高くする。

## 現状

`webrtc/src/whip.c` の送信処理は、`SSL_read` が 0 以下を返すまで無条件にボディを読み続けている。`webrtc/src/whip.c:1056` 付近:

```c
char buf[4096];
for (;;) {
  int n = SSL_read(ssl, buf, sizeof(buf));
  if (n <= 0) {
    break;
  }
  if (resp_len + (size_t)n + 1 > resp_cap) {
    resp_cap = (resp_len + (size_t)n + 1) * 2;
    char* new_resp = (char*)realloc(resp, resp_cap);
    if (new_resp == NULL) {
      free(resp);
      SSL_free(ssl);
      SSL_CTX_free(ctx);
      close(sock);
      on_response(NULL, user_data);
      return;
    }
    resp = new_resp;
  }
  memcpy(resp + resp_len, buf, (size_t)n);
  resp_len += (size_t)n;
}
resp[resp_len] = '\0';
```

ここではヘッダの `Content-Length` を読み取らず、`Transfer-Encoding: chunked` の判定やデチャンク処理も行っていない。受信したバイト列をそのまま連結しているだけで、後段の `whip_OnSendRequestResponse` が `\r\n\r\n` でヘッダとボディを分割するが、ボディ部分のチャンクフレーミング（チャンクサイズ行や終端チャンク）は解釈されないため、chunked レスポンスの場合はボディにチャンクサイズ等が混入する。

`webrtc/src/whep.c` も同一構造で、`webrtc/src/whep.c:1030` 付近に同じ読み取りループがある:

```c
char buf[4096];
for (;;) {
  int n = SSL_read(ssl, buf, sizeof(buf));
  if (n <= 0) {
    break;
  }
  ...
  memcpy(resp + resp_len, buf, (size_t)n);
  resp_len += (size_t)n;
}
resp[resp_len] = '\0';
```

C++ 版の `webrtc/src/whip.cpp` の `SendRequest` も同様に読み切りに依存している。`webrtc/src/whip.cpp:1036` 付近:

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

`webrtc/src/whep.cpp` の `SendRequest` も同様。なお、リクエスト送信側では自前で `Content-Length` を付与している（`webrtc/src/whip.cpp:747` 付近で `req += "Content-Length: " + std::to_string(offer_sdp.size())`）が、レスポンス側の `Content-Length` は解釈していない。

## 設計方針

- ヘッダをパースした段階で `Content-Length` ヘッダを読み取り、値が存在する場合はその長さ分だけボディを受信したら受信を完了とする。
- `Transfer-Encoding: chunked` が指定されている場合は、ボディをチャンク単位でデチャンクする。
  - 各チャンクは「16 進のチャンクサイズ行（`\r\n` 終端）」＋「チャンクデータ」＋「`\r\n`」で構成され、サイズ 0 のチャンクで終端となる。デチャンク後のバイト列を最終的なボディとして扱う。
- `Content-Length` と `chunked` のどちらも無い場合のみ、従来どおり接続クローズまでの読み切りにフォールバックする。
- ヘッダ終端（`\r\n\r\n`）を受信し終えるまではヘッダ用に読み進め、終端以降をボディとして扱う。ヘッダとボディが同じ `SSL_read` 呼び出しにまたがって到着し得る点に注意する。
- C 版（`whip.c` / `whep.c`）と C++ 版（`whip.cpp` / `whep.cpp`）の双方に同等の処理を入れる。
- ログメッセージ・エラーメッセージは英語で記述する。

## 完了条件

- `Content-Length` が指定されたレスポンスを、指定された長さちょうどでボディとして受信できる。
- `Transfer-Encoding: chunked` のレスポンスを、デチャンクして正しいボディとして受信できる。
- どちらのヘッダも無いレスポンスは従来どおり接続クローズまでの読み切りで受信できる。
- C 版・C++ 版の双方で同じ受信処理が行われる。
