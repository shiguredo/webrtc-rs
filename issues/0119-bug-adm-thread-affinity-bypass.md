# AudioDeviceModule のスレッドアフィニティが safe なコードだけで破られる

- Created: 2026-10-15
- Completed: {YYYY-MM-DD}
- Branch: feature/fix-adm-thread-affinity
- Polished: {YYYY-MM-DD}

## 目的

`AudioDeviceModule` (以下 ADM) を `PeerConnectionFactoryDependencies` に設定して `PeerConnectionFactory` から使う場合の排他契約を、 safe な Rust コードだけで破れないようにする。

`issues/0107-bug-safe-api-data-race.md` の「(7) 共有 ADM の直接操作と native 呼び出し」を本 issue に切り出した。 0107 は他のハンドラ、 共有映像バッファ、 `LoggingConfig` の初期化を扱う。

## 前提条件

`issues/0102-remove-worker-thread.md` の完了を前提とする。 0102 は `PeerConnectionFactoryDependencies::worker_thread` と `PeerConnectionFactoryInterface::worker_thread()` を削除し、 factory が使う thread を network thread に一本化する。 本 issue は worker thread を扱わず、 network thread のみを対象とする。

0102 は webrtc-build が削除系 CL を含むバージョンをリリースするまで着手できない。

## 現状

`AudioDeviceModule` (`src/api/audio_device_module.rs`) は `Clone` を持つが `Send` / `Sync` を持たない。 一方、 safe な `PeerConnectionFactoryDependencies::set_audio_device_module` (`src/api/peer_connection.rs`) は同じ ADM を `scoped_refptr` で保存し、 dependencies 自体は `Send` である。 元の ADM を生成スレッドに残しても、 dependencies を渡した先の native worker は同じ実体を呼べる。 同じ ADM を 2 個の dependencies に設定し、 異なる factory から使うことも safe API では禁止されていない。

`pc/connection_context.cc` の `ConnectionContext::AddRefMediaEngine` は worker thread 上で `media_engine_->Init()` を呼ぶ。 `media/engine/webrtc_voice_engine.cc` の `WebRtcVoiceEngine::Init` は `adm_helpers::Init(adm())` と `RegisterAudioCallback` を呼び、 `media/engine/adm_helpers.cc` の `Init` は `adm->Init()` などを呼ぶ。 これらと元のハンドルの safe な `init` / `recording_devices` / `recording_device_name` / `set_recording_device` が重なると、 同じ `adm_state` が排他参照を重複生成する。 `AudioDeviceModuleHandler` の `Send` と `&mut self` は、 1 個の native 利用者の直列呼び出しだけを根拠にしている。

未登録のカスタム ADM でも、 handler が同じ実体の別ハンドルへ再入すれば排他参照は重なる。 例えば `recording_devices(&self)` の callback が thread-local に保持した同じ ADM の `recording_devices` を再度呼ぶ場合、 Rust の共有借用は通るが、 両 trampoline は `&mut AudioDeviceModuleHandlerState` を作る。 `&mut self` を取る直接操作も、 `Clone` で作った別ハンドル経由の再入を防げない。

`issues/closed/0104-change-adm-handler-bounds.md` が全メソッドを `&mut self` にしたため、 C++ で `const` な読み取り専用の問い合わせ (`Recording()` など) まで `&mut self` になっている。 これは `AudioDeviceModuleHandler` の直列呼び出しを根拠にした変更だが、 callback から `is_recording` のような問い合わせを呼ぶだけで `adm_state` の可変参照が重なる状態を作っている。

### 再現手順

`unsafe` を書かずに、 ADM を別スレッドから使える状態を作れる (実際にコンパイルして確認済み)。

```rust
let adm = AudioDeviceModule::new(&env, AudioDeviceModuleAudioLayer::Dummy)
    .expect("AudioDeviceModule の生成に失敗しました"); // 利用者スレッド
deps.set_audio_device_module(&adm);                   // deps が ADM の参照を保持する
network.blocking_call(move || {                       // deps は Send なので network thread へ移せる
    Some(PeerConnectionFactory::create_modular(deps).ok())
});
let _ = adm.recording_devices();                      // 元のスレッドからも ADM を触れる
```

`deps` は `Send` であり、 ADM の参照を保持したまま別スレッドへ移せる。 移した先の native は network thread から ADM を呼ぶ。 元のスレッドに残した `adm` からも同じ実体を呼べるため、 2 つのスレッドが同じ `adm_state` に到達する。

### 検出状況

