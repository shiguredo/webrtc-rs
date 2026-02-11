#pragma once

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// rtc_base/logging
// -------------------------

extern const int webrtc_LogSeverity_LS_VERBOSE;
extern const int webrtc_LogSeverity_LS_INFO;
extern const int webrtc_LogSeverity_LS_WARNING;
extern const int webrtc_LogSeverity_LS_ERROR;
extern const int webrtc_LogSeverity_LS_NONE;
void webrtc_LogMessage_LogToDebug(int severity);
void webrtc_LogMessage_LogTimestamps();
void webrtc_LogMessage_LogThreads();
void webrtc_LogMessage_Print(int severity,
                             const char* file,
                             int line,
                             const char* fmt,
                             ...);

#define RTC_LOG_VERBOSE(fmt, ...)                                            \
  webrtc_LogMessage_Print(webrtc_LogSeverity_LS_VERBOSE, __FILE__, __LINE__, \
                          fmt, ##__VA_ARGS__)
#define RTC_LOG_INFO(fmt, ...)                                                 \
  webrtc_LogMessage_Print(webrtc_LogSeverity_LS_INFO, __FILE__, __LINE__, fmt, \
                          ##__VA_ARGS__)
#define RTC_LOG_WARNING(fmt, ...)                                            \
  webrtc_LogMessage_Print(webrtc_LogSeverity_LS_WARNING, __FILE__, __LINE__, \
                          fmt, ##__VA_ARGS__)
#define RTC_LOG_ERROR(fmt, ...)                                            \
  webrtc_LogMessage_Print(webrtc_LogSeverity_LS_ERROR, __FILE__, __LINE__, \
                          fmt, ##__VA_ARGS__)

#if defined(__cplusplus)
}
#endif
