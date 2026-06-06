# webrtc_CreateRandomString の長さ引数を size_t にする

- Priority: High
- Created: 2026-06-05
- Model: Opus 4.8
- Polished: 2026-06-06

## 目的

C ラッパー `webrtc_CreateRandomString` は長さ引数を `int` で受け取るが、ラップ対象の
`webrtc::CreateRandomString`（`<rtc_base/crypto_random.h>`）は `size_t` を取る。
`int` で受けると負値を渡せてしまい、`int` から `size_t` への暗黙変換で符号拡張により極端に
大きな値へ化け、`std::string` コンストラクタが `std::bad_alloc` を送出し、
C 境界を越えた未処理例外で異常終了する。
`webrtc/RULES.md` の「元の C++ API のシグネチャ・名前に忠実に」にも反しているため、
引数型を `size_t` に揃える。

## 優先度根拠

負値を `size_t` に暗黙変換することによる過大確保・未処理例外は堅牢性の重大な欠陥である。
C++ 例外が C 境界を越えることでプロセスが abort する経路が現存しており、優先度は High とする。

## 現状

C ラッパーの宣言・定義とも長さ引数の型が `int` になっている。

`webrtc/src/webrtc_c/rtc_base/crypto_random.h:14`:
```c
WEBRTC_EXPORT struct std_string_unique* webrtc_CreateRandomString(int length);
```

`webrtc/src/webrtc_c/rtc_base/crypto_random.cc:17-18`:
```cpp
WEBRTC_EXPORT struct std_string_unique* webrtc_CreateRandomString(int length) {
  auto str = std::make_unique<std::string>(webrtc::CreateRandomString(length));
```

Rust ラッパー `src/rtc_base/crypto_random.rs:6-17`:
```rust
pub fn random_string(len: i32) -> String {
    let raw = unsafe { ffi::webrtc_CreateRandomString(len) };
    CxxString::from_unique(
        NonNull::new(raw).expect("BUG: webrtc_CreateRandomString が null を返しました"),
    )
    .to_string()
    .expect("BUG: webrtc_CreateRandomString が不正な UTF-8 文字列を返しました")
}
pub fn random_bytes(len: usize) -> Vec<u8> {
    random_string(len as i32).into_bytes()
}
```

`random_bytes` は内部で `random_string(len as i32)` を呼んでおり、`usize` 値が
`i32::MAX` を超えた場合に切り詰めが発生する二次的な問題も内包している。

## 修正対象ファイル一覧

| ファイル | 行 | 修正内容 |
|----------|-----|----------|
| `webrtc/src/webrtc_c/rtc_base/crypto_random.h` | 14 | `int length` → `size_t length` |
| `webrtc/src/webrtc_c/rtc_base/crypto_random.cc` | 17 | `int length` → `size_t length` |
| `webrtc/src/whip.c` | 1339, 1355 | リテラル `16` は暗黙変換で通るため変更不要。`-Wconversion` 警告が出る場合は `16u` 化を検討 |
| `src/rtc_base/crypto_random.rs` | 6 | `random_string(len: i32)` → `random_string(len: usize)` |
| `src/rtc_base/crypto_random.rs` | 17 | `random_string(len as i32)` のキャストを削除（`usize` → `size_t` が直接渡る） |
| `src/lib.rs` | 30 | `pub use` の再エクスポートは自動追従するため変更不要（コンパイル確認のみ） |
| `examples/whip/src/main.rs` | 574, 580 | リテラル `16` は推論で通るため変更不要（コンパイル確認のみ） |
| `src/tests.rs` | 62, 68 | リテラル `8`, `16` は推論で通るため変更不要（コンパイル確認のみ） |

## 修正範囲と注意点

### C 側

- `crypto_random.h` の宣言と `crypto_random.cc` の定義の両方で、長さ引数の型を `int` から
  `size_t` に変更する
- `webrtc/RULES.md`（「元の C++ API のシグネチャ・名前に忠実に」）に従い、
  `webrtc::CreateRandomString(size_t)` と一致させる
- `webrtc/src/whip.c:1339,1355` の `webrtc_CreateRandomString(16)` 呼び出しは
  リテラル `16` のため暗黙変換で通る。ただしコンパイラの警告フラグ次第では
  `-Wconversion` による警告が発生しうるため、コンパイル確認時に警告の有無をチェックし、
  警告が出る場合は `16u` または `(size_t)16` への変更を検討する

### Rust 側

- FFI バインディングは bindgen により `crypto_random.h` から自動生成されるため、
  C ヘッダー変更後の再ビルドで `ffi::webrtc_CreateRandomString` のシグネチャは自動追従する
  （Rust 側で手動変更する FFI 宣言はない）
- **prebuilt 利用時の注意**: デフォルトの prebuilt ビルドでは、生成済みの `bindings.rs` が
  アーカイブからコピーされる。C ヘッダー変更後は prebuilt アーカイブ側の `bindings.rs` も
  再生成してリリースする必要がある
- `size_t` は bindgen のデフォルト動作により Rust の `usize` へマッピングされる

### 代替案の非採用理由

