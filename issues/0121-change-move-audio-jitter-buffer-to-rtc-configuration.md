# AudioOptions の jitter buffer 設定を PeerConnectionRtcConfiguration に移動する

- Created: 2026-11-25
- Completed: {YYYY-MM-DD}
- Branch: feature/change-move-audio-jitter-buffer-to-rtc-configuration
- Polished: {YYYY-MM-DD}

## 目的

`AudioOptions` の `audio_jitter_buffer_max_packets` / `audio_jitter_buffer_fast_accelerate` / `audio_jitter_buffer_min_delay_ms` は、現在の Rust API では `PeerConnectionFactory::create_audio_source` にしか渡せず、その経路では libwebrtc の NetEq に反映されない。利用者が設定しても何も起きない misleading な API になっている。

NetEq の設定が実際に反映される `PeerConnectionRtcConfiguration` に移動し、`AudioOptions` は APM (AEC / AGC / NS / highpass / stereo swapping) 専用にする。あわせて、既存の無効な API を削除して設定場所を 1 つに統一する。DSCP の追加は本 issue の対象外とし、別 issue で扱う。

## 現状

- `src/api/audio.rs` の `AudioOptions` は 3 つの jitter buffer getter / setter を持つ
- `AudioOptions` は `PeerConnectionFactory::create_audio_source` に渡され、libwebrtc の `PeerConnectionFactory::CreateAudioSource` が `ApplyGlobalAudioOptions` 経由で `WebRtcVoiceEngine::ApplyOptions` に渡す
- `ApplyOptions` は echo cancellation / AGC / noise suppression / highpass filter / stereo swapping は扱うが、jitter buffer の 3 フィールドは扱わない
- NetEq の jitter buffer 設定は受信チャネル側で、`WebRtcVoiceReceiveChannel` のコンストラクタが `AudioOptions` の 3 フィールドを読む
- その受信チャネルに渡る `AudioOptions` は `SdpOfferAnswerHandler` が `RTCConfiguration` から生成するもので、`create_audio_source` とは別経路
- したがって、`AudioOptions` の 3 setter は現状 silent no-op である
- `PeerConnectionRtcConfiguration` には jitter buffer の設定口がない
- `src/tests.rs` の `audio_options_set_and_get_options` は 3 フィールドの往復だけを確認しており、NetEq への反映は確認していない

## 設計方針

- Rust の `AudioOptions` から 3 つの jitter buffer getter / setter を削除する。APM 系の 5 フィールドは残す
- C API の `webrtc_AudioOptions_*` は libwebrtc の `AudioOptions` を忠実に写した薄いラッパーなので残す。Rust の公開 API からは使わなくなる
- `PeerConnectionRtcConfiguration` に 3 つの getter / setter を追加する
  - `audio_jitter_buffer_max_packets`: `i32`
  - `audio_jitter_buffer_fast_accelerate`: `bool`
  - `audio_jitter_buffer_min_delay_ms`: `i32`
- C API は `webrtc_PeerConnectionInterface_RTCConfiguration_get_audio_jitter_buffer_*` / `_set_audio_jitter_buffer_*` を追加する。getter は `const`、setter は非 `const` とし、`webrtc/RULES.md` のフィールド則に合わせる
- `AudioOptions` の `Option<T>` は使わない。`RTCConfiguration` のフィールドは既定値を持つため、未設定にしたい場合は setter を呼ばない
- doc には次を明記する
  - 受信側の音声 (NetEq) の設定であり、送信側には影響しない
  - 既定値は 200 / false / 0
  - `max_packets` は 20 未満が 20 として扱われる。getter は設定値を返す
  - `min_delay_ms` は base minimum delay であり、目標遅延は `max_packets` とパケット長で決まる上限 (75%) を受ける
  - PeerConnection 生成時に固定され、生成後の `set_configuration` では変更できない (`INVALID_MODIFICATION`)
- 移行例を doc または CHANGES に示す
  - `options.set_audio_jitter_buffer_max_packets(Some(50))` → `config.set_audio_jitter_buffer_max_packets(50)`
- テスト
  - `audio_options_set_and_get_options` から jitter buffer の 3 フィールドの確認を削除する
  - `rtc_configuration_audio_jitter_buffer_round_trip` を追加し、既定値と設定値の往復を確認する。`max_packets` は 20 以上 (例: 123) を使う
  - `PeerConnection::set_configuration` に変更後の設定を渡すとエラーになることを確認するテストは、既存テストの構成を見て追加を判断する
- `CHANGES.md` の `## develop` に `[CHANGE]` (AudioOptions から削除) と `[ADD]` (RTCConfiguration に追加) を記載する
- 既定 prebuilt には新しい C API シンボルが無いため、検証は `--features source-build` を前提とする

## 完了条件

- `AudioOptions` から 3 つの jitter buffer getter / setter が削除されている
- `PeerConnectionRtcConfiguration` に 3 つの getter / setter があり、C API と Rust の両方から設定できる
- `cargo test --workspace --features source-build`、`cargo clippy --workspace --features source-build --all-targets -- -D warnings`、`cargo fmt --all -- --check`、`python3 webrtc/run.py format --check` が成功する
- `src/tests.rs` の jitter buffer のテストが新しい API に移り、20 未満の値を使っていない
- `CHANGES.md` に `[CHANGE]` と `[ADD]` が記載されている

## 解決方法

未着手
