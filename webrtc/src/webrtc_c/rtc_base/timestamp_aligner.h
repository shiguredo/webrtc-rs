#pragma once

#include <stdint.h>

#include "../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::TimestampAligner
// -------------------------

WEBRTC_DECLARE_UNIQUE(webrtc_TimestampAligner);
struct webrtc_TimestampAligner_unique* webrtc_TimestampAligner_new();
int64_t webrtc_TimestampAligner_TranslateTimestamp(
    struct webrtc_TimestampAligner* self,
    int64_t timestamp_us,
    int64_t now_us);

#if defined(__cplusplus)
}
#endif