デバッグビルドでは libwebrtc のスレッドチェックで検出されることがあるが、 チェックを持たない実装 (Dummy や旧 `AudioDeviceWindowsCore` など) では検出されない。 リリースビルドではデータ競合 (未定義動作) に到達する。

`issues/closed/0104-change-adm-handler-bounds.md` は native 利用中の直接操作を Rustdoc で禁止する方針を採っているが、 safe API からの到達を防いでいない。

## 設計方針

案 D を採用する。 `AudioDeviceModule` が利用する thread を保持し、 各メソッドの呼び出しをその thread 上で実行する。 ポインタ以外のメンバを持つことを許容する。

`issues/0102-remove-worker-thread.md` の完了を前提とする。 0102 で `PeerConnectionFactoryDependencies::worker_thread` が削除され、 factory が使う thread は network thread に一本化されるため、 本 issue では worker thread を扱わない。

### ADM が thread を保持して呼び出しを寄せる

- `AudioDeviceModule::new(thread, env, layer)` / `new_with_handler(thread, handler)` / `from_refcounted_ptr(thread, raw_ref)` が利用する thread を受け取り、 メンバに保持する
- `init` / `recording_devices` / `recording_device_name` / `set_recording_device` は保持した thread に `blocking_call` で投げる
  - libwebrtc の `Thread::BlockingCall` は、 呼び出し元がその thread 上ならその場で functor を実行し、 別の thread なら PostTask して待つ。 したがって呼び出し元の thread を問わず、 native 呼び出しは保持した thread 上に揃う
  - これにより Rust 側の直接操作と native 利用者が同じ thread 上で直列化され、 `adm_state` が排他参照を重複生成しなくなる
- `&self` を取るメソッドを複数 thread から呼んでも実行は 1 本の thread に直列化されるため、 `AudioDeviceModule` に `Send` / `Sync` を実装する
  - `Clone` は維持する。 複製したハンドルは同じ thread を保持するため、 どちらから呼んでも同じ thread に揃う

### thread の生存

- 保持する thread が ADM より長く生存することを `new` 系の `# Safety` に置く。 ここだけが `unsafe` になる
- `blocking_call` は停止中の thread では functor を実行せず `R::default()` を返すため、 契約違反時は native を呼ばずに既定値が返る

### 食い違いの検査

- ADM が保持する thread と、 deps に設定された network thread が一致することを `set_audio_device_module` で検査し、 食い違えば panic する
- deps と ADM が同時に見えるのはこの関数だけである。 `create_modular` は deps から ADM の生ポインタしか得られず、 Rust 側の保持 thread と対応付けられない。 ADM の各メソッドからは deps が見えない。 そのため検査はここに置く
- `set_audio_device_module` は設定するだけの関数であり、 worker thread の受け渡しや ADM 側への記録は行わない
- この検査は native 利用開始前に食い違いを検出する

### ハンドラの mutability

`AudioDeviceModuleHandler` の全メソッドが `&mut self` になっているのは、 `issues/closed/0104-change-adm-handler-bounds.md` が C++ 側の `const` を考慮せず一律に変更したためである。 読み取り専用の問い合わせ (`Recording()` など) まで `&mut self` になっている。

- `webrtc::AudioDeviceModule` (`api/audio/audio_device.h`) の宣言に合わせ、 `const` のメソッドは `&self`、 非 `const` のメソッドは `&mut self` にする
  - `const`: `ActiveAudioLayer` / `Initialized` / `PlayoutIsInitialized` / `RecordingIsInitialized` / `Playing` / `Recording` / `SpeakerIsInitialized` / `MicrophoneIsInitialized` / `SpeakerVolume` / `MaxSpeakerVolume` / `MinSpeakerVolume` / `MicrophoneVolume` / `MaxMicrophoneVolume` / `MinMicrophoneVolume` / `SpeakerMute` / `MicrophoneMute` / `StereoPlayoutIsAvailable` / `StereoPlayout` / `StereoRecordingIsAvailable` / `StereoRecording` / `PlayoutDelay` / `BuiltInAECIsAvailable` / `BuiltInAGCIsAvailable` / `BuiltInNSIsAvailable` / `GetPlayoutUnderrunCount` / `GetStats` / `GetPlayoutAudioParameters` / `GetRecordAudioParameters`
  - 非 `const`: `Init` / `Terminate` / `RegisterAudioCallback` / `PlayoutDevices` / `RecordingDevices` / `PlayoutDeviceName` / `RecordingDeviceName` / `SetPlayoutDevice` / `SetRecordingDevice` / `InitPlayout` / `StartPlayout` / `StopPlayout` / `InitRecording` / `StartRecording` / `StopRecording` / `InitSpeaker` / `InitMicrophone` / `SetSpeakerVolume` / `SetMicrophoneVolume` / `SetSpeakerMute` / `SetMicrophoneMute` / `SetStereoPlayout` / `SetStereoRecording` / `EnableBuiltInAEC` / `EnableBuiltInAGC` / `EnableBuiltInNS`
