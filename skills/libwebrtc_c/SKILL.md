---
name: libwebrtc_c
description: webrtc-rs リポジトリ配下の webrtc/ サブプロジェクト (libwebrtc C++ API の薄い C ラッパー) の機能・設計ルールリファレンス。命名規則、*_refcounted / *_unique / *_vector / *_inlined_vector のメモリ管理、null チェック方針、Cbs ルール、RULES.md の移植ルール、webrtc_c.h の公開 API、CMake ビルドターゲット、WHIP/WHEP サンプル、セルフチェック手順に関する質問時に使用。
---

# libwebrtc_c

`webrtc-rs` リポジトリの `webrtc/` サブプロジェクトに存在する、libwebrtc C++ API の薄い C ラッパー層。Rust 側の `shiguredo_webrtc` crate はこの C API を通じて libwebrtc を利用する。

## 位置づけ

- C ラッパーは **libwebrtc との薄い対応のみ** を実装する
- 便利関数や独自機能の追加は禁止
- 元の C++ API のシグネチャ・名前に忠実に移植する
- C ラッパーのファイルパスは元の C++ ファイルのパスと一致させる (分割をある程度サボることは許容、例: `api/environment.h` は `api/environment/environment.h` と `api/environment/environment_factory.h` を統合)

## ディレクトリ構成

```
webrtc/
├── CMakeLists.txt          libwebrtc/LLVM ダウンロード、bindgen 用 install ターゲット、ビルド構成
├── RULES.md                C++ → C 移植ルール (本 skill の根拠)
├── android.toolchain.cmake Android 向けツールチェーン
├── run.py                  clang-format / clang-include-cleaner (IWYU) ヘルパー
├── scripts/                Apple 向け静的ライブラリ統合スクリプト
└── src/
    ├── webrtc_c.h          統合ヘッダ (全 C API を re-export)
    ├── whip.c, whep.c      WHIP/WHEP サンプル (C)
    ├── whip.cpp, whep.cpp  WHIP/WHEP サンプル (C++)
    └── webrtc_c/
        ├── api/            PeerConnection, JSEP, RTP, 統計, video/audio codec, environment, observer 等 (audio/, audio_codecs/, video/, video_codecs/, stats/ サブディレクトリを含む)
        ├── pc/             connection_context (接続管理)
        ├── rtc_base/       暗号、SSL、ロギング、スレッド、タイムスタンプ
        ├── media/          base/adapted_video_track_source, engine/simulcast_encoder_adapter
        ├── modules/        video_coding 関連 (common_constants, video_codec_interface)
        ├── sdk/android/    native_api/ (audio_device_module, codecs, jni), src/jni
        ├── sdk/objc/       components/ (audio, video_codec), native/api
        ├── android.h, ios.h, objc.h  プラットフォーム固有ヘッダ
        ├── jni_export.{cc,h}        Android JNI 連携
        ├── objc.mm                  Objective-C 連携
        ├── common.h                 マクロ集 (DECLARE_*, EXPORT)
        ├── common.impl.h            マクロ実装
        ├── std.{h,cc,impl.h}        std::string / std::vector / std::map の C ラッパー
        └── libyuv.{h,cc}            libyuv の C ラッパー
```

## 命名規則

C++ のシンボルを機械的に C 名へ写像する。

| C++ | C |
|-----|---|
| `webrtc::Xxx` (クラス) | `struct webrtc_Xxx` |
| `webrtc::Xxx::Yyy` (メソッド) | `webrtc_Xxx_Yyy` (関数) |
| `webrtc::Xxx::field` 読み書き | `webrtc_Xxx_get_field` / `webrtc_Xxx_set_field` |

名前空間・クラス境界は `_` で連結する。独自の名前省略や合成は行わない。

## メモリ管理パターン

C++ のオブジェクト寿命管理を C 側でどう扱うかは **3 パターン** に分かれる。さらに `std::vector` / `absl::InlinedVector` 向けのバリエーションもある。

### 1. `scoped_refptr<T>` → `T_refcounted*`

参照カウントを手動管理する。

| 操作 | 関数 |
|------|------|
| 参照追加 | `T_AddRef(ptr)` |
| 参照解放 | `T_Release(ptr)` |
| 生ポインタ取得 | `T_refcounted_get(ptr)` (必ずこれを経由する) |
| 型宣言 | `WEBRTC_DECLARE_REFCOUNTED(T)` |
| 型定義 | `WEBRTC_DEFINE_REFCOUNTED(T, CppType)` |

