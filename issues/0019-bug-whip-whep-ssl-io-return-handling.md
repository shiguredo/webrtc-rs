# whip/whep の SSL_read/SSL_write 戻り値とエラーキューの扱いを修正する

- Priority: High
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/fix-whip-whep-ssl-io-return-handling

## 目的

WHIP / WHEP の送受信処理で、OpenSSL の `SSL_read` / `SSL_write` の戻り値の扱いが正しくない。戻り値が 0 や負値の場合に `SSL_get_error` でエラー種別を判別せず、`WANT_READ` / `WANT_WRITE`（再試行が必要）と `ZERO_RETURN`（正常クローズ）とその他の致命的エラーを区別していない。また `SSL_write` の部分送信を再試行せず、戻り値が `<= 0` のときだけ失敗扱いにしているため、`req` の途中までしか送れなかったケースを検知できない。さらに、各 SSL 操作の前に `ERR_clear_error` を呼んでいないため、以前の操作が残したエラーキューのエントリを誤って現在のエラーとして解釈する恐れがある。これらを修正し、部分 I/O と切断を正しく扱う。

## 優先度根拠

High。`SSL_get_error` による分岐を行わないと、再試行可能な状態（`WANT_READ` / `WANT_WRITE`）と正常クローズ（`ZERO_RETURN`）と致命的エラーを取り違える。`SSL_write` の部分送信を再試行しないとリクエストが途中までしか送られず、サーバ側で不正なリクエストとして扱われる。`ERR_clear_error` を呼ばないと `ERR_get_error` が古いエラーを返し、ログやエラー判定が誤る。シグナリングの通信の正しさに直結するため優先度を高くする。

## 現状

`webrtc/src/whip.c` の `SSL_write` は戻り値が `<= 0` のときだけ失敗とみなし、部分送信の再試行も `SSL_get_error` による判別も行っていない。`webrtc/src/whip.c:1036` 付近:

```c
if (SSL_write(ssl, req, (int)strlen(req)) <= 0) {
  RTC_LOG_ERROR("SSL_write failed");
  SSL_free(ssl);
  SSL_CTX_free(ctx);
  close(sock);
  on_response(NULL, user_data);
  return;
}
```

戻り値が正でも `strlen(req)` 未満（部分送信）の可能性があるが、それを検出して残りを再送するループが無い。

受信側も `SSL_read` の戻り値が 0 以下なら一律に `break` しており、`SSL_get_error` で `WANT_READ` / `WANT_WRITE` / `ZERO_RETURN` / 致命的エラーを区別していない。`webrtc/src/whip.c:1056` 付近:

```c
char buf[4096];
for (;;) {
  int n = SSL_read(ssl, buf, sizeof(buf));
  if (n <= 0) {
    break;
  }
  ...
}
```

また、`SSL_connect` / `SSL_write` / `SSL_read` の各操作の前に `ERR_clear_error` を呼ぶ箇所がファイル全体に存在しない（`grep` で `SSL_get_error`・`ERR_clear_error` を検索しても該当無し）。

`webrtc/src/whep.c` も同一構造で、`webrtc/src/whep.c:1010` 付近の `SSL_write`:

```c
if (SSL_write(ssl, req, (int)strlen(req)) <= 0) {
  RTC_LOG_ERROR("SSL_write failed");
  ...
}
```

と、`webrtc/src/whep.c:1030` 付近の `SSL_read` ループ:

```c
char buf[4096];
for (;;) {
  int n = SSL_read(ssl, buf, sizeof(buf));
  if (n <= 0) {
    break;
  }
  ...
}
```

に同じ問題がある。

## 設計方針

- 各 SSL 操作（`SSL_connect` / `SSL_write` / `SSL_read`）の前に `ERR_clear_error` を呼び、エラーキューをクリアしてから操作する。
- `SSL_read` の戻り値が 0 以下のときは `SSL_get_error` でエラー種別を判別し、分岐する。
  - `SSL_ERROR_WANT_READ` / `SSL_ERROR_WANT_WRITE`: 再試行が必要な状態として扱う（ブロッキングソケットでも発生し得るため、再試行する）。
  - `SSL_ERROR_ZERO_RETURN`: 相手による正常な TLS クローズとして受信完了扱いにする。
  - それ以外: 致命的エラーとして失敗を返す。
- `SSL_write` は部分送信を考慮し、送信すべきバイト数を送り切るまでループで再送する。戻り値が 0 以下のときは `SSL_get_error` で `WANT_READ` / `WANT_WRITE` を判別し、再試行可能なら継続、致命的エラーなら失敗を返す。
- C 版（`whip.c` / `whep.c`）に同等の修正を入れる。C++ 版（`whip.cpp` / `whep.cpp`）の `SendRequest` にも同じ問題があるため、合わせて修正する。
- ログメッセージ・エラーメッセージは英語で記述する。

## 完了条件

- `SSL_write` が部分送信したケースでも、残りを再送して全バイトを送り切れる。
- `SSL_read` が `WANT_READ` / `WANT_WRITE` を返したケースを再試行で扱え、`ZERO_RETURN` を正常クローズとして扱い、致命的エラーを失敗として扱える。
- 各 SSL 操作の前に `ERR_clear_error` が呼ばれ、古いエラーキューの影響を受けない。
- C 版・C++ 版の双方で同じ I/O 処理が行われる。
