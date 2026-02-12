#pragma once

#include "../../common.h"
#include "../../std.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::RTCStatsReport
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_RTCStatsReport);

struct std_string_unique* webrtc_RTCStatsReport_ToJson(
    struct webrtc_RTCStatsReport* report);

#if defined(__cplusplus)
}
#endif
