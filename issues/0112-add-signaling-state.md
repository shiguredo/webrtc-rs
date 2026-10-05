# SignalingState の変化を購読できるようにする

- Created: 2026-10-05
- Completed: {YYYY-MM-DD}
- Branch: feature/add-signaling-state
- Polished: {YYYY-MM-DD}

## 目的

`PeerConnectionObserverHandler` から SignalingState (offer / answer の交換状態) の変化を購読できるようにする。sora-rust-sdk などの利用側が、WebRTC レベルのネゴシエーション状態を観測できるようにする。

## 現状

`PeerConnectionObserverHandler` (`src/api/peer_connection.rs`) には `on_connection_change` (`PeerConnectionState`) と `on_standardized_ice_connection_change` (`IceConnectionState`) と `on_ice_gathering_change` (`IceGatheringState`) があるが、SignalingState のコールバックが無い。

SignalingState に対応する公開型も無い。`PeerConnectionState` / `IceConnectionState` / `IceGatheringState` は `src/api/peer_connection.rs` に定義され、`from_int` で C 側の定数を変換している。同じ階層に `SignalingState` が無い。

C 側には次の欠落がある。

- `webrtc_PeerConnectionObserver_cbs` (`webrtc/src/webrtc_c/api/peer_connection_interface.h`) に `OnSignalingChange` の関数ポインタが無い
- `webrtc_PeerConnectionInterface_SignalingState` の型と定数が C ヘッダーに無い (`PeerConnectionState` / `IceConnectionState` / `IceGatheringState` は `typedef int` と `extern const int` で定義されている)
- `PeerConnectionObserverImpl::OnSignalingChange` (`webrtc/src/webrtc_c/api/peer_connection_interface.cc`) が空実装になっている
- SignalingState の getter は無い (状態の取得は observer のコールバックという既存の設計に合わせる)

`webrtc/src/whep.c` と `webrtc/src/whip.c` は `webrtc_PeerConnectionObserver_cbs` を `calloc` で 0 初期化し、一部のコールバックだけを設定している。`OnSignalingChange` を追加した場合は設定しないと null 呼び出しになる。

## 設計方針

- `SignalingState` を `src/api/peer_connection.rs` に追加する。既存の状態 enum と同じ形にする (`Stable` / `HaveLocalOffer` / `HaveRemoteOffer` / `HaveLocalPranswer` / `HaveRemotePranswer` / `Closed` / `Unknown(i32)`)。`from_int` を実装する
- `PeerConnectionObserverHandler` に `on_signaling_change(&mut self, new_state: SignalingState)` を追加する。既定実装は空にする
- `observer_on_signaling_change` の `extern "C"` 関数を追加し、`PeerConnectionObserver::new_with_handler` の `webrtc_PeerConnectionObserver_cbs` の初期化に `OnSignalingChange` を追加する
- C ヘッダーに `webrtc_PeerConnectionInterface_SignalingState` の `typedef int` と各値の `WEBRTC_EXPORT extern const int` を追加する (`webrtc_PeerConnectionInterface_SignalingState_kStable` と `_kHaveLocalOffer` と `_kHaveRemoteOffer` と `_kHaveLocalPranswer` と `_kHaveRemotePranswer` と `_kClosed`)
- C 側で各定数を `webrtc::PeerConnectionInterface::SignalingState` の値で定義する
- `webrtc_PeerConnectionObserver_cbs` に `OnSignalingChange` の関数ポインタを追加する。全コールバック必須の規約 (ヘッダーのコメントと `webrtc_PeerConnectionObserver_new` の assert) に合わせて assert にも追加する
- `PeerConnectionObserverImpl::OnSignalingChange` から `observer_.OnSignalingChange` を呼ぶ
- `webrtc/src/whep.c` と `webrtc/src/whip.c` の `observer_cbs` に `OnSignalingChange` を設定する

## 完了条件

- `PeerConnectionObserverHandler::on_signaling_change` が SignalingState の変化で呼ばれる
- `Stable` / `HaveLocalOffer` / `HaveRemoteOffer` / `HaveLocalPranswer` / `HaveRemotePranswer` / `Closed` を区別できる
- 既存の公開 API の挙動が変わらない

## 変更対象

- `src/api/peer_connection.rs` (`SignalingState`、`PeerConnectionObserverHandler`、`observer_on_signaling_change`、`PeerConnectionObserver::new_with_handler`)
- `webrtc/src/webrtc_c/api/peer_connection_interface.h`
- `webrtc/src/webrtc_c/api/peer_connection_interface.cc`
- `webrtc/src/whep.c` と `webrtc/src/whip.c`
- `skills/shiguredo-webrtc/SKILL.md` (イベント一覧の確認)
