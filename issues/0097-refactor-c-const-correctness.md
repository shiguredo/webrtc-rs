# webrtc_c の C API の const 性を正しく設定する

- Created: 2026-08-30
- Completed: {YYYY-MM-DD}
- Branch: feature/refactor-c-const-correctness
- Polished: {YYYY-MM-DD}
- Milestone:

## 目的

webrtc_c の C API は、libwebrtc の C++ メソッドの const 性を正しく反映できていない箇所が多い。読み取り専用の getter が `const struct ...* self` になっていない、また `const` 引数を `const_cast` で外しているため、C++ 側の const 契約が崩れている。これを一括で是正する。

## 現状

### const_cast の使用（const 引数を外している）

C++ 側の引数が `const` であるにもかかわらず、C コールバック・公開関数が非 const ポインタを受けており、`const_cast` で外している。

- `webrtc_c/api/video_codecs/video_decoder.cc`
  - `VideoDecoder::Configure(const Settings&)` / `Decode(const EncodedImage&)` の転送で `const_cast`
- `webrtc_c/api/video_codecs/video_encoder_factory.cc`
  - `VideoEncoderFactory::Create(const Environment&, const SdpVideoFormat&)` の転送で `const_cast`
- `webrtc_c/api/video_codecs/video_encoder.cc`
  - `Encode(const EncodedImage&, ...)` / `InitEncode(const VideoCodec&, const Settings&)` / `RegisterEncodeCompleteCallback` / `Encode(const VideoFrame&)` などで `const_cast`

### getter に const が付いていない

読み取り専用で値返しの getter が `const struct ...* self` になっていない。

- `webrtc_c/api/video/video_frame_metadata.h` の `webrtc_VideoFrameMetadata_GetFrameType` 等
- `webrtc_c/api/video_codecs/sdp_video_format.h` の `webrtc_SdpVideoFormat_get_name` 等
- `webrtc_c/api/video_codecs/video_decoder.h` の `webrtc_VideoDecoder_DecoderInfo_get_implementation_name` 等
- audio 側（`webrtc_c/api/audio_codecs/`）の SdpAudioFormat / AudioCodecInfo / AudioEncoder / AudioDecoder の値返し getter も同様

## 設計方針

- C++ メソッド（または読み取り専用のフィールド参照）が `const` のものは、C 関数の `self` を `const struct ...*` にする
- `const` 引数（`const Environment&` / `const SdpAudioFormat&` 等）は C API でも `const struct ...*` にし、`const_cast` を廃止する（C++ 側に `const` をそのまま渡す）
- フィールドへの可変参照を返す getter（`get_parameters` 等）は既存慣例のとおり非 const のままとする

## 完了条件

- webrtc_c に `const_cast` が残っていない（取得・設定とも）
- 読み取り専用の値返し getter が `const struct ...* self` になっている
- Rust 側のラッパー（`src/api/*.rs`）が const ポインタを正しく扱えること（`*mut` から `*const` が bindgen で生成され、`as_ptr()` の implicit coercion でコンパイルが通る）

## 解決方法

（詳細は polish / 実装時に確定する）
