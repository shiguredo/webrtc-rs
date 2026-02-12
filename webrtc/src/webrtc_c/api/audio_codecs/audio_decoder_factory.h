#pragma once

#include "../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::AudioDecoderFactory
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_AudioDecoderFactory);
struct webrtc_AudioDecoderFactory_refcounted*
webrtc_CreateBuiltinAudioDecoderFactory();

#if defined(__cplusplus)
}
#endif
