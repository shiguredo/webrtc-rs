# ログレベルの設定が無効になっている

- Created: 2026-08-25
- Completed: {YYYY-MM-DD} (例: 2024-07-01)
- Branch: hotfix/0.150.4
- Polished: {YYYY-MM-DD} (例: 2024-07-15)
- Milestone: 0.150.4

## 目的

`log::log_to_debug` を呼び出してもログレベルが変わらず、全ログが Info 扱いで
出力される問題を修正する。

## 現状

- `webrtc/src/webrtc_c/rtc_base/logging.cc` の `webrtc_LogMessage_LogToDebug` は
  `webrtc::LogMessage::LogToDebug` を呼ぶだけの旧 API を実装している。
- libwebrtc の `LoggingConfig` 導入コミット (2026-04-15、`1dd7260d65`
  "Introduce explicit configuration for logging initialization"、
  bug: webrtc:42234107) 以降、標準エラーへの出力判定は
  `LoggingConfig::debug_severity()` (デフォルト `LS_INFO`)、構築抑止は
  `LoggingConfig::min_severity()` に移っており、`LogToDebug` が触る
  `g_dbg_sev` は実効閾値に影響しなくなった。
- このコミットは M149 ブランチ (branch-heads/7827) に既に入っており、
  現在使用している m150.7871.3.1 にも含まれる。
- `LogToDebug` には現時点で deprecated 属性が付いていない
  (deprecated は `LogThreads` / `ConfigureLogging` のみ)。
- 結果として zakuro の `--log-level` (`none` / `warning` / `error` / `verbose` の
  指定) が機能せず、Info 以上が常に出力される。

## 設計方針

- libwebrtc の新しい初期化フロー (`InitializeLogging(LoggingConfig)`) に追従する。
- C ラッパーに `webrtc_LogMessage_InitializeLogging(int severity)` を追加し、
  `LoggingConfig` の `debug_severity` と `min_severity` を設定して
  `webrtc::InitializeLogging` を呼ぶ。
- Rust API は `log::initialize_logging(severity: Severity) -> bool` とし、
  旧 `log::log_to_debug` は削除する。呼び出し側はプロセスで最初のログ出力前に
  1 回だけ呼ぶ契約になる。
- 旧 C エクスポート `webrtc_LogMessage_LogToDebug` は削除する
  (Rust API からは利用できなくなる。ラッパー内の引用元はない)。

## 完了条件

- zakuro の `--log-level none` で libwebrtc / sora_sdk / zakuro のログが一切出力されない。
- `--log-level warning` / `--log-level error` / `--log-level verbose` で
  指定したレベル以上のログのみが出力される。
- `--log-level` 未指定 (デフォルト `info`) では従来通り Info 以上が出力される。

## 解決方法

- `webrtc/src/webrtc_c/rtc_base/logging.h` / `logging.cc`:
  `webrtc_LogMessage_InitializeLogging` を追加し、
  `webrtc_LogMessage_LogToDebug` を撤去する。
- `src/rtc_base/logging.rs`: `log::initialize_logging` を追加し、
  `log_to_debug` を削除する。
- `CHANGES.md`: `FIX` として記載し、バージョンを 0.150.4 に上げる。
