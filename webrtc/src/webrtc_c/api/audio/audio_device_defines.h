#pragma once

#include <stddef.h>
#include <stdint.h>

#if defined(__cplusplus)
extern "C" {
#endif

struct webrtc_AudioTransport;

struct webrtc_AudioTransport_cbs {
  int32_t (*RecordedDataIsAvailable)(const void* audio_samples,
                                     size_t n_samples,
                                     size_t n_bytes_per_sample,
                                     size_t n_channels,
                                     uint32_t samples_per_sec,
                                     uint32_t total_delay_ms,
                                     int32_t clock_drift,
                                     uint32_t current_mic_level,
                                     int key_pressed,
                                     uint32_t* new_mic_level,
                                     const int64_t* estimated_capture_time_ns,
                                     void* user_data);
  int32_t (*NeedMorePlayData)(size_t n_samples,
                              size_t n_bytes_per_sample,
                              size_t n_channels,
                              uint32_t samples_per_sec,
                              void* audio_samples,
                              size_t* n_samples_out,
                              int64_t* elapsed_time_ms,
                              int64_t* ntp_time_ms,
                              void* user_data);
  void (*PullRenderData)(int bits_per_sample,
                         int sample_rate,
                         size_t number_of_channels,
                         size_t number_of_frames,
                         void* audio_data,
                         int64_t* elapsed_time_ms,
                         int64_t* ntp_time_ms,
                         void* user_data);
};

struct webrtc_AudioTransport* webrtc_AudioTransport_new(
    struct webrtc_AudioTransport_cbs* cbs,
    void* user_data);
void webrtc_AudioTransport_delete(struct webrtc_AudioTransport* self);

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
    const int64_t* estimated_capture_time_ns);

int32_t webrtc_AudioTransport_NeedMorePlayData(
    struct webrtc_AudioTransport* self,
    size_t n_samples,
    size_t n_bytes_per_sample,
    size_t n_channels,
    uint32_t samples_per_sec,
    void* audio_samples,
    size_t* n_samples_out,
    int64_t* elapsed_time_ms,
    int64_t* ntp_time_ms);

void webrtc_AudioTransport_PullRenderData(struct webrtc_AudioTransport* self,
                                          int bits_per_sample,
                                          int sample_rate,
                                          size_t number_of_channels,
                                          size_t number_of_frames,
                                          void* audio_data,
                                          int64_t* elapsed_time_ms,
                                          int64_t* ntp_time_ms);

#if defined(__cplusplus)
}
#endif
