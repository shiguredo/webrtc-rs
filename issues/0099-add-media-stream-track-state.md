# MediaStreamTrack に state の取得口を追加する

- Created: 2026-09-04
- Completed: {YYYY-MM-DD}
- Branch: feature/add-media-stream-track-state
- Polished: {YYYY-MM-DD}

## 目的

トラックの生死（`LIVE` / `ENDED` 相当）を Rust 側から取得できるようにする。上位層ではトラックの表示切り替えや終了判定に使い、`on_remove_track` 到達の自前管理だけでは取りこぼし時の誤表示を避けられないため、正攻法の取得口が必要である。

## 現状

- `src/api/rtp.rs` の `MediaStreamTrack` は `kind` / `id` / `enabled` / `set_enabled` と音声・映像へのキャストだけを持ち、`state` に相当する取得口がない
- `webrtc/src/webrtc_c/api/media_stream_interface.h` と `media_stream_interface.cc` の C API も `kind` / `id` / `enabled` / `set_enabled` だけで、生死の取得を露出していない

## 設計方針

- `webrtc_c` にトラックの生死を返す C API を追加し、整数で返す（`DegradationPreference` の `to_int` / `from_int` と同様に Rust 側で列挙へ変換する）
- `src/api/rtp.rs` に `TrackState` 相当の列挙（生死を表す `Live` / `Ended` と将来値のための `Unknown`）を追加し、`MediaStreamTrack` に `state` を追加する
- 既存の `enabled` とは意味が異なる（有効 / 無効の設定値ではなく生死）ため、別項目として追加し、置き換えはしない

## 完了条件

- `MediaStreamTrack` から生死を列挙として取得できること
- `cargo test` と `cargo clippy --all-targets -- -D warnings` が成功すること

## 解決方法

（詳細は polish / 実装時に確定する）
