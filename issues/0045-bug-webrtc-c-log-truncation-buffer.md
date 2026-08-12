# webrtc_c のログ固定バッファによる切り詰めを緩和する

- Priority: Low
- Polished: 2026-08-12
- Created: 2026-06-05
- Model: Opus 4.8

## 目的

`webrtc_c` のログ整形処理が 4096 バイトの固定スタックバッファを使っており、これを超える長さのメッセージは切り詰められて出力される。長いログ（例: SDP のような長文）が途中で欠落すると、ログを使った調査が困難になる。切り詰めの発生範囲を縮小するため、バッファサイズを拡張する。

## 優先度根拠

機能そのものには影響せず、ログ出力時の情報欠落にとどまる。ただし長文ログの欠落はデバッグ時の障害になり得るため、対処しておく価値はある。優先度は Low とする。

## 現状

`webrtc/src/webrtc_c/rtc_base/logging.cc` で、ログバッファのサイズが 4096 バイト固定として `WEBRTC_LOG_BUFFER_SIZE` マクロで定義されている。

```cpp
#define WEBRTC_LOG_BUFFER_SIZE 4096
```

`webrtc/src/webrtc_c/rtc_base/logging.cc` の `webrtc_LogMessage_Print` は、このサイズのスタックバッファに `vsnprintf` で整形している。

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

`vsnprintf` は出力先サイズを超える分を書き込まないため、整形後のメッセージが 4096 バイト（終端を含む）を超えると以降が切り詰められ、ログが欠落する。実際に長文になり得るログは、`webrtc/src/whep.c` の HTTP レスポンス全体を出力する箇所（`RTC_LOG_INFO("Received response: %s", resp)`）が代表例である。

## 設計方針

採用方針: **バッファサイズを `4096` → `65536` (64KB) に拡張する**。

理由:
- 動的確保（後述の代替案）は複雑さが増し、メモリ不足時に例外送出（libwebrtc は例外無効ビルドが標準のため `std::terminate` によるプロセス終了が主）に至るリスクがある
- スタックバッファのままであり、wrapper 側で新たなメモリ確保を行わない
- 変更はマクロ定数 1 行で完了し、リスクが最小

64KB を超えるメッセージは依然として切り詰められるが、実際の利用経路（HTTP レスポンスログ等）で 64KB を超えるログは想定されないため許容する。なお `rtc_base/logging.h` の `LogMessage` には固定長バッファによる長さ制限の宣言が無く、メッセージは `std::string` ベースの `StringBuilder` に蓄積される実装のため、バッファ拡張の効果は下流で打ち消されない。64KB のスタックバッファは呼び出しスレッドのスタックを 64KB 消費するため、スタックサイズの小さいスレッド（macOS の pthread デフォルト 512KB 等）でログを呼ぶ場合は注意が必要である。

### 代替案（参考）: vsnprintf の二度呼による動的確保

64KB でも不足する場合の参考として、`vsnprintf(nullptr, 0, ...)` で必要なサイズを求めてから動的確保する方式がある。ただし `std::vector` の確保失敗時は上記のとおりプロセス終了に至るリスクがあるため、本 issue では採用しない。なお採用する場合は `#include <vector>` の追加が必要になる。

## テスト戦略

- `src/tests.rs` にテストを追加し、4096 バイトを超えるメッセージ（境界値 4097 バイト）と 65536 バイト近傍のメッセージ（境界値 65500 バイト）が切り詰められずに出力されることを確認する。ログは C++ 側（`webrtc::LogMessage`）が stderr へ直接書き込むため、Rust テストハーネスの標準的な出力キャプチャでは捕捉できない。テストバイナリをサブプロセスとして起動して stderr を捕捉し、出力内容を照合する方式で検証する。テストメッセージは `%` を含まない内容（例: 'A' の繰り返し）で生成する（`webrtc_LogMessage_Print` が fmt として `vsnprintf` に渡すため）。テスト実行時は `webrtc_LogMessage_LogToDebug` でログ出力を有効化しておく必要がある（libwebrtc のリリースビルドではデフォルトのログレベルが無効のため）
- 既存の短いメッセージの出力結果が変わらないことを確認する（バッファサイズの変更は整形結果に影響しないため。短いメッセージもサブプロセスで出力し、メッセージ全体が末尾まで含まれることを照合する）

## 完了条件

- ログバッファサイズが 65536 バイトに拡張されている
- テスト戦略に記載の検証が完了している（4096 バイト超のメッセージが切り詰められずに出力されること、既存の短いメッセージの出力結果が変わらないこと）
- `CHANGES.md` の `## develop` セクションに追記されている
