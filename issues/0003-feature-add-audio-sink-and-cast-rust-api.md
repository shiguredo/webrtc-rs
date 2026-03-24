# webrtc-rs に AudioSink / cast_to_audio_track() の Rust API を追加する

Created: 2026-03-24
Model: Opus 4.6

## 背景

Video 側には以下の Rust API が `src/api/video.rs` に実装されている:

- `VideoSinkHandler` trait (`on_frame()` / `on_discarded_frame()`)
- `VideoSink` struct (`new_with_handler()`)
- `VideoTrack::add_or_update_sink()` / `VideoTrack::remove_sink()`
- `MediaStreamTrack::cast_to_video_track()` (`src/api/rtp.rs`)

Audio 側にはこれらに相当する API が一切存在せず、`src/api/audio.rs` の `AudioTrack` は最小限の実装のみである。

## 根拠

音声データをアプリケーション側で受信・処理するには、Video と同等の Sink パターンが Audio にも必要である。現状では `MediaStreamTrack` を取得しても Audio トラックとして利用する手段がない。

## 対応内容

- `src/api/audio.rs` に `AudioSinkHandler` trait を追加する (`on_data()` コールバック)
- `src/api/audio.rs` に `AudioSink` struct を追加する (`new_with_handler()` ファクトリ)
- `AudioTrack` に `add_sink()` / `remove_sink()` メソッドを追加する
- `src/api/rtp.rs` の `MediaStreamTrack` に `cast_to_audio_track()` メソッドを追加する
- FFI 層に libwebrtc-c の対応する C 関数の `extern "C"` 宣言を追加する

## 依存

- #0001 (AudioSinkInterface の C API)
- #0002 (AudioTrackInterface の Sink 操作とキャスト C API)
