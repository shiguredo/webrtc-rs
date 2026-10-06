# 外部で作成した AudioDeviceModule を Rust 側で取り込めるようにする

- Created: 2026-10-06
- Completed: 2026-10-06
- Branch: feature/add-adopt-external-audio-device-module
- Polished: 2026-10-06

## 目的

Java 側で作成した `JavaAudioDeviceModule` の native ADM を、`shiguredo_webrtc::AudioDeviceModule` として扱えるようにする。

Sora Kotlin SDK は Android の音声入出力に Java の `JavaAudioDeviceModule` を使い、`audioSource` / ステレオ入出力 / ハードウェア AEC・NS / `AudioAttributes` を設定する。これらの設定は Java のビルダー API でしか指定できず、ネイティブ経路の ADM では代替できない。このため Java 側で作成した ADM を Rust 側へ渡す必要があるが、現在その取り込み手段が無く、Sora Kotlin SDK の Rust ブリッジはビルドできない。

## 前提条件

Sora Kotlin SDK の音声設定 (`SoraAudioConfiguration` の `audioSource` / `useStereoInput` / `useStereoOutput` / `useHardwareAcousticEchoCanceler` / `useHardwareNoiseSuppressor` / `audioAttributes`) を維持し、最大限活用する。このため Java 側で作成した `JavaAudioDeviceModule` を使い続ける。

ネイティブ経路の ADM ではこれらの設定を代替できない。

- `webrtc::CreateJavaAudioDeviceModule` は `GetDefaultAudioParameters` を `use_stereo_input=false` / `use_stereo_output=false` で呼び、`WebRtcAudioRecord` の `@CalledByNative` なコンストラクタを使う。このため audioSource は `DEFAULT_AUDIO_SOURCE` (VOICE_COMMUNICATION) 固定、ステレオは無効固定、ハードウェア AEC / NS は「対応端末で常時有効 (無効化できない)」になり、`AudioAttributes` は指定できない
- `CreateAndroidAudioDeviceModule` の `kPlatformDefaultAudio` は、AAudio 対応ビルドでは AAudio、そうでなければ端末の low-latency 対応状況で OpenSLES / Java 入力 + OpenSLES 出力 / Java を選ぶ。端末によって使われる実装が変わる。なお Rust の `AudioDeviceModule::new` が呼ぶのは `webrtc::CreateAudioDeviceModule` であり、`CreateAndroidAudioDeviceModule` は webrtc_c の C API には無い
- Java 側の `pauseRecording` / `resumeRecording` (音声ハードミュート) と `org.webrtc.AudioTrackSink` は Java の API であり、Java オブジェクトを保持し続ける必要がある

Sora Kotlin SDK 側の呼び分けと実機での接続確認は、本 issue の対応後に別リポジトリで行う。本 issue は取り込み API の追加までを対象とする。

## 現状

- `AudioDeviceModule` の公開されたコンストラクタは `AudioDeviceModule::new` / `AudioDeviceModule::new_with_handler` だけで、外部から渡された ADM を取り込む手段が無い。`ScopedRef::from_raw` はクレート内専用である
- Android 向けの C API には `webrtc_CreateJavaAudioDeviceModule` (`webrtc/src/webrtc_c/sdk/android/native_api/audio_device_module/audio_device_android.h`) がある。これは `webrtc_AudioDeviceModule_refcounted*` を返し、実装で `adm.release()` しているため、戻り値の参照 1 つは呼び出し側の所有になる。bindgen の入力 `android.h` がこのヘッダーを include しているため、Android ターゲットでは `ffi` から呼べる
- Java の `JavaAudioDeviceModule.getNative(long)` は、Java 側がフィールドに保持して `release()` で解放する native ADM のポインタを返す。保持している参照は Java 側のもので、呼び出し側の所有ではない
- 取り込む対象によって所有権の扱いが正反対になる。所有権を受け取る API を Java 側の借用ポインタに使うと Java 側の `release()` で二重解放になり、参照を増やす API を `webrtc_CreateJavaAudioDeviceModule` の戻り値に使うと参照が 1 つリークする
- Sora Kotlin SDK の Rust ブリッジは、Java 側から渡された ADM と自身で作成した ADM の両方で未実装の `AudioDeviceModule::from_refcounted_ptr` を呼んでおり、どちらの意味に倒しても片方が壊れる
- `AudioDeviceModule` は実装によって生成と削除を同じスレッドで行う必要がある (同一スレッドの制約があるのは iOS / Android などの一部のプラットフォーム) ため `Send` / `Sync` を実装していない

