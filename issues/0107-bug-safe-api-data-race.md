# safe API だけでデータ競合に到達できる箇所を無くす

- Created: 2026-09-20
- Completed: 2026-10-07
- Branch: feature/fix-safe-api-data-race
- Polished: 2026-10-06

## 目的

`unsafe` を一切書かずにデータ競合 (未定義動作) に到達できる公開 API を無くす。

`issues/0104-bug-safe-api-use-after-free.md` で use-after-free に到達する経路を洗い出した際に、ポインタの寿命とは別の原因で「同じ実体へ並行にアクセスできる」箇所が見つかった。use-after-free は解放済みメモリへのアクセスの問題であり、こちらは同じ実体への並行アクセスの問題なので別 issue として扱う。

LogSink と LoggingConfig の初期化を先行して安全化し、その後に共有映像バッファと残りのハンドラを対応する。先行対応だけで本 issue 全体を完了とはしない。observer の登録経路については 0104 の対応と組み合わせて排他条件を満たす必要がある。

以下は `shiguredo_webrtc 0.154.1-canary.5` と、依存する `webrtc-build m154.8037.4.1` の libwebrtc commit `7ee41bfd3f443545b06456f47f14a52bdf71180a` を確認した結果である。

## 現状

### (1) 共有ハンドルから safe な可変アクセスを配っている型

`I420Buffer` / `NV12Buffer` は `Send` で、`data_mut` / `y_data_mut` / `planes_mut` などの safe な可変アクセサを持つ。`I420Buffer` 自身は `Clone` を持たないが、同じ C++ 実体を指す所有ハンドルを複数作る経路が safe API にある。

- `I420Buffer::cast_to_video_frame_buffer` (`src/api/video_codec_common.rs`) は `&self` から同じ実体を指す `VideoFrameBuffer` を返す
- `VideoFrameBuffer::as_i420` / `as_nv12` (同) は `&self` から同じ実体を指す `I420Buffer` / `NV12Buffer` を返す
- `VideoFrameBuffer` は `Clone` と `Send` を持つ
- `VideoFrame::buffer` / `VideoFrameRef::buffer` (同) も同じ画素実体へのハンドルを返す。`VideoFrame::clone` がコピーするのは frame オブジェクトであり、その `scoped_refptr` が指す画素バッファは共有される

slice を返すアクセサ以外に、`I420Buffer::scale_from` / `NV12Buffer::crop_and_scale_from` (同) も safe な画素書き込み操作である。別ハンドルの `data()` などによる読み取りと競合できるため、書き手同士だけでなく読み取りと書き込みの排他も必要になる。

このため次のコードが safe で通り、同じ画素メモリへの可変アクセスが 2 本できる。これは問題の到達経路を示すコードであり、未修正の実装で実行する回帰テストにはしない。

```rust
let mut a = I420Buffer::new(640, 480);
let vfb = a.cast_to_video_frame_buffer();
let mut b = vfb.as_i420().expect("i420");
std::thread::spawn(move || { b.data_mut()[0] = 1; });
a.data_mut()[0] = 2;
```

並行させなくても、同一スレッドで `&mut [u8]` を 2 本同時に生かせる時点で Rust の参照規則に反する。

libwebrtc の可変アクセサ自体は唯一性を検査しない。

- `api/video/i420_buffer.cc` の `I420Buffer::MutableDataY` は `DataY` を `const_cast` するだけである
- `api/video/video_frame_buffer.h` の `VideoFrameBuffer` は `RefCountInterface` を継承し、唯一性を問い合わせる公開 API を持たない

Rust 側では `&mut self` を取っても他の所有ハンドルや native 側のアクセスを排除できないため、safe な可変アクセスの根拠にはならない。

`issues/closed/0111-add-clone-for-refcounted-wrappers.md` では I420 / NV12 の `Clone` 追加を本 issue の安全化まで保留している。競合は `Clone` を追加しなくても到達するため、本 issue では既存の共有経路を修正する。

### (2) 呼び出しの直列性を限定して扱うハンドラ

`Send` のみを要求し `&mut self` で呼ぶ設計は、同じハンドラへの呼び出しが重ならない場合に限り成立する。以下の native 側の根拠は、個々の登録先・インスタンスの通常経路についてのものである。

