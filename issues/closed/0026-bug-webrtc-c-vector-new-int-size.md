# WEBRTC_DEFINE_VECTOR 系マクロの _new / _resize のサイズ引数型を int から size_t にする

- Priority: Medium
- Polished: 2026-06-08
- Completed: 2026-06-08
- Created: 2026-06-05
- Model: Opus 4.8

## 目的

webrtc_c の `*_vector_new(int size)` / `*_vector_resize(int size)` 系 API のサイズ引数型を
`int` から `size_t` に変更する。`int` では負値を渡せてしまい、`int` から `size_t` への
暗黙変換で符号拡張により極端に大きな値へ化け、`std::vector` のコンストラクタが
`std::length_error` や `std::bad_alloc` を送出し、C 境界を越えた未処理例外で異常終了する。
型レベルで負値を排除することでこの問題を根本解決する。

`std::vector` のコンストラクタ (`explicit vector(size_type count)`) の引数型は `size_type`
(実質 `size_t`) であり、`webrtc/RULES.md` の「元の C++ API のシグネチャに忠実に」の原則や、
0014 の同種変更 (`webrtc_CreateRandomString` の `int` → `size_t`) とも整合する。

## 優先度根拠

正常な使い方では問題は起きないが、サイズ値の出所がデコード結果など信頼できない場合に、
負値による符号拡張で例外送出による異常終了に至る。
C++ 例外が C 境界を越えることでプロセスが abort する経路が現存しており、重要な欠陥である。
常時クラッシュではなく不正入力時の堅牢性の問題であるため Medium とする。

## 現状

`webrtc/src/webrtc_c/common.impl.h:65-70` の `WEBRTC_DEFINE_VECTOR` の `_vector_new` は、
`int size` を `std::vector<cpptype>(size)` にそのまま渡している。

```cpp
#define WEBRTC_DEFINE_VECTOR(type, cpptype)                               \
  WEBRTC_EXPORT struct WEBRTC_CONCAT(type, _vector) *                     \
      WEBRTC_CONCAT(type, _vector_new)(int size) {                        \
    auto vec = new std::vector<cpptype>(size);                            \
    return reinterpret_cast<struct WEBRTC_CONCAT(type, _vector)*>(vec);   \
  }                                                                       \
```

同種の `int size` を受け取るマクロが他にもある。宣言側 (`common.h`) と定義側 (`common.impl.h`)
の双方で `int` が使われている。

| マクロ | 関数 | ファイル:行 |
|--------|------|-------------|
| `WEBRTC_DECLARE_VECTOR` | `_vector_new(int size)` | `common.h:58` |
| | `_vector_resize(int size)` | `common.h:65-66` |
| `WEBRTC_DEFINE_VECTOR` | `_vector_new(int size)` | `common.impl.h:67` |
| | `_vector_resize(int size)` | `common.impl.h:88` |
| `WEBRTC_DECLARE_REFCOUNTED_VECTOR` | `_refcounted_vector_new(int size)` | `common.h:105` |
| | `_refcounted_vector_resize(int size)` | `common.h:113-114` |
| `WEBRTC_DEFINE_REFCOUNTED_VECTOR` | `_refcounted_vector_new(int size)` | `common.impl.h:153` |
| | `_refcounted_vector_resize(int size)` | `common.impl.h:180` |
| `WEBRTC_DECLARE_INLINED_VECTOR` | `_inlined_vector_new(int size)` | `common.h:130` |
| | `_inlined_vector_resize(int size)` | `common.h:137-138` |
| `WEBRTC_DEFINE_INLINED_VECTOR` | `_inlined_vector_new(int size)` | `common.impl.h:212` |
| | `_inlined_vector_resize(int size)` | `common.impl.h:237` |

これらのマクロを使っていて宣言が自動生成されるヘッダも影響を受けるが、再コンパイルで追従する:

- `webrtc/src/webrtc_c/std.h:32` (`WEBRTC_DECLARE_VECTOR(std_string)`)
- `webrtc/src/webrtc_c/api/video_codecs/video_codec.h:115` (`WEBRTC_DECLARE_VECTOR(webrtc_VideoFrameType)`)

## スコープ外

