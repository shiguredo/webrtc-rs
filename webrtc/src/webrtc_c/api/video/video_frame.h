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

extern const int webrtc_VideoRotation_0;

WEBRTC_DECLARE_UNIQUE(webrtc_VideoFrame);
struct webrtc_VideoFrame_unique* webrtc_VideoFrame_Create(
    struct webrtc_I420Buffer_refcounted* buffer,
    int rotation,
    int64_t timestamp_us,
    uint32_t timestamp_rtp);
int webrtc_VideoFrame_width(const struct webrtc_VideoFrame* self);
int webrtc_VideoFrame_height(const struct webrtc_VideoFrame* self);
int64_t webrtc_VideoFrame_timestamp_us(const struct webrtc_VideoFrame* self);
uint32_t webrtc_VideoFrame_timestamp_rtp(const struct webrtc_VideoFrame* self);
struct webrtc_I420Buffer_refcounted* webrtc_VideoFrame_video_frame_buffer(
    const struct webrtc_VideoFrame* self);

#if defined(__cplusplus)
}
#endif
