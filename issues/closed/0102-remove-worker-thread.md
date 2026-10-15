# libwebrtc の worker_thread 削除に追随して worker_thread を削除する

- Created: 2026-09-15
- Completed: 2026-10-16
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

前提条件は「削除 CL を含むバージョンが出るまで着手できない」としていたが、現 pin の m156.8078.3.5 の時点で libwebrtc が `PeerConnectionFactoryDependencies::worker_thread` を既に非推奨にしており、network thread と別の worker thread を渡す用途を廃止している。deprecation を根拠に削除する方針へ変更し、pin の更新を待たずに対応した。

現 pin での libwebrtc の挙動はソースで確認した。

- `pc/connection_context.cc` の `ConnectionContext` は `worker_thread` が未指定のとき network thread を worker thread として使う。別のスレッドは生成しない
- 同ファイルで `worker_thread` が network thread と異なるときだけ `RTC_LOG(LS_ERROR)` で DEPRECATION を出力する。network thread を渡していた従来の呼び出しでは出力されない
- `PeerConnectionFactory::worker_thread()` は非推奨になっていない。`ConnectionContext::worker_thread()` を返すだけのアクセサであり、削除 CL 後も network thread を返す形で残る

したがって worker thread は常に network thread に解決されるため、`set_worker_thread` を削除しても挙動は変わらない。

- C API の `webrtc_PeerConnectionFactoryDependencies_set_worker_thread` の宣言と実装を削除した
- Rust の `PeerConnectionFactoryDependencies::set_worker_thread` を削除した
- `create_modular_with_context` の proxy 生成は `factory->worker_thread()` のままにした。削除 CL 後も network thread を返すため、network thread へ置き換える必要はない
- `src/tests.rs` の 20 箇所、`examples/whip` / `examples/whep`、`README.md`、`skills/shiguredo-webrtc/SKILL.md` から呼び出しを削除した
- `AudioDeviceModule` のスレッド契約は worker thread ではなく network thread を指すようになったため、`src/api/audio_device_module.rs` と `CHANGES.md` の該当記述を修正した
- `CHANGES.md` の `## develop` に `[CHANGE]` を追記した

完了条件との差異:

- `Cargo.toml` の `package.metadata.external-dependencies.webrtc-build` は更新していない。deprecation を根拠に削除する方針へ変更し、現 pin で完結させたため
- 目的に挙げていた `PeerConnectionFactoryInterface::worker_thread()` は現 pin の libwebrtc に存在しない。削除対象は `PeerConnectionFactoryDependencies::worker_thread` のみだった

確認:

- `git grep worker_thread` は issues 以外で 0 件になった
- `cargo fmt --all -- --check` が通ることを確認した
- `cargo clippy --workspace --features source-build --all-targets -- -D warnings` が通ることを確認した
- `cargo test --workspace --features source-build` が通ることを確認した（158 + 35 + 1 + 8 件）
- `python3 webrtc/run.py format --check` が通ることを確認した
