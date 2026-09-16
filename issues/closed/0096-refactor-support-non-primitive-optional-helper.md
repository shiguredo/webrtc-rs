# optional 用ヘルパーを非プリミティブ型でも扱えるようにする

- Created: 2026-08-30
- Completed: 2026-09-16
- Branch: feature/refactor-optional-helper
- Polished: {YYYY-MM-DD}

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

`src/helper/optional.rs` のヘルパーを C API の値の種類ごとに揃え、`src/api/*.rs` に散っていた手動実装をすべてヘルパーの呼び出しに置き換えた。あわせて `webrtc_c` 側の optional のシグネチャも統一した。

- ヘルパーの構成を整理した
  - `has` を読んで `Option` に変換する部分を、モジュール内 private の `get_optional` / `set_optional` に集約した
  - 値の種類ごとのヘルパーはその薄いラッパーにした
    - `get_optional_scalar` / `set_optional_scalar`: スカラー（`Default` で初期化できる型）。既存の `get_optional` / `set_optional` をリネームしたもの
    - `get_optional_scalar2` / `set_optional_scalar2`: 2 つのスカラーの組。既存の `get_optional2` / `set_optional2` をリネームしたもの
    - `get_optional_object` / `set_optional_object`: 出力先 / 入力元が C オブジェクト。Rust 側のラッパーを生成して `as_ptr()` が返すポインタを渡す
    - `get_optional_ptr` / `set_optional_ptr`: 値が生ポインタ。getter は C API が出力したポインタを `Option<NonNull<U>>` で返し、所有 / 借用は C API の契約に従って呼び出し側が包む。setter は生ポインタをそのまま渡す（C API にこのシグネチャの setter がまだ無いため、現時点では呼び出し先が無い）
    - `get_optional_slice` / `set_optional_slice`: ポインタ + 長さで表されるデータ
  - どの C API のシグネチャにどのヘルパーを使うかを各ヘルパーのドキュメントに書いた
- `webrtc_c` の optional のシグネチャを統一した
  - `webrtc/RULES.md` に optional のルールを追記した（値の種類ごとの `int has` + `const T*`、getter の `out_has` という引数名、`has == 0` のときの扱い、`webrtc_c::OptionalGet` 系の利用）
  - `std::optional<webrtc::Timestamp>` を値渡ししていた `webrtc_TransformableFrameInterface_SetCaptureTime` / `webrtc_VideoFrameBuilder_set_presentation_timestamp_us` / `webrtc_VideoFrameBuilder_set_reference_time_us` を `const int64_t*` に変更し、C++ 実装を `webrtc_c::OptionalSetAs` に統一した。これにより Rust 側はこの 3 箇所を `set_optional_scalar` で書けるようになり、値渡し専用のヘルパーを増やさずに済んだ
  - ルールに合っていなかった既存箇所を直した（optional の getter の `has` 引数 6 件を `out_has` に統一、`webrtc_VideoFrame_color_space` が `has == 0` のとき値の出力先を書き換えないように `webrtc_c::OptionalGetAs` へ寄せる）
- 手動実装を置き換えた
  - `VideoEncoderScalingSettings::thresholds` / `VideoEncoderEncoderInfo::mapped_resolution` / `VideoEncoderEncoderInfo::get_encoder_bitrate_limits_for_resolution`
  - `RtpEncodingParameters::scale_resolution_down_to` / `scalability_mode` / `codec`
  - `RtpParameters::degradation_preference`（値が `int` のため `get_optional_scalar` + `DegradationPreference::from_int` / `to_int` へ寄せた）
  - `VideoFrame::color_space`
  - `TransformableVideoFrameInterface::rid`（`Option<Result<String>>` を `transpose` して `Result<Option<String>>` にする）
  - `VideoFrameMetadata::dependencies` / `set_dependencies`
  - `AudioEncoderFactoryOptions::set_codec_pair_id`（getter は C API が未設定を null で返す方式であり has / value 方式ではないため対象外とした）
  - 同じパターンの手動実装が残っていた `VideoFrameBuilder::set_presentation_timestamp` / `set_reference_time` / `set_color_space` / `set_update_rect`、`TransformableFrame::set_capture_time`、`LogLineRef::thread_id` も併せて置き換えた
- `RtpEncodingParameters::scalability_mode` の戻り値を `Option<Result<String>>` から `Result<Option<String>>` に変更した
- `CHANGES.md` の `## develop` にエントリ（`[CHANGE]` と `### misc`）を追加した
- `cargo clippy --workspace --features source-build -- -D warnings` / `cargo test --workspace --features source-build` / `python3 webrtc/run.py format --check` の成功を確認した
