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

### misc

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