- `PeerConnectionObserverHandler` (`src/api/peer_connection.rs`): `api/peer_connection_interface.h` の `signaling_thread()` は、その PeerConnection の observer callback が signaling thread 上で呼ばれることを示す
- `DtlsTransportObserverHandler` (`src/api/dtls_transport.rs`): `api/dtls_transport_interface.h` は、例外として明記された操作以外は network thread 上でアクセスする契約を持つ
- `VideoEncoderHandler` / `VideoDecoderHandler` (`src/api/video_encoder.rs` / `src/api/video_decoder.rs`): `video/video_stream_encoder.cc` は専用の `encoder_queue_` 上で `EncodeVideoFrame` / `SetEncoderRates` から encoder を呼び、`video/video_receive_stream2.cc` の `DecodeAndMaybeDispatchEncodedFrame` は `RTC_DCHECK_RUN_ON(&decode_sequence_checker_)` を持つ
- `DataChannelObserverHandler` (`src/api/data_channel.rs`): `api/data_channel_interface.h` の `IsOkToCallOnTheNetworkThread` の既定値 `false` では、その PeerConnection に属する signaling thread へ通知される。C ラッパーはこの既定値を変更しない

observer を受け取る `PeerConnectionDependencies::new` / `DataChannel::register_observer` / `DtlsTransport::register_observer` は safe な借用を受け取り、同じ observer の複数登録を禁止していない。異なる登録先の callback を直列にする保証にはならない。0104 で扱う登録 API の安全化と、同じ handler への重複 callback を排除する契約が必要である。

`VideoEncoder` / `VideoDecoder` は一意所有で `Clone` / `Sync` を持たず、公開の状態変更操作は `&mut self` を取る。native の通常の codec 呼び出し経路と合わせて根拠を記載する。これを共有 audio factory の安全性に一般化しない。

### (3) 同時呼び出しがあり得ることを確認したハンドラ

- `LogSinkHandler` (`src/rtc_base/logging.rs`): `rtc_base/logging.cc` の `LogMessage::~LogMessage` は、`config.sinks()` のループではログロックを取らずに `sink->OnLogMessage(log_line_)` を呼ぶ (直後の legacy な `streams_` のループは `MutexLock lock(&GetLoggingLock())` を取る)。crate の `LogSink` は `LoggingConfig::AddSink` で `config.sinks_` に入り、`log::initialize_logging` でグローバルに設定される。ログはどのスレッドからでも出るため、同じ sink が同時に呼ばれ得る
- `VideoFrameBufferHandler` (`src/api/video_codec_common.rs`): `VideoFrameBuffer` は `Clone + Send` であり、C ラッパーの `VideoFrameBufferImpl` は Rust callback を直接呼ぶ。`to_i420` / `crop_and_scale` だけでなく、safe な `kind` / `width` / `height` でも trampoline が `&mut VideoFrameBufferHandlerState` を作るため、読み取り同士でも競合する
- `AudioEncoderFactoryHandler` / `AudioDecoderFactoryHandler` (`src/api/audio.rs`): factory は `Clone + Send` であり、`get_supported_encoders` / `get_supported_decoders` / `query_audio_encoder` / `create` などを別ハンドルから safe に呼べる。C ラッパーの `AudioEncoderFactoryImpl` / `AudioDecoderFactoryImpl` は callback を直接呼び、trampoline が同じ state に `&mut` を作る。1 stream の worker thread の直列性では共有 factory を保護できない

映像 encoder / decoder factory は Rust の所有型が一意でも、native の `WebRtcVideoEngine` が同じ factory を複数の送受信 stream に渡す。各 stream の encoder queue / decode queue が同じ factory の `Create` を呼ぶため、factory handler の直列性は保証されない。`GetSupportedFormats` と `Create` の両 callback が同じ可変 state を使うので、`VideoEncoderFactoryHandler` / `VideoDecoderFactoryHandler` も対象である。codec 本体の一意所有と factory の native 内部共有を区別する。

