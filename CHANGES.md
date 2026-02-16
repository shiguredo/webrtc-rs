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
  - @voluntas

### misc

## 0.145.0

**リリース日**: 2026-02-12

