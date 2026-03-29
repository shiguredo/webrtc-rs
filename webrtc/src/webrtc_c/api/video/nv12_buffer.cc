#include "nv12_buffer.h"

#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>

// WebRTC
#include <api/scoped_refptr.h>
#include <api/video/nv12_buffer.h>

#include "../../common.h"
#include "../../common.impl.h"

// -------------------------
// webrtc::NV12Buffer
// -------------------------

extern "C" {
WEBRTC_DEFINE_REFCOUNTED(webrtc_NV12Buffer, webrtc::NV12Buffer);
WEBRTC_DEFINE_CAST_REFCOUNTED(webrtc_NV12Buffer,
                              webrtc_VideoFrameBuffer,
                              webrtc::NV12Buffer,
                              webrtc::VideoFrameBuffer);
WEBRTC_EXPORT struct webrtc_NV12Buffer_refcounted* webrtc_NV12Buffer_Create(
    int width,
    int height) {
  auto buf = webrtc::NV12Buffer::Create(width, height);
  return reinterpret_cast<struct webrtc_NV12Buffer_refcounted*>(buf.release());
}
WEBRTC_EXPORT int webrtc_NV12Buffer_width(
    const struct webrtc_NV12Buffer* self) {
  auto buf = reinterpret_cast<const webrtc::NV12Buffer*>(self);
  return buf->width();
}
WEBRTC_EXPORT int webrtc_NV12Buffer_height(
    const struct webrtc_NV12Buffer* self) {
  auto buf = reinterpret_cast<const webrtc::NV12Buffer*>(self);
  return buf->height();
}
WEBRTC_EXPORT int webrtc_NV12Buffer_chroma_width(
    const struct webrtc_NV12Buffer* self) {
  auto buf = reinterpret_cast<const webrtc::NV12Buffer*>(self);
  return (buf->width() + 1) / 2;
}
WEBRTC_EXPORT int webrtc_NV12Buffer_chroma_height(
    const struct webrtc_NV12Buffer* self) {
  auto buf = reinterpret_cast<const webrtc::NV12Buffer*>(self);
  return (buf->height() + 1) / 2;
}
WEBRTC_EXPORT uint8_t* webrtc_NV12Buffer_MutableDataY(
    struct webrtc_NV12Buffer* self) {
  auto buf = reinterpret_cast<webrtc::NV12Buffer*>(self);
  return buf->MutableDataY();
}
WEBRTC_EXPORT uint8_t* webrtc_NV12Buffer_MutableDataUV(
    struct webrtc_NV12Buffer* self) {
  auto buf = reinterpret_cast<webrtc::NV12Buffer*>(self);
  return buf->MutableDataUV();
}
WEBRTC_EXPORT int webrtc_NV12Buffer_StrideY(struct webrtc_NV12Buffer* self) {
  auto buf = reinterpret_cast<webrtc::NV12Buffer*>(self);
  return buf->StrideY();
}
WEBRTC_EXPORT int webrtc_NV12Buffer_StrideUV(struct webrtc_NV12Buffer* self) {
  auto buf = reinterpret_cast<webrtc::NV12Buffer*>(self);
  return buf->StrideUV();
}
WEBRTC_EXPORT void webrtc_NV12Buffer_CropAndScaleFrom(
    struct webrtc_NV12Buffer* self,
    struct webrtc_NV12Buffer* src,
    int offset_x,
    int offset_y,
    int crop_width,
    int crop_height) {
  auto dst_buf = reinterpret_cast<webrtc::NV12Buffer*>(self);
  auto src_buf = reinterpret_cast<webrtc::NV12Buffer*>(src);
  dst_buf->CropAndScaleFrom(*src_buf, offset_x, offset_y, crop_width,
                            crop_height);
}
}
