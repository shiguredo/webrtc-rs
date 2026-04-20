---
name: shiguredo_webrtc
description: 時雨堂の Rust 製 WebRTC ライブラリ shiguredo_webrtc (webrtc-rs) の機能・API リファレンス。PeerConnection、SDP/JSEP、DataChannel、AudioTrack/VideoTrack、RTP、DTLS、libyuv、Thread 管理、C++ 薄層ラッパー経由の libwebrtc 利用に関する質問時に使用。
---

# shiguredo_webrtc

libwebrtc の C API バインディングを Rust から安全に利用するためのライブラリ (webrtc-rs)。

## 特徴

- **2 層構造**: C++ 薄層 (`webrtc/` サブプロジェクトの C ラッパー) の上に Rust 安全 API を提供
- **prebuilt 利用**: `cargo build` のみで完結。デフォルトでは GitHub Releases から prebuilt libwebrtc を自動ダウンロードするため、CMake / libclang / C++ コンパイラのインストールは不要
- **`source-build` feature**: ソースから libwebrtc をビルドするオプションも提供
- **マルチプラットフォーム**: Ubuntu 22.04/24.04 (x86_64/arm64)、Windows 11/Server 2025、macOS Sequoia/Tahoe (arm64)、Raspberry Pi OS (64-bit, Debian 13)
- **外部依存ゼロ**: 実行時は pure Rust バインディング (build 時のみ `bindgen` 等を使用)

## バージョン情報

- crate 名: `shiguredo_webrtc`
- バージョン: 0.147.x (libwebrtc m147 ベース)
- Rust Edition: 2024
- 最小 Rust バージョン: 1.88
- ライセンス: Apache-2.0

### バージョニング規則

- メジャーバージョンは常に 0
- マイナーバージョンは libwebrtc の m バージョンと一致 (例: 0.147.x は m147)
- パッチバージョンは同一 m バージョン内での変更時にインクリメント

## モジュール構成

`src/api/` 以下の 17 モジュールが `pub use api::*;` によりクレートルートから直接参照できる。

| モジュール | 主な型 | 用途 |
|----------|--------|------|
| `peer_connection` | `PeerConnection`, `PeerConnectionFactory`, `PeerConnectionFactoryDependencies`, `PeerConnectionObserverHandler`, `RTCConfiguration`, `IceServer`, `IceTransportsType`, `BundlePolicy`, `RtcpMuxPolicy` | 接続の生成と管理、ICE 設定 |
| `audio` | `AudioTrack`, `AudioTrackSource`, `AudioEncoderFactory`, `AudioDecoderFactory`, `AudioProcessingBuilder` | 音声トラックとコーデック |
| `video` | `VideoTrack`, `VideoTrackSource`, `AdaptedVideoTrackSource`, `VideoSinkHandler`, `VideoSinkWants` | 映像トラックとフレーム配信 |
| `media_stream` | `MediaStream`, `MediaStreamTrack` | メディアストリーム抽象化 |
| `media_types` | `MediaType` | メディア種別 (audio/video/data) |
| `data_channel` | `DataChannel`, `DataChannelInit`, `DataChannelObserverHandler`, `DataChannelState` | SCTP データチャネル |
| `jsep` | `SessionDescription`, `IceCandidate`, `IceCandidateRef`, `SdpType` | SDP / ICE Candidate |
| `rtp` | `RtpTransceiver`, `RtpSender`, `RtpReceiver`, `RtpTransceiverInit`, `RtpCapabilities` | RTP 層の送受信 |
| `video_encoder` / `video_decoder` | `VideoEncoderFactory`, `VideoDecoderFactory`, カスタム実装用 trait | 映像コーデック |
| `video_codec_common` | `VideoFrame`, `VideoFrameRef`, `VideoFrameBuffer`, `VideoRotation`, `ColorSpace` | フレーム・バッファ基盤 |
| `audio_device_module` | `AudioDeviceModule`, `AudioDeviceModuleAudioLayer` | プラットフォーム音声 I/O |
| `dtls_transport` | `DtlsTransport`, `SSLCertificateVerifierHandler` | DTLS トランスポートと証明書検証 |
| `environment` | `Environment`, `ConnectionContext` | WebRTC 環境 |
| `rtc_error` | `RtcError` | libwebrtc の `RTCError` ラッパー |
| `rtc_event_log` | `RtcEventLogFactory` | イベントログ |
| `stats` | `RTCStatsReport` | 統計情報 |

