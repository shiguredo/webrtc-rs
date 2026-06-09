# webrtc_c のエラー伝達方法を統一する

- Priority: Low
- Polished: 2026-06-05
- Created: 2026-06-05
- Model: Opus 4.8

## 目的

`webrtc_c` の C ラッパー API では、エラーの伝達方法が `webrtc_RTCError_unique*` を戻り値として返す API と、`out` パラメータ（`webrtc_RTCError_unique**`）で返す API とで混在している。
このため、呼び出し側はどちらの方式かを API ごとに把握する必要があり、扱いがばらつく。
エラー伝達の方針を統一し、呼び出し側の扱いをそろえる。

## 優先度根拠

機能的には双方とも動作するため、即座の不具合にはつながらない。
ただし C-API としての一貫性・利用しやすさに関わり、利用側コードの単純化や誤用防止に資する。優先度は Low とする。

## 現状

戻り値でエラーを返す API が存在する。
`webrtc/src/webrtc_c/api/rtp_transceiver_interface.cc:63` の `webrtc_RtpTransceiverInterface_SetCodecPreferences` は、成功時に `nullptr`、失敗時に `webrtc::RTCError` を返す。

```cpp
WEBRTC_EXPORT webrtc_RTCError_unique*
webrtc_RtpTransceiverInterface_SetCodecPreferences(
    struct webrtc_RtpTransceiverInterface* self,
    struct webrtc_RtpCodecCapability_vector* codecs) {
  auto transceiver = reinterpret_cast<webrtc::RtpTransceiverInterface*>(self);
  auto vec = reinterpret_cast<std::vector<webrtc::RtpCodecCapability>*>(codecs);
  auto result = transceiver->SetCodecPreferences(*vec);
  if (result.ok()) {
    return nullptr;
  } else {
    return reinterpret_cast<webrtc_RTCError_unique*>(
        new webrtc::RTCError(result));
  }
}
```

同様に戻り値方式の API として、`webrtc/src/webrtc_c/api/rtp_sender_interface.h:19` の `webrtc_RtpSenderInterface_SetParameters` も `webrtc_RTCError_unique*` を返す宣言になっている。

一方、`out` パラメータでエラーを返す API も存在する。
`webrtc/src/webrtc_c/api/peer_connection_interface.cc:472` の `webrtc_PeerConnectionInterface_CreateDataChannelOrError` は、最終引数 `out_rtc_error`（`webrtc_RTCError_unique**`）にエラーを書き込む。

```cpp
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_CreateDataChannelOrError(
    struct webrtc_PeerConnectionInterface* self,
    const char* label,
    size_t label_len,
    struct webrtc_DataChannelInit* init,
    struct webrtc_DataChannelInterface_refcounted** out_data_channel,
    struct webrtc_RTCError_unique** out_rtc_error) {
```

同じく `out` パラメータ方式の API として、`webrtc/src/webrtc_c/api/peer_connection_interface.cc:497` の `webrtc_PeerConnectionInterface_AddTransceiver` をはじめ、`peer_connection_interface.cc` には `out_rtc_error` を取る API が複数存在する（`webrtc/src/webrtc_c/api/peer_connection_interface.cc:502`、`:524`、`:549`、`:573`、`:669`、`:1162` など）。

このように、エラーのみを返す API は戻り値方式、別の出力値とエラーを併せて返す API は `out` パラメータ方式という傾向はあるものの、`webrtc_c` 全体としてエラー伝達の方針が明文化・統一されていない。

## 設計方針

- `webrtc_c` におけるエラー伝達の方針を一つに定める（例: 値を返さずエラーのみを伝える API は戻り値、別の出力値を伴う API は `out` パラメータ、といった基準を明確化する。あるいはエラーは常に `out` パラメータに統一する、など）。
- 定めた方針に沿って、混在している API のエラー伝達方法をそろえる。
- 変更する場合は呼び出し側（`webrtc/src/whip.c` など）の扱いも合わせて更新し、成功・失敗時の挙動を変えない。

## 完了条件

- `webrtc_c` のエラー伝達方法に統一的な方針が定められている。
- 同種の API 間でエラー伝達方法が一貫している。
- 既存の呼び出し側の成功・失敗時の挙動が変わっていない。
