# webrtc_c のログ固定バッファによる切り詰めを解消する

- Priority: Low
- Created: 2026-06-05
- Model: Claude Opus 4.8
- Branch: feature/fix-webrtc-c-log-truncation-buffer

## 目的

`webrtc_c` のログ整形処理が 4096 バイトの固定スタックバッファを使っており、これを超える長さのメッセージは切り詰められて出力される。
長いログ（例: SDP のような長文）が途中で欠落すると、ログを使った調査が困難になる。
切り詰めが起きないように、または切り詰めを明示的に扱うように改善する。

## 優先度根拠

機能そのものには影響せず、ログ出力時の情報欠落にとどまる。
ただし長文ログの欠落はデバッグ時の障害になり得るため、整理しておく価値はある。優先度は Low とする。

## 現状

`webrtc/src/webrtc_c/rtc_base/logging.cc:16` で、ログバッファのサイズが 4096 バイト固定として定義されている。

```cpp
#define WEBRTC_LOG_BUFFER_SIZE 4096
```

`webrtc/src/webrtc_c/rtc_base/logging.cc:41` の `webrtc_LogMessage_Print` は、このサイズのスタックバッファに `vsnprintf` で整形している。

```cpp
WEBRTC_EXPORT void webrtc_LogMessage_Print(int severity,
                                           const char* file,
                                           int line,
                                           const char* fmt,
                                           ...) {
  char buf[WEBRTC_LOG_BUFFER_SIZE];
  va_list args;
  va_start(args, fmt);
  vsnprintf(buf, sizeof(buf), fmt, args);
  va_end(args);

  RTC_LOG_FILE_LINE(static_cast<webrtc::LoggingSeverity>(severity), file, line)
      << buf;
}
```

`vsnprintf` は出力先サイズを超える分を書き込まないため、整形後のメッセージが 4096 バイト（終端を含む）を超えると以降が切り捨てられ、ログが欠落する。

## 設計方針

- `vsnprintf` が返す「本来必要な長さ」を利用して、必要な長さ分のバッファを動的に確保し直してから整形し直す、といった方法でメッセージ全体を出力できるようにする。
- もしくは、固定長を維持したうえで切り詰めが発生したことを明示的に扱う（切り詰めを示す表示を付けるなど）方針も検討する。
- 入力バイナリのデコードではなくログ整形という確定的な用途であり、必要長は `vsnprintf` の戻り値から判明するため、その値に基づいて確保する。

## 完了条件

- 4096 バイトを超える長さのログメッセージが、欠落せずに出力される（または切り詰めが明示的に扱われる）。
- 既存の短いメッセージの出力結果は変わらない。
