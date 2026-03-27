# 変更履歴

- CHANGE
  - 下位互換のない変更
- UPDATE
  - 下位互換がある変更
- ADD
  - 下位互換がある追加
- FIX
  - バグ修正

## develop

- [ADD] `PeerConnectionFactory::get_rtp_receiver_capabilities` を追加する
  - @voluntas
- [UPDATE] iOS / Android 向け CMake プラットフォーム値を Cargo.toml から取得する
  - `CMAKE_OSX_DEPLOYMENT_TARGET` と `ANDROID_PLATFORM` の固定値を `package.metadata.build-config` に移動する
  - @melpon
- [ADD] Android `source-build` 時に Android NDK の自動セットアップを追加する
  - `ANDROID_NDK_HOME` / `ANDROID_NDK` が未設定または無効な場合に `target/android-sdk` へ Command-line Tools と NDK を自動インストールする
  - Command-line Tools / NDK のバージョンを `package.metadata.build-config` から指定可能にする
  - @melpon
- [ADD] `PeerConnection::remove_track` を追加する
  - `PeerConnectionInterface::RemoveTrackOrError` の C API / Rust バインディングを追加する
  - @voluntas
- [ADD] `RtpSender::set_track` を追加する
  - `RtpSenderInterface::SetTrack` の C API / Rust バインディングを追加する
  - @voluntas
- [ADD] `VideoEncoder::EncoderInfo` の全フィールドと関連型 API を追加する
  - `QpThresholds` / `ScalingSettings` / `ResolutionBitrateLimits` / `Resolution` を追加する
  - `ToString` / `GetEncoderBitrateLimitsForResolution` を追加する
  - `fps_allocation` / `preferred_pixel_formats` / optional フィールド操作を追加する
  - @melpon
- [CHANGE] AudioDeviceModule の `AudioParameters` / `Stats` C API を C++ の opaque API に変更する
  - `webrtc_AudioParameters` / `webrtc_AudioDeviceModule_Stats` の公開 field を廃止する
  - `GetPlayoutAudioParameters` / `GetRecordAudioParameters` / `GetStats` callback の out 引数を `*_unique**` に変更する
  - `webrtc_AudioDeviceModule_GetStats` の out 引数を `webrtc_AudioDeviceModule_Stats_unique**` に変更する
  - Rust の `AudioDeviceModuleHandler` で `get_playout_audio_parameters` / `get_record_audio_parameters` を out 引数で受ける形式に変更する
  - Rust 型名 `AudioDeviceModuleAudioParameters` を `AudioParameters` に変更する
  - @melpon
- [CHANGE] 公開 handler trait の `impl ... for ()` を削除する
  - `src` / `src/rtc_base` の公開 handler trait 15 箇所から `impl ... for ()` を削除する
  - `src/tests.rs` の `Box::new(())` をテスト専用 `NoopHandler` へ置換する
  - @melpon

## 0.146.1

**リリース日**: 2026-03-24

- [ADD] `PeerConnection::close` を追加する
  - @voluntas
- [ADD] `DtlsTransport` / `DtlsTransportObserver` / `DtlsTransportState` を追加する
  - @voluntas
- [ADD] `PeerConnection::lookup_dtls_transport_by_mid` を追加する
  - `DtlsTransport` のインスタンスを取得するために必要
  - @voluntas
- [UPDATE] libwebrtc m146 (m146.7680.3.1) に上げる
  - @voluntas
- [ADD] `AudioSinkHandler` / `AudioSink` を追加する
  - @voluntas
- [ADD] `AudioTrack` に `add_sink()` / `remove_sink()` を追加する
  - @voluntas
- [ADD] `MediaStreamTrack` に `cast_to_audio_track()` を追加する
  - @voluntas
- [ADD] トラックの有効/無効を制御する `MediaStreamTrack::enabled` / `MediaStreamTrack::set_enabled` を追加する
  - @voluntas
- [ADD] `IceServer::urls_len` を追加する
  - @melpon
- [ADD] Windows に対応する
  - @melpon

## 0.146.0

**リリース日**: 2026-03-15

- [CHANGE] VideoEncoder / VideoDecoder の API を handler と 1 対 1 に統一する
  - `VideoEncoder::init_encode` / `encode` / `set_rates` を handler と同じ引数に統一する
  - `VideoDecoder::configure` / `decode` を handler と同じ引数に統一する
  - `VideoEncoder::init_encode()` / `VideoEncoder::set_rates()` / `VideoDecoder::configure()` の 0 引数 API を削除する
  - `webrtc_GetDefaultVideoFormats` と `get_default_video_formats` を削除し、`webrtc-c` の独自 convenience API をなくす
  - @melpon
- [CHANGE] VideoEncoder / VideoDecoder / factory の callback API を handler trait 形式に変更する
  - `VideoEncoderHandler` / `VideoDecoderHandler` / `VideoEncoderFactoryHandler` / `VideoDecoderFactoryHandler` / `VideoEncoderEncodedImageCallbackHandler` を追加する
  - `new_with_callbacks` を `new_with_handler` に変更する
  - `VideoDecoderDecodedImageCallbackPtr` を追加し、decode 完了 callback 登録の受け取り型を `Option<VideoDecoderDecodedImageCallbackPtr>` に統一する
  - @melpon
