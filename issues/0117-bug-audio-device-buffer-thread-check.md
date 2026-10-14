# Windows のデバッグビルドで AudioDeviceBuffer のデストラクタがスレッド違反になる

- Created: 2026-10-14
- Completed: {YYYY-MM-DD}
- Branch: feature/fix-audio-device-buffer-thread-check
- Polished: {YYYY-MM-DD}

## 目的

Windows のデバッグビルド (`--features source-build,debug-build`) で `cargo test` を実行したとき、 `AudioDeviceBuffer` のデストラクタで libwebrtc のスレッド親和性チェックに引っかかってプロセスが異常終了する状態を解消する。

デバッグビルドでは libwebrtc の `RTC_DCHECK` が有効になる。 そのため、 リリースビルドでは表面化しなかった「`AudioDeviceBuffer` を生成したスレッドと破棄するスレッドの不一致」が検出されてテストが止まる。 デバッグビルドを利用できるようにするという `debug-build` feature の目的を満たすには、 この違反を解消する必要がある。

## 現状

### 再現手順

Windows x86_64 で、 README の「デバッグビルド (Windows)」に従って環境変数を設定したうえで、 次のコマンドを実行する。

```powershell
cargo test -p shiguredo_webrtc --lib --features source-build,debug-build -- --test-threads=1 tests::get_stats_delivers_report
```

### 実行結果

```
# Fatal error in: ..\..\..\..\_source\windows_x86_64\webrtc\src\modules\audio_device\audio_device_buffer.cc, line 86
# last system error: 0
# Check failed: (&main_thread_checker_)->IsCurrent()
# # Expected: TQ: 000001C5664B4700 Thread: 0000000000003E9C
# Actual:   TQ: 0000000000000000 Thread: 00000000000048D0
TaskQueue doesn't match
```

`exit code: 0x80000003` (ブレークポイント) でプロセスが異常終了する。 `--test-threads=1` でも再現する。

`cargo test -p shiguredo_webrtc --lib --features source-build,debug-build` を実行した場合、 51 件が成功した後に `tests::get_stats_delivers_report` で停止する。 `tests/test_libyuv.rs` の 35 件は成功する。

### リリースビルドとの差

同じテストはリリースビルドでは成功する。

```powershell
cargo test -p shiguredo_webrtc --lib --features source-build -- --test-threads=1 tests::get_stats_delivers_report
# test result: ok. 1 passed
```

`RTC_DCHECK` が無効なため、 スレッドの不一致そのものは存在していても検出されない。

### 失敗箇所

`modules/audio_device/audio_device_buffer.cc` の `AudioDeviceBuffer::~AudioDeviceBuffer` の先頭で `RTC_DCHECK_RUN_ON(&main_thread_checker_)` を取っている。

`audio_device_buffer.h` の `main_thread_checker_` は `SequenceChecker::kDetached` で初期化される `SequenceChecker` である。 `create_detached` が真のときだけコンストラクタで `Detach()` する。

### `create_detached` は無効のままである

`webrtc_CreateAudioDeviceModule` は `webrtc::CreateAudioDeviceModule` を呼ぶ。 その実装 (`api/audio/create_audio_device_module.cc`) は `AudioDeviceModuleImpl::Create` に委譲し、 これは `AudioDeviceModuleImpl(env, audio_layer)` の 2 引数コンストラクタを使う。 このコンストラクタは `audio_device_buffer_(env)` として `create_detached` の既定値 `false` を使う。

`audio_device_buffer.h` と `audio_device_impl.h` のコメントには「`create_detached` が真なら、 生成したスレッドとは別のスレッドで使える。 テストに有用である」と書かれている。 つまり `Detach()` はテスト用の経路であり、 現状の C ラッパーはその経路を通っていない。

したがって `main_thread_checker_` は `AudioDeviceBuffer` を生成したスレッドに紐付いたままである。 チェックに失敗したときの `Actual: TQ: 0000000000000000` は、 破棄がそのスレッド以外で起きたことを示す。

### 関連する構造

`src/tests.rs` の `get_stats_delivers_report` は、 `PeerConnectionFactoryDependencies` に `AudioDeviceModule::new(&env, AudioDeviceModuleAudioLayer::Dummy)` で生成した ADM を設定し、 `PeerConnectionFactory::create_modular` でファクトリを生成する。 生成した `factory` と `pc` をテストの最後に drop する。 `AudioDeviceBuffer` のデストラクタはこの drop の過程で走る。

デバッグビルドのログには `AudioDeviceModuleImpl::~AudioDeviceModuleImpl` と `AudioDeviceBuffer::~dtor` が並んで出ており、 ADM の破棄がワーカースレッド経由で起きていることがうかがえる。 どのスレッドで `AudioDeviceModule` を破棄するのが libwebrtc の想定であるかを、 テスト側の生成・破棄と C ラッパーの参照カウントの扱いを読んで確定させる必要がある。

### 補足

この問題は libwebrtc を m156.8078.3.5 に上げ、 Windows のビルドを clang-cl + libc++ に変更したことで、 デバッグビルドのテストが実際に走るようになって初めて表面化した。 デバッグ CRT の不一致 (`_CrtDbgReportW` の未解決) は別途解決済みであり、 本 issue はそれとは独立した問題である。

## 完了条件

- Windows x86_64 で `cargo test -p shiguredo_webrtc --lib --features source-build,debug-build` が最後まで完走し、 `tests::get_stats_delivers_report` を含む全テストが成功する
- リリースビルドのテスト結果が変わらない
- `AudioDeviceBuffer` の生成スレッドと破棄スレッドについて、 修正がなぜ libwebrtc の想定に沿うのかをコメントで説明できている
- `cargo fmt --all -- --check` / `cargo clippy --workspace --features source-build --all-targets -- -D warnings` が通る

## 解決方法

未確定。 `create_detached` が無効のまま `AudioDeviceBuffer` を生成スレッド以外で破棄していることは特定できたが、 どちらを直すべきかは決まっていない。

考えられる方向は次の 2 つである。

- `src/tests.rs` の `get_stats_delivers_report` が `AudioDeviceModule` を生成したスレッドで破棄するようにする (テスト側の修正)
- 生成スレッド以外で破棄され得るのが C ラッパーまたは libwebrtc の想定であるなら、 `AudioDeviceModule` の破棄を生成スレッドへ戻す、 または `create_detached` を有効にする経路を用意する (本体側の修正)

`webrtc_CreateAudioDeviceModule` に対応する libwebrtc の公開 API に `create_detached` を指定する口が無いため、 本体側で直す場合は C ラッパー独自の経路を足すことになり、 薄いラッパーを保つ方針 (`webrtc/RULES.md`) と衝突しないかの判断が要る。

テストのみの問題か本体側の問題かは調査で確定させる。 確定するまでは修正方針を決めない。