- `adm_state` を trampoline ごとに `&` / `&mut` で使い分ける。 `&self` メソッドは共有参照で呼ぶため、 callback から `is_recording` のような問い合わせを再入しても可変参照が重ならない
- `AudioDeviceModuleHandler` は `Send` のみを要求し、 `Sync` は要求しない。 保持した thread が直列化するため、 ハンドラ自身に排他や直列化の責任を負わせる必要がない

## 完了条件

- ADM が利用する thread を保持し、 4 個の直接操作がその thread 上で実行されること
- ADM が保持する thread と deps の network thread の食い違いが `set_audio_device_module` で検出されること
- thread の生存要求が `new` 系の `# Safety` として示されていること
- `AudioDeviceModuleHandler` のメソッドの `&self` / `&mut self` が C++ の `const` と一致していること
- `AudioDeviceModuleHandler` が `Send` のみを要求し、 `Sync` を要求していないこと
- `AudioDeviceModuleHandler` の直列呼び出しを実物の C ラッパーで検証すること。 モックやスタブ、 未修正の UB を実行するテストは使わないこと
- 公開 API を変更する場合は `CHANGES.md` の `## develop` に反映されていること
- `cargo fmt --all -- --check` / `cargo clippy --workspace --features source-build --all-targets -- -D warnings` / `cargo test --workspace --features source-build` が通ること

## 解決方法

### C API

- `webrtc/src/webrtc_c/api/peer_connection_interface.h` / `.cc` に `webrtc_PeerConnectionFactoryDependencies_get_network_thread` を追加する。 `deps->network_thread` を返すだけ
- `webrtc/src/webrtc_c/rtc_base/thread.h` / `.cc` に `webrtc_Thread_IsCurrent` を追加する。 `Thread::IsCurrent()` を返すだけ。 テストで、 呼び出しが保持した thread 上で実行されたことを確認するために使う

### Rust

- `AudioDeviceModule` に利用する thread のメンバを追加し、 `new` / `new_with_handler` / `from_refcounted_ptr` が `&Thread` を受け取るようにする。 `new` 系を `unsafe fn` にし、 `# Safety` に thread の生存要求を書く
- `init` / `recording_devices` / `recording_device_name` / `set_recording_device` を保持した thread への `blocking_call` 経由にする。 クロージャは `Send` を要求するため、 `!Send` な生ポインタを運ぶ薄いラッパーを `src/helper/` に用意する
- `Thread` に、 生ポインタ経由で `blocking_call` する crate 内部専用のヘルパーを追加する。 `Thread` を借用として保持しない型から呼ぶために使う
- `AudioDeviceModule` に `Send` / `Sync` を実装する。 `Clone` は thread も複製する
- `set_audio_device_module` で deps の network thread と ADM の保持 thread を比較し、 食い違えば panic する
- `AudioDeviceModuleHandler` のメソッドを C++ の `const` に合わせて `&self` / `&mut self` に分け、 `adm_state` を trampoline ごとに `&` / `&mut` で使い分ける

### テスト

- 実物の C ラッパーで、 別 thread から呼び出した ADM の操作が保持した thread 上で実行されることを `webrtc_Thread_IsCurrent` で確認する
- `AudioDeviceModuleHandler` の `Sync` 非要求と、 `&self` メソッドが再入しても可変参照が重ならないことを確認する
- `set_audio_device_module` の食い違い検査を確認する

## 検討したが採らなかった案

- A (0107 の案): `set_audio_device_module` を `unsafe fn` にし、 同時・再入が無いことを `# Safety` で要求する。 safe API からの到達自体は防げない
- B (0107 の案): 4 個の直接操作を `unsafe fn` にする。 callback からの再入を型で防げず、 利用者に毎回の `unsafe` を強いる
- C: `AudioDeviceModule` に `Send` だけを実装し、 契約をドキュメントで担保する。 同時呼び出しを型でも実行時検査でも防げない
- `create_modular` での食い違い検査: deps からは ADM の生ポインタしか得られず、 Rust 側の保持 thread と対応付けられないため不可
- trampoline での食い違い検査: native 実装の ADM では trampoline が呼ばれないため、 カスタム ADM の事後検出しかできない
- worker thread を deps から読む C API: 0102 で `worker_thread` が削除されるため不要
