# libwebrtc-c に AudioTrackInterface の Sink 操作とキャスト API を追加する

Created: 2026-03-24
Model: Opus 4.6

## 背景

Video 側には `webrtc_VideoTrackInterface_AddOrUpdateSink()` / `webrtc_VideoTrackInterface_RemoveSink()` が `media_stream_interface.h` に定義されている。また `MediaStreamTrackInterface` から `VideoTrackInterface` へのキャスト (`WEBRTC_DECLARE_CAST_REFCOUNTED`) も公開されている。

Audio 側には `AudioTrackInterface` から `MediaStreamTrackInterface` へのキャストは存在するが、以下が欠落している:

- `AudioTrackInterface` への `AddSink()` / `RemoveSink()`
- `MediaStreamTrackInterface` から `AudioTrackInterface` へのキャスト

## 根拠

AudioSinkInterface (#0001) を作成しても、AudioTrack に登録する手段がなければ音声データを受信できない。また `MediaStreamTrack` から `AudioTrack` を取り出すキャストがないと、RTP レシーバーから取得したトラックを AudioTrack として扱えない。

## 対応内容

- `media_stream_interface.h` に `webrtc_AudioTrackInterface_AddSink()` を追加する
- `media_stream_interface.h` に `webrtc_AudioTrackInterface_RemoveSink()` を追加する
- `media_stream_interface.h` に `WEBRTC_DECLARE_CAST_REFCOUNTED(webrtc_MediaStreamTrackInterface, webrtc_AudioTrackInterface)` を追加する
- 対応する `.cc` 実装を追加する

## 依存

- #0001 (AudioSinkInterface の C API)
