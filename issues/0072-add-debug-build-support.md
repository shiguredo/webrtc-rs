# libwebrtc のデバッグビルドに対応する

- Priority: Medium
- Created: 2026-07-10
- Completed: 2026-08-19
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

`webrtc/CMakeLists.txt:563` と `webrtc/CMakeLists.txt:801` では `NDEBUG` が無条件に定義されている。libwebrtc は `NDEBUG` の有無で ABI が変わるヘッダーファイルを含むため、デバッグビルド時には `NDEBUG` を定義してはならない。ただし CMake は `Release`（および `RelWithDebInfo` / `MinSizeRel`）の `CMAKE_<LANG>_FLAGS_<CONFIG>` に `/DNDEBUG` を自動付与し `Debug` には付与しない。そのためハードコードされた `NDEBUG` は Release では冗長・Debug では有害である。

Chromium/WebRTC のビルド設定（`build/config/BUILD.gn` の `config("debug")`）は Windows でイテレータデバッグを常に無効化する（`_HAS_ITERATOR_DEBUGGING=0`）ため、libwebrtc は Debug ビルドであってもイテレータデバッグレベル 0 相当の ABI になる。Debug 時に MSVC のデフォルト（`_DEBUG` により `_ITERATOR_DEBUG_LEVEL=2`）へ戻すと ABI 非互換が発生する。

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

CMakeLists.txt では、`WEBRTC_C_TARGET`（例: `ubuntu-24.04_x86_64`）とビルドタイプ（`Debug` / `Release`）から以下のパスを導出する。

`CMAKE_BUILD_TYPE` は `Debug` / `Release`（大文字先頭）だが、webrtc-build の出力ディレクトリ名は `debug` / `release`（小文字）である。そのため CMakeLists.txt 内で `string(TOLOWER ${CMAKE_BUILD_TYPE} BUILD_TYPE)` により小文字化してからパス構築に使用する:

- バイナリ: `${WEBRTC_BUILD_ROOT}/_build/${WEBRTC_C_TARGET}/${BUILD_TYPE}/`
- ヘッダー: `${WEBRTC_BUILD_ROOT}/_source/${WEBRTC_C_TARGET}/webrtc/src/`

### CMakeLists.txt の具体的な変更内容

`WEBRTC_BUILD_ROOT` 設定時、`CMakeLists.txt` への変更は以下の全項目を含む:

1. `WEBRTC_BUILD_VERSION` / `WEBRTC_BASE_URL` の必須チェック (`CMakeLists.txt:9-14`) を `WEBRTC_BUILD_ROOT` 未設定時のみに変更する
2. `WEBRTC_ARCHIVE_NAME` 決定ブロック (`CMakeLists.txt:121-141`) と `WEBRTC_URL` 構築 (`CMakeLists.txt:143`) を同様に `WEBRTC_BUILD_ROOT` 未設定時のみ実行する。`WEBRTC_ARCHIVE_NAME` ブロックの最終 `else()` は `FATAL_ERROR` を伴うため、未設定時以外はスキップしないと不要なエラーが発生する
3. WebRTC アーカイブのダウンロードブロック (`CMakeLists.txt:180-201`) を `WEBRTC_BUILD_ROOT` 未設定時のみ実行する
4. `WEBRTC_INCLUDE_DIR` (`CMakeLists.txt:204`) を `${WEBRTC_BUILD_ROOT}/_source/${WEBRTC_C_TARGET}/webrtc/src/` に設定する
5. `WEBRTC_LIBRARY_DIR` (`CMakeLists.txt:206-208`) を `${WEBRTC_BUILD_ROOT}/_build/${WEBRTC_C_TARGET}/${BUILD_TYPE}/` に設定する。`android_arm64` ターゲットでは、webrtc-build の成果物配置に応じて `arm64-v8a` サブディレクトリが必要かどうかを実装時に確認し、必要に応じてパスを調整する。`BUNDLE_STATIC_LIBS` (`CMakeLists.txt:441`) のパス構築が新しい `WEBRTC_LIBRARY_DIR` で正しく解決されることも確認する
6. `third_party` 系のインクルードパス (`CMakeLists.txt:421-431`) の解決方法は以下のとおり:
    - 以下のパスを存在確認なしで無条件に追加する:
      - `${WEBRTC_INCLUDE_DIR}/third_party/abseil-cpp`
      - `${WEBRTC_INCLUDE_DIR}/third_party/boringssl/src/include`
      - `${WEBRTC_INCLUDE_DIR}/third_party/libyuv/include`
      - `${WEBRTC_INCLUDE_DIR}/third_party/zlib`
    - `${WEBRTC_INCLUDE_DIR}/sdk/objc` と `${WEBRTC_INCLUDE_DIR}/sdk/objc/base` は `WEBRTC_BUILD_ROOT` 設定時には追加しない。これらの ObjC 用ヘッダーは webrtc-build のソースツリー内では `src/` 配下ではなく `sdk/` 直下に存在し、`WEBRTC_INCLUDE_DIR`（`src/`）の相対パスでは解決できないため
