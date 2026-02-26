#include "i420_buffer.h"

#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>

// WebRTC
#include <api/scoped_refptr.h>
#include <api/video/i420_buffer.h>

#include "../../common.impl.h"

// -------------------------
// webrtc::I420Buffer
// -------------------------

extern "C" {
WEBRTC_DEFINE_REFCOUNTED(webrtc_I420Buffer, webrtc::I420Buffer);
struct webrtc_I420Buffer_refcounted* webrtc_I420Buffer_Create(int width,
                                                              int height) {
  auto buf = webrtc::I420Buffer::Create(width, height);
  return reinterpret_cast<struct webrtc_I420Buffer_refcounted*>(buf.release());
}
int webrtc_I420Buffer_width(const struct webrtc_I420Buffer* self) {
  auto buf = reinterpret_cast<const webrtc::I420Buffer*>(self);
  return buf->width();
}
int webrtc_I420Buffer_height(const struct webrtc_I420Buffer* self) {
  auto buf = reinterpret_cast<const webrtc::I420Buffer*>(self);
  return buf->height();
}
uint8_t* webrtc_I420Buffer_MutableDataY(struct webrtc_I420Buffer* self) {
  auto buf = reinterpret_cast<webrtc::I420Buffer*>(self);
  return buf->MutableDataY();
}
uint8_t* webrtc_I420Buffer_MutableDataU(struct webrtc_I420Buffer* self) {
  auto buf = reinterpret_cast<webrtc::I420Buffer*>(self);
  return buf->MutableDataU();
}
uint8_t* webrtc_I420Buffer_MutableDataV(struct webrtc_I420Buffer* self) {
  auto buf = reinterpret_cast<webrtc::I420Buffer*>(self);
  return buf->MutableDataV();
}
int webrtc_I420Buffer_StrideY(struct webrtc_I420Buffer* self) {
  auto buf = reinterpret_cast<webrtc::I420Buffer*>(self);
  return buf->StrideY();
}
int webrtc_I420Buffer_StrideU(struct webrtc_I420Buffer* self) {
  auto buf = reinterpret_cast<webrtc::I420Buffer*>(self);
  return buf->StrideU();
}
int webrtc_I420Buffer_StrideV(struct webrtc_I420Buffer* self) {
  auto buf = reinterpret_cast<webrtc::I420Buffer*>(self);
  return buf->StrideV();
}
void webrtc_I420Buffer_ScaleFrom(struct webrtc_I420Buffer* self,
                                 struct webrtc_I420Buffer* src) {
  auto dst_buf = reinterpret_cast<webrtc::I420Buffer*>(self);
  auto src_buf = reinterpret_cast<webrtc::I420Buffer*>(src);
  dst_buf->ScaleFrom(*src_buf);
}
}
