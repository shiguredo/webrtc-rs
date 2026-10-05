# aarch64-apple-ios-sim 向け prebuilt を追加する

- Created: 2026-10-05
- Completed: {YYYY-MM-DD}
- Branch: feature/add-ios-simulator-prebuilt
- Polished: {YYYY-MM-DD}

## 目的

iOS シミュレーター (arm64) 向けの prebuilt (`libwebrtc_c-ios_sim_arm64.tar.gz`) を GitHub Releases に追加し、`aarch64-apple-ios-sim` で `shiguredo_webrtc` をビルドできるようにする。

iOS シミュレーター上で動作確認を行いたい iOS クライアントの需要があり、`aarch64-apple-ios-sim` 向けの prebuilt が必要である。

Intel シミュレーター (`x86_64-apple-ios-sim`) と Intel Mac (`x86_64-apple-darwin`) は対応しない (決定済み)。

## 前提条件

webrtc-build の `ios` 成果物 (`webrtc.ios.tar.gz`) に `lib/simulator/libwebrtc.a` (simulator:arm64) が追加され、リリースされていること。webrtc-build 側に追加を依頼する issue を起票済みであり、成果物レイアウトは両リポジトリで一致させる必要がある。

この前提が満たされるまで、本リポジトリの CI / リリースで `aarch64-apple-ios-sim` のビルド検証は通らない。先行して実装・検証する場合は、webrtc-build をローカルでビルドして `webrtc.ios.tar.gz` を生成するか、検証用リリースを用意する。

## 現状

- `build.rs` の `get_target_platform` は `CARGO_CFG_TARGET_OS` / `CARGO_CFG_TARGET_ARCH` の組で内部ターゲット名を決めており、`("ios", "aarch64")` を無条件に `ios_arm64` とする。`aarch64-apple-ios-sim` は `CARGO_CFG_TARGET_ABI=sim` / `CARGO_CFG_TARGET_ENV=sim` になるが、これを見ていない
- `webrtc/CMakeLists.txt` の iOS 判定は `WEBRTC_C_TARGET STREQUAL "ios_arm64"` で行われている
  - ダウンロードするアーカイブ名の解決 (`webrtc.ios.tar.gz`) と `WEBRTC_LIBRARY_DIR` の解決 (`lib` 固定)
  - OBJCXX ソース (`objc.mm` / `audio_session.mm`) の追加と framework のリンク設定
  - Apple 以外をダミー C++ としてコンパイルする条件
  - `WEBRTC_BUILD_ROOT` 使用時のソース / ビルドディレクトリ解決 (`_source/${WEBRTC_C_TARGET}/...` / `_build/${WEBRTC_C_TARGET}/...`)
- `build.rs` の `build_webrtc_c` は `ios_arm64` のときに `CMAKE_SYSTEM_NAME=iOS` / `CMAKE_OSX_ARCHITECTURES=arm64` / `CMAKE_OSX_DEPLOYMENT_TARGET` を設定する。シミュレーター用の `CMAKE_OSX_SYSROOT` は無い
- `build.rs` の `emit_link_directives` は `CARGO_CFG_TARGET_OS == "ios"` で device / simulator 共通の framework をリンクしており、この部分は変更不要
- `build.rs` の `generate_bindings` は `CARGO_CFG_TARGET_OS == "ios"` で `objc.h` も読み込んでおり、この部分は変更不要
- `webrtc/scripts/bundle_apple_static_library.sh` は `macos_arm64|ios_arm64` のみ許可しており、他のターゲット名ではエラーになる
- `.github/workflows/release.yml` の `build-prebuilt-ios` は `aarch64-apple-ios` のみをビルドし、`libwebrtc_c-ios_arm64.tar.gz` をアップロードする。シミュレーター向けは無い
- `.github/workflows/ci.yml` の `build-ios` は `aarch64-apple-ios` のみを検証している
- `build-prebuilt-apple-xcframework` は `libwebrtc_c-ios_arm64` と `libwebrtc_c-macos_arm64` から `libwebrtc_c.xcframework.zip` を生成しており、simulator スライスは含まない
- `skills/libwebrtc-c/SKILL.md` のサポートターゲット一覧に `ios_arm64` はあるが、シミュレーター向けは無い

## 設計方針

