# AudioDeviceModuleHandler の Send + Sync 要求を見直す

- Created: 2026-09-17
- Completed: 2026-09-18
- Branch: feature/change-adm-handler-bounds
- Polished: {YYYY-MM-DD}

## 目的

`AudioDeviceModuleHandler` は `Send + Sync` を要求しているが、libwebrtc の ADM 実装は公開メソッドを単一スレッドで呼ぶ前提で書かれており、`Sync` は過剰な要求の可能性がある。

`Sync` を要求すると、ハンドラが排他的なハンドル（例: `AudioTransportRefMut`）を保持したまま `&mut self` のメソッドを呼ぶために `Mutex` などの内部可変性が必須になる。libwebrtc の実際のスレッド契約を確認したうえで、bound とトランポリンの方式を適切に設定し直す。

## 現状

- `src/api/audio_device_module.rs` の `AudioDeviceModuleHandler` は `Send + Sync` を要求し、すべてのメソッドが `&self` を取る
- ADM のトランポリン（`adm_state`）は `user_data` から `&'static AudioDeviceModuleHandlerState` を作り、`&self` メソッドを呼ぶ
  - 同時に呼ばれても安全にするため、ハンドラに `Sync` を要求する設計になっている
- 一方 `AudioTransportHandler` は `Send` のみを要求し、トランポリンは `user_data` から `&mut` を作って `&mut self` メソッドを呼ぶ
- libwebrtc 側の根拠
  - `modules/audio_device/linux/audio_device_pulse_linux.h` の `AudioDeviceLinuxPulse` は「Stores thread ID in constructor. We can then use RTC_DCHECK_RUN_ON(...) to ensure that other methods are called from the same thread.」とコメントし、公開メソッドの先頭で `RTC_DCHECK(thread_checker_.IsCurrent())` を取る（`Detach` はしない）
  - `sdk/android/src/jni/audio_device/audio_device_module.cc` の Android ADM はコンストラクタと `Terminate()` の後に `thread_checker_.Detach()` するため、Init / Terminate のサイクル間でスレッドが変わり得るが、同時アクセスは想定していない
  - 音声スレッドが呼ぶのは `AudioDeviceBuffer` 経由の `AudioTransport` であり、ADM の仮想メソッドではない
- `AudioDeviceModule` 自体は生成と削除を同じスレッドで行う必要がある（一部プラットフォームの制約）ため `Send` / `Sync` を実装していない

## libwebrtc の ADM スレッド契約

`WEBRTC_BUILD_ROOT` で使う libwebrtc のソース
(`../webrtc-build/_source/ubuntu-24.04_x86_64/webrtc/src`、上流コミット
`c2b761bb73f0b2ced096274abb415f6c7559a28b` に shiguredo パッチを適用したもの) で確認した結果を記録する。

### 外側と内側の区別

`webrtc::AudioDeviceModule` (`api/audio/audio_device.h`) を実装する型には外側と内側がある。

- 外側は `modules/audio_device/audio_device_impl.h` の `AudioDeviceModuleImpl` で、内側の `AudioDeviceGeneric` を保持して公開メソッドをそのまま委譲する
- 内側は `modules/audio_device/audio_device_generic.h` の `AudioDeviceGeneric` を実装するプラットフォーム実装 (`AudioDeviceLinuxPulse` / `AudioDeviceIOS` / `AudioDeviceMac` / `AudioDeviceWindowsCore` など) で、`webrtc::AudioDeviceModule` の実装ではない
- このリポジトリの `webrtc/src/webrtc_c/api/audio/audio_device.cc` の `AudioDeviceModuleImpl` は外側を直接実装してコールバックへ委譲する。Rust ハンドラはこのコールバックとして呼ばれるため、ハンドラが実装するのは外側の `webrtc::AudioDeviceModule` であり、内側の `AudioDeviceGeneric` は介在しない

### 外側 (ハンドラが実装するインターフェース) の呼び出し元

