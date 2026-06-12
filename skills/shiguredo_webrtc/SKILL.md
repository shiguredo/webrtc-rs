---
name: shiguredo_webrtc
description: 時雨堂の Rust 製 WebRTC ライブラリ shiguredo_webrtc (webrtc-rs) の機能・API リファレンス。PeerConnection、SDP/JSEP、DataChannel、AudioTrack/VideoTrack、RTP、DTLS、libyuv、Thread 管理、C++ 薄層ラッパー経由の libwebrtc 利用、ログマクロに関する質問時に使用。
---

# shiguredo_webrtc

libwebrtc の C API バインディングを Rust から安全に利用するためのライブラリ (webrtc-rs)。

## 特徴

- **2 層構造**: C++ 薄層 (`webrtc/` サブプロジェクトの C ラッパー) の上に Rust 安全 API を提供
- **prebuilt 利用**: `cargo build` のみで完結。デフォルトでは GitHub Releases から prebuilt 済みの `libwebrtc_c.a` (Windows は `webrtc_c.lib`) と `bindings.rs` を自動ダウンロードするため、CMake / libclang / C++ コンパイラのインストールは不要
- **`source-build` feature**: ソースから libwebrtc / C ラッパーをビルドするオプションも提供 (C++ 側を変更する場合や prebuilt が無いプラットフォーム向け)
- **マルチプラットフォーム**: Ubuntu 24.04/22.04 (x86_64/arm64)、Windows 11/Server 2025 (x86_64)、macOS Tahoe 26/Sequoia 15 (arm64)、Raspberry Pi OS (64-bit, Debian 13)
- **外部依存ゼロ**: 実行時は pure Rust バインディング (build 時のみ `bindgen`, `nojson`, `shiguredo_cmake`, `shiguredo_toml` を使用)

## バージョン情報

- crate 名: `shiguredo_webrtc`
- 現行バージョン: 0.150.x (libwebrtc m150 ベース、`webrtc-build = "m150.7871.0.0"`)
- Rust Edition: 2024
- 最小 Rust バージョン: 1.88
- ライセンス: Apache-2.0

### バージョニング規則

- メジャーバージョンは常に 0
- マイナーバージョンは libwebrtc の m バージョンと一致 (例: 0.150.x は m150)
- パッチバージョンは同一 m バージョン内での変更時にインクリメント

## ビルド設定 (`Cargo.toml` メタデータ)

- `[package.metadata.external-dependencies.webrtc-build]` で libwebrtc バージョンと URL を管理
- `[package.metadata.build-config]`:
  - `cmake-osx-deployment-target = "16.0"`
  - `android-platform = "android-24"`
  - `android-commandlinetools-version = "14742923"`
  - `android-ndk-version = "27.2.12479018"`
- features: `default = []`, `source-build`, `local-export`

## モジュール構成

`src/api/` 以下の 17 モジュールが `pub use api::*;` によりクレートルートから直接参照できる。

