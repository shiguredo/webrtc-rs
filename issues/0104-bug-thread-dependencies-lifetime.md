# 依存に設定した `Thread` を先に drop すると use-after-free になる契約を直す

- Created: 2026-09-18
- Completed: {YYYY-MM-DD}
- Branch: feature/fix-thread-dependencies-lifetime
- Polished: {YYYY-MM-DD}

## 目的

`PeerConnectionFactoryDependencies::set_network_thread` / `set_signaling_thread` (`src/api/peer_connection.rs`) は safe な公開 API でありながら、`&Thread` から取り出した生ポインタを C++ 側の依存へ保存するだけで借用が呼び出しで終わる。このため `Thread` を先に drop するだけで safe Rust から use-after-free に到達でき、`unsafe` を一切書かずにメモリ破壊を起こせる状態になっている。

`PeerConnectionFactoryDependencies` に渡す `Thread` は、生成する factory の生存期間ずっと有効でなければならない。この契約を API の形で表現する。

## 現状

### Rust 側

`src/api/peer_connection.rs` の以下 3 つが `&Thread` を受け取り、`thread.raw()` の生ポインタを C++ 側へ渡すだけになっている。

- `PeerConnectionFactoryDependencies::set_network_thread`
- `PeerConnectionFactoryDependencies::set_worker_thread`
- `PeerConnectionFactoryDependencies::set_signaling_thread`

いずれも doc は「start 済みの Thread を設定する。ライフサイクル管理は呼び出し側で行う。」のみで、「factory より長生きさせること」を要求していない。

### C++ 側

- `webrtc/src/webrtc_c/api/peer_connection_interface.cc` の `webrtc_PeerConnectionFactoryDependencies_set_network_thread` / `set_signaling_thread` は `deps->network_thread = thread;` / `deps->signaling_thread = thread;` と生ポインタを代入するだけで所有権を取らない
- `refs/webrtc/src/api/peer_connection_interface.h` の `PeerConnectionFactoryDependencies` も `Thread*` の非所有ポインタとして定義されている
- `refs/webrtc/src/api/create_modular_peer_connection_factory.cc` の `CreateModularPeerConnectionFactory` が `dependencies.signaling_thread->IsCurrent()` で即座にデリファレンスする。この時点で解放済みなら UAF になる
- C API の `webrtc_CreateModularPeerConnectionFactory` は `std::move(*deps)` で依存を消費し、その後 `deps` を delete する

### 再現手順

`unsafe` を一切使わずに以下がコンパイルを通る。

```rust
let mut deps = PeerConnectionFactoryDependencies::new();
// 一時値は文の終わりで drop され、deps は解放済み Thread を指す
deps.set_signaling_thread(&Thread::new());
let _ = PeerConnectionFactory::create_modular(deps);
```

```rust
let deps = {
    let mut thread = Thread::new();
    thread.start();
    let mut deps = PeerConnectionFactoryDependencies::new();
    deps.set_network_thread(&thread);
    deps
}; // ここで thread が drop され、deps は解放済み Thread を指す
let _ = PeerConnectionFactory::create_modular(deps);
```

`Thread::new()` は `webrtc::Thread::Create()` の実体を一意所有し、`Thread::into_raw` を呼ばない限り `Drop` で破棄される。`Thread` の生ポインタを安全に得る公開 API は `Thread::raw` のほかに無く、呼び出し側が factory の寿命まで生存を保証する手段が型に無い。

### 関連 issue

- `issues/closed/0079-bug-observer-sink-lifetime-contract.md` は observer / sink 5 件と `AudioDeviceModule::new_with_handler` を対象に、drop 順序の契約を Rustdoc に明記する方針で closed になっている。本 issue が扱う `set_network_thread` / `set_signaling_thread` はその対象に含まれていない
- `issues/closed/0081-add-observer-sink-drop-detection.md` は登録状態の実行時検出を「C++ 側が真の登録状態を持つため Rust 側では実現できない」として closed にしている
- `issues/0102-remove-worker-thread.md` で `set_worker_thread` は削除予定である。本 issue の対象は `set_network_thread` / `set_signaling_thread` の 2 つとし、`set_worker_thread` は 0102 の削除に任せる

## 設計方針

`Thread` を factory より長生きさせる契約を API の形で表現する。候補は以下。

- **A. `unsafe fn` にして契約を `# Safety` に書く**: 生存保証を呼び出し側の責任として明示する。変更が最小で、`Thread` を呼び出し側のスコープに固定する既存の使い方（`src/tests.rs` の各テスト）をそのまま維持できる
- **B. `Thread` の所有権を依存が引き取る**: `set_*_thread` が `Thread` を値で受け取り、`Thread::into_raw` で所有権を C++ 側へ移す。呼び出し側が先に drop できなくなるため型で保証できるが、`PeerConnectionFactoryDependencies` が `Thread` を所有することになり、`std::move` で消費される既存の流れと整合させる設計が必要になる
- **C. 借用にライフタイムを付ける**: factory が依存より長生きするため `PhantomData<&'a Thread>` では不十分であり、単独では安全にできない

A と B のどちらを採るかは実装時に決定する。判断材料として、`Thread` は `unsafe impl Send` のみを持ち `Clone` を持たない一意所有型であり、複数の factory で同じ `Thread` を共有する使い方は現状のテストに無いことを確認している。

## 完了条件

- `set_network_thread` / `set_signaling_thread` のどちらについても、safe Rust で `Thread` を先に drop して UAF に到達する経路が無くなっていること
- 設定した `Thread` が factory より長生きしなければならない契約が、`# Safety` セクションまたは型（所有権）で表現されていること
- `PeerConnectionFactory::create_modular` / `create_modular_with_context` の doc に、依存に設定した `Thread` の生存要求が明記されていること
- `Thread::raw` の doc に、返した生ポインタの生存を呼び出し側が保証しなければならない旨が明記されていること
- `src/tests.rs` の既存テストがパスすること
- `cargo fmt --all -- --check` / `cargo clippy --workspace --features source-build --all-targets -- -D warnings` / `cargo test --workspace --features source-build` が通ること
- 公開 API の変更を伴うため `CHANGES.md` の `## develop` に `[CHANGE]` が記載されていること

## 解決方法

（詳細は polish / 実装時に確定する）