以下の項目は本 issue では扱わず、別途対応する:

- `_vector_get` / `_vector_set` / `_refcounted_vector_get` / `_refcounted_vector_set` /
  `_inlined_vector_get` / `_inlined_vector_set` の `int index` 引数 — 範囲外アクセスの未定義動作であり問題の性質が異なる。
  本変更後 `_vector_new(>INT_MAX)` で生成したベクタは `_vector_get(int)` / `_vector_set(int)` の
  引数型制約によりアクセス不能になるが、この非対称性も本 issue のスコープ外とする
- `_vector_size` / `_refcounted_vector_size` / `_inlined_vector_size` の戻り値型 `int` —
  `size_t` から `int` への縮小変換問題（0028 で部分的に扱われているが、残件は別 issue）。
  本変更で `_vector_new(size_t)` が `INT_MAX` 超の要素数を受け付けるようになるため、
  この問題の影響範囲は拡大する
- `WEBRTC_DEFINE_VECTOR_NO_DEFAULT_CTOR` の `_vector_new(void)` — 引数がないため影響なし
- `SIZE_MAX` など極端に大きな値に対する防護 — `size_t` の本質的な制約であり、C ラッパー全般の設計課題として別途扱う

## 設計方針

### C 側マクロ変更

- `WEBRTC_DECLARE_VECTOR` / `WEBRTC_DEFINE_VECTOR` の `_vector_new` と `_vector_resize` の `int size` → `size_t size`
- `WEBRTC_DECLARE_REFCOUNTED_VECTOR` / `WEBRTC_DEFINE_REFCOUNTED_VECTOR` の `_refcounted_vector_new` と `_refcounted_vector_resize` の `int size` → `size_t size`
- `WEBRTC_DECLARE_INLINED_VECTOR` / `WEBRTC_DEFINE_INLINED_VECTOR` の `_inlined_vector_new` と `_inlined_vector_resize` の `int size` → `size_t size`
- 宣言側 (`common.h`) と定義側 (`common.impl.h`) の両方を変更し、型を一致させる

### Rust 側変更

C API の型変更により bindgen が再生成する FFI 宣言は `usize` に追従する。
これに伴い、FFI を呼び出す Rust ラッパーの引数型も `i32` → `usize` に変更する必要がある。

| ファイル | 行 | 修正内容 |
|----------|-----|----------|
| `src/cxxstd.rs` | 164 | `StringVector::new(size: i32)` → `size: usize` |
| `src/api/peer_connection.rs` | 710 | `IceServerVector::new(size: i32)` → `size: usize` |
| `src/api/rtp.rs` | 344 | `RtpCodecCapabilityVector::new(size: i32)` → `size: usize` |
| `src/api/rtp.rs` | 432-433 | `resize` 内 `i32::try_from(len).unwrap_or(i32::MAX)` を削除し `len` を直接渡す |
| `src/api/rtp.rs` | 1024 | `RtpEncodingParametersVector::new(size: i32)` → `size: usize` |
| `src/api/rtp.rs` | 1058-1059 | `resize` 内 `i32::try_from(len).unwrap_or(i32::MAX)` を削除し `len` を直接渡す |
| `src/api/video_codec_common.rs` | 1715 | `VideoFrameTypeVector::new(size: i32)` → `size: usize` |
| `src/api/video_encoder.rs` | 94-96 | `resize` 内 `i32::try_from(len).unwrap_or(i32::MAX)` を削除し `len` を直接渡す |
| `src/api/video_encoder.rs` | 170-172 | `resize` 内 `i32::try_from(len).unwrap_or(i32::MAX)` を削除し `len` を直接渡す |

C ソース側 (`whip.c`: `_vector_new(0)` 2 回, `_vector_new(3)` 1 回, `whep.c`: 呼び出しなし) は
すべて整数リテラルのため変更不要。

なお、`resize` の `i32::try_from(...).unwrap_or(i32::MAX)` は C API への型変換のための記述であり
防護を意図したものではないが、結果として `i32::MAX` を上限とする弱い防護として機能していた。
本修正でこの上限が取り除かれるが、上限付与は本 issue の責務ではなく別途検討する。

## 後方互換への影響

