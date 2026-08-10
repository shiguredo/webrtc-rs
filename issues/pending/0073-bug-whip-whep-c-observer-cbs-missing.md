# whip.c / whep.c の PeerConnectionObserver コールバックが未設定でクラッシュする

- Created: 2026-08-03
- Completed: {YYYY-MM-DD}
- Branch: feature/fix-whip-whep-c-observer-cbs
- Polished: {YYYY-MM-DD}

## pending の理由

whip/whep 関連の対応を保留するため

## 目的

C 版サンプル (`webrtc/src/whip.c` / `webrtc/src/whep.c`) がどのビルド構成でも正常動作しない致命的バグを修正する。

## 現状

`SignalingWhip_Create` (`webrtc/src/whip.c` の `SignalingWhip_Create`) は `observer_cbs` に `OnConnectionChange` の 1 個のみ、`SignalingWhep_Create` (`webrtc/src/whep.c` の `SignalingWhep_Create`) は `OnConnectionChange` / `OnTrack` / `OnRemoveTrack` の 3 個のみ設定している。

一方、C API 側 (`webrtc/src/webrtc_c/api/peer_connection_interface.h` の `webrtc_PeerConnectionObserver_cbs`) は「全コールバックは必須 (null 非許容)」と明記し、`PeerConnectionObserverImpl` のコンストラクタ (`webrtc/src/webrtc_c/api/peer_connection_interface.cc` の `PeerConnectionObserverImpl`) は 9 個すべての non-null を `assert` する。未設定のまま残っているコールバック:

- `OnStandardizedIceConnectionChange`
- `OnIceCandidate`
- `OnIceCandidateError`
- `OnIceGatheringChange`
- `OnDataChannel`
- `OnDestroy`

この結果:

- Debug ビルド: `webrtc_PeerConnectionObserver_new` 呼び出し時の `assert` で即 abort する
- Release ビルド: `assert` が無効なため、ICE 収集中に必ず発火する `OnIceCandidate` の NULL 関数ポインタ呼び出し、および `webrtc_PeerConnectionObserver_delete` → `~PeerConnectionObserverImpl` の `OnDestroy(user_data_)` 呼び出しで確実にクラッシュする

なお C++ 版 (`webrtc/src/whip.cpp` / `webrtc/src/whep.cpp`) はクラス自体が `PeerConnectionObserver` を継承しており、この問題は存在しない。

## 設計方針

未設定の 6 個のコールバックに空実装を追加する。既存の C 版のスタイル (シグナリングスレッドで呼ばれる `RTC_LOG` 付き実装) に合わせ、未使用コールバックは `RTC_LOG_INFO` 程度のログを残すか、無視する空実装とする。`OnDestroy` は `SignalingWhip` / `SignalingWhep` 構造体の解放に必要な処理 (あれば) を実装し、不要なら空実装とする。

C 版は `OnIceCandidate` が空実装のままだと candidate がサーバに届かない (既知 issue 0015 等と関連) が、本 issue は「クラッシュの解消」に絞り、ICE candidate の送信処理は別 issue とする。

## 完了条件

- `SignalingWhip_Create` / `SignalingWhep_Create` が `webrtc_PeerConnectionObserver_cbs` の全 9 コールバックを non-null で設定すること
- Debug ビルドで `SignalingWhip_Connect` / `SignalingWhep_Connect` の `webrtc_PeerConnectionObserver_new` が abort しないこと
- Release ビルドで ICE 収集から切断までクラッシュせず、`webrtc_PeerConnectionObserver_delete` が正常に完了すること

## 解決方法

- `webrtc/src/whip.c` の `SignalingWhip_Create`: `OnStandardizedIceConnectionChange` / `OnIceCandidate` / `OnIceCandidateError` / `OnIceGatheringChange` / `OnDataChannel` / `OnDestroy` の 6 個を追加設定する
- `webrtc/src/whep.c` の `SignalingWhep_Create`: 同 6 個を追加設定する (うち `OnTrack` / `OnRemoveTrack` は設定済み)
- 対応する実装関数 (`SignalingWhip_OnIceCandidate` 等) を追加する
