#include "time_utils.h"

#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>

// WebRTC
#include <rtc_base/time_utils.h>

#include "../common.h"

// -------------------------
// rtc_base/time_utils
// -------------------------

extern "C" {
int64_t WEBRTC_EXPORT webrtc_TimeMillis() {
  return webrtc::TimeMillis();
}
}
