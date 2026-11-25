# PeerConnectionRtcConfiguration に audio jitter buffer 設定の取得・設定口を追加する

- Created: 2026-11-25
- Completed: {YYYY-MM-DD}
- Branch: feature/add-audio-jitter-buffer-to-rtc-configuration
- Polished: 2026-11-25

## 目的

Rust SDK から受信音声の NetEq jitter buffer を設定できるようにする。

`AudioOptions` の `audio_jitter_buffer_max_packets` / `audio_jitter_buffer_fast_accelerate` / `audio_jitter_buffer_min_delay_ms` は libwebrtc の `AudioOptions` に存在し、受信チャネルの構築時に読まれる。しかしその `AudioOptions` は `RTCConfiguration` から生成される別インスタンスであり、Rust API の `AudioOptions` を渡せる `PeerConnectionFactory::create_audio_source` の経路では NetEq に反映されない。`PeerConnectionRtcConfiguration` にも設定口がないため、Rust SDK から jitter buffer を設定する方法が存在しない。

`PeerConnectionRtcConfiguration` に 3 設定の取得・設定口を追加し、実際に NetEq へ反映される経路を提供する。DSCP の追加は本 issue の対象外とする。

## 現状

- libwebrtc m157 の `webrtc::PeerConnectionInterface::RTCConfiguration` は `audio_jitter_buffer_max_packets` (既定値 200) / `audio_jitter_buffer_fast_accelerate` (既定値 false) / `audio_jitter_buffer_min_delay_ms` (既定値 0) を持つ
- `SdpOfferAnswerHandler` は PeerConnection 生成時に `RTCConfiguration` から `AudioOptions` を生成し、その値が `RtpTransceiver` 経由で受信チャネルに渡る
- `WebRtcVoiceReceiveChannel` のコンストラクタは `audio_jitter_buffer_max_packets` を `std::max(20, value)` にして保持し、`AddRecvStream` の `BuildReceiveStreamConfig` が 3 設定を `AudioReceiveStreamInterface::Config` に設定する
- 一方、`PeerConnectionFactory::CreateAudioSource` は `ConnectionContext::ApplyGlobalAudioOptions` 経由で `WebRtcVoiceEngine::ApplyOptions` に `AudioOptions` を渡すが、`ApplyOptions` は 3 設定を扱わない
- Rust の `AudioOptions` は `PeerConnectionFactory::create_audio_source` にしか渡せないため、3 setter は NetEq に影響しない
- `webrtc/src/webrtc_c/api/peer_connection_interface.h` の `webrtc_PeerConnectionInterface_RTCConfiguration` と `src/api/peer_connection.rs` の `PeerConnectionRtcConfiguration` には、3 設定の取得・設定口がない
- `src/tests.rs` の `audio_options_set_and_get_options` は `AudioOptions` の 8 フィールドの往復を確認しているが、NetEq への反映は確認していない
- `src/tests.rs` に `PeerConnectionRtcConfiguration` の 3 設定を確認するテストはない

## 設計方針

- Rust の `AudioOptions` と C API の `webrtc_AudioOptions_*` は変更しない
  - libwebrtc の `AudioOptions` を写す薄いラッパーであり、3 設定は libwebrtc で受信チャネルの構築時に使われており deprecated でもない
  - 将来 `AudioOptions` の jitter 設定を使う経路を追加する可能性があるため、公開 API を減らさない
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
  - `min_delay_ms` は base minimum delay であり、NetEq 生成時に 0〜10000 ms にクランプされる。目標遅延は `max_packets` とパケット長から決まる 75% で上限を切られる
  - PeerConnection 生成時に固定され、生成後の `set_configuration` では変更できない (`INVALID_MODIFICATION`)。`set_configuration` を呼ぶ場合は生成時と同じ jitter buffer 値を設定した `PeerConnectionRtcConfiguration` を渡す必要がある
- `CHANGES.md` の `## develop` に `[ADD]` を記載し、利用例を示す。`PeerConnectionRtcConfiguration` は `PeerConnection::create` に渡す前に設定する
  ```rust
  let mut config = PeerConnectionRtcConfiguration::new();
  config.set_audio_jitter_buffer_max_packets(50);
  let pc = PeerConnection::create(&factory, &config, deps)?;
  ```
- テスト
  - `audio_options_set_and_get_options` は変更しない (8 フィールドの往復のまま)
  - `rtc_configuration_audio_jitter_buffer_round_trip` を追加し、既定値と設定値の往復を確認する。通常の往復は `max_packets` 20 以上 (例: 123) を使う。20 未満の値は `RTCConfiguration` の生値として保持されること (getter が 10 を返すこと) を別途確認し、NetEq の実効値が 20 であることをコメントする
  - `PeerConnection::set_configuration` に生成時と同じ設定を渡すと成功し、jitter buffer の値だけを変えた設定を渡すとエラーになることを確認するテストを追加する。`RtcError` は種別を公開していないため、エラーになることまでを確認する
  - NetEq の実効値を直接確認する API はないため、テストは `RTCConfiguration` と Rust の往復、および `set_configuration` の拒否までを確認する
- 既定 prebuilt には新しい C API シンボルが無いため、検証は `--features source-build` を前提とする

## 完了条件

- Rust の `AudioOptions` と C API の `webrtc_AudioOptions_*` に jitter buffer の 3 getter / 3 setter が残っている
- `PeerConnectionRtcConfiguration` に 3 つの getter / setter があり、C API の関数が追加され、Rust の getter / setter から設定できる
- C API の getter は `const`、setter は非 `const` になっている
- doc に受信側のみ、既定値 200 / false / 0、`max_packets` の 20 未満クランプと getter の生値、`min_delay_ms` の意味、生成時固定、`set_configuration` の注意が書かれている
- `PeerConnection::set_configuration` に生成時と同じ設定を渡すと成功し、変更した設定を渡すとエラーになることを確認するテストがある
- `cargo test --workspace --features source-build`、`cargo clippy --workspace --features source-build --all-targets -- -D warnings`、`cargo fmt --all -- --check`、`python3 webrtc/run.py format --check` が成功する
- `src/tests.rs` に `PeerConnectionRtcConfiguration` の jitter buffer のテストが追加され、既定値・通常値の往復・20 未満の生値保持・`set_configuration` の拒否を確認している。`audio_options_set_and_get_options` は 8 フィールドのまま
- `CHANGES.md` に `[ADD]` が記載され、利用例が示されている

## 解決方法

未着手
