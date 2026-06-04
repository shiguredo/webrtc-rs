# peer_connection_interface.h のコメントアウト済み旧仮想関数を削除する

- Priority: Low
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/refactor-pc-interface-remove-commented-code

## 目的

`webrtc/src/webrtc_c/api/peer_connection_interface.h` の `struct webrtc_PeerConnectionObserver_cbs` には、元になった C++ の `webrtc::PeerConnectionObserver` の仮想関数がコメントとして残っている。対応する関数ポインタは既に実メンバとして定義されており、コメントは冗長な残骸であるため削除する。

## 優先度根拠

ビルドや動作には影響しないコメントのみの問題であり Low とする。ただしヘッダの可読性を下げているため整理する価値はある。

## 現状

レビュー時点で実コードを確認済み。`struct webrtc_PeerConnectionObserver_cbs` の冒頭 (`peer_connection_interface.h:292-297`) に、C++ 由来の仮想関数がコメントとして残っている。

```c
struct webrtc_PeerConnectionObserver_cbs {
  // void OnSignalingChange(
  //     webrtc::PeerConnectionInterface::SignalingState new_state) override {
  //   RTC_LOG(LS_INFO) << "OnSignalingChange: new_state="
  //                    << webrtc::PeerConnectionInterface::AsString(new_state);
  // }
  // void OnDataChannel(webrtc::scoped_refptr<webrtc::DataChannelInterface> data_channel) override {}
```

同じ構造体の末尾 (`peer_connection_interface.h:326-329`) にも同種のコメントが残っている。

```c
  // void OnIceCandidate(const webrtc::IceCandidate* candidate) override {}
  // void OnIceCandidateError(const std::string& address, int port, const std::string& url, int error_code, const std::string& error_text) override {}
  // void OnTrack(webrtc::scoped_refptr<webrtc::RtpTransceiverInterface> transceiver) override {}
  // void OnRemoveTrack(webrtc::scoped_refptr<webrtc::RtpReceiverInterface> receiver) override {}
```

これらに対応する関数ポインタ（`OnConnectionChange`、`OnIceCandidate`、`OnIceCandidateError`、`OnTrack`、`OnRemoveTrack`、`OnDataChannel` など）は、同構造体内 (`peer_connection_interface.h:298-325`) に実メンバとして既に定義されている。

## 設計方針

- `peer_connection_interface.h:292-297` および `peer_connection_interface.h:326-329` のコメントアウト済み旧仮想関数を削除する。
- 実メンバの定義には手を加えない。

## 完了条件

- 当該コメントアウトが除去される。
- 構造体の実メンバ定義が変わらないこと。
