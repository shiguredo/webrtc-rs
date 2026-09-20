# safe API だけでデータ競合に到達できる箇所を無くす

- Created: 2026-09-20
- Completed: {YYYY-MM-DD}
- Branch: feature/fix-safe-api-data-race
- Polished: {YYYY-MM-DD}

## 目的

`unsafe` を一切書かずにデータ競合 (未定義動作) に到達できる公開 API を無くす。

`issues/0104-bug-safe-api-use-after-free.md` で use-after-free に到達する経路を洗い出した際に、ポインタの寿命とは別の原因で「同じ実体へ並行にアクセスできる」箇所が見つかった。use-after-free は解放済みメモリへのアクセスの問題であり、こちらは同じ実体への並行アクセスの問題なので別 issue として扱う。

## 現状

### (1) 共有ハンドルから safe な可変アクセスを配っている型

`I420Buffer` / `NV12Buffer` は `Send` で、`data_mut` / `y_data_mut` / `planes_mut` などの safe な可変アクセサを持つ。`I420Buffer` 自身は `Clone` を持たないが、同じ C++ 実体を指す所有ハンドルを複数作る経路が safe API にある。

- `I420Buffer::cast_to_video_frame_buffer` (`src/api/video_codec_common.rs`) は `&self` から同じ実体を指す `VideoFrameBuffer` を返す
- `VideoFrameBuffer::as_i420` / `as_nv12` (同) は `&self` から同じ実体を指す `I420Buffer` / `NV12Buffer` を返す
- `VideoFrameBuffer` は `Clone` と `Send` を持つ

`VideoFrameBuffer::to_i420` / `crop_and_scale` は C++ 側で非 const である (`api/video/video_frame_buffer.h` の `ToI420` / `CropAndScale`)。`Clone` でハンドルを複製できるため、同じ実体に対して非 const 呼び出しを同時に行える。`webrtc/src/webrtc_c/api/video/video_frame_buffer.cc` の `VideoFrameBufferImpl::ToI420` / `CropAndScale` は `cbs_` 経由で Rust のハンドラを呼ぶので、`VideoFrameBufferHandler` の状態にも同時に `&mut` が入る。

このため次のコードが safe で通り、同じ画素メモリへの可変アクセスが 2 本できる。

```rust
let mut a = I420Buffer::new(640, 480);
let vfb = a.cast_to_video_frame_buffer();
let mut b = vfb.as_i420().expect("i420");
std::thread::spawn(move || { b.data_mut()[0] = 1; });
a.data_mut()[0] = 2;
```

並行させなくても、同一スレッドで `&mut [u8]` を 2 本同時に生かせる時点で Rust の参照規則に反する。

libwebrtc は可変アクセスに唯一性を要求していない。

- `api/video/i420_buffer.h` の `I420Buffer::MutableDataY` は `DataY` を `const_cast` するだけである
- `api/video/video_frame_buffer.h` の `VideoFrameBuffer` は `RefCountInterface` を継承し、唯一性を問い合わせる公開 API を持たない

つまり「共有中は書き換えない」という規約で回避しており、Rust 側は規約ではなく型か実行時チェックで表現する必要がある。

なお `scoped_refptr<I420Buffer>` はコピー可能であり、`I420Buffer` に `Clone` を追加するのが自然である (同じ crate でも `VideoFrameBuffer` / `VideoTrack` / `VideoTrackSource` / `AudioDeviceModule` は `ScopedRef::clone` による `Clone` を実装している)。`Clone` を追加すると、この経路はより直接になる。

### (2) libwebrtc が同時に呼ばないことを確認したハンドラ

各ハンドラが `Send` のみを要求し `&mut self` で呼ばれること自体は、libwebrtc が同じ実体を同時に呼ばないなら問題ない。libwebrtc の実装を確認した結果、以下は同時呼び出しにならない。

