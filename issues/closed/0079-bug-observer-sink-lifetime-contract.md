# observer / sink を登録したまま drop した場合の use-after-free の契約を文書化する

- Created: 2026-08-03
- Completed: 2026-08-20
- Branch: feature/fix-observer-sink-lifetime-contract
- Polished: 2026-08-12

## 目的

observer / sink を登録したまま drop すると use-after-free になる契約が API に文書化されておらず、誤用により safe Rust で UAF が発生し得る状態を、契約を Rustdoc に明記して誤用のリスクを低減する。

## 現状

以下の型は、登録先 (track / transport / data channel / peer connection) より先に drop すると、C++ 側の wrapper オブジェクト破棄後にコールバックが発火して解放済みメモリへアクセスする構造になっている:

- `DataChannelObserver` (`src/api/data_channel.rs` の `register_observer`): libwebrtc の契約 (`api/data_channel_interface.h` の「UnregisterObserver should be called before the observer object is destroyed.」) があり、`unregister_observer` は存在するが、API に要求・文書化されていない。登録先の `DataChannel` が `on_data_channel` コールバックで動的生成されるため、特に drop 順序を誤りやすい
- `DtlsTransportObserver` (`src/api/dtls_transport.rs` の `register_observer`): `unregister_observer` は存在するが、要求・文書化されていない
- `VideoSink` (`src/api/video.rs` の `add_or_update_sink`): `remove_sink` は存在するが、要求・文書化されていない
- `AudioTrackSink` (`src/api/audio.rs` の `add_sink`): `remove_sink` は存在するが、要求・文書化されていない
- `PeerConnectionObserver` (`src/api/peer_connection.rs` の `PeerConnectionDependencies::new` / `PeerConnection::create`): libwebrtc の契約 (`api/peer_connection_interface.h` の「after this method completes ... the observer object can be safely destroyed.」) があり、Rust API では `PeerConnectionDependencies::new` に `&PeerConnectionObserver` を渡すと借用が `PeerConnectionDependencies::new` 呼び出しで終わり、observer を先に drop できる

C++ 側の wrapper はデストラクタでのみ `OnDestroy(user_data_)` を呼ぶ (例: `webrtc/src/webrtc_c/api/video/video_sink_interface.cc` の `VideoSinkInterfaceImpl` 等)。登録解除 (`RemoveSink` / `UnregisterObserver`) は、VideoTrack / AudioTrack では libwebrtc の proxy 経由（`pc/proxy.h` の `MethodCall::Marshal` が `PostTask` 後に完了を待つ同期呼び出し）、DataChannel では proxy をバイパスして内部の `BlockingCall` による同期呼び出し（`pc/sctp_data_channel.cc` の `BYPASS_PROXY_METHOD`）、DtlsTransport では直接実装により実行され、いずれも呼び出しが返る時点で登録解除は完了している。スレッド制約は型ごとに異なり、Rust 側の型はこれを明示していない:

- VideoTrack / AudioTrack: proxy 経由の同期呼び出しにより、どのスレッドからでも呼べる (`src/api/video.rs` / `src/api/audio.rs` の `unsafe impl Sync` は proxy を根拠にしている)
- DataChannel: proxy をバイパスした直接実装であり、network thread 以外のどのスレッドからでも呼べるが、network thread からの呼び出しとコールバック内からの呼び出しは不可 (libwebrtc の `pc/sctp_data_channel.cc` が禁止している)
- DtlsTransport: proxy を持たず、`register_observer` / `unregister_observer` は owner thread (network thread) でのみアクセス可能 (libwebrtc の `api/dtls_transport_interface.h` の「This object is created on the network thread, and can only be accessed on that thread, except for functions explicitly marked otherwise.」が根拠。`Information()` は「This function can be called from other threads.」の例外があり、Rust の `DtlsTransport::state` はどのスレッドからでも呼べる)

各型の `Drop` は登録解除を呼ばないため、登録したまま drop すると libwebrtc 内部に未解除の登録が残り、コールバック発火時に解放済みメモリへアクセスする。

## 設計方針

API の Rustdoc に drop 順序の契約を明記する。以下を文書化する:

- 各型の登録 API (`register_observer` / `add_or_update_sink` / `add_sink` / `PeerConnectionDependencies::new`) に、登録解除 (`unregister_observer` / `remove_sink` / `close`) を呼んでから drop することを明記する
- 登録解除 API は同期であり、呼び出しが返った時点でコールバックが発火しないことが保証される旨を明記する（根拠は型ごとに書き分ける。VideoTrack は libwebrtc の `api/video/video_source_interface.h` の「RemoveSink must guarantee that at the time the method returns, there is no current and no future calls to VideoSinkInterface::OnFrame.」が API 契約として存在する。AudioTrack / DataChannel / DtlsTransport は API 契約としての保証文言がなく、同期実行（proxy 経由 / `BlockingCall` / 直接実装）に依存する旨を doc に添える）
- スレッド制約を型ごとに書き分けて明記する（VideoTrack / AudioTrack はどのスレッドからでも呼べる。DataChannel は network thread 以外から呼べるが、コールバック内から呼んではならない。コールバックは signaling thread で発火する。DtlsTransport の `register_observer` / `unregister_observer` は owner thread (network thread) でのみ呼べる。`state` はどのスレッドからでも呼べる旨を併記する）
- `PeerConnectionObserver` は、`PeerConnection::close` を呼んでから drop すること（`PeerConnection` に `Drop` impl はなく、PeerConnection が生存中に `close` を呼ばずに observer を先に drop した場合は observer のコールバック停止が保証されない。逆に `PeerConnection` を先に drop した場合（最後の参照の解放）は、C++ 側の破棄が同期で完了するため、その後 observer を drop してよい）
- `DataChannel::close` は `unregister_observer` を呼ばないため、close 後も observer は登録解除してから drop すること（close 後の最後の `on_state_change` で UAF になる誤用を防ぐ）

