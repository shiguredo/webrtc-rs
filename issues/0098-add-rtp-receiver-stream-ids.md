# RtpReceiver に stream_ids の取得口を追加する

- Created: 2026-09-04
- Completed: {YYYY-MM-DD}
- Branch: feature/add-rtp-receiver-stream-ids
- Polished: 2026-09-04

## 目的

受信器が保持する Stream ID 群を Rust 側から取得できるようにする。上位層ではトラックと Stream の対応付けに Stream ID を使うため、取得口がないと同等の情報を提供できない。

## 現状

- `src/api/rtp.rs` の `RtpReceiver` は `track` と `set_frame_transformer` だけを持ち、`stream_ids` に相当する取得口がない
- `webrtc/src/webrtc_c/api/rtp_receiver_interface.h` と `rtp_receiver_interface.cc` の C API も `track` と `SetFrameTransformer` だけで、受信器の Stream ID 群を露出していない
- 送信側の初期値としては `src/api/rtp.rs` の `RtpTransceiverInit` が `stream_ids` を持つが、受信後の取得には使えない

## 設計方針

- `webrtc_c` に受信器が保持する Stream ID 群を返す C API を追加する（例： `webrtc_RtpReceiverInterface_stream_ids`）。 libwebrtc の `RtpReceiverInterface::stream_ids` の値返し結果を、新規確保した `std_string_vector` に複製して返す。所有権は呼び出し側へ移し、Rust 側が破棄する。`webrtc_RtpTransceiverInit_get_stream_ids` のような内部ベクタへの借用は使わない
- `std_string_vector` に複製関数がなければ追加する（`webrtc_RtpEncodingParameters_vector_clone` が `new` で複製する先例に倣う）
- `src/cxxstd.rs` の `StringVector` に FFI ポインタから所有権を引き受ける口を追加した上で、`src/api/rtp.rs` の `RtpReceiver` に `stream_ids` を追加し、所有型の `StringVector` を返す
- 読み取り専用の値返し getter として、C 関数の `self` は `const` 扱いにする
- 要素が空の場合も空ベクタとして返し、`None` やエラーにはしない

## 完了条件

- `RtpReceiver` から取得した Stream ID 群が、空の場合に空ベクタとなり、非空の場合に内容が一致することをテストで確認できること
- `cargo test` と `cargo clippy --all-targets -- -D warnings` が成功すること

## 解決方法

（詳細は polish / 実装時に確定する）
