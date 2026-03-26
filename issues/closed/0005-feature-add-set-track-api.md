# RtpSenderInterface に SetTrack C API と Rust バインディングを追加する

Created: 2026-03-26
Completed: 2026-03-26
Model: Opus 4.6

## 概要

libwebrtc-c に `webrtc_RtpSenderInterface_SetTrack` を追加し、webrtc-rs の `RtpSender::set_track` から利用できるようにする。

## 背景

現在 libwebrtc-c の `RtpSenderInterface` には `GetParameters` / `SetParameters` はあるが、`SetTrack` が存在しない。
`SetTrack` はセンダーのトラックを差し替える API で、`null` を渡すとトラックなしにもできる。
`RemoveTrackOrError` とは異なり、トランシーバーの方向は変えずにトラック自体を入れ替える操作。

## 対応内容

### libwebrtc-c

- `webrtc_RtpSenderInterface_SetTrack(self, track)` を追加する
- `track` は `null` を許容する（トラックを外す操作）
- `bool` を返す（成功時 `true`、型不一致時 `false`）

### webrtc-rs

- バンドルヘッダー (.h / .cc) に同じ C API を追加する
- `RtpSender::set_track(&mut self, track: Option<&MediaStreamTrack>) -> bool` を追加する

## 解決方法

libwebrtc-c のヘッダーと実装に `webrtc_RtpSenderInterface_SetTrack` を追加した。
webrtc-rs のバンドルヘッダー (.h / .cc) にも同じ C API を追加し、`RtpSender::set_track` を実装した。
`track` に `None` を渡すと `null` を C API に渡してトラックを外す操作ができる。
