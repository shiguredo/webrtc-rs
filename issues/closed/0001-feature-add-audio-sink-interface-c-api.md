# C 層に AudioTrackSinkInterface と AudioTrack の Sink 操作 API を追加する

Created: 2026-03-24
Completed: 2026-03-24
Model: Opus 4.6

## 背景

Video 側には `video_sink_interface.h` が存在し、`webrtc_VideoSinkInterface_cbs` コールバック構造体、`webrtc_VideoSinkInterface_new()` / `webrtc_VideoSinkInterface_delete()` が公開されている。また `media_stream_interface.h` に `webrtc_VideoTrackInterface_AddOrUpdateSink()` / `RemoveSink()` が定義されている。

Audio 側にはこれに相当する C API が一切存在しないため、Rust 側で AudioSink を実装できない。

なお `MediaStreamTrackInterface ↔ AudioTrackInterface` のキャストは既に C 層に実装済みである。

## 根拠

libwebrtc の `AudioTrackSinkInterface` は `OnData()` コールバックを持っており、音声フレームを受け取るために必要なインターフェースである。Video 側と同等の機能を Audio 側にも提供するには、C バインディング層でこのインターフェースを公開し、AudioTrack への登録手段を提供する必要がある。

## 対応内容

- `webrtc/src/webrtc_c/api/audio/audio_track_sink_interface.h` を新規作成する
  - `webrtc_AudioTrackSinkInterface_cbs` コールバック構造体を定義する（OnData, OnDestroy）
  - `webrtc_AudioTrackSinkInterface_new()` ファクトリ関数を追加する
  - `webrtc_AudioTrackSinkInterface_delete()` デストラクタ関数を追加する
- `webrtc/src/webrtc_c/api/audio/audio_track_sink_interface.cc` を新規作成する
  - `AudioTrackSinkInterfaceImpl` クラスを実装する
- `webrtc/src/webrtc_c/api/media_stream_interface.h` に以下を追加する
  - `webrtc_AudioTrackInterface_AddSink()` 宣言
  - `webrtc_AudioTrackInterface_RemoveSink()` 宣言
- `webrtc/src/webrtc_c/api/media_stream_interface.cc` に対応する実装を追加する
- `webrtc/CMakeLists.txt` にソースファイルを追加する
- `webrtc/src/webrtc_c.h` に include を追加する

## 依存

- なし

## 解決方法

- `audio_track_sink_interface.h/.cc` を新規作成し、`AudioTrackSinkInterfaceImpl` クラスで `webrtc::AudioTrackSinkInterface` を継承して `OnData()` コールバックを C 関数ポインタに委譲する実装を行った
- `media_stream_interface.h/.cc` に `webrtc_AudioTrackInterface_AddSink()` / `RemoveSink()` を追加した
- `CMakeLists.txt` と `webrtc_c.h` を更新した
