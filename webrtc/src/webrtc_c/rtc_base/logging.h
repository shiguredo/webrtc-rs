#pragma once

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "../common.h"
#include "../std.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// rtc_base/logging
// -------------------------

WEBRTC_EXPORT extern const int webrtc_LogSeverity_LS_VERBOSE;
WEBRTC_EXPORT extern const int webrtc_LogSeverity_LS_INFO;
WEBRTC_EXPORT extern const int webrtc_LogSeverity_LS_WARNING;
WEBRTC_EXPORT extern const int webrtc_LogSeverity_LS_ERROR;
WEBRTC_EXPORT extern const int webrtc_LogSeverity_LS_NONE;

// -------------------------
// webrtc::LogSink
// -------------------------

WEBRTC_DECLARE_UNIQUE(webrtc_LogSink);
// 全コールバックは必須（null 非許容）。
// 呼び出し側は全関数ポインタを非 null で設定しなければならない。
struct webrtc_LogSink_cbs {
  // OnLogMessage(const LogLineRef& line)
  void (*OnLogMessage_log_line_ref)(const struct webrtc_LogLineRef* line,
                                    void* user_data);
  void (*OnDestroy)(void* user_data);
};
WEBRTC_EXPORT struct webrtc_LogSink_unique* webrtc_LogSink_new(
    const struct webrtc_LogSink_cbs* cbs,
    void* user_data);

// -------------------------
// webrtc::LogLineRef
// -------------------------

struct webrtc_LogLineRef;
WEBRTC_EXPORT void webrtc_LogLineRef_message(
    const struct webrtc_LogLineRef* self,
    const char** out_message,
    size_t* out_message_len);
WEBRTC_EXPORT struct std_string_unique* webrtc_LogLineRef_DefaultLogLine(
    const struct webrtc_LogLineRef* self);
WEBRTC_EXPORT void webrtc_LogLineRef_filename(
    const struct webrtc_LogLineRef* self,
    const char** out_filename,
    size_t* out_filename_len);
WEBRTC_EXPORT int webrtc_LogLineRef_line(const struct webrtc_LogLineRef* self);
WEBRTC_EXPORT void webrtc_LogLineRef_thread_id(
    const struct webrtc_LogLineRef* self,
    int* out_has,
    int64_t* out_thread_id);
WEBRTC_EXPORT int64_t
webrtc_LogLineRef_timestamp(const struct webrtc_LogLineRef* self);
WEBRTC_EXPORT void webrtc_LogLineRef_tag(const struct webrtc_LogLineRef* self,
                                         const char** out_tag,
                                         size_t* out_tag_len);
WEBRTC_EXPORT int webrtc_LogLineRef_severity(
    const struct webrtc_LogLineRef* self);
WEBRTC_EXPORT void webrtc_LogLineRef_queue_name(
    const struct webrtc_LogLineRef* self,
    const char** out_queue_name,
    size_t* out_queue_name_len);

// -------------------------
// webrtc::LoggingConfig
// -------------------------

struct webrtc_LoggingConfig;
WEBRTC_EXPORT struct webrtc_LoggingConfig* webrtc_LoggingConfig_new();
WEBRTC_EXPORT void webrtc_LoggingConfig_delete(
    struct webrtc_LoggingConfig* self);
WEBRTC_EXPORT int webrtc_LoggingConfig_min_severity(
    const struct webrtc_LoggingConfig* self);
WEBRTC_EXPORT void webrtc_LoggingConfig_set_min_severity(
    struct webrtc_LoggingConfig* self,
    int severity);
WEBRTC_EXPORT int webrtc_LoggingConfig_debug_severity(
    const struct webrtc_LoggingConfig* self);
WEBRTC_EXPORT void webrtc_LoggingConfig_set_debug_severity(
    struct webrtc_LoggingConfig* self,
    int severity);
WEBRTC_EXPORT int webrtc_LoggingConfig_log_thread(
    const struct webrtc_LoggingConfig* self);
WEBRTC_EXPORT void webrtc_LoggingConfig_set_log_thread(
    struct webrtc_LoggingConfig* self,
    int log_thread);
WEBRTC_EXPORT int webrtc_LoggingConfig_log_timestamp(
    const struct webrtc_LoggingConfig* self);
WEBRTC_EXPORT void webrtc_LoggingConfig_set_log_timestamp(
    struct webrtc_LoggingConfig* self,
    int log_timestamp);
WEBRTC_EXPORT int webrtc_LoggingConfig_log_queue_name(
    const struct webrtc_LoggingConfig* self);
WEBRTC_EXPORT void webrtc_LoggingConfig_set_log_queue_name(
    struct webrtc_LoggingConfig* self,
    int log_queue_name);
WEBRTC_EXPORT int webrtc_LoggingConfig_log_to_stderr(
    const struct webrtc_LoggingConfig* self);
WEBRTC_EXPORT void webrtc_LoggingConfig_set_log_to_stderr(
    struct webrtc_LoggingConfig* self,
    int log_to_stderr);
WEBRTC_EXPORT void webrtc_LoggingConfig_log_prefix(
    const struct webrtc_LoggingConfig* self,
    const char** out_prefix,
    size_t* out_len);
WEBRTC_EXPORT void webrtc_LoggingConfig_set_log_prefix(
    struct webrtc_LoggingConfig* self,
    const char* prefix,
    size_t prefix_len);
WEBRTC_EXPORT void webrtc_LoggingConfig_AddSink(
    struct webrtc_LoggingConfig* self,
    struct webrtc_LogSink_unique* sink);

WEBRTC_EXPORT bool webrtc_LogMessage_InitializeLogging(
    struct webrtc_LoggingConfig* config);
WEBRTC_EXPORT void webrtc_LogMessage_Print(int severity,
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
