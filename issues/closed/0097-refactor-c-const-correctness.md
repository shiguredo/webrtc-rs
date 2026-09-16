# webrtc_c の C API の const 性を正しく設定する

- Created: 2026-08-30
- Completed: 2026-09-17
- Branch: feature/refactor-c-const-correctness
- Polished: {YYYY-MM-DD}

## 目的

webrtc_c の C API は、libwebrtc の C++ メソッドの const 性を正しく反映できていない箇所が多い。読み取り専用の getter が `const struct ...* self` になっていない、また `const` 引数を `const_cast` で外しているため、C++ 側の const 契約が崩れている。これを一括で是正する。

## 現状

### const_cast の使用（const 引数を外している）

C++ 側の引数が `const` であるにもかかわらず、C コールバック・公開関数が非 const ポインタを受けており、`const_cast` で外している。

- `webrtc_c/api/video_codecs/video_decoder.cc`
  - `VideoDecoder::Configure(const Settings&)` / `Decode(const EncodedImage&)` の転送で `const_cast`
- `webrtc_c/api/video_codecs/video_encoder_factory.cc`
  - `VideoEncoderFactory::Create(const Environment&, const SdpVideoFormat&)` の転送で `const_cast`
- `webrtc_c/api/video_codecs/video_encoder.cc`
  - `Encode(const EncodedImage&, ...)` / `InitEncode(const VideoCodec&, const Settings&)` / `RegisterEncodeCompleteCallback` / `Encode(const VideoFrame&)` などで `const_cast`

### getter に const が付いていない

読み取り専用で値返しの getter が `const struct ...* self` になっていない。

- `webrtc_c/api/video/video_frame_metadata.h` の `webrtc_VideoFrameMetadata_GetFrameType` 等
- `webrtc_c/api/video_codecs/sdp_video_format.h` の `webrtc_SdpVideoFormat_get_name` 等
- `webrtc_c/api/video_codecs/video_decoder.h` の `webrtc_VideoDecoder_DecoderInfo_get_implementation_name` 等
- audio 側（`webrtc_c/api/audio_codecs/`）の SdpAudioFormat / AudioCodecInfo / AudioEncoder / AudioDecoder の値返し getter も同様

## 設計方針

- C++ メソッド（または読み取り専用のフィールド参照）が `const` のものは、C 関数の `self` を `const struct ...*` にする
- `const` 引数（`const Environment&` / `const SdpAudioFormat&` 等）は C API でも `const struct ...*` にし、`const_cast` を廃止する（C++ 側に `const` をそのまま渡す）
- フィールドへの可変参照を返す getter（`get_parameters` 等）は既存慣例のとおり非 const のままとする

## 完了条件

- webrtc_c に `const_cast` が残っていない（取得・設定とも）
- 読み取り専用の値返し getter が `const struct ...* self` になっている
- Rust 側のラッパー（`src/api/*.rs`）が const ポインタを正しく扱えること（`*mut` から `*const` が bindgen で生成され、`as_ptr()` の implicit coercion でコンパイルが通る）

## 解決方法

webrtc_c の C API 全体を対象に、libwebrtc の C++ シグネチャと突き合わせて const 性を是正した。`const_cast` は全廃し、読み取り専用の getter と const 引数を const ポインタに揃えた。

- `self` の const 化
  - 宣言 (`*.h`) と定義 (`*.cc`) を同時に `const struct ...* self` にし、内部の `reinterpret_cast<webrtc::Xxx*>(self)` も `reinterpret_cast<const webrtc::Xxx*>(self)` にした
  - 全 908 個の `self` のうち 392 個を const 化した。残り 516 個は C++ 側が非 const メソッドを呼ぶ、またはフィールドへの可変参照を返すため非 const のままとした
  - 判定は目視ではなくコンパイラで検証した。全 `self` を機械的に const 化してビルドし、const 化できない箇所だけを戻す作業を収束するまで繰り返し、最終的に非 const として残ったものが「C++ 側が書き換える」関数であることを保証している
- 入力ポインタ引数の const 化
  - `struct Xxx* name` 形式の入力引数を `const struct Xxx* name` にした（`out_` で始まる出力引数と `*_delete` は対象外）
  - C++ 側が非 const 参照/ポインタで受ける引数はコンパイルエラーになるため const 化せず、`webrtc_c::OptionalSet` のようにフィールドを書き換える経路も同様に非 const のままとした