- `PeerConnectionObserverHandler` (`src/api/peer_connection.rs`): `api/peer_connection_interface.h` の `signaling_thread()` に「The thread on which all PeerConnectionObserver callbacks will be invoked」とある
- `DtlsTransportObserverHandler` (`src/api/dtls_transport.rs`): `api/dtls_transport_interface.h` に「This object is created on the network thread, and can only be accessed on that thread, except for functions explicitly marked otherwise.」とある
- `VideoEncoderHandler` / `VideoDecoderHandler` (`src/api/video_encoder.rs` / `src/api/video_decoder.rs`): `video/video_stream_encoder.cc` は専用の `encoder_queue_` 上で `EncodeVideoFrame` / `SetEncoderRates` から encoder を呼び、`video/video_receive_stream2.cc` の `DecodeAndMaybeDispatchEncodedFrame` は `RTC_DCHECK_RUN_ON(&decode_sequence_checker_)` を持つ
- `AudioEncoderFactoryHandler` / `AudioDecoderFactoryHandler` (`src/api/audio.rs`): `audio/audio_send_stream.cc` の `AudioSendStream::Reconfigure` (`RTC_DCHECK_RUN_ON(&worker_thread_checker_)`) から `ReconfigureSendCodec` → `SetupSendCodec` を経て `encoder_factory->Create` が呼ばれる
- `DataChannelObserverHandler` (`src/api/data_channel.rs`): `api/data_channel_interface.h` の `IsOkToCallOnTheNetworkThread` の doc に、既定値の `false` は「notifications will be delivered on the signaling thread associated with the peerconnection instance」とある。`pc/sctp_data_channel.cc` の通知も `CacheStateAndCallBackOnSignalingThread` で signaling thread へ渡され、コールバックは `RTC_DCHECK_RUN_ON(signaling_thread())` の下で実行される
- `VideoSinkHandler` (`src/api/video.rs`): `VideoBroadcaster::OnFrame` (`api/video/video_broadcaster.cc`) は broadcaster ごとの `sinks_and_wants_lock_` の下で sink を呼ぶ。1 つの登録先では常に直列に呼ばれる
- `AudioTrackSinkHandler` (`src/api/audio.rs`): `RemoteAudioSource::OnData` (`pc/remote_audio_source.cc`) は「Called on the externally-owned audio callback thread, via/from webrtc.」というコメントを持ち、source ごとの `sink_lock_` の下で sink を呼ぶ。1 つの登録先では常に直列に呼ばれる

これらは `Send` のみで正しく、変更は不要である。ただし現状の Rustdoc には呼び出しスレッドの記載が無く、同じ懸念を繰り返し調査することになる。

### (3) 同時呼び出しがあり得ることを確認したハンドラ

- `LogSinkHandler` (`src/rtc_base/logging.rs`): `rtc_base/logging.cc` の `LogMessage::~LogMessage` は、`config.sinks()` のループではログロックを取らずに `sink->OnLogMessage(log_line_)` を呼ぶ (直後の legacy な `streams_` のループは `MutexLock lock(&GetLoggingLock())` を取る)。crate の `LogSink` は `LoggingConfig::AddSink` で `config.sinks_` に入り、`log::initialize_logging` でグローバルに設定される。ログはどのスレッドからでも出るため、同じ sink が同時に呼ばれ得る

### (4) 同じ handler を複数の登録先で使うことを型で禁止できれば望ましいもの

`VideoSink` / `AudioTrackSink` の登録 API は `&self` と `&sink` を取るため、同じ sink を複数の track に登録できる。登録先が別スレッドで配信する場合は同時呼び出しになる。

ただし、1 sink 1 登録先で使う限り libwebrtc は単一スレッドでしか呼ばないし、crate としても同じ handler を複数の登録先で使い回すことは想定していない。そのため優先度は低く、どう扱うかは実装時に判断する。

### (5) `Send` の宣言が型と合っていないもの

- `SdpAudioFormatRef<'a>` (`src/api/audio.rs`) は `Send` を宣言しているが、`SdpAudioFormat` は `Sync` ではない。借用が生きている間は元の値を可変で借りられないため到達経路は確認できていないが、宣言の根拠が型に無い
- `RawBufferWriter<'a, T>` (`src/util.rs`) は `T: Send` を要求しない無条件の `Send` である。`T` は現状 `i16` のみで、ハンドラのメソッド引数の匿名ライフタイムにより呼び出しの外へ持ち出せないため到達経路は確認できていない

### 対象外 (調査の結果 safe API からデータ競合に到達しないと確認したもの)

- `BufferRef` / `BufferRefMut` / `BufferS16Ref` / `BufferS16RefMut` (`src/rtc_base/buffer.rs`): 借用が生きている間は元の `Buffer` を可変で借りられないため、別スレッドの読み書きと競合しない
- `VideoFrame` / `EncodedImage` / `EncodedImageBuffer` (`src/api/video_codec_common.rs`): `Clone` は実体のコピー (`webrtc_VideoFrame_copy` など) であるか存在せず、可変アクセサが他のハンドルと実体を共有しない

## 設計方針

「同じ実体を共有し得る型からは、唯一性が保証されない限り可変アクセスを公開しない」を基本方針とする。`Send` を外して回避する方法は採らない (フレームを別スレッドへ渡す用途が成立しなくなるため)。

### (1) 共有ハンドルからの可変アクセス

`Clone` は C++ の `scoped_refptr` と同じ挙動として維持したうえで、可変アクセサを次のいずれかにする。どれを採るかは実装時に確定する。

- `unsafe fn` にして「この実体への他のハンドルが無いこと」を `# Safety` に書く。変更が最小である (`RawBufferWriter::write` と同じ方針)
- 唯一所有の型に可変アクセサを移す (`I420BufferBuilder` のような型を作り、`freeze` で共有型にする)。型で保証でき、safe な可変アクセスを残せる
- 唯一性を実行時に確認して `Option<&mut [u8]>` を返す。ただし libwebrtc は唯一性を問い合わせる公開 API を持たない (`RefCountedBase::HasOneRef` は protected である)。実体は `make_ref_counted` が作る `rtc::RefCountedObject<T>` で、その `HasOneRef` は public virtual なので C API 側で `dynamic_cast` すれば取得できるが、`make_ref_counted` の内部型に依存する。恒久的に使うなら libwebrtc 側に公開の問い合わせを足す

