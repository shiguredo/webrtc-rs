#include "video_frame.h"

#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>
#include <memory>
#include <optional>

// WebRTC
#include <api/scoped_refptr.h>
#include <api/video/i420_buffer.h>
#include <api/video/color_space.h>
#include <api/video/video_frame.h>
#include <api/video/video_frame_buffer.h>
#include <api/video/video_rotation.h>
#include <api/units/timestamp.h>

#include "../../common.h"
#include "../../common.impl.h"
#include "../../std.impl.h"
#include "color_space.h"
#include "video_frame_buffer.h"

// -------------------------
// webrtc::VideoFrame
// -------------------------

extern "C" {

WEBRTC_DEFINE_UNIQUE(webrtc_VideoFrame_UpdateRect, webrtc::VideoFrame::UpdateRect);
WEBRTC_DEFINE_UNIQUE(webrtc_VideoFrameBuilder, webrtc::VideoFrame::Builder);

WEBRTC_EXPORT struct webrtc_VideoFrame_UpdateRect_unique*
webrtc_VideoFrame_UpdateRect_new() {
  auto update_rect = std::make_unique<webrtc::VideoFrame::UpdateRect>();
  return reinterpret_cast<struct webrtc_VideoFrame_UpdateRect_unique*>(
      update_rect.release());
}
WEBRTC_EXPORT int webrtc_VideoFrame_UpdateRect_get_offset_x(
    const struct webrtc_VideoFrame_UpdateRect* self) {
  auto update_rect =
      reinterpret_cast<const webrtc::VideoFrame::UpdateRect*>(self);
  return update_rect->offset_x;
}
WEBRTC_EXPORT void webrtc_VideoFrame_UpdateRect_set_offset_x(
    struct webrtc_VideoFrame_UpdateRect* self,
    int value) {
  auto update_rect = reinterpret_cast<webrtc::VideoFrame::UpdateRect*>(self);
  update_rect->offset_x = value;
}
WEBRTC_EXPORT int webrtc_VideoFrame_UpdateRect_get_offset_y(
    const struct webrtc_VideoFrame_UpdateRect* self) {
  auto update_rect =
      reinterpret_cast<const webrtc::VideoFrame::UpdateRect*>(self);
  return update_rect->offset_y;
}
WEBRTC_EXPORT void webrtc_VideoFrame_UpdateRect_set_offset_y(
    struct webrtc_VideoFrame_UpdateRect* self,
    int value) {
  auto update_rect = reinterpret_cast<webrtc::VideoFrame::UpdateRect*>(self);
  update_rect->offset_y = value;
}
WEBRTC_EXPORT int webrtc_VideoFrame_UpdateRect_get_width(
    const struct webrtc_VideoFrame_UpdateRect* self) {
  auto update_rect =
      reinterpret_cast<const webrtc::VideoFrame::UpdateRect*>(self);
  return update_rect->width;
}
WEBRTC_EXPORT void webrtc_VideoFrame_UpdateRect_set_width(
    struct webrtc_VideoFrame_UpdateRect* self,
    int value) {
  auto update_rect = reinterpret_cast<webrtc::VideoFrame::UpdateRect*>(self);
  update_rect->width = value;
}
WEBRTC_EXPORT int webrtc_VideoFrame_UpdateRect_get_height(
    const struct webrtc_VideoFrame_UpdateRect* self) {
  auto update_rect =
      reinterpret_cast<const webrtc::VideoFrame::UpdateRect*>(self);
  return update_rect->height;
}
WEBRTC_EXPORT void webrtc_VideoFrame_UpdateRect_set_height(
    struct webrtc_VideoFrame_UpdateRect* self,
    int value) {
  auto update_rect = reinterpret_cast<webrtc::VideoFrame::UpdateRect*>(self);
  update_rect->height = value;
}

WEBRTC_EXPORT struct webrtc_VideoFrameBuilder_unique* webrtc_VideoFrameBuilder_new(
    struct webrtc_VideoFrameBuffer_refcounted* buffer) {
  if (buffer == nullptr) {
    return nullptr;
  }
  auto raw = webrtc_VideoFrameBuffer_refcounted_get(buffer);
  webrtc::scoped_refptr<webrtc::VideoFrameBuffer> buf(
      reinterpret_cast<webrtc::VideoFrameBuffer*>(raw));
  auto builder = std::make_unique<webrtc::VideoFrame::Builder>();
  builder->set_video_frame_buffer(buf);
  return reinterpret_cast<struct webrtc_VideoFrameBuilder_unique*>(
      builder.release());
}
WEBRTC_EXPORT struct webrtc_VideoFrame_unique* webrtc_VideoFrameBuilder_build(
    struct webrtc_VideoFrameBuilder* self) {
  auto builder = reinterpret_cast<webrtc::VideoFrame::Builder*>(self);
  auto frame = std::make_unique<webrtc::VideoFrame>(builder->build());
  return reinterpret_cast<struct webrtc_VideoFrame_unique*>(frame.release());
}
WEBRTC_EXPORT void webrtc_VideoFrameBuilder_set_timestamp_ms(
    struct webrtc_VideoFrameBuilder* self,
    int64_t timestamp_ms) {
  auto builder = reinterpret_cast<webrtc::VideoFrame::Builder*>(self);
  builder->set_timestamp_ms(timestamp_ms);
}
WEBRTC_EXPORT void webrtc_VideoFrameBuilder_set_timestamp_us(
    struct webrtc_VideoFrameBuilder* self,
    int64_t timestamp_us) {
  auto builder = reinterpret_cast<webrtc::VideoFrame::Builder*>(self);
  builder->set_timestamp_us(timestamp_us);
}
WEBRTC_EXPORT void webrtc_VideoFrameBuilder_set_presentation_timestamp_us(
    struct webrtc_VideoFrameBuilder* self,
    int has,
    int64_t presentation_timestamp_us) {
  auto builder = reinterpret_cast<webrtc::VideoFrame::Builder*>(self);
  if (has == 0) {
    builder->set_presentation_timestamp(std::nullopt);
    return;
  }
  builder->set_presentation_timestamp(
      webrtc::Timestamp::Micros(presentation_timestamp_us));
}
WEBRTC_EXPORT void webrtc_VideoFrameBuilder_set_reference_time_us(
    struct webrtc_VideoFrameBuilder* self,
    int has,
    int64_t reference_time_us) {
  auto builder = reinterpret_cast<webrtc::VideoFrame::Builder*>(self);
  if (has == 0) {
    builder->set_reference_time(std::nullopt);
    return;
  }
  builder->set_reference_time(webrtc::Timestamp::Micros(reference_time_us));
}
WEBRTC_EXPORT void webrtc_VideoFrameBuilder_set_rtp_timestamp(
    struct webrtc_VideoFrameBuilder* self,
    uint32_t rtp_timestamp) {
  auto builder = reinterpret_cast<webrtc::VideoFrame::Builder*>(self);
  builder->set_rtp_timestamp(rtp_timestamp);
}
WEBRTC_EXPORT void webrtc_VideoFrameBuilder_set_timestamp_rtp(
    struct webrtc_VideoFrameBuilder* self,
    uint32_t timestamp_rtp) {
  auto builder = reinterpret_cast<webrtc::VideoFrame::Builder*>(self);
  builder->set_timestamp_rtp(timestamp_rtp);
}
WEBRTC_EXPORT void webrtc_VideoFrameBuilder_set_ntp_time_ms(
    struct webrtc_VideoFrameBuilder* self,
    int64_t ntp_time_ms) {
  auto builder = reinterpret_cast<webrtc::VideoFrame::Builder*>(self);
  builder->set_ntp_time_ms(ntp_time_ms);
}
WEBRTC_EXPORT void webrtc_VideoFrameBuilder_set_rotation(
    struct webrtc_VideoFrameBuilder* self,
    int rotation) {
  auto builder = reinterpret_cast<webrtc::VideoFrame::Builder*>(self);
  builder->set_rotation(static_cast<webrtc::VideoRotation>(rotation));
}
WEBRTC_EXPORT void webrtc_VideoFrameBuilder_set_color_space(
    struct webrtc_VideoFrameBuilder* self,
    int has,
    const struct webrtc_ColorSpace* color_space) {
  auto builder = reinterpret_cast<webrtc::VideoFrame::Builder*>(self);
  if (has == 0) {
    builder->set_color_space(std::nullopt);
    return;
  }
  auto value = reinterpret_cast<const webrtc::ColorSpace*>(color_space);
  if (value == nullptr) {
    builder->set_color_space(std::nullopt);
    return;
  }
  builder->set_color_space(*value);
}
WEBRTC_EXPORT void webrtc_VideoFrameBuilder_set_id(
    struct webrtc_VideoFrameBuilder* self,
    uint16_t id) {
  auto builder = reinterpret_cast<webrtc::VideoFrame::Builder*>(self);
  builder->set_id(id);
}
WEBRTC_EXPORT void webrtc_VideoFrameBuilder_set_update_rect(
    struct webrtc_VideoFrameBuilder* self,
    int has,
    const struct webrtc_VideoFrame_UpdateRect* update_rect) {
  auto builder = reinterpret_cast<webrtc::VideoFrame::Builder*>(self);
  if (has == 0) {
    builder->set_update_rect(std::nullopt);
    return;
  }
  auto value = reinterpret_cast<const webrtc::VideoFrame::UpdateRect*>(
      update_rect);
  if (value == nullptr) {
    builder->set_update_rect(std::nullopt);
    return;
  }
  builder->set_update_rect(*value);
}
WEBRTC_EXPORT void webrtc_VideoFrameBuilder_set_is_repeat_frame(
    struct webrtc_VideoFrameBuilder* self,
    int is_repeat_frame) {
  auto builder = reinterpret_cast<webrtc::VideoFrame::Builder*>(self);
  builder->set_is_repeat_frame(is_repeat_frame != 0);
}

WEBRTC_DEFINE_UNIQUE(webrtc_VideoFrame, webrtc::VideoFrame);
WEBRTC_EXPORT struct webrtc_VideoFrame_unique* webrtc_VideoFrame_copy(
    const struct webrtc_VideoFrame* self) {
  auto frame = reinterpret_cast<const webrtc::VideoFrame*>(self);
  if (frame == nullptr) {
    return nullptr;
  }
  auto copied = std::make_unique<webrtc::VideoFrame>(*frame);
  return reinterpret_cast<struct webrtc_VideoFrame_unique*>(copied.release());
}
WEBRTC_EXPORT int webrtc_VideoFrame_width(
    const struct webrtc_VideoFrame* self) {
  auto frame = reinterpret_cast<const webrtc::VideoFrame*>(self);
  return frame->width();
}
WEBRTC_EXPORT int webrtc_VideoFrame_height(
    const struct webrtc_VideoFrame* self) {
  auto frame = reinterpret_cast<const webrtc::VideoFrame*>(self);
  return frame->height();
}
WEBRTC_EXPORT int64_t
webrtc_VideoFrame_timestamp_us(const struct webrtc_VideoFrame* self) {
  auto frame = reinterpret_cast<const webrtc::VideoFrame*>(self);
  return frame->timestamp_us();
}
WEBRTC_EXPORT uint32_t
webrtc_VideoFrame_timestamp_rtp(const struct webrtc_VideoFrame* self) {
  auto frame = reinterpret_cast<const webrtc::VideoFrame*>(self);
  return frame->rtp_timestamp();
}
WEBRTC_EXPORT uint16_t webrtc_VideoFrame_id(
    const struct webrtc_VideoFrame* self) {
  auto frame = reinterpret_cast<const webrtc::VideoFrame*>(self);
  return frame->id();
}
WEBRTC_EXPORT int64_t
webrtc_VideoFrame_ntp_time_ms(const struct webrtc_VideoFrame* self) {
  auto frame = reinterpret_cast<const webrtc::VideoFrame*>(self);
  return frame->ntp_time_ms();
}
WEBRTC_EXPORT void webrtc_VideoFrame_presentation_timestamp_us(
    const struct webrtc_VideoFrame* self,
    int* out_has,
    int64_t* out_value) {
  auto frame = reinterpret_cast<const webrtc::VideoFrame*>(self);
  const auto& timestamp = frame->presentation_timestamp();
  webrtc_c::OptionalGetAs(timestamp, out_has, out_value,
                          [&]() { return timestamp.value().us(); });
}
WEBRTC_EXPORT void webrtc_VideoFrame_reference_time_us(
    const struct webrtc_VideoFrame* self,
    int* out_has,
    int64_t* out_value) {
  auto frame = reinterpret_cast<const webrtc::VideoFrame*>(self);
  const auto& reference_time = frame->reference_time();
  webrtc_c::OptionalGetAs(reference_time, out_has, out_value,
                          [&]() { return reference_time.value().us(); });
}
WEBRTC_EXPORT int webrtc_VideoFrame_rotation(
    const struct webrtc_VideoFrame* self) {
  auto frame = reinterpret_cast<const webrtc::VideoFrame*>(self);
  return static_cast<int>(frame->rotation());
}
WEBRTC_EXPORT void webrtc_VideoFrame_color_space(
    const struct webrtc_VideoFrame* self,
    int* out_has,
    struct webrtc_ColorSpace_unique** out_value) {
  auto frame = reinterpret_cast<const webrtc::VideoFrame*>(self);
  const auto& color_space = frame->color_space();
  const bool has_value = color_space.has_value();
  if (out_has != nullptr) {
    *out_has = has_value ? 1 : 0;
  }
  if (out_value == nullptr) {
    return;
  }
  if (!has_value) {
    *out_value = nullptr;
    return;
  }
  auto copied = std::make_unique<webrtc::ColorSpace>(*color_space);
  *out_value =
      reinterpret_cast<struct webrtc_ColorSpace_unique*>(copied.release());
}
WEBRTC_EXPORT int webrtc_VideoFrame_has_update_rect(
    const struct webrtc_VideoFrame* self) {
  auto frame = reinterpret_cast<const webrtc::VideoFrame*>(self);
  return frame->has_update_rect() ? 1 : 0;
}
WEBRTC_EXPORT struct webrtc_VideoFrame_UpdateRect_unique* webrtc_VideoFrame_update_rect(
    const struct webrtc_VideoFrame* self) {
  auto frame = reinterpret_cast<const webrtc::VideoFrame*>(self);
  auto update_rect = std::make_unique<webrtc::VideoFrame::UpdateRect>(
      frame->update_rect());
  return reinterpret_cast<struct webrtc_VideoFrame_UpdateRect_unique*>(
      update_rect.release());
}
WEBRTC_EXPORT int webrtc_VideoFrame_is_repeat_frame(
    const struct webrtc_VideoFrame* self) {
  auto frame = reinterpret_cast<const webrtc::VideoFrame*>(self);
  return frame->is_repeat_frame() ? 1 : 0;
}
WEBRTC_EXPORT struct webrtc_VideoFrameBuffer_refcounted*
webrtc_VideoFrame_video_frame_buffer(const struct webrtc_VideoFrame* self) {
  auto frame = reinterpret_cast<const webrtc::VideoFrame*>(self);
  auto buf = frame->video_frame_buffer();
  return reinterpret_cast<struct webrtc_VideoFrameBuffer_refcounted*>(
      buf.release());
}
}
