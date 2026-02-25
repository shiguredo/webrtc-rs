#include "audio_device_defines.h"

#include <stddef.h>
#include <stdint.h>

#include <api/audio/audio_device_defines.h>
#include <optional>

namespace {

class AudioTransportImpl : public webrtc::AudioTransport {
 public:
  AudioTransportImpl(const struct webrtc_AudioTransport_cbs* cbs,
                     void* user_data)
      : user_data_(user_data) {
    if (cbs != nullptr) {
      cbs_ = *cbs;
    }
  }

  ~AudioTransportImpl() override {
    if (cbs_.OnDestroy != nullptr) {
      cbs_.OnDestroy(user_data_);
    }
  }

  int32_t RecordedDataIsAvailable(const void* audio_samples,
                                  size_t n_samples,
                                  size_t n_bytes_per_sample,
                                  size_t n_channels,
                                  uint32_t samples_per_sec,
                                  uint32_t total_delay_ms,
                                  int32_t clock_drift,
                                  uint32_t current_mic_level,
                                  bool key_pressed,
                                  uint32_t& new_mic_level) override {
    return RecordedDataIsAvailable(audio_samples, n_samples, n_bytes_per_sample,
                                   n_channels, samples_per_sec, total_delay_ms,
                                   clock_drift, current_mic_level, key_pressed,
                                   new_mic_level, std::nullopt);
  }

  int32_t RecordedDataIsAvailable(
      const void* audio_samples,
      size_t n_samples,
      size_t n_bytes_per_sample,
      size_t n_channels,
      uint32_t samples_per_sec,
      uint32_t total_delay_ms,
      int32_t clock_drift,
      uint32_t current_mic_level,
      bool key_pressed,
      uint32_t& new_mic_level,
      std::optional<int64_t> estimated_capture_time_ns) override {
    if (cbs_.RecordedDataIsAvailable == nullptr) {
      return 0;
    }
    return cbs_.RecordedDataIsAvailable(
        audio_samples, n_samples, n_bytes_per_sample, n_channels,
        samples_per_sec, total_delay_ms, clock_drift, current_mic_level,
        key_pressed ? 1 : 0, &new_mic_level,
        estimated_capture_time_ns.has_value() ? &*estimated_capture_time_ns
                                              : nullptr,
        user_data_);
  }

  int32_t NeedMorePlayData(size_t n_samples,
                           size_t n_bytes_per_sample,
                           size_t n_channels,
                           uint32_t samples_per_sec,
                           void* audio_samples,
                           size_t& n_samples_out,
                           int64_t* elapsed_time_ms,
                           int64_t* ntp_time_ms) override {
    if (cbs_.NeedMorePlayData == nullptr) {
      return 0;
    }
    return cbs_.NeedMorePlayData(n_samples, n_bytes_per_sample, n_channels,
                                 samples_per_sec, audio_samples, &n_samples_out,
                                 elapsed_time_ms, ntp_time_ms, user_data_);
  }

  void PullRenderData(int bits_per_sample,
                      int sample_rate,
                      size_t number_of_channels,
                      size_t number_of_frames,
                      void* audio_data,
                      int64_t* elapsed_time_ms,
                      int64_t* ntp_time_ms) override {
    if (cbs_.PullRenderData == nullptr) {
      return;
    }
    cbs_.PullRenderData(bits_per_sample, sample_rate, number_of_channels,
                        number_of_frames, audio_data, elapsed_time_ms,
                        ntp_time_ms, user_data_);
  }

 private:
  webrtc_AudioTransport_cbs cbs_{};
  void* user_data_ = nullptr;
};

}  // namespace

extern "C" {

struct webrtc_AudioTransport* webrtc_AudioTransport_new(
    const struct webrtc_AudioTransport_cbs* cbs,
    void* user_data) {
  auto transport = new AudioTransportImpl(cbs, user_data);
  return reinterpret_cast<struct webrtc_AudioTransport*>(transport);
}

void webrtc_AudioTransport_delete(struct webrtc_AudioTransport* self) {
  auto transport = reinterpret_cast<AudioTransportImpl*>(self);
  delete transport;
}

int32_t webrtc_AudioTransport_RecordedDataIsAvailable(
    struct webrtc_AudioTransport* self,
    const void* audio_samples,
    size_t n_samples,
    size_t n_bytes_per_sample,
    size_t n_channels,
    uint32_t samples_per_sec,
    uint32_t total_delay_ms,
    int32_t clock_drift,
    uint32_t current_mic_level,
    int key_pressed,
    uint32_t* new_mic_level,
    const int64_t* estimated_capture_time_ns) {
  auto transport = reinterpret_cast<webrtc::AudioTransport*>(self);
  uint32_t new_mic_level_value = 0;
  std::optional<int64_t> estimated_capture_time_ns_value;
  if (estimated_capture_time_ns != nullptr) {
    estimated_capture_time_ns_value = *estimated_capture_time_ns;
  }
  int32_t ret = transport->RecordedDataIsAvailable(
      audio_samples, n_samples, n_bytes_per_sample, n_channels, samples_per_sec,
      total_delay_ms, clock_drift, current_mic_level, key_pressed != 0,
      new_mic_level_value, estimated_capture_time_ns_value);
  if (new_mic_level != nullptr) {
    *new_mic_level = new_mic_level_value;
  }
  return ret;
}

int32_t webrtc_AudioTransport_NeedMorePlayData(
    struct webrtc_AudioTransport* self,
    size_t n_samples,
    size_t n_bytes_per_sample,
    size_t n_channels,
    uint32_t samples_per_sec,
    void* audio_samples,
    size_t* n_samples_out,
    int64_t* elapsed_time_ms,
    int64_t* ntp_time_ms) {
  auto transport = reinterpret_cast<webrtc::AudioTransport*>(self);
  size_t n_samples_out_value = 0;
  int32_t ret = transport->NeedMorePlayData(
      n_samples, n_bytes_per_sample, n_channels, samples_per_sec, audio_samples,
      n_samples_out_value, elapsed_time_ms, ntp_time_ms);
  if (n_samples_out != nullptr) {
    *n_samples_out = n_samples_out_value;
  }
  return ret;
}

void webrtc_AudioTransport_PullRenderData(struct webrtc_AudioTransport* self,
                                          int bits_per_sample,
                                          int sample_rate,
                                          size_t number_of_channels,
                                          size_t number_of_frames,
                                          void* audio_data,
                                          int64_t* elapsed_time_ms,
                                          int64_t* ntp_time_ms) {
  auto transport = reinterpret_cast<webrtc::AudioTransport*>(self);
  transport->PullRenderData(bits_per_sample, sample_rate, number_of_channels,
                            number_of_frames, audio_data, elapsed_time_ms,
                            ntp_time_ms);
}
}
