#pragma once

#include "../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::VideoDecoderFactory
// -------------------------

WEBRTC_DECLARE_UNIQUE(webrtc_VideoDecoderFactory);
struct webrtc_VideoDecoderFactory_unique*
webrtc_CreateBuiltinVideoDecoderFactory();

#if defined(__cplusplus)
}
#endif
