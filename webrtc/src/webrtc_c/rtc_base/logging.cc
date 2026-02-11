#include "logging.h"

#include <stdarg.h>
#include <stddef.h>
#include <stdio.h>

// WebRTC
#include <rtc_base/logging.h>

// -------------------------
// rtc_base/logging
// -------------------------

#define WEBRTC_LOG_BUFFER_SIZE 4096

extern "C" {
const int webrtc_LogSeverity_LS_VERBOSE =
    static_cast<int>(webrtc::LoggingSeverity::LS_VERBOSE);
const int webrtc_LogSeverity_LS_INFO =
    static_cast<int>(webrtc::LoggingSeverity::LS_INFO);
const int webrtc_LogSeverity_LS_WARNING =
    static_cast<int>(webrtc::LoggingSeverity::LS_WARNING);
const int webrtc_LogSeverity_LS_ERROR =
    static_cast<int>(webrtc::LoggingSeverity::LS_ERROR);
const int webrtc_LogSeverity_LS_NONE =
    static_cast<int>(webrtc::LoggingSeverity::LS_NONE);

void webrtc_LogMessage_LogToDebug(int severity) {
  webrtc::LogMessage::LogToDebug(
      static_cast<webrtc::LoggingSeverity>(severity));
}
void webrtc_LogMessage_LogTimestamps() {
  webrtc::LogMessage::LogTimestamps();
}
void webrtc_LogMessage_LogThreads() {
  webrtc::LogMessage::LogThreads();
}

void webrtc_LogMessage_Print(int severity,
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
}
