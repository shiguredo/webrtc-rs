# 借用型 `XxxRef` を読み取り専用にし `XxxRefMut` を新設する

- Created: 2026-09-16
- Completed: 2026-09-18
- Branch: feature/refactor-readonly-ref-types
- Polished: {YYYY-MM-DD}

## 目的

`XxxRef` 型は C++ オブジェクトを借用するハンドルだが、現在は借用先を書き換える可変アクセサを持っている。このため所有していないオブジェクトを借用ハンドル越しに書き換えられてしまう。

可変アクセサを持つハンドルを `Copy` にすると `&mut` の排他性を型で保証できず、`let mut a = r; let mut b = r;` で同一オブジェクトへの可変ハンドルが複数できてしまう。`XxxRef` はすべて `unsafe impl Send` なので、複製したハンドルを別スレッドへ送れば同一オブジェクトを同時に書き換えられる。可変アクセサと `Copy` は同じ型に同居させられない。

一方で、借用先を直接書き換える操作は必要になる。`SdpVideoFormat::parameters_mut` が返す `MapStringString` の `set` のように、コピーを挟まず参照先を書き換えたい場面がある。所有型経由に一本化するとコピーが挟まり、借用先の直接書き換えが表現できなくなる。

そこで借用ハンドルを「読み取り専用で `Copy` な `XxxRef`」と「書き換え可能で `Copy` でない `XxxRefMut`」に分ける。可変アクセサを持つ型を `Copy` にしないことで排他性を型で保証し、借用先の直接書き換えも維持する。

## 現状

### 可変アクセサを持つ借用ハンドル

`XxxRef` は 39 個あり、うち 24 個に `&mut self` を取るメソッドが合計 103 個ある。

- `RtpEncodingParametersRef`: `set_rid` / `set_ssrc` / `set_max_bitrate_bps` / `set_min_bitrate_bps` / `set_max_framerate` / `set_scale_resolution_down_by` / `set_scale_resolution_down_to` / `set_active` / `set_adaptive_ptime` / `set_scalability_mode` / `set_codec` / `set_bitrate_priority` / `set_network_priority` / `set_request_key_frame` / `set_num_temporal_layers` (15 個)
- `CodecSpecificInfoRef`: `set_codec_type` / `set_end_of_picture` / VP8, VP9, H264 の各フィールドセッター (19 個)
- `IceServerRef`: `add_url` / `set_username` / `set_password` / `set_tls_cert_policy` / `set_tls_client_identity` (5 個)
- `SimulcastStreamRef`: `set_width` / `set_height` / `set_min_bitrate_kbps` / `set_target_bitrate_kbps` / `set_max_bitrate_kbps` (5 個)
- `VideoCodecRef`: `set_codec_type` / `set_width` / `set_height` / 各ビットレートセッターなど (9 個)
- `EncodedImageRef`: `set_encoded_data` / `set_rtp_timestamp` / `set_encoded_width` / `set_encoded_height` / `set_frame_type` / `set_qp` (6 個)
- `RtpCodecRef`: `set_kind` / `set_name` / `set_clock_rate` / `set_num_channels` / `parameters` (5 個)
- `RtpCodecCapabilityRef`: `set_kind` / `set_name` / `set_clock_rate` / `set_num_channels` / `parameters` (5 個)
- `IceServerVectorRef`: `push` (1 個)
- `RtpCodecCapabilityVectorRef`: `push` / `resize` / `set` (3 個)
- `VideoFrameTypeVectorRef`: `push` (1 個)
- `VideoEncoderFramerateFractionInlinedVectorRef`: `push` / `set` / `resize` / `clear` (4 個)
- `VideoFrameBufferKindInlinedVectorRef`: `push` / `set` / `resize` / `clear` (4 個)
- `VideoEncoderResolutionBitrateLimitsVectorRef`: `push` / `set` / `clear` (3 個)
- `VideoEncoderQpThresholdsRef`: `set_low` / `set_high` (2 個)
- `VideoEncoderScalingSettingsRef`: `set_thresholds` / `set_min_pixels_per_frame` (2 個)
- `VideoEncoderResolutionBitrateLimitsRef`: `set_frame_size_pixels` / `set_min_start_bitrate_bps` / `set_min_bitrate_bps` / `set_max_bitrate_bps` (4 個)
- `VideoEncoderResolutionRef`: `set_width` / `set_height` (2 個)
- `SdpAudioFormatRef`: `parameters_mut` (1 個)
- `SdpVideoFormatRef`: `parameters_mut` (1 個)
- `BufferRef`: `append_data` / `clear` (2 個)
- `BufferS16Ref`: `append_data` / `clear` (2 個)
- `CxxStringRef`: `append` (1 個)
- `StringVectorRef`: `push` (1 個)

