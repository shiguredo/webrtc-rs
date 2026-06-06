# 未使用のスレッドアクセサを削除する

- Priority: Low
- Polished: 2026-06-05
- Created: 2026-06-05
- Model: Opus 4.8

## 目的

`webrtc/src/whip.cpp` と `webrtc/src/whep.cpp` の `class PeerConnectionFactory` には、呼び出し箇所のないスレッドアクセサ（`network_thread()` / `worker_thread()` / `signaling_thread()`）が定義されている。デッドコードを削除してクラスを整理する。

## 優先度根拠

ビルドや動作には影響しないデッドコードであり Low とする。ただし未使用アクセサはクラスの公開面を不必要に広げるため整理する価値はある。

## 現状

レビュー時点で実コードと `rg` による参照状況を確認済み。`whip.cpp:159-165` の `class PeerConnectionFactory` に 3 つのアクセサが定義されている。

```cpp
  webrtc::Thread* network_thread() const { return network_thread_.get(); }
  webrtc::Thread* worker_thread() const { return worker_thread_.get(); }
  webrtc::Thread* signaling_thread() const { return signaling_thread_.get(); }
  webrtc::scoped_refptr<webrtc::PeerConnectionFactoryInterface>
  peer_connection_factory() const {
    return factory_;
  }
```

`whep.cpp:146-152` にも同じ 3 つのアクセサが定義されている。

`rg "network_thread\(\)|worker_thread\(\)|signaling_thread\(\)"` で確認したところ、3 つのアクセサはいずれも定義（`whip.cpp:159-161`、`whep.cpp:146-148`）のみで、呼び出し箇所がない（いずれも参照ゼロを確認）。なお同じクラスの `peer_connection_factory()` は別物で、`whip.cpp:1115` および `whep.cpp:881` から呼び出されており、削除対象ではない。

## 設計方針

- `whip.cpp:159-161` および `whep.cpp:146-148` の `network_thread()` / `worker_thread()` / `signaling_thread()` を削除する。
- `peer_connection_factory()` および各スレッドのメンバ変数（`network_thread_` など、デストラクタで使用）には手を加えない。

## 完了条件

- 両ファイルから未使用アクセサ `network_thread()` / `worker_thread()` / `signaling_thread()` が除去される。
- `whip` と `whep` の挙動が変わらないこと。
