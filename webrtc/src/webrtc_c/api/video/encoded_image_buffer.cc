#include "encoded_image_buffer.h"

#include <cassert>

// WebRTC
#include <api/scoped_refptr.h>
#include <api/video/encoded_image.h>

#include "../../common.impl.h"

extern "C" {
WEBRTC_DEFINE_REFCOUNTED(webrtc_EncodedImageBuffer,
                         webrtc::EncodedImageBufferInterface);

struct webrtc_EncodedImageBuffer_refcounted* webrtc_EncodedImageBuffer_Create() {
  auto buffer = webrtc::EncodedImageBuffer::Create();
  return reinterpret_cast<struct webrtc_EncodedImageBuffer_refcounted*>(
      buffer.release());
}

struct webrtc_EncodedImageBuffer_refcounted*
webrtc_EncodedImageBuffer_Create_from_data(const uint8_t* data, size_t size) {
  auto buffer = webrtc::EncodedImageBuffer::Create(data, size);
  return reinterpret_cast<struct webrtc_EncodedImageBuffer_refcounted*>(
      buffer.release());
}

size_t webrtc_EncodedImageBuffer_size(struct webrtc_EncodedImageBuffer* self) {
  assert(self != nullptr);
  auto buffer = reinterpret_cast<webrtc::EncodedImageBufferInterface*>(self);
  return buffer->size();
}

const uint8_t* webrtc_EncodedImageBuffer_data(
    struct webrtc_EncodedImageBuffer* self) {
  assert(self != nullptr);
  auto buffer = reinterpret_cast<webrtc::EncodedImageBufferInterface*>(self);
  return buffer->data();
}
}
