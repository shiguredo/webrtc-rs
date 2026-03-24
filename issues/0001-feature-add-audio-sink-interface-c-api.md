# libwebrtc-c に AudioSinkInterface の C API を追加する

Created: 2026-03-24
Model: Opus 4.6

## 背景

Video 側には `video_sink_interface.h` が存在し、`webrtc_VideoSinkInterface_cbs` コールバック構造体、`webrtc_VideoSinkInterface_new()` / `webrtc_VideoSinkInterface_delete()` が公開されている。

Audio 側にはこれに相当する C API が一切存在しないため、Rust 側で AudioSink を実装できない。

## 根拠

libwebrtc の `AudioTrackSinkInterface` は `OnData()` コールバックを持っており、音声フレームを受け取るために必要なインターフェースである。Video 側と同等の機能を Audio 側にも提供するには、まず C バインディング層でこのインターフェースを公開する必要がある。

## 対応内容

- `libwebrtc-c/include/libwebrtc-c/api/media_stream_interface.h` に `webrtc_AudioTrackSinkInterface_cbs` コールバック構造体を定義する
- `webrtc_AudioTrackSinkInterface_new()` ファクトリ関数を追加する
- `webrtc_AudioTrackSinkInterface_delete()` デストラクタ関数を追加する
- 対応する `.cc` 実装を追加する

## 依存

- なし（libwebrtc-c 側の作業）
