# URLParts_Parse の strndup/strdup 戻り値を検査して NULL 参照を防ぐ

- Priority: High
- Created: 2026-06-05
- Model: Opus 4.8
- Branch: feature/fix-whip-whep-urlparts-strndup-null-deref
- Polished: 2026-08-05

## pending の理由

whip/whep 関連の対応を保留するため

## 目的

`URLParts_Parse`（`webrtc/src/whip.c` / `whep.c`）は URL を分解する際に
`strndup` / `strdup` を複数回呼ぶが、その戻り値が NULL になる失敗を一切検査していない。
`strndup` が NULL を返すと、`@` の探索（`strchr(user_pass_host_port, '@')`）や
`:` の探索（`strchr(host_port, ':')`）に NULL が渡り NULL 参照でクラッシュする。
また `parts->path_query_fragment` / `parts->host` が NULL のままパースが成功すると、
呼び出し元の `strlen` でも NULL 参照になる。これを防ぐため、各 `strndup` / `strdup` の
戻り値を検査して失敗時にはパース失敗として安全に返すようにする。

## 再現手順

1. `webrtc/src/whip.c` / `whep.c` をビルドし、メモリ確保失敗を注入する
   `LD_PRELOAD` / `DYLD_INSERT_LIBRARIES` ライブラリ付きで実行する
   （例: フックライブラリの malloc を特定呼び出し回数で NULL を返すようにする）
2. 任意の有効な URL を `URLParts_Parse` に渡す
3. メモリ確保が失敗した `strndup` / `strdup` の戻り値がそのまま `strchr` に渡され、
   セグメンテーション違反でプロセスがクラッシュする（`parts->path_query_fragment` /
   `parts->host` が NULL のまま呼び出し元の `strlen` でクラッシュする経路もある）

## 優先度根拠

NULL 参照はプロセスのクラッシュに直結する。メモリ確保失敗時に発生し得る
堅牢性の欠陥であり、シグナリング接続の入口で起きるため、優先度は High とする。

## 現状

`URLParts_Parse` 内で呼ばれる各 `strndup` / `strdup` が戻り値を検査していない。
whip.c と whep.c は同一実装。

whip.c の全確保箇所（`URLParts_Parse` 内の呼び出し順）:

| # | 呼び出し | 代入先 |
|---|----------|--------|
| 1 | `strndup(url, scheme_len)` | `parts->scheme` |
| 2 | `strdup("")` | `parts->path_query_fragment`（slash==NULL 時） |
| 3 | `strdup(slash)` | `parts->path_query_fragment`（slash!=NULL 時） |
| 4 | `strndup(p, uphp_len)` | `user_pass_host_port`（ローカル変数） |
| 5 | `strdup("")` | `parts->user_pass` (at==NULL 時) |
| 6 | `strndup(user_pass_host_port, ...)` | `parts->user_pass` (at!=NULL 時) |
| 7 | `strdup(at + 1)` | `host_port`（ローカル変数, at!=NULL 時） |
| 8 | `strdup("")` | `parts->port` (colon==NULL 時) |
| 9 | `strndup(host_port, ...)` | `parts->host` (colon!=NULL 時) |
| 10 | `strdup(colon + 1)` | `parts->port` (colon!=NULL 時) |

whep.c も対応する全 10 箇所が同様に未検査。

## 設計方針

### NULL 検査とエラーハンドリング

各 `strndup` / `strdup` の直後で戻り値が NULL でないか検査する。
いずれかが NULL の場合は `goto cleanup` でエラーパスへ飛び、
確保済みリソースを解放して `return 0` する。

### ローカル変数の初期化

`user_pass_host_port` と `host_port` は関数冒頭で `NULL` 初期化する。
確保箇所 1〜3 が失敗した時点ではこれらは未割り当てだが、cleanup で無条件に
`free()` するため、NULL 初期化が必須である（未初期化のまま free すると未定義動作）。

### リソース解放の注意点（NULL 簿記方式）

`URLParts_clear` は `parts->*` メンバのみを解放する。`user_pass_host_port` と
`host_port` はローカル変数であり、cleanup で個別に `free()` が必要だが、
次の 3 つの状態を区別しなければならない。

- `user_pass_host_port` は at!=NULL 分岐で `free(user_pass_host_port)` 済み
  （通常パス）
- `user_pass_host_port` は at==NULL 分岐で `host_port` へ移動済み
  （`host_port = user_pass_host_port` で同一バッファを共有する）
- `host_port` は colon!=NULL 分岐で `free(host_port)` 済み、または
  `parts->host` へ移動済み（`parts->host = host_port`）

このため、NULL 簿記方式を採る:

