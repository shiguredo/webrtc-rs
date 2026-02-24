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

- [CHANGE] Ubuntu 24.04 LTS arm64 / Ubuntu 22.04 LTS arm64 / Raspberry Pi OS arm64 のクロスコンパイルに対応する
  - @voluntas
- [CHANGE] 状態を変更するメソッドのレシーバーを `&mut self` に統一する
  - `set_*` / `add_*` / `push*` / `init*` / `encode*` / `decode*` などの API を対象にする
  - サンプルとテストの呼び出し側も `mut` 前提へ更新する
- [UPDATE] video codec 関連 API のモジュール構成を整理する
  - `video_codec_common.rs` / `video_encoder.rs` / `video_decoder.rs` に分割する
  - `EncodedImage` 系、`VideoFrame` 系、`SdpVideoFormat` 系などの共通型を `video_codec_common.rs` に集約する
- [UPDATE] audio API のモジュール構成を整理する
  - `AudioDeviceModule` / `AudioTransport` 実装を `audio_device_module.rs` へ分離する
- [UPDATE] `Xxx` / `XxxRef` の API 一貫性を改善する
  - 公開操作メソッド名を揃え、`Xxx` 側は `XxxRef` へ委譲する
  - `Xxx` と `XxxRef` の定義位置を隣接化する
- [UPDATE] `user_data` の前提チェックを `assert!` に統一する
- [ADD] `VideoEncoderFactory` / `VideoDecoderFactory` / `VideoEncoder` / `VideoDecoder` の C API / Rust API を追加する
  - callback 構造体ベースで Rust 実装を差し込めるようにする
- [ADD] `video_error_codes.h` に対応する `VideoCodecStatus` を追加する
  - `VideoEncoder` / `VideoDecoder` の戻り値を `i32` ではなく `VideoCodecStatus` で扱えるようにする
- [FIX] `I420Buffer` の `width` / `height` 取得を C++ API 呼び出しへ変更する
  - Rust 側の `width` / `height` キャッシュ保持を廃止する
  - `webrtc_I420Buffer_width` / `webrtc_I420Buffer_height` を追加する
- [FIX] callback の寿命管理を `OnDestroy` 方式へ統一する
  - `AudioTransport` / `VideoSink` / `DataChannelObserver` / `PeerConnectionObserver` / `CreateSessionDescriptionObserver` / `SetLocalDescriptionObserver` / `SetRemoteDescriptionObserver` を対象にする
  - C callback 構造体へ `OnDestroy` を追加し、`*_new` / `*_make_ref_counted` の `cbs` 引数を `const` 化する

### misc

- [UPDATE] README から未使用の `WEBRTC_C_LIB_PATH` 環境変数の記載を削除する
- [UPDATE] CI のキャッシュ設定を調整する

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
