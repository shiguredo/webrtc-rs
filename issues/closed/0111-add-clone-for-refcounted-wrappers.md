# 参照カウントで複製できる公開型に Clone を実装する

- Created: 2026-10-05
- Completed: 2026-10-05
- Branch: feature/add-clone-for-refcounted-wrappers
- Polished: {YYYY-MM-DD}

## 目的

libwebrtc の `scoped_refptr` と同じように参照カウントで実体を共有できる公開型に `Clone` を実装し、利用側が同じ実体を指すハンドルを安全に増やせるようにする。

同じ実体を指すハンドルを増やす用途は、送信トラックのミュートのように「接続が所有している実体を別の場所からも操作したい」場合に発生する。現状は `Clone` を持つ型が一部に限られるため、利用側は生の C API (`webrtc_MediaStreamTrackInterface_AddRef` / `webrtc_MediaStreamTrackInterface_Release` など) を自前で宣言して参照カウントを再実装する必要がある。型ごとに `Clone` の有無がばらばらであること自体が、利用側に毎回「この型は複製できるのか」を調べさせる原因になっている。

## 現状

`ScopedRef` (`src/helper/ref_count.rs`) は `Clone` を実装しており、`ScopedRef::clone` が `RefCountedHandle::add_ref` を呼んで参照カウントを増やす。`ScopedRef` を保持する公開型のうち `Clone` が実装済みなのは次の 6 つで、残りの型には無い。

実装済み:

- `AdaptedVideoTrackSource` / `VideoTrackSource` / `VideoTrack` (`src/api/video.rs`)
- `VideoFrameBuffer` (`src/api/video_codec_common.rs`)
- `MediaStream` (`src/api/media_stream.rs`)
- `AudioDeviceModule` (`src/api/audio_device_module.rs`)

`Clone` が無い型:

| 型 | ファイル |
|---|---|
| `AudioTrack` / `AudioTrackSource` / `AudioDecoderFactory` / `AudioEncoderFactory` | `src/api/audio.rs` |
| `MediaStreamTrack` / `RtpSender` / `RtpReceiver` / `RtpTransceiver` | `src/api/rtp.rs` |
| `DataChannel` | `src/api/data_channel.rs` |
| `DtlsTransport` | `src/api/dtls_transport.rs` |
| `FrameTransformer` | `src/api/frame_transformer.rs` |
| `PeerConnection` / `PeerConnectionFactory` / `ConnectionContext` / `SetLocalDescriptionObserver` / `SetRemoteDescriptionObserver` | `src/api/peer_connection.rs` |
| `I420Buffer` / `NV12Buffer` / `EncodedImageBuffer` | `src/api/video_codec_common.rs` |

## 設計方針

`ScopedRef::clone` による `Clone` (実装済みの 6 型と同じ実装) を基本とする。実装してよい条件は、その型が共有実体への排他的アクセスを前提とする参照を貸し出さないことである。

`&mut self` を取るメソッドを持っていても、FFI を呼んで新しい値や `Result` を返すだけで共有実体の中身への `&mut` 参照を返さないなら `Clone` を実装してよい (`RtpSender::set_track` / `RtpSender::set_parameters` / `RtpSender::set_frame_transformer` / `RtpReceiver::set_frame_transformer` / `RtpTransceiver::set_codec_preferences` が該当する)。

### 実装する型

- `src/api/audio.rs`: `AudioTrack` / `AudioTrackSource` / `AudioDecoderFactory` / `AudioEncoderFactory`
- `src/api/rtp.rs`: `MediaStreamTrack` / `RtpSender` / `RtpReceiver` / `RtpTransceiver`
- `src/api/data_channel.rs`: `DataChannel`
- `src/api/dtls_transport.rs`: `DtlsTransport`
- `src/api/peer_connection.rs`: `PeerConnection` / `PeerConnectionFactory` / `ConnectionContext`
- `src/api/video_codec_common.rs`: `EncodedImageBuffer`

### 実装しない型

