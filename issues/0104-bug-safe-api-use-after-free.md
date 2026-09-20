# safe API だけで use-after-free に到達できる箇所を無くす

- Created: 2026-09-18
- Completed: {YYYY-MM-DD}
- Branch: feature/fix-safe-api-use-after-free
- Polished: {YYYY-MM-DD}

## 目的

`unsafe` を一切書かずに use-after-free や二重解放に到達できる公開 API を無くす。

`PeerConnectionFactoryDependencies::set_network_thread` / `set_signaling_thread` (`src/api/peer_connection.rs`) は safe な公開 API でありながら、`&Thread` から取り出した生ポインタを C++ 側の依存へ保存するだけで借用が呼び出しで終わる。このため `Thread` を先に drop するだけで safe Rust から use-after-free に到達でき、`unsafe` を一切書かずにメモリ破壊を起こせる状態になっている。

同じ構造、つまり「C++ が Rust 側オブジェクトへの非所有ポインタを保存し、Rust 側の型がその生存期間を表現していない」公開 API は `Thread` だけではない。`webrtc/src/webrtc_c` の C API 全体から、libwebrtc に非所有ポインタを保存させる呼び出し (observer / sink の登録と、依存構造体への生ポインタ代入) を洗い出して Rust の公開 API と対応付けた結果、以下に挙げる 9 件が確認できた。

本 issue は個別のバグとして `Thread` だけを直すのではなく、「safe API から use-after-free に到達できる経路を残さない」という一つの方針として 9 件すべてを扱う。

## 現状

### 対象 (safe API から use-after-free に到達できるもの)

Rust 側の型は C++ 実体を一意所有して `Drop` で破棄するか、`ConnectionContext` からの借用が呼び出しで終わる。一方 C++ 側は渡されたポインタを保存し、呼び出し後も参照し続ける。

| # | Rust API | C++ 側の保存箇所 |
|---|---|---|
| 1 | `PeerConnectionFactoryDependencies::set_network_thread` (`src/api/peer_connection.rs`) | `webrtc_PeerConnectionFactoryDependencies_set_network_thread` (`webrtc/src/webrtc_c/api/peer_connection_interface.cc`) が `deps->network_thread` に代入する |
| 2 | `PeerConnectionFactoryDependencies::set_signaling_thread` (同) | `webrtc_PeerConnectionFactoryDependencies_set_signaling_thread` が `deps->signaling_thread` に代入する |
| 3 | `PeerConnectionFactoryDependencies::set_worker_thread` (同) | `webrtc_PeerConnectionFactoryDependencies_set_worker_thread` が `deps->worker_thread` に代入する |
| 4 | `PeerConnectionDependencies::new` (同) | `webrtc_PeerConnectionDependencies_new` が `webrtc::PeerConnectionDependencies` を observer の生ポインタで構築する |
| 5 | `PeerConnectionDependencies::set_proxy` (同) | `webrtc_PeerConnectionDependencies_set_proxy` が `webrtc::BasicPortAllocator` を `NetworkManager*` / `PacketSocketFactory*` で構築する。借用元は `ConnectionContext` |
| 6 | `DataChannel::register_observer` (`src/api/data_channel.rs`) | `webrtc_DataChannelInterface_RegisterObserver` (`webrtc/src/webrtc_c/api/data_channel_interface.cc`) が libwebrtc の `DataChannelInterface::RegisterObserver` に生ポインタを渡す |
| 7 | `DtlsTransport::register_observer` (`src/api/dtls_transport.rs`) | `webrtc_DtlsTransportInterface_RegisterObserver` (`webrtc/src/webrtc_c/api/dtls_transport_interface.cc`) が libwebrtc の `DtlsTransportInterface::RegisterObserver` に生ポインタを渡す |
| 8 | `VideoTrack::add_or_update_sink` (`src/api/video.rs`) | `webrtc_VideoTrackInterface_AddOrUpdateSink` (`webrtc/src/webrtc_c/api/media_stream_interface.cc`) が libwebrtc の `VideoBroadcaster` に `VideoSinkInterface<VideoFrame>*` を保存させる |
| 9 | `AudioTrack::add_sink` (`src/api/audio.rs`) | `webrtc_AudioTrackInterface_AddSink` (同) が libwebrtc の `AudioTrack` 経由で `RemoteAudioSource` に `AudioTrackSinkInterface*` を保存させる (ローカル track の source は保存しない) |

