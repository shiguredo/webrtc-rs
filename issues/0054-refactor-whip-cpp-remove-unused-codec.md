# whip.cpp の未使用コーデック変数とコメントアウトを削除する

- Priority: Low
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/refactor-whip-cpp-remove-unused-codec

## 目的

`webrtc/src/whip.cpp` の `WhipClient::Run` には、実際には使われない VP9 / H264 / H265 のコーデック変数と、過去の実装をコメントとして残したコードが存在する。これらを削除して、実際に利用されている AV1 のみが残る状態にし、サンプルの意図を明確にする。

## 優先度根拠

ビルドや動作には影響しないデッドコードであり、機能上の問題はないため Low とする。ただしサンプルコードとして読み手を混乱させるため、整理する価値はある。

## 現状

レビュー時点で実コードと `rg` による参照状況を確認済み。

### 未使用コーデック変数 vp9_codec / h264_codec / h265_codec

`whip.cpp:1121-1143` で 4 つのコーデック変数が宣言・設定される。

```cpp
    webrtc::RtpCodecCapability vp9_codec;
    webrtc::RtpCodecCapability av1_codec;
    webrtc::RtpCodecCapability h264_codec;
    webrtc::RtpCodecCapability h265_codec;
    vp9_codec.kind = webrtc::MediaType::VIDEO;
    vp9_codec.name = "VP9";
    vp9_codec.parameters["profile-id"] = "0";
    vp9_codec.clock_rate = 90000;
    av1_codec.kind = webrtc::MediaType::VIDEO;
    av1_codec.name = "AV1";
    av1_codec.clock_rate = 90000;
    av1_codec.parameters["level-idx"] = "5";
    av1_codec.parameters["profile"] = "0";
    av1_codec.parameters["tier"] = "0";
    h264_codec.kind = webrtc::MediaType::VIDEO;
    h264_codec.name = "H264";
    h264_codec.clock_rate = 90000;
    h264_codec.parameters["profile-level-id"] = "42001f";
    h264_codec.parameters["level-asymmetry-allowed"] = "1";
    h264_codec.parameters["packetization-mode"] = "1";
    h265_codec.kind = webrtc::MediaType::VIDEO;
    h265_codec.name = "H265";
    h265_codec.clock_rate = 90000;
```

`rg "vp9_codec" whip.cpp` / `rg "h264_codec" whip.cpp` / `rg "h265_codec" whip.cpp` で確認したところ、`vp9_codec` は宣言・設定のみで実コードからの参照がない（参照ゼロを確認）。`h264_codec` と `h265_codec` は `send_encodings[N].codec = ...` への代入がすべてコメントアウトされており（下記）、実コードからの参照がない（参照ゼロを確認）。実際に `send_encodings[N].codec` へ代入されるのは `av1_codec` のみである（`whip.cpp:1155`、`1160`、`1166`）。

### コメントアウトされたコーデック選択コード

`whip.cpp:642-665` に、`SetCodecPreferences` 用の旧コーデック選択ループがコメントとして残っている。

```cpp
      //for (const webrtc::RtpCodecCapability& codec : cap.codecs) {
      //  if (codec.name == "H264") {
      //    codecs.push_back(codec);
      //    break;
      //  }
      //}
```

（同様に H265 / VP9 / AV1 のブロックが続く）

`whip.cpp:1152-1165` にも、`send_encodings[N].codec` への代入の旧実装がコメントとして残っている。

```cpp
    // send_encodings[0].codec = av1_codec;
    // send_encodings[0].scalability_mode = "L1T2";
    // send_encodings[0].codec = h264_codec;
    send_encodings[0].codec = av1_codec;
```

## 設計方針

- 未使用の `vp9_codec` / `h264_codec` / `h265_codec` の宣言と設定（`whip.cpp:1121-1143` の該当行）を削除し、実際に使われる `av1_codec` のみを残す。
- `whip.cpp:642-665` および `whip.cpp:1152-1165` のコメントアウト済み旧コードを削除する。
- 削除後も AV1 を使った既存の挙動が変わらないことを確認する。

## 完了条件

- 未使用コーデック変数 `vp9_codec` / `h264_codec` / `h265_codec` が除去される。
- 上記のコメントアウト済み旧コードが除去される。
- `whip` の挙動が変わらないこと。
