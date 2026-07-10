# libwebrtc のデバッグビルドに対応する

- Priority: Medium
- Created: 2026-07-10
- Model: DeepSeek V4 Pro
- Branch: feature/add-debug-build-support
- Polished: 2026-07-10

## 目的

現在のビルドシステムは libwebrtc の Release ビルド済みバイナリをリモートからダウンロードすることしか想定していない。
デバッグビルドのバイナリはリモートに存在しないため、ローカルで webrtc-build をデバッグビルドし、その成果物を利用する仕組みが必要。
環境変数でローカルの webrtc-build プロジェクトルートを指定できるようにし、libwebrtc 本体と C ラッパーをデバッグビルドでコンパイル可能にする。

## 優先度根拠

Medium: ユーザー向けの機能追加ではなく、libwebrtc 内部を含めたデバッグが必要な開発時に有用な機能であるため。ただし prebuilt バイナリではデバッグビルドが提供されておらず、現状のワークアラウンド（手動でのビルド成果物差し替え）は非効率である。

## 現状

### build.rs

`build.rs:864-876` で `CMAKE_BUILD_TYPE` とプロファイルが `"Release"` に固定されている:

```rust
let profile = "release";
config.profile("Release");
config.define("CMAKE_BUILD_TYPE", "Release")
```

`build.rs:169-176` の `should_use_prebuilt()` は `CARGO_FEATURE_SOURCE_BUILD` で `source-build` feature を判断するが、デバッグビルドの概念は存在しない:

```rust
fn should_use_prebuilt() -> bool {
    if env::var("CARGO_FEATURE_SOURCE_BUILD").is_ok() {
        return false;
    }
    true
}
```

### CMakeLists.txt

`webrtc/CMakeLists.txt:143` で `WEBRTC_URL` を構築し、`webrtc/CMakeLists.txt:180-201` でダウンロードしている。常にリモートからのダウンロードが前提であり、ローカルの webrtc-build 出力ディレクトリを指定する仕組みは存在しない。

また `webrtc/CMakeLists.txt:212-217` では、ダウンロードしたアーカイブ内の `VERSIONS` ファイルから LLVM 関連のコミットハッシュを読み取り、`webrtc/CMakeLists.txt:237-294` で WebRTC 用の Clang と libc++ ヘッダーをダウンロードしている。この LLVM セットアップは WebRTC ダウンロードの成否に依存しており、ダウンロードをスキップする場合は別途 LLVM の取得方法を定義する必要がある。

### Windows 特有の問題

`webrtc/CMakeLists.txt:563` と `webrtc/CMakeLists.txt:801` では `NDEBUG` が無条件に定義されている。また `webrtc/CMakeLists.txt:565` では `_ITERATOR_DEBUG_LEVEL=0` がハードコードされている。libwebrtc は `NDEBUG` の有無で ABI が変わるヘッダーファイルを含むため、デバッグビルド時にはこれらの定義を削除または条件付きにする必要がある。`_ITERATOR_DEBUG_LEVEL` は MSVC の STL コンテナのメモリレイアウトを決定するため、Debug/Release 間で値が異なると ABI 非互換が発生する。

## 設計方針

以下の 2 つの制御を分離して導入する:

1. **libwebrtc の取得元**: 環境変数 `WEBRTC_BUILD_ROOT` でローカルの webrtc-build プロジェクトルートを指定する
2. **ビルドタイプ (Debug/Release)**: `debug-build` Cargo feature で指定する

### 環境変数 `WEBRTC_BUILD_ROOT`

webrtc-build プロジェクトのルートディレクトリを指定する。
この変数が設定されているとき:

- **build.rs**: `main()` の先頭でエラーチェックを行う（後述の `### source-build feature 未指定時のエラー` 参照）。エラーチェック通過後、`should_use_prebuilt()` は `WEBRTC_BUILD_ROOT` の有無も判定条件に加え、設定時は `false` を返す（ソースビルドへ進む）
- **build.rs**: 環境変数の値を CMake に `-DWEBRTC_BUILD_ROOT=...` として渡す。相対パスが指定された場合は `CARGO_MANIFEST_DIR` を基準に `fs::canonicalize()` で絶対パスへ正規化した上で CMake へ渡す
- **CMakeLists.txt**: `WEBRTC_BUILD_ROOT` が定義されている場合、WebRTC のダウンロードをスキップし、以下からパスを解決する

webrtc-build のディレクトリ構造:

```
../webrtc-build/
  _build/
    ubuntu-24.04_x86_64/
      debug/     # libwebrtc.a 等のビルド済みバイナリ
      release/   # libwebrtc.a 等のビルド済みバイナリ
  _source/
    ubuntu-24.04_x86_64/
      webrtc/
        src/     # webrtc のヘッダーファイル群
```