7. **LLVM と libc++ の取得**: `WEBRTC_BUILD_ROOT` 設定時は VERSIONS ファイルの読み取り (`CMakeLists.txt:212-217`) と LLVM のダウンロード (`CMakeLists.txt:222-294`) をスキップする。webrtc のソースツリー内に LLVM と libc++ が既に含まれているため、以下のパスを直接利用する。各パスは設定前に存在確認を行い、存在しない場合は `message(FATAL_ERROR ...)` で停止する:
    - Clang コンパイラ: `${WEBRTC_INCLUDE_DIR}/third_party/llvm-build/Release+Assets/bin/clang` および `clang++`。`CMAKE_C_COMPILER` / `CMAKE_CXX_COMPILER` をこのパスで上書きする
    - `llvm-ar`: ARMv8 クロスコンパイル用に `${WEBRTC_INCLUDE_DIR}/third_party/llvm-build/Release+Assets/bin/llvm-ar` を使用する。`CMAKE_AR` (`CMakeLists.txt:323`) も同様に上書きする
    - libc++ ヘッダー: `LIBCXX_INCLUDE_DIR` (`CMakeLists.txt:307`) を `${WEBRTC_INCLUDE_DIR}/third_party/libc++/src/include` に設定する。これにより既存の `target_compile_options` の `-nostdinc++` / `-isystem` ブロック (`CMakeLists.txt:443-453`) が正しいパスを参照する
    - libc++ の `__config_site` と `__assertion_handler` についても `${WEBRTC_INCLUDE_DIR}/third_party/libc++/src/include/` 内に存在することを確認し、存在しない場合はエラー停止する。既存コード (`CMakeLists.txt:280-289`) のような buildtools からのコピーは不要

    なお、Windows ターゲットでは `WEBRTC_USE_WEBRTC_CLANG` / `WEBRTC_USE_WEBRTC_LIBCXX` は引き続き `FALSE` のままとする（`CMakeLists.txt:164-167` の既存設定を維持）。Windows では MSVC と標準ライブラリを使用する。
8. **NDEBUG のハードコード削除**: Windows の `WEBRTC_CPP_TARGETS` (`CMakeLists.txt:563`) と `WEBRTC_C_TARGETS` (`CMakeLists.txt:801`) にハードコードされた `NDEBUG` を削除し、`CMAKE_BUILD_TYPE` に委ねる（Release では CMake が自動で `/DNDEBUG` を付与し、Debug では付与しない）。一方 `_ITERATOR_DEBUG_LEVEL=0` (`CMakeLists.txt:565`) は削除・条件付き化の対象とせず、全ビルドタイプで常時維持する。これにより Debug ビルドした libwebrtc（イテレータデバッグ無効・`NDEBUG` なし）と C ラッパーの ABI を一致させる

