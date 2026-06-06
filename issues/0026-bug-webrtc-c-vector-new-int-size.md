# WEBRTC_DEFINE_VECTOR の _new(int size) を不正サイズに耐性を持たせる

- Priority: Medium
- Polished: 2026-06-06
- Created: 2026-06-05
- Model: Opus 4.8

## 目的

webrtc_c の `*_vector_new(int size)` 系 API が、負値や過大なサイズを渡された場合でも異常終了しないようにする。現状は `int` のサイズをそのまま `std::vector` のコンストラクタに渡しており、不正な値で `std::length_error` や `std::bad_alloc` が送出されて C アプリケーションが異常終了する恐れがあるため、入力サイズを信用しない方針に合わせて是正する。

## 優先度根拠

正常な使い方では問題は起きないが、サイズ値の出所がデコード結果など信頼できない場合に、負値や過大値で例外送出による異常終了に至る。CLAUDE.md の Rust 方針では入力サイズを信用せず事前確保を避ける指針が示されており、C ラッパー側の同種の問題も整合させる価値がある。常時クラッシュではなく不正入力時の堅牢性の問題であるため Medium とする。

## 現状

`webrtc/src/webrtc_c/common.impl.h:65-70` の `WEBRTC_DEFINE_VECTOR` の `_vector_new` は、`int size` を `std::vector<cpptype>(size)` にそのまま渡している。

```cpp
#define WEBRTC_DEFINE_VECTOR(type, cpptype)                               \
  WEBRTC_EXPORT struct WEBRTC_CONCAT(type, _vector) *                     \
      WEBRTC_CONCAT(type, _vector_new)(int size) {                        \
    auto vec = new std::vector<cpptype>(size);                            \
    return reinterpret_cast<struct WEBRTC_CONCAT(type, _vector)*>(vec);   \
  }                                                                       \
```

`int` が負値の場合、`size_t` への暗黙変換で極端に大きな値となり `std::length_error` または `std::bad_alloc` が送出される。過大な正値の場合も同様にメモリ確保に失敗して例外送出に至る。C の境界 (`WEBRTC_EXPORT` 関数) を C++ 例外が越えると未定義動作・異常終了の原因となる。

同種の `int size` を受け取って `std::vector` / `absl::InlinedVector` を確保する定義が他にもある。

- `webrtc/src/webrtc_c/common.impl.h:151-157` の `WEBRTC_DEFINE_REFCOUNTED_VECTOR` の `_refcounted_vector_new(int size)` (`new std::vector<webrtc::scoped_refptr<cpptype>>(size)`)
- `webrtc/src/webrtc_c/common.impl.h:210-216` の `WEBRTC_DEFINE_INLINED_VECTOR` の `_inlined_vector_new(int size)` (`new absl::InlinedVector<cpptype, max_size>(size)`)

加えて、`_vector_resize(int size)` (`webrtc/src/webrtc_c/common.impl.h:87-91` ほか) や `_vector_get(int index)` / `_vector_set(int index, ...)` なども `int` を受け取っており、負値や範囲外の値に対する検証がない。これらも同じく `int` をそのまま使っている点で同種の問題を持つ。

なお、これらの `_vector_new(int size)` のシグネチャはヘッダ側のマクロでも定義されている (`webrtc/src/webrtc_c/common.h:54-58` の `WEBRTC_DECLARE_VECTOR` で `_vector_new(int size)`)。

## 設計方針

- `_vector_new` に渡された `size` を検証し、`size <= 0` の場合は空のベクタ（`new std::vector<cpptype>()`）を返す。既存の呼び出し元に `nullptr` チェックが存在しないことを考慮し、`nullptr` を返すより空ベクタを返す方が安全
- 過大な正値については、`std::vector` の確保失敗時に C 境界を例外が越えないよう、`try`/`catch(std::bad_alloc)` で捕捉し `nullptr` を返す。CLAUDE.md の「入力データを信用せず事前確保を避ける」考え方と整合させる
- 同種の `WEBRTC_DEFINE_REFCOUNTED_VECTOR` / `WEBRTC_DEFINE_INLINED_VECTOR` の `_new(int size)` についても同一の検証を追加する
- `_vector_get(int index)` / `_vector_set(int index, ...)` の範囲外アクセスは問題の性質が異なる（C++ 例外の C 境界越境ではなく std::vector の未定義動作）ため、本 issue では扱わず別 issue とする
- `_vector_resize(int size)` についても同様の `size <= 0` 検証と `bad_alloc` 捕捉を追加する

## テスト戦略

- 既存の whip.c / whep.c がビルド可能であることを確認する
- C++ 単体テスト（whip/whep のテストバイナリ内）で以下を検証:
  - `_vector_new(-1)` が空ベクタを返すこと
  - `_vector_new(0)` が空ベクタを返すこと
  - `_vector_new(SIZE_MAX)` が `nullptr` を返すこと（bad_alloc 捕捉）

## 後方互換への影響

API シグネチャ（`int size`）は変更されない。内部実装の堅牢性向上であり、後方互換は維持される。

## 完了条件

- `_vector_new` / `_refcounted_vector_new` / `_inlined_vector_new` / `_vector_resize` に
  負値や過大値を渡しても C++ 例外による異常終了が発生しない
- `size <= 0` の場合は空ベクタが返る
- 過大値でメモリ確保に失敗した場合は `nullptr` が返る
- 不正な `size` を渡した際の挙動がヘッダ側のコメントで明確になっている
- `_vector_get` / `_vector_set` の範囲外アクセス問題は別 issue 化されている