1. 通常パスの `free()` 直後に該当ローカル変数を `NULL` 化する
   （`free(user_pass_host_port); user_pass_host_port = NULL;` 等）
2. at==NULL 分岐で `host_port = user_pass_host_port` とした直後に
   `user_pass_host_port = NULL` とする（所有権は `host_port` へ移る）
3. `host_port` を `parts->host` へ移動した直後に `host_port = NULL` とする
   （解放は `URLParts_clear` が担うため二重解放を避ける）
4. cleanup では `user_pass_host_port` と `host_port` を無条件に
   `free()` する（NULL 化されているため二重解放・リークが起きない）。
   その後 `URLParts_clear(parts)` を呼ぶ

この方式なら「どの確保が失敗したか」を追跡する必要がなく、失敗箇所ごとの
解放分岐が不要になる。

### 空文字列 strdup の扱い

`strdup("")` は 1 バイト確保であるが、注入環境では失敗し得るため、
一貫性のため検査対象に含める。

### 変更対象

- `webrtc/src/whip.c` の `URLParts_Parse`
- `webrtc/src/whep.c` の `URLParts_Parse`
- 呼び出し元（`whip_OnCreateOfferSuccess` / `whep_OnCreateOfferSuccess` 内）は
  既に戻り値 0 をエラーとして処理しているため、修正不要。関数シグネチャ・戻り値の
  セマンティクスも変更なし

### 他 issue との関係

- `issues/0012` は `URLParts_GetPort` / `SendRequest` を対象としており、
  `URLParts_Parse` 本体は対象外。本 issue を先に解決することで `URLParts_Parse` の
  安全性を確保した上で 0012 の拡張を行える
- `issues/0063`（`URLParts` の未使用 `user_pass` フィールド削除）は本 issue の
  確保箇所 5・6 を対象領域とするため、0063 を先に実装すると確保箇所が 10 → 8 に
  変わる。実装順序に注意すること

## テスト戦略

モック・スタブは禁止されているが、`LD_PRELOAD` による `malloc` のフックは
テスト対象コードの差し替えではなく C ライブラリへの故障注入であり、禁止に該当しない。
この手法で各確保箇所を決定的に失敗させて検証する。

- 再現手順と同じ `LD_PRELOAD` による `malloc` フック（macOS では
  `DYLD_INSERT_LIBRARIES`）を使い、`URLParts_Parse` 内の確保だけを対象に
  失敗回数をカウントして、各確保箇所のそれぞれで NULL を返すようにし、
  `URLParts_Parse` がクラッシュせず 0 を返すことを確認する。1 回のパースで実行される
  確保は最大 7 箇所（slash の有無・at の有無・colon の有無で排他）のため、
  単一 URL では全 10 箇所を網羅できない。全 10 箇所を網羅するには、全要素
  （slash あり・at あり・colon あり、例: `http://user:pass@host:8080/path`）と
  各要素なし（slash なし・at なし・colon なし、例: `http://host`）の URL パターンを
  失敗回数と組み合わせる
- 二重解放・リークの検出は、`LD_PRELOAD` / `DYLD_INSERT_LIBRARIES` のフック
  ライブラリ内で確保・解放の簿記を持たせて検出する（malloc と free をラップし、
  解放済みポインタの再解放を検出する）。ASAN の `allocator_may_return_null=1` は
  実際にメモリ不足が発生した時に NULL を返すだけであり、通常サイズの URL では
  失敗経路を通らないため、失敗経路の検証はフックライブラリ側が担う
- 更に `ASAN_OPTIONS=allocator_may_return_null=1` を有効にしたビルドで、
  whip.c / whep.c の各種 URL パターン（scheme あり、ポートあり／なし、
  user_pass あり／なしの組み合わせ）を `main` 関数内で繰り返し
  `URLParts_Parse` に与え、正常系のパースが従来どおり成功し、各フィールド
  （scheme・host・port・path_query_fragment・user_pass）が期待値どおり設定される
  ことを確認する（挙動不変の検証）。二重解放・リークの検出は上記の
  フックライブラリの簿記が担う（macOS の ASAN には LeakSanitizer が含まれないため、
  リーク検出を ASAN に依存しない）
- 失敗経路でメモリリークや二重解放が発生しないことも上記の検証で確認する
- `main` 関数は実接続フローを実行するため、テストコードはフラグ分岐などで
  実フローと共存させる

## 完了条件

- `strndup` / `strdup` のいずれかが失敗（NULL 返却）してもクラッシュせず、
  `URLParts_Parse` が 0 を返す
- 失敗経路でメモリリークや二重解放が発生しない
- 正常系では従来どおりパースに成功し、URLParts の各フィールドが正しく設定される
- whip.c / whep.c の両方で対応される