なお `RtpCodecRef::parameters` と `RtpCodecCapabilityRef::parameters` は `&mut self` を要求して `MapStringString<'a>` を返す。`&self` から取れる `parameters` ではないため、借用先を書き換えるには呼び出し元が可変参照を持つ必要がある。

### `&self` から可変ハンドルを取得できる経路

`&self` を取るメソッドが可変アクセサ付きの借用ハンドルを返しているため、共有参照しか持たない呼び出し元でも借用先を書き換えられる。

- `RtpCodecCapabilityRef::cast_to_codec` は `RtpCodecRef` を返す。C 側の `WEBRTC_DEFINE_CAST` は同一オブジェクトを `static_cast` して返すだけなので、返った `RtpCodecRef` のセッターは元の `RtpCodecCapability` を書き換える
- `RtpEncodingParametersRef::codec` は `Option<RtpCodecRef>` を返す。C API の `webrtc_RtpEncodingParameters_get_codec` は非 const の `struct webrtc_RtpCodec**` を出力する
- `RtpCapabilities::codecs` は `RtpCodecCapabilityVectorRef` を返し、その `push` / `resize` / `set` に到達できる
- `RTCConfiguration::servers` は `IceServerVectorRef` を返し、その `push` に到達できる
- `SdpAudioFormatRef::parameters_mut` / `SdpVideoFormatRef::parameters_mut` は `MapStringString` を返し、その `set` に到達できる

### `Copy` にできない借用ビュー

`MapStringString<'a>` は C++ の `std::map<std::string, std::string>` の借用ビューで、`set(&mut self)` を持つ。`Copy` にすると同一の map への可変ハンドルが複数できてしまう。

### 借用先にしか実体が無い型

`SimulcastStreamRef` へのセッターは C API の `webrtc_SimulcastStream_set_width` などにしか存在しない。`SimulcastStream` の所有型は無く、C API にも copy 関数が無い。`VideoCodec::simulcast_stream` は親の `VideoCodec` が持つ配列要素を `webrtc_VideoCodec_simulcast_stream_at` で借用して返している。

### 公開署名に残っている `&mut`

- `AudioEncoderHandler::encode` の `encoded: &mut BufferRef<'_>`
- `AudioDecoderHandler::generate_plc` の `concealment_audio: &mut BufferS16Ref<'_>`
- `AudioDecoder::generate_plc` の `concealment_audio: &mut BufferS16Ref<'_>`

いずれも C++ 側が作業用バッファを渡して中身を書かせる契約になっている。

### 重複している読み取りアクセサ

`XxxRef` と対応する owned 型 `Xxx` は同じ読み取りアクセサを持っており、owned 型側は `self.as_ref().xxx()` で委譲している。

### const 引数を受けるコールバックでの `cast_mut()`

C API は libwebrtc の C++ シグネチャに合わせ、読み取り専用の引数を `const struct ...*` で受ける。一方 Rust 側のコールバックは `XxxRef` を作るために `expect_non_null(ptr.cast_mut(), ...)` で const を外している。

- `src/api/audio.rs`: 7 箇所 (`AudioEncoderFactoryHandler::query_audio_encoder` / `create`、`AudioDecoderFactoryHandler::is_supported_decoder` / `create`、`AudioEncoderHandler::on_received_uplink_allocation`)
- `src/api/video_decoder.rs` / `src/api/video_encoder.rs`: 13 箇所 (`VideoDecoderHandler::configure` / `decode`、`VideoDecoderFactoryHandler::create`、`VideoEncoderHandler::init_encode` / `encode` / `set_rates`、`VideoEncoderEncodedImageCallbackHandler::on_encoded_image`、`VideoEncoderFactoryHandler::create`)

`cast_mut()` は型レベルの権限を戻すだけで C++ の `const_cast` と同じ意味を持つ。このため C++ 側が読み取り専用と宣言した引数を、次の借用型の可変アクセサ越しに書き換えられる状態になっている。

- `EncodedImageRef` (`VideoDecoderHandler::decode` / `VideoEncoderEncodedImageCallbackHandler::on_encoded_image`): `set_encoded_data` / `set_rtp_timestamp` など
- `CodecSpecificInfoRef` (`VideoEncoderEncodedImageCallbackHandler::on_encoded_image`): `set_codec_type` / `set_end_of_picture` など
- `VideoCodecRef` (`VideoEncoderHandler::init_encode`): `set_codec_type` / `set_width` など
- `VideoFrameTypeVectorRef` (`VideoEncoderHandler::encode`): `push`
- `SdpVideoFormatRef` (`VideoEncoderFactoryHandler::create` / `VideoDecoderFactoryHandler::create`): `parameters_mut`

