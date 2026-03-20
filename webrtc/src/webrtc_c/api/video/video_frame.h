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

WEBRTC_DECLARE_UNIQUE(webrtc_VideoFrame);
struct webrtc_VideoFrame_unique* WEBRTC_EXPORT
webrtc_VideoFrame_Create(struct webrtc_I420Buffer_refcounted* buffer,
                         int rotation,
                         int64_t timestamp_us,
                         uint32_t timestamp_rtp);
int WEBRTC_EXPORT webrtc_VideoFrame_width(const struct webrtc_VideoFrame* self);
int WEBRTC_EXPORT
webrtc_VideoFrame_height(const struct webrtc_VideoFrame* self);
int64_t WEBRTC_EXPORT
webrtc_VideoFrame_timestamp_us(const struct webrtc_VideoFrame* self);
uint32_t WEBRTC_EXPORT
webrtc_VideoFrame_timestamp_rtp(const struct webrtc_VideoFrame* self);
struct webrtc_I420Buffer_refcounted* WEBRTC_EXPORT
webrtc_VideoFrame_video_frame_buffer(const struct webrtc_VideoFrame* self);

#if defined(__cplusplus)
}
#endif
