# Rust 層に AudioSink / AudioTrack sink 操作 / cast_to_audio_track() を追加する

Created: 2026-03-24
Completed: 2026-03-24
Model: Opus 4.6

## 背景

Video 側には以下の Rust API が実装されている:

- `VideoSinkHandler` trait（`on_frame()` / `on_discarded_frame()`）
- `VideoSink` struct（`new_with_handler()`）
- `VideoTrack::add_or_update_sink()` / `VideoTrack::remove_sink()`
- `MediaStreamTrack::cast_to_video_track()`

Audio 側にはこれらに相当する API が一切存在しない。

## 根拠

音声データをアプリケーション側で受信・処理するには、Video と同等の Sink パターンが Audio にも必要である。現状では `MediaStreamTrack` を取得しても Audio トラックとして利用する手段がない。

C 層には `MediaStreamTrackInterface ↔ AudioTrackInterface` のキャストが既に実装済みであり、#0001 で AudioTrackSinkInterface の C API が追加されれば、Rust 側のラッパーを実装できる。

## 対応内容

- `src/api/audio.rs` に以下を追加する
  - `AudioSinkHandler` trait（`on_data()` コールバック）
  - `AudioSink` struct（`new_with_handler()` ファクトリ）
  - `AudioTrack::add_sink()` / `AudioTrack::remove_sink()` メソッド
- `src/api/rtp.rs` の `MediaStreamTrack` に `cast_to_audio_track()` を追加する
- `CHANGES.md` を更新する

## 依存

- #0001 (C 層に AudioTrackSinkInterface と AudioTrack の Sink 操作 API を追加する)

## 解決方法

- `audio.rs` に `AudioSinkHandler` trait、`AudioSink` struct、`AudioTrack::add_sink()` / `remove_sink()` を追加した
- `rtp.rs` の `MediaStreamTrack` に `cast_to_audio_track()` を追加した（C 層に既存のキャスト関数を利用）
- `CHANGES.md` を更新した