なお `VideoDecoderSettingsRef` / `EnvironmentRef` / `VideoFrameRef` / `VideoEncoderSettingsRef` / `VideoEncoderRateControlParametersRef` は可変アクセサを持たないが、`XxxRef` が `*mut` を保持している限り同じく `cast_mut()` が必要になる。

## 設計方針

### 1. `XxxRef<'a>` を読み取り専用にする

可変アクセサをすべて外し、`#[derive(Clone, Copy)]` を付ける。手書きの `unsafe impl Send` は維持する。

`Deref` の実装で一時値への参照を返すために `Copy` であることが必要になるため、`Copy` は全型に付ける。

### 2. `XxxRefMut<'a>` を全 39 型に新設する

`PhantomData<&'a mut T>` を持ち、`XxxRef` から移した可変アクセサをすべてここに置く。`Copy` は付けない。取得経路は `&mut self` からだけにする。

`Copy` でないため `&mut self` の排他性が型で保証され、同一オブジェクトへの可変ハンドルが複数できる経路が無くなる。将来この型に可変アクセサを足しても排他性は壊れない。

### 3. `XxxRef` は const ポインタを、`XxxRefMut` は可変ポインタを保持する

`XxxRef<'a>` が保持するポインタを `*const ffi::T` にし、`as_ptr()` も `*const ffi::T` を返す。`XxxRefMut<'a>` は `*mut ffi::T` を保持し、`as_mut_ptr()` を返す。

- `XxxRef::from_raw` は `*const ffi::T`、`XxxRefMut::from_raw` は `*mut ffi::T` を受ける
- `NonNull<T>` は可変ポインタを表す型で `as_mut()` を持つため const を表現できず、const 側の保持には使えない。`XxxRef` は生ポインタを持つ
- `XxxRefMut` から `XxxRef` を作る `Deref` は const を付ける方向の変換になるため、const を外すキャストは不要になる

これにより、C 側のコールバックが受け取る `*const` から `cast_mut()` で const を外さずに `XxxRef` を作れるようになる。

### 4. 読み取りアクセサは `XxxRef` にだけ書く

`XxxRefMut<'a>` に `std::ops::Deref<Target = XxxRef<'a>>` を実装する。`XxxRef` が `Copy` なので `deref` は `&XxxRef::from_raw(self.raw.cast_const())` を返せる。これにより読み取りアクセサの二重管理が無くなる。

`DerefMut` は付けない。`XxxRef` に可変アクセサが無いため、`&mut XxxRef` を得ても読み取りしかできない。

`Deref` は標準ライブラリの既存トレイトであり、この実装は新規トレイトの作成には当たらない。同種の前例として `src/api/frame_transformer.rs` の `TransformableVideoFrame` が `TransformableFrame` に対して `std::ops::Deref` / `DerefMut` を実装している。

### 5. owned 型に `as_mut` を追加する

owned 型に `as_mut(&mut self) -> XxxRefMut<'_>` を追加し、`as_ref(&self) -> XxxRef<'_>` と対にする。owned 型の `&mut self` メソッド内部の `self.as_ref().set_xxx(...)` を `self.as_mut().set_xxx(...)` に置き換える。

### 6. `&self` から可変ハンドルを取得できる経路を塞ぐ

- `RtpCodecCapabilityRef::cast_to_codec` は読み取り専用の `RtpCodecRef` を返す形のまま残し、書き換え用の `RtpCodecCapabilityRefMut::cast_to_codec_mut` を新設する
- `RtpEncodingParametersRef::codec` は読み取り専用のまま残し、書き換え用の `RtpEncodingParametersRefMut::codec_mut` を新設する
- `RtpCapabilities::codecs` と `RTCConfiguration::servers` は `&self` の読み取りと `&mut self` の書き換えに分ける
- `SdpAudioFormatRef::parameters_mut` / `SdpVideoFormatRef::parameters_mut` は `XxxRefMut` 側へ移す
- `VideoCodec::simulcast_stream` は `&self` で `SimulcastStreamRef<'_>` を返し、書き換え用に `&mut self` で `SimulcastStreamRefMut<'_>` を返す `simulcast_stream_mut` を新設する。`SimulcastStream` の所有型や C API の copy 関数は追加しない
- `AudioEncoderHandler::encode` / `AudioDecoderHandler::generate_plc` / `AudioDecoder::generate_plc` の `&mut BufferRef<'_>` / `&mut BufferS16Ref<'_>` を `&mut BufferRefMut<'_>` / `&mut BufferS16RefMut<'_>` に変更する

