# FileRotatingLogSink と CallSessionFileRotatingLogSink を利用可能にする

- Created: 2026-08-27
- Completed: {YYYY-MM-DD}
- Branch: feature/add-file-rotating-log-sink
- Polished: {YYYY-MM-DD}

## 目的

- 0088 で追加した LogSink 設定機能 (`log::LoggingConfig::add_sink`) は、コールバック型のカスタム sink を前提としており、実際に add_sink へ渡せる具象の LogSink が存在しない
- libwebrtc が提供する `webrtc::FileRotatingLogSink` と `webrtc::CallSessionFileRotatingLogSink` (`rtc_base/log_sinks.h`) を Rust から生成して add_sink へ渡せるようにし、アプリケーションがログをファイルへローテーション出力する機能を追加実装なしで使えるようにする

## 現状

- 0088 は `webrtc_LoggingConfig_AddSink` (`webrtc/src/webrtc_c/rtc_base/logging.h` / `logging.cc`) と Rust API `log::LoggingConfig::add_sink` (`src/rtc_base/logging.rs`) を用意しているが、渡せる sink はコールバック型のカスタム実装のみである
- libwebrtc は `rtc_base/log_sinks.h` に次の 2 クラスを提供しているが、C ラッパーも Rust バインディングもない
  - `webrtc::FileRotatingLogSink` : `log_dir_path` / `log_prefix` / `max_log_size` / `num_log_files` を指定し、`FileRotatingStream` でファイルへ書き込む
  - `webrtc::CallSessionFileRotatingLogSink` : `log_dir_path` / `max_total_log_size` を指定し、`CallSessionFileRotatingStream` で書き込む
- どちらも `webrtc::LogSink` を継承し、`Init()` を呼んでから `LoggingConfig::AddSink` へ渡す必要がある
- コンストラクタ引数の制約 (`num_log_files > 1`、`max_log_size > 0`) は C++ 側 (`FileRotatingStream`) で `RTC_DCHECK_GT` により検証される (Debug ビルド時)。`Init()` 前にログを書こうとすると stderr へ警告を出して何も書き込まない (`log_sinks.cc` の `OnLogMessage`)

## 設計方針

- 0088 の `webrtc_LogSink_unique` / `webrtc_LoggingConfig_AddSink` を再利用し、実装は `webrtc/src/webrtc_c/rtc_base/` に `log_sinks.h` / `log_sinks.cc` を新規追加する
- C 側 (`webrtc_c/rtc_base/log_sinks.h` / `log_sinks.cc`):
  - `webrtc_FileRotatingLogSink_new` と `webrtc_CallSessionFileRotatingLogSink_new` を追加し、`struct webrtc_LogSink_unique*` で返す。内部では派生型 (`webrtc::FileRotatingLogSink` 等) を `webrtc::LogSink` へ upcast して返す
  - 初期化を `webrtc_FileRotatingLogSink_Init(struct webrtc_LogSink_unique*) -> bool` / `webrtc_CallSessionFileRotatingLogSink_Init(...) -> bool` として公開する。`Init()` は派生型にしか存在しないため、内部で `static_cast` により downcast して呼ぶ。downcast の前提として、対象の sink が本ヘッダのコンストラクタで生成されたことをヘッダに明記する
  - `DisableBuffering()` を使いたい場合も同様の関数を公開する
  - 生成した sink は `webrtc_LoggingConfig_AddSink` へそのまま渡せる (所有権が config へ移る)。引数の妥当性は C++ 側の検証に委ねる
- Rust 側:
  - `src/rtc_base/log_sinks.rs` を新規追加し、`log::LoggingConfig::add_sink` へ渡せるラッパーを公開する (`src/rtc_base/mod.rs` へ登録)
- ヘッダは `webrtc/src/webrtc_c.h` の `#include "webrtc_c/rtc_base/log_sinks.h"` を追加する。`.cc` は `webrtc/CMakeLists.txt` の `webrtc_c` ソース一覧へ `logging.cc` と同様に追記する
- 本 issue は 0088 (`webrtc_LoggingConfig_AddSink` / `webrtc_LogSink_unique`) に依存する
- テスト方針: `src/tests.rs` に、`FileRotatingLogSink` / `CallSessionFileRotatingLogSink` を生成して `Init()`、`add_sink` 後に `log::print` を呼び、一時ディレクトリ配下のファイルへ書き込まれることを検証する

## 完了条件

- Rust から `FileRotatingLogSink` / `CallSessionFileRotatingLogSink` を生成し、`Init()` 後 `log::LoggingConfig::add_sink` に渡せる
- `log::print` を呼ぶと、指定したディレクトリ配下のローテーション対象ファイルへメッセージが書き込まれる
