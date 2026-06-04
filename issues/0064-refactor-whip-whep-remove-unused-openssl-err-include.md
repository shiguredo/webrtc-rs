# whip.c/whep.c の未使用 openssl/err.h を削除する

- Priority: Low
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/refactor-whip-whep-remove-unused-openssl-err-include

## 目的

`webrtc/src/whip.c` と `webrtc/src/whep.c` は `<openssl/err.h>` を include しているが、`ERR_` 系関数を一切呼んでいない。未使用の include を削除して依存を整理する。

## 優先度根拠

ビルドや動作には影響しない未使用 include であり Low とする。ただし不要な依存は読み手を誤解させるため整理する価値はある。

## 現状

レビュー時点で実コードと `rg` による参照状況を確認済み。`whip.c:13` で `<openssl/err.h>` を include している。

```c
#include <openssl/err.h>
#include <openssl/ssl.h>
```

`whep.c:9` でも同様に include している。

```c
#include <netdb.h>
#include <openssl/err.h>
#include <openssl/ssl.h>
#include <sys/socket.h>
```

`rg "ERR_" whip.c` / `rg "ERR_" whep.c` で確認したところ、いずれも `ERR_` 系関数の参照が一切ない（マッチゼロを確認）。一方で `rg "ERR_"` を `.cpp` 側にかけると、`whip.cpp:1021`/`1027`/`1032` と `whep.cpp:759`/`765`/`770` で `ERR_get_error()` が使われており、`.cpp` 側は `<openssl/err.h>` が必要であるため削除できない。

## 設計方針

- `whip.c:13` および `whep.c:9` の `#include <openssl/err.h>` を削除する。
- `<openssl/ssl.h>` など他の include には手を加えない。
- `.cpp` 側の `<openssl/err.h>` は `ERR_get_error()` で使用しているため削除しない。

## 完了条件

- `.c` 側（`whip.c` と `whep.c`）から未使用 include `<openssl/err.h>` が除去される。
- `whip` と `whep` がビルドでき、挙動が変わらないこと。