**禁止事項**:

- `struct T_refcounted*` を直接 C++ 型にキャストしない (必ず `T_refcounted_get()` 経由)
- C++ オブジェクトを渡すときは `webrtc::scoped_refptr<CppType>` で構築し、`p.release()` で取得したものだけをキャストする

### 2. `std::unique_ptr<T>` → `T_unique*`

単一所有の寿命管理。

| 操作 | 関数 |
|------|------|
| 解放 | `T_unique_delete(ptr)` |
| 生ポインタ取得 | `T_unique_get(ptr)` (必ずこれを経由する) |
| 型宣言 | `WEBRTC_DECLARE_UNIQUE(T)` |
| 型定義 | `WEBRTC_DEFINE_UNIQUE(T, CppType)` |

**禁止事項**:

- `struct T_unique*` を直接 C++ 型にキャストしない
- C++ オブジェクトを渡すときは `std::unique_ptr<CppType>` で構築し、`p.release()` で取得したものだけをキャストする

### 3. スタックオブジェクト (`CppType` 値) → `T*`

C++ でスタック配置するクラスは C 側ではヒープに置いて明示解放する。

| 操作 | 関数 |
|------|------|
| 解放 | `T_delete(ptr)` |

### 4. `std::vector<T>` → `T_vector*`

`WEBRTC_DECLARE_VECTOR(T)` で宣言。`_new(size)` / `_delete` / `_get(i)` / `_size` / `_resize(size)` / `_set(i, val)` / `_push_back(val)` を提供する。デフォルトコンストラクタを持たない型向けには `WEBRTC_DECLARE_VECTOR_NO_DEFAULT_CTOR(T)` を使い、`_new()` が引数なしになる代わりに `_clear` が追加される。`scoped_refptr` の vector 用には `WEBRTC_DECLARE_REFCOUNTED_VECTOR(T)` を使い、要素は `T_refcounted*` として扱う。

### 5. `absl::InlinedVector<T, N>` → `T_inlined_vector*`

`WEBRTC_DECLARE_INLINED_VECTOR(T)` で宣言。`_new(size)` / `_delete` / `_get(i)` / `_size` / `_resize(size)` / `_set(i, val)` / `_push_back(val)` / `_clear` を提供する。

### `std::optional<CppType>` の扱い

`struct T*` として扱い、`nullptr` を空値として表現する。引数に `CppType` を取る C++ 関数を移植するときは `struct T*` を受け取り、C++ 側の内部でコピー・ムーブを行う。

## null チェック方針

- **C ラッパーではポインタ引数の null チェックを原則として行わない**
  - libwebrtc の薄いラッパーであり、C 側から null が渡されたらそのまま C++ 側へ渡す
  - それによって生じるクラッシュや未定義動作は呼び出し側の責任
- デリファレンスが必要な箇所では、デバッグビルドで契約違反を検出できるよう `assert(ptr != nullptr)` を入れる
- 以下の場合は null チェックを許容する:
  - C++ 側が valid に null を返しうる戻り値を C 側の型に変換する際、変換にデリファレンスが必要でクラッシュする場合 (C++ → C 方向の null は正しく伝える)
  - C 側の invariant を表明する `assert()`

## Cbs 構造体のコールバック関数ポインタ

- **null 非許容** を原則とする
  - 呼び出し側は Cbs の全関数ポインタを非 null で設定しなければならない
  - Cbs 構築時に `assert(cbs->OnXxx != nullptr)` で契約違反を検出する
  - ディスパッチ時の null チェックは行わず、無条件呼び出しとする
- 例外:
  - `AudioDeviceModule_cbs` (デフォルト実装 + 部分上書き方式のため適用外)
  - `RTCStatsCollectorCallback_cbs` は `OnDestroy` を持たないが、それ以外は他と同様に扱う

## 戻り値の扱い

### `RTCErrorOr<T>`

複数の戻り値を持つ型。out パラメータに分割する。

- `RTCErrorOr<webrtc::scoped_refptr<CppType>>` の場合: `struct webrtc_RTCError*` と `struct CppType_refcounted*` を出力引数に追加

### `std::string`

`struct std_string_unique*` として返し、利用側は `std_string_unique_delete` で解放する。

### C-API 全体の一貫性

`*_unique` でない C 関数の移植で C++ 側が戻り値を返す場合、C-API 全体の一貫性を優先して **out パラメータ方式で統一する** ことがある (recent change)。

## セルフチェック手順

新規 API 追加時・変更時は **必ず** 以下を実施する。

