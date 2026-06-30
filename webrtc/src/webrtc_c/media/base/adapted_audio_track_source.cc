#include "adapted_audio_track_source.h"

#include <stddef.h>
#include <stdint.h>

#include <algorithm>
#include <vector>

// WebRTC
#include <api/make_ref_counted.h>
#include <api/media_stream_interface.h>
#include <api/scoped_refptr.h>

#include "../../common.h"
#include "../../common.impl.h"

// -------------------------
// webrtc::AdaptedAudioTrackSource
// -------------------------

class AdaptedAudioTrackSourceWrapper : public webrtc::AudioSourceInterface {
 public:
  AdaptedAudioTrackSourceWrapper(int sample_rate, size_t channels)
      : sample_rate_(sample_rate), channels_(channels) {}

  webrtc::MediaSourceInterface::SourceState state() const override {
    return webrtc::MediaSourceInterface::kLive;
  }

  bool remote() const override { return false; }

  void RegisterObserver(webrtc::ObserverInterface* /*observer*/) override {
    // SourceState は常に kLive で変化しないため observer への通知は不要。
  }

  void UnregisterObserver(webrtc::ObserverInterface* /*observer*/) override {
    // 状態不変のため何もしない。
  }

  void AddSink(webrtc::AudioTrackSinkInterface* sink) override {
    sinks_.push_back(sink);
  }

  void RemoveSink(webrtc::AudioTrackSinkInterface* sink) override {
    sinks_.erase(
        std::remove(sinks_.begin(), sinks_.end(), sink),
        sinks_.end());
  }

  void OnData(const int16_t* audio_data, size_t samples_per_channel) {
    for (auto* sink : sinks_) {
      // WebRTC 内部の AudioFrame に基づいて bits_per_sample は 16 固定。
      // number_of_frames は各チャンネルあたりのサンプル数 = samples_per_channel。
      sink->OnData(audio_data, 16, sample_rate_, channels_, samples_per_channel);
    }
  }

 private:
  int sample_rate_;
  size_t channels_;
  std::vector<webrtc::AudioTrackSinkInterface*> sinks_;
};

extern "C" {
WEBRTC_DEFINE_REFCOUNTED(webrtc_AdaptedAudioTrackSource,
                         AdaptedAudioTrackSourceWrapper);

WEBRTC_EXPORT struct webrtc_AdaptedAudioTrackSource_refcounted*
webrtc_AdaptedAudioTrackSource_Create(int sample_rate, size_t channels) {
  auto src =
      webrtc::make_ref_counted<AdaptedAudioTrackSourceWrapper>(sample_rate,
                                                                channels);
  return reinterpret_cast<struct webrtc_AdaptedAudioTrackSource_refcounted*>(
      src.release());
}

WEBRTC_EXPORT void webrtc_AdaptedAudioTrackSource_OnData(
    struct webrtc_AdaptedAudioTrackSource* self,
    const int16_t* audio_data,
    size_t samples_per_channel) {
  auto src = reinterpret_cast<AdaptedAudioTrackSourceWrapper*>(self);
  src->OnData(audio_data, samples_per_channel);
}
WEBRTC_DEFINE_CAST_REFCOUNTED(webrtc_AdaptedAudioTrackSource,
                              webrtc_AudioSourceInterface,
                              AdaptedAudioTrackSourceWrapper,
                              webrtc::AudioSourceInterface);
}
