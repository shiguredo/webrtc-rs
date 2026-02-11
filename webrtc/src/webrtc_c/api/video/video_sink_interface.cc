#include "video_sink_interface.h"

#include <api/video/video_frame.h>
#include <api/video/video_sink_interface.h>
#include "video_frame.h"

class VideoSinkInterfaceImpl
    : public webrtc::VideoSinkInterface<webrtc::VideoFrame> {
 public:
  VideoSinkInterfaceImpl(struct webrtc_VideoSinkInterface_cbs* cbs,
                         void* user_data)
      : cbs_(cbs), user_data_(user_data) {}

  void OnFrame(const webrtc::VideoFrame& frame) override {
    if (cbs_ != nullptr && cbs_->OnFrame != nullptr) {
      auto* frame_ptr =
          reinterpret_cast<const struct webrtc_VideoFrame*>(&frame);
      cbs_->OnFrame(frame_ptr, user_data_);
    }
  }

  void OnDiscardedFrame() override {
    if (cbs_ != nullptr && cbs_->OnDiscardedFrame != nullptr) {
      cbs_->OnDiscardedFrame(user_data_);
    }
  }

 private:
  struct webrtc_VideoSinkInterface_cbs* cbs_;
  void* user_data_;
};

extern "C" {
struct webrtc_VideoSinkInterface* webrtc_VideoSinkInterface_new(
    struct webrtc_VideoSinkInterface_cbs* cbs,
    void* user_data) {
  auto sink = new VideoSinkInterfaceImpl(cbs, user_data);
  return reinterpret_cast<struct webrtc_VideoSinkInterface*>(sink);
}

void webrtc_VideoSinkInterface_delete(struct webrtc_VideoSinkInterface* self) {
  auto sink = reinterpret_cast<VideoSinkInterfaceImpl*>(self);
  delete sink;
}
}
