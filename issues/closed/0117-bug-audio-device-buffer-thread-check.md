# Windows のデバッグビルドで AudioDeviceBuffer のデストラクタがスレッド違反になる

- Created: 2026-10-14
- Completed: 2026-10-15
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

`AudioDeviceBuffer` の `main_thread_checker_` は `SequenceChecker::kDetached` で作られ、 最初にチェックしたスレッドに紐付く。 テストでは ADM のハンドルをファクトリに渡したあとも保持していたため、 ファクトリを破棄したあとにテストスレッドで最後の参照を解放していた。 その結果、 利用 (`RegisterAudioCallback`) は worker thread、 破棄はテストスレッドで行われ、 チェックに失敗していた。

- `src/tests.rs` の `set_dummy_audio_device_module` で、 ADM のハンドルを依存に設定したあとすぐに破棄するようにした。 最後の参照は libwebrtc が保持するものになるため、 利用と破棄の両方が worker thread で行われる
- `src/api/audio_device_module.rs` と `src/api/dtls_transport.rs` の Rustdoc を、 libwebrtc の実装に沿ったスレッド契約に修正した。 ADM は生成・利用・破棄をすべて同じスレッドで行い、 ファクトリに渡した場合は worker thread がそのスレッドになる。 `DtlsTransport` の observer の登録と解除は transport を生成したネットワークスレッドから行う
- テストを契約に従わせた。 ネットワークスレッドにソケットサーバーを用意し、 `DtlsTransport` の observer の登録と解除をネットワークスレッド経由にした。 ファクトリに渡す ADM を生成する箇所はテスト用のヘルパーにまとめた
- 停止中のスレッドへ `Thread::blocking_call` を呼び出したときに、 実行されなかったクロージャが解放されない問題を修正した。 libwebrtc の `BlockingCall` は停止中のスレッドではクロージャを実行せずに戻るため、 `Option` で包んで呼び出し元で回収し、 必ず破棄する
- 停止要求を確認できる `Thread::is_quitting` を追加した。 `quit` / `stop` の後は `true` になり、 `stop` で join したスレッドを `start` で再開すると `false` に戻る
- `webrtc/src/webrtc_c/rtc_base/thread.cc` で `self` を受け取る関数に `assert` を追加した
- ADM のスレッドアフィニティを型で扱う設計の検討は `issues/0119-bug-adm-thread-affinity-bypass.md` に分けた

デバッグビルドで 157 件、 リリースビルドで 158 件、 doctest が 9 件すべて成功し、 `cargo fmt --all -- --check` と `cargo clippy --workspace --features source-build --all-targets -- -D warnings` が通ることを確認した。 `tests::get_stats_delivers_report` を含む全テストが成功している。
