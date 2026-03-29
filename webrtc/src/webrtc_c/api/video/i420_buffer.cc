#include "i420_buffer.h"

#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>

// WebRTC
#include <api/scoped_refptr.h>
#include <api/video/i420_buffer.h>

#include "../../common.h"
#include "../../common.impl.h"

// -------------------------
// webrtc::I420Buffer
// -------------------------

extern "C" {
WEBRTC_DEFINE_REFCOUNTED(webrtc_I420Buffer, webrtc::I420Buffer);
WEBRTC_DEFINE_CAST_REFCOUNTED(webrtc_I420Buffer,
                              webrtc_VideoFrameBuffer,
                              webrtc::I420Buffer,
                              webrtc::VideoFrameBuffer);
WEBRTC_EXPORT struct webrtc_I420Buffer_refcounted* webrtc_I420Buffer_Create(
    int width,
    int height) {
  auto buf = webrtc::I420Buffer::Create(width, height);
  return reinterpret_cast<struct webrtc_I420Buffer_refcounted*>(buf.release());
}
WEBRTC_EXPORT int webrtc_I420Buffer_width(
    const struct webrtc_I420Buffer* self) {
  auto buf = reinterpret_cast<const webrtc::I420Buffer*>(self);
  return buf->width();
}
WEBRTC_EXPORT int webrtc_I420Buffer_height(
    const struct webrtc_I420Buffer* self) {
  auto buf = reinterpret_cast<const webrtc::I420Buffer*>(self);
  return buf->height();
}
WEBRTC_EXPORT int webrtc_I420Buffer_chroma_width(
    const struct webrtc_I420Buffer* self) {
  auto buf = reinterpret_cast<const webrtc::I420Buffer*>(self);
  return buf->ChromaWidth();
}
WEBRTC_EXPORT int webrtc_I420Buffer_chroma_height(
    const struct webrtc_I420Buffer* self) {
  auto buf = reinterpret_cast<const webrtc::I420Buffer*>(self);
  return buf->ChromaHeight();
}
WEBRTC_EXPORT uint8_t* webrtc_I420Buffer_MutableDataY(
    struct webrtc_I420Buffer* self) {
  auto buf = reinterpret_cast<webrtc::I420Buffer*>(self);
  return buf->MutableDataY();
}
WEBRTC_EXPORT uint8_t* webrtc_I420Buffer_MutableDataU(
    struct webrtc_I420Buffer* self) {
  auto buf = reinterpret_cast<webrtc::I420Buffer*>(self);
  return buf->MutableDataU();
}
WEBRTC_EXPORT uint8_t* webrtc_I420Buffer_MutableDataV(
    struct webrtc_I420Buffer* self) {
  auto buf = reinterpret_cast<webrtc::I420Buffer*>(self);
  return buf->MutableDataV();
}
WEBRTC_EXPORT int webrtc_I420Buffer_StrideY(struct webrtc_I420Buffer* self) {
  auto buf = reinterpret_cast<webrtc::I420Buffer*>(self);
  return buf->StrideY();
}
WEBRTC_EXPORT int webrtc_I420Buffer_StrideU(struct webrtc_I420Buffer* self) {
  auto buf = reinterpret_cast<webrtc::I420Buffer*>(self);
  return buf->StrideU();
}
WEBRTC_EXPORT int webrtc_I420Buffer_StrideV(struct webrtc_I420Buffer* self) {
  auto buf = reinterpret_cast<webrtc::I420Buffer*>(self);
  return buf->StrideV();
}
WEBRTC_EXPORT void webrtc_I420Buffer_ScaleFrom(struct webrtc_I420Buffer* self,
                                               struct webrtc_I420Buffer* src) {
  auto dst_buf = reinterpret_cast<webrtc::I420Buffer*>(self);
  auto src_buf = reinterpret_cast<webrtc::I420Buffer*>(src);
  dst_buf->ScaleFrom(*src_buf);
}
}
