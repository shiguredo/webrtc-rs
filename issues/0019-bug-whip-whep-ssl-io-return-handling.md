# whip/whep の SSL_read/SSL_write 戻り値とエラーキューの扱いを修正する

- Priority: High
- Created: 2026-06-05
- Model: Opus 4.8
- Polished: 2026-06-06

## 目的

WHIP / WHEP の送受信処理で、OpenSSL の `SSL_read` / `SSL_write` / `SSL_connect` の戻り値の
扱いが正しくない。戻り値が 0 や負値の場合に `SSL_get_error` でエラー種別を判別せず、
`WANT_READ` / `WANT_WRITE`（再試行が必要）と `ZERO_RETURN`（正常クローズ）と致命的エラーを
区別していない。また各 SSL 操作の前に `ERR_clear_error` を呼んでいないため、以前の操作が
残したエラーキューのエントリを誤って現在のエラーとして解釈する恐れがある。
これらを修正し、SSL I/O のエラーハンドリングを正しく行う。

## 優先度根拠

`SSL_get_error` による分岐を行わないと、再試行可能な状態（`WANT_READ` / `WANT_WRITE`）と
正常クローズ（`ZERO_RETURN`）と致命的エラーを取り違える。`ERR_clear_error` を呼ばないと
`ERR_get_error` が古いエラーを返し、ログやエラー判定が誤る。
シグナリング通信の正しさと診断性に直結するため優先度は High とする。

## 現状

4 ファイルすべてで、`ERR_clear_error` / `SSL_get_error` の使用実績がない。
C++ 版では失敗時に `ERR_get_error()` でエラーコードをログ出力しているが、
直前の `ERR_clear_error()` が無いため古いエラーキューを拾う可能性がある。

`whip.c:1036` — `SSL_write` 部分送信を考慮していない:
```c
if (SSL_write(ssl, req, (int)strlen(req)) <= 0) {
```

`whip.c:1058` — `SSL_read` のエラー種別を判別していない:
```c
int n = SSL_read(ssl, buf, sizeof(buf));
if (n <= 0) {
  break;
}
```

`whep.c:1010`、`whep.c:1032`、`whip.cpp:1031`、`whip.cpp:1040`、
`whep.cpp:769`、`whep.cpp:778` も同様の問題がある。

## 設計方針

- 各 SSL 操作（`SSL_connect` / `SSL_write` / `SSL_read`）の前に `ERR_clear_error` を呼び、
  エラーキューをクリアしてから操作する
- `SSL_read` の戻り値が 0 以下のときは `SSL_get_error` でエラー種別を判別し分岐する:
  - `SSL_ERROR_WANT_READ` / `SSL_ERROR_WANT_WRITE`: 再試行（ブロッキングソケットでも
    TLS 再ネゴシエーション時に発生し得る）
  - `SSL_ERROR_ZERO_RETURN`: 正常な TLS クローズとして受信完了扱い
  - それ以外: 致命的エラーとして失敗を返す
- `SSL_write` は送信すべきバイト数を送り切るまでループで再送する。
  戻り値が 0 以下のときは `SSL_get_error` で判別し、再試行可能なら継続、致命傷なら失敗
- `SSL_connect` の戻り値も同様に `SSL_get_error` で判別する
- C 版・C++ 版の 4 ファイルすべてに対応する

## 完了条件

- `SSL_write` が部分送信したケースでも残りを再送して全バイトを送り切れる
- `SSL_read` が `WANT_READ` / `WANT_WRITE` を返したケースを再試行で扱え、
  `ZERO_RETURN` を正常クローズとして扱い、致命的エラーを失敗として扱える
- 各 SSL 操作の前に `ERR_clear_error` が呼ばれ、古いエラーキューの影響を受けない
- C 版・C++ 版の 4 ファイルで同じ I/O 処理が行われる