| モジュール | 主な型 | 用途 |
|----------|--------|------|
| `peer_connection` | `PeerConnection`, `PeerConnectionFactory`, `PeerConnectionFactoryDependencies`, `PeerConnectionFactoryOptions`, `PeerConnectionDependencies`, `PeerConnectionRtcConfiguration`, `PeerConnectionOfferAnswerOptions`, `PeerConnectionObserver`, `PeerConnectionObserverHandler`, `PeerConnectionState`, `IceConnectionState`, `IceGatheringState`, `IceCandidateError`, `IceTransportsType`, `TlsCertPolicy`, `ConnectionContext`, `NetworkManagerRef`, `PacketSocketFactoryRef`, `IceServer`, `IceServerRef`, `IceServerVector`, `IceServerVectorRef`, `CreateSessionDescriptionObserver`, `CreateSessionDescriptionObserverHandler`, `SetLocalDescriptionObserver`, `SetLocalDescriptionObserverHandler`, `SetRemoteDescriptionObserver`, `SetRemoteDescriptionObserverHandler` | 接続の生成と管理、ICE 設定、Observer |
| `audio` | `AudioTrack`, `AudioTrackSource`, `AudioTrackSink`, `AudioTrackSinkHandler`, `AudioEncoderFactory`, `AudioDecoderFactory`, `AudioProcessingBuilder` | 音声トラックとコーデック |
| `audio_device_module` | `AudioDeviceModule`, `AudioDeviceModuleAudioLayer`, `AudioDeviceModuleHandler`, `AudioDeviceModuleStats`, `AudioParameters`, `AudioTransport`, `AudioTransportRef`, `AudioTransportHandler` | プラットフォーム音声 I/O、カスタム ADM |
| `video` | `VideoTrack`, `VideoTrackSource`, `AdaptedVideoTrackSource`, `AdaptedSize`, `AdaptFrameResult`, `VideoSink`, `VideoSinkHandler`, `VideoSinkWants` | 映像トラックとフレーム配信 |
| `media_stream` | `MediaStream`, `MediaStreamTrack` | メディアストリーム抽象化 |
| `media_types` | `MediaType` | メディア種別 (audio/video/data) |
| `data_channel` | `DataChannel`, `DataChannelInit`, `DataChannelObserver`, `DataChannelObserverHandler`, `DataChannelState` | SCTP データチャネル |
| `jsep` | `SessionDescription`, `IceCandidate`, `IceCandidateRef`, `SdpType`, `SdpParseError` | SDP / ICE Candidate |
| `rtp` | `RtpTransceiver`, `RtpSender`, `RtpReceiver`, `RtpTransceiverInit`, `RtpTransceiverDirection`, `RtpCapabilities`, `RtpCodec`, `RtpCodecRef`, `RtpCodecCapability`, `RtpCodecCapabilityRef`, `RtpCodecCapabilityVector`, `RtpCodecCapabilityVectorRef`, `RtpEncodingParameters`, `RtpEncodingParametersRef`, `RtpEncodingParametersVector`, `RtpParameters`, `Resolution`, `Priority`, `DegradationPreference`, `default_bitrate_priority` | RTP 層の送受信 |
| `video_codec_common` | `VideoFrame`, `VideoFrameRef`, `VideoFrameBuilder`, `VideoFrameBuffer`, `VideoFrameBufferKind`, `VideoFrameBufferHandler`, `VideoFrameBufferHandlerAny`, `VideoFrameUpdateRect`, `VideoRotation`, `ColorSpace`, `I420Buffer`, `NV12Buffer`, `SdpVideoFormat`, `SdpVideoFormatRef`, `ScalabilityMode`, `VideoCodecRef`, `VideoCodecType`, `VideoCodecStatus`, `VideoFrameType`, `VideoFrameTypeVector`, `VideoFrameTypeVectorRef`, `EncodedImage`, `EncodedImageRef`, `EncodedImageBuffer`, `CodecSpecificInfo`, `CodecSpecificInfoRef`, `H264PacketizationMode` | フレーム・バッファ・コーデック共通 |
| `video_encoder` | `VideoEncoder`, `VideoEncoderHandler`, `VideoEncoderFactory`, `VideoEncoderFactoryHandler`, `VideoEncoderEncoderInfo`, `VideoEncoderSettingsRef`, `VideoEncoderRateControlParametersRef`, `VideoEncoderQpThresholds`, `VideoEncoderScalingSettings`, `VideoEncoderResolution`, `VideoEncoderResolutionBitrateLimits`, `VideoEncoderEncodedImageCallback`, `VideoEncoderEncodedImageCallbackRef`, `VideoEncoderEncodedImageCallbackHandler`, `VideoEncoderEncodedImageCallbackResult`, `VideoEncoderEncodedImageCallbackResultError`, `VideoEncoderEncodedImageCallbackPtr` ほか参照型 | 映像エンコーダー (組み込み + カスタム) |
| `video_decoder` | `VideoDecoder`, `VideoDecoderHandler`, `VideoDecoderFactory`, `VideoDecoderFactoryHandler`, `VideoDecoderDecoderInfo`, `VideoDecoderSettingsRef`, `VideoDecoderDecodedImageCallbackRef`, `VideoDecoderDecodedImageCallbackPtr` | 映像デコーダー (組み込み + カスタム) |
| `dtls_transport` | `DtlsTransport`, `DtlsTransportState`, `DtlsTransportObserver`, `DtlsTransportObserverHandler` | DTLS トランスポートと証明書検証連携 |
| `environment` | `Environment`, `EnvironmentRef` | WebRTC 環境 |
| `rtc_error` | `RtcError` | libwebrtc の `RTCError` ラッパー |
| `rtc_event_log` | `RtcEventLogFactory` | イベントログ |
| `stats` | `RTCStatsReport` | 統計情報 |

## クレートルート直下の再公開

`src/lib.rs` から参照できる主要型:

