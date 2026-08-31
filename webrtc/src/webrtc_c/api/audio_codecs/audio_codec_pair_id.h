#pragma once

#include <stdint.h>

#include "../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::AudioCodecPairId
// -------------------------

struct webrtc_AudioCodecPairId;
WEBRTC_EXPORT struct webrtc_AudioCodecPairId* webrtc_AudioCodecPairId_Create();
WEBRTC_EXPORT struct webrtc_AudioCodecPairId* webrtc_AudioCodecPairId_copy(
    const struct webrtc_AudioCodecPairId* self);
WEBRTC_EXPORT void webrtc_AudioCodecPairId_delete(
    struct webrtc_AudioCodecPairId* self);
WEBRTC_EXPORT uint64_t webrtc_AudioCodecPairId_NumericRepresentation(
    const struct webrtc_AudioCodecPairId* self);
WEBRTC_EXPORT int webrtc_AudioCodecPairId_is_equal(
    const struct webrtc_AudioCodecPairId* a,
    const struct webrtc_AudioCodecPairId* b);
WEBRTC_EXPORT int webrtc_AudioCodecPairId_less(
    const struct webrtc_AudioCodecPairId* a,
    const struct webrtc_AudioCodecPairId* b);

#if defined(__cplusplus)
}
#endif
