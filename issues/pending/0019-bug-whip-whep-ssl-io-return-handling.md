# whip/whep の SSL_read/SSL_write 戻り値とエラーキューの扱いを修正する

- Priority: High
- Created: 2026-06-05
- Model: Opus 4.8
- Branch: feature/fix-whip-whep-ssl-io-return-handling
- Polished: 2026-06-06

## pending の理由

whip/whep 関連の対応を保留するため

## 目的

WHIP / WHEP の送受信処理で、`SSL_read` / `SSL_write` / `SSL_connect` の戻り値の扱いが
正しくない。戻り値が 0 や負値の場合に `SSL_get_error` でエラー種別を判別せず、
`WANT_READ` / `WANT_WRITE`（再試行が必要）と `ZERO_RETURN`（正常クローズ）と致命的エラーを
区別していない。また各 SSL 操作の前に `ERR_clear_error` を呼んでいないため、以前の操作が
残したエラーキューのエントリを誤って現在のエラーとして解釈する恐れがある。
これらを修正し、SSL I/O のエラーハンドリングを正しく行う。

## 優先度根拠

シグナリング通信の正しさと診断性に直結するため優先度は High とする。

## 現状

4 ファイルすべてで、`ERR_clear_error` / `SSL_get_error` の使用実績がない。

- `whip_SendRequest`（`webrtc/src/whip.c`）内の `SSL_write` — 部分送信（戻り値が正かつ
  `strlen(req)` 未満）の場合に成功と誤判定する:
  ```c
  if (SSL_write(ssl, req, (int)strlen(req)) <= 0) {
  ```

- 同 `whip_SendRequest` 内の `SSL_read` — エラー種別を判別していない:
  ```c
  int n = SSL_read(ssl, buf, sizeof(buf));
  if (n <= 0) {
    break;
  }
  ```

- `whep_SendRequest`（`webrtc/src/whep.c`）の `SSL_write` / `SSL_read`、
  `SignalingWhip::SendRequest`（`webrtc/src/whip.cpp`）の `SSL_write` / `SSL_read`、
  `SignalingWhep::SendRequest`（`webrtc/src/whep.cpp`）の `SSL_write` / `SSL_read` も
  同様の問題がある。C++ 版は失敗時に `ERR_get_error()` でエラーコードをログ出力しているが、
  直前の `ERR_clear_error()` が無いため古いエラーキューを拾う可能性がある。

なお、この 4 ファイルは OpenSSL API 経由で BoringSSL（libwebrtc 同梱）にリンクしている
（`webrtc/CMakeLists.txt` の `third_party/boringssl/src/include`）。

## 設計方針

- 各 SSL 操作（`SSL_connect` / `SSL_write` / `SSL_read`）の前に `ERR_clear_error` を呼び、
  エラーキューをクリアしてから操作する
- `SSL_read` の戻り値が 0 以下のときは `SSL_get_error` でエラー種別を判別し分岐する:
  - `SSL_ERROR_WANT_READ` / `SSL_ERROR_WANT_WRITE`: 再試行する。再試行はソケットが
    読み書き可能になるのを待ってから行う（非ブロッキング化した場合にビジーループに
    ならないよう、`select()` / `poll()` 等で待つ旨を実装に含める）
  - `SSL_ERROR_ZERO_RETURN`: 正常な TLS クローズとして受信完了扱い
  - `SSL_ERROR_SYSCALL` で `ret == 0`（unexpected EOF、close_notify なしの接続クローズ）:
    受信完了扱いとする。HTTP/1.1 + `Connection: close` の実装はサーバが close_notify を
    送らず TCP を閉じるケースを前提としており、これを致命的エラー扱いにすると正常な
    レスポンスを破棄する回帰を招く（0018 のフォールバック読み切りモードと整合）。
    unexpected EOF の判定は `SSL_read` の戻り値が 0 であること（`ret == 0`）で行う
    （BoringSSL の `SSL_ERROR_SYSCALL` 定義も EOF 判定を errno でなく戻り値 0 で行う）。
    errno は EOF 時に更新されず、直前の失敗したシステムコールの値（stale）が残り得る
    ため、errno == 0 を判定に使わない
  - それ以外: 致命的エラーとして失敗を返す
- `SSL_write` は送信すべきバイト数を送り切るまでループで再送する。正の部分送信が返ったら
  書き込み済みバイト数だけポインタと残り長さを進める。`WANT_READ` / `WANT_WRITE` による
  再試行は同一引数で行う（OpenSSL / BoringSSL の規約）。`SSL_ERROR_ZERO_RETURN` は
  ピアが close_notify を送った状態であり、再試行不能な致命的エラーとして失敗を返す。
  それ以外のエラーも失敗を返す
