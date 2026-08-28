# out 引数 + out_error の FFI ボイラープレートを共通ヘルパー化する

- Created: 2026-08-28
- Completed: {YYYY-MM-DD}
- Branch: feature/refactor-out-param-error-helper
- Polished: {YYYY-MM-DD}

## 目的

C API が結果を out 引数 (出力ポインタ) と `out_error` (`webrtc_RTCError_unique`) で返し、Rust 側で null 検査・エラー検査をする定型コードが 20 箇所以上に散在している。これを共通ヘルパーに集約して各呼び出しを数行に縮め、`Error::NullPointer` のメッセージ表記ゆれも解消する。挙動は一切変更しない。

## 現状

`src/api/peer_connection.rs` に最も集中し、`src/api/jsep.rs` にも同型がある。定型は次の形に分類できる。

### Shape A: out ポインタ → `Error::NullPointer` → `Result<T>` (4 箇所)

```rust
let mut out = std::ptr::null_mut();
unsafe { ffi::...(self.raw_ref.as_ptr(), ..., &mut out); };
let out = NonNull::new(out).ok_or(Error::NullPointer("... が null を返しました"))?;
let raw_ref = ScopedRef::<XxxHandle>::from_raw(out);
Ok(Xxx::from_scoped_ref(raw_ref))
```

- `PeerConnectionFactory::create_video_track` (`src/api/peer_connection.rs`)
- `PeerConnectionFactory::create_audio_source`
- `PeerConnectionFactory::create_audio_track`
- `PeerConnectionFactory::create_local_media_stream` (メッセージが英語 "returned null" で表記ゆれ)

### Shape B: out オブジェクト + out_error の 2 出力 (5 箇所)

`if !out_error.is_null()` でエラーを検査し、エラーでなければ `NonNull::new(out).expect("BUG: ...")` で必須値を取り出す。

- `PeerConnection::create` (out_pc / out_error)
- `PeerConnection::create_data_channel` (out_dc / out_error)
- `PeerConnection::add_transceiver` (out_transceiver / out_error)
- `PeerConnection::add_transceiver_with_track` (out_transceiver / out_error)
- `PeerConnection::add_track` (out_sender / out_error)

### Shape C: out_error のみ → `Err(Error::RtcError)` (2 箇所)

- `PeerConnection::set_configuration`
- `PeerConnection::remove_track`

### その他の variant

- Shape D: 追加の out_context を返す `PeerConnectionFactory::create_modular_with_context` (`src/api/peer_connection.rs`)
- Shape E: 文字列 (`std_string_unique`) を out で受け取り `from_unique` する `SessionDescription::to_string` / `IceCandidateRef::sdp_mid` / `IceCandidateRef::to_string` / `SdpParseError::line` / `SdpParseError::description` (`src/api/jsep.rs`)
- Shape F: out_error を `SdpParseError` で受け取る `IceCandidate::new` (`src/api/jsep.rs`)
- Shape G: 複数フィールド (AdaptedSize) を複数 out で受ける `AdaptedVideoTrackSource::adapt_frame` (`src/api/video.rs`)

## 設計方針

- `src/api/optional.rs` と同じクロージャ方式の共通ヘルパーを 1 箇所に追加する (例: out を受け取って `NonNull` と `Error::NullPointer` に変換する `get_out`、out オブジェクト + out_error を検査する `call_with_out_error`)。クロージャに out / out_error のポインタを渡す形にする
- 呼び出し側はヘルパー呼び出し数行になり、`Error::NullPointer` / `Error::RtcError` へ変換する定型とメッセージ表記ゆれが消える
- 例外テキスト (「out_pc と out_error が両方 null です」など) は意味を失わないよう個別に扱う
- C API (`webrtc/src/webrtc_c/`) と bindgen 生成の FFI 定義は変更しない

## 完了条件

- 上記の定型がすべて共通ヘルパー経由に置き換わっている
- `Error::NullPointer` / `Error::RtcError` の検査とメッセージの表記が揃っている
- 挙動がリファクタ前と同一である (テストで担保する)
- ビルドと全テストが通る
- `CHANGES.md` の develop に `### misc` エントリを追記する
