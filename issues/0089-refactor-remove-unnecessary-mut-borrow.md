# 実体とずれている `&mut` / `&mut self` を `&`・値渡し・`&self` に変更する

- Created: 2026-08-26
- Completed: {YYYY-MM-DD}
- Branch: feature/refactor-immutable-borrow-for-readonly-args
- Polished: {YYYY-MM-DD}

## 目的

`src/api/peer_connection.rs` の公開 API のうち、mutability の使い方が実体とずれているシグネチャを修正し、読み取り・所有・共有の実態を型で明示する。

対象は以下の 3 系統。

- 対応する C API が引数を一切書き換えないのに `&mut` で受け取っている引数を `&` に変更する
- 対応する C API が `std::move` で引数を消費するのに `&mut` で受け取っている引数を値渡し (`Deps`) に変更する
- 共有ハンドルの実体を持ちながら `&mut self` になっているレシーバを `&self` に変更する

現状の呼び出し側は「options / init / config / deps を渡すだけ」の用途でも `let mut` を強制され、書き換えも消費も起きないにもかかわらず可変束縛が必要になる。Rust の参照設計としては、読み取り専用で渡す参照は `&T` が正しく、消費する値は所有権の移動 (値渡し) が正しく、共有ハンドルへのメソッドは `&self` が正しい。

## 現状

### 読み取り専用なのに `&mut` で受け取っている引数 (6 箇所)

`src/api/peer_connection.rs` の以下のメソッドが引数を `&mut` で受け取っているが、対応する C API は引数を一切書き換えない。

- `PeerConnection::create` の `config: &mut PeerConnectionRtcConfiguration`
- `PeerConnection::set_configuration` の `config: &mut PeerConnectionRtcConfiguration`
- `PeerConnection::create_offer` の `options: &mut PeerConnectionOfferAnswerOptions`
- `PeerConnection::create_answer` の `options: &mut PeerConnectionOfferAnswerOptions`
- `PeerConnection::create_data_channel` の `init: &mut DataChannelInit`
- `PeerConnection::add_transceiver` / `PeerConnection::add_transceiver_with_track` の `init: &mut RtpTransceiverInit`

対応する C API (`webrtc/src/webrtc_c/api/peer_connection_interface.cc`) は引数を書き換えない。

- `webrtc_PeerConnectionFactoryInterface_CreatePeerConnectionOrError` は rtc_config を const 参照で libwebrtc に渡す
- `webrtc_PeerConnectionInterface_SetConfiguration` は config を const 参照で渡す
- `webrtc_PeerConnectionInterface_CreateOffer` / `CreateAnswer` は options を const 参照で渡す (null のときはローカルの空オプションを使う)
- `webrtc_PeerConnectionInterface_CreateDataChannelOrError` は init を const ポインタとして libwebrtc に渡す
- `webrtc_PeerConnectionInterface_AddTransceiver` / `AddTransceiverWithTrack` は init を deref してコピーする

### C 側が `std::move` で消費するのに `&mut` で受け取っている引数 (3 箇所)

以下の `deps` は C 側が `std::move` で中身を消費する。`&mut` だと「呼び出し後に再利用可能」という誤った含意になるため、値渡しにして所有権の移動を型で示す。

- `PeerConnectionFactory::create_modular` の `deps: &mut PeerConnectionFactoryDependencies`
- `PeerConnectionFactory::create_modular_with_context` の `deps: &mut PeerConnectionFactoryDependencies`
- `PeerConnection::create` の `deps: &mut PeerConnectionDependencies`

libwebrtc 本体の C++ API 自体が `deps` を値渡しで受け取っており、C ラッパー (`webrtc_CreateModularPeerConnectionFactory` / `webrtc_CreateModularPeerConnectionFactoryWithContext` / `webrtc_PeerConnectionFactoryInterface_CreatePeerConnectionOrError`) は `std::move(*deps)` で消費する。同じ流儀の既存例として、`PeerConnection::set_local_description` / `set_remote_description` の `desc`、`initialize_logging` の `config`、`IceServer::set_tls_client_identity` の `identity` がすでに値渡しになっている。

値渡し化しても、deps が参照するスレッド・ファクトリー・observer のライフサイクルは従来どおり呼び出し側管理であり、moved-from 状態の deps を delete しても安全である。

### `&mut self` になっているレシーバ (2 箇所)

- `PeerConnection::add_ice_candidate` (`&mut self`)
- `PeerConnection::set_configuration` (`&mut self`)

`impl PeerConnection` の他のメソッド (`create_offer` / `set_local_description` / `set_remote_description` / `add_track` / `close` など) は全て `&self` であり、状態を変えるものも `&self` で動く。`PeerConnection` はシグナリングスレッド上の実体を `PeerConnectionProxy` 経由で直列化アクセスする共有ハンドルで (interior-mutable)、純 Rust で実装するなら `Rc<RefCell<PeerConnectionInner>>` のような形になり、全メソッドが `&self` になる。`&mut self` は「wrapper が状態を一意に所有して直接変異する」という別モデルの署名であり、実体と矛盾する。

### 変更対象外

- `PeerConnection::create_offer` / `create_answer` の `observer: &mut CreateSessionDescriptionObserver` はコールバック経由でハンドラ状態が書き換わるため `&mut` が正しい
- `PeerConnectionFactory::set_options` はすでに `&PeerConnectionFactoryOptions` で正しいため対象外

## 設計方針

- 読み取り専用の 6 引数を `&mut T` から `&T` に変更する
- C 側が消費する 3 引数を `&mut T` から値渡し `T` に変更する
- 2 メソッドのレシーバを `&mut self` から `&self` に変更する
- C API 側は変更しない (const 扱い・std::move のまま。変更の必要がない)

## 完了条件

- 上記 6 引数がすべて `&T` になっている
- 上記 3 引数がすべて値渡しになっている
- 上記 2 レシーバが `&self` になっている
- ビルドと全テストが通る
- 対応する呼び出し側で `let mut` が不要になっている

## 解決方法

- `src/api/peer_connection.rs` の該当メソッドのシグネチャを修正する
- 呼び出し側 (`src/tests.rs`、`examples/whip`、`examples/whep` など) の `let mut ...` を `let ...` に、`&mut deps` を `deps` に変更する