- [CHANGE] ほぼ全ての callback API を handler trait 形式に統一する
  - `VideoSink` / `DataChannelObserver` / `PeerConnectionObserver` / `CreateSessionDescriptionObserver` / `SetLocalDescriptionObserver` / `SetRemoteDescriptionObserver` / `AudioTransport` / `AudioDeviceModule` を対象にする
  - `*Builder` / `*Callbacks` / closure 直渡し `new(...)` を削除する
  - `PeerConnection::get_stats(FnOnce)` は変更しない
  - @melpon
- [CHANGE] libyuv 変換 API 名を libyuv と 1 対 1 に統一する
  - `i420_to_argb` を削除し、`convert_from_i420` と `LibyuvFourcc` を追加する
  - `i420_to_nv12` を追加する
  - @melpon
- [ADD] TURN 用 HTTP Proxy 設定 API を追加する
  - `PeerConnectionDependencies::set_proxy(...)` を追加する
  - `NetworkManagerRef` / `PacketSocketFactoryRef` を追加する
  - `ConnectionContext` を追加する
  - `PeerConnectionFactory::create_modular_with_context(...)` を追加する
  - `ConnectionContext::default_network_manager()` / `default_socket_factory()` を追加する
  - @melpon
- [ADD] `SdpVideoFormat` の parameters / scalability modes を扱う API を追加する
  - `ScalabilityMode` 型を追加する
  - `SdpVideoFormat::new_with_parameters` と `SdpVideoFormat::scalability_modes` を追加する
  - `SdpVideoFormatRef::scalability_modes` を追加する
  - @melpon
- [ADD] `VideoCodecType` の文字列変換 API を追加する
  - `VideoCodecType::as_str`
  - `TryFrom<&str> for VideoCodecType`
  - `FromStr for VideoCodecType`
  - @melpon
- [UPDATE] libwebrtc m146 (m146.7680.0.0) に上げる
  - @voluntas
- [UPDATE] cmake の代わりに shiguredo_cmake を利用する
  - @melpon
- [ADD] `version()` 関数を追加する
  - @voluntas
- [ADD] PeerConnectionObserverHandler に `on_standardized_ice_connection_change` / `on_ice_gathering_change` / `on_ice_candidate_error` コールバックを追加する
  - `IceConnectionState` / `IceGatheringState` / `IceCandidateError` 型を追加する
  - @voluntas
- [ADD] DtlsTransportInterface と PeerConnection の ICE/DTLS 関連 C API を追加する
  - @voluntas
- [ADD] TLS 証明書検証向け C API / Rust API を追加する
  - `TlsCertPolicy` と `IceServer::set_tls_cert_policy` を追加する
  - `SSLCertificateRef` / `SSLCertChainRef` と `SSLCertificateVerifier` を追加する
  - `PeerConnectionDependencies::set_tls_cert_verifier` を追加する
  - @melpon @voluntas

### misc

- [ADD] prebuilt リリース時に SHA256 チェックサムファイルを生成する
  - @voluntas
- [ADD] prebuilt ダウンロード時に SHA256 チェックサムを検証する
  - @voluntas
- [UPDATE] prebuilt ダウンロードに rustls / flate2 / tar クレートの代わりに curl + tar コマンドを利用する
  - @voluntas
- [UPDATE] ci.yml を main ブランチとタグでは実行しないようにする
  - @voluntas

## 0.145.2

**リリース日**: 2026-02-26

- [CHANGE] Ubuntu 24.04 LTS arm64 / Ubuntu 22.04 LTS arm64 / Raspberry Pi OS arm64 のクロスコンパイルに対応する
  - @voluntas
- [CHANGE] 状態を変更するメソッドのレシーバーを `&mut self` に統一する
  - `set_*` / `add_*` / `push*` / `init*` / `encode*` / `decode*` などの API を対象にする
  - サンプルとテストの呼び出し側も `mut` 前提へ更新する
  - @melpon
- [UPDATE] video codec 関連 API のモジュール構成を整理する
  - `video_codec_common.rs` / `video_encoder.rs` / `video_decoder.rs` に分割する
  - `EncodedImage` 系、`VideoFrame` 系、`SdpVideoFormat` 系などの共通型を `video_codec_common.rs` に集約する
  - @melpon
- [UPDATE] audio API のモジュール構成を整理する
  - `AudioDeviceModule` / `AudioTransport` 実装を `audio_device_module.rs` へ分離する
  - @melpon
- [UPDATE] `Xxx` / `XxxRef` の API 一貫性を改善する
  - 公開操作メソッド名を揃え、`Xxx` 側は `XxxRef` へ委譲する
  - `Xxx` と `XxxRef` の定義位置を隣接化する
  - @melpon
- [CHANGE] `VideoEncoderFactory` / `VideoDecoderFactory` の `create` を `Ref` 受け取りに統一し、`create_from_ref` を削除する
  - @melpon
