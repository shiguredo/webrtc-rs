# RtpReceiver に stream_ids の取得口を追加する

- Created: 2026-09-04
- Completed: {YYYY-MM-DD}
- Branch: feature/add-rtp-receiver-stream-ids
- Polished: {YYYY-MM-DD}

## 目的

受信トラックに紐づく Stream ID 群を Rust 側から取得できるようにする。Sora の利用ではトラックに Stream ID が 1 つだけ紐づき、上位層でトラックと Stream の対応付けに使うため、取得口がないと同等の情報を提供できない。

## 現状

- `src/api/rtp.rs` の `RtpReceiver` は `track` と `set_frame_transformer` だけを持ち、`stream_ids` に相当する取得口がない
- `webrtc/src/webrtc_c/api/rtp_receiver_interface.h` と `rtp_receiver_interface.cc` の C API も `track` と `SetFrameTransformer` だけで、受信器の Stream ID 群を露出していない
- 送信側の初期値としては `src/api/rtp.rs` の `RtpTransceiverInit` が `stream_ids` を持つが、受信後の取得には使えない

## 設計方針

- `webrtc_c` に受信器が保持する Stream ID 群を返す C API を追加し、結果を `std_string_vector` の複製として返す
- `src/api/rtp.rs` の `RtpReceiver` に `stream_ids` を追加し、所有型の `StringVector` を返す（`RtpParameters::encodings` が所有型のベクタを返す先例に倣う）
- 要素が空の場合も空ベクタとして返し、`None` やエラーにはしない

## 完了条件

- `RtpReceiver` から Stream ID 群を `StringVector` として取得できること
- `cargo test` と `cargo clippy --all-targets -- -D warnings` が成功すること

## 解決方法

（詳細は polish / 実装時に確定する）
