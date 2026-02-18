# 変更履歴

- CHANGE
  - 下位互換のない変更
- UPDATE
  - 下位互換がある変更
- ADD
  - 下位互換がある追加
- FIX
  - バグ修正

## develop

- [CHANGE] Ubuntu 24.04 LTS arm64 / Ubuntu 22.04 LTS arm64 / Raspberry Pi OS arm64 のクロスコンパイルに対応する
  - @voluntas
- [ADD] リリース時に prebuilt `libwebrtc_c.a` と `bindings.rs` を GitHub Releases に配布し、`cargo build` 時に自動ダウンロードする
  - `--features source-build` でソースビルドに切り替え可能
  - @voluntas

### misc


## 0.145.1

**リリース日**: 2026-02-17

- [ADD] Ubuntu 24.04 LTS arm64 / Ubuntu 22.04 LTS arm64 をサポート
  - @voluntas
- [ADD] `IceCandidate` と `PeerConnection::add_ice_candidate` の C API / Rust API を追加する
  - `IceCandidate::new`
  - `IceCandidate::sdp_mid`
  - `IceCandidate::sdp_mline_index`
  - `PeerConnection::add_ice_candidate`
  - @voluntas
- [ADD] `RtpParameters` と `RtpSender::get_parameters` / `set_parameters` の C API / Rust API を追加する
  - `RtpParameters`
  - `DegradationPreference`
  - `RtpSender::get_parameters`
  - `RtpSender::set_parameters`
  - `RtpEncodingParameters` の optional フィールド API を `out_has + ポインタ値` 方式に統一
  - @melpon

## 0.145.0

**リリース日**: 2026-02-12