CMakeLists.txt では、`WEBRTC_C_TARGET`（例: `ubuntu-24.04_x86_64`）とビルドタイプ（`Debug` / `Release`）から以下のパスを導出する:

- バイナリ: `${WEBRTC_BUILD_ROOT}/_build/${WEBRTC_C_TARGET}/${BUILD_TYPE}/`
- ヘッダー: `${WEBRTC_BUILD_ROOT}/_source/${WEBRTC_C_TARGET}/webrtc/src/`

### CMakeLists.txt の具体的な変更内容

`WEBRTC_BUILD_ROOT` 設定時、`CMakeLists.txt` への変更は以下の全項目を含む:

1. `WEBRTC_BUILD_VERSION` / `WEBRTC_BASE_URL` の必須チェック (`CMakeLists.txt:9-14`) を `WEBRTC_BUILD_ROOT` 未設定時のみに変更する
2. `WEBRTC_URL` 構築 (`CMakeLists.txt:143`) を同様に条件付きにする
3. WebRTC アーカイブのダウンロードブロック (`CMakeLists.txt:180-201`) を `WEBRTC_BUILD_ROOT` 未設定時のみ実行する
4. `WEBRTC_INCLUDE_DIR` (`CMakeLists.txt:204`) を `${WEBRTC_BUILD_ROOT}/_source/${WEBRTC_C_TARGET}/webrtc/src/` に設定する
5. `WEBRTC_LIBRARY_DIR` (`CMakeLists.txt:206-208`) を `${WEBRTC_BUILD_ROOT}/_build/${WEBRTC_C_TARGET}/${BUILD_TYPE}/` に設定する
6. `third_party` 系のインクルードパス (`CMakeLists.txt:421-431`) が上記の `WEBRTC_INCLUDE_DIR` 配下に存在するか確認する。存在しない場合、webrtc-build 側のヘッダー配置規則に合わせてインクルードパスを調整する
7. **VERSIONS ファイルの読み取り** (`CMakeLists.txt:212-217`) を `${WEBRTC_BUILD_ROOT}/_source/${WEBRTC_C_TARGET}/webrtc/VERSIONS` から行う。ファイルが存在しない場合は `WEBRTC_USE_WEBRTC_CLANG` と `WEBRTC_USE_WEBRTC_LIBCXX` を強制的に `FALSE` にし、システムのコンパイラを使用する
8. **NDEBUG の条件付き定義**: Windows の `WEBRTC_CPP_TARGETS` (`CMakeLists.txt:563`) と `WEBRTC_C_TARGETS` (`CMakeLists.txt:801`) にハードコードされた `NDEBUG` を `CMAKE_BUILD_TYPE=Debug` 時には削除する。同様に `_ITERATOR_DEBUG_LEVEL=0` (`CMakeLists.txt:565`) も Debug 時は定義しない（MSVC のデフォルト値 `_ITERATOR_DEBUG_LEVEL=2` に任せる）。これにより Debug ビルドした libwebrtc と C ラッパーの ABI を一致させる

`WEBRTC_BUILD_ROOT` は Debug/Release の切り替えとは無関係であり、ローカルの Release ビルドを利用する用途にも使える。

### ビルドタイプ (Debug/Release)

libwebrtc は `NDEBUG` の有無で ABI が変わるヘッダーファイルを含む。そのため webrtc-build と C ラッパー（`./webrtc`）のプロファイルは同一でなければならない。

新しく `debug-build` Cargo feature を追加し、この feature の有無で CMake ビルドタイプを決定する:

- `debug-build` 有効 → `profile = "debug"; config.profile("Debug"); CMAKE_BUILD_TYPE=Debug`
- `debug-build` 無効（デフォルト） → `profile = "release"; config.profile("Release"); CMAKE_BUILD_TYPE=Release`

Cargo の `PROFILE` 環境変数には連動させない。ユーザーが明示的に `--features debug-build` を指定した場合のみ Debug ビルドとなる。

### bindgen の NDEBUG マクロ対応

`build.rs:1314` の `generate_bindings()` は libclang 経由でヘッダーをパースして Rust バインディングを生成する。libwebrtc のヘッダーが `#ifdef NDEBUG` / `#ifndef NDEBUG` で構造体レイアウトや関数シグネチャを変える場合、`generate_bindings()` に `-DNDEBUG` または `-UNDEBUG` を渡す必要が生じる。`debug-build` feature 有効時は `bindgen::Builder` に `-UNDEBUG` を追加し、無効時は従来通り `-DNDEBUG` を維持する。

