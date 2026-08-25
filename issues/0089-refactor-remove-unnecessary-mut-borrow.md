# 読み取り専用なのに `&mut` で受け取っている引数を `&` に変更する

- Created: 2026-08-26
- Completed: {YYYY-MM-DD}
- Branch: feature/refactor-immutable-borrow-for-readonly-args
- Polished: {YYYY-MM-DD}

## 目的

`src/api/peer_connection.rs` の公開 API のうち、対応する C API が引数を一切書き換えないにもかかわらず `&mut` で受け取っている 6 箇所の引数を `&` に変更し、読み取り専用であることを型で明示する。

現状は呼び出し側が「options / init / config を渡すだけ」の用途で `let mut` を強制され、書き換えが起きないにもかかわらず可変束縛が必要になる。Rust の参照設計としては、読み取り専用で渡す参照は `&T` が正しく、`&mut T` は無用な制約になる。

## 現状

- `src/api/peer_connection.rs` の以下のメソッドが引数を `&mut` で受け取っている
  - `PeerConnection::create` の `config: &mut PeerConnectionRtcConfiguration`
  - `PeerConnection::set_configuration` の `config: &mut PeerConnectionRtcConfiguration`
  - `PeerConnection::create_offer` の `options: &mut PeerConnectionOfferAnswerOptions`
  - `PeerConnection::create_answer` の `options: &mut PeerConnectionOfferAnswerOptions`
  - `PeerConnection::create_data_channel` の `init: &mut DataChannelInit`
  - `PeerConnection::add_transceiver` / `PeerConnection::add_transceiver_with_track` の `init: &mut RtpTransceiverInit`
- 対応する C API (`webrtc/src/webrtc_c/api/peer_connection_interface.cc`) は引数を書き換えない
  - `webrtc_PeerConnectionFactoryInterface_CreatePeerConnectionOrError` は rtc_config を const 参照で libwebrtc に渡す
  - `webrtc_PeerConnectionInterface_SetConfiguration` は config を const 参照で渡す
  - `webrtc_PeerConnectionInterface_CreateOffer` / `CreateAnswer` は options を const 参照で渡す (null のときはローカルの空オプションを使う)
  - `webrtc_PeerConnectionInterface_CreateDataChannelOrError` は init を const ポインタとして libwebrtc に渡す
  - `webrtc_PeerConnectionInterface_AddTransceiver` / `AddTransceiverWithTrack` は init を deref してコピーする
- 一方、以下の引数は `&mut` が正しく変更対象外
  - `create_modular` / `create_modular_with_context` / `create` の `deps` は C 側が `std::move` で移動消費する
  - `create_offer` / `create_answer` の `observer` はコールバック経由でハンドラ状態が書き換わる

## 設計方針

- 上記 6 箇所の引数を `&mut T` から `&T` に変更する
- C API 側は変更しない (const 扱いのまま。変更の必要がない)
- `PeerConnectionFactory::set_options` はすでに `&PeerConnectionFactoryOptions` で正しいため対象外

## 完了条件

- 上記 6 箇所の引数がすべて `&T` になっている
- ビルドと全テストが通る
- 呼び出し側で `let mut` が不要になる

## 解決方法

- `src/api/peer_connection.rs` の該当メソッドのシグネチャを `&T` に変更する
- 呼び出し側 (`src/tests.rs` など) の `let mut ...` を `let ...` に変更する