SDP 完了 observer も、生成・設定 API の借用が戻った後に native が参照カウントで保持し、非同期に通知する。同じ `CreateSessionDescriptionObserver` / `SetLocalDescriptionObserver` / `SetRemoteDescriptionObserver` を異なる signaling thread の PeerConnection に再利用できるため、PC ごとの直列性は共有 handler を保護しない。3 系統の callback も共有参照と同期可能な handler に揃える。

LogSink の `handler_state` も `&mut LogSinkHandlerState` を返す。ハンドラ内部で Mutex を取るだけでは、ロックを取る前に trampoline が重複する排他参照を作る問題を防げない。

カスタム映像バッファの debug 用 `callback_thread` は通常の `Option<ThreadId>` であり、trampoline が `&mut` を作った後に読み書きされる。release には存在せず、直列のスレッド移動まで拒否するため、排他の保証には使えない。

### (4) 同じ sink / 完了 callback を複数の登録先で使えるハンドラ

`VideoSink` / `AudioTrackSink` の登録 API は `&self` と `&sink` を取るため、同じ sink を複数の track に登録できる。登録先が別スレッドで配信する場合は同時呼び出しになる。

- `VideoBroadcaster::OnFrame` / `OnDiscardedFrame` (`api/video/video_broadcaster.cc`) は broadcaster ごとの `sinks_and_wants_lock_` の下で sink を呼ぶ
- `RemoteAudioSource::OnData` (`pc/remote_audio_source.cc`) は source ごとの `sink_lock_` の下で audio sink を呼ぶ

これらのロックは登録先をまたぐ排他を保証しない。現在の `VideoSinkHandler` / `AudioTrackSinkHandler` は `Send` と `&mut self` を使い、trampoline も可変 state を作るため、複数登録は本 issue の必須修正対象である。

`VideoEncoderEncodedImageCallbackHandler` (`src/api/video_encoder.rs`) も `Send` と `&mut self` を使い、`video_encoder_encoded_image_callback_on_encoded_image` は可変 state を作る。safe な `VideoEncoder::register_encode_complete_callback` は `VideoEncoderEncodedImageCallbackRefMut<'_>` を受け取るが、借用は登録呼び出しだけで終わり、同じ callback を複数の encoder に逐次登録できる。

`webrtc/src/webrtc_c/api/video_codecs/video_encoder.cc` の `webrtc_VideoEncoder_RegisterEncodeCompleteCallback` は非所有の callback ポインタを native encoder へ渡す。例えば `modules/video_coding/codecs/vp8/libvpx_vp8_encoder.cc` の `LibvpxVp8Encoder::RegisterEncodeCompleteCallback` は `encoded_complete_callback_ = callback;` で保存し、エンコード時に `OnEncodedImage` を呼ぶ。各 encoder が一意所有でも、別スレッドに移した 2 個の encoder から同じ完了 callback へ通知できるため、encoder 本体の直列性だけではこの handler の排他を保証できない。

### (5) `Send` の根拠を確認する型

- `SdpAudioFormatRef<'a>` (`src/api/audio.rs`) は `Send` を宣言し、所有型 `SdpAudioFormat` は `Sync` を実装していない。ただし、それだけでデータ競合になるわけではない。借用型の公開操作は const な読み取りとコピーであり、借用中は元の値を可変で借りられない。現行 safe API から書き込みと競合する到達経路は確認できていない
- `RawBufferWriter<'a, T>` (`src/util.rs`) は `T: Send` を要求しない無条件の `Send` である。`write` は既に `unsafe fn` だが、排他借用を別スレッドへ移す型として `Send` の境界を合わせる必要がある。現状の実利用は `i16` のみであり、これによる safe API からの競合は確認できていない

### (6) LoggingConfig 初期化と native ログの競合

`log::initialize_logging` (`src/rtc_base/logging.rs`) は現在 safe である。libwebrtc の `InitializeLogging` (`rtc_base/logging.cc`) は `GetOrInitConfig` で config を公開した後、`LogTimestamps` / `SetLogQueueNames` / `SetLogToStderr` で非 atomic な static bool を更新する。一方、`LogMessage` のコンストラクタと `OutputToDebug` はこれらを同じロックなしに読む。

