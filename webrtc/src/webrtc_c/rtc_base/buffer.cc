#include "buffer.h"

#include <cstddef>
#include <cstdint>

// WebRTC
#include <rtc_base/buffer.h>

#include "../common.h"

extern "C" {
WEBRTC_EXPORT struct webrtc_Buffer* webrtc_Buffer_new() {
  auto buffer = new webrtc::Buffer();
  return reinterpret_cast<struct webrtc_Buffer*>(buffer);
}
WEBRTC_EXPORT void webrtc_Buffer_delete(struct webrtc_Buffer* self) {
  auto buffer = reinterpret_cast<webrtc::Buffer*>(self);
  delete buffer;
}
WEBRTC_EXPORT void webrtc_Buffer_Clear(struct webrtc_Buffer* self) {
  auto buffer = reinterpret_cast<webrtc::Buffer*>(self);
  buffer->Clear();
}
WEBRTC_EXPORT void webrtc_Buffer_AppendData(struct webrtc_Buffer* self,
                                            const uint8_t* data,
                                            size_t len) {
  auto buffer = reinterpret_cast<webrtc::Buffer*>(self);
  buffer->AppendData(data, len);
}
WEBRTC_EXPORT size_t webrtc_Buffer_size(const struct webrtc_Buffer* self) {
  auto buffer = reinterpret_cast<const webrtc::Buffer*>(self);
  return buffer->size();
}
WEBRTC_EXPORT const uint8_t* webrtc_Buffer_data(
    const struct webrtc_Buffer* self) {
  auto buffer = reinterpret_cast<const webrtc::Buffer*>(self);
  return buffer->data();
}

// -------------------------
// webrtc::BufferT<int16_t>
// -------------------------

WEBRTC_EXPORT void webrtc_BufferS16_Clear(struct webrtc_BufferS16* self) {
  auto buffer = reinterpret_cast<webrtc::BufferT<int16_t>*>(self);
  buffer->Clear();
}
WEBRTC_EXPORT void webrtc_BufferS16_AppendData(struct webrtc_BufferS16* self,
                                               const int16_t* data,
                                               size_t len) {
  auto buffer = reinterpret_cast<webrtc::BufferT<int16_t>*>(self);
  buffer->AppendData(data, len);
}
WEBRTC_EXPORT size_t
webrtc_BufferS16_size(const struct webrtc_BufferS16* self) {
  auto buffer = reinterpret_cast<const webrtc::BufferT<int16_t>*>(self);
  return buffer->size();
}
WEBRTC_EXPORT const int16_t* webrtc_BufferS16_data(
    const struct webrtc_BufferS16* self) {
  auto buffer = reinterpret_cast<const webrtc::BufferT<int16_t>*>(self);
  return buffer->data();
}
}
