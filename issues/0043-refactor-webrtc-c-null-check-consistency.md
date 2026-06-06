# webrtc_c の null チェックの一貫性を整える

- Priority: Low
- Polished: 2026-06-05
- Created: 2026-06-05
- Model: Opus 4.8

## 目的

`webrtc_c` の C ラッパー API 間で、ポインタ引数に対する null チェックの有無が統一されていない。
同種のセッター系 API でありながら、ある API はチェックせず、ある API もチェックしないなど方針が定まっていないため、利用側から見た振る舞いの予測が難しい。
null チェックの方針を明確にし、同種の API で一貫させる。

## 優先度根拠

現状の利用箇所では null が渡る経路は確認されていないため、即座の不具合にはつながりにくい。
ただし C-API として境界の堅牢性に関わるため、方針を整えておく意義はある。優先度は Low とする。

## 現状

`webrtc/src/webrtc_c/api/rtp_transceiver_interface.cc:46` の `webrtc_RtpTransceiverInit_set_send_encodings` は、引数 `encodings` を null チェックせずにそのまま `reinterpret_cast` してデリファレンス（`*vec`）している。

```cpp
WEBRTC_EXPORT void webrtc_RtpTransceiverInit_set_send_encodings(
    struct webrtc_RtpTransceiverInit* self,
    struct webrtc_RtpEncodingParameters_vector* encodings) {
  auto init = reinterpret_cast<webrtc::RtpTransceiverInit*>(self);
  auto vec =
      reinterpret_cast<std::vector<webrtc::RtpEncodingParameters>*>(encodings);
  init->send_encodings = *vec;
}
```

`webrtc/src/webrtc_c/api/data_channel_interface.cc:152` の `webrtc_DataChannelInit_set_protocol` も、引数 `protocol` を null チェックせずに `std::string(protocol, protocol_len)` を構築している。

```cpp
WEBRTC_EXPORT void webrtc_DataChannelInit_set_protocol(
    struct webrtc_DataChannelInit* self,
    const char* protocol,
    size_t protocol_len) {
  auto init = reinterpret_cast<webrtc::DataChannelInit*>(self);
  init->protocol = std::string(protocol, protocol_len);
}
```

いずれも値を設定するセッター系 API でありながら、ポインタ引数に対する扱い（null をどう扱うか）が API ごとに明示されておらず、方針として一貫していない。

## 設計方針

- セッター系 C-API におけるポインタ引数の null の扱いについて、方針を一つに定める（例: null は不正入力として扱い早期 return する、もしくは呼び出し側が非 null を保証する前提を統一的に明記する）。
- 定めた方針に従って、同種の API 間で null チェックの有無を揃える。
- 既存の正常系の挙動は変えない。

## 完了条件

- セッター系の同種 C-API において、ポインタ引数の null チェックの方針が統一されている。
- 上記 2 つの API を含め、同種の API のチェック有無が一貫している。