### `source-build` feature 未指定時のエラー

`WEBRTC_BUILD_ROOT` が設定されているにもかかわらず `source-build` feature が無効の場合、ビルドエラーとして停止する。エラーメッセージでは「WEBRTC_BUILD_ROOT が設定されていますが、source-build feature が有効になっていません。--features source-build を指定してください」と案内する。

### prebuilt 利用かつ `debug-build` feature 有効時のエラー

prebuilt バイナリは Release のみ提供されているため、`WEBRTC_BUILD_ROOT` が未設定（prebuilt 利用）かつ `debug-build` feature が有効な場合は、ビルドエラーとして停止する。エラーメッセージでは「デバッグビルドの prebuilt バイナリは提供されていません。ローカルの webrtc-build を利用するために WEBRTC_BUILD_ROOT 環境変数を設定してください」と案内する。

### 組み合わせ

| WEBRTC_BUILD_ROOT | source-build feature | debug-build feature | 取得元 | CMake ビルドタイプ |
|---|---|---|---|---|
| 未設定 | 無効 | 無効 | リモート prebuilt | Release |
| 未設定 | 無効 | 有効 | -- | **エラー** |
| 設定 | 無効 | -- | -- | **エラー** |
| 設定 | 有効 | 無効 | ローカル | Release |
| 設定 | 有効 | 有効 | ローカル | Debug |

### 制約と注意点

- **`local-export` feature との共存**: `local-export` feature（`build.rs:1114-1146`）は CMake ビルド出力先へのシンボリックリンクを作成する。`debug-build` を有効にすると `out_dir/_build/${target}/debug/` と `out_dir/_build/${target}/release/` の両方が生成されうるが、`local-export` は親ディレクトリへのリンクを作成するため、両方のプロファイルがリンク先で参照可能になる。競合は発生せず、追加の対応は不要。
- **webrtc-build の内部構造への依存**: 本設計は webrtc-build プロジェクトの内部ディレクトリ構造（`_build/${target}/${profile}/`, `_source/${target}/webrtc/src/`）に依存している。webrtc-build 側でこの構造が変更された場合、CMakeLists.txt と build.rs のパス導出ロジックを追従更新する必要がある。
- **cargo rebuild の検出限界**: `rerun-if-env-changed=WEBRTC_BUILD_ROOT` を追加しても、環境変数の値が変わらないまま webrtc-build の成果物だけが更新された場合、cargo は再ビルドをトリガーしない。ユーザーは成果物更新後に `cargo clean` を実行する必要がある。

### 変更対象

- `Cargo.toml`: `debug-build` feature を追加（`[features]` セクション）
- `build.rs`: `main()` の先頭にエラーチェックを追加。`WEBRTC_BUILD_ROOT` 設定 + `source-build` 無効の組み合わせ、`debug-build` 有効 + `WEBRTC_BUILD_ROOT` 未設定の組み合わせの 2 パターン
- `build.rs`: `should_use_prebuilt()` に `WEBRTC_BUILD_ROOT` のチェックを追加。設定時は `false` を返す
- `build.rs`: `build_webrtc_c()` のプロファイル設定を `debug-build` feature の有無で分岐。`profile` 変数、`config.profile()`、`CMAKE_BUILD_TYPE` を条件付きで切り替える
- `build.rs`: `build_webrtc_c()` から CMake へ `WEBRTC_BUILD_ROOT` を伝達（絶対パスに正規化した上で `-DWEBRTC_BUILD_ROOT=...` として渡す）
- `build.rs`: `generate_bindings()` に `debug-build` feature 有効時は `-UNDEBUG` を `clang_arg` として追加する
- `build.rs`: `main()` に `rerun-if-env-changed=WEBRTC_BUILD_ROOT` と `rerun-if-env-changed=CARGO_FEATURE_DEBUG_BUILD` を追加
- `webrtc/CMakeLists.txt`: `WEBRTC_BUILD_ROOT` 定義時の分岐を追加（`### CMakeLists.txt の具体的な変更内容` の全項目を参照）
- `webrtc/CMakeLists.txt`: Windows 向けの `NDEBUG` / `_ITERATOR_DEBUG_LEVEL=0` を `CMAKE_BUILD_TYPE=Debug` 時に定義しないように条件付き化
- `CHANGES.md`: `## develop` に `[ADD] libwebrtc のデバッグビルドに対応する` エントリを追加

### 利用手順（ユーザー向け）

**通常のビルド（変更なし）:**

```sh
cargo build --release
```

