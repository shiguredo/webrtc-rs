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
struct webrtc_I420Buffer_refcounted* webrtc_I420Buffer_Create(int width,
                                                              int height);
int webrtc_I420Buffer_width(const struct webrtc_I420Buffer* self);
int webrtc_I420Buffer_height(const struct webrtc_I420Buffer* self);
uint8_t* webrtc_I420Buffer_MutableDataY(struct webrtc_I420Buffer* self);
uint8_t* webrtc_I420Buffer_MutableDataU(struct webrtc_I420Buffer* self);
uint8_t* webrtc_I420Buffer_MutableDataV(struct webrtc_I420Buffer* self);
int webrtc_I420Buffer_StrideY(struct webrtc_I420Buffer* self);
int webrtc_I420Buffer_StrideU(struct webrtc_I420Buffer* self);
int webrtc_I420Buffer_StrideV(struct webrtc_I420Buffer* self);
void webrtc_I420Buffer_ScaleFrom(struct webrtc_I420Buffer* self,
                                 struct webrtc_I420Buffer* src);

#if defined(__cplusplus)
}
#endif
