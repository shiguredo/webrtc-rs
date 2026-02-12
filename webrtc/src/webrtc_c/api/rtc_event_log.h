#pragma once

#include "../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::RtcEventLogFactory
// -------------------------

WEBRTC_DECLARE_UNIQUE(webrtc_RtcEventLogFactory);
struct webrtc_RtcEventLogFactory_unique* webrtc_RtcEventLogFactory_Create();

#if defined(__cplusplus)
}
#endif
