# MediaStreamTrack に state の取得口を追加する

- Created: 2026-09-04
- Completed: {YYYY-MM-DD}
- Branch: feature/add-media-stream-track-state
- Polished: 2026-09-04

## 目的

トラックの生死（`LIVE` / `ENDED` 相当）を Rust 側から取得できるようにする。上位層ではトラックの表示切り替えや終了判定に使い、`on_remove_track` 到達の自前管理だけでは取りこぼし時の誤表示を避けられないため、正攻法の取得口が必要である。

## 現状

- `src/api/rtp.rs` の `MediaStreamTrack` は `kind` / `id` / `enabled` / `set_enabled` と音声・映像へのキャストだけを持ち、`state` に相当する取得口がない
- `webrtc/src/webrtc_c/api/media_stream_interface.h` と `media_stream_interface.cc` の C API も `kind` / `id` / `enabled` / `set_enabled` だけで、生死の取得を露出していない

## 設計方針

- `webrtc_c` にトラック自体の生死を返す C API を追加する（例： `webrtc_MediaStreamTrackInterface_state`）。露出元は `MediaStreamTrackInterface::state` とし、`MediaSourceInterface::SourceState` ではない。`DataChannelState` / `DtlsTransportState` と同様に typedef と `kLive` / `kEnded` 対応定数で表す。読み取り専用の値返し getter として、C 関数の `self` は `const` 扱いにする
- `src/api/rtp.rs` に `MediaStreamTrackState` を追加する（`DtlsTransportState` / `DataChannelState` の接頭辞付き命名に倣い、素の `TrackState` にはしない）。生死を表す `Live` / `Ended` と将来値のための `Unknown(i32)` を持ち、`from_int` のみを提供する（読み取り専用のため `to_int` は作らない）
- `MediaStreamTrack` に `state` を追加し、`MediaStreamTrackState` を非 optional で返す
- 既存の `enabled` とは意味が異なる（有効 / 無効の設定値ではなく生死）ため、別項目として追加し、置き換えはしない

## 完了条件

- `MediaStreamTrack::state` が `MediaStreamTrackState` を非 optional で返し、`Live` / `Ended` の値対応と `Unknown` の扱いをテストで確認できること
- `cargo test` と `cargo clippy --all-targets -- -D warnings` が成功すること

## 解決方法

（詳細は polish / 実装時に確定する）
