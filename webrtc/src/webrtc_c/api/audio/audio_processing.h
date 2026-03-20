#pragma once

#include "../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::AudioProcessingBuilderInterface
// -------------------------

WEBRTC_DECLARE_UNIQUE(webrtc_AudioProcessingBuilderInterface);
struct webrtc_AudioProcessingBuilderInterface_unique* WEBRTC_EXPORT
webrtc_BuiltinAudioProcessingBuilder_Create();

#if defined(__cplusplus)
}
#endif
