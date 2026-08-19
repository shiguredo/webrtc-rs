# webrtc_c のログ固定バッファによる切り詰めを緩和する

- Priority: Low
- Polished: 2026-08-12
- Created: 2026-06-05
- Model: Opus 4.8

## 目的

`webrtc_c` のログ整形処理が 4096 バイトの固定スタックバッファを使っており、これを超える長さのメッセージは切り詰められて出力される。長いログ（例: SDP のような長文）が途中で欠落すると、ログを使った調査が困難になる。C++ 側 (`webrtc::LogMessage`) にはメッセージ長の制限が無いため、固定バッファを動的確保に置き換えて、切り詰めなく全メッセージを出力する。

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

採用方針: **`vsnprintf` の二度呼による動的確保で、切り詰めなしに全メッセージを出力する**。

libwebrtc の C++ 側 (`webrtc::LogMessage`) にはメッセージ長の制限が無い。

- `LogMessage` はメッセージを `StringBuilder print_stream_` に蓄積する（`rtc_base/logging.h` の `LogMessage::print_stream_`）。`StringBuilder` は `std::string` ベースで動的リサイズされる（`rtc_base/strings/string_builder.cc` の `StringBuilder::operator<<`）
- 出力行の組み立ても `LogLineRef::DefaultLogLine()` が `StringBuilder` で行う（`rtc_base/logging.cc` の `LogLineRef::DefaultLogLine`）
- stderr への出力は `fprintf(stderr, "%s", msg_str.c_str())` であり、長さ制限が無い（`rtc_base/logging.cc` の `LogMessage::OutputToDebug`）
- Android のみ logcat の行長制限対策として `kMaxLogLineSize` (1024-60) ごとに分割して出力するが、分割するだけで欠落はしない（`rtc_base/logging.cc` の `LogMessage::OutputToDebug`）

したがって切り詰めの原因は webrtc_c 側の `webrtc_LogMessage_Print` が `vsnprintf` を 4096 バイトの固定スタックバッファに書き込んでいることだけであり、動的確保化の効果は下流で打ち消されない。

実装の流れは以下のとおり:

1. `va_copy` で可変長引数を複製し、`vsnprintf(nullptr, 0, ...)` で整形後の必要サイズを求める
2. 必要サイズの `std::string` を確保して `vsnprintf` で整形する
3. 整形結果を `RTC_LOG_FILE_LINE(...) << message` で出力する
4. `vsnprintf` が負の値を返した場合は整形失敗として、ログ出力を諦める（または空文字列を出力する）

この二度呼パターンは libwebrtc 自身の `StringBuilder::AppendFormat`（`rtc_base/strings/string_builder.cc` の `StringBuilder::AppendFormat`）が既に採用している実装である。動的確保失敗時のプロセス終了リスクは、下流の `RTC_LOG` が `StringBuilder`（std::string ベース）で既に動的確保しているため、本変更で本質的に変わらない。

`WEBRTC_LOG_BUFFER_SIZE` マクロと固定スタックバッファは削除する。`std::string` を使うため `#include <string>` を追加する（`rtc_base/logging.h` が間接的に include しているが、直接 include して明示する）。

### 代替案（参考）: バッファサイズの拡張

固定バッファを `4096` → `65536` (64KB) に拡張する案。変更はマクロ定数 1 行で完了しリスクが最小だが、64KB を超えるメッセージは依然として切り詰められる。64KB のスタックバッファは呼び出しスレッドのスタックを 64KB 消費するため、スタックサイズの小さいスレッド（macOS の pthread デフォルト 512KB 等）でログを呼ぶ場合は注意が必要になる。切り詰めが根本解消されないため、本 issue では採用しない。

## テスト戦略

- `src/tests.rs` にテストを追加し、4096 バイトを超えるメッセージと 65536 バイトを超えるメッセージ（例: 70000 バイト）が切り詰められずに出力されることを確認する。ログは C++ 側（`webrtc::LogMessage`）が stderr へ直接書き込むため、Rust テストハーネスの標準的な出力キャプチャでは捕捉できない。テストバイナリをサブプロセスとして起動して stderr を捕捉し、出力内容を照合する方式で検証する。テストメッセージは `%` を含まない内容（例: 'A' の繰り返し）で生成する（`webrtc_LogMessage_Print` が fmt として `vsnprintf` に渡すため）。テスト実行時は `webrtc_LogMessage_LogToDebug` でログ出力を有効化しておく必要がある（libwebrtc のリリースビルドではデフォルトのログレベルが無効のため）
- 既存の短いメッセージの出力結果が変わらないことを確認する（動的確保化は整形結果に影響しないため。短いメッセージもサブプロセスで出力し、メッセージ全体が末尾まで含まれることを照合する）
- 出力される全メッセージ長が入力メッセージ長と一致することを照合する（切り詰めの有無を長さで検証する）

## 完了条件

- 固定バッファが無くなり、`vsnprintf` の二度呼による動的確保で全メッセージが切り詰めなしに出力される
- `WEBRTC_LOG_BUFFER_SIZE` マクロと固定スタックバッファが削除されている
- `vsnprintf` が負の値を返した場合のエラーハンドリングが実装されている
- テスト戦略に記載の検証が完了している（4096 バイト超・65536 バイト超のメッセージが切り詰められずに出力されること、既存の短いメッセージの出力結果が変わらないこと）
- `CHANGES.md` の `## develop` セクションに追記されている