## 設計方針

- Java 側で作成した `JavaAudioDeviceModule` を置き換えず、その native ADM を共有する。ネイティブ側で別の ADM を作って差し替えることはしない
- 生ポインタを受け取る API は `unsafe fn` とし、`# Safety` に契約を書く (`issues/0104-bug-safe-api-use-after-free.md` / `issues/0107-bug-safe-api-data-race.md` と同じ方針)
- 所有権の意味が異なる 2 つのコンストラクタを分ける
  - 借用ポインタを取り込むもの: 参照カウントを 1 増やしてから保持する (`ScopedRef::clone` と同じ扱い)。呼び出し側が持つ参照は消費しない。Java 側の `release()` や GC の影響を受けない
  - 所有権を受け取るもの: 参照カウントを増やさずに保持する。`webrtc_CreateJavaAudioDeviceModule` の戻り値のように、呼び出し側が所有する参照を渡す
  - 引数は `*mut ffi::webrtc_AudioDeviceModule_refcounted` か `*mut c_void` のどちらかにし、null は `Option<Self>` の `None` で表す。`getNative` は非所有の非 null ポインタを返す契約だが、JNI 境界では Java の `long` の 0 が null ポインタに対応し、生ポインタを受け取る API では null を型で排除できない。所有権を受け取る側も同じ形に揃える。生ポインタを受け取る公開コンストラクタの先例は無く (`ScopedRef::from_raw` 系は `issues/closed/0078-bug-scoped-ref-from-raw-safety.md` で `pub(crate)` 化した)、Sora Kotlin SDK の現行の呼び出しは JNI から得た `jlong` を `*mut c_void` にキャストして渡しているため、`*mut c_void` なら SDK 側の型変換が不要になる
  - `# Safety` には、借用側は「ポインタが有効な ADM を指し、参照カウントが 1 以上あること」、所有側は「ポインタが有効な ADM を指し、同じ参照を他者が解放しないこと」を書く
  - メソッド名は `new` / `new_with_handler` の流儀に揃えて実装時に確定する
- 取り込み後も Java 側のオブジェクトを独立して使い続けられること。Java のビルダー項目を将来増やしても本 API の変更が不要であること
- `ScopedRef::from_raw` / `RTCStatsReport::from_refcounted_ptr` をクレート内専用にした判断 (`issues/closed/0078-bug-scoped-ref-from-raw-safety.md`) は、所有権を奪う危険な API を公開面に残すと `unsafe fn` 化しても誤用の余地が残ることを根拠に、汎用 API を外部公開しないというものである。ADM は JNI から渡されたポインタを取り込むニーズがあるため、汎用 API の再公開ではなく用途を限定した `unsafe fn` を追加する
- 生成と削除を同じスレッドで行う契約は新しい API でも維持し、Rustdoc に明記する
- 本 issue は取り込み API の追加に絞り、`webrtc_CreateJavaAudioDeviceModule` の Rust 公開ラッパーは対象外とする。ラッパーを追加する場合は `#[cfg(target_os = "android")]` で gate する必要がある。この関数は `webrtc/src/webrtc_c/android.h` 経由で `target_os == "android"` のときだけ bindgen の入力に入るため、ホストの `cargo test` からは参照できない
- テストはモックを使わず、実際の ADM で参照カウントの増減を確認する。参照カウントを読む C API は無いため、`webrtc_CreateAudioDeviceModuleWithCallback` に渡したハンドラの `OnDestroy` (`src/helper/handler.rs` の `destroy_handler` が `Box` を破棄する) を破棄の観測点にする

## 完了条件

- Java 側が所有する ADM のポインタを取り込める。取り込んだ後に Java 側が `release()` を呼んでも Rust 側の `AudioDeviceModule` が使えること
- 所有権付きで渡された ADM を取り込める。参照カウントを余分に増やさないこと
- `src/tests.rs` に、破棄の観測によるテストを追加する。参照カウントを読む C API は無いため `webrtc_CreateAudioDeviceModuleWithCallback` に渡したハンドラの `OnDestroy` (`src/helper/handler.rs` の `destroy_handler`) を観測点にし、ホスト上で実行する。破棄の観測には `AudioDeviceModule::new_with_handler` の戻り値を使う (`AudioDeviceModule::new` が呼ぶ `webrtc_CreateAudioDeviceModule` は callback を設定しないため観測できない)
  - 借用取り込みでは、1 つ目のハンドルを drop した後も 2 つ目のハンドルが使え、2 つ目のハンドルを drop したときに 1 回だけ破棄されること
  - 所有権取り込みでは、取り込む側が唯一の参照を持つ状態を用意し、取り込んだハンドルを drop したときに 1 回だけ破棄されること (参照が余分に残っていれば破棄されない)
