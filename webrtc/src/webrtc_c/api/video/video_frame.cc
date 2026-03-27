#include "video_frame.h"

#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>
#include <memory>

// WebRTC
#include <api/scoped_refptr.h>
#include <api/video/i420_buffer.h>
#include <api/video/video_frame.h>
#include <api/video/video_rotation.h>

#include "../../common.h"
#include "../../common.impl.h"
#include "i420_buffer.h"

// -------------------------
// webrtc::VideoFrame
// -------------------------

extern "C" {

WEBRTC_EXPORT const int webrtc_VideoRotation_0 =
    static_cast<int>(webrtc::kVideoRotation_0);
WEBRTC_EXPORT const int webrtc_VideoRotation_90 =
    static_cast<int>(webrtc::kVideoRotation_90);
WEBRTC_EXPORT const int webrtc_VideoRotation_180 =
    static_cast<int>(webrtc::kVideoRotation_180);
WEBRTC_EXPORT const int webrtc_VideoRotation_270 =
    static_cast<int>(webrtc::kVideoRotation_270);

WEBRTC_DEFINE_UNIQUE(webrtc_VideoFrame, webrtc::VideoFrame);
WEBRTC_EXPORT struct webrtc_VideoFrame_unique* webrtc_VideoFrame_Create(
    struct webrtc_I420Buffer_refcounted* buffer,
    int rotation,
    int64_t timestamp_us,
    uint32_t timestamp_rtp) {
  webrtc::scoped_refptr<webrtc::I420Buffer> buf(
      reinterpret_cast<webrtc::I420Buffer*>(buffer));
  auto frame = std::make_unique<webrtc::VideoFrame>(
      webrtc::VideoFrame::Builder()
          .set_video_frame_buffer(buf)
          .set_rotation(static_cast<webrtc::VideoRotation>(rotation))
          .set_timestamp_us(timestamp_us)
          .set_timestamp_rtp(timestamp_rtp)
          .build());
  return reinterpret_cast<struct webrtc_VideoFrame_unique*>(frame.release());
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
WEBRTC_EXPORT int webrtc_VideoFrame_rotation(
    const struct webrtc_VideoFrame* self) {
  auto frame = reinterpret_cast<const webrtc::VideoFrame*>(self);
  return static_cast<int>(frame->rotation());
}
WEBRTC_EXPORT struct webrtc_I420Buffer_refcounted*
webrtc_VideoFrame_video_frame_buffer(const struct webrtc_VideoFrame* self) {
  auto frame = reinterpret_cast<const webrtc::VideoFrame*>(self);
  auto buf = frame->video_frame_buffer()->ToI420();
  return reinterpret_cast<struct webrtc_I420Buffer_refcounted*>(buf.release());
}

}