公式の `g3doc/logging/initialization.md` は、他の WebRTC API 呼び出しとログ出力より前の初期化を要求する。Rust の `initialize_logging` だけを Mutex で囲んでも、native 内部のログ出力を止められないため、この初期化順序を safe API の内部で保証したことにはならない。

設定が先行するログ出力で確定した場合や、既に初期化済みの場合の `false` は native の仕様である。これと、初回の設定適用中に別スレッドからログを出す競合は区別する。

### (7) 共有 ADM の直接操作と native 呼び出し

`AudioDeviceModule` (`src/api/audio_device_module.rs`) は `Clone` を持つが `Send` / `Sync` を持たない。一方、safe な `PeerConnectionFactoryDependencies::set_audio_device_module` (`src/api/peer_connection.rs`) は同じ ADM を `scoped_refptr` で保存し、dependencies 自体は `Send` である。元の ADM を生成スレッドに残しても、dependencies を渡した先の native worker は同じ実体を呼べる。同じ ADM を 2 個の dependencies に設定し、異なる factory から使うことも safe API では禁止されていない。

`pc/connection_context.cc` の `ConnectionContext::AddRefMediaEngine` は worker thread 上で `media_engine_->Init()` を呼ぶ。`media/engine/webrtc_voice_engine.cc` の `WebRtcVoiceEngine::Init` は `adm_helpers::Init(adm())` と `RegisterAudioCallback` を呼び、`media/engine/adm_helpers.cc` の `Init` は `adm->Init()` などを呼ぶ。これらと元のハンドルの safe な `init` / `recording_devices` / `recording_device_name` / `set_recording_device` が重なると、同じ `adm_state` が排他参照を重複生成する。`AudioDeviceModuleHandler` の `Send` と `&mut self` は、1 個の native 利用者の直列呼び出しだけを根拠にしている。

未登録のカスタム ADM でも、handler が同じ実体の別ハンドルへ再入すれば排他参照は重なる。例えば `recording_devices(&self)` の callback が thread-local に保持した同じ ADM の `recording_devices` を再度呼ぶ場合、Rust の共有借用は通るが、両 trampoline は `&mut AudioDeviceModuleHandlerState` を作る。`&mut self` を取る直接操作も、`Clone` で作った別ハンドル経由の再入を防げない。

`issues/closed/0104-change-adm-handler-bounds.md` は native 利用中の直接操作を Rustdoc で禁止する方針を採っているが、safe API からの到達を防いでいない。本 issue では通常の native 呼び出しの直列性と、共有・直接操作・再入の排他契約を区別して安全化する。

### 対象外 (調査の結果 safe API からデータ競合に到達しないと確認したもの)

- `BufferRef` / `BufferRefMut` / `BufferS16Ref` / `BufferS16RefMut` (`src/rtc_base/buffer.rs`): 借用が生きている間は元の `Buffer` を可変で借りられないため、別スレッドの読み書きと競合しない
- `VideoFrame` の metadata (`src/api/video_codec_common.rs`): frame オブジェクトは一意所有され、コピーしても metadata は別実体になる。ただし画素バッファの共有は (1) の対象であり、`VideoFrame` 全体を対象外にはしない
- `EncodedImage` / `EncodedImageBuffer` (同): image の metadata は一意所有される。`EncodedImageBuffer::clone` は画素ではなく符号化済みデータを共有するが、公開 `data()` は読み取り専用で、safe なデータ書き込み API はない

## 設計方針

共有実体への書き込みと、直列性を内部で保証できない ADM の利用は `unsafe fn` の排他契約とし、並行 callback を許す handler は `Send + Sync` と共有参照で扱う。既存の `Send` と `Clone` は維持する。handler が可変状態を持つ場合は、実装側で Mutex や atomic などにより保護する。

libwebrtc の独自パッチ、未取り込み変更の backport、`make_ref_counted` の内部型に依存する唯一性判定は採らない。既存の LoggingConfig 経路を使い、legacy な `AddLogToStream` 経路へ切り替えない。

### (1) 共有ハンドルからの可変アクセス

