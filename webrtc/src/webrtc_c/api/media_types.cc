#include "media_types.h"

#include <stdarg.h>
#include <stddef.h>

// WebRTC
#include <api/media_types.h>

// -------------------------
// webrtc::MediaType
// -------------------------

extern "C" {
extern const int webrtc_MediaType_AUDIO =
    static_cast<int>(webrtc::MediaType::AUDIO);
extern const int webrtc_MediaType_VIDEO =
    static_cast<int>(webrtc::MediaType::VIDEO);
}
