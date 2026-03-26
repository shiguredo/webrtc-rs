# libwebrtc-c に RemoveTrackOrError C API を追加する

Created: 2026-03-26
Model: Opus 4.6

## 概要

libwebrtc-c に `webrtc_PeerConnectionInterface_RemoveTrackOrError` を追加する。

## 背景

現在 libwebrtc-c には `AddTrack` はあるが、対になる `RemoveTrackOrError` が存在しない。
トラックの削除ができないため、WHIP/WHEP などのシナリオでトラックのライフサイクル管理が不完全になっている。

## 対応内容

- `webrtc::PeerConnectionInterface::RemoveTrackOrError` を薄くラップする C API を追加する
- 既存の `AddTrack` と同様のエラーハンドリングパターン (`out_rtc_error`) を使う
- シグネチャ: `webrtc_PeerConnectionInterface_RemoveTrackOrError(self, sender, out_rtc_error)`
