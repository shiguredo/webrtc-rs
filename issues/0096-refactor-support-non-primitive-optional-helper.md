# optional 用ヘルパーを非プリミティブ型でも扱えるようにする

- Created: 2026-08-30
- Completed: {YYYY-MM-DD}
- Branch: feature/refactor-optional-helper
- Polished: {YYYY-MM-DD}
- Milestone:

## 目的

`src/helper/optional.rs` の `get_optional` / `set_optional`（および `get_optional_bool` / `set_optional_bool` / `get_optional2` / `set_optional2`）は、参照する C API の値が「スカラー（数値・bool・数値表現できる enum 等）」の場合にしか使えない。構造体・文字列・スライス・借用参照・unique ポインタを値に持つ optional の get / set はヘルパーを使わず手動実装になっており、パターンが散らばっている。これを統一する。

## 現状

`get_optional<T: Default>` は `T::default()` で出力バッファを用意するため `T: Default` を要求し、かつ Rust の値型が C API の出力先（`*mut T`）と 1:1 で一致するスカラーに限られる。`set_optional` は値 `T` への `*const T` を渡すため、生の FFI ポインタを内包するラッパー型（例: `AudioCodecPairId`）には適用できない（`*const RustWrapper` と `*const webrtc_<CType>` が別型になるため）。

その結果、以下のように手動実装になっている（いずれも `let mut has = 0;` + 出力バッファ用意、または `match value { Some(v) => ... }` 形式）。

- `src/api/video_encoder.rs`
  - `VideoEncoderScalingSettings::thresholds`（`VideoEncoderQpThresholds`）
  - `VideoEncoderEncoderInfo::mapped_resolution`（`VideoEncoderResolution`）
  - `VideoEncoderEncoderInfo::get_encoder_bitrate_limits_for_resolution`（`VideoEncoderResolutionBitrateLimits`）
- `src/api/rtp.rs`
  - `RtpEncodingParameters::scale_resolution_down_to`（`Resolution`）
  - `RtpEncodingParameters::scalability_mode`（`Result<String>`）
  - `RtpEncodingParameters::codec`（`RtpCodecRef`）
  - `RtpParameters::degradation_preference`（`DegradationPreference`。値は `int` のため本来 `get_optional` + 変換で書けるにもかかわらず手動）
- `src/api/video_codec_common.rs`
  - `VideoFrame::color_space`（`ColorSpace`）
- `src/api/frame_transformer.rs`
  - `TransformableVideoFrameInterface::rid`（`Result<String>`）
  - `VideoFrameMetadata::dependencies`（`&[i64]`）

また、audio codec の `webrtc::AudioEncoderFactory::Options` の `codec_pair_id`（`AudioCodecPairId`、デフォルト構築不可）も同様の手動実装となる。

いずれも動作は正しいが、ヘルパーのスカラー限定により「スカラーはヘルパー、非プリミティブは手動」という二本立てになっている。

## 設計方針

下記のいずれかに統一する（詳細は対応時に確定する）。

- (a) `get_optional` / `set_optional` を拡張し、出力バッファの生成をクロージャで委譲する・ラッパーの生ポインタを渡せる形にする（`OptionalGet` / `OptionalSet` 相当を Rust 側へ持ち込む）
- (b) スカラーで済む箇所（`degradation_preference` など）をヘルパーへ寄せ、非プリミティブは「`as_ptr()` を渡す手動実装」に統一してパターンを整理する

## 完了条件

- 非プリミティブ型の optional についても、共通ヘルパーまたは統一した手動パターンで get / set が書けるようになっている
- `src/api/*.rs` の optional の get / set に見られる手動実装が二本立てにならず、パターンが統一されている

## 解決方法

（詳細は polish / 実装時に確定する）
