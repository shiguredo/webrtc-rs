# webrtc-rs に remove_track の Rust バインディングを追加する

Created: 2026-03-26
Model: Opus 4.6

## 概要

libwebrtc-c の `webrtc_PeerConnectionInterface_RemoveTrackOrError` に対応する Rust バインディングを webrtc-rs に追加する。

## 背景

issue #0003 で libwebrtc-c に `RemoveTrackOrError` C API が追加される予定。
webrtc-rs 側にも対応する FFI バインディングと Rust API が必要。

## 対応内容

- libwebrtc-c の `webrtc_PeerConnectionInterface_RemoveTrackOrError` に対する FFI 宣言を追加する
- `PeerConnection` に `remove_track` メソッドを追加する
- 既存の `add_track` と同様のエラーハンドリングパターンを使う
