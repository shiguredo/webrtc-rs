#pragma once

#include "../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::VideoEncoderFactory
// -------------------------

WEBRTC_DECLARE_UNIQUE(webrtc_VideoEncoderFactory);
struct webrtc_VideoEncoderFactory_unique*
webrtc_CreateBuiltinVideoEncoderFactory();

#if defined(__cplusplus)
}
#endif