- `api/audio/audio_device.h` の `webrtc::AudioDeviceModule` にはスレッド契約のコメントが無い
- 外側の `AudioDeviceModuleImpl` はロックもスレッドチェッカーも持たず、`AudioDeviceGeneric` へそのまま委譲する
- `audio/audio_state.cc` の `AudioState` は `SetPlayout` / `SetRecording` / `AddSendingStream` / `RemoveSendingStream` / `AddReceivingStream` / `RemoveReceivingStream` のいずれも `RTC_DCHECK_RUN_ON(&worker_thread_checker_)` を取ってから ADM を呼ぶ
- `media/engine/webrtc_voice_engine.cc` は `WebRtcVoiceEngine::adm()` で `RTC_DCHECK_RUN_ON(&worker_thread_checker_)` を取り、ADM の呼び出しはすべてこのアクセサ経由になる。`WebRtcVoiceEngine::Init()` から呼ぶ `media/engine/adm_helpers.cc` の `adm_helpers::Init` も worker thread 上で動く
- `audio/channel_receive.cc` の `ChannelReceive::UpdatePlayoutTimestamp` は `RTC_DCHECK_RUN_ON(&worker_thread_checker_)` を取ってから `PlayoutDelay` を呼ぶ
- `pc/` 層 (`ConnectionContext::AddRefMediaEngine` / `PeerConnection::SetAudioPlayout` など) も worker thread から呼ぶ。`RtcStatsCollector` は worker thread に Post したタスクの中で `GetAudioDeviceStats` を取る
- `audio/voip/voip_core.cc` の `VoipCore` は VoIP の呼び出し元スレッドから ADM を呼ぶ。このリポジトリの C API は VoIP を公開していない
- ADM を生成したスレッドと libwebrtc が ADM を呼ぶスレッドは同じとは限らない。`PeerConnection` の生成時には `worker_thread()->BlockingCall` の中で `ConnectionContext::MediaEngineReference` が作られ、`AddRefMediaEngine` → `MediaEngine::Init()` → `ADM::Init()` が worker thread 上で走る (`pc/peer_connection.cc` と `pc/connection_context.cc`)。このため `Send` は必要になる

### 内側 (ハンドラには届かない) の並行呼び出し

プラットフォーム実装は内部スレッドから自分自身 (内側) のメソッドを呼ぶことがある。外側のインターフェースもコールバックも経由しないため、Rust ハンドラには届かない。

- `modules/audio_device/linux/audio_device_pulse_linux.cc` の `PlayThreadProcess` / `RecThreadProcess` は自前スレッドから `PlayoutDevices()` / `RecordingDevices()` を呼ぶ
- `sdk/objc/native/src/audio/audio_device_ios.mm` の `OnGetPlayoutData` はオーディオ I/O スレッドから `PlayoutDelay()` を呼ぶ
- `modules/audio_device/mac/audio_device_mac.cc` の `HandleDeviceChange` は HAL の通知スレッドから `MicrophoneIsInitialized()` / `SpeakerIsInitialized()` を呼ぶ
- Android の `AndroidAudioDeviceModule` は外側を直接実装するが、Java の録音 / 再生スレッドは `AudioRecordJni` / `AudioTrackJni` から `AudioDeviceBuffer` (AudioTransport) を呼ぶだけで、ADM の公開メソッドは呼ばない
- 音声データの受け渡しは `AudioDeviceBuffer` が保持する `AudioTransport` 経由であり、`AudioTransportImpl` は ADM を参照しない

### プラットフォーム実装が前提にしているスレッド

- Linux PulseAudio (`modules/audio_device/linux/audio_device_pulse_linux.h`) は `SequenceChecker thread_checker_` を持ち、「Stores thread ID in constructor. We can then use RTC_DCHECK_RUN_ON(...) to ensure that other methods are called from the same thread.」とコメントしたうえで公開メソッドの先頭で `RTC_DCHECK(thread_checker_.IsCurrent())` を取る。`Detach()` はしない
- Windows の既定経路は `modules/audio_device/win/audio_device_core_win.h` の `AudioDeviceWindowsCore` で、スレッドチェッカーは無く内部に `Mutex` を持つ。`audio_device_module_win.cc` の `WindowsAudioDeviceModule` は `is_win && !build_with_chromium` のときだけビルドされ、単一スレッド契約をコメントと `RTC_DCHECK_RUN_ON(&thread_checker_)` で明示している
- Android (`sdk/android/src/jni/audio_device/audio_device_module.cc`) の `AndroidAudioDeviceModule` はコンストラクタと `Terminate()` の後に `thread_checker_.Detach()` し、`Init()` / `Terminate()` で `IsCurrent()` を取る。Init / Terminate のサイクルをまたぐとスレッドが変わり得る
- iOS (`sdk/objc/native/src/audio/audio_device_ios.{h,mm}`) の `AudioDeviceIOS` は `RTC_DCHECK_RUN_ON(thread_)` を取り、オーディオ I/O 側は `io_thread_checker_` で別に保護する
- Linux ALSA (`modules/audio_device/linux/audio_device_alsa_linux.cc`) と macOS (`modules/audio_device/mac/audio_device_mac.cc`) にはスレッドチェッカーが無く、macOS とファイル ADM (`modules/audio_device/dummy/file_audio_device.h`) は内部に `Mutex` を持つ
- `RTC_DCHECK_RUN_ON` は `RTC_DCHECK` なので release ビルドでは何も検査しない。`SequenceChecker` も既定では初回チェック時にスレッドへ遅延して束縛される

### 利用側が ADM を直接呼ぶ場合の制約

`AudioDeviceModule` は refcounted なハンドルなので、`PeerConnectionFactoryDependencies` に渡した後も利用側にハンドルが残る。そのハンドルからメソッドを呼ぶと、libwebrtc が worker thread で同じ ADM を呼んでいるのと同時になり得る。

この「単一スレッドで使う」制約はこのリポジトリ固有のものではなく、libwebrtc の ADM 実装自身が持っている。

