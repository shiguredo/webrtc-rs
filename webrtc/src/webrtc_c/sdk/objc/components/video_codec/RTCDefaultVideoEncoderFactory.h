#pragma once

#include "../../../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

struct webrtc_objc_RTCVideoEncoderFactory;

WEBRTC_EXPORT struct webrtc_objc_RTCVideoEncoderFactory*
webrtc_objc_RTCDefaultVideoEncoderFactory_new(void);
WEBRTC_EXPORT void webrtc_objc_RTCVideoEncoderFactory_release(
    struct webrtc_objc_RTCVideoEncoderFactory* self);

#if defined(__cplusplus)
}
#endif