`WEBRTC_BUILD_ROOT` は Debug/Release の切り替えとは無関係であり、ローカルの Release ビルドを利用する用途にも使える。

### ビルドタイプ (Debug/Release)

libwebrtc は `NDEBUG` の有無で ABI が変わるヘッダーファイルを含む。そのため webrtc-build と C ラッパー（`./webrtc`）のプロファイルは同一でなければならない。

新しく `debug-build` Cargo feature を追加し、この feature の有無で CMake ビルドタイプを決定する:

- `debug-build` 有効 → `profile = "debug"; config.profile("Debug"); CMAKE_BUILD_TYPE=Debug`
- `debug-build` 無効（デフォルト） → `profile = "release"; config.profile("Release"); CMAKE_BUILD_TYPE=Release`

Cargo の `PROFILE` 環境変数には連動させない。ユーザーが明示的に `--features debug-build` を指定した場合のみ Debug ビルドとなる。

### bindgen の NDEBUG マクロ対応

`build.rs:1314` の `generate_bindings()` は libclang 経由でヘッダーをパースして Rust バインディングを生成する。libwebrtc のヘッダーが `#ifdef NDEBUG` / `#ifndef NDEBUG` で構造体レイアウトや関数シグネチャを変える場合、`generate_bindings()` に `-DNDEBUG` または `-UNDEBUG` を渡す必要が生じる。`debug-build` feature 有効時は `bindgen::Builder` に `-UNDEBUG` を追加する。無効時（Release ビルド）は新たに `-DNDEBUG` を追加する。

なお、現在の `generate_bindings()` (`build.rs:1314-1363`) は `-DNDEBUG` を明示的に渡しておらず、prebuilt の `bindings.rs` も `-DNDEBUG` なしで生成されている。そのため `source-build` 単独利用（`debug-build` 無効）時に `-DNDEBUG` を追加することで、prebuilt のバインディングと生成結果に差異が生じる可能性がある。実装時には両者の差分を確認し、構造体レイアウトの不一致がないことを検証すること。

### `source-build` feature 未指定時のエラー

`WEBRTC_BUILD_ROOT` が設定されているにもかかわらず `source-build` feature が無効の場合、ビルドエラーとして停止する。これは、prebuilt バイナリには C ラッパー（`webrtc_c`）のビルド済みバイナリと bindgen 生成済み `bindings.rs` が含まれており（`build.rs:548-590`）、ローカルの libwebrtc バイナリだけを差し替えることができないためである。ローカルの libwebrtc と組み合わせるには、C ラッパーをソースから再ビルドし bindgen でバインディングを再生成する必要がある。

エラーメッセージでは「WEBRTC_BUILD_ROOT が設定されていますが、source-build feature が有効になっていません。ローカルの libwebrtc と組み合わせるには C ラッパーのソースビルドが必要です。--features source-build を指定してください」と案内する。

### prebuilt 利用かつ `debug-build` feature 有効時のエラー

prebuilt バイナリは Release のみ提供されており、またリモートからダウンロードされる libwebrtc も Release のみである。そのため `WEBRTC_BUILD_ROOT` が未設定（ローカルの webrtc-build 成果物が利用できない）かつ `debug-build` feature が有効な場合は、`source-build` feature の有無にかかわらずビルドエラーとして停止する。エラーメッセージでは「debug-build を利用するには WEBRTC_BUILD_ROOT 環境変数でローカルの webrtc-build プロジェクトルートを指定してください」と案内する。

### 組み合わせ

| WEBRTC_BUILD_ROOT | source-build feature | debug-build feature | 取得元 | CMake ビルドタイプ |
|---|---|---|---|---|---|
| 未設定 | 無効 | 無効 | リモート prebuilt | Release |
| 未設定 | 無効 | 有効 | -- | **エラー** |
| 未設定 | 有効 | 無効 | ローカル（C ラッパーのみソースビルド、libwebrtc バイナリはリモートダウンロード） | Release |
| 未設定 | 有効 | 有効 | -- | **エラー** |
| 設定 | 無効 | -- | -- | **エラー** |
| 設定 | 有効 | 無効 | ローカル | Release |
| 設定 | 有効 | 有効 | ローカル | Debug |

