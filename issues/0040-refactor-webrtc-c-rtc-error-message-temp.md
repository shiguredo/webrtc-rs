# webrtc_RTCError_message の一時文字列二重構築を解消する

- Priority: Low
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/refactor-webrtc-c-rtc-error-message-temp

## 目的

`webrtc_RTCError_message` が、メッセージ文字列の長さを得るためだけに一時 `std::string` を構築しており、`message()` の呼び出しと合わせて文字列の評価が無駄に重複している。
この無駄を解消し、文字列の構築を 1 回にまとめる。

## 優先度根拠

エラー取得時にのみ通る経路であり、性能上の影響は軽微である。
機能的な不具合もない。
そのため優先度は Low とする。ただし「薄いラッパーに無駄な処理を残さない」観点から整理しておく価値はある。

## 現状

`webrtc/src/webrtc_c/api/rtc_error.cc:23` の `webrtc_RTCError_message` は、出力メッセージのポインタを返すために `err->message()` を呼び、さらに長さを求めるために `std::string(err->message()).size()` でもう一度メッセージから一時 `std::string` を構築している。

```cpp
WEBRTC_EXPORT void webrtc_RTCError_message(struct webrtc_RTCError* self,
                                           const char** out_message,
                                           size_t* out_len) {
  auto err = reinterpret_cast<webrtc::RTCError*>(self);
  if (out_message != nullptr) {
    *out_message = err->message();
  }
  if (out_len != nullptr) {
    *out_len = std::string(err->message()).size();
  }
}
```

`out_message` と `out_len` の双方を求める場合、`err->message()` が 2 回呼ばれ、そのうち 1 回は長さ算出のためだけに一時 `std::string` を構築している。

## 設計方針

- メッセージのポインタを一度だけ取得し、その値をもとに `out_message` と `out_len` の両方を埋める。
- 長さの算出に一時 `std::string` を構築しない方法（取得済みポインタからの長さ計算）に置き換え、文字列にかかわる評価を 1 回に抑える。
- 既存の引数仕様（`out_message`・`out_len` がそれぞれ NULL の場合は書き込まない）は維持する。

## 完了条件

- `webrtc_RTCError_message` における一時 `std::string` の構築が行われなくなっている。
- `out_message` と `out_len` が現状と同じ値（ポインタと文字列長）を返す。
