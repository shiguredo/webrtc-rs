# RTCConfiguration に always_negotiate_data_channels オプションを追加する

Created: 2026-04-18
Completed: 2026-04-19
Model: Claude Opus 4.7

## 概要

`webrtc::PeerConnectionInterface::RTCConfiguration::always_negotiate_data_channels` を Rust 側から設定できるようにする。
C API と Rust API を追加する。

## 背景

libwebrtc に 2025-10-23 に `always_negotiate_data_channels` オプションが追加された。

- commit: `6e9dccadeea4517e0f85b62be35fcda5affcfaad`
- Cr-Commit-Position: `refs/heads/main@{#46025}`
- 著者: Philipp Hancke
- レビュー: <https://webrtc-review.googlesource.com/c/src/+/417783>
- 参考: <https://github.com/w3c/webrtc-pc/issues/3072>

このオプションを有効にすると、DataChannel を事前に作成していなくても、Offer / Answer の SDP に DataChannel 用の `m=application` セクションが常にネゴシエーションされる。
後から DataChannel を作る可能性がある用途 (DataChannel の遅延生成) では重要な機能のため、`webrtc-rs` でも設定可能にする必要がある。

現状 `PeerConnectionRtcConfiguration` には `set_type` / `servers` しか公開されておらず、このフィールドへアクセスする手段が無い。

対象フィールドは libwebrtc m147 ツリー (webrtc-rs がビルド対象とする `m147.7727.10.0`) の `api/peer_connection_interface.h:695` に存在することを確認済み。

## 対応内容

- `webrtc/src/webrtc_c/api/peer_connection_interface.h` に setter を宣言する
- `webrtc/src/webrtc_c/api/peer_connection_interface.cc` に setter を実装する
- `src/api/peer_connection.rs` の `PeerConnectionRtcConfiguration` に `set_always_negotiate_data_channels` メソッドを追加する
- `CHANGES.md` の `## develop` に `[ADD]` エントリを追加する

## 解決方法

コミット `0e0aecf` で実装し、develop へは `83b5a11` (#55, 2026-04-19) でマージ済み。
issue の対応内容 4 項目をすべて実装した。行番号は実装当時のもので、以降の変更でずれる可能性がある。

- C API 宣言: `webrtc/src/webrtc_c/api/peer_connection_interface.h:96-99` に `webrtc_PeerConnectionInterface_RTCConfiguration_set_always_negotiate_data_channels` を宣言した
- C API 実装: `webrtc/src/webrtc_c/api/peer_connection_interface.cc:389-397` で `RTCConfiguration::always_negotiate_data_channels` フィールドへ代入する実装を追加した
- Rust API: `src/api/peer_connection.rs:476-484` に `PeerConnectionRtcConfiguration::set_always_negotiate_data_channels` を追加した
- 変更履歴: `CHANGES.md` に `[ADD]` エントリとして記載した。リリース時に `## develop` から `## 0.147.1` (リリース日 2026-04-20) へ昇格している
- テスト: `src/tests.rs` の `always_negotiate_data_channels_adds_data_section()` で、createOffer 実行時に `set_always_negotiate_data_channels(true)` なら SDP に `m=application` が含まれ、デフォルト (false) では含まれないことを対照実験で検証した