### 制約と注意点

- **`local-export` feature との共存**: `local-export` feature（`build.rs:1114-1146`）は CMake ビルド出力先へのシンボリックリンクを作成する。`debug-build` を有効にすると `out_dir/_build/${target}/debug/` と `out_dir/_build/${target}/release/` の両方が生成されうるが、`local-export` は親ディレクトリへのリンクを作成するため、両方のプロファイルがリンク先で参照可能になる。競合は発生せず、追加の対応は不要。
- **webrtc-build の内部構造への依存**: 本設計は webrtc-build プロジェクトの内部ディレクトリ構造（`_build/${target}/${profile}/`, `_source/${target}/webrtc/src/`）に依存している。webrtc-build 側でこの構造が変更された場合、CMakeLists.txt と build.rs のパス導出ロジックを追従更新する必要がある。
- **cargo rebuild の検出**: `WEBRTC_BUILD_ROOT` 設定時は `build.rs` の `main()` で `${WEBRTC_BUILD_ROOT}/_build/${WEBRTC_C_TARGET}/${profile}/libwebrtc.a`（Windows の場合は `webrtc.lib`）のパスに対して `cargo::rerun-if-changed` を出力する。これにより、webrtc-build 側で libwebrtc.a が再ビルドされた場合、cargo が変更を検知してビルドスクリプトを再実行し、CMake ビルドとリンクが再実行される。`rerun-if-env-changed=WEBRTC_BUILD_ROOT` だけでは環境変数の値が変わらない限り検出されないため、ファイル単位の依存指定が必須である。
- **Android + WEBRTC_BUILD_ROOT における LLVM パス**: `build_webrtc_c()` は Android ターゲット時に `ANDROID_OVERRIDE_C_COMPILER` / `ANDROID_OVERRIDE_CXX_COMPILER` を CMake へ渡している（`build.rs:918-927`）。`WEBRTC_BUILD_ROOT` 設定時はこれらの変数を渡さない（変更対象参照）。これに伴い `webrtc/android.toolchain.cmake` に `if(DEFINED ANDROID_OVERRIDE_C_COMPILER)` ガードを追加し、未定義時は CMakeLists.txt で設定されたコンパイラ（WebRTC の Clang）をそのまま使うように修正する。`CMAKE_TOOLCHAIN_FILE` と `ANDROID_OVERRIDE_TOOLCHAIN_FILE` は `WEBRTC_BUILD_ROOT` 設定時も引き続き渡す。
- **`fs::canonicalize()` のエラーハンドリング**: `WEBRTC_BUILD_ROOT` に指定されたパスが存在しない場合、`fs::canonicalize()` はエラーを返す。この場合、明示的なエラーメッセージ（例: 「WEBRTC_BUILD_ROOT で指定されたパスが存在しないかアクセスできません: <path>」）で panic すること。空文字列が指定された場合も存在しないパスと同様に扱い、エラーとする。
- **`--release` と `debug-build` の同時指定**: 技術的に可能だが非推奨。C++ 側は Debug（`NDEBUG` なし、最適化なし）、Rust 側は Release（最適化あり）となり、`NDEBUG` 依存のヘッダーで ABI 不整合が発生するリスクがある。`build.rs` でこの組み合わせが検出された場合は標準エラーに警告を出力すること。利用手順でも注意喚起する。

### 変更対象

