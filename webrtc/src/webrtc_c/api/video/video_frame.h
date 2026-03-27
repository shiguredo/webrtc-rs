#pragma once

#include <stdint.h>

#include "../../common.h"
#include "i420_buffer.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::VideoFrame
// -------------------------

WEBRTC_EXPORT extern const int webrtc_VideoRotation_0;
WEBRTC_EXPORT extern const int webrtc_VideoRotation_90;
WEBRTC_EXPORT extern const int webrtc_VideoRotation_180;
WEBRTC_EXPORT extern const int webrtc_VideoRotation_270;

WEBRTC_DECLARE_UNIQUE(webrtc_VideoFrame);
WEBRTC_EXPORT struct webrtc_VideoFrame_unique* webrtc_VideoFrame_Create(
    struct webrtc_I420Buffer_refcounted* buffer,
    int rotation,
    int64_t timestamp_us,
    uint32_t timestamp_rtp);
WEBRTC_EXPORT int webrtc_VideoFrame_width(const struct webrtc_VideoFrame* self);
WEBRTC_EXPORT int webrtc_VideoFrame_height(
    const struct webrtc_VideoFrame* self);
WEBRTC_EXPORT int64_t
webrtc_VideoFrame_timestamp_us(const struct webrtc_VideoFrame* self);
WEBRTC_EXPORT uint32_t
webrtc_VideoFrame_timestamp_rtp(const struct webrtc_VideoFrame* self);
WEBRTC_EXPORT int webrtc_VideoFrame_rotation(const struct webrtc_VideoFrame* self);
WEBRTC_EXPORT struct webrtc_I420Buffer_refcounted*
webrtc_VideoFrame_video_frame_buffer(const struct webrtc_VideoFrame* self);

#if defined(__cplusplus)
}
#endif
