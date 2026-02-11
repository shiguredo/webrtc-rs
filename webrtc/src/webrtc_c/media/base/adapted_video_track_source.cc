#include "adapted_video_track_source.h"

#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>
#include <optional>

// WebRTC
#include <api/make_ref_counted.h>
#include <api/media_stream_interface.h>
#include <api/scoped_refptr.h>
#include <api/video/video_frame.h>
#include <media/base/adapted_video_track_source.h>

#include "../../api/video/video_frame.h"
#include "../../common.impl.h"

// -------------------------
// webrtc::AdaptedVideoTrackSource
// -------------------------

class AdaptedVideoTrackSourceWrapper : public webrtc::AdaptedVideoTrackSource {
 public:
  AdaptedVideoTrackSourceWrapper() = default;

  // これらは必要になったら webrtc_AdaptedVideoTrackSource_cbs みたいな構造体を用意して
  // C からコールバックを登録できるようにする。
  bool is_screencast() const override { return false; }
  std::optional<bool> needs_denoising() const override { return false; }
  webrtc::MediaSourceInterface::SourceState state() const override {
    return webrtc::MediaSourceInterface::kLive;
  }
  bool remote() const override { return false; }

  // AdaptFrame と OnFrame は protected メソッドなので、public メソッドを
  // 経由して呼び出せるようにする。
  bool AdaptFramePublic(int width,
                        int height,
                        int64_t timestamp_us,
                        int* adapted_width,
                        int* adapted_height,
                        int* crop_width,
                        int* crop_height,
                        int* crop_x,
                        int* crop_y) {
    return AdaptFrame(width, height, timestamp_us, adapted_width,
                      adapted_height, crop_width, crop_height, crop_x, crop_y);
  }
  void OnFramePublic(const webrtc::VideoFrame& frame) { OnFrame(frame); }
};

extern "C" {
WEBRTC_DEFINE_REFCOUNTED(webrtc_AdaptedVideoTrackSource,
                         AdaptedVideoTrackSourceWrapper);

struct webrtc_AdaptedVideoTrackSource_refcounted*
webrtc_AdaptedVideoTrackSource_Create() {
  auto src = webrtc::make_ref_counted<AdaptedVideoTrackSourceWrapper>();
  return reinterpret_cast<struct webrtc_AdaptedVideoTrackSource_refcounted*>(
      src.release());
}

int webrtc_AdaptedVideoTrackSource_AdaptFrame(
    struct webrtc_AdaptedVideoTrackSource* self,
    int width,
    int height,
    int64_t timestamp_us,
    int* out_adapted_width,
    int* out_adapted_height,
    int* out_crop_width,
    int* out_crop_height,
    int* out_crop_x,
    int* out_crop_y) {
  auto src = reinterpret_cast<AdaptedVideoTrackSourceWrapper*>(self);
  return src->AdaptFramePublic(width, height, timestamp_us, out_adapted_width,
                               out_adapted_height, out_crop_width,
                               out_crop_height, out_crop_x, out_crop_y)
             ? 1
             : 0;
}

void webrtc_AdaptedVideoTrackSource_OnFrame(
    struct webrtc_AdaptedVideoTrackSource* self,
    struct webrtc_VideoFrame* frame) {
  auto src = reinterpret_cast<AdaptedVideoTrackSourceWrapper*>(self);
  auto f = reinterpret_cast<webrtc::VideoFrame*>(frame);
  src->OnFramePublic(*f);
}
WEBRTC_DEFINE_CAST_REFCOUNTED(webrtc_AdaptedVideoTrackSource,
                              webrtc_VideoTrackSourceInterface,
                              AdaptedVideoTrackSourceWrapper,
                              webrtc::VideoTrackSourceInterface);
}
