---
name: libwebrtc_c
description: webrtc-rs リポジトリ配下の webrtc/ サブプロジェクト (libwebrtc C++ API の薄い C ラッパー) の機能・設計ルールリファレンス。命名規則、*_refcounted / *_unique メモリ管理、RULES.md の移植ルール、webrtc_c.h の公開 API、CMake ビルド、WHIP/WHEP サンプル、セルフチェック手順に関する質問時に使用。
---

# libwebrtc_c

`webrtc-rs` リポジトリの `webrtc/` サブプロジェクトに存在する、libwebrtc C++ API の薄い C ラッパー層。Rust 側の `shiguredo_webrtc` crate はこの C API を通じて libwebrtc を利用する。

## 位置づけ

- C ラッパーは **libwebrtc との薄い対応のみ** を実装する
- 便利関数や独自機能の追加は禁止
- 元の C++ API のシグネチャ・名前に忠実に移植する
- C ラッパーのファイルパスは元の C++ ファイルのパスと一致させる (分割をある程度サボることは許容)

## ディレクトリ構成

```
webrtc/
├── CMakeLists.txt          libwebrtc ダウンロード、LLVM/Clang 設定、ビルド構成 (852 行規模)
├── RULES.md                C++ → C 移植ルール (本 skill の根拠)
├── android.toolchain.cmake Android 向けツールチェーン
├── run.py                  ビルド/実行エントリポイント
├── scripts/                Apple 向け静的ライブラリ統合スクリプト
└── src/
    ├── webrtc_c.h          統合ヘッダ (全 C API を re-export)
    ├── whip.c, whep.c      WHIP/WHEP サンプル (C)
    └── webrtc_c/
        ├── api/            PeerConnection, JSEP, RTP, 統計, video codec, environment 等
        ├── pc/             connection_context (接続管理)
        ├── rtc_base/       暗号、SSL、ロギング、スレッド、タイムスタンプ
        ├── media/          AdaptedVideoTrackSource, SimulcastEncoderAdapter
        ├── modules/        ビデオコーデック関連定数
        ├── sdk/android/    JNI 関連 (jni_export.cc/.h)
        ├── sdk/objc/       Objective-C 関連 (objc.mm)
        ├── common.h, common.impl.h  共通マクロ
        ├── std.h, std.cc, std.impl.h  std::string / std::vector の C ラッパー
        └── libyuv.h, libyuv.cc        libyuv の C ラッパー
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

C++ のオブジェクト寿命管理を C 側でどう扱うかは **3 パターン** に分かれる。

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

### `std::optional<CppType>` の扱い

`struct T*` として扱い、`nullptr` を空値として表現する。引数に `CppType` を取る C++ 関数を移植するときは `struct T*` を受け取り、C++ 側の内部でコピー・ムーブを行う。

## 戻り値の扱い

### `RTCErrorOr<T>`

複数の戻り値を持つ型。出力引数に分割する。

- `RTCErrorOr<webrtc::scoped_refptr<CppType>>` の場合: `struct webrtc_RTCError*` と `struct CppType_refcounted*` を出力引数に追加

### `std::string`

`struct std_string_unique*` として返し、利用側は `std_string_unique_delete` で解放する。

## セルフチェック手順

新規 API 追加時・変更時は **必ず** 以下を実施する。

1. 作業開始前に RULES.md を読み直し、関係するルール (薄いラッパー、元の C++ パスと名前、命名規則、`*_unique` / `*_refcounted` の扱い) を箇条書きにする
2. 対応する C++ パスと型名を必ず開いて照合し、C 側のファイル・シンボル名が元の C++ に一致しているか確認する
3. `*_unique` / `*_refcounted` へのキャストが必ず `*_unique_get` / `*_refcounted_get` / `release` 経由になっているか `rg` でチェックする
4. 便利関数やパラメータ展開を追加していないか、各変更ブロックごとに「薄いラッパーか」を自問する
5. 変更後に再度 RULES.md を読み直し、全ルール順守をチェックリスト形式で確認してから回答する

## 統合ヘッダ `src/webrtc_c.h`

以下のヘッダを re-export する。

| ヘッダ | 主な対応 C API |
|--------|---------------|
| `api/peer_connection_interface.h` | `webrtc_PeerConnection`, `webrtc_RTCConfiguration`, `webrtc_IceServer`, 各種 Observer |
| `api/environment.h` | `webrtc_Environment` |
| `api/jsep.h` | `webrtc_SdpType`, `webrtc_SessionDescription_unique` |
| `api/ref_count.h` | `webrtc_RefCountInterface_ref`, `webrtc_RefCountInterface_Create` (C アプリ側から refcount に相乗りするため) |
| `api/rtc_error.h` | `webrtc_RTCError`, `webrtc_RTCErrorOr` 系 |
| `api/media_stream_interface.h` | `webrtc_MediaStream`, `webrtc_VideoTrack`, `webrtc_AudioTrack` |
| `api/video/*` | `webrtc_VideoFrame`, `webrtc_I420Buffer`, `webrtc_NV12Buffer`, `webrtc_ColorSpace`, `webrtc_EncodedImage`, `webrtc_VideoRotation` |
| `api/video_codecs/*` | `webrtc_VideoCodec`, `webrtc_VideoEncoder`, `webrtc_VideoDecoder`, `webrtc_SdpVideoFormat`, `webrtc_SimulcastStream` |
| `api/rtp_*.h` | `webrtc_RtpReceiver`, `webrtc_RtpSender`, `webrtc_RtpTransceiver` |

## C アプリから refcount に相乗りする

C アプリ側で `webrtc::scoped_refptr` に相互変換できるクラスを作りたい場合:

1. 構造体のメンバーに `webrtc_RefCountInterface_ref*` を持たせる
2. `webrtc_RefCountInterface_Create` を呼んで作成
3. 参照カウントが 0 になったときに呼ばれるコールバック関数を設定する

## ビルド

### コマンド

| 用途 | コマンド |
|------|---------|
| リリースビルド | `python3 run.py build ubuntu-24.04_x86_64` |
| リリース実行 (WHIP) | `./_build/ubuntu-24.04_x86_64/release/webrtc_c/whip_c` |
| ローカル libwebrtc を使ったデバッグビルド | `python3 run.py build ubuntu-24.04_x86_64 --local-webrtc-build-dir ../../webrtc-build/_worktree/<ver> --debug` |
| デバッグ実行 | `./_build/ubuntu-24.04_x86_64/debug/webrtc_c/whip_c` |

### 主要ビルドターゲット

| ターゲット | 種別 | 内容 |
|-----------|------|------|
| `webrtc_c` | 静的ライブラリ | C API 本体 (50+ ソースファイル) |
| `whip_c`, `whep_c` | 実行可能 (C) | WHIP/WHEP の C サンプル |
| `whip_cpp`, `whep_cpp` | 実行可能 (C++) | WHIP/WHEP の C++ サンプル |

### リンク対象

- `libwebrtc.a` (外部依存、prebuilt または webrtc-build で生成)
- `Threads::Threads`
- Windows: `winmm`, `ws2_32`
- Android: `OpenSLES` (JNI 経由)

### 参照用 libwebrtc ヘッダ

ビルド後、`./_install/ubuntu-24.04_x86_64/release/webrtc/include` 以下に libwebrtc のヘッダが展開される。C++ 側のシグネチャを確認するときはここを見る。

### デバッグ

- VSCode で `lldb-dap` プラグインと lldb-dap バイナリをインストールし、`../.vscode/launch.json` の各種パスを設定するとデバッグ実行可能

## プラットフォーム固有

- **Apple (macOS / iOS)**: `scripts/` の静的ライブラリ統合スクリプトを利用
- **Android**: `android.toolchain.cmake` + `sdk/android/` の JNI 連携
- **Objective-C**: `sdk/objc/` の `objc.mm` 経由

## WHIP / WHEP サンプル

- `src/whip.c` / `src/whep.c` が C API の利用例
- C++ 版 `whip.cpp` / `whep.cpp` も同梱し、C/C++ 双方からの利用を示す
- サンプルは **お手本** なので性能と堅牢性を両立させる方針 (リポジトリの `CLAUDE.md` を参照)

## 関連

- Rust 側 API (`shiguredo_webrtc` crate): `shiguredo_webrtc` skill を参照
- 移植ルールの一次情報: `webrtc/RULES.md`