- `Cargo.toml`: `debug-build` feature を追加（`[features]` セクション）
- `build.rs`: `main()` の先頭にエラーチェックを追加。以下の 2 つの検査でカバーする（3 つのエラー行を捕捉する）:
  - `WEBRTC_BUILD_ROOT` 設定 + `source-build` 無効 → エラー（組み合わせ表の 設定/無効/-- 行を捕捉）
  - `WEBRTC_BUILD_ROOT` 未設定 + `debug-build` 有効 → エラー（組み合わせ表の 未設定/無効/有効 と 未設定/有効/有効 の両行を捕捉）
  また `WEBRTC_BUILD_ROOT` が設定されている場合、指定されたパスが存在し、かつ `_source/${WEBRTC_C_TARGET}/webrtc/src/` と `_build/${WEBRTC_C_TARGET}/${profile}/`（`profile` は `"debug"` または `"release"`）が存在することを確認し、存在しない場合は明示的なエラーメッセージで panic する。さらに、`_build/${WEBRTC_C_TARGET}/${profile}/libwebrtc.a`（Windows では `webrtc.lib`）のパスに対して `cargo::rerun-if-changed` を出力し、libwebrtc.a の変更を cargo が検知できるようにする。`--release`（`OPT_LEVEL` が `"0"` 以外）かつ `debug-build` feature 有効時は、標準エラーに警告を出力する
- `build.rs`: `should_use_prebuilt()` に `WEBRTC_BUILD_ROOT` のチェックを追加。設定時は `false` を返す
- `build.rs`: `build_webrtc_c()` のプロファイル設定を `debug-build` feature の有無で分岐。`profile` 変数、`config.profile()`、`CMAKE_BUILD_TYPE` を条件付きで切り替える
- `build.rs`: `build_webrtc_c()` から CMake へ `WEBRTC_BUILD_ROOT` を伝達（絶対パスに正規化した上で `-DWEBRTC_BUILD_ROOT=...` として渡す）。Android ターゲットかつ `WEBRTC_BUILD_ROOT` 設定時は `ANDROID_OVERRIDE_C_COMPILER` / `ANDROID_OVERRIDE_CXX_COMPILER` を CMake に渡さない
- `build.rs`: `generate_bindings()` に `debug-build` feature 有効時は `-UNDEBUG` を、無効時は `-DNDEBUG` を `clang_arg` として追加する
- `build.rs`: `main()` に `rerun-if-env-changed=WEBRTC_BUILD_ROOT` と `rerun-if-env-changed=CARGO_FEATURE_DEBUG_BUILD` を追加する
- `webrtc/CMakeLists.txt`: `WEBRTC_BUILD_ROOT` 定義時の分岐を追加（`### CMakeLists.txt の具体的な変更内容` の全項目を参照）。また `WEBRTC_BUILD_ROOT` が定義されている場合、受け取った値を `get_filename_component(... ABSOLUTE)` で正規化する
- `webrtc/CMakeLists.txt`: Windows 向けにハードコードされた `NDEBUG` を削除し `CMAKE_BUILD_TYPE` に委ねる。`_ITERATOR_DEBUG_LEVEL=0` は変更せず全ビルドタイプで常時維持する。なお `_ITERATOR_DEBUG_LEVEL=0` は C++ ターゲット（`CMakeLists.txt:565`）にのみ存在し、C ターゲット（`CMakeLists.txt:801`）には存在しない。C ターゲットは MSVC STL イテレータを使用しないため追加不要
- `webrtc/android.toolchain.cmake`: `ANDROID_OVERRIDE_C_COMPILER` / `ANDROID_OVERRIDE_CXX_COMPILER` が未定義の場合、`CMAKE_C_COMPILER` / `CMAKE_CXX_COMPILER` を上書きしないようガードを追加する（`if(DEFINED ANDROID_OVERRIDE_C_COMPILER)` 等）。未定義時は CMakeLists.txt 側で設定されたコンパイラ（WebRTC の Clang）をそのまま使う
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
3. `cargo build --features source-build,debug-build` を実行する（通常は Cargo の dev プロファイルでビルドする。`--release` との同時指定も技術的には可能だが、`debug_build` feature の主目的である libwebrtc 内部のデバッグに加えて Cargo 側の最適化が有効になるため、典型的なデバッグ用途では推奨されない）