- `src/api/video_codec_common.rs` の `I420Buffer` / `NV12Buffer`: `data_mut` / `y_data_mut` / `planes_mut` が共有実体への `&mut [u8]` を返す。参照カウントを増やす `Clone` を実装すると、同じ画素メモリへの `&mut` を safe Rust で同時に 2 本作れる。この問題と `I420Buffer` の `Clone` の扱いは `issues/0107-bug-safe-api-data-race.md` が扱う
- `src/api/peer_connection.rs` の `SetLocalDescriptionObserver` / `SetRemoteDescriptionObserver`: ハンドラが `Send` のみを要求し `&mut self` で呼ばれる。`Clone` を実装すると同じ observer を複数の `PeerConnection` に渡せ、別々の signaling thread から同じハンドラへ同時に `&mut self` が入る経路ができる。1 回の `set_local_description` / `set_remote_description` にしか使わない型であり、複製する用途も無い
- `src/api/frame_transformer.rs` の `FrameTransformer`: Rustdoc で「1 エンドポイントにだけ設定する」ことが契約になっている。参照カウントで複製できるようにすると、同一の実体を複数エンドポイントへ設定する形を用意することになり契約と矛盾する

### 対象外 (参照カウントで複製できない型)

- `RtcEventLogFactory` (`src/api/rtc_event_log.rs`): `webrtc_RtcEventLogFactory_unique` を保持する唯一所有の型
- `VideoEncoder` (`src/api/video_encoder.rs`) / `VideoDecoder` (`src/api/video_decoder.rs`): `webrtc_VideoEncoder_unique` / `webrtc_VideoDecoder_unique` を保持する唯一所有の型
- `CreateSessionDescriptionObserver` (`src/api/peer_connection.rs`): C API に `Release` はあるが `AddRef` が無く、ハンドルを増やす手段が無い

## 完了条件

- 「実装する型」のすべてに `Clone` が実装されている
- 各型について、`Clone` で作ったハンドルが元のハンドルを drop した後も使えること (参照カウントが増えていること) と、`Clone` 経由の操作が元のハンドルから見えること (実体を共有していること) を `src/tests.rs` のテストで確認している
- 「実装しない型」に `Clone` が追加されていない
- `src/tests.rs` の既存テストがパスする
- `cargo fmt --all -- --check` / `cargo clippy --workspace --features source-build --all-targets -- -D warnings` / `cargo test --workspace --features source-build` が通る
- 公開 API の追加なので `CHANGES.md` の `## develop` に `[ADD]` が記載されている

## 解決方法

- 「実装する型」の 14 型に `ScopedRef::clone` で参照カウントを増やす `Clone` を実装した
  - `src/api/audio.rs` の `AudioTrack` / `AudioTrackSource` / `AudioDecoderFactory` / `AudioEncoderFactory`
  - `src/api/rtp.rs` の `MediaStreamTrack` / `RtpSender` / `RtpReceiver` / `RtpTransceiver`
  - `src/api/data_channel.rs` の `DataChannel`
  - `src/api/dtls_transport.rs` の `DtlsTransport`
  - `src/api/peer_connection.rs` の `PeerConnection` / `PeerConnectionFactory` / `ConnectionContext`
  - `src/api/video_codec_common.rs` の `EncodedImageBuffer`
- `Clone` の実装は既存の 6 型と同じく `raw_ref` を `ScopedRef::clone` で複製するだけにした
- `src/tests.rs` に `audio_refcounted_wrappers_clone` / `rtp_and_peer_connection_refcounted_wrappers_clone` / `encoded_image_buffer_clone` を追加した
  - 実体のポインタを持つ型はポインタの一致で、`RtpReceiver` と `DtlsTransport` は受信トラックの id と state の一致で実体の共有を確認する
  - `MediaStreamTrack` と `AudioTrack` は clone 経由の `set_enabled` が元のハンドルから見えることも確認する
  - いずれも元のハンドルを drop した後に clone を使えることを確認する
- `CHANGES.md` の `## develop` に `[ADD]` を追記した
- `cargo fmt --all -- --check` / `cargo clippy --workspace --features source-build --all-targets -- -D warnings` / `cargo test --workspace --features source-build` の成功を確認した
