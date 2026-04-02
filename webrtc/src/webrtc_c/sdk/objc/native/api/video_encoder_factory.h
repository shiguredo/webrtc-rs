#pragma once

#include "../../../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

struct webrtc_objc_RTCVideoEncoderFactory;
struct webrtc_VideoEncoderFactory_unique;

WEBRTC_EXPORT struct webrtc_VideoEncoderFactory_unique*
webrtc_ObjCToNativeVideoEncoderFactory(
    struct webrtc_objc_RTCVideoEncoderFactory* objc_video_encoder_factory);

#if defined(__cplusplus)
}
#endif
