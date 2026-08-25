#include "video_content_type.h"

#include <api/video/video_content_type.h>

#include "../../common.h"

extern "C" {
WEBRTC_EXPORT const int webrtc_VideoContentType_UNSPECIFIED =
    static_cast<int>(webrtc::VideoContentType::UNSPECIFIED);
WEBRTC_EXPORT const int webrtc_VideoContentType_SCREENSHARE =
    static_cast<int>(webrtc::VideoContentType::SCREENSHARE);
}
