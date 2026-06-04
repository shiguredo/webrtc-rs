# whip.c の手書き strlen による HTTP リクエスト組み立てを整理する

- Priority: Low
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/refactor-whip-manual-strlen-http-build

## 目的

`webrtc/src/whip.c` の WHIP シグナリングでは、HTTP リクエスト文字列を手書きの `strlen` の積み上げと `malloc` / `snprintf` の組み合わせで組み立てている。リクエスト本文とサイズ計算の式が別々に書かれているため、片方だけ変更すると容易に不整合が起き、バッファ長の計算ミスを誘発しやすい。HTTP リクエストの組み立てを安全で読みやすい形に整理し、保守性を高める。

## 優先度根拠

現状の実装は動作しており、機能上の不具合が確認されているわけではない。あくまで可読性と保守性の改善であり、利用者への影響もないため Low とする。

## 現状

`webrtc/src/whip.c:905` 付近では、HTTP リクエストのバッファ長を `strlen` でリテラル文字列ごとに加算して求めている。

```c
  size_t req_len =
      strlen("POST ") + strlen(target) + strlen(" HTTP/1.1\r\nHost: ") +
      strlen(parts.host) + 1 + strlen(URLParts_GetPort(&parts)) +
      strlen("\r\n") +
      strlen("Content-Type: application/sdp\r\nContent-Length: ") +
      strlen(content_length) +
      strlen("\r\nUser-Agent: Whip-Client\r\nConnection: close\r\n\r\n") +
      strlen(offer_sdp_str) + 1;
  char* req = (char*)malloc(req_len);
```

そして `webrtc/src/whip.c:922` 付近で、上記の長さ計算とは別にもう一度同じリクエスト書式を `snprintf` で組み立てている。

```c
  snprintf(req, req_len,
           "POST %s HTTP/1.1\r\nHost: %s:%s\r\n"
           "Content-Type: application/sdp\r\nContent-Length: %s\r\n"
           "User-Agent: Whip-Client\r\nConnection: close\r\n\r\n%s",
           target, parts.host, URLParts_GetPort(&parts), content_length,
           offer_sdp_str);
```

リクエスト書式が長さ計算用（`req_len` の各 `strlen`）と出力用（`snprintf` の書式文字列）の 2 箇所に重複して書かれており、ヘッダ文字列やフィールドを変更する際に両方を矛盾なく更新しなければならない。`webrtc/src/whip.c:889` 付近の `target` 組み立ても同様に `malloc` と `snprintf` の組で書かれている。

## 設計方針

HTTP リクエストの組み立てを、長さ計算と書式が二重管理にならない安全な文字列構築方法に整理する。具体的な手段は実装時に検討するが、リクエスト書式の定義箇所を一本化し、バッファ長の計算ミスが起きにくい構造にすることを方針とする。既存のメモリ確保失敗時のエラーハンドリングおよび解放処理は維持する。

## 完了条件

- HTTP リクエストの組み立てが、長さ計算と書式の二重管理を解消した安全で読みやすい形になっている。
- リクエストの送信内容が現状と等価である。
- メモリ確保失敗時のエラーハンドリングと後処理が維持されている。