- `I420Buffer` の `y_data_mut` / `u_data_mut` / `v_data_mut` / `data_mut` / `planes_mut` / `scale_from` を `unsafe fn` にする
- `NV12Buffer` の `y_data_mut` / `uv_data_mut` / `data_mut` / `planes_mut` / `crop_and_scale_from` を `unsafe fn` にする
- `# Safety` に、操作中、または返した可変 slice が生きている間、書き込み先または返却領域に重なる他の Rust 参照が生存しないことを要求する。stride の余白と読み取り専用 slice も含む。返した slice からの再借用と、その借用期間内の由来ポインタの同期利用は許可する。それ以外のハンドル・native 利用者による読み書きを排除する。cast、frame 内のバッファ、codec や source へ渡したハンドルも含む。`&mut self` だけではこの条件を満たさないことを説明する
- テスト・サンプルの呼び出し箇所では、書き込み中に別ハンドルや native 利用者がアクセスしない根拠を示す。`unsafe {}` を機械的に足すだけで済ませない

読み取り API と既存のハンドル共有は維持する。新しい builder 型、`Sync`、I420 / NV12 の `Clone` 追加は本 issue の必須変更に含めない。

カスタム `VideoFrameBufferHandler` の安全化は (3) で行う。C++ の `ToI420` / `CropAndScale` が非 const という事実だけを理由に、Rust の同名操作を一律に unsafe にする方針は採らない。

### (2) 直列性を前提にするハンドラと 0104 への依存

- DTLS observer の登録・解除は owning network thread の排他を必要とする。両操作を unsafe とし、callback 実行中の解除を禁止する
- `PeerConnectionObserverHandler` / `DtlsTransportObserverHandler` / `DataChannelObserverHandler` は `Send` と `&mut self` を維持する。Rustdoc には登録先ごとの呼び出しスレッドと、複数登録先をまたぐ直列性までは保証されないことを書く
- `PeerConnectionDependencies::new` / `DataChannel::register_observer` / `DtlsTransport::register_observer` は、0104 での生存期間の安全化に加え、同じ handler への callback が同時または再入で重ならない契約を必要とする。unsafe 登録 API の `# Safety` に、1 observer を 1 登録先で使い、callback 中の操作で同じ handler に再入させない条件を記載する
- 0104 が登録を型で保証する設計を採る場合も、寿命だけでなく上記の重複 callback を排除することを確認する。safe な登録 API に使用上の注意を書くだけでは完了条件を満たさない
- `VideoEncoderHandler` / `VideoDecoderHandler` は現在の `Send` と `&mut self` を維持し、確認した通常経路の直列性を Rustdoc に記載する

登録 API の生存期間の変更は 0104 が担当し、本 issue は重複 callback の排除条件を担当する。LogSink の先行対応には 0104 の完了を要求しないが、observer の経路を含めた本 issue 全体の完了には登録 API の安全化が必要である。

### (3) 同時呼び出しがあり得るハンドラ

- `LogSinkHandler` は `Send + Sync` とし、`on_log_message` を `&self` にする。`handler_state` と `log_sink_on_log_line_ref` も共有参照を使い、callback の入口で可変 state を作らない
- `VideoFrameBufferHandler` は `Send + Sync` とし、`to_i420` / `crop_and_scale` を含む全メソッドを `&self` にする。`kind` / `width` / `height` を含む全 trampoline を共有参照へ変更する。`callback_thread` と thread 固定チェックは撤去する
- `AudioEncoderFactoryHandler` / `AudioDecoderFactoryHandler` / `VideoEncoderFactoryHandler` / `VideoDecoderFactoryHandler` は `Send + Sync` とし、全メソッドを `&self` にする。対応する全 trampoline を共有参照へ変更する
- SDP 完了 observer の 3 handler は `Send + Sync` と `&self` にし、成功・失敗・完了の全 trampoline を共有参照へ変更する
- 各 handler の利用箇所を新しい trait に合わせる。並行して呼ばれる契約を Rustdoc に書き、可変状態の同期は handler 実装が担当する

通常 callback の入口で state 自体を可変借用せず、handler 内で必要な状態だけを保護する。最終破棄時に Box の所有権を回収する処理とは区別する。`VideoFrameBuffer::as_native_mut` は unsafe のままとし、返した可変参照の有効期間全体で他の参照と callback を排除する契約を維持する。

