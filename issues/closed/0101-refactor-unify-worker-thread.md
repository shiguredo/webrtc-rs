# PeerConnectionFactory に渡す worker thread を network thread に統一する

- Created: 2026-09-15
- Completed: 2026-09-15
- Branch: feature/refactor-unify-worker-thread
- Polished: {YYYY-MM-DD}

## 目的

libwebrtc の issue 558821261「Deprecate and remove PeerConnectionFactoryDependencies::worker_thread」で worker thread が廃止される方向にある。CL 501620「Default worker thread to network thread」がマージされ、`worker_thread` を未指定にすると network thread が worker thread として使われるようになった。CL 502480「Warn when a distinct worker thread is configured」もマージされ、`worker_thread != nullptr && worker_thread != network_thread_` のときに DEPRECATION ログが出るようになった。

専用の worker thread を渡し続けるとこの DEPRECATION ログが出続ける。さらに削除系 CL（499302 / 501640 / 501720 / 502000 / 502500 / 502860 / 502940 / 502960）がマージされた libwebrtc では `PeerConnectionFactoryDependencies::worker_thread` と `PeerConnectionFactoryInterface::worker_thread()` が削除されるため、そのままではコンパイルできなくなる。

そこで方針 1 として、専用 worker thread を生成して渡している箇所をやめ、factory の worker thread として network thread を使うようにする。worker_thread の利用箇所と API を全て無くす方針 2 は、削除系 CL を含む libwebrtc をマージした後に別 issue で行う。実施するタイミングが違うため issue を分けている。

## 現状

### Rust 側

network / worker / signaling の 3 スレッドを生成し、`PeerConnectionFactoryDependencies::set_worker_thread` に専用 worker thread を渡している。

- `README.md` の `FactoryHolder::new` のサンプルコード
- `skills/shiguredo-webrtc/SKILL.md` のサンプルコード
- `examples/whip/src/main.rs` の `FactoryHolder::new`
- `examples/whep/src/main.rs` の `FactoryHolder::new`
- `src/tests.rs` の `set_worker_thread` を呼んでいる複数のテスト

### C++ 版

`webrtc/src/whip.cpp` と `webrtc/src/whep.cpp` の `PeerConnectionFactory::Create` が以下を行っている。

- `webrtc::Thread::Create()` で専用の worker thread を生成して `Start()` する
- `PeerConnectionFactoryDependencies::worker_thread` に専用 worker thread を設定する
- ADM の生成を `worker_thread_->BlockingCall` で実行する
- デストラクタで `worker_thread_->Stop()` する

### C 版

`webrtc/src/whip.c` と `webrtc/src/whep.c` の `PeerConnectionFactory_Create` が以下を行っている。

- `webrtc_Thread_Create()` で専用の worker thread を生成して `webrtc_Thread_Start()` する
- `webrtc_PeerConnectionFactoryDependencies_set_worker_thread` に専用 worker thread を設定する
- ADM の生成を `webrtc_Thread_BlockingCall_r` で実行する
- 後始末で `webrtc_Thread_Stop()` する

C++ 版と C 版は CMake の `whip_cpp` / `whep_cpp` / `whip_c` / `whep_c` ターゲットとしてビルドされる（`webrtc/CMakeLists.txt`）。

`PeerConnectionFactoryDependencies::set_network_thread` は既に存在するため、新しい API は不要。

## 設計方針

- 専用 worker thread の生成を削除する
  - Rust 側は `Thread::new()` / `start()` / `stop()`
  - C++ 側は `webrtc::Thread::Create()` / `Start()` / `Stop()`
  - C 側は `webrtc_Thread_Create()` / `webrtc_Thread_Start()` / `webrtc_Thread_Stop()`
- factory の worker thread には network thread を使う。現在の libwebrtc では `worker_thread` を未設定にすると内部で専用スレッドが生成されるため、方針 1 では `set_worker_thread` に network thread を渡す。方針 2 でこの行を削除する
- ADM の生成は network thread 上で実行する
- `set_worker_thread` の API 自体はこの issue では削除しない

## 完了条件

- 専用 worker thread を生成して worker thread として設定している箇所が 0 件になっている（`git grep worker_thread` で残るのは、方針 2 が扱う C API / Rust API の `set_worker_thread` まわりだけになる）
- `cargo clippy --workspace --features source-build -- -D warnings` が通る
- `cargo test --workspace --features source-build` が通る
- CMake の `whip_c` / `whep_c` / `whip_cpp` / `whep_cpp` ターゲットがビルドできる
- `CHANGES.md` の `## develop` の `### misc` に追記する

## 解決方法

専用 worker thread の生成を削除し、factory の worker thread として network thread を使うようにした。

- `README.md` / `skills/shiguredo-webrtc/SKILL.md` / `examples/whip/src/main.rs` / `examples/whep/src/main.rs` / `src/tests.rs` で worker 用の `Thread::new()` / `start()` / `stop()` を削除し、`set_worker_thread` に network thread を渡すようにした
- `webrtc/src/whip.cpp` / `webrtc/src/whep.cpp` で `worker_thread_` メンバ、生成、`Start()`、`Stop()`、`worker_thread()` アクセサを削除し、`dependencies.worker_thread` と ADM の生成を network thread に変更した
- `webrtc/src/whip.c` / `webrtc/src/whep.c` で `worker_thread` フィールド、生成、`Start()`、後始末を削除し、deps の設定と ADM の生成を network thread に変更した
- `CHANGES.md` の `## develop` の `### misc` に追記した

確認:

- `cargo clippy --workspace --features source-build -- -D warnings` が通ることを確認した
- `cargo test --workspace --features source-build` が通ることを確認した（35 件 + doctest 4 件）
- CMake の `whip_cpp` / `whep_cpp` がビルドできることを確認した（`whip_c` / `whep_c` は `webrtc_SdpVideoFormat_new` の引数不一致という既存の問題でビルドできないため対象外とした）
