#pragma once

#include <stdint.h>

#include "../../../../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::RTPVideoHeaderVP8
// -------------------------

struct webrtc_RTPVideoHeaderVP8;

WEBRTC_EXPORT struct webrtc_RTPVideoHeaderVP8* webrtc_RTPVideoHeaderVP8_new();
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP8_delete(
    struct webrtc_RTPVideoHeaderVP8* self);
WEBRTC_EXPORT struct webrtc_RTPVideoHeaderVP8* webrtc_RTPVideoHeaderVP8_copy(
    const struct webrtc_RTPVideoHeaderVP8* self);

WEBRTC_EXPORT int webrtc_RTPVideoHeaderVP8_get_nonReference(
    struct webrtc_RTPVideoHeaderVP8* self);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP8_set_nonReference(
    struct webrtc_RTPVideoHeaderVP8* self,
    int value);
WEBRTC_EXPORT int16_t
webrtc_RTPVideoHeaderVP8_get_pictureId(struct webrtc_RTPVideoHeaderVP8* self);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP8_set_pictureId(
    struct webrtc_RTPVideoHeaderVP8* self,
    int16_t value);
WEBRTC_EXPORT int16_t
webrtc_RTPVideoHeaderVP8_get_tl0PicIdx(struct webrtc_RTPVideoHeaderVP8* self);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP8_set_tl0PicIdx(
    struct webrtc_RTPVideoHeaderVP8* self,
    int16_t value);
WEBRTC_EXPORT uint8_t
webrtc_RTPVideoHeaderVP8_get_temporalIdx(struct webrtc_RTPVideoHeaderVP8* self);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP8_set_temporalIdx(
    struct webrtc_RTPVideoHeaderVP8* self,
    uint8_t value);
WEBRTC_EXPORT int webrtc_RTPVideoHeaderVP8_get_layerSync(
    struct webrtc_RTPVideoHeaderVP8* self);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP8_set_layerSync(
    struct webrtc_RTPVideoHeaderVP8* self,
    int value);
WEBRTC_EXPORT int webrtc_RTPVideoHeaderVP8_get_keyIdx(
    struct webrtc_RTPVideoHeaderVP8* self);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP8_set_keyIdx(
    struct webrtc_RTPVideoHeaderVP8* self,
    int value);
WEBRTC_EXPORT int webrtc_RTPVideoHeaderVP8_get_partitionId(
    struct webrtc_RTPVideoHeaderVP8* self);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP8_set_partitionId(
    struct webrtc_RTPVideoHeaderVP8* self,
    int value);
WEBRTC_EXPORT int webrtc_RTPVideoHeaderVP8_get_beginningOfPartition(
    struct webrtc_RTPVideoHeaderVP8* self);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP8_set_beginningOfPartition(
    struct webrtc_RTPVideoHeaderVP8* self,
    int value);

#if defined(__cplusplus)
}
#endif
