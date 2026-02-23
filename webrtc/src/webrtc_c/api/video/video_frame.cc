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

#include "../../common.impl.h"
#include "i420_buffer.h"

// -------------------------
// webrtc::VideoFrame
// -------------------------

extern "C" {
const int webrtc_VideoRotation_0 = static_cast<int>(webrtc::kVideoRotation_0);

WEBRTC_DEFINE_UNIQUE(webrtc_VideoFrame, webrtc::VideoFrame);
struct webrtc_VideoFrame_unique* webrtc_VideoFrame_Create(
    struct webrtc_I420Buffer_refcounted* buffer,
    int rotation,
    int64_t timestamp_us) {
  return webrtc_VideoFrame_Create_with_timestamp_rtp(buffer, rotation,
                                                     timestamp_us, 0);
}
struct webrtc_VideoFrame_unique* webrtc_VideoFrame_Create_with_timestamp_rtp(
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
int webrtc_VideoFrame_width(const struct webrtc_VideoFrame* self) {
  auto frame = reinterpret_cast<const webrtc::VideoFrame*>(self);
  return frame->width();
}
int webrtc_VideoFrame_height(const struct webrtc_VideoFrame* self) {
  auto frame = reinterpret_cast<const webrtc::VideoFrame*>(self);
  return frame->height();
}
int64_t webrtc_VideoFrame_timestamp_us(const struct webrtc_VideoFrame* self) {
  auto frame = reinterpret_cast<const webrtc::VideoFrame*>(self);
  return frame->timestamp_us();
}
uint32_t webrtc_VideoFrame_timestamp_rtp(const struct webrtc_VideoFrame* self) {
  auto frame = reinterpret_cast<const webrtc::VideoFrame*>(self);
  return frame->rtp_timestamp();
}
struct webrtc_I420Buffer_refcounted* webrtc_VideoFrame_video_frame_buffer(
    const struct webrtc_VideoFrame* self) {
  auto frame = reinterpret_cast<const webrtc::VideoFrame*>(self);
  auto buf = frame->video_frame_buffer()->ToI420();
  return reinterpret_cast<struct webrtc_I420Buffer_refcounted*>(buf.release());
}
}
