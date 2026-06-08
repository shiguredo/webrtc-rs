#include "video_sink_interface.h"

#include <assert.h>

#include <api/video/video_frame.h>
#include <api/video/video_sink_interface.h>

#include "../../common.h"
#include "video_frame.h"

class VideoSinkInterfaceImpl
    : public webrtc::VideoSinkInterface<webrtc::VideoFrame> {
 public:
  VideoSinkInterfaceImpl(const struct webrtc_VideoSinkInterface_cbs* cbs,
                         void* user_data)
      : user_data_(user_data) {
    assert(cbs != nullptr);
    assert(cbs->OnFrame != nullptr);
    assert(cbs->OnDiscardedFrame != nullptr);
    assert(cbs->OnDestroy != nullptr);
    cbs_ = *cbs;
  }

  ~VideoSinkInterfaceImpl() override { cbs_.OnDestroy(user_data_); }

  void OnFrame(const webrtc::VideoFrame& frame) override {
    auto* frame_ptr = reinterpret_cast<const struct webrtc_VideoFrame*>(&frame);
    cbs_.OnFrame(frame_ptr, user_data_);
  }

  void OnDiscardedFrame() override { cbs_.OnDiscardedFrame(user_data_); }

 private:
  webrtc_VideoSinkInterface_cbs cbs_{};
  void* user_data_;
};

extern "C" {
WEBRTC_EXPORT struct webrtc_VideoSinkInterface* webrtc_VideoSinkInterface_new(
    const struct webrtc_VideoSinkInterface_cbs* cbs,
    void* user_data) {
  auto sink = new VideoSinkInterfaceImpl(cbs, user_data);
  return reinterpret_cast<struct webrtc_VideoSinkInterface*>(sink);
}

WEBRTC_EXPORT void webrtc_VideoSinkInterface_delete(
    struct webrtc_VideoSinkInterface* self) {
  auto sink = reinterpret_cast<VideoSinkInterfaceImpl*>(self);
  delete sink;
}
}