- `const_cast` の全廃（13 箇所、4 ファイル）
  - `webrtc_VideoDecoder_cbs.Configure` / `.Decode`、`webrtc_VideoEncoder_cbs.InitEncode` / `.Encode` / `.SetRates`、`webrtc_VideoEncoder_EncodedImageCallback_cbs.OnEncodedImage`、`webrtc_VideoEncoderFactory_cbs.Create` / `webrtc_VideoDecoderFactory_cbs.Create` の引数を `const struct ...*` にした
  - あわせて `webrtc_VideoDecoder_Configure` / `webrtc_VideoDecoder_Decode` / `webrtc_VideoEncoder_InitEncode` / `webrtc_VideoEncoder_Encode` / `webrtc_VideoEncoder_SetRates` / `webrtc_VideoEncoder_EncodedImageCallback_OnEncodedImage` / `webrtc_VideoEncoderFactory_Create` / `webrtc_VideoDecoderFactory_Create` の引数も `const struct ...*` にした
  - C++ 側は `const_cast` を廃止し、`reinterpret_cast<const webrtc::Xxx*>(&value)` でそのまま渡すようにした
- マクロが生成する getter
  - `WEBRTC_DECLARE_VECTOR` / `WEBRTC_DECLARE_VECTOR_NO_DEFAULT_CTOR` の `_vector_size`、`WEBRTC_DECLARE_REFCOUNTED_VECTOR` の `_refcounted_vector_size`、`WEBRTC_DECLARE_INLINED_VECTOR` の `_inlined_vector_size`、`WEBRTC_DECLARE_VARIANT` の `_index` を const 化した
  - `_vector_get` などの要素への可変参照を返す getter は設計方針どおり非 const のままとした
- 設計方針の例外
  - `webrtc_SdpVideoFormat_get_name` / `webrtc_SdpAudioFormat_get_name` は現状の「等」に挙げられていたが、`get_parameters` と同じくフィールド (`name`) への可変参照を借用ポインタで返す getter であるため、設計方針の「フィールドへの可変参照を返す getter は既存慣例のとおり非 const のままとする」に従い非 const のままとした
  - `webrtc_VideoFrameMetadata_GetCsrcs` のようにヒープへ複製して返す getter は const 化した
- Rust 側
  - bindgen が生成するポインタが `*mut` から `*const` になっても、`as_ptr()` が返す `*mut` からの implicit coercion で `src/api/*.rs` の呼び出しはそのままコンパイルできた
  - C 側から呼ばれるコールバック関数ポインタ 8 個（`video_decoder_configure` / `video_decoder_decode` / `video_decoder_factory_create` / `video_encoder_encoded_image_callback_on_encoded_image` / `video_encoder_init_encode` / `video_encoder_encode` / `video_encoder_set_rates` / `video_encoder_factory_create`）は引数を `*const` に変更した
  - `XxxRef` が `*mut` を保持しているため、const ポインタから `XxxRef` を作る箇所は `cast_mut()` で const を外している。C++ の `const_cast` と同じ意味になるこの変換を無くすには `XxxRef` を const ポインタベースにする必要があり、借用型を読み取り専用にする issue 0103 の設計方針に「`XxxRef` は `*const`、`XxxRefMut` は `*mut` を保持する」を追記した
  - `cast_mut()` を使う 20 箇所すべてに、const を外している理由と「借用先を書き換えないこと」のコメントを追加した
- ドキュメント
  - `webrtc/RULES.md` に「C++ 側の const 性を C API でもそのまま反映する」ルールと、セルフチェック手順の確認項目を追加した
  - `skills/libwebrtc-c/SKILL.md` に const 性の節とセルフチェック手順を追加した
  - Rust 側の公開 API に変更が無いため `CHANGES.md` の `## develop` の `### misc` に `[UPDATE]` として記載した
- 対象外とした範囲
  - ObjC のオブジェクトハンドル (`objc_*` / `webrtc_objc_*`) を扱う C API (`objc.h` / `objc.mm` / `sdk/objc/**`) は const 化していない。ObjC の `id` は const を表現できず、対応する ObjC メソッドにも const が無いため、非 const のままが元の API と一致する。const 化すると `__bridge` で const を外すことになり、`release` や setter のような書き換える関数まで const になってしまう
  - この誤りは macOS / iOS の CI で検出した。Linux / Android では ObjC のファイルがダミー実装として C++ コンパイルされるため、ObjC 側の const 化の誤りがコンパイルエラーにならない
- 確認したコマンド
  - `cargo fmt --all -- --check`
  - `cargo clippy --workspace --features source-build -- -D warnings`
  - `cargo test --workspace --features source-build`
  - `python3 webrtc/run.py format --check`
  - `cmake --build ... --target bundled_webrtc_c_bundling` / `whep_c` / `whip_cpp` / `whep_cpp`
