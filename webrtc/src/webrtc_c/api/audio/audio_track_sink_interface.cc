#include "audio_track_sink_interface.h"

#include <stddef.h>

#include <api/media_stream_interface.h>

#include "../../common.h"

class AudioTrackSinkInterfaceImpl : public webrtc::AudioTrackSinkInterface {
 public:
  AudioTrackSinkInterfaceImpl(
      const struct webrtc_AudioTrackSinkInterface_cbs* cbs,
      void* user_data)
      : user_data_(user_data) {
    if (cbs != nullptr) {
      cbs_ = *cbs;
    }
  }

  ~AudioTrackSinkInterfaceImpl() override {
    if (cbs_.OnDestroy != nullptr) {
      cbs_.OnDestroy(user_data_);
    }
  }

  void OnData(const void* audio_data,
              int bits_per_sample,
              int sample_rate,
              size_t number_of_channels,
              size_t number_of_frames) override {
    if (cbs_.OnData != nullptr) {
      cbs_.OnData(audio_data, bits_per_sample, sample_rate, number_of_channels,
                  number_of_frames, user_data_);
    }
  }

 private:
  webrtc_AudioTrackSinkInterface_cbs cbs_{};
  void* user_data_;
};

extern "C" {
WEBRTC_EXPORT struct webrtc_AudioTrackSinkInterface*
webrtc_AudioTrackSinkInterface_new(
    const struct webrtc_AudioTrackSinkInterface_cbs* cbs,
    void* user_data) {
  auto sink = new AudioTrackSinkInterfaceImpl(cbs, user_data);
  return reinterpret_cast<struct webrtc_AudioTrackSinkInterface*>(sink);
}

WEBRTC_EXPORT void webrtc_AudioTrackSinkInterface_delete(
    struct webrtc_AudioTrackSinkInterface* self) {
  auto sink = reinterpret_cast<AudioTrackSinkInterfaceImpl*>(self);
  delete sink;
}
}
