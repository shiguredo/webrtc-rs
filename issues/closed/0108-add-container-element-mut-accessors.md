# コンテナの要素を直接書き換える可変アクセサを追加する

- Created: 2026-09-28
- Completed: 2026-09-28
- Branch: feature/add-container-element-mut-accessors
- Polished: {YYYY-MM-DD}

## 目的

コンテナ型とその借用型が持つ `get` は読み取り専用の `XxxRef` か値のコピーを返すため、要素を 1 つ書き換えるには、要素を読んで別の値を作り `set` で戻す往復が必要になる。`set` を持たない型では要素の書き換え自体ができない。

`&mut self` から要素の可変借用 (`XxxRefMut`) を直接取得できる `get_mut` を追加し、書き換えのために要素を組み立て直さなくて済むようにする。

## 現状

`issues/0103-refactor-readonly-ref-types.md` で借用ハンドルは読み取り専用の `XxxRef` と書き換え用の `XxxRefMut` に分かれたが、コンテナの要素については `get` に対応する `get_mut` が追加されていない。要素型の `XxxRefMut` は所有型の `as_mut` からしか取得できず、コンテナ経由では取得できない。

同じ形の欠落が次の 7 箇所にある。

| 対象 | 読み取り | 現在の書き換え手段 | 要素型の可変ハンドル |
|---|---|---|---|
| `RtpEncodingParametersVector` (`src/api/rtp.rs`) | `get` | `set` のみ | `RtpEncodingParametersRefMut` はあるがコンテナ経由では取得できない |
| `RtpCodecCapabilityVector` / `RtpCodecCapabilityVectorRefMut` (`src/api/rtp.rs`) | `get` | `set` のみ | `RtpCodecCapabilityRefMut` はあるがコンテナ経由では取得できない |
| `IceServerVector` / `IceServerVectorRefMut` (`src/api/peer_connection.rs`) | `get` | 無し (`push` のみ) | `IceServerRefMut` はあるがコンテナ経由では取得できない |
| `VideoEncoderResolutionBitrateLimitsVectorRefMut` (`src/api/video_encoder.rs`) | `get` | `set` のみ | `VideoEncoderResolutionBitrateLimitsRefMut` はあるがコンテナ経由では取得できない |
| `NaluInfoVector` (`src/api/video_codec_specifics.rs`) | `get` | 無し | `NaluInfoRefMut` が無い |
| `StringVector` / `StringVectorRefMut` (`src/cxxstd.rs`) | `get` (`String` のコピー) | 無し (`push` のみ) | `CxxStringRefMut` はあるがコンテナ経由では取得できない |
| `VideoFrameTypeVector` / `VideoFrameTypeVectorRefMut` (`src/api/video_codec_common.rs`) | `get` | 無し (`push` のみ) | 要素が値型 (`VideoFrameType`) のため `get_mut` ではなく `set` が必要 |

`get` と `set` を持つ型でも、`RtpEncodingParameters` や `RtpCodecCapability` はフィールド数が多いため、`XxxRef` から読んだ値を `Xxx::new()` で組み立て直す必要がある。

C API 側は `WEBRTC_DECLARE_VECTOR` / `WEBRTC_DEFINE_VECTOR` が非 const の `_vector_get` と `_vector_set` を生成済みで、`webrtc_VideoFrameType` を除いて追加は不要。`webrtc_VideoFrameType` には `webrtc_VideoFrameType_value` と `webrtc_VideoFrameType_vector_push_back_value` はあるが、値を書き込む関数が無い。

## 設計方針

- 上の表の各型に `get_mut(&mut self, index: usize) -> Option<XxxRefMut<'_>>` を追加する
  - 範囲外の index は既存の `get` と揃えて `None` を返す
  - 戻り値のライフタイムは `'a` ではなく `'_` にする。`'a` を返すと呼び出しごとに借用が切れ、同一要素への可変ハンドルを 2 本作れてしまう (`issues/0103-refactor-readonly-ref-types.md` のレビューで `parameters_mut` など 7 箇所を `'_` に縛ったのと同じ理由)
  - `set` を持たない `IceServerVector` / `IceServerVectorRefMut` / `NaluInfoVector` には、要素を丸ごと差し替える `set(&mut self, index, value: &Xxx) -> bool` も追加して既存型と揃える
- 要素が値型の `VideoFrameTypeVector` / `VideoFrameTypeVectorRefMut` には `set(&mut self, index, value: VideoFrameType) -> bool` を追加する (`VideoEncoderFramerateFractionInlinedVectorRefMut` / `VideoFrameBufferKindInlinedVectorRefMut` と同じ形)
  - 値を書き込む C API として `webrtc_VideoFrameType_vector_set_value` を `webrtc_VideoFrameType_vector_push_back_value` と同じ形で追加する
