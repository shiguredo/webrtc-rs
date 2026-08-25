#include "logging.h"

#include <stdarg.h>
#include <stddef.h>
#include <stdio.h>
#include <string>
#include <utility>

// WebRTC
#include <rtc_base/logging.h>

#include "../common.h"

// -------------------------
// rtc_base/logging
// -------------------------

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