- `build.rs` の `get_target_platform` を `CARGO_CFG_TARGET_ABI=sim` で分岐させ、`("ios", "aarch64")` + `sim` を内部ターゲット名 `ios_sim_arm64` とする (`WEBRTC_C_TARGET` による明示指定は引き続き最優先)
- `build.rs` の `build_webrtc_c` の iOS クロスコンパイル設定を `ios_sim_arm64` にも適用する
  - `CMAKE_SYSTEM_NAME=iOS` / `CMAKE_OSX_ARCHITECTURES=arm64` / `CMAKE_OSX_DEPLOYMENT_TARGET` は共通
  - `ios_sim_arm64` のときだけ `CMAKE_OSX_SYSROOT=iphonesimulator` を追加する
- `webrtc/CMakeLists.txt`
  - `WEBRTC_ARCHIVE_NAME` の解決: `ios_sim_arm64` は `webrtc.ios.tar.gz` (device と同一)
  - `WEBRTC_LIBRARY_DIR` の解決: `ios_sim_arm64` は `${WEBRTC_DIR}/lib/simulator`
  - iOS の CMake 分岐 (`ios_arm64` 判定の全箇所) を `ios_sim_arm64` にも適用する
  - Apple 以外をダミー C++ としてコンパイルする条件に `ios_sim_arm64` を追加する
  - `WEBRTC_BUILD_ROOT` 使用時は `_source/ios_sim_arm64` / `_build/ios_sim_arm64` を参照する。ただし webrtc-build のソース / ビルドディレクトリ名は `ios` であり、`ios_arm64` でも同じ不一致がある。`WEBRTC_BUILD_ROOT` 経由の iOS ローカルビルドを使う場合の対応は、実装時に iOS の既存経路と合わせて確認する (本 issue の主対象は prebuilt / source-build のリモート経路)
- `webrtc/scripts/bundle_apple_static_library.sh` の `case` に `ios_sim_arm64` を追加する
- `.github/workflows/release.yml`
  - 既存の `build-prebuilt-ios` を `aarch64-apple-ios` / `aarch64-apple-ios-sim` の matrix に変更し、`libwebrtc_c-ios_sim_arm64.tar.gz` と `.sha256` をアップロードする
  - シミュレーターの成果物は XCFramework 生成の入力 (`libwebrtc_c-xcframework-*`) に含めない
  - `publish` の `needs` に反映する
- `.github/workflows/ci.yml` の `build-ios` に `aarch64-apple-ios-sim` の `--features source-build` ビルド検証を追加する
- `libwebrtc_c.xcframework.zip` への simulator スライス追加は本 issue の対象外とする (per-target の prebuilt 追加を本 issue の範囲とし、XCFramework の構成変更は別途判断する)
- `Cargo.toml` の `package.metadata.external-dependencies.webrtc-build` を、シミュレーター向け `libwebrtc.a` を含む最初の webrtc-build リリースに更新する
- `skills/libwebrtc-c/SKILL.md` のサポートターゲット一覧に `ios_sim_arm64` を追加する
- prebuilt の追加なので `CHANGES.md` の `## develop` に `[ADD]` を記載する

## 完了条件

- `cargo build --release --features source-build --target aarch64-apple-ios-sim` が成功する (更新後の webrtc-build リリースの `webrtc.ios.tar.gz` を利用)
- `--features source-build` なしの prebuilt 利用で `aarch64-apple-ios-sim` のビルドが成功し、`libwebrtc_c-ios_sim_arm64.tar.gz` をダウンロードして `libwebrtc_c.a` をリンクできる
- 生成したライブラリが `lipo -info` で arm64、`otool -l` の LC_BUILD_VERSION で platform 7 (iOS Simulator) と確認できる
- `aarch64-apple-ios` (device) の prebuilt ビルドと成果物に差分がない
- GitHub Actions の `build-prebuilt-ios` 相当で `libwebrtc_c-ios_sim_arm64.tar.gz` が GitHub Release にアップロードされている
- GitHub Actions の `build-ios` が device / simulator の両方で成功する
- `skills/libwebrtc-c/SKILL.md` のサポートターゲットに `ios_sim_arm64` が記載されている
- CHANGES.md の `## develop` に `[ADD]` が記載されている

## 解決方法

未着手
