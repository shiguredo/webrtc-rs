#include "timestamp_aligner.h"

#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>
#include <memory>

// WebRTC
#include <rtc_base/timestamp_aligner.h>

#include "../common.impl.h"

// -------------------------
// webrtc::TimestampAligner
// -------------------------

extern "C" {
WEBRTC_DEFINE_UNIQUE(webrtc_TimestampAligner, webrtc::TimestampAligner);
struct webrtc_TimestampAligner_unique* webrtc_TimestampAligner_new() {
  auto aligner = std::make_unique<webrtc::TimestampAligner>();
  return reinterpret_cast<struct webrtc_TimestampAligner_unique*>(
      aligner.release());
}
int64_t webrtc_TimestampAligner_TranslateTimestamp(
    struct webrtc_TimestampAligner* self,
    int64_t timestamp_us,
    int64_t now_us) {
  auto aligner = reinterpret_cast<webrtc::TimestampAligner*>(self);
  return aligner->TranslateTimestamp(timestamp_us, now_us);
}
}
