#include "log_sinks.h"

// WebRTC
#include <absl/strings/string_view.h>
#include <rtc_base/log_sinks.h>

#include <memory>
#include <utility>

#include "../common.h"
#include "../common.impl.h"

extern "C" {
WEBRTC_DEFINE_UNIQUE(webrtc_FileRotatingLogSink, webrtc::FileRotatingLogSink);
WEBRTC_DEFINE_CAST(webrtc_FileRotatingLogSink,
                   webrtc_LogSink,
                   webrtc::FileRotatingLogSink,
                   webrtc::LogSink);

WEBRTC_EXPORT struct webrtc_FileRotatingLogSink_unique*
webrtc_FileRotatingLogSink_new(const char* log_dir_path,
                               size_t log_dir_path_len,
                               const char* log_prefix,
                               size_t log_prefix_len,
                               size_t max_log_size,
                               size_t num_log_files) {
  // make_unique で構築し、release() した生ポインタを派生型のまま返す（RULES.md）。
  auto sink = std::make_unique<webrtc::FileRotatingLogSink>(
      absl::string_view(log_dir_path, log_dir_path_len),
      absl::string_view(log_prefix, log_prefix_len), max_log_size,
      num_log_files);
  return reinterpret_cast<struct webrtc_FileRotatingLogSink_unique*>(
      sink.release());
}

WEBRTC_EXPORT bool webrtc_FileRotatingLogSink_Init(
    struct webrtc_FileRotatingLogSink_unique* self) {
  auto* raw = webrtc_FileRotatingLogSink_unique_get(self);
  auto* cpp = reinterpret_cast<webrtc::FileRotatingLogSink*>(raw);
  return cpp->Init();
}

WEBRTC_EXPORT bool webrtc_FileRotatingLogSink_DisableBuffering(
    struct webrtc_FileRotatingLogSink_unique* self) {
  auto* raw = webrtc_FileRotatingLogSink_unique_get(self);
  auto* cpp = reinterpret_cast<webrtc::FileRotatingLogSink*>(raw);
  return cpp->DisableBuffering();
}

WEBRTC_DEFINE_UNIQUE(webrtc_CallSessionFileRotatingLogSink,
                     webrtc::CallSessionFileRotatingLogSink);
WEBRTC_DEFINE_CAST(webrtc_CallSessionFileRotatingLogSink,
                   webrtc_LogSink,
                   webrtc::CallSessionFileRotatingLogSink,
                   webrtc::LogSink);

WEBRTC_EXPORT struct webrtc_CallSessionFileRotatingLogSink_unique*
webrtc_CallSessionFileRotatingLogSink_new(const char* log_dir_path,
                                          size_t log_dir_path_len,
                                          size_t max_total_log_size) {
  auto sink = std::make_unique<webrtc::CallSessionFileRotatingLogSink>(
      absl::string_view(log_dir_path, log_dir_path_len), max_total_log_size);
  return reinterpret_cast<struct webrtc_CallSessionFileRotatingLogSink_unique*>(
      sink.release());
}

WEBRTC_EXPORT bool webrtc_CallSessionFileRotatingLogSink_Init(
    struct webrtc_CallSessionFileRotatingLogSink_unique* self) {
  auto* raw = webrtc_CallSessionFileRotatingLogSink_unique_get(self);
  auto* cpp = reinterpret_cast<webrtc::CallSessionFileRotatingLogSink*>(raw);
  return cpp->Init();
}

WEBRTC_EXPORT bool webrtc_CallSessionFileRotatingLogSink_DisableBuffering(
    struct webrtc_CallSessionFileRotatingLogSink_unique* self) {
  auto* raw = webrtc_CallSessionFileRotatingLogSink_unique_get(self);
  auto* cpp = reinterpret_cast<webrtc::CallSessionFileRotatingLogSink*>(raw);
  return cpp->DisableBuffering();
}

}  // extern "C"