- **`unsigned int` で十分では？**: `size_t` は元の C++ API の型であり、
  `webrtc/RULES.md` のルールに従うため変更しない
- **ラッパー内で上限チェックを追加しては？**: RULES.md の「薄いラッパー」原則に反する。
  SIZE_MAX 対策は本 issue のスコープ外とし、 C ラッパー全般の設計課題として別途扱う

## エッジケース

| ケース | 期待される挙動 |
|--------|---------------|
| `length = 0` | `random_string(0)` は空文字列を返す。 `random_bytes(0)` は空 Vec を返す |
| `length > i32::MAX`（修正前） | `random_bytes` 経由で渡すと `len as i32` で切り詰められ、要求より短い文字列が返る（**本修正で解消**） |
| 負値（修正前） | `int` → `size_t` の暗黙変換で符号拡張され、SIZE_MAX 相当の値になる（**本修正で解消**） |
| `length = SIZE_MAX` | `std::bad_alloc` が C 境界を越える（本修正後も残存する既知の制約） |

## 後方互換への影響

- Rust 公開 API `pub fn random_string(len: i32)` → `pub fn random_string(len: usize)` は
  後方互換のない変更（`[CHANGE]`）である
- `CHANGES.md` の `## develop` セクションに以下を追記する:
  ```
  - [CHANGE] random_string の引数型を i32 から usize に変更する
    - webrtc_CreateRandomString の C API 引数型 (size_t) と一致させる
    - @実装者
  ```
- `random_bytes` のシグネチャは変わらない
- C 側の `webrtc_CreateRandomString` の ABI 変更は共有ライブラリのエクスポートシンボルに影響する。
  本ライブラリの外部コンシューマが動的リンクしている場合は再コンパイルが必要

## テスト戦略

### 既存テストの継続確認

- `src/tests.rs:60-69` の `random_string_has_requested_length` /
  `random_bytes_length_matches` が引き続き pass することを確認する
- テストコード内に `i32` を前提とした記述がないか確認する（現状なし）

### 追加する単体テスト

`src/tests.rs` に以下を追加する:

```rust
#[test]
fn random_string_zero_length() {
    let s = random_string(0);
    assert_eq!(s.len(), 0, "長さ 0 を指定したら空文字列が返ること");
}

#[test]
fn random_bytes_zero_length() {
    let b = random_bytes(0);
    assert_eq!(b.len(), 0, "長さ 0 を指定したら空 Vec が返ること");
}

#[test]
fn random_string_usize_values() {
    // usize → size_t がキャストなしで直接渡ることの検証。
    // i32::MAX を超える値で実際にメモリ確保すると OOM のため、
    // 実用的な上限値 65536 で検証する。
    let s = random_string(65536);
    assert_eq!(s.len(), 65536, "65536 バイトの文字列が返ること");
}
```

### PBT の推奨（別途インフラ整備後に実施）

プロジェクトに PBT インフラ（`pbt/Cargo.toml`、`pbt/tests/`）が整備された後に以下を推奨する:

- `pbt/tests/prop_rtc_base/main.rs` に `mod crypto_random;` サブモジュールを追加
- strategy: `0usize..=65536`（SIZE_MAX を含む全範囲の `any::<usize>()` は
  `std::bad_alloc` による abort でプロセスごと落ちるため、安全な上限を設定する）
- プロパティ: `random_string(len).len() == len` および `random_bytes(len).len() == len`

### Fuzzing の推奨（別途インフラ整備後に実施）

プロジェクトに cargo-fuzz インフラ（`fuzz/Cargo.toml`、`fuzz/fuzz_targets/`）が
整備された後に以下を推奨する:

- fuzz ターゲット: 任意の `usize` 値で `random_string` / `random_bytes` が
  `std::bad_alloc` 以外でパニックしないことを検証する
- `cargo fuzz` は子プロセス単位で fuzz するため、SIZE_MAX 入力による abort は
  クラッシュとして報告される点に注意（SIZE_MAX は既知の制約として許容する）

## スコープ外

- `webrtc::CreateRandomString` の他オーバーロード（`(size_t, std::string*)`、
  `(size_t, absl::string_view, std::string*)`）— 本修正では C ラッパー化していない
- `random_bytes` が `random_string` の薄いラッパーになっていない問題（RULES.md 違反の可能性）—
  本 issue では型変更のみ行い、関数の存廃は別 issue で扱う
- `SIZE_MAX` 等の極端な値に対する防護— C ラッパー全般の設計課題として別途扱う

## 完了条件

- `webrtc_CreateRandomString` の長さ引数が `int` から `size_t` に変わり、負値を渡せなくなる
- 受け取った `size_t` がそのまま `webrtc::CreateRandomString` へ渡る
- 宣言（`.h`）と定義（`.cc`）の型が一致する
- Rust ラッパー `random_string` の引数型が `usize` になり、`random_bytes` 内の
  `len as i32` キャストが削除され、`random_string(len)` が直接呼ばれていること
- すべての呼び出し元（C: `whip.c`、Rust: `examples/whip`、`tests.rs`）がコンパイル可能である
- 既存テスト `random_string_has_requested_length` / `random_bytes_length_matches` が pass する
- 新規追加した単体テストが pass する