- `SSL_connect` の戻り値も同様に `SSL_get_error` で判別し、`WANT_READ` / `WANT_WRITE` は
  再試行、それ以外は失敗を返す
- 失敗時はエラー種別（`SSL_get_error` の戻り値）とエラーキューの詳細
  （`ERR_error_string` 相当）を英語のログに出力する。C++ 版は現状どおり
  `ERR_get_error()` のエラーコードを併記する
- C 版・C++ 版の 4 ファイルすべてに対応する

### 依存関係

- `issues/0018`（Content-Length / chunked 解釈）は本 issue を前提とする。0018 の
  「`Content-Length` 宣言値に満たない ZERO_RETURN はボディ欠損としてエラー」は、
  本 issue で ZERO_RETURN と致命的エラーを判別できて初めて成立する。本 issue の
  「ZERO_RETURN / unexpected EOF = 受信完了」は、0018 の Content-Length framing 実装時に
  条件付きで調整される（宣言長未満のクローズは欠損としてエラー）
- `issues/0064`（`<openssl/err.h>` の include 削除）は `ERR_` 系関数を一切呼んでいない
  前提で include 削除を計画している。本 issue で `ERR_clear_error` / `SSL_get_error` を
  追加するため 0064 の前提が崩れる。本 issue を先に実装し、0064 の前提を再評価すること
- `issues/0012`（URL scheme に応じた接続方式の切り替え）は本 issue と同じ TLS セット
  アップブロック（`SSL_connect` 以降）を改修する。実装順序によっては競合する
- `issues/0023`（C++ 版 `SendRequest` の `on_response` 二重呼び出し）は同じ
  `SignalingWhip::SendRequest` / `SignalingWhep::SendRequest` の失敗パスを対象とする。
  実装順序によっては競合する
- `issues/0034`（接続タイムアウト）は同じソケットにタイムアウトを設定する。タイムアウト
  時は `SSL_read` が `SSL_ERROR_SYSCALL` + errno == EAGAIN で戻るため、本 issue の
  SYSCALL 分岐（`ret == 0` のみ受信完了）と相互作用する。実装順序によっては競合する

## 完了条件

- `SSL_write` が部分送信したケースでも残りを再送して全バイトを送り切れる
- `SSL_read` が `WANT_READ` / `WANT_WRITE` を返したケースを再試行で扱え、
  `ZERO_RETURN`（正常クローズ）と unexpected EOF（`SSL_ERROR_SYSCALL` + `ret == 0`）を
  受信完了として扱い、致命的エラーを失敗として扱える
- `SSL_connect` の戻り値も `SSL_get_error` で判別し、`WANT_READ` / `WANT_WRITE` は再試行、
  それ以外は失敗として扱える
- 各 SSL 操作の前に `ERR_clear_error` が呼ばれ、古いエラーキューの影響を受けない
- 失敗時にエラー種別（`SSL_get_error` の戻り値）とエラーキューの詳細、C++ 版は
  `ERR_get_error()` のエラーコードを含む英語のログが出力される
- C 版・C++ 版の 4 ファイルで同じ I/O 処理が行われる

## テスト戦略

`webrtc/` 配下の C/C++ サンプルには自動テスト基盤がないため、手動確認で行う。

- `SSL_write` の部分送信: `SSL_MODE_ENABLE_PARTIAL_WRITE` は設定していないため、
  ブロッキングソケットでは正の部分送信は返らず、全長返却かエラーのみである。部分送信
  パスを検証するには、テスト時に `SSL_MODE_ENABLE_PARTIAL_WRITE` を一時的に有効化するか、
  非ブロッキングソケット + 送信バッファ満杯の構成にする必要がある。ループで送り切る
  実装自体は防御的対応として残す
- `WANT_READ` / `WANT_WRITE` の再試行: ブロッキングソケットでは通常発生しない。再試行
  ロジックは防御的実装として検証する
- close_notify ありの正常クローズ（`ZERO_RETURN`）と close_notify なしの接続クローズ
  （`SSL_ERROR_SYSCALL` + errno == 0）の両方を、サーバ側のシャットダウン方法を変えて
  確認し、どちらも受信完了として扱われることを確認する
- 試験用サーバは実サーバとして位置づけ、モック・スタブは使わない（AGENTS.md の「モックや
  スタブは絶対に利用しないこと」に従う）