**ローカル webrtc-build のリリースビルドを利用する場合:**

```sh
export WEBRTC_BUILD_ROOT=../webrtc-build
cargo build --release --features source-build
```

**ローカル webrtc-build のデバッグビルドを利用する場合:**

1. 別途 webrtc-build をローカルでデバッグビルドする（本 issue のスコープ外）
2. 環境変数を設定する: `export WEBRTC_BUILD_ROOT=../webrtc-build`
3. `cargo build --features source-build,debug-build` を実行する（Cargo の dev プロファイルでビルドする。`--release` は指定しないこと）

**prebuilt で debug-build を指定した場合（エラー）:**

```sh
cargo build --features debug-build
# → エラー: デバッグビルドの prebuilt バイナリは提供されていません。
#           WEBRTC_BUILD_ROOT 環境変数を設定してください。
```

## テスト戦略

本変更はビルドシステムの変更であるため、ユニットテストや PBT の対象にはならない。以下の手動確認をもってテストとする:

1. **通常ビルドの継続確認**: `WEBRTC_BUILD_ROOT` 未設定、`source-build` 無効、`debug-build` 無効で `cargo build --release` が成功すること
2. **エラーケース 1 の確認**: `WEBRTC_BUILD_ROOT` 設定 + `source-build` 無効でビルドがエラー停止し、適切なエラーメッセージが表示されること
3. **エラーケース 2 の確認**: `WEBRTC_BUILD_ROOT` 未設定 + `debug-build` 有効でビルドがエラー停止し、適切なエラーメッセージが表示されること
4. **デバッグビルドの確認**: `WEBRTC_BUILD_ROOT` 設定 + `source-build` + `debug-build` で CMake ビルドが `CMAKE_BUILD_TYPE=Debug` で実行され、`NDEBUG` が定義されず、ビルドが成功すること
5. **ローカル Release ビルドの確認**: `WEBRTC_BUILD_ROOT` 設定 + `source-build` 有効 + `debug-build` 無効で CMake ビルドが `CMAKE_BUILD_TYPE=Release` で実行され、成功すること

テストの前提として、確認する各環境にあらかじめ webrtc-build の成果物（バイナリ・ヘッダー・VERSIONS ファイル）を配置しておく必要がある。`WEBRTC_BUILD_ROOT` には絶対パスを使用することが推奨される。

## 完了条件

- `Cargo.toml` に `debug-build` feature が追加されている
- `WEBRTC_BUILD_ROOT` 設定済みかつ `source-build` feature 無効時にエラーで停止すること（適切なエラーメッセージ付き）
- `WEBRTC_BUILD_ROOT` 未設定かつ `debug-build` feature 有効時にエラーで停止すること（適切なエラーメッセージ付き）
- 環境変数 `WEBRTC_BUILD_ROOT` が設定されているとき、CMakeLists.txt が指定ディレクトリ配下の `_source/${WEBRTC_C_TARGET}/webrtc/src/` をインクルードパス、`_build/${WEBRTC_C_TARGET}/${BUILD_TYPE}/` をライブラリパスとして利用し、ダウンロードをスキップすること
- `WEBRTC_BUILD_ROOT` 設定時は `_source/${WEBRTC_C_TARGET}/webrtc/VERSIONS` から LLVM 情報を読み取ること。VERSIONS ファイルが存在しない場合はシステムコンパイラを使用すること
- `WEBRTC_BUILD_ROOT` に相対パスが指定された場合、`CARGO_MANIFEST_DIR` を基準に絶対パスへ正規化して CMake へ渡すこと
- `debug-build` feature 有効時に `CMAKE_BUILD_TYPE=Debug`、無効時に `CMAKE_BUILD_TYPE=Release` でビルドされること
- `debug-build` feature 有効時は Windows 向けの `NDEBUG` 定義と `_ITERATOR_DEBUG_LEVEL=0` 定義を削除し、Debug ビルドした libwebrtc と ABI を一致させること
- `debug-build` feature 有効時は `generate_bindings()` に `-UNDEBUG` を渡し、Debug ビルドの libwebrtc ヘッダーと一致するバインディングを生成すること
- `source-build` feature 未指定かつ `WEBRTC_BUILD_ROOT` 未設定かつ `debug-build` 無効の通常ビルドが引き続き成功すること
- `rerun-if-env-changed=WEBRTC_BUILD_ROOT` と `rerun-if-env-changed=CARGO_FEATURE_DEBUG_BUILD` が `main()` に追加されていること
- `CHANGES.md` の `## develop` に `[ADD] libwebrtc のデバッグビルドに対応する` エントリが追加されている
