#pragma once

#include "../../../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

struct webrtc_objc_RTCVideoDecoderFactory;

WEBRTC_EXPORT struct webrtc_objc_RTCVideoDecoderFactory*
webrtc_objc_RTCDefaultVideoDecoderFactory_new(void);
WEBRTC_EXPORT void webrtc_objc_RTCVideoDecoderFactory_release(
    struct webrtc_objc_RTCVideoDecoderFactory* self);

#if defined(__cplusplus)
}
#endif