## クレートルート直下の再公開

`src/lib.rs` から参照できる主要型:

| 区分 | 型・関数 |
|------|---------|
| エラー | `Error`, `Result` |
| C++ 標準型ラッパー (`cxxstd`) | `CxxString`, `CxxStringRef`, `MapStringString`, `MapStringStringIter`, `StringVector`, `StringVectorRef` |
| libyuv | `LibyuvFourcc`, `abgr_to_i420()`, `convert_from_i420()`, `i420_copy()`, `i420_to_nv12()`, `nv12_copy()`, `nv12_to_i420()`, `yuy2_to_i420()` |
| 参照カウント | `RefCountedHandle`, `ScopedRef` |
| rtc_base | `Thread`, `TimestampAligner`, `SSLCertChainRef`, `SSLCertificateRef`, `SSLCertificateVerifier`, `SSLCertificateVerifierHandler`, `SSLIdentity`, `log()`, `random_bytes()`, `random_string()`, `rtc_log_format_file()`, `time_millis()` |
| FFI | `ffi` (`bindgen` 生成の raw バインディング。通常は利用者が直接触らない) |

## 主要な Observer / Handler trait

コールバックは trait として定義されている。Send 必須。

| trait | 主なメソッド | 用途 |
|-------|-------------|------|
| `PeerConnectionObserverHandler` | `on_signaling_change`, `on_ice_candidate`, `on_ice_candidate_error`, `on_ice_connection_change`, `on_connection_change`, `on_track`, `on_add_stream`, `on_remove_stream`, `on_data_channel`, `on_renegotiation_needed`, `on_ice_gathering_change` ほか | PeerConnection のイベント購読 |
| `DataChannelObserverHandler` | `on_state_change`, `on_message`, `on_buffered_amount_change` | DataChannel のイベント購読 |
| `VideoSinkHandler` | `on_frame(VideoFrameRef)`, `on_discarded_frame()` | 映像フレームの受信 |
| `SSLCertificateVerifierHandler` | 証明書検証ロジック | DTLS カスタム証明書検証 |

## PeerConnectionFactory 構築フロー

README.md に記載のフローを要約。`PeerConnectionFactoryDependencies` にスレッドとファクトリ群を登録して `PeerConnectionFactory::create_modular_with_context()` を呼ぶ。

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

## エラー型

- `Error`: libwebrtc 由来のエラーを Rust 側に流した結果 (詳細は `src/error.rs` 参照)
- `RtcError`: libwebrtc の `RTCError` ラッパー (コードと詳細メッセージを保持)
- `Result<T>`: `std::result::Result<T, Error>` のエイリアス

## 参照カウント管理

libwebrtc の `scoped_refptr` 相当を Rust 側で安全に扱うための型:

- `RefCountedHandle`: refcounted オブジェクトへのハンドル
- `ScopedRef<H>`: `H: RefCountedHandle` に対するスコープ付き参照
- 生ポインタを保持する型 (`PeerConnection`, `DataChannel`, `RtpTransceiver` 等) は `Send` / 適切な場合 `Sync` が実装されている

## libyuv

色フォーマット変換をバンドル。`LibyuvFourcc` でピクセルフォーマット (I420, NV12, YUY2, ABGR 等) を表現し、各関数で変換する。

## ビルド

- `cargo build`: prebuilt libwebrtc を `build.rs` が自動ダウンロードして bindgen でバインディング生成
- `cargo build --features source-build`: ソースから libwebrtc をビルド (`shiguredo_cmake` 経由)
- ビルド設定は `Cargo.toml` の `[package.metadata.build-config]` / `[package.metadata.external-dependencies]` で管理
  - libwebrtc バージョン: `webrtc-build = "m147.7727.10.0"`
  - macOS deployment target: 16.0
  - Android platform: android-24, NDK 27.2.12479018

## ワークスペース構成

- ルートクレート `shiguredo_webrtc` (`/Cargo.toml`)
- サンプル群 `examples/*` (お手本として性能と堅牢性を両立させる方針)

## Rust 側で完結する範囲

本 skill は Rust API のみを対象とする。C++ → C ラッパー (`webrtc/` サブプロジェクト) の設計・実装・移植ルールは `libwebrtc_c` skill を参照。
