# std_string_size の size_t から int への切り詰めを修正する

- Priority: Medium
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/fix-webrtc-c-std-string-size-int-truncation

## 目的

webrtc_c の `std_string_size` が返す文字列長を正しく表現できるようにする。現状は `std::string::size()` が返す `size_t` を `int` に切り詰めて返しており、2GiB を超える文字列で値が破綻するため、戻り値型を `size_t` にして正しいサイズを返せるようにする。

## 優先度根拠

通常扱う文字列長では `int` の範囲に収まり問題は顕在化しないが、2GiB を超える文字列では `int` への切り詰めで負値や誤った値が返り、それを長さとして利用するアプリケーション側で範囲外アクセスやメモリ破壊を招く恐れがある。発生条件は限定的だが、サイズを返す API としての正しさに直結し、戻り値型の変更で根本的に解消できるため Medium とする。

## 現状

`webrtc/src/webrtc_c/std.cc:19-22` の `std_string_size` は `size()` の結果を `int` に切り詰めて返している。

```cpp
WEBRTC_EXPORT int std_string_size(struct std_string* self) {
  auto str = reinterpret_cast<std::string*>(self);
  return static_cast<int>(str->size());
}
```

`std::string::size()` の戻り値は `size_t` だが、これを `static_cast<int>` で `int` へ変換しているため、文字列長が `INT_MAX` (約 2GiB) を超えると値が破綻する。

戻り値型はヘッダ側でも `int` として宣言されている (`webrtc/src/webrtc_c/std.h:16`)。

```cpp
WEBRTC_EXPORT int std_string_size(struct std_string* self);
```

なお、`std.h` は既に `#include <stddef.h>` を含んでおり (`webrtc/src/webrtc_c/std.h:3`)、`size_t` を利用可能である。また同ファイル内には `std_string_append` が `size_t len` を受け取る例 (`webrtc/src/webrtc_c/std.h:18-20`) があり、`size_t` を使う API は既存である。

## 設計方針

`std_string_size` の戻り値型を `int` から `size_t` に変更する。実装側 (`webrtc/src/webrtc_c/std.cc:19-22`) の `static_cast<int>` を撤去し、`size()` の戻り値をそのまま `size_t` で返す。ヘッダ側 (`webrtc/src/webrtc_c/std.h:16`) の宣言も `size_t` に合わせる。Rust 側など C API の利用箇所が `std_string_size` の戻り値型に依存している場合は、`size_t` への変更に追従させる。

## 完了条件

- `std_string_size` の戻り値型が `size_t` になり、実装から `int` への切り詰めが排除されている。
- ヘッダの宣言と実装の戻り値型が一致している。
- 2GiB を超える文字列に対しても切り詰めなく正しいサイズが返る。
- 戻り値型の変更に伴う呼び出し側 (存在する場合) の追従が完了している。