1. 作業開始前に RULES.md を読み直し、関係するルール (薄いラッパー、元の C++ パスと名前、命名規則、`*_unique` / `*_refcounted` の扱い、null チェック方針、Cbs の null 非許容) を箇条書きにする
2. 対応する C++ パスと型名を必ず開いて照合し、C 側のファイル・シンボル名が元の C++ に一致しているか確認する
3. `*_unique` / `*_refcounted` へのキャストが必ず `*_unique_get` / `*_refcounted_get` / `release` 経由になっているか `rg` でチェックする
4. 便利関数やパラメータ展開を追加していないか、各変更ブロックごとに「薄いラッパーか」を自問する
5. 変更後に再度 RULES.md を読み直し、全ルール順守をチェックリスト形式で確認してから回答する

## 統合ヘッダ `src/webrtc_c.h`

以下のヘッダを re-export する (`IWYU pragma: begin_exports` / `end_exports` で囲む)。

| カテゴリ | ヘッダ |
|----------|--------|
| 環境・基盤 | `api/environment.h`, `api/ref_count.h`, `api/rtc_error.h`, `api/rtc_event_log.h`, `api/priority.h`, `api/media_types.h` |
| PeerConnection / JSEP | `api/peer_connection_interface.h`, `api/jsep.h`, `api/set_local_description_observer_interface.h`, `api/set_remote_description_observer_interface.h` |
| Media | `api/media_stream_interface.h`, `api/data_channel_interface.h`, `api/dtls_transport_interface.h` |
| 音声 | `api/audio/audio_device.h`, `api/audio/audio_processing.h`, `api/audio_codecs/audio_decoder_factory.h`, `api/audio_codecs/audio_encoder_factory.h` |
| 映像 | `api/video/{video_frame,video_frame_buffer,i420_buffer,nv12_buffer,color_space,encoded_image,video_rotation,video_sink_interface,video_source_interface}.h` |
| 映像コーデック | `api/video_codecs/{video_codec,video_encoder,video_decoder,video_encoder_factory,video_decoder_factory,sdp_video_format,simulcast_stream}.h` |
| RTP | `api/rtp_parameters.h`, `api/rtp_receiver_interface.h`, `api/rtp_sender_interface.h`, `api/rtp_transceiver_direction.h`, `api/rtp_transceiver_interface.h` |
| 統計 | `api/stats/rtc_stats_collector_callback.h`, `api/stats/rtc_stats_report.h` |
| PeerConnection 内部 | `pc/connection_context.h` |
| Media 実装 | `media/base/adapted_video_track_source.h`, `media/engine/simulcast_encoder_adapter.h` |
| Video coding | `modules/video_coding/codecs/interface/common_constants.h`, `modules/video_coding/include/video_codec_interface.h` |
| rtc_base | `rtc_base/{crypto_random,logging,ssl_adapter,ssl_certificate,ssl_identity,thread,time_utils,timestamp_aligner}.h` |
| Apple SDK | `sdk/objc/components/video_codec/{RTCDefaultVideoEncoderFactory,RTCDefaultVideoDecoderFactory}.h`, `sdk/objc/native/api/{video_encoder_factory,video_decoder_factory}.h` |
| 共通 | `common.h`, `std.h`, `libyuv.h` |

## C アプリから refcount に相乗りする

C アプリ側で `webrtc::scoped_refptr` に相互変換できるクラスを作りたい場合:

1. 構造体のメンバーに `webrtc_RefCountInterface_ref*` を持たせる
2. `webrtc_RefCountInterface_Create` を呼んで作成
3. 参照カウントが 0 になったときに呼ばれるコールバック関数を設定する

## ビルドと CMake ターゲット

### CMake 必須変数

| 変数 | 用途 |
|------|------|
| `WEBRTC_C_TARGET` | ターゲット OS/アーキ (例: `ubuntu-24.04_x86_64`, `macos_arm64`, `windows_x86_64`, `android_arm64`, `ios_arm64`, `raspberry-pi-os_armv8`) |
| `WEBRTC_BUILD_VERSION` | libwebrtc バージョン (例: `m150.7871.0.0`) |
| `WEBRTC_BASE_URL` | webrtc-build リリースのベース URL |
| `WEBRTC_C_SYSROOT` | ARMv8 クロスコンパイル時のみ必須 |

### 主要ビルドターゲット