**WEBRTC_BUILD_ROOT 未設定で debug-build を指定した場合（エラー）:**

```sh
cargo build --features debug-build
# → エラー: debug-build を利用するには WEBRTC_BUILD_ROOT 環境変数で
#          ローカルの webrtc-build プロジェクトルートを指定してください。
```

## テスト戦略

本変更はビルドシステムの変更であるため、ユニットテストや PBT の対象にはならない。以下の手動確認をもってテストとする:

1. **通常ビルドの継続確認**: `WEBRTC_BUILD_ROOT` 未設定、`source-build` 無効、`debug-build` 無効で `cargo build --release` が成功すること
2. **エラーケース 1 の確認**: `WEBRTC_BUILD_ROOT` 設定 + `source-build` 無効でビルドがエラー停止し、適切なエラーメッセージが表示されること
3. **エラーケース 2 の確認**: `WEBRTC_BUILD_ROOT` 未設定 + `source-build` 無効 + `debug-build` 有効でビルドがエラー停止し、適切なエラーメッセージが表示されること
4. **デバッグビルドの確認**: `WEBRTC_BUILD_ROOT` 設定 + `source-build` + `debug-build` で CMake ビルドが `CMAKE_BUILD_TYPE=Debug` で実行され、`NDEBUG` が定義されず、`_ITERATOR_DEBUG_LEVEL=0` が維持され、ビルドが成功すること
5. **ローカル Release ビルドの確認**: `WEBRTC_BUILD_ROOT` 設定 + `source-build` 有効 + `debug-build` 無効で CMake ビルドが `CMAKE_BUILD_TYPE=Release` で実行され、成功すること

テストの前提として、確認する各環境にあらかじめ webrtc-build の成果物（バイナリ・ヘッダー）を配置しておく必要がある。`WEBRTC_BUILD_ROOT` には絶対パスを使用することが推奨される。

加えて、`source-build` + `debug-build` feature 有効時に `WEBRTC_BUILD_ROOT` を設定した上で `cargo test --features source-build,debug-build` を実行し、`NDEBUG` の有無で影響を受ける可能性のある既存テスト（`src/tests.rs`、`tests/test_libyuv.rs` 等）がデバッグビルドでもパスすることを確認する。`NDEBUG` 有無により構造体レイアウトやマクロ展開結果が変わった場合、既存テストが失敗する可能性があるため、実装時に必ず確認すること。

## 完了条件

