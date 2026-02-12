#pragma once

#include "../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::AudioEncoderFactory
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_AudioEncoderFactory);
struct webrtc_AudioEncoderFactory_refcounted*
webrtc_CreateBuiltinAudioEncoderFactory();

#if defined(__cplusplus)
}
#endif
