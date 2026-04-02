#include "priority.h"

// WebRTC
#include <api/priority.h>

#include "../common.h"

extern "C" {
WEBRTC_EXPORT extern const int webrtc_Priority_kVeryLow =
    static_cast<int>(webrtc::Priority::kVeryLow);
WEBRTC_EXPORT extern const int webrtc_Priority_kLow =
    static_cast<int>(webrtc::Priority::kLow);
WEBRTC_EXPORT extern const int webrtc_Priority_kMedium =
    static_cast<int>(webrtc::Priority::kMedium);
WEBRTC_EXPORT extern const int webrtc_Priority_kHigh =
    static_cast<int>(webrtc::Priority::kHigh);
}