### 7. `MapStringString` を `Ref` / `RefMut` に分ける

`MapStringString<'a>` を `MapStringStringRef<'a>`（読み取り専用、`Copy`）と `MapStringStringRefMut<'a>`（`set(&mut self)` を持ち `Copy` でない）に分ける。`src/lib.rs` の re-export も更新する。

### 8. `RawBufferWriter` は対象外

`RawBufferWriter<'a, T>` は書き込み位置を自分で持つライタで、対応する C++ オブジェクトが 1 対 1 で存在しない。`Ref` / `RefMut` に分けられないため `write(&mut self)` のまま維持する。

### 9. owned 型から `Ref` / `RefMut` への委譲は行わない

`Deref::deref(&self)` は `self` の中に実在する値を参照で返す必要があり、その場で組み立てたハンドルを返せない。owned 型に `Ref` をフィールドとして持たせれば可能だが、`NonNull<ffi::T_unique>` と `NonNull<ffi::T>` の 2 本持ちにする全 owned 型の所有表現の作り替えになり、`ScopedRef` を持つハンドル系の型には適用できない。読み取りアクセサ 1 行分の重複は受け入れる。

### 10. マクロと新規トレイトは作らない

規約に従い、マクロと新規トレイトは作らない。`Deref` のような既存トレイトへの実装だけを行う。

### 11. 変更履歴

外部 API の破壊的変更なので `CHANGES.md` に `[CHANGE]` として記載する。C API (`webrtc/`) の変更は無い。

## 完了条件

- 全 39 の `XxxRef` に可変アクセサが無く、`#[derive(Clone, Copy)]` が付いている
- 全 39 の `XxxRef` に対応する `XxxRefMut` があり、`Copy` ではなく、`std::ops::Deref<Target = XxxRef>` を実装している
- `&self` から可変アクセサ付きのハンドルを取得できる経路が残っていない
- `MapStringString` が `MapStringStringRef` / `MapStringStringRefMut` に分かれている
- `XxxRefMut` を返す `as_mut` または `_mut` 付きのアクセサからしか借用先を書き換えられない
- `XxxRef` が `*const` を、`XxxRefMut` が `*mut` を保持しており、const を外すための `cast_mut()` が `src/` に残っていない
- `cargo fmt --all -- --check` / `cargo clippy --workspace --features source-build -- -D warnings` / `cargo test --workspace --features source-build` が通る
- `CHANGES.md` に `[CHANGE]` として記載されている

## 対象型

`XxxRefMut` を新設する 39 型。既に `#[derive(Clone, Copy)]` 済みの型も対象に含める。

| モジュール | 型 |
|---|---|
| `src/cxxstd.rs` | `CxxStringRef`, `StringVectorRef` |
| `src/rtc_base/buffer.rs` | `BufferRef`, `BufferS16Ref` |
| `src/rtc_base/logging.rs` | `LogLineRef` |
| `src/rtc_base/ssl_certificate.rs` | `SSLCertificateRef`, `SSLCertChainRef` |
| `src/api/environment.rs` | `EnvironmentRef` |
| `src/api/audio_device_module.rs` | `AudioTransportRef` |
| `src/api/jsep.rs` | `IceCandidateRef` |
| `src/api/audio.rs` | `SdpAudioFormatRef` |
| `src/api/rtp.rs` | `RtpCodecRef`, `RtpCodecCapabilityRef`, `RtpCodecCapabilityVectorRef`, `RtpEncodingParametersRef` |
| `src/api/peer_connection.rs` | `NetworkManagerRef`, `PacketSocketFactoryRef`, `IceServerRef`, `IceServerVectorRef` |
| `src/api/video_codec_common.rs` | `SdpVideoFormatRef`, `VideoFrameRef`, `VideoFrameTypeVectorRef`, `SimulcastStreamRef`, `VideoCodecRef`, `EncodedImageRef` |
| `src/api/video_codec_specifics.rs` | `NaluInfoRef` |
| `src/api/video_decoder.rs` | `VideoDecoderSettingsRef`, `VideoDecoderDecodedImageCallbackRef` |
| `src/api/video_encoder.rs` | `VideoEncoderFramerateFractionInlinedVectorRef`, `VideoFrameBufferKindInlinedVectorRef`, `VideoEncoderQpThresholdsRef`, `VideoEncoderScalingSettingsRef`, `VideoEncoderResolutionBitrateLimitsRef`, `VideoEncoderResolutionBitrateLimitsVectorRef`, `VideoEncoderResolutionRef`, `VideoEncoderSettingsRef`, `VideoEncoderRateControlParametersRef`, `CodecSpecificInfoRef`, `VideoEncoderEncodedImageCallbackRef` |