| ターゲット | 種別 | 内容 |
|-----------|------|------|
| `webrtc_c` | 静的ライブラリ | C API 本体 (50+ ソースファイル) |
| `whip_c`, `whep_c` | 実行可能 (C) | WHIP/WHEP の C サンプル |
| `whip_cpp`, `whep_cpp` | 実行可能 (C++) | WHIP/WHEP の C++ サンプル |
| `bundled_webrtc_c` | バンドル静的ライブラリ | `webrtc_c.a` と `libwebrtc.a` を統合 (`whip_c` / `whep_c` のリンク用) |

### サポートターゲット

- `ubuntu-24.04_x86_64`, `ubuntu-24.04_armv8`
- `ubuntu-22.04_x86_64`, `ubuntu-22.04_armv8`
- `raspberry-pi-os_armv8`
- `macos_arm64`
- `ios_arm64`
- `android_arm64`
- `windows_x86_64`

### リンク対象 (主要なもの)

- `libwebrtc.a` (外部依存、prebuilt または webrtc-build で生成)
- `Threads::Threads`
- Linux: `X11`, `dl`, `rt`
- Windows: `winmm`, `ws2_32`, `Strmiids`, `dmoguids`, `iphlpapi`, `msdmo`, `Secur32`, `wmcodecdspuuid`
- macOS: `AVFoundation`, `AppKit`, `AudioToolbox`, `CoreAudio`, `CoreMedia`, `IOSurface`, `Metal`, `MetalKit`, `OpenGL`, `QuartzCore`, `ScreenCaptureKit`, `VideoToolbox` (framework)
- iOS: `CoreFoundation`, `AVFoundation`, `AudioToolbox`, `CoreAudio`, `CoreMedia`, `CoreVideo`, `VideoToolbox`, `Metal`, `IOSurface`, `QuartzCore`, `UIKit` (framework)
- Android: `log`, `OpenSLES` (JNI 経由)

### 参照用 libwebrtc ヘッダ

ビルド後、`_install/<target>/<profile>/webrtc/include` 以下に libwebrtc のヘッダが展開される (CMake 単体ビルド時) または cargo の `OUT_DIR` 配下に展開される。C++ 側のシグネチャを確認するときはここを見る。

### Rust の build.rs 経由のビルド

通常はリポジトリルートから `cargo build` (デフォルト: prebuilt ダウンロード) または `cargo build --features source-build` を使う。`webrtc/` 配下を CMake で直接ビルドするユースケースは限定的。

### デバッグ

- VSCode で `lldb-dap` プラグインと lldb-dap バイナリをインストールし、`.vscode/launch.json` の各種パスを設定するとデバッグ実行可能

## プラットフォーム固有

- **Apple (macOS / iOS)**: `scripts/` の静的ライブラリ統合スクリプト + `sdk/objc/` 配下の `components/audio/audio_session.mm` (iOS) や `objc.mm`
- **Android**: `android.toolchain.cmake` + `sdk/android/` の JNI 連携 (`jni_export.cc`, `sdk/android/src/jni/jvm.cc`, `sdk/android/native_api/audio_device_module/audio_device_android.cc`, `sdk/android/native_api/codecs/wrapper.cc`, `sdk/android/native_api/jni/class_loader.cc`)
- **Objective-C**: `sdk/objc/` の `objc.mm` 経由 (iOS のみ追加コンパイル、macOS は本体側)

## WHIP / WHEP サンプル

- `src/whip.c` / `src/whep.c` が C API の利用例
- C++ 版 `src/whip.cpp` / `src/whep.cpp` も同梱し、C/C++ 双方からの利用を示す
- サンプルは **お手本** なので性能と堅牢性を両立させる方針 (リポジトリの `CLAUDE.md` を参照)

## ヘルパースクリプト `run.py`

- `python3 run.py format [--check]`: `webrtc/src/webrtc_c/` 配下に `clang-format -i` を適用 (`--check` で確認のみ)
- `python3 run.py iwyu <target> [--profile release] [--check]`: `clang-include-cleaner` を `compile_commands.json` 経由で実行 (`--check` 無しで `--edit` 付き)
  - `IWYU_EXCLUDED_GLOBS` で `android.h`, `ios.h`, `jni_export.{cc,h}`, `objc.h`, `objc.mm`, `sdk/**` を除外

ビルド・実行コマンドそのものは `run.py` に存在しない。ビルドは CMake 直叩きまたは `cargo build` 経由で行う。

## 関連

- Rust 側 API (`shiguredo_webrtc` crate): `shiguredo_webrtc` skill を参照
- 移植ルールの一次情報: `webrtc/RULES.md`
