# PeerConnectionRtcConfiguration に cpu_adaptation の取得・設定口を追加する

- Created: 2026-09-10
- Completed: {YYYY-MM-DD}
- Branch: feature/add-cpu-adaptation
- Polished: {YYYY-MM-DD}

## 目的

Rust SDK から CPU アダプテーション（負荷に応じて映像のフレームレートや解像度を自動調整する機能）の有効 / 無効を設定できるようにする。

C++ SDK の `SoraSignalingConfig` には `cpu_adaptation` フィールドがあり、macOS のサイマルキャスト時に解像度が無限に低下する問題を回避する目的で使われている。Rust SDK からも同じ設定を行えるようにするため、webrtc-rs の `PeerConnectionRtcConfiguration` に取得・設定口を追加する。

## 現状

- libwebrtc m154 の `webrtc::PeerConnectionInterface::RTCConfiguration` は `cpu_adaptation()` / `set_cpu_adaptation(bool)` を持ち、値は `media_config.video.enable_cpu_adaptation`（既定値 `true`）に格納される
- `webrtc/src/webrtc_c/api/peer_connection_interface.h` と `.cc` の `webrtc_PeerConnectionInterface_RTCConfiguration` は `set_type` / `set_sdp_semantics` / `set_enable_gcm_crypto_suites` / `set_always_negotiate_data_channels` / `get_servers` のみで、`cpu_adaptation` の取得・設定がない
- `src/api/peer_connection.rs` の `PeerConnectionRtcConfiguration` にも `cpu_adaptation` の取得・設定がない

## 設計方針

- `webrtc_c` に libwebrtc のアクセサへ薄く委譲する C API を追加する
  - 取得: `webrtc_PeerConnectionInterface_RTCConfiguration_cpu_adaptation(const struct webrtc_PeerConnectionInterface_RTCConfiguration* self)`
    - C++ のメソッド名に合わせて `get_` を付けない。読み取り専用のため `self` は `const` にする（`webrtc_MediaStreamTrackInterface_state` と同じ扱い）
  - 設定: `webrtc_PeerConnectionInterface_RTCConfiguration_set_cpu_adaptation(struct webrtc_PeerConnectionInterface_RTCConfiguration* self, int cpu_adaptation)`
  - bool は既存の bool 系 API（`webrtc_RtpEncodingParameters_get_active` / `set_active`）と同じく `int` の 0/1 で受け渡し、設定時は `!= 0` で真偽に変換する
- `src/api/peer_connection.rs` の `PeerConnectionRtcConfiguration` に `cpu_adaptation(&self) -> bool` / `set_cpu_adaptation(&mut self, enable: bool)` を追加する
- 既定値は libwebrtc に任せ、webrtc-rs 側では上書きしない

## 完了条件

- `PeerConnectionRtcConfiguration` の `cpu_adaptation()` が既定で `true` を返し、`set_cpu_adaptation(false)` で `false`、`true` に戻すと `true` に戻る往復を `src/tests.rs` のテストで確認できること
- `cargo test` と `cargo clippy --all-targets -- -D warnings` が成功すること

## 解決方法

（詳細は polish / 実装時に確定する）
