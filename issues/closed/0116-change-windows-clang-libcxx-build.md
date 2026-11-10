# Windows のビルドに libwebrtc が利用する Clang (clang-cl) と libc++ を利用する

- Created: 2026-10-14
- Completed: 2026-10-15
- Branch: feature/change-windows-clang-libcxx-build
- Polished: {YYYY-MM-DD}

## 目的

Windows の `shiguredo_webrtc` のビルドを、MSVC (cl.exe) + MSVC STL から libwebrtc が利用する Clang (clang-cl) + libc++ に変更する。MSVC でビルドした場合、libwebrtc が利用する libc++ とは標準ライブラリが異なるため、libwebrtc との ABI 互換性を保証できない。libwebrtc と同じコンパイラと標準ライブラリを使うことで、C++ の型やシンボルの解釈を libwebrtc と一致させる。

## 現状

### Windows のビルド構成

`webrtc/CMakeLists.txt` は `WEBRTC_USE_WEBRTC_LIBCXX` と `WEBRTC_USE_WEBRTC_CLANG` を定義しているが、`WEBRTC_C_TARGET` が `windows_x86_64` の場合は両方を `FALSE` にしている。そのため Windows では MSVC (cl.exe) と MSVC STL でビルドされる。

`build.rs` には Windows 向けに libc++ と Clang を設定する経路がなく、CMake のジェネレータも既定 (Visual Studio ジェネレータ) のままである。

### clang-cl をそのまま指定できない理由

Windows の libwebrtc には `clang.exe` / `clang++.exe` が含まれず `clang-cl.exe` のみが含まれる。clang-cl は MSVC (cl.exe) 互換のコマンドラインを解釈するドライバであり、`-nostdinc++` や `-isystem` のような GCC 互換のオプションはそのままでは解釈されない。

### 静的ライブラリの作成

WebRTC の Clang には `llvm-lib.exe` が含まれない。CMake の `CMAKE_AR` を解決できず、静的ライブラリ (Rust から見た `webrtc_c.lib`) を作成できない。

### ビルドツールの入手

`--features source-build` で libwebrtc をソースからビルドする場合、Ninja が必要になる。環境によっては導入されていないため、ビルド手順が環境依存になる。

## 設計方針

- Windows の `WEBRTC_C_TARGET` でも `WEBRTC_USE_WEBRTC_LIBCXX` と `WEBRTC_USE_WEBRTC_CLANG` を有効にする
- コンパイラには libwebrtc に含まれる `clang-cl.exe` を指定する。Windows の Clang には `clang-cl.exe` しか含まれないため、`clang.exe` / `clang++.exe` を探さない
- clang-cl に GCC 互換のオプションを渡す場合は `/clang:` を経由する。libc++ の指定は `/clang:-nostdinc++` と `/I<libc++ のインクルードディレクトリ>` になる
- CRT は libwebrtc に合わせて静的リンク (`/MT`) にする。CMake が既定で付ける `/MD` を打ち消す必要がある
- 例外処理 (`/EHsc`) は clang-cl では既定で無効なため明示的に有効にする
- libc++ の名前空間と ABI バージョンは libwebrtc が利用する `__config_site` から与えられる。`__config_site` が定義しない `_LIBCPP_HARDENING_MODE` はアーカイブのビルドと同じ値を明示し、`_LIBCPP_DISABLE_VISIBILITY_ANNOTATIONS` でシンボルに `__declspec(dllimport)` が付かないようにする (`_ITERATOR_DEBUG_LEVEL` は MSVC STL 固有のため削除する)
- CMake のジェネレータは Ninja にする。Visual Studio ジェネレータはツールセットを自前で決めるため `CMAKE_CXX_COMPILER` で指定したコンパイラを利用しない
- `CMAKE_AR` には Visual Studio の LLVM に含まれる `llvm-lib.exe` を指定する。WebRTC の Clang に `llvm-lib.exe` は含まれず、静的ライブラリを作成できないため
- Ninja はインストール状況に依存しないように自動でダウンロードする。バージョンと SHA256 は `Cargo.toml` の `[package.metadata.build-config]` に追加し、ダウンロードしたアーカイブを検証する
- Visual Studio の環境変数 (`LIB` など) はリンクに必要だが、コンパイラとしては Visual Studio を利用しない

## 完了条件

- Windows x86_64 で `cargo build --features source-build` が通り、`shiguredo_webrtc` のテストが通る
- ビルドに使うコンパイラが libwebrtc に含まれる `clang-cl.exe`、標準ライブラリが libc++、CRT が静的リンクになる
- Ninja が自動でダウンロードされ、環境に Ninja が無くてもビルドできる
- Windows 以外のターゲットのビルド結果が変わらない

