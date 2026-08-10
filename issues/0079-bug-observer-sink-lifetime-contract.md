# observer / sink 登録中 drop による use-after-free の契約を文書化する

- Created: 2026-08-03
- Completed: {YYYY-MM-DD}
- Branch: feature/fix-observer-sink-lifetime-contract
- Polished: {YYYY-MM-DD}

## 目的

observer / sink を登録したまま drop すると use-after-free になる契約が API に文書化されておらず、safe Rust で UAF が到達可能な状態を解消する。

## 現状

以下の型は、登録先 (track / transport / data channel) より先に drop すると、C++ 側の wrapper オブジェクト破棄後にコールバックが発火して解放済みメモリへアクセスする構造になっている:

- `DataChannelObserver` (`src/api/data_channel.rs` の `register_observer`): libwebrtc の契約 (`api/data_channel_interface.h` の `UnregisterObserver` を observer 破棄前に呼ぶこと) があり、`unregister_observer` は存在するが、API に要求・文書化されていない。`on_data_channel` コールバックで動的生成されるため特に drop 順序を誤りやすい
- `DtlsTransportObserver` (`src/api/dtls_transport.rs` の `register_observer`)
- `VideoSink` (`src/api/video.rs` の `AddOrUpdateSink`): `remove_sink` を要求していない
- `AudioTrackSink` (`src/api/audio.rs` の `add_sink`)
- `PeerConnectionObserver` (`src/api/peer_connection.rs` の `PeerConnectionDependencies::new`): libwebrtc の契約 (`api/peer_connection_interface.h` の observer 破棄は `Close()` 完了後) があり、Rust API では借用が create 呼び出しで終わり、observer を先に drop できる

C++ 側の wrapper はデストラクタでのみ `OnDestroy(user_data_)` を呼ぶ (例: `webrtc/src/webrtc_c/api/media_stream_interface.cc` の `VideoSinkImpl` 等)。登録解除 (`RemoveSink` / `UnregisterObserver`) は libwebrtc の proxy を経由して非同期に実行されるため、Rust 側の `Drop` との同期がどこにもない。

## 設計方針

API の Rustdoc に drop 順序の契約を明記し、誤用をコンパイル時に防げない場合は実行時の検出手段を提供する。少なくとも以下を文書化する:

- observer / sink を登録した型は、登録先のオブジェクトを破棄する前に `unregister_observer` / `remove_sink` / `Close` を呼ぶか、登録解除を保証する drop 順序を守ること
- コールバックの非同期性 (`RemoveSink` が proxy 経由で非同期に実行される) への言及

## 完了条件

- 上記 5 種の API すべてに drop 順序の契約が Rustdoc で明記されていること
- `src/tests.rs` の既存テストがパスすること

## 解決方法

- 各 API の公開メソッドに `# ライフタイム` セクションを追加し、登録解除・破棄の順序を日本語で記載する
- `PeerConnectionObserver` は `PeerConnection::close` との関係 (`close` 後に observer を drop できること) を記載する
- `DataChannelObserver` は `unregister_observer` を呼ぶべき旨を記載する
- 可能であれば `Debug` アサーションや登録状態のトラッキングにより、誤った drop 順序を検出できる仕組みを検討する (実装可否は調査のうえ判断)
