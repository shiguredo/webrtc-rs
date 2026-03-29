#include "video_frame_buffer.h"

#include <assert.h>
#include <stddef.h>
#include <stdint.h>

// WebRTC
#include <api/make_ref_counted.h>
#include <api/scoped_refptr.h>
#include <api/video/i420_buffer.h>
#include <api/video/video_frame_buffer.h>

#include "../../common.h"
#include "../../common.impl.h"
#include "i420_buffer.h"

namespace {

class VideoFrameBufferImpl : public webrtc::VideoFrameBuffer {
 public:
  VideoFrameBufferImpl(const struct webrtc_VideoFrameBuffer_cbs* cbs,
                       void* user_data)
      : user_data_(user_data) {
    if (cbs != nullptr) {
      cbs_ = *cbs;
    }
  }

  ~VideoFrameBufferImpl() override {
    if (cbs_.OnDestroy != nullptr) {
      cbs_.OnDestroy(user_data_);
    }
  }

  Type type() const override {
    if (cbs_.type == nullptr) {
      return Type::kNative;
    }
    return static_cast<Type>(cbs_.type(user_data_));
  }

  int width() const override {
    if (cbs_.width == nullptr) {
      return 0;
    }
    return cbs_.width(user_data_);
  }

  int height() const override {
    if (cbs_.height == nullptr) {
      return 0;
    }
    return cbs_.height(user_data_);
  }

  webrtc::scoped_refptr<webrtc::I420BufferInterface> ToI420() override {
    if (cbs_.ToI420 == nullptr) {
      return nullptr;
    }
    auto raw_ref = cbs_.ToI420(user_data_);
    if (raw_ref == nullptr) {
      return nullptr;
    }
    auto raw = webrtc_I420Buffer_refcounted_get(raw_ref);
    assert(raw != nullptr);
    if (raw == nullptr) {
      return nullptr;
    }
    auto i420 = reinterpret_cast<webrtc::I420Buffer*>(raw);
    webrtc::scoped_refptr<webrtc::I420Buffer> buffer(i420);
    webrtc_I420Buffer_Release(reinterpret_cast<struct webrtc_I420Buffer*>(raw));
    return buffer;
  }

 private:
  webrtc_VideoFrameBuffer_cbs cbs_{};
  void* user_data_ = nullptr;
};

}  // namespace

extern "C" {
WEBRTC_DEFINE_REFCOUNTED(webrtc_VideoFrameBuffer, webrtc::VideoFrameBuffer);

WEBRTC_EXPORT int webrtc_VideoFrameBuffer_type(
    const struct webrtc_VideoFrameBuffer* self) {
  auto buffer = reinterpret_cast<const webrtc::VideoFrameBuffer*>(self);
  return static_cast<int>(buffer->type());
}

WEBRTC_EXPORT int webrtc_VideoFrameBuffer_width(
    const struct webrtc_VideoFrameBuffer* self) {
  auto buffer = reinterpret_cast<const webrtc::VideoFrameBuffer*>(self);
  return buffer->width();
}

WEBRTC_EXPORT int webrtc_VideoFrameBuffer_height(
    const struct webrtc_VideoFrameBuffer* self) {
  auto buffer = reinterpret_cast<const webrtc::VideoFrameBuffer*>(self);
  return buffer->height();
}

WEBRTC_EXPORT struct webrtc_I420Buffer_refcounted* webrtc_VideoFrameBuffer_ToI420(
    struct webrtc_VideoFrameBuffer* self) {
  auto buffer = reinterpret_cast<webrtc::VideoFrameBuffer*>(self);
  auto i420 = buffer->ToI420();
  if (i420 == nullptr) {
    return nullptr;
  }
  auto copied = webrtc::I420Buffer::Copy(*i420);
  if (copied == nullptr) {
    return nullptr;
  }
  return reinterpret_cast<struct webrtc_I420Buffer_refcounted*>(copied.release());
}

WEBRTC_EXPORT struct webrtc_VideoFrameBuffer_refcounted*
webrtc_VideoFrameBuffer_make_ref_counted(
    const struct webrtc_VideoFrameBuffer_cbs* cbs,
    void* user_data) {
  auto impl = webrtc::make_ref_counted<VideoFrameBufferImpl>(cbs, user_data);
  return reinterpret_cast<struct webrtc_VideoFrameBuffer_refcounted*>(
      impl.release());
}
}