- `Cargo.toml` に `debug-build` feature が追加されている
- `WEBRTC_BUILD_ROOT` 設定済みかつ `source-build` feature 無効時にエラーで停止すること（適切なエラーメッセージ付き）
- `WEBRTC_BUILD_ROOT` 未設定かつ `debug-build` feature 有効時にエラーで停止すること（適切なエラーメッセージ付き）
- 環境変数 `WEBRTC_BUILD_ROOT` が設定されているとき、CMakeLists.txt が指定ディレクトリ配下の `_source/${WEBRTC_C_TARGET}/webrtc/src/` をインクルードパス、`_build/${WEBRTC_C_TARGET}/${BUILD_TYPE}/` をライブラリパスとして利用し、ダウンロードをスキップすること
- `WEBRTC_BUILD_ROOT` 設定時は CMakeLists.txt が VERSIONS ファイルの読み取りと LLVM のダウンロードをスキップし、webrtc ソースツリー内の `${WEBRTC_INCLUDE_DIR}/third_party/llvm-build/Release+Assets/` を Clang コンパイラとして、`${WEBRTC_INCLUDE_DIR}/third_party/libc++/src/include` を libc++ ヘッダーとして利用すること
- `WEBRTC_BUILD_ROOT` に相対パスが指定された場合、`CARGO_MANIFEST_DIR` を基準に絶対パスへ正規化して CMake へ渡すこと
- `debug-build` feature 有効時に `CMAKE_BUILD_TYPE=Debug`、無効時に `CMAKE_BUILD_TYPE=Release` でビルドされること
- Windows 向けにハードコードされた `NDEBUG` 定義を削除し、`CMAKE_BUILD_TYPE=Debug` 時に `NDEBUG` が定義されないこと（Release 時は CMake が自動付与）。`_ITERATOR_DEBUG_LEVEL=0` は全ビルドタイプで維持され、Debug ビルドした libwebrtc と ABI が一致すること
- `debug-build` feature 有効時は `generate_bindings()` に `-UNDEBUG` を、無効時は `-DNDEBUG` を `clang_arg` として渡し、各ビルドタイプの libwebrtc ヘッダーと一致するバインディングを生成すること
- `source-build` feature 未指定かつ `WEBRTC_BUILD_ROOT` 未設定かつ `debug-build` 無効の通常ビルドが引き続き成功すること
- `rerun-if-env-changed=WEBRTC_BUILD_ROOT` と `rerun-if-env-changed=CARGO_FEATURE_DEBUG_BUILD` が `main()` に追加されていること。また `WEBRTC_BUILD_ROOT` 設定時は libwebrtc.a のパスに対して `cargo::rerun-if-changed` が出力され、libwebrtc.a の変更が cargo に検知されること
- `CHANGES.md` の `## develop` に `[ADD] libwebrtc のデバッグビルドに対応する` エントリが追加されている

## 解決方法

主要な実装は PR #65 (コミット `54b4096`) で develop へマージ済みであり、`CHANGES.md` の 0.150.3 でリリース済み。

実装内容:

- `Cargo.toml` に `debug-build` feature を追加した
- `build.rs` の `main()` に `WEBRTC_BUILD_ROOT` 設定 + `source-build` 無効時のエラー、`WEBRTC_BUILD_ROOT` 未設定 + `debug-build` 有効時のエラーを追加した
- `build.rs` の `should_use_prebuilt()` に `WEBRTC_BUILD_ROOT` チェックを追加し、設定時はソースビルドへ進むようにした
- `build.rs` の `build_webrtc_c()` で `debug-build` feature の有無により `CMAKE_BUILD_TYPE` を Debug / Release に切り替え、`WEBRTC_BUILD_ROOT` を絶対パスへ正規化して CMake へ渡すようにした
- `webrtc/CMakeLists.txt` に `WEBRTC_BUILD_ROOT` 定義時の分岐を追加し、ローカルの webrtc-build 成果物（バイナリ・ヘッダー・Clang・libc++）を利用するようにした
- `webrtc/CMakeLists.txt` の Windows 向けハードコード `NDEBUG` を `$<$<NOT:$<CONFIG:Debug>>:NDEBUG>` に置き換え、`_ITERATOR_DEBUG_LEVEL=0` は全ビルドタイプで維持した
- `webrtc/android.toolchain.cmake` に `ANDROID_OVERRIDE_C_COMPILER` 未定義時のガードを追加した
- `build.rs` の `main()` に `rerun-if-env-changed=WEBRTC_BUILD_ROOT` / `CARGO_FEATURE_DEBUG_BUILD` と、libwebrtc.a の `rerun-if-changed` を追加した

上記に加え、完了条件「`debug-build` feature 無効時は `-DNDEBUG` を `clang_arg` として渡す」は、後続のコミット `48d0776` で実装した。

bindgen が使う libclang はデフォルトで `NDEBUG` を定義しないことを調査で確認した。そのため Debug ビルド時は `-UNDEBUG` を（libclang のデフォルトである NDEBUG 未定義を明示的に保証）、Release ビルド時は `-DNDEBUG` を渡し、CMake が Release で自動付与する NDEBUG とバインディング生成を一致させた。
