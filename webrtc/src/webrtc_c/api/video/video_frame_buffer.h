#pragma once

#include <stdint.h>

#include "../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::VideoFrameBuffer
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_VideoFrameBuffer);

struct webrtc_I420Buffer_refcounted;

struct webrtc_VideoFrameBuffer_cbs {
  int (*type)(void* user_data);
  int (*width)(void* user_data);
  int (*height)(void* user_data);
  struct webrtc_I420Buffer_refcounted* (*ToI420)(void* user_data);
  void (*OnDestroy)(void* user_data);
};

WEBRTC_EXPORT int webrtc_VideoFrameBuffer_type(
    const struct webrtc_VideoFrameBuffer* self);
WEBRTC_EXPORT int webrtc_VideoFrameBuffer_width(
    const struct webrtc_VideoFrameBuffer* self);
WEBRTC_EXPORT int webrtc_VideoFrameBuffer_height(
    const struct webrtc_VideoFrameBuffer* self);
WEBRTC_EXPORT struct webrtc_I420Buffer_refcounted* webrtc_VideoFrameBuffer_ToI420(
    struct webrtc_VideoFrameBuffer* self);
WEBRTC_EXPORT struct webrtc_VideoFrameBuffer_refcounted*
webrtc_VideoFrameBuffer_make_ref_counted(
    const struct webrtc_VideoFrameBuffer_cbs* cbs,
    void* user_data);

#if defined(__cplusplus)
}
#endif
