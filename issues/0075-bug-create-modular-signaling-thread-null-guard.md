# CreateModularPeerConnectionFactoryWithContext の signaling_thread null ガード欠落

- Created: 2026-08-03
- Completed: {YYYY-MM-DD}
- Branch: feature/fix-create-modular-signaling-thread-null-guard
- Polished: {YYYY-MM-DD}

## 目的

signaling_thread 未設定時に `webrtc_CreateModularPeerConnectionFactoryWithContext` が null deref でクラッシュする問題を修正する。

## 現状

`CreateModularPeerConnectionFactoryWithContext` (`webrtc/src/webrtc_c/api/peer_connection_interface.cc` の同名関数) は `dependencies.signaling_thread->BlockingCall(...)` を null ガードなしで実行する。

libwebrtc main の `webrtc::CreateModularPeerConnectionFactory` (`api/create_modular_peer_connection_factory.cc`) は以下のようにガードしており、signaling_thread が null の場合は現在のスレッド上でそのまま処理を続行する:

```cpp
if (dependencies.signaling_thread &&
    !dependencies.signaling_thread->IsCurrent()) {
  return dependencies.signaling_thread->BlockingCall(...);
}
```

`webrtc_PeerConnectionFactoryDependencies_new` はデフォルト構築で `signaling_thread = nullptr` となり、libwebrtc 側は null を有効入力として扱う (`pc/connection_context.cc` の `MaybeWrapThread` が null 時に現在のスレッドをラップする)。つまり、C API の有効な使い方 (signaling_thread 未設定) で null deref クラッシュが発生する。

なお非 context 版の `webrtc_CreateModularPeerConnectionFactory` (`webrtc/src/webrtc_c/api/peer_connection_interface.cc` の同名関数) は libwebrtc の関数をそのまま呼ぶためガード済みであり、2 つの C 関数で挙動が不整合。

## 設計方針

libwebrtc と同じガードを C ラッパー側に追加する。`signaling_thread` が null の場合と、現在のスレッドが signaling_thread と同一の場合に `BlockingCall` をスキップして直接実行する。

## 完了条件

- `signaling_thread` 未設定で `webrtc_CreateModularPeerConnectionFactoryWithContext` を呼んでもクラッシュしないこと
- 既存の正常フロー (signaling_thread 設定済み) の挙動が変わらないこと

## 解決方法

- `CreateModularPeerConnectionFactoryWithContext` (`webrtc/src/webrtc_c/api/peer_connection_interface.cc`) に libwebrtc main と同等のガードを追加する
- ガード時のコードの複製を避けるため、呼び出し部分をローカル関数またはラムダに切り出してから `signaling_thread && !IsCurrent()` の条件で分岐する
