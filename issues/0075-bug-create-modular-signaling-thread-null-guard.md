# CreateModularPeerConnectionFactoryWithContext の signaling_thread null ガード欠落

- Created: 2026-08-03
- Completed: {YYYY-MM-DD}
- Branch: feature/fix-create-modular-signaling-thread-null-guard
- Polished: 2026-08-12

## 目的

signaling_thread 未設定時に `webrtc_CreateModularPeerConnectionFactoryWithContext` が null deref でクラッシュする問題を修正する。

## 現状

内部ヘルパー関数 `CreateModularPeerConnectionFactoryWithContext` (`webrtc/src/webrtc_c/api/peer_connection_interface.cc`) は `dependencies.signaling_thread->BlockingCall(...)` を null ガードなしで実行する。公開 C 関数 `webrtc_CreateModularPeerConnectionFactoryWithContext` はこのヘルパーを呼ぶ。

libwebrtc (m151.7922.0.0、`branch-heads/7922`) の `webrtc::CreateModularPeerConnectionFactory` (`api/create_modular_peer_connection_factory.cc`) は以下のようにガードしており、signaling_thread が null の場合は現在のスレッド上でそのまま処理を続行する:

```cpp
if (dependencies.signaling_thread &&
    !dependencies.signaling_thread->IsCurrent()) {
  return dependencies.signaling_thread->BlockingCall(...);
}
```

`webrtc_PeerConnectionFactoryDependencies_new` はデフォルト構築で `signaling_thread = nullptr` となり、libwebrtc 側は null を有効入力として扱う (libwebrtc 本体の `pc/connection_context.cc` の `MaybeWrapThread` が null 時に現在のスレッドをラップする)。つまり、C API の有効な使い方 (signaling_thread 未設定) で null deref クラッシュが発生する。

なお非 context 版の `webrtc_CreateModularPeerConnectionFactory` (同じファイルの同名関数) は libwebrtc の `webrtc::CreateModularPeerConnectionFactory` をそのまま呼ぶためガード済みであり、2 つの C 関数で挙動が不整合。

## 設計方針

libwebrtc と同じガードを C ラッパー側に追加する。signaling_thread が null の場合と、現在のスレッドが signaling_thread と同一の場合に `BlockingCall` をスキップして直接実行する。`IsCurrent()` チェックは libwebrtc 本体の `CreateModularPeerConnectionFactory` のガードと同一構造を踏襲するためのものであり、上流仕様への追従と将来の `BlockingCallImpl` 実装変更に対する防御の意味がある。

本ガードは C API のポインタ引数に対する null チェックではなく、`PeerConnectionFactoryDependencies` のメンバに対する libwebrtc 側仕様の追従であり、RULES.md の「C ラッパーではポインタ引数の null チェックを原則として行わない」には抵触しない。

## 完了条件

- `signaling_thread` 未設定で `webrtc_CreateModularPeerConnectionFactoryWithContext` を呼んでもクラッシュしないこと（`src/tests.rs` に `set_signaling_thread` を呼ばずに `create_modular_with_context` が成功するテストを追加して検証する。既存テスト `create_modular_with_context_returns_default_network_objects` の焼き直しでよい。先にテストを追加して修正前のコードで一度落ちることを確認してから修正する。修正前は null deref でテストプロセスが SIGSEGV により終了するため、修正の有無を確実に検出できる。signaling_thread 未設定時は現在のスレッドが wrapped されて signaling_thread になるため、テストは factory / context へのアクセスをすべてテストスレッドから行い、`drop(context)` / `drop(factory)` で必ず破棄すること（wrapped スレッドの unwrap は `~ConnectionContext` でのみ行われるため））
- 既存の正常フロー (signaling_thread 設定済み) の挙動が変わらないこと（`src/tests.rs` の既存の signaling_thread 設定済みテストが green のままであることで検証する。`peer_connection_factory_and_capabilities` と `create_modular_with_context_returns_default_network_objects` が該当する）
- `CHANGES.md` の `## develop` セクションに `[FIX]` エントリが追加されていること

## 解決方法

- `CreateModularPeerConnectionFactoryWithContext` (`webrtc/src/webrtc_c/api/peer_connection_interface.cc` の内部ヘルパー関数) に libwebrtc と同等のガードを追加する
- ガード時のコードの複製を避けるため、呼び出し部分をローカル関数またはラムダに切り出してから `signaling_thread && !IsCurrent()` の条件で分岐する
