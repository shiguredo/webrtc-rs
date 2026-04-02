#pragma once

#include <stdint.h>

#include "../../common.h"
#include "color_space.h"
#include "video_frame_buffer.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::VideoFrame
// -------------------------

WEBRTC_DECLARE_UNIQUE(webrtc_VideoFrame_UpdateRect);

WEBRTC_EXPORT struct webrtc_VideoFrame_UpdateRect_unique*
webrtc_VideoFrame_UpdateRect_new();
WEBRTC_EXPORT int webrtc_VideoFrame_UpdateRect_get_offset_x(
    const struct webrtc_VideoFrame_UpdateRect* self);
WEBRTC_EXPORT void webrtc_VideoFrame_UpdateRect_set_offset_x(
    struct webrtc_VideoFrame_UpdateRect* self,
    int value);
WEBRTC_EXPORT int webrtc_VideoFrame_UpdateRect_get_offset_y(
    const struct webrtc_VideoFrame_UpdateRect* self);
WEBRTC_EXPORT void webrtc_VideoFrame_UpdateRect_set_offset_y(
    struct webrtc_VideoFrame_UpdateRect* self,
    int value);
WEBRTC_EXPORT int webrtc_VideoFrame_UpdateRect_get_width(
    const struct webrtc_VideoFrame_UpdateRect* self);
WEBRTC_EXPORT void webrtc_VideoFrame_UpdateRect_set_width(
    struct webrtc_VideoFrame_UpdateRect* self,
    int value);
WEBRTC_EXPORT int webrtc_VideoFrame_UpdateRect_get_height(
    const struct webrtc_VideoFrame_UpdateRect* self);
WEBRTC_EXPORT void webrtc_VideoFrame_UpdateRect_set_height(
    struct webrtc_VideoFrame_UpdateRect* self,
    int value);

WEBRTC_DECLARE_UNIQUE(webrtc_VideoFrameBuilder);

WEBRTC_EXPORT struct webrtc_VideoFrameBuilder_unique*
webrtc_VideoFrameBuilder_new(struct webrtc_VideoFrameBuffer_refcounted* buffer);
WEBRTC_EXPORT struct webrtc_VideoFrame_unique* webrtc_VideoFrameBuilder_build(
    struct webrtc_VideoFrameBuilder* self);
WEBRTC_EXPORT void webrtc_VideoFrameBuilder_set_timestamp_ms(
    struct webrtc_VideoFrameBuilder* self,
    int64_t timestamp_ms);
WEBRTC_EXPORT void webrtc_VideoFrameBuilder_set_timestamp_us(
    struct webrtc_VideoFrameBuilder* self,
    int64_t timestamp_us);
WEBRTC_EXPORT void webrtc_VideoFrameBuilder_set_presentation_timestamp_us(
    struct webrtc_VideoFrameBuilder* self,
    int has,
    int64_t presentation_timestamp_us);
WEBRTC_EXPORT void webrtc_VideoFrameBuilder_set_reference_time_us(
    struct webrtc_VideoFrameBuilder* self,
    int has,
    int64_t reference_time_us);
WEBRTC_EXPORT void webrtc_VideoFrameBuilder_set_rtp_timestamp(
    struct webrtc_VideoFrameBuilder* self,
    uint32_t rtp_timestamp);
WEBRTC_EXPORT void webrtc_VideoFrameBuilder_set_timestamp_rtp(
    struct webrtc_VideoFrameBuilder* self,
    uint32_t timestamp_rtp);
WEBRTC_EXPORT void webrtc_VideoFrameBuilder_set_ntp_time_ms(
    struct webrtc_VideoFrameBuilder* self,
    int64_t ntp_time_ms);
WEBRTC_EXPORT void webrtc_VideoFrameBuilder_set_rotation(
    struct webrtc_VideoFrameBuilder* self,
    int rotation);
WEBRTC_EXPORT void webrtc_VideoFrameBuilder_set_color_space(
    struct webrtc_VideoFrameBuilder* self,
    int has,
    const struct webrtc_ColorSpace* color_space);
WEBRTC_EXPORT void webrtc_VideoFrameBuilder_set_id(
    struct webrtc_VideoFrameBuilder* self,
    uint16_t id);
WEBRTC_EXPORT void webrtc_VideoFrameBuilder_set_update_rect(
    struct webrtc_VideoFrameBuilder* self,
    int has,
    const struct webrtc_VideoFrame_UpdateRect* update_rect);
WEBRTC_EXPORT void webrtc_VideoFrameBuilder_set_is_repeat_frame(
    struct webrtc_VideoFrameBuilder* self,
    int is_repeat_frame);

WEBRTC_DECLARE_UNIQUE(webrtc_VideoFrame);
WEBRTC_EXPORT struct webrtc_VideoFrame_unique* webrtc_VideoFrame_copy(
    const struct webrtc_VideoFrame* self);
WEBRTC_EXPORT int webrtc_VideoFrame_width(const struct webrtc_VideoFrame* self);
WEBRTC_EXPORT int webrtc_VideoFrame_height(
    const struct webrtc_VideoFrame* self);
WEBRTC_EXPORT int64_t
webrtc_VideoFrame_timestamp_us(const struct webrtc_VideoFrame* self);
WEBRTC_EXPORT uint32_t
webrtc_VideoFrame_timestamp_rtp(const struct webrtc_VideoFrame* self);
WEBRTC_EXPORT uint16_t
webrtc_VideoFrame_id(const struct webrtc_VideoFrame* self);
WEBRTC_EXPORT int64_t
webrtc_VideoFrame_ntp_time_ms(const struct webrtc_VideoFrame* self);
WEBRTC_EXPORT void webrtc_VideoFrame_presentation_timestamp_us(
    const struct webrtc_VideoFrame* self,
    int* out_has,
    int64_t* out_value);
WEBRTC_EXPORT void webrtc_VideoFrame_reference_time_us(
    const struct webrtc_VideoFrame* self,
    int* out_has,
    int64_t* out_value);
WEBRTC_EXPORT int webrtc_VideoFrame_rotation(
    const struct webrtc_VideoFrame* self);
WEBRTC_EXPORT void webrtc_VideoFrame_color_space(
    const struct webrtc_VideoFrame* self,
    int* out_has,
    struct webrtc_ColorSpace_unique** out_value);
WEBRTC_EXPORT int webrtc_VideoFrame_has_update_rect(
    const struct webrtc_VideoFrame* self);
WEBRTC_EXPORT struct webrtc_VideoFrame_UpdateRect_unique*
webrtc_VideoFrame_update_rect(const struct webrtc_VideoFrame* self);
WEBRTC_EXPORT int webrtc_VideoFrame_is_repeat_frame(
    const struct webrtc_VideoFrame* self);
WEBRTC_EXPORT struct webrtc_VideoFrameBuffer_refcounted*
webrtc_VideoFrame_video_frame_buffer(const struct webrtc_VideoFrame* self);

#if defined(__cplusplus)
}
#endif
