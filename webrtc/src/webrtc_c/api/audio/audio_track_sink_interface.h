#pragma once

#include <stddef.h>

#include "../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

struct webrtc_AudioTrackSinkInterface;

struct webrtc_AudioTrackSinkInterface_cbs {
  void (*OnData)(const void* audio_data,
                 int bits_per_sample,
                 int sample_rate,
                 size_t number_of_channels,
                 size_t number_of_frames,
                 void* user_data);
  void (*OnDestroy)(void* user_data);
};

WEBRTC_EXPORT struct webrtc_AudioTrackSinkInterface*
webrtc_AudioTrackSinkInterface_new(
    const struct webrtc_AudioTrackSinkInterface_cbs* cbs,
    void* user_data);
WEBRTC_EXPORT void webrtc_AudioTrackSinkInterface_delete(
    struct webrtc_AudioTrackSinkInterface* self);

#if defined(__cplusplus)
}
#endif