### (4) 複数登録できる sink / 完了 callback

- `VideoSinkHandler` / `AudioTrackSinkHandler` / `VideoEncoderEncodedImageCallbackHandler` は `Send + Sync` とし、全メソッドを `&self` にする。対応する全 trampoline を共有参照へ変更する
- 同じ sink / 完了 callback が複数の登録先から同時に呼ばれても、Rust の排他参照を重複生成しない設計にする。1 sink / callback につき 1 登録先という文書化だけで解決したことにしない
- video / audio sink の登録中の生存期間と解除後の破棄の契約は 0104 で扱う。handler の共有参照化では use-after-free は解消しない

エンコード完了 callback についても、共有参照化が保証するのは callback が生存している間の重複通知の安全性である。非所有登録の生存期間の安全化はデータ競合とは別の問題として扱い、共有参照化だけで解消したとはしない。

### (5) `Send` の宣言

- `SdpAudioFormatRef` の `Send` は維持し、const な読み取りだけを公開し、借用中は元の値への可変アクセスを排除することを根拠としてコメントを書く。所有型の `!Sync` だけを理由に `Send` を外したり、所有型へ新たな `Sync` を追加したりしない
- `RawBufferWriter` の `Send` 実装に `T: Send` を要求する。現在の `i16` の利用は維持する

### (6) LoggingConfig 初期化

- `log::initialize_logging` を `unsafe fn` にする。初回の設定適用は他の WebRTC API とログ出力の開始前に行い、呼び出し中は他スレッドの WebRTC 利用や native ログ出力を排除することを `# Safety` に記載する
- 呼び出し側のテスト・サンプルは、上記の初期化順序と排他の根拠を示す。初期化だけの Rust Mutex を安全性の根拠にしない
- 初期化が遅かった場合や再初期化の場合に `false` を返し、渡した config が採用されない native の仕様は維持する。その通常動作の検証も並行した WebRTC 利用を止めた独立プロセスで行う

ログ severity の設定・フィルタリング機能、`log::print` の文字列の扱い、ログの付加情報を増やす変更は、本 issue のデータ競合修正には含めない。上流のログレベル修正を待つことと、現在の API の安全性を直すことを混同しない。

### (7) ADM の共有と直接操作

- `PeerConnectionFactoryDependencies::set_audio_device_module` を `unsafe fn` にする。`# Safety` には、dependencies への設定時から native 利用の終了まで、同じ ADM の callback が他の factory・元のハンドル・clone からの操作と同時または再入で重ならないことを要求する。異なる worker thread の factory 間で共有しないことも明記する
- `AudioDeviceModule::init` / `recording_devices` / `recording_device_name` / `set_recording_device` を `unsafe fn` にする。呼び出し中は同じ実体への他の直接操作・native 呼び出し・callback からの再入が無いことを `# Safety` に要求する。getter も可変 handler を呼ぶため対象に含める
- `AudioDeviceModuleHandler` の `Send` と `&mut self`、ADM の `Clone` と `!Send` / `!Sync` は維持する。通常の native 呼び出しの直列性を、共有ハンドルの直接操作まで safe にする根拠として使わない
- 既存のテスト・サンプルでは、factory への引き渡し前の直接操作と引き渡し後の native 利用を区別し、同時・再入 callback を排除する根拠を示す。unsafe 化対象を safe に呼べないことも確認し、未修正の競合や排他参照の重複を実行して検証しない

### 対応順

1. LogSink の trait・trampoline と LoggingConfig 初期化を安全化し、利用箇所とテストを更新する。この段階で SDK が安全な LoggingConfig 経路へ接続できる
2. I420 / NV12 の全画素書き込み操作と、カスタム VideoFrameBuffer の全 callback を安全化する
3. 共有 audio / video factory、複数登録できる video / audio sink、エンコード完了 callback、SDP 完了 observer を安全化する
4. ADM の共有・直接操作の排他契約と `Send` の根拠を反映し、0104 と組み合わせた observer 登録の排他条件を確認する