- Rustdoc に所有権とスレッドの契約が書かれていること
- `src/lib.rs` の `compile_fail_doctests` にある「所有権や生ポインタを受け取るコンストラクタは crate 内部専用」という記述を、用途を限定した `unsafe fn` を公開する例外を反映した内容に更新すること
- `cargo fmt --all -- --check` / `cargo clippy --workspace --features source-build --all-targets -- -D warnings` / `cargo test --workspace --features source-build` が通ること
- `CHANGES.md` の `## develop` に `[ADD]` エントリが追加されていること (担当者行を含む)

## 解決方法

所有権の意味が異なる 2 つの取り込みコンストラクタを `AudioDeviceModule` に追加した。どちらも `unsafe fn` で、引数は `*mut c_void`、null の場合は `None` を返す。

- `AudioDeviceModule::from_borrowed_refcounted_ptr` (`src/api/audio_device_module.rs`): 借用中の refcounted ポインタを取り込む。`ScopedRef::from_borrowed_raw` で参照カウントを 1 増やしてから保持するため、呼び出し側の参照は消費しない
- `AudioDeviceModule::from_refcounted_ptr` (同): 所有権を持つ refcounted ポインタを取り込む。参照カウントを増やさずに `ScopedRef::from_raw` で保持する
- `# Safety` に、参照カウントと所有権の契約 (借用側は参照カウントが 1 以上で呼び出し中に最後の参照が解放されないこと、所有側は参照 1 つ分の所有権が呼び出し側にあること)、誤用した場合の帰結 (借用側に所有権付きのポインタを渡すとリークすること、所有側に同じポインタを 2 回渡すと二重解放になること、呼び出し後に渡した参照を解放してはならないこと、借用中のポインタを所有側に渡すと過剰に解放されること)、借用側に渡すポインタの出所 (Java 側が参照を保持している `JavaAudioDeviceModule.getNative(long)` の戻り値など) を書いた
- 型 `AudioDeviceModule` の Rustdoc に、利用者がメソッドを直接呼ぶ場合は同じスレッドから呼び、`PeerConnectionFactory` に渡した後は libwebrtc が worker thread から呼ぶため別スレッドから直接呼ばない契約を明記した。あわせて `as_ptr` / `as_refcounted_ptr` の Rustdoc に、参照カウントを変えず所有権も引き受けないこと、返したポインタが有効なのは `self` の生存中だけであること、共有する場合は `*mut c_void` にキャストして `from_borrowed_refcounted_ptr` に渡すことを書いた
- `ScopedRef::from_borrowed_raw` (`src/helper/ref_count.rs`) を追加し、`ScopedRef::clone` をこれに委譲して参照カウント増加の実装を 1 箇所にまとめた

`src/tests.rs` に 3 つのテストを追加した。破棄の観測点には `AudioDeviceModule::new_with_handler` に渡したハンドラの `Drop` (ADM の `OnDestroy` から `destroy_handler` を経て呼ばれる) を使った。

- 借用取り込みでは、1 つ目のハンドルを drop した後も 2 つ目のハンドルが使え、2 つ目のハンドルを drop したときに 1 回だけ破棄されること
- 所有権取り込みでは、呼び出し側が唯一の参照を持つ状態で取り込み、drop したときに 1 回だけ破棄されること
- null ポインタはどちらのコンストラクタでも `None` になること

`from_borrowed_refcounted_ptr` の Rustdoc には、外部クレートとしてコンパイルされる doctest で ADM を共有して取り込む例を載せ、公開 API であることを固定した。あわせて `src/lib.rs` の `compile_fail_doctests` と `skills/shiguredo-webrtc/SKILL.md` の同趣旨の記述を、用途を限定した `unsafe fn` を公開する例外とその条件を反映した内容に更新し、`CHANGES.md` の `## develop` に `[ADD]` エントリと `### misc` の `[UPDATE]` エントリ (`src/lib.rs` の compile_fail doctest の説明の更新と `ScopedRef::clone` の委譲) を追加した。

`cargo fmt --all -- --check` / `cargo clippy --workspace --features source-build --all-targets -- -D warnings` / `cargo test --workspace --features source-build` が通ることを確認した。