- `NaluInfoRefMut` (`PhantomData<&'a mut webrtc_NaluInfo>` を持ち `Copy` でない型) を `RtpEncodingParametersRefMut` と同じ形で新設する。`issues/0103-refactor-readonly-ref-types.md` では構築経路が無いために削除された型で、`get_mut` の追加によって構築経路ができる
- `StringVector::get_mut` は既存の `get` が範囲外で `Error::OutOfIndex` を返すため `Result<CxxStringRefMut<'_>>` を返す
- マクロと新規トレイトは作らない。所有型の `get_mut` は所有型が自分でハンドルを組み立てる (`as_mut()` の戻り値は一時値になるため委譲できない)
- 追加のみで既存 API は変えない

## 完了条件

- 上表の 7 箇所すべてで、要素の可変アクセサ (`get_mut` または `set`) を `&mut self` から取得できる
- 可変アクセサの戻り値のライフタイムが `'_` になっており、同一要素への可変ハンドルを 2 本作れない
- `&self` から可変アクセサを取得できる経路が増えていない
- `src/tests.rs` に、各型で要素を書き換えた結果が `get` から読めることを確認するテストがある
  - 所有型と `XxxVectorRefMut` の両方の経路を確認する
  - 範囲外の index で `None` / `Err` / `false` が返ることを確認する
- `cargo fmt --all -- --check` / `cargo clippy --workspace --features source-build -- -D warnings` / `cargo test --workspace --features source-build` が成功する
- `CHANGES.md` の `## develop` 節に `[ADD]` として記載されている

## 解決方法

- 表の 7 箇所すべてに `get_mut(&mut self, index: usize)` を追加した
  - `RtpEncodingParametersVector` / `RtpCodecCapabilityVector` / `RtpCodecCapabilityVectorRefMut` は `RtpEncodingParametersRefMut` / `RtpCodecCapabilityRefMut` を返す
  - `IceServerVector` / `IceServerVectorRefMut` は `IceServerRefMut`、`NaluInfoVector` は新設した `NaluInfoRefMut`、`VideoEncoderResolutionBitrateLimitsVectorRefMut` は `VideoEncoderResolutionBitrateLimitsRefMut` を返す
  - `StringVector` / `StringVectorRefMut` は既存の `get` に揃えて範囲外で `Error::OutOfIndex` を返すため `Result<CxxStringRefMut<'_>>` を返す
- 戻り値のライフタイムは `'_` にした。所有型の `get_mut` は `as_mut()` の戻り値が一時値になるため委譲できず、自身のポインタからハンドルを組み立てている
- 要素を丸ごと差し替える `set(&mut self, index, value) -> bool` を `IceServerVector` / `IceServerVectorRefMut` に追加し、`NaluInfoVector` にも同じ形で追加した
- 要素が値型の `VideoFrameTypeVector` / `VideoFrameTypeVectorRefMut` には `set(&mut self, index, value: VideoFrameType) -> bool` を追加した
  - 値を書き込む C API として `webrtc_VideoFrameType_vector_set_value` を `webrtc_VideoFrameType_vector_push_back_value` と同じ形で追加した
- `NaluInfoRefMut` を `RtpEncodingParametersRefMut` と同じ形 (`NonNull` と `cref` を保持し、セッターと読み取りの転送メソッドを持つ `Copy` でない型) で新設した
- マクロと新規トレイトは作らず、追加のみで既存 API は変えていない
- `src/lib.rs` の `compile_fail` doctest に、コンテナの `get_mut` が `'_` に縛られて同一要素への可変ハンドルを 2 本作れないこと (E0499) を固定するケースを追加した
- `src/tests.rs` に 7 本のテストを追加し、所有型と `XxxVectorRefMut` の両経路で書き換えた結果が `get` から読めることと、範囲外の index で `None` / `Err` / `false` が返ることを確認した
- `CHANGES.md` の `## develop` 節に `[ADD]` を記載し、README の「借用ハンドル」節と `skills/shiguredo-webrtc/SKILL.md` に要素アクセサの規約を追記した
- `cargo fmt --all -- --check` / `cargo clippy --workspace --features source-build --all-targets -- -D warnings` / `cargo test --workspace --features source-build` / `python3 webrtc/run.py format --check` / `prek run` の成功を確認した
- prebuilt 利用者が `webrtc_VideoFrameType_vector_set_value` を使えるようにするには、この変更を含むバージョンをリリースして `libwebrtc_c-*.tar.gz` を作り直す必要がある