各段階で必要な公開 API の変更と検証を行い、全段階の完了をもって本 issue を完了とする。

## 完了条件

- I420 / NV12 の可変 slice と `scale_from` / `crop_and_scale_from` が safe API から呼べず、書き込み先または返却領域に重なる他の Rust 参照の生存禁止と、別ハンドル・native 利用者による読み書きの排他契約が `# Safety` に記載されていること。返却 slice からの有効な再借用を許可し、cast と VideoFrame 経由の共有も含むこと
- `LogSinkHandler` / `VideoFrameBufferHandler` / `AudioEncoderFactoryHandler` / `AudioDecoderFactoryHandler` / `VideoSinkHandler` / `AudioTrackSinkHandler` / `VideoEncoderEncodedImageCallbackHandler` / `VideoEncoderFactoryHandler` / `VideoDecoderFactoryHandler` と SDP 完了 observer の 3 handler が `Send + Sync` と `&self` を使い、通常 callback の trampoline も共有参照を使うこと
- カスタム VideoFrameBuffer の getter を含む callback に可変 state の生成が残らず、debug / release の両方で並行アクセスと直列のスレッド移動を扱えること
- `initialize_logging` が unsafe となり、初期化順序と並行 native ログ出力の排除契約、初期化失敗時の `false` の意味が記載されていること
- 上記の公開 API に合わせて既存の handler 実装、テスト、サンプルが更新され、unsafe 呼び出しの排他条件が確認できること
- 実物の C ラッパーを通じ、複数スレッドの LogSink、clone した映像バッファと audio factory、native に共有される video factory、複数登録先の sink / エンコード完了 callback、異なる signaling thread の SDP 完了 observer を検証すること。モックやスタブ、未修正の UB を実行するテストは使わないこと
- logging のグローバル初期化の検証は独立プロセスで行い、初期適用・再適用の結果と複数スレッドからのログ受信を検証すること
- 直列性を前提にする handler の Rustdoc が保証の範囲を示し、observer の登録 API で同時・再入 callback が排除されていること。0104 の登録 API の安全化が未完了なら、本 issue 全体を完了としないこと
- `SdpAudioFormatRef` の readonly な `Send` の根拠がコメントに記載され、`RawBufferWriter` の `Send` が `T: Send` に限定されていること
- ADM の factory 引き渡しと 4 個の直接操作が unsafe となり、native 利用者・他のハンドル・callback からの同時・再入を排除する契約が記載されていること。`AudioDeviceModuleHandler` は `Send` と `&mut self` を維持し、通常の直列呼び出しを実物の C ラッパーで検証すること
- `unsafe impl Send` / `Sync` を追加・変更した箇所には、その根拠がコメントで書かれていること
- `src/tests.rs` の既存テストがパスすること
- `cargo fmt --all -- --check` / `cargo clippy --workspace --features source-build --all-targets -- -D warnings` / `cargo test --workspace --features source-build` が通ること
- 公開 API の変更を伴うため `CHANGES.md` の `## develop` に `[CHANGE]` が記載されていること

## 解決方法

### 共有 callback と排他契約

- ログ、カスタム映像バッファ、音声・映像 factory、映像・音声 sink、エンコード完了 callback、SDP 完了 observer の計 12 系統を `Send + Sync` と `&self` に変更した。通常の trampoline は共有参照を使い、最終破棄時の所有権回収と区別した。カスタム映像バッファのスレッド固定状態と検査も削除し、handler state を既存の共通型へ揃えた
- `src/api/video_codec_common.rs` の I420 / NV12 の全画素書き込み API を unsafe にし、書き込み先または返却領域に重なる Rust 参照の生存禁止と、cast・VideoFrame・native 利用者を含む排他条件を記載した。stride の余白を含む返却領域の有効期間全体が対象で、返却 slice からの有効な再借用と由来ポインタの同期利用は許可する。`as_native_mut` は返却参照が生存する全期間の排他を要求する
- `src/rtc_base/logging.rs` の `initialize_logging` を unsafe にし、WebRTC 利用・ログ出力の開始前の初期適用と、再適用を含む呼び出し中の全体排他を要求した。遅い初期化・再適用が `false` を返す動作は維持した
- `src/api/audio_device_module.rs` の 4 個の直接操作と、`PeerConnectionFactoryDependencies::set_audio_device_module` を unsafe にした。ハンドル・clone・factory・native・callback からの同時操作と再入を排除し、異なる worker thread の factory 間での共有を禁止した
- `PeerConnectionDependencies::new`、`DataChannel::register_observer`、`DtlsTransport::register_observer` に、observer の生存期間、1 登録先だけでの利用、同時・再入 callback の排除を要求する unsafe 契約を設けた。DTLS の登録・解除は owning network thread で行う unsafe 操作とした。PC observer の保持期間は dependencies の生存中と、生成した PC の close 復帰または全ハンドル破棄までに揃えた
- `src/util.rs` の `RawBufferWriter` の `Send` を `T: Send` に限定し、`SdpAudioFormatRef` の readonly な `Send` の根拠を記載した。codec 本体と ADM の直列 handler は `Send` と `&mut self` を維持した