| 区分 | 型・関数 |
|------|---------|
| バージョン | `version()` |
| エラー | `Error`, `Result` |
| C++ 標準型ラッパー (`cxxstd`) | `CxxString`, `CxxStringRef`, `MapStringString`, `MapStringStringIter`, `StringVector`, `StringVectorRef` |
| libyuv | `LibyuvFourcc`, `LibyuvRotationMode`, `abgr_to_i420()`, `convert_from_i420()`, `convert_to_i420()`, `i420_copy()`, `i420_to_nv12()`, `mjpg_size()`, `mjpg_to_i420()`, `mjpg_to_nv12()`, `nv12_copy()`, `nv12_to_i420()`, `yuy2_to_i420()` |
| 参照カウント | `RefCountedHandle`, `ScopedRef` |
| rtc_base | `Thread`, `TimestampAligner`, `SSLCertChainRef`, `SSLCertificateRef`, `SSLCertificateVerifier`, `SSLCertificateVerifierHandler`, `SSLIdentity`, `log` (モジュール: `Severity`, `log_to_debug`, `enable_timestamps`, `enable_threads`, `print`), `random_bytes()`, `random_string()`, `rtc_log_format_file()`, `time_millis()` |
| ログマクロ (`#[macro_export]`) | `rtc_log_verbose!`, `rtc_log_info!`, `rtc_log_warning!`, `rtc_log_error!` |
| FFI | `ffi` (`bindgen` 生成の raw バインディング。通常は利用者が直接触らない) |

## 主要な Observer / Handler trait

コールバックは trait として定義されている。`Send` 必須。

| trait | 主なメソッド | 用途 |
|-------|-------------|------|
| `PeerConnectionObserverHandler` | `on_signaling_change`, `on_ice_candidate`, `on_ice_candidate_error`, `on_ice_connection_change`, `on_connection_change`, `on_track`, `on_add_stream`, `on_remove_stream`, `on_data_channel`, `on_renegotiation_needed`, `on_ice_gathering_change` ほか | PeerConnection のイベント購読 |
| `DataChannelObserverHandler` | `on_state_change`, `on_message`, `on_buffered_amount_change` | DataChannel のイベント購読 |
| `DtlsTransportObserverHandler` | DTLS 状態遷移コールバック | DTLS トランスポートイベント |
| `CreateSessionDescriptionObserverHandler` | SDP 生成完了コールバック | createOffer/createAnswer 結果 |
| `SetLocalDescriptionObserverHandler` / `SetRemoteDescriptionObserverHandler` | 設定完了コールバック | setLocalDescription/setRemoteDescription 結果 |
| `VideoSinkHandler` | `on_frame(VideoFrameRef)`, `on_discarded_frame()` | 映像フレームの受信 |
| `AudioTrackSinkHandler` | 音声サンプル受信コールバック | AudioTrack シンク |
| `SSLCertificateVerifierHandler` | 証明書検証ロジック | DTLS カスタム証明書検証 |
| `AudioDeviceModuleHandler` | デフォルト実装 + 部分上書き方式の各種コールバック | カスタム音声デバイス |
| `AudioTransportHandler` | 音声トランスポート | カスタム音声トランスポート |
| `VideoEncoderHandler` / `VideoEncoderFactoryHandler` | エンコード / ファクトリ | カスタム映像エンコーダー |
| `VideoDecoderHandler` / `VideoDecoderFactoryHandler` | デコード / ファクトリ | カスタム映像デコーダー |
| `VideoEncoderEncodedImageCallbackHandler` | エンコード完了通知 | エンコード結果受信 |
| `VideoFrameBufferHandler` / `VideoFrameBufferHandlerAny` | フレームバッファ実装 | カスタム映像バッファ |

## PeerConnectionFactory 構築フロー

README に記載のフローを要約。`PeerConnectionFactoryDependencies` にスレッドとファクトリ群を登録して `PeerConnectionFactory::create_modular_with_context()` を呼び、`(PeerConnectionFactory, ConnectionContext)` を受け取る。

```rust
use shiguredo_webrtc::{
    AudioDecoderFactory, AudioDeviceModule, AudioDeviceModuleAudioLayer,
    AudioEncoderFactory, AudioProcessingBuilder, ConnectionContext, Environment,
    PeerConnectionFactory, PeerConnectionFactoryDependencies,
    RtcEventLogFactory, Thread, VideoDecoderFactory, VideoEncoderFactory,
};

let env = Environment::new();
let mut network = Thread::new_with_socket_server();
let mut worker = Thread::new();
let mut signaling = Thread::new();
network.start();
worker.start();
signaling.start();

let mut deps = PeerConnectionFactoryDependencies::new();
deps.set_network_thread(&network);
deps.set_worker_thread(&worker);
deps.set_signaling_thread(&signaling);
deps.set_event_log_factory(RtcEventLogFactory::new());

let adm = AudioDeviceModule::new(&env, AudioDeviceModuleAudioLayer::Dummy)?;
deps.set_audio_device_module(&adm);
deps.set_audio_encoder_factory(&AudioEncoderFactory::builtin());
deps.set_audio_decoder_factory(&AudioDecoderFactory::builtin());
deps.set_video_encoder_factory(VideoEncoderFactory::builtin());
deps.set_video_decoder_factory(VideoDecoderFactory::builtin());
deps.set_audio_processing_builder(AudioProcessingBuilder::new_builtin());
deps.enable_media();

let (factory, context) =
    PeerConnectionFactory::create_modular_with_context(&mut deps)?;
```

