# whip.cpp で codec の未設定時参照を修正する

- Priority: High
- Created: 2026-06-05
- Model: Opus 4.8
- Polished: 2026-06-06

## 目的

whip.cpp は各 `send_encoding` のコーデックを処理する際、`send_encoding.codec` が
`std::optional<webrtc::Codec>` であるにもかかわらず、値の有無を確認せずに
`send_encoding.codec->name` でログ出力している。`codec` が値を持たない場合、
`std::optional::operator->` は未定義動作となりクラッシュする。参照前に `codec` の
有無を確認するように修正する。

## 優先度根拠

未定義動作はプロセスのクラッシュに直結する堅牢性の欠陥である。`send_encodings` の内容次第で
発生し得るため、優先度は High とする。

## 現状

`webrtc/src/whip.cpp:696-708` — `send_encoding.codec->name` を参照しているが、
`send_encoding.codec` が値を持つかどうかの確認はその直後の L701
`if (send_encoding.codec && ...)` で初めて行われる。null チェックの前に `->name` を
参照している:

```cpp
for (auto& send_encoding : video_init.send_encodings) {
  RTC_LOG(LS_WARNING)
      << "send_encoding: " << send_encoding.codec->name;  // ← L698: 未定義動作
  for (auto& codec : media_desc->codecs()) {
    RTC_LOG(LS_WARNING) << "codec: " << codec.name;
    if (send_encoding.codec &&                              // ← L701: 遅すぎる
        webrtc::IsSameRtpCodec(codec, *send_encoding.codec)) {
      ...
    }
  }
}
```

## 設計方針

- `send_encoding.codec->name` を参照する前に `send_encoding.codec.has_value()` を確認する
- `codec` が未設定の場合は `"(no codec)"` などの安全な文字列をログに出力する
- L701 の `if (send_encoding.codec && ...)` 内の処理に影響は与えない

## 完了条件

- `send_encoding.codec` が値を持たない（`std::nullopt`）場合でもクラッシュしない
- `codec` 未設定時は安全な代替文字列がログ出力される
