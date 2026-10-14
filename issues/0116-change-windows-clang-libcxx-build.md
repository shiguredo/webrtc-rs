# Windows のビルドに libwebrtc が利用する Clang (clang-cl) と libc++ を利用する

- Created: 2026-10-14
- Completed: {YYYY-MM-DD}
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

`webrtc/CMakeLists.txt` を変更する。

- `WEBRTC_USE_WEBRTC_LIBCXX` と `WEBRTC_USE_WEBRTC_CLANG` を Windows でも `TRUE` のままにする (`WEBRTC_C_TARGET` が `windows_x86_64` の場合に `FALSE` にする分岐を削除する)
- `WEBRTC_BUILD_ROOT` が指定された場合のローカル Clang の存在チェックを、Windows では `clang-cl.exe`、それ以外では `clang++` に対して行う
- `WEBRTC_USE_WEBRTC_CLANG` の `CMAKE_C_COMPILER` / `CMAKE_CXX_COMPILER` に、Windows では `clang-cl.exe` を指定する
- libc++ のコンパイルオプションを、Windows では `/clang:-nostdinc++` と `/I${LIBCXX_INCLUDE_DIR}`、それ以外では `-nostdinc++` と `-isystem${LIBCXX_INCLUDE_DIR}` にする
- Windows のコンパイルオプションに `/EHsc` と `/MT` を追加する
- Windows のコンパイル定義から `_ITERATOR_DEBUG_LEVEL` を削除し、`_LIBCPP_HARDENING_MODE=_LIBCPP_HARDENING_MODE_EXTENSIVE` と `_LIBCPP_DISABLE_VISIBILITY_ANNOTATIONS` を追加する

`build.rs` を変更する。

- Windows かつ Windows ターゲットの場合に `setup_windows_clang_build` を呼び、CMake の設定に `CMAKE_TRY_COMPILE_TARGET_TYPE=STATIC_LIBRARY` を追加する。リンクには Visual Studio の環境変数 (`LIB`) が必要なため、コンパイラのチェックを静的ライブラリの作成だけに留める
- ジェネレータに Ninja を指定し、`CMAKE_MAKE_PROGRAM` にダウンロードした `ninja.exe` を指定する
- `vswhere.exe` で Visual Studio を検出し、`CMAKE_AR` に `llvm-lib.exe` を指定する。見つからない場合は、必要なコンポーネントを案内するメッセージで panic する
- Ninja と `vswhere.exe` のダウンロードと SHA256 の検証を行う。`vswhere.exe` の SHA256 検証は既存のダウンロード処理と共通化する

`Cargo.toml` の `[package.metadata.build-config]` に `ninja-version` と `ninja-win-sha256` を追加する。

`README.md` の Windows の前提条件を更新し、`C++ Clang tools for Windows` が `llvm-lib` の入手元になること、C++ コンパイラは別途インストール不要であること、Ninja は自動でダウンロードされることを記載する。