- C API の ABI が変更される（`size_t` はプラットフォームにより `int` とサイズが異なる可能性がある）
- Rust 公開 API も変更される（`new(size: i32)` → `new(size: usize)`、`resize` の型変換ロジック削除）
- これは後方互換のない変更 (`[CHANGE]`) である
- 外部の C コンシューマーが動的リンクしている場合は再コンパイルが必要
- Rust API の利用者は `new(-1)` 等の呼び出しがコンパイルエラーになる

## CHANGES.md

```
## develop

- [CHANGE] WEBRTC_DEFINE_VECTOR 系マクロの _new / _resize のサイズ引数型を int から size_t に変更する
  - 負値による符号拡張で C++ 例外が C 境界を越える問題を型レベルで防止する
  - Rust ラッパー側の new 引数も i32 → usize に変更する
  - @実装者
```

## テスト戦略

- 既存の whip.c / whep.c がビルド可能であることを確認する
- Cargo の全単体テストが pass することを確認する（`StringVector::new(0)` 等のリテラル呼び出しは型推論で追従）
- `_refcounted_vector_new` / `_inlined_vector_new` / `_refcounted_vector_resize` は呼び出し元がコードベース内に存在しないため、ビルドが通ることの確認のみで十分

## 完了条件

- `common.h` の `WEBRTC_DECLARE_VECTOR` / `WEBRTC_DECLARE_REFCOUNTED_VECTOR` / `WEBRTC_DECLARE_INLINED_VECTOR` の `_new` / `_resize` のサイズ引数型が `size_t` になっている
- `common.impl.h` の `WEBRTC_DEFINE_VECTOR` / `WEBRTC_DEFINE_REFCOUNTED_VECTOR` / `WEBRTC_DEFINE_INLINED_VECTOR` の定義側も `size_t` に揃っている
- 上記 Rust ラッパー 9 箇所の型変更 / `i32::try_from` 削除が完了している
- 宣言と定義の型が一致している
- 既存の whip.c / whep.c がビルド可能で、警告が増えていない
- Cargo の全単体テストが pass する
- スコープ外の項目（`_vector_get` / `_vector_set` の `int index`、`_vector_size` 戻り値の `int`）が issue に明記されている

## 解決方法

以下の 3 つのカテゴリの変更を実施した:

### C 側マクロ変更

- `common.h` の `WEBRTC_DECLARE_VECTOR` / `WEBRTC_DECLARE_REFCOUNTED_VECTOR` / `WEBRTC_DECLARE_INLINED_VECTOR` の `_new` / `_resize` のサイズ引数型を `int` から `size_t` に変更した
- `common.impl.h` の `WEBRTC_DEFINE_VECTOR` / `WEBRTC_DEFINE_REFCOUNTED_VECTOR` / `WEBRTC_DEFINE_INLINED_VECTOR` の定義側も同様に `int` から `size_t` に変更した
- 宣言と定義の型を一致させた

### Rust 側変更

- `src/cxxstd.rs`: `StringVector::new(size: i32)` → `size: usize`
- `src/api/peer_connection.rs`: `IceServerVector::new(size: i32)` → `size: usize`
- `src/api/rtp.rs`: `RtpCodecCapabilityVector::new(size: i32)` → `size: usize`
- `src/api/rtp.rs`: `RtpCodecCapabilityVectorRef::resize` の `i32::try_from(len).unwrap_or(i32::MAX)` を削除し `len` を直接渡す
- `src/api/rtp.rs`: `RtpEncodingParametersVector::new(size: i32)` → `size: usize`
- `src/api/rtp.rs`: `RtpEncodingParametersVector::resize` の `i32::try_from(len).unwrap_or(i32::MAX)` を削除し `len` を直接渡す
- `src/api/video_codec_common.rs`: `VideoFrameTypeVector::new(size: i32)` → `size: usize`
- `src/api/video_encoder.rs`: `resize` の `i32::try_from(len).unwrap_or(i32::MAX)` を削除し `len` を直接渡す (2 箇所)

### CHANGES.md

- `[CHANGE]` エントリを追加した

### テスト

- `cargo test --all-targets` で全 101 件のテストが pass することを確認した
- `cargo check` でビルドが警告なしで完了することを確認した
