# AudioOptions の jitter buffer 設定を PeerConnectionRtcConfiguration に移動する

- Created: 2026-11-25
- Completed: {YYYY-MM-DD}
- Branch: feature/change-move-audio-jitter-buffer-to-rtc-configuration
- Polished: 2026-11-25

## 目的

`AudioOptions` の `audio_jitter_buffer_max_packets` / `audio_jitter_buffer_fast_accelerate` / `audio_jitter_buffer_min_delay_ms` は、現在の Rust API では `PeerConnectionFactory::create_audio_source` にしか渡せず、その経路では libwebrtc の NetEq に反映されない。利用者が設定しても何も起きない misleading な API になっている。

NetEq の設定が実際に反映される `PeerConnectionRtcConfiguration` に移動し、Rust の `AudioOptions` は音声処理 (AEC / AGC / NS / highpass / stereo swapping) 専用にする。あわせて、Rust の公開 API では既存の無効な API を削除して設定場所を 1 つに統一する。DSCP の追加は本 issue の対象外とする。

## 現状

- `src/api/audio.rs` の `AudioOptions` は 3 つの jitter buffer getter / setter を持つ
- `AudioOptions` は `PeerConnectionFactory::create_audio_source` に渡され、libwebrtc の `PeerConnectionFactory::CreateAudioSource` が `ApplyGlobalAudioOptions` 経由で `WebRtcVoiceEngine::ApplyOptions` に渡す
- `ApplyOptions` は echo cancellation / AGC / noise suppression / highpass filter / stereo swapping をエンジン全体の音声処理として扱うが、jitter buffer の 3 フィールドは扱わない
- NetEq の jitter buffer 設定は受信チャネル側で、`WebRtcVoiceReceiveChannel` のコンストラクタと `AddRecvStream` の `BuildReceiveStreamConfig` が `AudioOptions` の 3 フィールドを読む
- その受信チャネルに渡る `AudioOptions` は `SdpOfferAnswerHandler` が `RTCConfiguration` から生成するもので、`create_audio_source` とは別経路
- したがって、`AudioOptions` の 3 setter は値自体は保持するが、NetEq への効果がない
- `PeerConnectionRtcConfiguration` には jitter buffer の設定口がない
- `src/tests.rs` の `audio_options_set_and_get_options` は 8 フィールドの往復を確認しているが、jitter buffer の 3 フィールドについては NetEq への反映を確認していない

## 設計方針

- Rust の `AudioOptions` から 3 つの jitter buffer getter / setter を削除する。音声処理系の 5 フィールド (AEC / AGC / NS / highpass / stereo swapping) は残す
- C API の `webrtc_AudioOptions_*` は libwebrtc の `AudioOptions` を写した低レベル層として残す。jitter buffer 用の 3 設定は Rust の公開 API から使わなくなるが、音声処理系の 5 フィールドは引き続き使う。C API の `webrtc_AudioOptions_*` にある jitter buffer の 3 設定に対応する 6 関数 (3 getter + 3 setter) には、`CreateAudioSource` 経由では NetEq に反映されない旨をヘッダーにコメントする
- `PeerConnectionRtcConfiguration` に 3 つの getter / setter を追加する
  - `audio_jitter_buffer_max_packets`: `i32`
  - `audio_jitter_buffer_fast_accelerate`: `bool`
  - `audio_jitter_buffer_min_delay_ms`: `i32`
- C API は `webrtc_PeerConnectionInterface_RTCConfiguration_get_audio_jitter_buffer_*` / `_set_audio_jitter_buffer_*` を追加する。getter は `const`、setter は非 `const` とし、`webrtc/RULES.md` のフィールド則に合わせる
- 新しい `PeerConnectionRtcConfiguration` の API では `Option<T>` を使わない。`RTCConfiguration` のフィールドは `std::optional` ではなく既定値 200 / false / 0 を持つため、未設定は表現できない。setter を呼ばない場合は既定値が使われる
- doc には次を明記する
  - 受信側の音声 (NetEq) の設定であり、送信側には影響しない
  - 既定値は 200 / false / 0
  - `max_packets` は 20 未満が 20 として扱われる。getter は設定値を返す
  - `min_delay_ms` は base minimum delay であり、その実効上限は 10000 ms と `max_packets`・パケット長から決まる 75% の小さい方になる。目標遅延自体は 75% で上限を切られる
  - PeerConnection 生成時に固定され、生成後の `set_configuration` では変更できない (`INVALID_MODIFICATION`)
- 移行例を `CHANGES.md` に示す。`PeerConnectionRtcConfiguration` は `PeerConnection::create` に渡す前に設定する
  ```rust
  let mut config = PeerConnectionRtcConfiguration::new();
  config.set_audio_jitter_buffer_max_packets(50);
  let pc = PeerConnection::create(&factory, &config, deps)?;
  ```
- テスト
  - `audio_options_set_and_get_options` から jitter buffer の 3 フィールドの確認を削除する
  - `rtc_configuration_audio_jitter_buffer_round_trip` を追加し、既定値と設定値の往復を確認する。通常の往復は `max_packets` 20 以上 (例: 123) を使う。20 未満の値は `RTCConfiguration` の生値として保持されること (getter が 10 を返すこと) を別途確認し、NetEq の実効値が 20 であることをコメントする
  - `PeerConnection::set_configuration` に変更後の設定を渡すとエラーになることを確認するテストを追加する。`RtcError` は種別を公開していないため、エラーになることまでを確認する
- `CHANGES.md` の `## develop` に `[CHANGE]` (AudioOptions から削除) と `[ADD]` (RTCConfiguration に追加) を記載する
- 既定 prebuilt には新しい C API シンボルが無いため、検証は `--features source-build` を前提とする

## 完了条件

- Rust の `AudioOptions` から 3 つの jitter buffer getter / setter が削除され、音声処理系の 5 フィールドは残っている
- `PeerConnectionRtcConfiguration` に 3 つの getter / setter があり、C API の関数が追加され、Rust の getter / setter から設定できる
- C API の getter は `const`、setter は非 `const` になっている
- doc に受信側のみ、既定値 200 / false / 0、`max_packets` の 20 未満クランプと getter の生値、`min_delay_ms` の意味、生成時固定が書かれている
- C API の `webrtc_AudioOptions_*` にある jitter buffer の 3 設定に対応する 6 関数 (3 getter + 3 setter) に、`CreateAudioSource` 経由では NetEq に反映されない旨のコメントがある
- `PeerConnection::set_configuration` に変更後の設定を渡すとエラーになることを確認するテストがある
- `cargo test --workspace --features source-build`、`cargo clippy --workspace --features source-build --all-targets -- -D warnings`、`cargo fmt --all -- --check`、`python3 webrtc/run.py format --check` が成功する
- `src/tests.rs` の jitter buffer のテストが新しい API に移り、既定値・通常値の往復・20 未満の生値保持・`set_configuration` の拒否を確認している
- `CHANGES.md` に `[CHANGE]` と `[ADD]` が記載され、移行例が示されている

## 解決方法

未着手
