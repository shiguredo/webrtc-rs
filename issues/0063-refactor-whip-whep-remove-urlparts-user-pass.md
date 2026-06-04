# URLParts の未使用 user_pass を削除する

- Priority: Low
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/refactor-whip-whep-remove-urlparts-user-pass

## 目的

`whip` / `whep` の C 版・C++ 版いずれの `URLParts` にも、Parse 時に格納されるが後段で一度も読み出されない `user_pass` フィールドが存在する。デッドコードを削除して構造体を整理する。ただし汎用 URL パーサとしての将来利用を残す判断もあり得るため、その点も含めて整理する。

## 優先度根拠

ビルドや動作には影響しないデッドコードであり Low とする。ただし未使用メンバは構造体の意図を曖昧にするため整理する価値はある。

## 現状

レビュー時点で実コードと `rg` による参照状況を確認済み。`user_pass` は 4 ファイルすべての `URLParts` に存在する。

C 版の宣言は `whip.c:208` と `whep.c:260`。

```c
struct URLParts {
  char* scheme;
  char* user_pass;
  char* host;
  char* port;
  char* path_query_fragment;
};
```

C++ 版の宣言は `whip.cpp:267` と `whep.cpp:254`。

```cpp
struct URLParts {
  std::string scheme;
  std::string user_pass;
  std::string host;
  std::string port;
  std::string path_query_fragment;
```

いずれも Parse の中で `user_pass` に値が格納される（C 版は `whip.c:254`/`257` と `whep.c:306`/`309`、C++ 版は `whip.cpp:296`/`299` と `whep.cpp:283`/`286`）。

`rg "user_pass"` で各ファイルを確認したところ、`user_pass`（`user_pass_host_port` のような別の変数名を除く）は宣言・解放・Parse 時の代入のみに現れ、格納後に値を読み出す箇所がない。なお `scheme` は `GetPort`（C 版 `URLParts_GetPort`、C++ 版 `URLParts::GetPort`）で参照されるが、`user_pass` はどこからも参照されていない。

## 設計方針

- 4 ファイルの `URLParts` から `user_pass` フィールドと、Parse 時の関連する代入（および C 版の解放・NULL 初期化）を削除する。
- ただし `URLParts` を汎用 URL パーサとして将来も `user_pass` を扱えるよう残す判断もあり得る。その場合は削除せず、残す根拠（汎用パーサとしての位置付け）を明確にする。どちらの方針を取るかを対応時に決める。

## 完了条件

- `user_pass` が 4 ファイルから除去される。または、残す判断とその根拠が明確になる。
- `whip` と `whep` の挙動が変わらないこと。
