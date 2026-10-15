# AudioDeviceModule のスレッドアフィニティが safe なコードだけで破られる

- Created: 2026-10-15
- Completed: {YYYY-MM-DD}
- Branch: feature/fix-adm-thread-affinity
- Polished: {YYYY-MM-DD}

## 目的

`AudioDeviceModule` (以下 ADM) を `PeerConnectionFactoryDependencies` に設定して `PeerConnectionFactory` から使う場合の排他契約を、 safe な Rust コードだけで破れないようにする。

`issues/0107-bug-safe-api-data-race.md` の「(7) 共有 ADM の直接操作と native 呼び出し」を本 issue に切り出した。 0107 は他のハンドラ、 共有映像バッファ、 `LoggingConfig` の初期化を扱う。

## 現状

`AudioDeviceModule` (`src/api/audio_device_module.rs`) は `Clone` を持つが `Send` / `Sync` を持たない。 一方、 safe な `PeerConnectionFactoryDependencies::set_audio_device_module` (`src/api/peer_connection.rs`) は同じ ADM を `scoped_refptr` で保存し、 dependencies 自体は `Send` である。 元の ADM を生成スレッドに残しても、 dependencies を渡した先の native worker は同じ実体を呼べる。 同じ ADM を 2 個の dependencies に設定し、 異なる factory から使うことも safe API では禁止されていない。

`pc/connection_context.cc` の `ConnectionContext::AddRefMediaEngine` は worker thread 上で `media_engine_->Init()` を呼ぶ。 `media/engine/webrtc_voice_engine.cc` の `WebRtcVoiceEngine::Init` は `adm_helpers::Init(adm())` と `RegisterAudioCallback` を呼び、 `media/engine/adm_helpers.cc` の `Init` は `adm->Init()` などを呼ぶ。 これらと元のハンドルの safe な `init` / `recording_devices` / `recording_device_name` / `set_recording_device` が重なると、 同じ `adm_state` が排他参照を重複生成する。 `AudioDeviceModuleHandler` の `Send` と `&mut self` は、 1 個の native 利用者の直列呼び出しだけを根拠にしている。

未登録のカスタム ADM でも、 handler が同じ実体の別ハンドルへ再入すれば排他参照は重なる。 例えば `recording_devices(&self)` の callback が thread-local に保持した同じ ADM の `recording_devices` を再度呼ぶ場合、 Rust の共有借用は通るが、 両 trampoline は `&mut AudioDeviceModuleHandlerState` を作る。 `&mut self` を取る直接操作も、 `Clone` で作った別ハンドル経由の再入を防げない。

### 再現手順

`unsafe` を書かずに、 ADM を別スレッドから使える状態を作れる (実際にコンパイルして確認済み)。

```rust
let adm = AudioDeviceModule::new(&env, AudioDeviceModuleAudioLayer::Dummy)
    .expect("AudioDeviceModule の生成に失敗しました"); // 利用者スレッド
deps.set_audio_device_module(&adm);                   // deps が ADM の参照を保持する
network.blocking_call(move || {                       // deps は Send なので worker thread へ移せる
    Some(PeerConnectionFactory::create_modular(deps).ok())
});
let _ = adm.recording_devices();                      // 元のスレッドからも ADM を触れる
```

### 検出状況

デバッグビルドでは libwebrtc のスレッドチェックで検出されることがあるが、 チェックを持たない実装 (Dummy や旧 `AudioDeviceWindowsCore` など) では検出されない。 リリースビルドではデータ競合 (未定義動作) に到達する。

`issues/closed/0104-change-adm-handler-bounds.md` は native 利用中の直接操作を Rustdoc で禁止する方針を採っているが、 safe API からの到達を防いでいない。

## 設計方針

未確定。 0107 から引き継いだ案と、 検討中の案を並べる。

- A (0107 の案): `PeerConnectionFactoryDependencies::set_audio_device_module` を `unsafe fn` にする。 `# Safety` には、 dependencies への設定時から native 利用の終了まで、 同じ ADM の callback が他の factory、 元のハンドル、 clone からの操作と同時または再入で重ならないことを要求する。 異なる worker thread の factory 間で共有しないことも明記する
- B (0107 の案): `AudioDeviceModule::init` / `recording_devices` / `recording_device_name` / `set_recording_device` を `unsafe fn` にする。 呼び出し中は同じ実体への他の直接操作、 native 呼び出し、 callback からの再入が無いことを `# Safety` に要求する。 getter も可変 handler を呼ぶため対象に含める
- C: `AudioDeviceModule` に `Send` を実装し、 契約はドキュメントで担保する (`Thread` / `DtlsTransport` / `DataChannel` と同じ扱い)。 `Thread::blocking_call` で worker thread へ移して呼び出せるようになる。 変更は `unsafe impl Send` の追加のみ
- D: `AudioDeviceModule` は `Send` にせず、 生成を `AudioDeviceModule::new(worker, env, layer)` に限定し、 呼び出しを `adm.on_worker(&mut worker).<method>()` のような marshalling API に限定する (生メソッドは `pub(crate)` にする)。 契約を型で強制できるが、 破壊的変更と API 追加が大きい

検討時の制約:

- Rust 側の型は C++ オブジェクトへのポインタ (`ScopedRef`) 以外の状態を持たせない。 そのため D で「生成時の worker thread を ADM に記録して `on_worker` で検査する」ことはできない
- ADM のメソッドは `&self` を取るため、 単に `Send` / `Sync` を実装すると複数スレッドからの同時呼び出しを型では防げなくなる
- `AudioDeviceModuleHandler` の `Send` と `&mut self` を維持するか、 ADM の `Clone` を維持するかは案ごとに決める

## 完了条件

- ADM の factory への引き渡しと 4 個の直接操作について、 native 利用者、 他のハンドル、 callback からの同時・再入を排除する契約が、 型または `# Safety` として示されていること
- safe なコードだけでは ADM を複数スレッドから使えないこと
- `AudioDeviceModuleHandler` の直列呼び出しを実物の C ラッパーで検証すること。 モックやスタブ、 未修正の UB を実行するテストは使わないこと
- 公開 API を変更する場合は `CHANGES.md` の `## develop` に反映されていること
- `cargo fmt --all -- --check` / `cargo clippy --workspace --features source-build --all-targets -- -D warnings` / `cargo test --workspace --features source-build` が通ること

## 解決方法

未確定。
