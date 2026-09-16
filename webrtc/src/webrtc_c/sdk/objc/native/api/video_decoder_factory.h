#pragma once

#include "../../../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

struct webrtc_objc_RTCVideoDecoderFactory;
struct webrtc_VideoDecoderFactory_unique;

WEBRTC_EXPORT struct webrtc_VideoDecoderFactory_unique*
webrtc_ObjCToNativeVideoDecoderFactory(
    struct webrtc_objc_RTCVideoDecoderFactory* objc_video_decoder_factory);

#if defined(__cplusplus)
}
#endif