併せて `src/cxxstd.rs` の `MapStringString` を `MapStringStringRef` / `MapStringStringRefMut` に分ける。

## 解決方法

- `XxxRef` から可変アクセサを外して `#[derive(Clone, Copy)]` を付け、可変アクセサは新設した `XxxRefMut` に移した
- `XxxRefMut` は `Copy` にせず、`std::ops::Deref<Target = XxxRef>` で `XxxRef` の読み取りアクセサを共有する（`Deref` は実在する `XxxRef` への参照を返す必要があるため、`cref` フィールドとして保持する）
- 借用ハンドルが保持するポインタを非 null 型に変え、`XxxRef` は `ConstNonNull`、`XxxRefMut` は `NonNull` を保持するようにした（`NonNull` は `*mut T` 用の API しか持たないため `ConstNonNull` を追加し、クレートルートから参照できるようにした）
- `XxxRef::from_raw` / `XxxRefMut::from_raw` / `CxxStringRef::from_ptr` を `pub(crate)` に限定し、`unsafe fn` と `fn` が混在していたのを安全関数に統一した（借用先の寿命を型で保証できず、外部に公開すると safe Rust から不正なハンドルを作れてしまうため）
- 所有権を受け取る `CxxString::from_unique` / `RtcError::from_unique_ptr` / `SdpParseError::from_unique_ptr` / `SessionDescription::from_unique_ptr` も同様に `pub(crate)` に限定した（`CxxString::into_raw` は譲渡方向なので public のまま。再監査で public かつ safe に所有権を取る関数はこの 4 つだけであることを確認した）
- `webrtc_c` に読み取り専用の借用を返す `_const` 版 getter・`_refcounted_get_const`・`WEBRTC_DECLARE_CAST_CONST` を追加し、`AddRef` / `Release` の引数を `const struct CType*` にした（`src/` から const を外すキャストを全廃した）
- 構築経路が無く未使用だった可変ハンドル（`NaluInfoRefMut` / `VideoDecoderSettingsRefMut` / `VideoEncoderSettingsRefMut` / `VideoEncoderRateControlParametersRefMut` / `SSLCertificateRefMut` / `SSLCertChainRefMut` / `LogLineRefMut` / `VideoDecoderDecodedImageCallbackRef` 系）を削除した
- 完了条件のうち 2 点は実装時に変わった
  - 「全 39 の `XxxRef` に対応する `XxxRefMut`」は、未使用の可変ハンドル 8 型を削除したため `XxxRefMut` が 32 型になった
  - 「`XxxRef` が `*const`、`XxxRefMut` が `*mut` を保持」は、非 null を型で表す `ConstNonNull` / `NonNull` を保持する形になった
- レビューで、`XxxRef` の `Copy` と `XxxRefMut` の `Deref` の組み合わせにより safe なコードで use-after-free を作れることが判明したため、`XxxRefMut` から `Deref` を削除した
  - `Deref::Target` は `XxxRef<'a>` に固定され、`deref()` が返す参照の中身が `'a` を持つため、`Copy` でその値を借用の外へ持ち出せる。持ち出したハンドルから得た借用を保持したまま `XxxRefMut` の書き換えメソッドを呼ぶと、C++ 側の再確保で解放された領域を読むことになる
  - 読み取りアクセサは `XxxRefMut` に同じシグネチャの転送メソッド（`self.cref.xxx()` の 1 行）として用意し、借用や借用ハンドルを返すものは戻り値を `'_` に短縮した。`XxxRefMut::as_ref(&self) -> XxxRef<'_>` も同じ規則に従う
  - `cref` は転送時に一時値を作らないために必要なので保持した。`Copy` / `Clone` は `XxxRef` に残しているため、利用側の書き換えは無い
- `CHANGES.md` の `## develop` 節に `[CHANGE]` と misc のエントリを追加した
- `cargo fmt --all -- --check` / `cargo clippy --workspace --features source-build -- -D warnings` / `cargo test --workspace --features source-build` / `prek run --files` の成功を確認した
