# AudioOptions による音声処理設定の API を追加する

- Created: 2026-08-25
- Completed: {YYYY-MM-DD}
- Branch: feature/add-audio-options
- Polished: {YYYY-MM-DD}

## 目的

音声トラックのノイズ系処理（エコーキャンセラ、自動ゲインコントロール、ノイズサプレッサ、ハイパスフィルタ）を webrtc-rs のユーザーが制御できるようにする。

libwebrtc は WebRtcVoiceEngine がこれらの処理をデフォルトで有効化している。そのため、マイク音声をそのまま取り込みたい録音用途や、ライブ配信で RTP へのヘッドルームを確保したい用途など、音声処理を無効化したいアプリケーションは webrtc-rs では手段がなく、諦めるかフォークするしか現状選択肢がない。

一般的なサービスの Sora 相当（`--disable-echo-cancellation` / `--disable-auto-gain-control` / `--disable-noise-suppression` / `--disable-highpass-filter`）の制御を実現するための最小 API を提供する。

## 現状

- libwebrtc の `WebRtcVoiceEngine::CreateDefaultAudioOptions()`（`media/engine/webrtc_voice_engine.cc`）が次のデフォルト値を設定する
  - `echo_cancellation = true`
  - `auto_gain_control = true`
  - `noise_suppression = true`（iOS は VPIO 内蔵のため `false`）
  - `highpass_filter = true`
- webrtc-rs は `cricket::AudioOptions` を C API として公開しておらず、`webrtc_PeerConnectionFactoryInterface_CreateAudioSource`（`webrtc/src/webrtc_c/api/peer_connection_interface.cc` の `CreateAudioSource`）が空の `webrtc::AudioOptions` を渡している
- Rust API は `src/api/peer_connection.rs` の `create_audio_source`（引数なしの `Result<AudioTrackSource>`）のみ

## 設計方針

- 変更対象は音声ソースの生成時に AudioOptions を渡せれば十分で、APM（`AudioProcessing::Config`）を直接触らない
  - `WebRtcVoiceEngine::ApplyOptions` が AudioOptions を APM 設定へ反映するため、音声トラックのオプション経由で制御するのが libwebrtc の意図した入口になる
- webrtc_c に `webrtc::AudioOptions` のラッパー `webrtc_AudioOptions` を追加する
  - 実装は `webrtc/src/webrtc_c/api/audio/audio_options.h` / `audio_options.cc` を新規作成する
  - `webrtc::AudioOptions` が `std::optional<bool>` を持っているため、setter 形式で optional に入れる（`webrtc_PeerConnectionFactoryInterface_Options` 系と同じ new / delete / set_xxx 形式）
  - setter は次の 4 つを提供する
    - `webrtc_AudioOptions_set_echo_cancellation`
    - `webrtc_AudioOptions_set_auto_gain_control`
    - `webrtc_AudioOptions_set_noise_suppression`
    - `webrtc_AudioOptions_set_highpass_filter`
  - setter を呼ばないフィールドは optional が空のままにし、WebRtcVoiceEngine のデフォルトが適用される状態を維持する
- `webrtc_PeerConnectionFactoryInterface_CreateAudioSource` のシグネチャに `struct webrtc_AudioOptions* options` を追加する
  - bindgen で Rust 側の FFI は自動生成される（`webrtc/src/webrtc_c.h` からの bindgen）
- `webrtc/CMakeLists.txt` の `webrtc_c` ライブラリのソースリストに `audio_options.cc` を追加し、`webrtc_c.h` にインクルードを追加する
- Rust API
  - `src/api/audio.rs` に `AudioOptions` を追加（`webrtc_AudioOptions` のラッパー。Drop で delete、setter は各オプションの有効/無効を設定）
  - `src/api/peer_connection.rs` の `create_audio_source` に `options: &AudioOptions` 引数を追加する
  - 既存の呼び出し元（`src/tests.rs` の `create_audio_source`）は未設定の `AudioOptions` を渡す形に更新する

## 完了条件

- `AudioOptions` の setter で echo cancellation / auto gain control / noise suppression / highpass filter を設定できる
- 何も設定しない場合、従来どおり WebRtcVoiceEngine のデフォルト（各処理が有効）の挙動を維持する
- `create_audio_source` が引数なしの既存ユーザからはシグネチャ変更が見えるが、挙動は互換である

## 解決方法

- 上記の設計方針に従い、webrtc_c の C API と Rust API を追加する
- `src/tests.rs` に次のテストを追加する
  - `AudioOptions` を生成して各 setter を呼び、drop できること
  - `create_audio_source` に設定付き `AudioOptions` を渡して生成できること
  - 既存の引数なし `create_audio_source` と同じ文脈で、未設定 `AudioOptions` を渡して従来と同じように生成できること