## 完了条件

- 以下の API すべてに drop 順序の契約が Rustdoc で明記されていること:
  - `DataChannel::register_observer` (`src/api/data_channel.rs`): `unregister_observer` を呼んでから observer を drop すること
  - `DtlsTransport::register_observer` (`src/api/dtls_transport.rs`): `unregister_observer` を呼んでから observer を drop すること
  - `VideoTrack::add_or_update_sink` (`src/api/video.rs`): すべての登録先の `remove_sink` を呼んでから sink を drop すること
  - `AudioTrack::add_sink` (`src/api/audio.rs`): すべての登録先の `remove_sink` を呼んでから sink を drop すること
- `PeerConnectionDependencies::new` の doc (`src/api/peer_connection.rs`): 渡した observer は `close` を呼んでから drop すること（`PeerConnection::create` が失敗した場合は `PeerConnectionDependencies` を drop した後に observer を drop してよい旨も併記する）
- `PeerConnection::create` の doc (`src/api/peer_connection.rs`): observer は `close` を呼んでから drop すること（`create` が失敗した場合は `PeerConnectionDependencies` を drop した後に observer を drop してよい旨も併記する）
- `PeerConnection::close` の doc (`src/api/peer_connection.rs`): `close` を呼ぶと、呼び出しが返った時点で observer のコールバックが発火しないことが保証され、`close` 後に observer を drop できること
- `PeerConnectionObserver` の型 doc (`src/api/peer_connection.rs`): `close` を呼んでから drop すること
- `DataChannel::close` の doc (`src/api/data_channel.rs`): close は `unregister_observer` を呼ばない旨を明記すること
- 登録解除 API (`unregister_observer` × 2 / `remove_sink` × 2) に、呼び出しが返った時点でコールバックが発火しないことが保証される旨が明記されていること（VideoTrack は API 契約。AudioTrack / DataChannel / DtlsTransport は実装依存であり、スレッド制約つきの書き分けで記載する。DataChannel はコールバック内から呼ばない旨も明記する）
- 本 issue の変更は doc 追記のみで挙動の変化が無いため、テストは不要である
- `CHANGES.md` の `## develop` セクションに `### misc` サブセクションを新設し、[UPDATE] エントリが追記されていること

## 解決方法

- 各 API の登録メソッドの Rustdoc に登録解除・破棄の順序を日本語で記載する（既存の `# Safety` セクション形式には合わせず、`# ライフタイム` セクションを新設する）。登録 API の doc に、登録解除の同期保証と型ごとのスレッド制約（どのスレッドからでも呼べる / network thread 以外から呼べる / owner thread のみ）を併記する。DataChannel の doc にはコールバック内から `unregister_observer` を呼ばない旨も併記する
- `PeerConnectionObserver` は `PeerConnection::close` との関係を記載する（`close` の既存 doc「この呼び出し後、PeerConnectionObserver のコールバックは呼ばれなくなる」に「`close` 後に observer を drop できること」を追記する。`PeerConnection::create` の doc、`PeerConnectionDependencies::new` の doc、`PeerConnectionObserver` の型 doc にも同旨を記載する）
- 実行時検出機構（`Debug` アサーションや登録状態のトラッキング）の検討は、本 issue のスコープ外とし別 issue で対応する
- 1 回限り observer（`CreateSessionDescriptionObserver` / `SetLocalDescriptionObserver` / `SetRemoteDescriptionObserver`）は、参照カウント管理（`make_ref_counted`）により Rust ラッパーを先に drop しても実体は C++ 側の参照で生存し、本 issue が扱う「登録したまま drop すると UAF」の構造ではない。ただしコールバック発火時のライフタイム契約は未文書化であり、本 issue の対象外とする（別 issue で対応）
- 同種の「登録したまま drop」構造を持つその他の API も本 issue の対象外とする（本 issue は上記の 4 型 + `PeerConnectionObserver` に限定する。対象外の型は `src/api/audio_device_module.rs` の `AudioDeviceModule::new_with_handler` のように、handler の所有権が C++ 側へ移譲され `OnDestroy` で解放される構造のものも含むが、これは「登録したまま drop すると UAF」にはならないため対象外）
