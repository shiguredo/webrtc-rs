#pragma once

#include "../../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::DecodeTargetIndication
// -------------------------

WEBRTC_EXPORT extern const int webrtc_DecodeTargetIndication_NotPresent;
WEBRTC_EXPORT extern const int webrtc_DecodeTargetIndication_Discardable;
WEBRTC_EXPORT extern const int webrtc_DecodeTargetIndication_Switch;
WEBRTC_EXPORT extern const int webrtc_DecodeTargetIndication_Required;

#if defined(__cplusplus)
}
#endif
