# URLParts_Parse の strndup/strdup 戻り値を検査して NULL 参照を防ぐ

- Priority: High
- Created: 2026-06-05
- Model: Opus 4.8
- Polished: 2026-06-05

## 目的

`URLParts_Parse`（whip.c / whep.c）は URL を分解する際に `strndup` / `strdup` を
複数回呼ぶが、その戻り値が NULL になる失敗を一切検査していない。
`strndup` が NULL を返した直後にその結果を `strchr` に渡しているため、
メモリ確保失敗時に NULL 参照でクラッシュする。これを防ぐため、各 `strndup` / `strdup` の
戻り値を検査して失敗時にはパース失敗として安全に返すようにする。

## 再現手順

1. whip.c / whep.c をメモリ確保失敗が注入できる環境でビルドする
   （例: `LD_PRELOAD` で `malloc` をフックして特定呼び出し回数で NULL を返す）
2. 任意の有効な URL を `URLParts_Parse` に渡す
3. メモリ確保が失敗した `strndup` / `strdup` の戻り値がそのまま `strchr` に渡され、
   セグメンテーション違反でプロセスがクラッシュする

## 優先度根拠

NULL 参照はプロセスのクラッシュに直結する。メモリ確保失敗時や境界条件で発生し得る
堅牢性の欠陥であり、シグナリング接続の入口で起きるため、優先度は High とする。

## 現状

whip.c:227-274 の `URLParts_Parse` 内で呼ばれる各 `strndup` / `strdup` が戻り値を
検査していない。whep.c:279-326 も同一実装。

whip.c の全確保箇所:

| 行 | 呼び出し | 代入先 |
|----|---------|--------|
| 237 | `strndup(url, scheme_len)` | `parts->scheme` |
| 244 | `strdup("")` | `parts->path_query_fragment` (slash==NULL 時) |
| 247 | `strdup(slash)` | `parts->path_query_fragment` (slash!=NULL 時) |
| 249 | `strndup(p, uphp_len)` | `user_pass_host_port`（ローカル変数） |
| 254 | `strdup("")` | `parts->user_pass` (at==NULL 時) |
| 257-258 | `strndup(user_pass_host_port, ...)` | `parts->user_pass` (at!=NULL 時) |
| 259 | `strdup(at + 1)` | `host_port`（ローカル変数, at!=NULL 時） |
| 266 | `strdup("")` | `parts->port` (colon==NULL 時) |
| 268 | `strndup(host_port, ...)` | `parts->host` (colon!=NULL 時) |
| 269 | `strdup(colon + 1)` | `parts->port` (colon!=NULL 時) |

whep.c も対応する箇所（289, 296, 299, 301, 306, 309-310, 311, 318, 320, 321）が
すべて未検査。

## 設計方針

### NULL 検査とエラーハンドリング

各 `strndup` / `strdup` の直後で戻り値が NULL でないか検査する。
いずれかが NULL の場合は `goto cleanup` でエラーパスへ飛び、
確保済みリソースを解放して `return 0` する。

### リソース解放の注意点

`URLParts_clear` は `parts->*` メンバのみを解放する。
`user_pass_host_port` と `host_port` はローカル変数であり、
`parts->*` への代入前の失敗時は個別に `free()` が必要。

解放パターンの分岐（`@` の有無 × `:` の有無）:

| `@` | `:` | 解放すべきローカル変数 |
|-----|-----|----------------------|
| なし | なし | なし（`user_pass_host_port` は `parts->host` へ移動済み） |
| なし | あり | `host_port`（通常パスでは whip.c:270 で free 済みだが、確保失敗時は未解放） |
| あり | なし | `user_pass_host_port`（whip.c:260 で通常 free 済みだが、`host_port` 確保失敗時は未解放） |
| あり | あり | `user_pass_host_port` と `host_port`（それぞれ通常パスで free 済みだが、それぞれの確保失敗時に対応が必要） |

`cleanup` ラベルでは `URLParts_clear(parts)` に加えて、
`user_pass_host_port` と `host_port` のうち `parts->host` へ移動済みでないもの
（まだ所有権が移譲されていないポインタ）のみを `free()` する。
既に `parts->host` へ移動されている場合は `URLParts_clear` が解放するため、
二重解放を避ける必要がある。

### 空文字列 strdup の扱い

`strdup("")` は 1 バイト確保であり現実的に失敗しないが、一貫性のため検査対象に含める。

### 変更対象

- `webrtc/src/whip.c` の `URLParts_Parse` (227-274)
- `webrtc/src/whep.c` の `URLParts_Parse` (279-326)
- 呼び出し元（whip.c:879, whep.c:853）は既に戻り値 0 のエラーハンドリングを行っているため、
  修正不要。関数シグネチャ・戻り値のセマンティクスも変更なし

### 他 issue との関係

- `issues/0012` も同一関数 `URLParts_Parse` を修正対象としており、本 issue を先に解決することで
  `URLParts_Parse` の安全性を確保した上で 0012 の拡張を行える

## テスト戦略

AGENTS.md によりモック・スタブは禁止されているため、実装差し替えによる注入テストは不可。
代わりに以下の方針で検証する:

- AddressSanitizer (`ASAN_OPTIONS=allocator_may_return_null=1`) を有効にしたビルドで
  whip.c / whep.c の各種 URL パターン（scheme あり／なし、ポートあり／なし、
  user_pass あり／なしの組み合わせ）を `main` 関数内で繰り返し `URLParts_Parse` に与え、
  クラッシュしないこと・メモリリークが発生しないことを確認する
- コード上の全確保箇所 10 箇所（1 回の呼び出しで実行されるのは条件分岐により最大 7 箇所）
  のいずれかが失敗したケースでもパスすることをストレステストで確認する

## 完了条件

- `strndup` / `strdup` のいずれかが失敗（NULL 返却）してもクラッシュせず、
  `URLParts_Parse` が 0 を返す
- 失敗経路でメモリリークや二重解放が発生しない
- whip.c / whep.c の両方で対応される
