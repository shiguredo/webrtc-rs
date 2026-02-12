#pragma once

#include <stdint.h>

#include "../../api/video/video_frame.h"
#include "../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::AdaptedVideoTrackSource
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_AdaptedVideoTrackSource);
struct webrtc_AdaptedVideoTrackSource_refcounted*
webrtc_AdaptedVideoTrackSource_Create();
int webrtc_AdaptedVideoTrackSource_AdaptFrame(
    struct webrtc_AdaptedVideoTrackSource* self,
    int width,
    int height,
    int64_t timestamp_us,
    int* out_adapted_width,
    int* out_adapted_height,
    int* out_crop_width,
    int* out_crop_height,
    int* out_crop_x,
    int* out_crop_y);
void webrtc_AdaptedVideoTrackSource_OnFrame(
    struct webrtc_AdaptedVideoTrackSource* self,
    struct webrtc_VideoFrame* frame);
WEBRTC_DECLARE_CAST_REFCOUNTED(webrtc_AdaptedVideoTrackSource,
                               webrtc_VideoTrackSourceInterface);

#if defined(__cplusplus)
}
#endif
