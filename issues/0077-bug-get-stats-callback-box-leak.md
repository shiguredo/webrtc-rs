# get_stats のコールバック未発火時に user_data の Box がリークする

- Created: 2026-08-03
- Completed: {YYYY-MM-DD}
- Branch: feature/fix-get-stats-callback-box-leak
- Polished: {YYYY-MM-DD}

## 目的

`PeerConnection::get_stats` (`src/api/peer_connection.rs`) で、コールバックが呼ばれない経路で `Box::into_raw` した user_data が恒久リークする問題を修正する。

## 現状

`PeerConnection::get_stats` は `PeerConnectionStatsCallbackState` (クロージャを保持する `Box`) を `Box::into_raw` で C 側に渡し、`peer_connection_on_stats` (同ファイル) で `Box::from_raw` により回収する。C 側の `RTCStatsCollectorCallbackImpl` (`webrtc/src/webrtc_c/api/peer_connection_interface.cc` の同名クラス) は `OnStatsDelivered` コールバックのみを持ち、破棄通知 (`OnDestroy`) を持たない。

libwebrtc の `RTCStatsCollector` は PC 破棄時に `CancelPendingRequestAndGetShutdownTasks` で安全フラグを無効化するのみで、pending のコールバックに `OnStatsDelivered` を呼ばずに破棄する (`pc/rtc_stats_collector.cc` の `~RTCStatsCollector`)。つまり:

- レポート配信前に PC が破棄された場合
- `OnStatsDelivered` が null report で early return する場合 (report パラメータの null 経路)

のどちらかで `Box::from_raw` が実行されず、クロージャの `Box` がリークする。特に「`get_stats` 呼び出し直後に PC を drop する」正常フローで発火しうる。

## 設計方針

C 側の `RTCStatsCollectorCallbackImpl` に破棄通知 (`OnDestroy`) を追加し、コールバック未発火のまま C++ オブジェクトが破棄される場合に Rust 側の `Box` を回収できるようにする。

## 完了条件

- PC 破棄時に `get_stats` のコールバックが未発火でも `Box` がリークしないこと
- `OnStatsDelivered` が正常に呼ばれる経路の挙動が変わらないこと

## 解決方法

- `webrtc/src/webrtc_c/api/peer_connection_interface.h` の `webrtc_RTCStatsCollectorCallback_cbs` に `OnDestroy` を追加する
- `RTCStatsCollectorCallbackImpl` (`webrtc/src/webrtc_c/api/peer_connection_interface.cc`) にデストラクタを追加し、`OnDestroy(user_data_)` を呼ぶ
- `RTCStatsCollectorCallbackImpl` のコンストラクタで `assert(cbs->OnDestroy != nullptr)` を追加する (RULES.md の Cbs 契約に従う)
- Rust 側 `get_stats` の cbs 初期化に `OnDestroy` を追加し、破棄時に `Box::from_raw` で回収するハンドラを実装する
- `peer_connection_on_stats` の null report パスが二重回収しないようにする (既に `Box::from_raw` 済みのため、null report 時は回収済みの `Box` を drop するだけに整理する)
