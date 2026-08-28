#include "logging.h"

#include <stdarg.h>
#include <stddef.h>
#include <stdio.h>
#include <cassert>
#include <cstdint>
#include <memory>
#include <string>
#include <utility>

// WebRTC
#include <absl/strings/string_view.h>
#include <rtc_base/logging.h>

#include "../common.h"
#include "../common.impl.h"
#include "../std.h"
#include "../std.impl.h"

// -------------------------
// rtc_base/logging
// -------------------------

namespace {

// webrtc::LogSink の C ラッパー実装。
//
// libwebrtc は LoggingConfig::AddSink で登録した sink へ常に
// OnLogMessage(const LogLineRef&) を届けるため、そのオーバーロードだけを
// オーバーライドして C のコールバックへ転送する。他のオーバーロード
// （std::string / absl::string_view / severity / tag の各版）は互換のための
// 残存であり、webrtc::LogSink の既定実装（委譲）に任せる。
class LogSinkImpl : public webrtc::LogSink {
 public:
  LogSinkImpl(const struct webrtc_LogSink_cbs* cbs, void* user_data)
      : user_data_(user_data) {
    assert(cbs != nullptr);
    // 全コールバックは null 非許容（RULES.md）。
    assert(cbs->OnLogMessage_log_line_ref != nullptr);
    assert(cbs->OnDestroy != nullptr);
    cbs_ = *cbs;
  }

  ~LogSinkImpl() override { cbs_.OnDestroy(user_data_); }

  // 純粋仮想の実装。LogMessage は LoggingConfig::AddSink で登録された sink へ
  // 常に OnLogMessage(const LogLineRef&) を届けるため、このメソッドは抽象クラス
  // にならないための実装であり、実務では呼ばれない。Cbs に string 版コールバック
  // は設けず、情報を失わないよう LogLineRef 版へ統一している。
  void OnLogMessage(const std::string& /* message */) override {}

  void OnLogMessage(const webrtc::LogLineRef& line) override {
    // webrtc::LogLineRef をそのまま C の型として渡す（reinterpret_cast）。
    auto* c_line = reinterpret_cast<const struct webrtc_LogLineRef*>(&line);
    cbs_.OnLogMessage_log_line_ref(c_line, user_data_);
  }

 private:
  webrtc_LogSink_cbs cbs_{};
  void* user_data_ = nullptr;
};

}  // namespace

