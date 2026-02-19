# webrtc-rs

[![shiguredo_webrtc](https://img.shields.io/crates/v/shiguredo_webrtc.svg)](https://crates.io/crates/shiguredo_webrtc)
[![Documentation](https://docs.rs/shiguredo_webrtc/badge.svg)](https://docs.rs/shiguredo_webrtc)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## About Shiguredo's open source software

We will not respond to PRs or issues that have not been discussed on Discord. Also, Discord is only available in Japanese.

Please read <https://github.com/shiguredo/oss> before use.

## 時雨堂のオープンソースソフトウェアについて

利用前に <https://github.com/shiguredo/oss> をお読みください。

## 概要

libwebrtc の C API バインディングを Rust から安全に利用するためのライブラリです。

## 特徴

- [webrtc-build](https://github.com/shiguredo-webrtc-build/webrtc-build) でビルドされた libwebrtc を利用
- C++ の薄い C API ラッパー層と Rust の安全な API 層の 2 層構造
- `cargo build` だけでビルド
  - CMake / webrtc-build のダウンロード -> C++ コンパイル -> bindgen

## サポートプラットフォーム

- Ubuntu 24.04 LTS x86_64
- Ubuntu 22.04 LTS x86_64
- Ubuntu 24.04 LTS arm64
- Ubuntu 22.04 LTS arm64
- macOS Tahoe 26 arm64
- macOS Sequoia 15 arm64

### Ubuntu の対応バージョン

直近の LTS 2 バージョンをサポートします。

### macOS の対応バージョン

直近の 2 バージョンをサポートします。

### 将来対応予定

- Windows 11 x86_64
- Windows Server 2025 x86_64

## バージョニング

- メジャーバージョンは常に 0
- マイナーバージョンは libwebrtc の m バージョンと一致 (例: 0.145.x は m145)
- パッチバージョンは同一 m バージョン内での変更時にインクリメント

## 使い方

### 依存の追加

```toml
[dependencies]
shiguredo_webrtc = "0.145"
```

### PeerConnectionFactory の生成

```rust
use shiguredo_webrtc::{
    AudioDecoderFactory, AudioDeviceModule, AudioDeviceModuleAudioLayer,
    AudioEncoderFactory, AudioProcessingBuilder, Environment,
    PeerConnectionFactory, PeerConnectionFactoryDependencies,
    RtcEventLogFactory, Thread, VideoDecoderFactory, VideoEncoderFactory,
};
use std::sync::Arc;

pub struct FactoryHolder {
    factory: PeerConnectionFactory,
    _network: Thread,
    _worker: Thread,
    _signaling: Thread,
}

impl FactoryHolder {
    pub fn new() -> Option<Arc<Self>> {
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
        let event_log = RtcEventLogFactory::new();
        deps.set_event_log_factory(event_log);
        let adm = AudioDeviceModule::new(
            &env, AudioDeviceModuleAudioLayer::Dummy,
        ).ok()?;
        deps.set_audio_device_module(&adm);
        let audio_enc = AudioEncoderFactory::builtin();
        let audio_dec = AudioDecoderFactory::builtin();
        deps.set_audio_encoder_factory(&audio_enc);
        deps.set_audio_decoder_factory(&audio_dec);
        let video_enc = VideoEncoderFactory::builtin();
        let video_dec = VideoDecoderFactory::builtin();
        deps.set_video_encoder_factory(video_enc);
        deps.set_video_decoder_factory(video_dec);
        let apb = AudioProcessingBuilder::new_builtin();
        deps.set_audio_processing_builder(apb);
        deps.enable_media();

        let factory = PeerConnectionFactory::create_modular(&mut deps).ok()?;
        Some(Arc::new(Self {
            factory,
            _network: network,
            _worker: worker,
            _signaling: signaling,
        }))
    }
}
```

## 対応 API

### PeerConnection

- `PeerConnectionFactory`
  - PeerConnection の生成
- `PeerConnection`
  - 接続の管理
- `PeerConnectionFactoryDependencies`
  - ファクトリの依存関係設定
- `PeerConnectionFactoryOptions`
  - ファクトリオプション (暗号化無効化、SSL バージョン設定)
- `PeerConnectionDependencies`
  - PeerConnection の依存関係設定
- `PeerConnectionRtcConfiguration`
  - ICE / 接続設定
- `PeerConnectionOfferAnswerOptions`
  - Offer/Answer オプション (ICE リスタート、Simulcast レイヤー数など)
- `PeerConnectionObserver` / `PeerConnectionObserverBuilder`
  - イベントコールバック
- `PeerConnectionState`
  - 接続状態
- `CreateSessionDescriptionObserver`
  - SDP 生成コールバック
- `SetLocalDescriptionObserver`
  - ローカル SDP 設定コールバック
- `SetRemoteDescriptionObserver`
  - リモート SDP 設定コールバック

### メディア

- `AudioTrackSource` / `AudioTrack`
  - 音声トラック
- `VideoTrackSource` / `VideoTrack`
  - 映像トラック
- `AdaptedVideoTrackSource`
  - アダプティブ映像ソース
- `AdaptedSize` / `AdaptFrameResult`
  - アダプティブフレーム結果
- `AudioDeviceModule`
  - 音声デバイスインターフェース
- `AudioDeviceModuleAudioLayer`
  - 音声デバイスレイヤー種別 (PlatformDefault, Dummy など)
- `AudioDeviceModuleCallbacks` / `AudioDeviceModuleStats`
  - カスタム ADM コールバックと統計
- `AudioTransport` / `AudioTransportRef` / `AudioTransportCallbacks`
  - 音声トランスポート
- `MediaStreamTrack`
  - メディアストリームトラック
- `I420Buffer`
  - I420 フォーマットの映像バッファ
- `VideoFrame` / `VideoFrameRef`
  - 映像フレーム
- `VideoSink` / `VideoSinkBuilder`
  - 映像フレームシンク
- `VideoSinkWants`
  - 映像シンク要求設定
- `SdpVideoFormat`
  - 映像フォーマット

### 映像コーデック

- `VideoCodecRef` / `VideoCodecType` / `VideoCodecStatus`
  - 映像コーデック共通型とステータス
- `VideoFrameType` / `VideoFrameTypeVector` / `VideoFrameTypeVectorRef`
  - エンコード対象フレーム種別
- `EncodedImageBuffer` / `EncodedImage` / `EncodedImageRef`
  - エンコード済み映像データ
- `CodecSpecificInfo` / `CodecSpecificInfoRef` / `H264PacketizationMode`
  - コーデック固有情報
- `VideoEncoder`
  - カスタム映像エンコーダー
- `VideoEncoderCallbacks` / `VideoEncoderFactoryCallbacks`
  - エンコーダー / エンコーダーファクトリーの callback
- `VideoEncoderEncoderInfo` / `VideoEncoderSettingsRef` / `VideoEncoderRateControlParametersRef`
  - エンコーダー設定とメタ情報
- `VideoEncoderEncodedImageCallback` / `VideoEncoderEncodedImageCallbackRef`
  - エンコード完了 callback
- `VideoEncoderEncodedImageCallbackCallbacks`
  - エンコード完了 callback のハンドラー設定
- `VideoEncoderEncodedImageCallbackResult`
  - エンコード完了 callback の戻り値
- `VideoEncoderEncodedImageCallbackResultError`
  - エンコード完了 callback のエラーコード
- `VideoEncoderEncodedImageCallbackPtr`
  - C API 側 callback ポインターのラッパー
- `VideoDecoder`
  - カスタム映像デコーダー
- `VideoDecoderCallbacks` / `VideoDecoderFactoryCallbacks`
  - デコーダー / デコーダーファクトリーの callback
- `VideoDecoderDecoderInfo` / `VideoDecoderSettingsRef`
  - デコーダー設定とメタ情報
- `VideoDecoderDecodedImageCallbackRef`
  - デコード完了 callback

### RTP

- `RtpCapabilities`
  - コーデック能力
- `RtpCodecCapability`
  - 個別コーデック設定
- `RtpCodecCapabilityVector`
  - コーデック能力ベクタ
- `RtpEncodingParameters` / `RtpEncodingParametersVector`
  - エンコーディング設定
- `RtpParameters`
  - RTP 送信パラメータ
- `DegradationPreference`
  - 映像劣化方針
- `RtpTransceiver` / `RtpSender` / `RtpReceiver`
  - トランシーバー管理
- `RtpTransceiverDirection`
  - 送受信方向
- `RtpTransceiverInit`
  - トランシーバー初期化
- `MediaType`
  - メディア種別 (Audio / Video)

### DataChannel

- `DataChannel`
  - 双方向データ転送
- `DataChannelInit`
  - DataChannel 初期化設定 (ordered, protocol)
- `DataChannelObserver` / `DataChannelObserverBuilder`
  - データチャネルイベント
- `DataChannelState`
  - チャネル状態

### JSEP

- `SessionDescription`
  - SDP Offer/Answer
- `SdpType`
  - SDP タイプ (Offer, Answer, PrAnswer, Rollback)
- `IceCandidate`
  - ICE 候補
- `IceServer` / `IceServerVector`
  - ICE サーバー設定
- `IceTransportsType`
  - ICE トランスポートモード

### 統計

- `RTCStatsReport`
  - 統計レポート

### エラー

- `RtcError`
  - libwebrtc のエラー型
- `Error` / `Result`
  - Rust エラー型

### ユーティリティ

- `Environment`
  - WebRTC 環境の初期化
- `Thread`
  - スレッド管理
- `AudioEncoderFactory` / `AudioDecoderFactory`
  - 音声コーデックファクトリ
- `VideoEncoderFactory` / `VideoDecoderFactory`
  - 映像コーデックファクトリ
- `AudioProcessingBuilder`
  - 音声処理パイプライン
- `RtcEventLogFactory`
  - イベントログ
- `TimestampAligner`
  - タイムスタンプ調整
- `abgr_to_i420` / `i420_to_argb` / `nv12_to_i420` / `yuy2_to_i420`
  - カラーフォーマット変換 (libyuv)
- `random_bytes` / `random_string`
  - ランダム生成
- `time_millis` / `thread_sleep_ms`
  - 時間ユーティリティ
- `log` / `rtc_log_format_file`
  - ログ機能

## ビルド要件

- Rust 1.88 以上
- libclang (bindgen が利用)
- CMake / webrtc-build は build.rs が自動ダウンロード

## 環境変数

- `WEBRTC_C_TARGET`
  - ビルドターゲットを変更する場合に指定 (デフォルトはホスト環境に応じて自動判定)

## ライセンス

Apache License 2.0

```text
Copyright 2026-2026, Wandbox LLC (Original Author)
Copyright 2026-2026, Shiguredo Inc.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
