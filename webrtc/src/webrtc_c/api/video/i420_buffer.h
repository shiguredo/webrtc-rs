#pragma once

#include <stdint.h>

#include "../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::I420Buffer
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_I420Buffer);
struct webrtc_VideoFrameBuffer_refcounted;

WEBRTC_EXPORT struct webrtc_I420Buffer_refcounted* webrtc_I420Buffer_Create(
    int width,
    int height);
WEBRTC_EXPORT int webrtc_I420Buffer_width(const struct webrtc_I420Buffer* self);
WEBRTC_EXPORT int webrtc_I420Buffer_height(
    const struct webrtc_I420Buffer* self);
WEBRTC_EXPORT uint8_t* webrtc_I420Buffer_MutableDataY(
    struct webrtc_I420Buffer* self);
WEBRTC_EXPORT uint8_t* webrtc_I420Buffer_MutableDataU(
    struct webrtc_I420Buffer* self);
WEBRTC_EXPORT uint8_t* webrtc_I420Buffer_MutableDataV(
    struct webrtc_I420Buffer* self);
WEBRTC_EXPORT int webrtc_I420Buffer_StrideY(struct webrtc_I420Buffer* self);
WEBRTC_EXPORT int webrtc_I420Buffer_StrideU(struct webrtc_I420Buffer* self);
WEBRTC_EXPORT int webrtc_I420Buffer_StrideV(struct webrtc_I420Buffer* self);
WEBRTC_EXPORT void webrtc_I420Buffer_ScaleFrom(struct webrtc_I420Buffer* self,
                                               struct webrtc_I420Buffer* src);
WEBRTC_DECLARE_CAST_REFCOUNTED(webrtc_I420Buffer, webrtc_VideoFrameBuffer);

#if defined(__cplusplus)
}
#endif
