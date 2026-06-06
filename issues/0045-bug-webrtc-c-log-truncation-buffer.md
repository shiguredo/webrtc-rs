# webrtc_c のログ固定バッファによる切り詰めを解消する

- Priority: Low
- Polished: 2026-06-06
- Created: 2026-06-05
- Model: Opus 4.8

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

### 推奨案: バッファサイズの拡張

動的確保の複雑さとメモリ不足時のログ喪失リスクを考慮し、第一段階として
バッファサイズを `4096` → `65536` (64KB) に拡張する。

- SDP 等の長文ログは通常数 KB〜十数 KB であり、64KB で十分カバーできる
- スタックバッファのままであり、メモリ不足時にもログ関数自体はクラッシュしない
- 変更は 1 行（マクロ定数の変更）で完了し、リスクが最小

### 代替案: vsnprintf の二度呼びによる動的確保

より厳密な対応が必要な場合は以下を検討する（本 issue では推奨案を優先）:

```cpp
WEBRTC_EXPORT void webrtc_LogMessage_Print(int severity,
                                           const char* file,
                                           int line,
                                           const char* fmt,
                                           ...) {
  va_list args;
  va_start(args, fmt);
  int needed = vsnprintf(nullptr, 0, fmt, args);
  va_end(args);
  if (needed < 0) {
    return;  // フォーマットエラー
  }
  std::vector<char> buf(static_cast<size_t>(needed) + 1);
  va_start(args, fmt);
  vsnprintf(buf.data(), buf.size(), fmt, args);
  va_end(args);
  RTC_LOG_FILE_LINE(static_cast<webrtc::LoggingSeverity>(severity), file, line)
      << buf.data();
}
```

ただし、ログ関数はメモリ不足時にも呼ばれうるため、`std::vector` 確保失敗時に
`std::bad_alloc` を送出してログ関数自体がクラッシュするリスクがある。

## テスト戦略

- 4096 バイトを超えるメッセージを実際に出力し、切り詰められていないことを確認する
- 既存の短いメッセージの出力結果が変わらないことを確認する

## 完了条件

- ログバッファサイズが 65536 バイトに拡張されている、または動的確保方式が実装されている
- 長文ログが欠落せずに出力される
- 既存の短いメッセージの出力結果は変わらない