### 利用箇所と native 通知入口

- 既存テストと WHIP / WHEP の handler を新しい trait に合わせ、unsafe 呼び出しの排他・生存期間の根拠を記載した
- `README.md` と `skills/shiguredo-webrtc/SKILL.md` の factory 生成例を unsafe な ADM 引き渡しへ追従させ、掲載コードのコンパイルを確認した
- WHIP / WHEP は接続中の再 connect を拒否し、Drop で factory・native thread が生存する間に PC と observer を破棄する。WHEP は close の復帰後に sink と remote track を解除するため、保留中の OnTrack による再保持を排除する
- `webrtc/src/webrtc_c/api/media_stream_interface.h` / `.cc` に、public virtual method を直接転送する `webrtc_AudioTrackSinkInterface_OnData` を追加した。実際に 2 個の remote audio track へ登録した sink の並行通知を検証できる
- `CHANGES.md` に `[CHANGE]`、`[ADD]`、`[FIX]` を記載した。`webrtc/src/webrtc_c/rtc_base/logging.h` は C の整形チェックで検出した macro の空白・改行だけを整えた

### 検証

- 実物の C ラッパーを使い、ログの並行通知、映像バッファの全 callback の重複とスレッド移動、排他的書き込み後の共有画素読み取り、実 Opus factory の並行利用、2 映像 source への sink 登録、2 encoder への完了 callback 登録、映像 factory の全通知入口、2 remote audio track への sink 登録、異なる signaling thread の 2 PC による SDP observer の成功・失敗・完了通知を検証した
- SDP の通知は操作ごとに両 PC の通知完了を待ってから次へ進める。各 callback は有限の待ち合わせを使い、両通知がタイムアウトせず重なったことを検証する
- 音声 sink とエンコード完了 callback の重複通知は、実登録を行ったうえで public C 通知入口を直接呼ぶ検証である。RTP の受信・デコードや実圧縮による通知の全経路を検証したとはしない
- WHIP / WHEP の再 connect と自動 Drop、実 remote video track の保持から WHEP の Closed 通知・sink 解除までの順序を検証した。モック・スタブ・未修正 UB の実行は用いていない
- logging の初期適用・暗黙初期化後の拒否・再適用と並行通知は独立プロセスで検証した
- 新しい 21 個の unsafe API と `RawBufferWriter` の Send 制約に、22 個の compile-fail テストを追加した
- `cargo test --workspace --features source-build` と同じコマンドの `--release` は、それぞれ 240 件成功した。内訳は lib 170 件、libyuv 35 件、WHEP 2 件、WHIP 1 件、通常 doctest 2 件、compile-fail doctest 30 件である
- `cargo fmt --all -- --check`、`cargo clippy --workspace --features source-build --all-targets -- -D warnings`、`python3 webrtc/run.py format --check`、`git diff --check` が通過した

### 保証の範囲

本 issue が依存する 3 系統の observer 登録は、上記の unsafe 契約で安全化した。0104 にあるその他の非所有登録の生存期間は、共有 handler の変更だけで解消したとはしない。エンコード完了 callback についても、生存中の重複通知の安全性と、登録先より先に破棄しない責務は別である。
