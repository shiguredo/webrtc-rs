#pragma once

#include <stddef.h>
#include <stdint.h>

#include "../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::AdaptedAudioTrackSource
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_AdaptedAudioTrackSource);
WEBRTC_EXPORT struct webrtc_AdaptedAudioTrackSource_refcounted*
webrtc_AdaptedAudioTrackSource_Create(int sample_rate, size_t channels);
WEBRTC_EXPORT void webrtc_AdaptedAudioTrackSource_OnData(
    struct webrtc_AdaptedAudioTrackSource* self,
    const int16_t* audio_data,
    size_t samples_per_channel);
WEBRTC_DECLARE_CAST_REFCOUNTED(webrtc_AdaptedAudioTrackSource,
                               webrtc_AudioSourceInterface);

#if defined(__cplusplus)
}
#endif