- `modules/audio_device/win/audio_device_module_win.cc` のクラスコメントは「An instance must be created, destroyed and used on one and the same thread, i.e., all public methods must also be called on the same thread.」と明記する
- `sdk/objc/native/src/audio/audio_device_ios.h` も「All supported public methods must also be called on the same thread.」と明記する
- `sdk/android/src/jni/audio_device/audio_device_module.cc` は「must then be used on one and the same thread」としつつ、Init / Terminate のサイクルをまたぐ場合だけ `thread_checker_.Detach()` を許す
- `modules/audio_device/audio_device_impl.h` の `create_detached` は「生成したスレッドとは別のスレッドで使えるようにする（テスト用）」ためのオプションで、既定は生成したスレッドでの利用になる

型で防ぐことはできない。`AudioDeviceModule` の `Clone` は refcounted ハンドルとして必要であり、ハンドオフの前後を型で区別しても、ハンドオフ前に作ったハンドルが同じオブジェクトを指し続ける。libwebrtc 側がどのスレッドから呼ぶかは Rust の型からは見えないため、同時呼び出しを型で禁止する方法は無い。

したがって、この制約への対応は C++ の ADM と同じくドキュメントで示す。排他が不要なオブジェクトに `Mutex` を追加するのは過剰であり、生ポインタを引数に取る `AudioTransport` の直接呼び出しと違って `unsafe` にする理由も無い。ハンドラの doc コメントに禁止事項として記載する。

### 結論

ハンドラが実装する外側の `webrtc::AudioDeviceModule` について、同時に 2 つの公開メソッドが別スレッドで動く経路は確認できない。設計方針の条件分岐は「同時呼び出しが確認できない」側に確定し、`Sync` を維持する必要は無い。

`Send` は必要になる。ADM を生成したスレッドと libwebrtc が ADM を呼ぶスレッドは異なり得るため (`PeerConnection` の生成時は worker thread から `ADM::Init()` が呼ばれる)、`!Send` なハンドラを受理すると値が別スレッドへ渡ってしまう。

`AudioDeviceModule` のメソッドを利用側が直接呼ぶ場合は別で、`PeerConnectionFactory` に ADM を渡した後に別スレッドから呼ぶと libwebrtc の呼び出しと同時になり得る。この場合は `&mut self` の排他性が破れるため、ハンドラのドキュメントで「ADM を渡した後は同じ ADM を別スレッドから直接呼ばないこと」を条件として明記する。

## 設計方針

- libwebrtc の契約に合わせ、`AudioDeviceModuleHandler` の要求を `Send` のみにする
- 併せて ADM のトランポリンを `&mut` ベース（`AudioTransport` と同じ方式）に変更する
  - ハンドラ側は `Mutex` のような内部可変性なしで排他的なハンドルを保持できるようになる
  - `Sync` を外すことで `&self` 経由の共有アクセスが型で禁止され、同時呼び出しが起きる使い方はコンパイルエラーになる
- ただし libwebrtc は ADM の同時呼び出しを明文化していないため、実装前に `AudioDeviceModule` の全仮想メソッドの呼び出し元スレッドを確認する
  - 同時呼び出しが確認できた場合は `Sync` を維持し、`AudioTransportRefMut` 側の扱い（`Mutex` を強制する現状の設計）を再検討する

## 完了条件

- libwebrtc の ADM のスレッド契約が issue に記録されている
- `AudioDeviceModuleHandler` の bound と ADM トランポリンの方式が、確認した契約と一致している
- `cargo clippy --workspace --features source-build -- -D warnings` と `cargo test --workspace --features source-build` が通る

## 解決方法

- `AudioDeviceModuleHandler` の要求を `Send + Sync` から `Send` に変更し、全 62 メソッドの `&self` を `&mut self` にした
- ADM の trampoline を `&mut` ベースに変更した。`adm_state` が `user_data` から `&mut AudioDeviceModuleHandlerState` を返し、各 trampoline がハンドラの `&mut self` メソッドを呼ぶ
- `src/tests.rs` に `audio_device_module_handler_requires_only_send` を追加した。`Cell<i32>` を持つ `!Sync` なハンドラを実装できることで `Sync` が要求されていないことを型で確認し、呼び出しごとに `&mut self` で状態が保持されることを確認する
- 利用側が `PeerConnectionFactory` に渡した後に同じ ADM を別スレッドから呼ぶと libwebrtc の呼び出しと同時になり得る点を、ハンドラの doc コメントに禁止事項として記載した。これは libwebrtc の ADM が持つ単一スレッド契約と同じ制約で型では防げないため、`Mutex` による排他や `unsafe` 化はしない
- `CHANGES.md` の `## develop` に `[CHANGE]` エントリを追加した
- `cargo fmt --all -- --check`、`cargo clippy --workspace --features source-build -- -D warnings`、`cargo test --workspace --features source-build` の成功を確認した