extern "C" {
WEBRTC_EXPORT const int webrtc_LogSeverity_LS_VERBOSE =
    static_cast<int>(webrtc::LoggingSeverity::LS_VERBOSE);
WEBRTC_EXPORT const int webrtc_LogSeverity_LS_INFO =
    static_cast<int>(webrtc::LoggingSeverity::LS_INFO);
WEBRTC_EXPORT const int webrtc_LogSeverity_LS_WARNING =
    static_cast<int>(webrtc::LoggingSeverity::LS_WARNING);
WEBRTC_EXPORT const int webrtc_LogSeverity_LS_ERROR =
    static_cast<int>(webrtc::LoggingSeverity::LS_ERROR);
WEBRTC_EXPORT const int webrtc_LogSeverity_LS_NONE =
    static_cast<int>(webrtc::LoggingSeverity::LS_NONE);

// -------------------------
// webrtc::LoggingConfig
// -------------------------

WEBRTC_EXPORT struct webrtc_LoggingConfig* webrtc_LoggingConfig_new() {
  auto config = new webrtc::LoggingConfig();
  return reinterpret_cast<struct webrtc_LoggingConfig*>(config);
}

WEBRTC_EXPORT void webrtc_LoggingConfig_delete(
    struct webrtc_LoggingConfig* self) {
  auto config = reinterpret_cast<webrtc::LoggingConfig*>(self);
  delete config;
}

WEBRTC_EXPORT int webrtc_LoggingConfig_min_severity(
    const struct webrtc_LoggingConfig* self) {
  auto config = reinterpret_cast<const webrtc::LoggingConfig*>(self);
  return static_cast<int>(config->min_severity());
}

WEBRTC_EXPORT void webrtc_LoggingConfig_set_min_severity(
    struct webrtc_LoggingConfig* self,
    int severity) {
  auto config = reinterpret_cast<webrtc::LoggingConfig*>(self);
  config->set_min_severity(static_cast<webrtc::LoggingSeverity>(severity));
}

WEBRTC_EXPORT int webrtc_LoggingConfig_debug_severity(
    const struct webrtc_LoggingConfig* self) {
  auto config = reinterpret_cast<const webrtc::LoggingConfig*>(self);
  return static_cast<int>(config->debug_severity());
}

WEBRTC_EXPORT void webrtc_LoggingConfig_set_debug_severity(
    struct webrtc_LoggingConfig* self,
    int severity) {
  auto config = reinterpret_cast<webrtc::LoggingConfig*>(self);
  config->set_debug_severity(static_cast<webrtc::LoggingSeverity>(severity));
}

WEBRTC_EXPORT int webrtc_LoggingConfig_log_thread(
    const struct webrtc_LoggingConfig* self) {
  auto config = reinterpret_cast<const webrtc::LoggingConfig*>(self);
  return config->log_thread();
}

WEBRTC_EXPORT void webrtc_LoggingConfig_set_log_thread(
    struct webrtc_LoggingConfig* self,
    int log_thread) {
  auto config = reinterpret_cast<webrtc::LoggingConfig*>(self);
  config->set_log_thread(log_thread != 0);
}

WEBRTC_EXPORT int webrtc_LoggingConfig_log_timestamp(
    const struct webrtc_LoggingConfig* self) {
  auto config = reinterpret_cast<const webrtc::LoggingConfig*>(self);
  return config->log_timestamp();
}

WEBRTC_EXPORT void webrtc_LoggingConfig_set_log_timestamp(
    struct webrtc_LoggingConfig* self,
    int log_timestamp) {
  auto config = reinterpret_cast<webrtc::LoggingConfig*>(self);
  config->set_log_timestamp(log_timestamp != 0);
}

WEBRTC_EXPORT int webrtc_LoggingConfig_log_queue_name(
    const struct webrtc_LoggingConfig* self) {
  auto config = reinterpret_cast<const webrtc::LoggingConfig*>(self);
  return config->log_queue_name();
}

WEBRTC_EXPORT void webrtc_LoggingConfig_set_log_queue_name(
    struct webrtc_LoggingConfig* self,
    int log_queue_name) {
  auto config = reinterpret_cast<webrtc::LoggingConfig*>(self);
  config->set_log_queue_name(log_queue_name != 0);
}

WEBRTC_EXPORT int webrtc_LoggingConfig_log_to_stderr(
    const struct webrtc_LoggingConfig* self) {
  auto config = reinterpret_cast<const webrtc::LoggingConfig*>(self);
  return config->log_to_stderr();
}

WEBRTC_EXPORT void webrtc_LoggingConfig_set_log_to_stderr(
    struct webrtc_LoggingConfig* self,
    int log_to_stderr) {
  auto config = reinterpret_cast<webrtc::LoggingConfig*>(self);
  config->set_log_to_stderr(log_to_stderr != 0);
}

WEBRTC_EXPORT void webrtc_LoggingConfig_log_prefix(
    const struct webrtc_LoggingConfig* self,
    const char** out_prefix,
    size_t* out_len) {
  auto config = reinterpret_cast<const webrtc::LoggingConfig*>(self);
  auto prefix = config->log_prefix();
  *out_prefix = prefix.data();
  *out_len = prefix.size();
}

WEBRTC_EXPORT void webrtc_LoggingConfig_set_log_prefix(
    struct webrtc_LoggingConfig* self,
    const char* prefix,
    size_t prefix_len) {
  auto config = reinterpret_cast<webrtc::LoggingConfig*>(self);
  config->set_log_prefix(absl::string_view(prefix, prefix_len));
}

WEBRTC_EXPORT void webrtc_LoggingConfig_AddSink(
    struct webrtc_LoggingConfig* self,
    struct webrtc_LogSink_unique* sink) {
  auto config = reinterpret_cast<webrtc::LoggingConfig*>(self);
  // unique_ptr へ所有権を移して LoggingConfig::AddSink へ渡す。これ以降は
  // config 側が持つため、呼び出し側では webrtc_LogSink_unique_delete を
  // 呼んではならない（二重解放になる）。
  // *_unique から C++ 型へは *_unique_get() を必ず経由する（RULES.md）。
  auto sink_raw = webrtc_LogSink_unique_get(sink);
  auto sink_ptr = reinterpret_cast<webrtc::LogSink*>(sink_raw);
  config->AddSink(std::unique_ptr<webrtc::LogSink>(sink_ptr));
}

// -------------------------
// webrtc::LogSink
// -------------------------

WEBRTC_DEFINE_UNIQUE(webrtc_LogSink, webrtc::LogSink);

WEBRTC_EXPORT struct webrtc_LogSink_unique* webrtc_LogSink_new(
    const struct webrtc_LogSink_cbs* cbs,
    void* user_data) {
  auto sink = std::make_unique<LogSinkImpl>(cbs, user_data);
  return reinterpret_cast<struct webrtc_LogSink_unique*>(sink.release());
}

// -------------------------
// webrtc::LogLineRef
// -------------------------

WEBRTC_EXPORT void webrtc_LogLineRef_message(
    const struct webrtc_LogLineRef* self,
    const char** out_message,
    size_t* out_message_len) {
  auto line = reinterpret_cast<const webrtc::LogLineRef*>(self);
  auto value = line->message();
  *out_message = value.data();
  *out_message_len = value.size();
}

WEBRTC_EXPORT struct std_string_unique* webrtc_LogLineRef_DefaultLogLine(
    const struct webrtc_LogLineRef* self) {
  auto line = reinterpret_cast<const webrtc::LogLineRef*>(self);
  // std::string を返すため std_string_unique* にして返す（RULES.md）。
  auto value = std::make_unique<std::string>(line->DefaultLogLine());
  return reinterpret_cast<struct std_string_unique*>(value.release());
}

WEBRTC_EXPORT void webrtc_LogLineRef_filename(
    const struct webrtc_LogLineRef* self,
    const char** out_filename,
    size_t* out_filename_len) {
  auto line = reinterpret_cast<const webrtc::LogLineRef*>(self);
  auto value = line->filename();
  *out_filename = value.data();
  *out_filename_len = value.size();
}

WEBRTC_EXPORT int webrtc_LogLineRef_line(const struct webrtc_LogLineRef* self) {
  auto line = reinterpret_cast<const webrtc::LogLineRef*>(self);
  return line->line();
}

WEBRTC_EXPORT void webrtc_LogLineRef_thread_id(
    const struct webrtc_LogLineRef* self,
    int* out_has,
    int64_t* out_thread_id) {
  auto line = reinterpret_cast<const webrtc::LogLineRef*>(self);
  // std::optional<PlatformThreadId> を has / value で表現する。
  // 値はプラットフォームでビット幅が異なるため int64_t に揃える。
  auto thread_id = line->thread_id();
  webrtc_c::OptionalGetAs(thread_id, out_has, out_thread_id,
                          [&]() { return static_cast<int64_t>(*thread_id); });
}

WEBRTC_EXPORT int64_t
webrtc_LogLineRef_timestamp(const struct webrtc_LogLineRef* self) {
  auto line = reinterpret_cast<const webrtc::LogLineRef*>(self);
  // webrtc::Timestamp はマイクロ秒でやり取りする（RULES.md）。
  return line->timestamp().us();
}

WEBRTC_EXPORT void webrtc_LogLineRef_tag(const struct webrtc_LogLineRef* self,
                                         const char** out_tag,
                                         size_t* out_tag_len) {
  auto line = reinterpret_cast<const webrtc::LogLineRef*>(self);
  auto value = line->tag();
  *out_tag = value.data();
  *out_tag_len = value.size();
}

WEBRTC_EXPORT int webrtc_LogLineRef_severity(
    const struct webrtc_LogLineRef* self) {
  auto line = reinterpret_cast<const webrtc::LogLineRef*>(self);
  return static_cast<int>(line->severity());
}

WEBRTC_EXPORT void webrtc_LogLineRef_queue_name(
    const struct webrtc_LogLineRef* self,
    const char** out_queue_name,
    size_t* out_queue_name_len) {
  auto line = reinterpret_cast<const webrtc::LogLineRef*>(self);
  auto value = line->queue_name();
  *out_queue_name = value.data();
  *out_queue_name_len = value.size();
}

WEBRTC_EXPORT bool webrtc_LogMessage_InitializeLogging(
    struct webrtc_LoggingConfig* config) {
  auto logging_config = reinterpret_cast<webrtc::LoggingConfig*>(config);
  return webrtc::InitializeLogging(std::move(*logging_config));
}

WEBRTC_EXPORT void webrtc_LogMessage_Print(int severity,
                                           const char* file,
                                           int line,
                                           const char* fmt,
                                           ...) {
  va_list args;
  va_start(args, fmt);

  // vsnprintf に渡した va_list の値は不定になるため、va_copy で複製して
  // 必要サイズを求める。
  va_list args_copy;
  va_copy(args_copy, args);
  int len = vsnprintf(nullptr, 0, fmt, args_copy);
  va_end(args_copy);
  if (len < 0) {
    // 整形失敗時はログ出力を諦める。
    va_end(args);
    return;
  }

  std::string message;
  message.resize(static_cast<size_t>(len));
  vsnprintf(message.data(), static_cast<size_t>(len) + 1, fmt, args);
  va_end(args);

  RTC_LOG_FILE_LINE(static_cast<webrtc::LoggingSeverity>(severity), file, line)
      << message;
}
}