- [CHANGE] `RtpCodecCapabilityVector` / `RtpCodecCapabilityVectorRef` の `push` / `set` を `Ref` 受け取りに統一する
  - `push_ref` を削除する
  - @melpon
- [CHANGE] `SdpVideoFormat` の `is_equal` を所有型 API に統一する
  - `SdpVideoFormat::is_equal(SdpVideoFormatRef)` に変更する
  - `SdpVideoFormatRef::is_equal` を削除する
  - @melpon
- [CHANGE] `SdpVideoFormat` の複製経路を C++ 側 copy API に統一する
  - C API `webrtc_SdpVideoFormat_copy` を追加する
  - Rust 側で `SdpVideoFormat: Clone` と `SdpVideoFormatRef::to_owned` を追加する
  - @melpon
- [CHANGE] `VideoFrame` 生成 API を `from_i420` に統一する
  - `VideoFrame::from_i420(buffer, timestamp_us, timestamp_rtp)` に変更する
  - `VideoFrame::from_i420_with_timestamp_rtp` を削除する
  - C API を `webrtc_VideoFrame_Create(buffer, rotation, timestamp_us, timestamp_rtp)` に統一する
  - `webrtc_VideoFrame_Create_with_timestamp_rtp` を削除する
  - @melpon
- [UPDATE] `user_data` の前提チェックを `assert!` に統一する
  - @melpon
- [ADD] リリース時に prebuilt `libwebrtc_c.a` と `bindings.rs` を GitHub Releases に配布し、`cargo build` 時に自動ダウンロードする
  - `--features source-build` でソースビルドに切り替え可能
  - @voluntas
- [ADD] `VideoEncoderFactory` / `VideoDecoderFactory` / `VideoEncoder` / `VideoDecoder` の C API / Rust API を追加する
  - callback 構造体ベースで Rust 実装を差し込めるようにする
  - @melpon
- [ADD] custom video codec factory 実装向けの C API / Rust API を追加する
  - `VideoDecoderDecodedImageCallbackRef::decoded`
  - `VideoEncoderFactory::get_supported_formats`
  - `VideoDecoderFactory::get_supported_formats`
  - `VideoEncoderFactory::create`
  - `VideoDecoderFactory::create`
  - `VideoFrame::rtp_timestamp`
  - `VideoFrameRef::rtp_timestamp`
  - `VideoFrame::from_i420(buffer, timestamp_us, timestamp_rtp)`
  - `I420Buffer::y_data_mut` / `u_data_mut` / `v_data_mut`
  - `SdpVideoFormatRef::as_ptr` (`pub(crate)`)
  - `VideoFrameRef::as_ptr` (`pub(crate)`)
  - callback / factory / timestamp / mutable plane の回帰テストを追加する
  - @melpon
- [ADD] `video_error_codes.h` に対応する `VideoCodecStatus` を追加する
  - `VideoEncoder` / `VideoDecoder` の戻り値を `i32` ではなく `VideoCodecStatus` で扱えるようにする
  - @melpon
- [FIX] `I420Buffer` の `width` / `height` 取得を C++ API 呼び出しへ変更する
  - Rust 側の `width` / `height` キャッシュ保持を廃止する
  - `webrtc_I420Buffer_width` / `webrtc_I420Buffer_height` を追加する
  - @melpon
- [FIX] callback の寿命管理を `OnDestroy` 方式へ統一する
  - `AudioTransport` / `VideoSink` / `DataChannelObserver` / `PeerConnectionObserver` / `CreateSessionDescriptionObserver` / `SetLocalDescriptionObserver` / `SetRemoteDescriptionObserver` を対象にする
  - C callback 構造体へ `OnDestroy` を追加し、`*_new` / `*_make_ref_counted` の `cbs` 引数を `const` 化する
  - @melpon

### misc

- [UPDATE] shiguredo_http11 を 2026.1.0 に上げる
  - @voluntas
- [UPDATE] README から未使用の `WEBRTC_C_LIB_PATH` 環境変数の記載を削除する
  - @melpon
- [UPDATE] CI のキャッシュ設定を調整する
  - @melpon

## 0.145.1

**リリース日**: 2026-02-17

- [ADD] Ubuntu 24.04 LTS arm64 / Ubuntu 22.04 LTS arm64 をサポート
  - @voluntas
- [ADD] `IceCandidate` と `PeerConnection::add_ice_candidate` の C API / Rust API を追加する
  - `IceCandidate::new`
  - `IceCandidate::sdp_mid`
  - `IceCandidate::sdp_mline_index`
  - `PeerConnection::add_ice_candidate`
  - @voluntas
- [ADD] `RtpParameters` と `RtpSender::get_parameters` / `set_parameters` の C API / Rust API を追加する
  - `RtpParameters`
  - `DegradationPreference`
  - `RtpSender::get_parameters`
  - `RtpSender::set_parameters`
  - `RtpEncodingParameters` の optional フィールド API を `out_has + ポインタ値` 方式に統一
  - @melpon

## 0.145.0

**リリース日**: 2026-02-12
