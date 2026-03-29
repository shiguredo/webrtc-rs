#pragma once

#include <stdint.h>

#include "../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::NV12Buffer
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_NV12Buffer);
struct webrtc_VideoFrameBuffer_refcounted;

WEBRTC_EXPORT struct webrtc_NV12Buffer_refcounted* webrtc_NV12Buffer_Create(
    int width,
    int height);
WEBRTC_EXPORT int webrtc_NV12Buffer_width(const struct webrtc_NV12Buffer* self);
WEBRTC_EXPORT int webrtc_NV12Buffer_height(
    const struct webrtc_NV12Buffer* self);
WEBRTC_EXPORT uint8_t* webrtc_NV12Buffer_MutableDataY(
    struct webrtc_NV12Buffer* self);
WEBRTC_EXPORT uint8_t* webrtc_NV12Buffer_MutableDataUV(
    struct webrtc_NV12Buffer* self);
WEBRTC_EXPORT int webrtc_NV12Buffer_StrideY(struct webrtc_NV12Buffer* self);
WEBRTC_EXPORT int webrtc_NV12Buffer_StrideUV(struct webrtc_NV12Buffer* self);
WEBRTC_EXPORT void webrtc_NV12Buffer_CropAndScaleFrom(
    struct webrtc_NV12Buffer* self,
    struct webrtc_NV12Buffer* src,
    int offset_x,
    int offset_y,
    int crop_width,
    int crop_height);
WEBRTC_DECLARE_CAST_REFCOUNTED(webrtc_NV12Buffer, webrtc_VideoFrameBuffer);

#if defined(__cplusplus)
}
#endif
