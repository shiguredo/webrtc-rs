#include "rtc_error.h"

#include <stdarg.h>
#include <stddef.h>
#include <string>

// WebRTC
#include <api/rtc_error.h>

#include "../common.impl.h"

// -------------------------
// webrtc::RTCError
// -------------------------

extern "C" {
WEBRTC_DEFINE_UNIQUE(webrtc_RTCError, webrtc::RTCError);
int webrtc_RTCError_ok(struct webrtc_RTCError* self) {
  auto err = reinterpret_cast<webrtc::RTCError*>(self);
  return err->ok() ? 1 : 0;
}
void webrtc_RTCError_message(struct webrtc_RTCError* self,
                             const char** out_message,
                             size_t* out_len) {
  auto err = reinterpret_cast<webrtc::RTCError*>(self);
  if (out_message != nullptr) {
    *out_message = err->message();
  }
  if (out_len != nullptr) {
    *out_len = std::string(err->message()).size();
  }
}
}
