#include "media_types.h"

#include <stdarg.h>
#include <stddef.h>

// WebRTC
#include <api/media_types.h>

#include "../common.h"

// -------------------------
// webrtc::MediaType
// -------------------------

extern "C" {
WEBRTC_EXPORT extern const int webrtc_MediaType_AUDIO =
    static_cast<int>(webrtc::MediaType::AUDIO);
WEBRTC_EXPORT extern const int webrtc_MediaType_VIDEO =
    static_cast<int>(webrtc::MediaType::VIDEO);
}