### 再現手順

`Thread` (#1 / #2) は `unsafe` を一切使わずに次のコードがコンパイルを通る。

```rust
let mut deps = PeerConnectionFactoryDependencies::new();
// 一時値は文の終わりで drop され、deps は解放済み Thread を指す
deps.set_signaling_thread(&Thread::new());
let _ = PeerConnectionFactory::create_modular(deps);
```

```rust
let deps = {
    let mut thread = Thread::new();
    thread.start();
    let mut deps = PeerConnectionFactoryDependencies::new();
    deps.set_network_thread(&thread);
    deps
}; // ここで thread が drop され、deps は解放済み Thread を指す
let _ = PeerConnectionFactory::create_modular(deps);
```

登録系 (#6〜#9) と observer 依存 (#4) は、登録 API が `&T` を取るだけで所有権もライフタイムも結び付けていないため、登録解除を呼ばずに drop するだけで到達する。例えば #6 は次の順で到達する。

```rust
let observer = DataChannelObserver::new_with_handler(Box::new(handler));
data_channel.register_observer(&observer);
drop(observer); // unregister_observer を呼んでいない
// 以降 DataChannel がコールバックを発火すると解放済み observer を触る
```

#5 は `ConnectionContext` を保持する `PeerConnectionFactory` より `PeerConnection` が長生きすると、port allocator が解放済み `NetworkManager` / `PacketSocketFactory` を触る。

`Thread::new` は `webrtc::Thread::Create()` の実体を一意所有し、`Thread::into_raw` を呼ばない限り `Drop` で破棄する。`Thread` の生ポインタを安全に得る公開 API は `Thread::raw` のほかに無く、呼び出し側が factory の寿命まで生存を保証する手段が型に無い。observer / sink についても同様に、登録先より長生きさせることを型で表現する手段が無い。

### 関連 issue

- `issues/closed/0079-bug-observer-sink-lifetime-contract.md` は observer / sink 5 件と `AudioDeviceModule::new_with_handler` を対象に、drop 順序の契約を Rustdoc に明記する方針で closed になっている。契約の文書化では safe API からの到達を防げないため、本 issue は #4 と #6〜#9 を対象に含め、API 側で防ぐ方針に置き換える
- `issues/closed/0081-add-observer-sink-drop-detection.md` は登録状態の実行時検出を「C++ 側が真の登録状態を持つため Rust 側では実現できない」として closed にしている
- `issues/0102-remove-worker-thread.md` で #3 の `set_worker_thread` は削除予定である。本 issue では #3 を 0102 の削除に任せる

### 対象外 (調査の結果 safe API から UAF に到達しないと確認したもの)

- refcounted 型を `&T` で受け取る API (`set_audio_encoder_factory` / `set_audio_decoder_factory` / `set_audio_device_module` / `MediaStream::add_audio_track` / `MediaStream::add_video_track` / `RtpSender::set_track` / `RtpSender::set_frame_transformer` / `RtpReceiver::set_frame_transformer` / `VideoFrame::set_video_frame_buffer` / `PeerConnection::add_transceiver_with_track` / `PeerConnection::add_track` / `PeerConnection::remove_track`) は、C++ 側が `scoped_refptr` への代入か値コピーで保持するため Rust 側の drop では解放されない
- `unique_ptr` へ move する API (`set_event_log_factory` / `set_audio_processing_builder` / `set_video_encoder_factory` / `set_video_decoder_factory` / `PeerConnectionDependencies::set_tls_cert_verifier` / `IceServer::set_tls_client_identity` / `EnvironmentFactory::set_field_trials` / `LoggingConfig::add_sink`) は所有権が C++ へ移る
- `set_env` は `std::optional<webrtc::Environment>` への値コピーで、`webrtc::Environment` は utility の実体を `scoped_refptr` で共有する
- 一方通行 observer (`CreateSessionDescriptionObserver` / `SetLocalDescriptionObserver` / `SetRemoteDescriptionObserver`) は `make_ref_counted` と `Release` で参照カウント管理される
- コールバックで渡される借用型 (`LogLineRef` / `EncodedImageRef` / `VideoFrameRef` など) は trait メソッドの匿名ライフタイムに束縛されるため、ハンドラから呼び出しの外へ持ち出せない

### 対象外 (UAF ではないが safe API から未定義動作に到達し得るもの)

本 issue の完了条件には含めず、別途扱う。

- `Send` / `Sync` の根拠が不足していることによるデータ競合 (例: `I420Buffer` / `NV12Buffer` の参照カウント越しの可変アクセス、ハンドラの `&mut self` が同時に呼ばれ得る型)
- `AudioProcessingBuilder::into_raw` を外部から呼んで戻り値を捨てた場合の leak

## 設計方針

「C++ が非所有ポインタを保持する API は、生存期間の契約を `unsafe fn` の `# Safety` として要求するか、型 (所有権・ライフタイム) で保証する」を基本方針とする。契約の Rustdoc への記載だけで済ませない。

対象ごとの方針は以下を基本とし、どちらを採るかは実装時に確定する。

- 登録系 (#6〜#9): `unsafe fn` にして、登録解除を呼んでから drop することを `# Safety` に書く。変更が最小で、`src/tests.rs` の既存の使い方をそのまま維持できる。代わりに、登録解除を `Drop` で行うガード型を返す設計にすれば safe のままにできるため、公開 API の形状変更を許容できるならこちらを優先する
- observer 依存 (#4): `PeerConnectionDependencies::new` を `unsafe fn` にするか、`PeerConnection` が observer の所有権を引き取る形にして型で保証する
- `Thread` (#1 / #2): `set_network_thread` / `set_signaling_thread` を `unsafe fn` にするか、依存が `Thread` の所有権を引き取る形にして型で保証する。`Thread` は `unsafe impl Send` のみを持ち `Clone` を持たない一意所有型であり、複数の factory で同じ `Thread` を共有する使い方は現状のテストに無い
- proxy (#5): 借用元が `ConnectionContext` で、`PeerConnection` がそれより長生きし得るため `PhantomData` の借用では表現できない。`unsafe fn` と `# Safety` を基本とする

`unsafe fn` にする場合、呼び出し側は `unsafe {}` で囲むだけで意味は変わらない。現時点の呼び出し箇所は `src/tests.rs` と `examples/` の合計で `Thread` の 3 種が 57、`PeerConnectionDependencies::new` が 16、登録系が 5 である。

## 完了条件

- #1 / #2 について、safe Rust で `Thread` を先に drop して use-after-free に到達する経路が無くなっていること
- #4 / #5 / #6 / #7 / #8 / #9 について、safe Rust で登録先・依存先より先に対象を drop して use-after-free に到達する経路が無くなっていること
- #3 は `issues/0102-remove-worker-thread.md` の削除をもって解消とし、本 issue では対応しないこと
- 上記すべてについて、生存期間の契約が `# Safety` セクションまたは型 (所有権・ライフタイム) で表現されていること
- `PeerConnectionFactory::create_modular` / `create_modular_with_context` の doc に、依存に設定した `Thread` の生存要求が明記されていること
- `Thread::raw` の doc に、返した生ポインタの生存を呼び出し側が保証しなければならない旨が明記されていること
- `src/tests.rs` の既存テストがパスすること
- `cargo fmt --all -- --check` / `cargo clippy --workspace --features source-build --all-targets -- -D warnings` / `cargo test --workspace --features source-build` が通ること
- 公開 API の変更を伴うため `CHANGES.md` の `## develop` に `[CHANGE]` が記載されていること

## 解決方法

（詳細は polish / 実装時に確定する）
