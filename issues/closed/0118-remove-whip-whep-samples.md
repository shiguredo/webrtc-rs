# C/C++ の WHIP / WHEP サンプルを削除する

- Created: 2026-10-15
- Completed: 2026-10-16
- Branch: feature/remove-whip-whep-samples
- Polished: {YYYY-MM-DD}

## 目的

`webrtc/src/whip.c` / `whep.c` / `whip.cpp` / `whep.cpp` と、それらに対応する CMake ターゲット (`whip_c` / `whep_c` / `whip_cpp` / `whep_cpp`) を削除する。

4 ファイルは C API / C++ API の利用例として置かれているが、どのサポートプラットフォームでもビルドできず、利用例として機能していない。

## 現状

### Windows x86_64 では 4 ターゲットともビルドできない

`webrtc/CMakeLists.txt` は 4 ファイルを `whip_c` / `whep_c` / `whip_cpp` / `whep_cpp` として定義しているが、いずれもビルドできない。

- `whip.c` / `whep.c` は `strings.h` / `pthread.h` / `unistd.h` / `netdb.h` / `sys/socket.h` を前提としており、`#ifdef _WIN32` の分岐を持たない。`strncasecmp` / `pthread_*` / `close` / `socklen_t` / `usleep` をそのまま使っている
- `whip.cpp` / `whep.cpp` は `#ifdef _WIN32` で Winsock に切り替えているが、`gai_strerror` の呼び出しが残っている。`webrtc/CMakeLists.txt` の Windows 向け `target_compile_definitions` が `UNICODE` と `_UNICODE` を定義するため、Windows SDK の `ws2tcpip.h` が `gai_strerror` を `gai_strerrorW` にマクロ定義する。`gai_strerrorW` は `WCHAR*` を返すため `std::cerr` へ出力できず、コンパイルに失敗する
- `whip.cpp` / `whep.cpp` は `gai_strerror` を直してもリンクできない。libwebrtc に含まれる Rust の std が要求する `ntdll.lib` (`NtWriteFile` / `NtCreateNamedPipeFile`) と `userenv.lib` (`GetUserProfileDirectoryW`) が、Windows 向け `target_link_libraries` に無い

### C 版は Linux でもビルドできない

`whip.c` の `whip_SdpVideoFormat_from_RtpCodec` は `webrtc_SdpVideoFormat_new` を 3 引数で呼んでいるが、宣言は `const char* name, size_t name_len` の 2 引数である。`whep.c` も同じ呼び出しを持つ。この不一致は Windows に限らずビルドを妨げる。

### 壊れていることが検出されない

`build.rs` の `build_webrtc_c` は `config.build_target("bundled_webrtc_c_bundling")` しか呼ばない。そのため `cargo build --workspace --features source-build` では 4 ターゲットはビルドされない。`.github/workflows/ci.yml` / `release.yml` と `prek.toml` のフックも同じ経路しか使わないため、壊れた状態が残り続ける。

### 代替となる利用例がある

利用例としては Rust の `examples/whip` / `examples/whep` があり、prebuilt と source-build の両方で Windows を含めてビルドできる。C API 本体 (`webrtc/src/webrtc_c/`) と `webrtc_c.h` はそのまま残る。

## 設計方針

- `webrtc/src/whip.c` / `whep.c` / `whip.cpp` / `whep.cpp` を削除する
- `webrtc/CMakeLists.txt` から 4 ターゲットを削除し、`WEBRTC_CPP_TARGETS` / `WEBRTC_C_TARGETS` の `foreach` を `webrtc_c` に直接適用する形へ整理する。`whip_c` / `whep_c` 向けのプラットフォーム別リンク設定も不要になる
- `bundle_static_library(webrtc_c ...)` と `bundled_webrtc_c` は残す。`build.rs` が `bundled_webrtc_c_bundling` をビルドして `webrtc_c.lib` を取り出すため
- `skills/libwebrtc-c/SKILL.md` の主要ビルドターゲット表と WHIP / WHEP サンプルの記述を更新する
- `CHANGES.md` に `[REMOVE]` を追記する。develop にある C++20 モジュールスキャンの FIX は対象 (`whip_cpp` / `whep_cpp`) が無くなるため削除する
- 4 ファイルのいずれかを対象にしている issue を closed にする。対象は `issues/` と `issues/pending/` を `whip.c` / `whep.c` / `whip.cpp` / `whep.cpp` で検索して洗い出す (本 issue の作成時点で 40 件)

## 完了条件

- 4 ファイルが削除されている
- `webrtc/CMakeLists.txt` から 4 ターゲットとそれら向けの設定が削除されている
- Linux / macOS / Windows で `cargo build --workspace --features source-build` が通り、`webrtc_c.lib` / `libwebrtc_c.a` の生成結果が変わらない
- `cargo test --workspace --features source-build` と `cargo clippy --workspace --features source-build --all-targets -- -D warnings` が通る
- `skills/libwebrtc-c/SKILL.md` と `CHANGES.md` が更新されている
- 4 ファイルのいずれかを対象にしていた issue がすべて closed になっている

## 解決方法

- `webrtc/src/whip.c` / `whep.c` / `whip.cpp` / `whep.cpp` を削除する
- `webrtc/CMakeLists.txt` から `whip_c` / `whep_c` / `whip_cpp` / `whep_cpp` の 4 ターゲットを削除し、`WEBRTC_CPP_TARGETS` / `WEBRTC_C_TARGETS` を廃止して `webrtc_c` に直接設定を適用する。これに伴い各プラットフォーム別ブロックの `foreach` と、後半の `whip_c` / `whep_c` 向けリンク設定が不要になるため削除する
- `skills/libwebrtc-c/SKILL.md` のディレクトリ構成・主要ビルドターゲット表・WHIP / WHEP サンプル節を削除し、`description` からもサンプルの記述を外す
- `webrtc/RULES.md` のビルドと実行の節から、削除された `whip_c` を指す実行コマンド 2 行を削除する
- `CHANGES.md` に削除を追記する。設計方針では `[REMOVE]` としていたが、`shiguredo-changelog` の種別は CHANGE / ADD / UPDATE / FIX の 4 つであり `[REMOVE]` は存在しないため、後方互換のない変更として `[CHANGE]` とする。develop にあった C++20 モジュールスキャンの `[FIX]` は対象が無くなるため削除する
- 4 ファイルを対象としていた issue 40 件を closed にする。`issues/` と `issues/pending/` を `whip.c` / `whep.c` / `whip.cpp` / `whep.cpp` / `whip_c` / `whep_c` で検索すると 42 件が該当するが、うち `0027-bug-webrtc-c-std-map-iterator-lifetime` と `0092-add-file-rotating-log-sink` は `whip_c` が `webrtc_c` などの別名に部分一致しただけの誤検出であり、対象外として `issues/pending/` に残す
- `0073-bug-whip-whep-c-observer-cbs-missing` には解決方法が既に書かれていたが、削除対象のコードへの修正手順であったため、削除による解決へ差し替える

### 検証結果

- `cargo build --workspace --features source-build` が成功する。`webrtc/_build/<target>/release/build/bundled/` に `libwebrtc_c.a` が生成され、パスとファイル名は変更前と同じである
- `cargo clippy --workspace --features source-build --all-targets -- -D warnings` が成功する
- `cargo test --workspace --features source-build` が 202 件すべて成功する
- CMake のターゲット一覧から 4 ターゲットが消え、`webrtc_c` と `bundled_webrtc_c_bundling` だけになる
- macOS / Windows でのビルドは CMake 4.2 以上を要するため手元では検証できず、CI での確認に委ねる
