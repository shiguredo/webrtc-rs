# FileRotatingLogSink と CallSessionFileRotatingLogSink を利用可能にする

- Created: 2026-08-27
- Completed: 2026-08-29
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
  - `webrtc_FileRotatingLogSink_new` / `webrtc_CallSessionFileRotatingLogSink_new` を追加し、派生型のハンドル `struct webrtc_FileRotatingLogSink_unique*`（および CallSession 版）で返す
  - 初期化を `webrtc_FileRotatingLogSink_Init(struct webrtc_FileRotatingLogSink_unique*) -> bool` 等として公開する。派生型で受けるため downcast は不要
  - 基底 `webrtc::LogSink` への変換は既存マクロ `WEBRTC_DECLARE_CAST` / `WEBRTC_DEFINE_CAST`（`SimulcastEncoderAdapter` と同じ方式）で `webrtc_FileRotatingLogSink_cast_to_webrtc_LogSink` 等を提供する。所有権の移転は Rust 側の `ManuallyDrop` + `cast::<*mut webrtc_LogSink_unique>()` で行い、C 側は基底ビューを返す
  - `DisableBuffering()` を使いたい場合も同様の関数を公開する
  - 引数の妥当性は C++ 側の検証に委ねる
- Rust 側:
  - `src/rtc_base/log_sinks.rs` を新規追加し、`log::LoggingConfig::add_sink` へ渡せるラッパーを公開する (`src/rtc_base/mod.rs` へ登録)
- ヘッダは `webrtc/src/webrtc_c.h` の `#include "webrtc_c/rtc_base/log_sinks.h"` を追加する。`.cc` は `webrtc/CMakeLists.txt` の `webrtc_c` ソース一覧へ `logging.cc` と同様に追記する
- 本 issue は 0088 (`webrtc_LoggingConfig_AddSink` / `webrtc_LogSink_unique`) に依存する
- テスト方針: `src/tests.rs` に、`FileRotatingLogSink` / `CallSessionFileRotatingLogSink` を生成して `Init()`、`add_sink` 後に `log::print` を呼び、一時ディレクトリ配下のファイルへ書き込まれることを検証する

## 完了条件

- Rust から `FileRotatingLogSink` / `CallSessionFileRotatingLogSink` を生成し、`Init()` 後 `log::LoggingConfig::add_sink` に渡せる
- `log::print` を呼ぶと、指定したディレクトリ配下のローテーション対象ファイルへメッセージが書き込まれる

## 解決方法

- C API (`webrtc/src/webrtc_c/rtc_base/log_sinks.h` / `log_sinks.cc` を新規追加) に次を追加した
  - `webrtc_FileRotatingLogSink_new` / `webrtc_FileRotatingLogSink_Init` / `webrtc_FileRotatingLogSink_DisableBuffering` と `webrtc_FileRotatingLogSink_cast_to_webrtc_LogSink`
  - `webrtc_CallSessionFileRotatingLogSink_new` / `webrtc_CallSessionFileRotatingLogSink_Init` / `webrtc_CallSessionFileRotatingLogSink_DisableBuffering` と `webrtc_CallSessionFileRotatingLogSink_cast_to_webrtc_LogSink`
  - `_new` は `std::make_unique` + `release()` で生成した派生型を、派生型のハンドル `struct webrtc_FileRotatingLogSink_unique*`（および CallSession 版）として返す
  - `_Init` / `_DisableBuffering` は `WEBRTC_DEFINE_UNIQUE` が生成する `_unique_get` で取り出した派生型ポインタに対してそのまま呼ぶ（downcast は不要）
  - 基底 `webrtc::LogSink` ビューへの変換は既存マクロ `WEBRTC_DECLARE_CAST` / `WEBRTC_DEFINE_CAST`（`SimulcastEncoderAdapter` と同じ方式）を利用する
  - 引数の妥当性 (`num_log_files > 1`、`max_log_size > 0`) は libwebrtc 側の `RTC_DCHECK_GT` に委ね、C 側では検証しない
  - `webrtc_c.h` へのインクルード追加と `webrtc/CMakeLists.txt` の `webrtc_c` ソース一覧への追記を行った
- Rust API (`src/rtc_base/log_sinks.rs` を新規追加) に次を追加した
  - `log::FileRotatingLogSink` / `log::CallSessionFileRotatingLogSink` は派生型の `_unique` を直接保持するラッパー（`impl Drop` で `webrtc_FileRotatingLogSink_unique_delete` 等を呼ぶ）
  - `new`（内部で `Init()` まで実行する。libwebrtc の `Init()` は Chromium がコンストラクタで例外を投げられないためにエラーハンドリングを分離したもので、Rust では生成時に初期化まで行い、失敗時は生成済み sink を破棄して `Err` を返す）/ `disable_buffering` と、基底へ変換する `into_base(self) -> log::LogSink`（消費型。`SimulcastEncoderAdapter::cast_to_video_encoder` と同じく、C の cast を呼んで同一アドレスを確認し、`ManuallyDrop` + `cast::<*mut webrtc_LogSink_unique>()` で所有権を移す）
  - `src/rtc_base/logging.rs` の `LogSink` に `from_raw_unique` を pub(crate) で追加し、`into_base` が基底ラッパーを構築できるようにした
  - `log::LoggingConfig::add_sink` は既存のまま変更せず、利用側が `config.add_sink(sink.into_base())` と書く
- テスト (`src/tests.rs`) を追加した
  - `logging_file_rotating_sink_writes_to_file` (+ サブプロセス用 `logging_file_rotating_sink_helper`): 一時ディレクトリ配下へ `log::print` のメッセージが書き込まれることを `FileRotatingLogSink` / `CallSessionFileRotatingLogSink` の両方で検証する
- `CHANGES.md` の `## develop` に ADD エントリを追記した