### Thread の扱い

`Thread` は `start()` 済みのものを `set_*_thread` で登録する。ライフサイクル (停止・破棄) は呼び出し側で管理する必要がある。一般的には `FactoryHolder` 等でまとめて保持する。

### PeerConnection の TURN Proxy / 暗号化無効化

- `PeerConnectionFactoryOptions` で暗号化無効化を指定可能
- `PeerConnectionDependencies::set_proxy(...)` で TURN 用の HTTP Proxy を設定

## エラー型

- `Error`: libwebrtc 由来のエラーを Rust 側に流した結果 (詳細は `src/error.rs` 参照)
- `RtcError`: libwebrtc の `RTCError` ラッパー (コードと詳細メッセージを保持)
- `Result<T>`: `std::result::Result<T, Error>` のエイリアス

## 参照カウント管理

libwebrtc の `scoped_refptr` 相当を Rust 側で安全に扱うための型:

- `RefCountedHandle`: refcounted オブジェクトへのハンドル trait
- `ScopedRef<H>`: `H: RefCountedHandle` に対するスコープ付き参照
- 生ポインタを保持する型 (`PeerConnection`, `DataChannel`, `RtpTransceiver` 等) は `Send` / 適切な場合 `Sync` が実装されている

## libyuv

色フォーマット変換をバンドル。`LibyuvFourcc` (Argb, Bgra, Mjpg) でピクセルフォーマットを表現し、`LibyuvRotationMode` (Rotate0/90/180/270) で回転を指定する。主な関数:

- I420 ↔ 他フォーマット: `convert_from_i420`, `convert_to_i420`, `i420_to_nv12`, `nv12_to_i420`, `abgr_to_i420`, `yuy2_to_i420`
- コピー: `i420_copy`, `nv12_copy`
- MJPEG: `mjpg_size`, `mjpg_to_i420`, `mjpg_to_nv12`

## ログ機能

- `log` モジュール (`rtc_base::logging::log`):
  - `Severity` enum (`Verbose`, `Info`, `Warning`, `Error`, `None`, `Raw(i32)`)
  - `log_to_debug(severity)`, `enable_timestamps()`, `enable_threads()`, `print(severity, file, line, message)`
- ログマクロ: `rtc_log_verbose!`, `rtc_log_info!`, `rtc_log_warning!`, `rtc_log_error!`
  - 内部で `rtc_log_format_file(env!("CARGO_PKG_NAME"), file!())` を呼んで `<crate>::<filename>` 形式に整形する

## ビルド

- `cargo build`: prebuilt 済みの `libwebrtc_c.a` / `webrtc_c.lib` と `bindings.rs` を GitHub Releases から自動ダウンロード (CMake / libclang / C++ コンパイラ不要)
- `cargo build --features source-build`: ソースから C++ ラッパーと bindgen を実行 (`shiguredo_cmake` 経由で CMake を取得)
- ビルド設定は `Cargo.toml` の `[package.metadata.build-config]` / `[package.metadata.external-dependencies]` で管理
- 環境変数: `WEBRTC_C_TARGET` でターゲットを明示指定可能 (デフォルトはホスト環境から自動判定)

### 必要なツール

- prebuilt: Rust 1.88+ / `curl` / `tar` / (Linux のみ) `libx11-dev`
- source-build (Linux): 上記 + `build-essential`, `libclang-dev`
- source-build (macOS): 上記 + Xcode Command Line Tools
- source-build (Windows): 上記 + Visual Studio 2022 (`Desktop development with C++` + `C++ Clang tools for Windows`)

## ワークスペース構成

- ルートクレート `shiguredo_webrtc` (`/Cargo.toml`)
- サンプル群 `examples/*` (お手本として性能と堅牢性を両立させる方針)

## Rust 側で完結する範囲

本 skill は Rust API のみを対象とする。C++ → C ラッパー (`webrtc/` サブプロジェクト) の設計・実装・移植ルールは `libwebrtc_c` skill を参照。
