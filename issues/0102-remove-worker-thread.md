# libwebrtc の worker_thread 削除に追随して worker_thread を削除する

- Created: 2026-09-15
- Completed: {YYYY-MM-DD}
- Branch: feature/remove-worker-thread
- Polished: {YYYY-MM-DD}

## 目的

libwebrtc の issue 558821261「Deprecate and remove PeerConnectionFactoryDependencies::worker_thread」で worker thread が廃止される。削除系 CL（499302 / 501640 / 501720 / 502000 / 502500 / 502860 / 502940 / 502960）はレビュー中で、マージされると `PeerConnectionFactoryDependencies::worker_thread` と `PeerConnectionFactoryInterface::worker_thread()` が削除される。

先に方針 1（`issues/0101-refactor-unify-worker-thread.md`）で専用 worker thread を network thread に統一しておき、本方針 2 で worker_thread の利用箇所と API を全て無くす。公開 API の削除を伴う破壊的変更であり、実施するタイミングも違うため、方針 1 とは別 issue にしている。

## 前提条件

**webrtc-build が 558821261 の削除 CL を含むバージョンをリリースし、本リポジトリの libwebrtc pin（`Cargo.toml` の `package.metadata.external-dependencies.webrtc-build`）を更新できる状態になっていること。現時点でそのバージョンは存在しないため、この issue はその前提が満たされるまで着手できない。**

## 現状

worker_thread に依存しているのは以下。

- C API: `webrtc/src/webrtc_c/api/peer_connection_interface.h` と `webrtc/src/webrtc_c/api/peer_connection_interface.cc` の `webrtc_PeerConnectionFactoryDependencies_set_worker_thread`
- C API 実装: `webrtc/src/webrtc_c/api/peer_connection_interface.cc` の `CreateModularPeerConnectionFactoryWithContext` が `webrtc::PeerConnectionFactoryProxy::Create(factory->signaling_thread(), factory->worker_thread(), factory)` を呼んでいる
- Rust: `src/api/peer_connection.rs` の `PeerConnectionFactoryDependencies::set_worker_thread`

## 設計方針

- `set_worker_thread`（C API / Rust API）を削除する
- `CreateModularPeerConnectionFactoryWithContext` の proxy 生成は network thread を使う形に置き換える
- 公開 API の削除を伴う破壊的変更なので、`CHANGES.md` の `## develop` に `[CHANGE]` を記載し、しかるべきバージョン更新を行う

## 完了条件

- `set_worker_thread` が C API / Rust API から消えている
- `git grep worker_thread` が issues 以外で 0 件になっている
- `Cargo.toml` の `package.metadata.external-dependencies.webrtc-build` が 558821261 の削除 CL を含むバージョンに更新されている
- `CHANGES.md` の `## develop` に `[CHANGE]` が入る
- `cargo test` が通る

## 解決方法

（詳細は polish / 実装時に確定する）