`Send` は維持し、書き手を排除できることを前提に `Sync` (読み取り専用の並行アクセス) を追加できるかも併せて検討する。

### (2) 単一スレッドで呼ばれるハンドラ

- 変更は不要である。確認した根拠 (どのスレッドから呼ばれるか) を Rustdoc に書く

### (3) 同時呼び出しがあり得るハンドラ

- `LogSinkHandler`: `FrameTransformerHandler` と同じ形 (`Send + Sync` を要求して `&self` にし、状態はハンドラ側で保護する) に揃える。libwebrtc が config sinks をロック外で呼ぶため、sink 側で排他する必要がある

### (4) 同じ handler を複数の登録先で使うことの禁止 (任意)

`VideoSink` / `AudioTrackSink` について、同じ handler を複数の登録先で使わないことをどう扱うかは、実装時に改めて判断する。現時点で検討した案と trade-off は以下。

- 文書化のみ: Rustdoc に 1 sink 1 登録先と書く。変更が最小である。型でも実行時でも保証はしない
- 実行時チェック: 登録時に登録先の同一性を sink 側に記録し、別の登録先への登録を `Err` または panic にする。`&sink` のままで wants の更新も許せる。型保証ではないが、オブジェクトの占有も `Drop` のスレッド問題も無い。`issues/closed/0081-add-observer-sink-drop-detection.md` で実行時検出を断念した経緯があるが、これは登録時 1 回の決定的な検査であり、コールバック中に登録状態を検出する話とは性質が違う
- track 側で sink を作る API に変える: `VideoTrack::create_sink(handler, wants) -> VideoSink` のように sink を track が作り、sink が登録先を保持して `Drop` で解除する。構造的に 1 sink 1 登録先になり、ガードのような占有も無い。`VideoTrack` / `AudioTrack` の解除は proxy 経由で任意スレッドから呼べるため `Drop` 解除と相性が良い。公開 API の変更は大きい
- `&mut` を取るガード型: 登録中 sink を排他借用するため型で強制できるが、次の問題がある
  - 登録中は sink を一切操作できなくなる。今回の対象型は公開メソッドが `new_with_handler` と `as_ptr` しかないため実害は小さいが、将来の API 追加を制限する
  - ガードの `Drop` が解除を呼ぶが、解除できるスレッドは型ごとに異なる。`DtlsTransport` の登録解除は owner thread (network thread) のみ (`api/dtls_transport_interface.h`) なので、別スレッドで drop すると契約違反になる。`issues/0104-bug-safe-api-use-after-free.md` の登録系をこの形にするのは `DtlsTransport` には適用できない
  - `let _ = track.add_or_update_sink(...)` と書くとその場でガードが drop されて解除される
  - 同じ track への `add_or_update_sink` による wants の更新ができなくなるため、ガードに更新用のメソッドが必要になる

このため `issues/0104-bug-safe-api-use-after-free.md` の登録系は `unsafe fn` と `# Safety` を基本とし、ガード型は採らない。

### (5) `Send` の宣言

- `SdpAudioFormatRef` は `SdpAudioFormat` に `Sync` を実装するか `Send` を外す
- `RawBufferWriter` は `T: Send` を要求する

## 完了条件

- `I420Buffer` / `NV12Buffer` / `VideoFrameBuffer` について、safe Rust で同じ実体への可変アクセスを同時に 2 本作れないこと
- `I420Buffer` に `Clone` を追加した場合も同じ条件が保たれること
- `LogSinkHandler` について、同じ sink が同時に呼ばれても未定義動作にならないこと
- `VideoSink` / `AudioTrackSink` について、1 つの handler を複数の登録先で使わないことが、Rustdoc への記載・実行時チェック・型のいずれかで表現されていること (どの方法を採るかは実装時に確定する)
- 単一スレッドで呼ばれることを確認したハンドラについて、その根拠が Rustdoc に書かれていること
- `SdpAudioFormatRef` / `RawBufferWriter` の `Send` の根拠が型で表現されていること
- `unsafe impl Send` / `Sync` を追加・変更した箇所には、その根拠がコメントで書かれていること
- `src/tests.rs` の既存テストがパスすること
- `cargo fmt --all -- --check` / `cargo clippy --workspace --features source-build --all-targets -- -D warnings` / `cargo test --workspace --features source-build` が通ること
- 公開 API の変更を伴うため `CHANGES.md` の `## develop` に `[CHANGE]` が記載されていること

## 解決方法

（詳細は polish / 実装時に確定する）
