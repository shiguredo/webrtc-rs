# PeerConnectionFactory::get_rtp_receiver_capabilities を追加する

Created: 2026-03-27
Completed: 2026-03-27
Model: Opus 4.6

## 背景

コーデックプリファレンス機能の実装において、送信側の `get_rtp_sender_capabilities` は既に C API・Rust バインディングともに実装済みだが、受信側の `get_rtp_receiver_capabilities` が未実装である。

## 根拠

コーデックプリファレンスを設定する際、送信側だけでなく受信側のコーデック一覧も取得できる必要がある。C++ の `PeerConnectionFactoryInterface` には `GetRtpReceiverCapabilities` が存在するが、C API (libwebrtc-c) に対応する関数が未定義のため、Rust 側からも利用できない。

## 対応内容

1. libwebrtc-c に `webrtc_PeerConnectionFactoryInterface_GetRtpReceiverCapabilities` を追加する
2. webrtc-rs の vendored C ヘッダー・実装に同じ変更を反映する
3. `PeerConnectionFactory::get_rtp_receiver_capabilities` Rust メソッドを追加する

## 解決方法

- `webrtc/src/webrtc_c/api/peer_connection_interface.h` に `webrtc_PeerConnectionFactoryInterface_GetRtpReceiverCapabilities` 宣言を追加した
- `webrtc/src/webrtc_c/api/peer_connection_interface.cc` に `PeerConnectionFactoryInterface::GetRtpReceiverCapabilities` を呼び出す C API 実装を追加した
- `src/api/peer_connection.rs` に `PeerConnectionFactory::get_rtp_receiver_capabilities` を追加し、`ffi::webrtc_PeerConnectionFactoryInterface_GetRtpReceiverCapabilities` を Rust から利用できるようにした
- `CHANGES.md` に `[ADD] PeerConnectionFactory::get_rtp_receiver_capabilities` のエントリを追加した
- 実装コミット: `95434d2` (`PeerConnectionFactory::get_rtp_receiver_capabilities を追加する (#41)`)
