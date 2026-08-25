# FFI の optional 値 (has/value) を持つ getter/setter のボイラープレートを共通ヘルパー化する

- Created: 2026-08-26
- Completed: {YYYY-MM-DD}
- Branch: feature/refactor-optional-value-accessor
- Polished: {YYYY-MM-DD}

## 目的

C API が `out_has` / `out_value` 方式で optional 値を返し、`has` + 値ポインタ方式で optional 値を設定する getter/setter の実装が、アクセサごとに同じ定型的なコードを繰り返している。これを共通ヘルパーにまとめて、各アクセサの本体を数行に縮める。挙動は一切変更しない。

## 現状

C API (`webrtc/src/webrtc_c/api/*.h`) は optional 値を次の形で表現する。

- getter: `void get_x(struct X* self, int* out_has, TYPE* out_value)` (`self` は実体のポインタ)
- setter: `void set_x(struct X* self, int has, const TYPE* value)` (`has == 0` のとき `value` は null ポインタ)

これに対する Rust 側のボイラープレートは、アクセサごとに次の 2 パターンを毎回書き下している。

- getter

```rust
let mut has = 0;
let mut value = 0;
unsafe { ffi::get_x(self.raw.as_ptr(), &mut has, &mut value) };
if has == 0 { None } else { Some(value) }
```

- setter

```rust
match value {
    Some(v) => unsafe {
        ffi::set_x(self.raw.as_ptr(), 1, &v);
    },
    None => unsafe {
        ffi::set_x(self.raw.as_ptr(), 0, std::ptr::null());
    },
}
```

bool のときはさらに `let raw = if v { 1 } else { 0 };` による c_int への変換が加わる。

この同一パターンは以下のシンボルに点在する。getter の `if has == 0 { None } else ...` だけでも `src/` 配下に多数ある。

### `src/api/rtp.rs`

- `RtpCodec::clock_rate` / `RtpCodec::set_clock_rate` (i32)
- `RtpCodec::num_channels` / `RtpCodec::set_num_channels` (i32)
- `RtpEncodingParameters::ssrc` / `RtpEncodingParameters::set_ssrc` (u32)
- `RtpEncodingParameters::max_bitrate_bps` / `RtpEncodingParameters::set_max_bitrate_bps` (i32)
- `RtpEncodingParameters::min_bitrate_bps` / `RtpEncodingParameters::set_min_bitrate_bps` (i32)
- `RtpEncodingParameters::max_framerate` / `RtpEncodingParameters::set_max_framerate` (f64)
- `RtpEncodingParameters::scale_resolution_down_by` / `RtpEncodingParameters::set_scale_resolution_down_by` (f64)
- `RtpEncodingParameters::num_temporal_layers` / `RtpEncodingParameters::set_num_temporal_layers` (i32)

### `src/api/frame_transformer.rs`

- `TransformableFrame::receive_time` / `presentation_timestamp` / `capture_time` / `sender_capture_time_offset` (i64 の getter)
- `TransformableFrame::set_capture_time` (C が値をポインタでなく値渡しで受ける variant)
- `VideoFrameMetadata::frame_id` / `VideoFrameMetadata::set_frame_id` (i64)

### `src/api/video_encoder.rs`

- `VideoEncoderScalingSettings::thresholds` / `VideoEncoderScalingSettings::set_thresholds`
- `VideoEncoderEncoderInfo::is_qp_trusted` / `VideoEncoderEncoderInfo::set_is_qp_trusted` (bool)
- `VideoEncoderEncoderInfo::min_qp` / `VideoEncoderEncoderInfo::set_min_qp` (i32)
- `VideoEncoderEncoderInfo::mapped_resolution`
- `VideoEncoderEncoderInfo::get_encoder_bitrate_limits_for_resolution` (オブジェクトの `value.as_ptr()` に書き出す variant)

### `src/api/video_codec_common.rs`

- `VideoFrame::playback_time` / `VideoFrame::reference_time` (値を Duration へ変換する getter)

### `src/api/audio.rs` (`AudioOptions`)

- `AudioOptions::echo_cancellation` / `set_echo_cancellation` ほか 8 アクセサ (bool 6 個 + i32 2 個)

ただし `AudioOptions` は現在 develop に存在しない (未マージの作業ブランチ上の実装)。**この issue の対象に含めるが、`AudioOptions` の追加対応はその作業が develop へマージされた後に行う**。マージ前に一括で対応した場合、develop ベースのこのブランチでは `AudioOptions` を含むビルドができないため。

### 対象外の variant

以下の variant は共通化の主対象から外し、個別判断とする。

- 値がポインタでなく値渡しになる setter (`TransformableFrame::set_capture_time`)
- `has` フラグ付きだが値がオブジェクトで `value.as_ptr()` に書き出される getter / `v.as_ptr()` を渡す setter (`VideoEncoderScalingSettings::thresholds` など)。ここだけ `if has == 0 { None } else ...` の分岐整形を揃えるに留める

## 設計方針

- プリミティブ数値型 (`i32` / `u32` / `f64` / `i64`) 向けの共通ヘルパーを 1 箇所に定義する
  - `Option<T>` を直接扱うため、setter の `has` は `Some` なら 1 / `None` なら 0 に組み立て、`None` で `value` に null を渡す
  - getter は `out_has` / `out_value` で戻し、`has == 0` なら `None` を返す
- bool は `1` / `0` の c_int へ変換をヘルパー側へ寄せる
- ヘルパーの形 (ジェネリック関数 + 関数ポインタ引数、またはマクロ) は実装時に決める。FFI 関数は bindgen 生成の free function として存在するため、関数ポインタ引数として渡せる
- C API (`webrtc/`) と FFI 定義は変更しない
- `AudioOptions` は develop へのマージ後にこのヘルパーを使う形へ追従する

## 完了条件

- 上記の「現状」に列挙したアクセサが、getter は 1 箇所の共通ヘルパー呼び出し、setter は 1 行の共通ヘルパー呼び出しに置き換わっている
- 対象外 variant は `if has == 0 { None } else ...` の表現が揃っている
- getter / setter の挙動がリファクタ前と同一である (テストで担保する)
- ビルドと全テストが通る
- `AudioOptions` の対応は develop マージ後に行われている (未マージのまま進行してビルドが壊れていないこと)

## 解決方法

- `src/` 配下に共通ヘルパー (例: `get_optional` / `set_optional` 相当) を追加し、上記モジュールの該当アクセサを置き換える
- `src/tests.rs` に、対象アクセサの挙動が変わらないことを検証するテストを追加する (既存テストが存在するアクセサは既存テストで担保)
- `AudioOptions` は develop マージ後に、このヘルパーへ追従する