## 解決方法

`webrtc/CMakeLists.txt` を変更した。`WEBRTC_USE_WEBRTC_LIBCXX` と `WEBRTC_USE_WEBRTC_CLANG` を Windows でも `TRUE` のままにし、`WEBRTC_C_TARGET` が `windows_x86_64` の場合に両方を `FALSE` にする分岐を削除した。

- ローカル Clang の存在チェックを、Windows では `clang-cl.exe`、それ以外では `clang++` に対して行う
- `CMAKE_C_COMPILER` / `CMAKE_CXX_COMPILER` に、Windows では `clang-cl.exe` を指定する
- libc++ のコンパイルオプションを、Windows では `/clang:-nostdinc++` と `/I${LIBCXX_INCLUDE_DIR}`、それ以外では `-nostdinc++` と `-isystem${LIBCXX_INCLUDE_DIR}` にする
- Windows のコンパイルオプションに `/EHsc` を追加する
- Windows のコンパイル定義から `_ITERATOR_DEBUG_LEVEL` を削除し、`_LIBCPP_HARDENING_MODE=_LIBCPP_HARDENING_MODE_EXTENSIVE` と `_LIBCPP_DISABLE_VISIBILITY_ANNOTATIONS` を追加する
- `CXX_SCAN_FOR_MODULES OFF` を `WEBRTC_CPP_TARGETS` の全ターゲットに適用する。`webrtc_c` にしか適用されておらず、`whip_cpp` / `whep_cpp` がビルドに失敗していた

CRT は `MSVC_RUNTIME_LIBRARY` で指定する。手書きの `/MT` は指定しない。clang-cl は後勝ちのため、`/MT` があると `MSVC_RUNTIME_LIBRARY` が選ぶ `/MTd` を打ち消してしまい、Debug でも Release CRT になる。C のターゲット (`whip_c` / `whep_c`) にも同じ `MSVC_RUNTIME_LIBRARY` を指定し、C++ のターゲットと CRT を揃える。

`build.rs` を変更した。

- Windows かつ Windows ターゲットの場合に `setup_windows_clang_build` を呼び、ジェネレータに Ninja、`CMAKE_MAKE_PROGRAM` にダウンロードした `ninja.exe` を指定する
- `CMAKE_TRY_COMPILE_TARGET_TYPE=STATIC_LIBRARY` を指定し、コンパイラのチェックを静的ライブラリの作成だけに留める。リンクには Visual Studio の環境変数 (`LIB`) が必要なため
- `vswhere.exe` で Visual Studio を検出し、`CMAKE_AR` に `llvm-lib.exe` を指定する。見つからない場合は、必要なコンポーネントを案内するメッセージで panic する
- Ninja と `vswhere.exe` をダウンロードし、SHA256 を検証する
- `debug-build` かつ Windows の場合に `check_windows_debug_crt_settings` で `RUSTFLAGS` を確認する。libwebrtc のデバッグビルドは静的デバッグ CRT でコンパイルされており、Rust が既定でリンクするリリース CRT と衝突するため、`/NODEFAULTLIB:msvcrt` と `/DEFAULTLIB:libcmtd` が指定されていなければ、必要な指定を案内するメッセージで panic する。ライブラリの build script から最終バイナリのリンクは変更できないため、検証だけを行う。C/C++ を含む依存の有無は build script から知り得ないため `CFLAGS` / `CXXFLAGS` は検査せず、案内に留める

`Cargo.toml` の `[package.metadata.build-config]` に `ninja-version` と `ninja-win-sha256` を追加した。

`README.md` に Windows の前提条件とデバッグビルドの手順を記載し、`skills/shiguredo-webrtc/SKILL.md` の必要なツールの記述を追随させた。

Windows x86_64 で `cargo build --workspace --features source-build` が通り、`cargo test --workspace --features source-build` が 158 + 35 + 2 + 8 件すべて通ることを確認した。`debug-build` では `RUSTFLAGS` と `CFLAGS` / `CXXFLAGS` を指定したうえで `whip.exe` と `whep.exe` がビルドでき、実行できることを確認した。生成したオブジェクトが `dumpbin /directives` で Debug では `libcmtd.lib`、Release では `libcmt.lib` を要求することも確認した。

Windows 以外のターゲットは CI (ubuntu-22.04 / 24.04 / 26.04、macos-15 / 26) で確認する。
