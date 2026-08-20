#include "logging.h"

#include <stdarg.h>
#include <stddef.h>
#include <stdio.h>
#include <string>

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

WEBRTC_EXPORT void webrtc_LogMessage_LogToDebug(int severity) {
  webrtc::LogMessage::LogToDebug(
      static_cast<webrtc::LoggingSeverity>(severity));
}
WEBRTC_EXPORT void webrtc_LogMessage_LogTimestamps() {
  webrtc::LogMessage::LogTimestamps();
}
WEBRTC_EXPORT void webrtc_LogMessage_LogThreads() {
  webrtc::LogMessage::LogThreads();
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
