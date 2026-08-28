# observer / callback の new_with_handler 登録・on_destroy 破棄の骨格を共通ヘルパー化する

- Created: 2026-08-28
- Completed: {YYYY-MM-DD}
- Branch: feature/refactor-observer-registration-helper
- Polished: {YYYY-MM-DD}

## 目的

コールバック型ハンドラ (observer / sink / callback) の生成・破棄の骨格が 19 型にわたって同一の形で繰り返されている。登録 (`Box::into_raw` → user_data → cbs → new → `expect_non_null_with_cleanup`) と破棄 (`on_destroy` での `Box::from_raw`) を共通ヘルパーに集約する。挙動は一切変更しない。

## 現状

次の骨格が 19 型で繰り返されている。

```rust
struct XxxHandlerState {
    handler: Box<dyn XxxHandler>,
}
// unsafe impl Send

pub fn new_with_handler(handler: Box<dyn XxxHandler>) -> Self {
    let user_data = Box::into_raw(Box::new(XxxHandlerState { handler })) as *mut c_void;
    let cbs = ffi::webrtc_X_cbs {
        ...: Some(xxx_trampoline),
        OnDestroy: Some(xxx_on_destroy),
    };
    let raw = expect_non_null_with_cleanup(
        unsafe { ffi::webrtc_X_new(&cbs, user_data) },
        "webrtc_X_new",
        || {
            let _ = Box::from_raw(user_data as *mut XxxHandlerState);
        },
    );
    Self { raw }
}

unsafe extern "C" fn xxx_on_destroy(user_data: *mut c_void) {
    assert!(!user_data.is_null());
    let _ = Box::from_raw(user_data as *mut XxxHandlerState);
}
```

該当 19 型 (いずれも `fn new_with_handler` を持つ):

- `VideoSink` (`src/api/video.rs`)
- `DataChannelObserver` (`src/api/data_channel.rs`)
- `DtlsTransportObserver` (`src/api/dtls_transport.rs`)
- `VideoFrameBuffer` (`src/api/video_codec_common.rs`)
- `VideoEncoder` / `VideoEncoderEncodedImageCallback` / `VideoEncoderFactory` (`src/api/video_encoder.rs`)
- `VideoDecoder` / `VideoDecoderFactory` (`src/api/video_decoder.rs`)
- `FrameTransformer` (`src/api/frame_transformer.rs`)
- `AudioTrackSink` (`src/api/audio.rs`)
- `AudioDeviceModule` / `AudioTransport` (`src/api/audio_device_module.rs`)
- `PeerConnectionObserver` / `CreateSessionDescriptionObserver` / `SetLocalDescriptionObserver` / `SetRemoteDescriptionObserver` (`src/api/peer_connection.rs`)
- `SSLCertificateVerifier` (`src/rtc_base/ssl_certificate.rs`)
- `LogSink` (`src/rtc_base/logging.rs`)

trampoline (extern "C" fn) 本体はコールバックのシグネチャごとに異なるため、本 issue の対象は「登録・破棄の骨格」に限定する。

## 設計方針

- マクロは使わない。ジェネリクスと普通のヘルパー関数で骨格を集約する
  - `XxxHandlerState { handler: Box<dyn XxxHandler> }` を `HandlerState<H>` としてジェネリック化する
  - 破棄 (`assert!` + `Box::from_raw`) をジェネリック関数に集約し、各型の on_destroy はそれを呼ぶだけにする
  - 登録時の「user_data を `c_void` 化し、new が null を返したら Box を回収してから panic」を共通ヘルパーに集約する
- trampoline 本体と `ffi::..._cbs` の各関数ポインタは各型のまま残す
- C API (`webrtc/src/webrtc_c/`) と bindgen 生成の FFI 定義は変更しない

## 完了条件

- 19 型すべての登録・破棄の骨格が共通ヘルパー経由になっている
- 生成失敗時の Box 回収・二重解放なし (従来と同じライフサイクル契約を満たす)
- 挙動がリファクタ前と同一である (panic の発生条件が変わらない)
- ビルドと全テストが通る
- `CHANGES.md` の develop に `### misc` エントリを追記する
