# 外部で作成した AudioDeviceModule を Rust 側で取り込めるようにする

- Created: 2026-10-06
- Completed: {YYYY-MM-DD}
- Branch: feature/add-adopt-external-audio-device-module
- Polished: {YYYY-MM-DD}

## 目的

Java 側で作成した `JavaAudioDeviceModule` の native ADM を、`shiguredo_webrtc::AudioDeviceModule` として扱えるようにする。

Sora Kotlin SDK は Android の音声入出力に Java の `JavaAudioDeviceModule` を使い、`audioSource` / ステレオ入出力 / ハードウェア AEC・NS / `AudioAttributes` を設定する。これらの設定は Java のビルダー API でしか指定できず、ネイティブ経路の ADM では代替できない。このため Java 側で作成した ADM を Rust 側へ渡す必要があるが、現在その取り込み手段が無く、Sora Kotlin SDK の Rust ブリッジはビルドできない。

## 前提条件

Sora Kotlin SDK の音声設定 (`SoraAudioConfiguration` の `audioSource` / `useStereoInput` / `useStereoOutput` / `useHardwareAcousticEchoCanceler` / `useHardwareNoiseSuppressor` / `audioAttributes`) を維持し、最大限活用する。このため Java 側で作成した `JavaAudioDeviceModule` を使い続ける。

ネイティブ経路の ADM ではこれらの設定を代替できない。

- `webrtc::CreateJavaAudioDeviceModule` は `GetDefaultAudioParameters` を `use_stereo_input=false` / `use_stereo_output=false` で呼び、`WebRtcAudioRecord` の `@CalledByNative` なコンストラクタを使う。このため audioSource は `DEFAULT_AUDIO_SOURCE` (VOICE_COMMUNICATION) 固定、ステレオは無効固定、ハードウェア AEC / NS は「対応端末で常時有効 (無効化できない)」になり、`AudioAttributes` は指定できない
- `CreateAndroidAudioDeviceModule` の `kPlatformDefaultAudio` は、AAudio 対応ビルドでは AAudio、そうでなければ端末の low-latency 対応状況で OpenSLES / Java 入力 + OpenSLES 出力 / Java を選ぶ。端末によって使われる実装が変わる
- Java 側の `pauseRecording` / `resumeRecording` (音声ハードミュート) と `AudioTrackSink` は Java の API であり、Java オブジェクトを保持し続ける必要がある

## 現状

- `AudioDeviceModule` の公開 API は `AudioDeviceModule::new` / `AudioDeviceModule::new_with_handler` と `Clone` だけで、外部から渡された ADM を取り込む手段が無い。`ScopedRef::from_raw` はクレート内専用である
- Android 向けの C API には `webrtc_CreateJavaAudioDeviceModule` (`webrtc/src/webrtc_c/sdk/android/native_api/audio_device_module/audio_device_android.h`) がある。これは `webrtc_AudioDeviceModule_refcounted*` を返し、実装で `adm.release()` しているため、戻り値の参照 1 つは呼び出し側の所有になる。bindgen の入力 `android.h` がこのヘッダーを include しているため、Android ターゲットでは `ffi` から呼べる
- Java の `JavaAudioDeviceModule.getNative(long)` は、Java 側がフィールドに保持して `release()` で解放する native ADM のポインタを返す。保持している参照は Java 側のもので、呼び出し側の所有ではない
- 取り込む対象によって所有権の扱いが正反対になる。所有権を受け取る API を Java 側の借用ポインタに使うと Java 側の `release()` で二重解放になり、参照を増やす API を `webrtc_CreateJavaAudioDeviceModule` の戻り値に使うと参照が 1 つリークする
- Sora Kotlin SDK の Rust ブリッジは、Java 側から渡された ADM と自身で作成した ADM の両方で未実装の `AudioDeviceModule::from_refcounted_ptr` を呼んでおり、どちらの意味に倒しても片方が壊れる
- `AudioDeviceModule` は生成と削除を同じスレッドで行う必要があるため `Send` / `Sync` を実装していない

## 設計方針

- Java 側で作成した `JavaAudioDeviceModule` を置き換えず、その native ADM を共有する。ネイティブ側で別の ADM を作って差し替えることはしない
- 生ポインタを受け取る API は `unsafe fn` とし、`# Safety` に契約を書く (`issues/0104-bug-safe-api-use-after-free.md` / `issues/0107-bug-safe-api-data-race.md` と同じ方針)
- 所有権の意味が異なる 2 つのコンストラクタを分ける
  - 借用ポインタを取り込むもの: 参照カウントを 1 増やしてから保持する (`ScopedRef::clone` と同じ扱い)。呼び出し側が持つ参照は消費しない。Java 側の `release()` や GC の影響を受けない
  - 所有権を受け取るもの: 参照カウントを増やさずに保持する。`webrtc_CreateJavaAudioDeviceModule` の戻り値のように、呼び出し側が所有する参照を渡す
  - メソッド名と引数の型 (`ffi` の refcounted 型か `*mut c_void` か) は、既存の公開 API の流儀に合わせて実装時に確定する
- 取り込み後も Java 側のオブジェクトを独立して使い続けられること。Java のビルダー項目を将来増やしても本 API の変更が不要であること
- `ScopedRef::from_raw` / `RTCStatsReport::from_refcounted_ptr` をクレート内専用にした判断 (`issues/closed/0078-bug-scoped-ref-from-raw-safety.md`) は「外部での構築ニーズが無い」ことが根拠である。ADM は JNI から渡されたポインタを取り込むニーズがあるため、汎用 API の再公開ではなく用途を限定した API を追加する
- 生成と削除を同じスレッドで行う契約は新しい API でも維持し、Rustdoc に明記する
- テストはモックを使わず、実際の ADM で参照カウントの増減を確認する
- 公開 API の追加なので `CHANGES.md` の `## develop` に `[ADD]` を記載する

## 完了条件

- Java 側が所有する ADM のポインタを取り込める。取り込んだ後に Java 側が `release()` を呼んでも Rust 側の `AudioDeviceModule` が使えること
- 所有権付きで渡された ADM を取り込める。参照カウントを余分に増やさないこと
- `src/tests.rs` に参照カウントの増減を確認するテストを追加する
  - 借用取り込みで作った 2 つ目のハンドルが、1 つ目のハンドルを drop した後も使えること
  - 所有権取り込みで参照がリークしないこと
  - 生成と削除を同じスレッドで行うこと
- Android 実機またはエミュレーターで、`SoraAudioConfiguration` の全項目 (audioSource / ステレオ入出力 / ハードウェア AEC・NS / AudioAttributes) を反映した ADM で接続でき、Java 側の pause / resume と AudioTrackSink が使えること (Sora Kotlin SDK 側の対応は別リポジトリ)
- Rustdoc に所有権とスレッドの契約が書かれていること
- `cargo fmt --all -- --check` / `cargo clippy --workspace --features source-build --all-targets -- -D warnings` / `cargo test --workspace --features source-build` が通ること

## 解決方法

未着手
