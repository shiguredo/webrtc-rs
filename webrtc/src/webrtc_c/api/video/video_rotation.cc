#include "video_rotation.h"

// WebRTC
#include <api/video/video_rotation.h>

#include "../../common.h"

extern "C" {

WEBRTC_EXPORT const int webrtc_VideoRotation_0 =
    static_cast<int>(webrtc::kVideoRotation_0);
WEBRTC_EXPORT const int webrtc_VideoRotation_90 =
    static_cast<int>(webrtc::kVideoRotation_90);
WEBRTC_EXPORT const int webrtc_VideoRotation_180 =
    static_cast<int>(webrtc::kVideoRotation_180);
WEBRTC_EXPORT const int webrtc_VideoRotation_270 =
    static_cast<int>(webrtc::kVideoRotation_270);

}  // extern "C"
