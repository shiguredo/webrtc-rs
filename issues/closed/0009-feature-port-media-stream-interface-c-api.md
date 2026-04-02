# MediaStreamInterface と CreateLocalMediaStream の C API / Rust 移植を追加する

Created: 2026-04-02
Completed: 2026-04-02
Model: GPT-5 Codex

## 概要

`webrtc::MediaStreamInterface` の主要 API と、`PeerConnectionFactoryInterface::CreateLocalMediaStream` の C API / Rust API を追加する。

## 背景

現状の `webrtc-rs` は `MediaStreamTrack` 単位の操作は可能だが、`MediaStreamInterface` 自体を Rust から直接操作できない。
そのため stream 単位での track コンテナ操作 (`GetAudioTracks` / `GetVideoTracks` / `Find*` / `AddTrack` / `RemoveTrack`) ができず、libwebrtc の API で想定される stream ライフサイクル管理をそのまま扱えない。
また `CreateLocalMediaStream` が Rust API に存在しないため、ローカル stream 生成経路が欠けている。

## 対応内容

- `webrtc::MediaStreamInterface` の C API を追加する
- `PeerConnectionFactoryInterface::CreateLocalMediaStream` の C API を追加する
- Rust 側に `MediaStream` ラッパーと `PeerConnectionFactory::create_local_media_stream` を追加する
- `MediaStream` 操作のテストを追加する
- `CHANGES.md` に develop 向けの変更履歴を追加する

## 解決方法

- `webrtc/src/webrtc_c/api/media_stream_interface.h/.cc` に `webrtc_MediaStreamInterface_*` を追加し、`id` / `GetAudioTracks` / `GetVideoTracks` / `Find*` / `AddTrack*` / `RemoveTrack*` を C API として公開した
- track vector は `common.h` / `common.impl.h` に追加した `WEBRTC_DECLARE_REFCOUNTED_VECTOR` / `WEBRTC_DEFINE_REFCOUNTED_VECTOR` を使って `scoped_refptr` vector をラップし、`*_vector_get` が直接参照カウント付きポインタを返すようにした
- `webrtc/src/webrtc_c/api/peer_connection_interface.h/.cc` に `webrtc_PeerConnectionFactoryInterface_CreateLocalMediaStream` を追加した
- Rust 側で `src/api/media_stream.rs` を追加し、`MediaStream` と各操作 API (`audio_tracks` / `video_tracks` / `find_*` / `add_*` / `remove_*`) を実装した
- `PeerConnectionFactory::create_local_media_stream` を追加し、`src/ref_count.rs` に `MediaStreamHandle` を追加した
- `src/tests.rs` に `create_local_media_stream_returns_requested_id` と `media_stream_track_round_trip` を追加し、追加 API の往復動作を検証可能にした
- `CHANGES.md` の `## develop` に今回の追加内容を `[ADD]` で追記した
