# カスタム LogSink を設定可能にする

- Created: 2026-08-26
- Completed: {YYYY-MM-DD}
- Branch: feature/add-custom-log-sink
- Polished: {YYYY-MM-DD}

## 目的

アプリケーションが自前のログ処理 (ファイル出力、バッファリング、外部システムへの転送など) を行えるようにする。現状はログの出力先が標準エラー (`log_to_stderr`) に限られており、アプリケーションが libwebrtc / webrtc-rs のログを加工・転送する手段がない。

## 現状

- `webrtc::LoggingConfig` の C ラッパー `webrtc_LoggingConfig` (`webrtc/src/webrtc_c/rtc_base/logging.h` / `logging.cc`) は 7 フィールド (min_severity / debug_severity / log_thread / log_timestamp / log_queue_name / log_to_stderr / log_prefix) を公開しているが、`sinks` は公開していない
- libwebrtc の `webrtc::LoggingConfig::AddSink(std::unique_ptr<LogSink>)` でカスタム sink を登録できるが、`webrtc::LogSink` は抽象クラス (pure virtual `OnLogMessage(const std::string&)`) のため C から直接インスタンス化できない
- Rust API `log::LoggingConfig` (`src/rtc_base/logging.rs`) も同様に sink を設定できない

## 設計方針

- 既存のコールバック型ハンドラのパターン (`webrtc_SSLCertificateVerifier` / `SSLCertificateVerifierHandler`、`webrtc/src/webrtc_c/rtc_base/ssl_certificate.{h,cc}` と `src/rtc_base/ssl_certificate.rs`) に倣う
- C 側:
  - `webrtc_LogSink_cbs` 構造体 (コールバック関数ポインタ + `OnDestroy`、null 非許容) と `webrtc_LogSink_new(const struct webrtc_LogSink_cbs* cbs, void* user_data)` を追加し、`struct webrtc_LogSink_unique*` を返す
  - `webrtc::LogSink` を継承する C++ 実装クラス `LogSinkImpl` を追加し、libwebrtc から呼ばれる `OnLogMessage` を C コールバックへディスパッチする
  - ログメッセージと重大度を C コールバックへ渡すため、`LogSinkImpl` は `OnLogMessage(const std::string&)` (純粋仮想) と `OnLogMessage(const std::string&, LoggingSeverity)` の両方をオーバーライドする (libwebrtc の `LogMessage` は `OnLogMessage(const LogLineRef&)` 経由で届き、既定実装が `(メッセージ, 重大度)` のオーバーロードへ集約する)
  - `webrtc_LoggingConfig_AddSink(struct webrtc_LoggingConfig* self, struct webrtc_LogSink_unique* sink)` を追加し、`LoggingConfig::AddSink` へ所有権を移して委譲する
- Rust 側:
  - `log::LogSinkHandler` trait と `log::LogSink` (`SSLCertificateVerifier` と同型のラッパー) を追加する
  - `log::LoggingConfig::add_sink(&mut self, sink: LogSink)` を追加する
  - コールバックで受けた `(message, severity)` は `&str` と `log::Severity` に変換してハンドラへ渡す
- `webrtc::LoggingConfig::sinks()` (登録済み sink の参照取得) は用途が無いため C ラッパーは提供しない

## 完了条件

- C 側で `webrtc_LogSink_new` と `webrtc_LoggingConfig_AddSink` を使ってカスタム sink を登録できる
- Rust 側で `log::LoggingConfig::add_sink` に `LogSinkHandler` を登録でき、ログ出力時に `on_log_message` がメッセージと重大度付きで呼ばれる
- config の破棄時に `OnDestroy` 経由で Rust 側のリソース (`Box<dyn LogSinkHandler>`) が正しく解放される (二重解放・リークなし)

## 解決方法

- 上記の設計方針に従い、C API (`webrtc/src/webrtc_c/rtc_base/logging.{h,cc}`) と Rust API (`src/rtc_base/logging.rs`) を追加する
- `src/tests.rs` に次のテストを追加する
  - `LogSinkHandler` を登録した config で `log::print` を呼び、コールバックでメッセージと重大度が受け取れること
  - ハンドラの drop 時に `OnDestroy` が呼ばれ、二重解放されないこと
