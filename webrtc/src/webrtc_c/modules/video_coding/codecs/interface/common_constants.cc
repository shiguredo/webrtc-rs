#include "common_constants.h"

// WebRTC
#include <modules/video_coding/codecs/interface/common_constants.h>

#include "../../../../common.h"

extern "C" {

WEBRTC_EXPORT extern const int16_t webrtc_kNoPictureId = webrtc::kNoPictureId;
WEBRTC_EXPORT extern const int16_t webrtc_kNoTl0PicIdx = webrtc::kNoTl0PicIdx;
WEBRTC_EXPORT extern const uint8_t webrtc_kNoTemporalIdx =
    webrtc::kNoTemporalIdx;
WEBRTC_EXPORT extern const int webrtc_kNoKeyIdx = webrtc::kNoKeyIdx;
}
