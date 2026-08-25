#include "audio_options.h"

// WebRTC
#include <api/audio_options.h>

// WebRTC C
#include "../../std.impl.h"

WEBRTC_EXPORT struct webrtc_AudioOptions* webrtc_AudioOptions_new() {
  return reinterpret_cast<struct webrtc_AudioOptions*>(
      new webrtc::AudioOptions());
}

WEBRTC_EXPORT void webrtc_AudioOptions_delete(
    struct webrtc_AudioOptions* self) {
  delete reinterpret_cast<webrtc::AudioOptions*>(self);
}

WEBRTC_EXPORT void webrtc_AudioOptions_get_echo_cancellation(
    struct webrtc_AudioOptions* self,
    int* out_has,
    int* out_value) {
  auto options = reinterpret_cast<webrtc::AudioOptions*>(self);
  webrtc_c::OptionalGetAs(
      options->echo_cancellation, out_has, out_value,
      [&]() { return options->echo_cancellation.value() ? 1 : 0; });
}

WEBRTC_EXPORT void webrtc_AudioOptions_set_echo_cancellation(
    struct webrtc_AudioOptions* self,
    int has,
    const int* value) {
  auto options = reinterpret_cast<webrtc::AudioOptions*>(self);
  webrtc_c::OptionalSetAs(options->echo_cancellation, has, value,
                          [&]() { return *value != 0; });
}

WEBRTC_EXPORT void webrtc_AudioOptions_get_auto_gain_control(
    struct webrtc_AudioOptions* self,
    int* out_has,
    int* out_value) {
  auto options = reinterpret_cast<webrtc::AudioOptions*>(self);
  webrtc_c::OptionalGetAs(
      options->auto_gain_control, out_has, out_value,
      [&]() { return options->auto_gain_control.value() ? 1 : 0; });
}

WEBRTC_EXPORT void webrtc_AudioOptions_set_auto_gain_control(
    struct webrtc_AudioOptions* self,
    int has,
    const int* value) {
  auto options = reinterpret_cast<webrtc::AudioOptions*>(self);
  webrtc_c::OptionalSetAs(options->auto_gain_control, has, value,
                          [&]() { return *value != 0; });
}

WEBRTC_EXPORT void webrtc_AudioOptions_get_noise_suppression(
    struct webrtc_AudioOptions* self,
    int* out_has,
    int* out_value) {
  auto options = reinterpret_cast<webrtc::AudioOptions*>(self);
  webrtc_c::OptionalGetAs(
      options->noise_suppression, out_has, out_value,
      [&]() { return options->noise_suppression.value() ? 1 : 0; });
}

WEBRTC_EXPORT void webrtc_AudioOptions_set_noise_suppression(
    struct webrtc_AudioOptions* self,
    int has,
    const int* value) {
  auto options = reinterpret_cast<webrtc::AudioOptions*>(self);
  webrtc_c::OptionalSetAs(options->noise_suppression, has, value,
                          [&]() { return *value != 0; });
}

WEBRTC_EXPORT void webrtc_AudioOptions_get_highpass_filter(
    struct webrtc_AudioOptions* self,
    int* out_has,
    int* out_value) {
  auto options = reinterpret_cast<webrtc::AudioOptions*>(self);
  webrtc_c::OptionalGetAs(options->highpass_filter, out_has, out_value, [&]() {
    return options->highpass_filter.value() ? 1 : 0;
  });
}

WEBRTC_EXPORT void webrtc_AudioOptions_set_highpass_filter(
    struct webrtc_AudioOptions* self,
    int has,
    const int* value) {
  auto options = reinterpret_cast<webrtc::AudioOptions*>(self);
  webrtc_c::OptionalSetAs(options->highpass_filter, has, value,
                          [&]() { return *value != 0; });
}

WEBRTC_EXPORT void webrtc_AudioOptions_get_stereo_swapping(
    struct webrtc_AudioOptions* self,
    int* out_has,
    int* out_value) {
  auto options = reinterpret_cast<webrtc::AudioOptions*>(self);
  webrtc_c::OptionalGetAs(options->stereo_swapping, out_has, out_value, [&]() {
    return options->stereo_swapping.value() ? 1 : 0;
  });
}

WEBRTC_EXPORT void webrtc_AudioOptions_set_stereo_swapping(
    struct webrtc_AudioOptions* self,
    int has,
    const int* value) {
  auto options = reinterpret_cast<webrtc::AudioOptions*>(self);
  webrtc_c::OptionalSetAs(options->stereo_swapping, has, value,
                          [&]() { return *value != 0; });
}

WEBRTC_EXPORT void webrtc_AudioOptions_get_audio_jitter_buffer_max_packets(
    struct webrtc_AudioOptions* self,
    int* out_has,
    int* out_value) {
  auto options = reinterpret_cast<webrtc::AudioOptions*>(self);
  webrtc_c::OptionalGetAs(
      options->audio_jitter_buffer_max_packets, out_has, out_value,
      [&]() { return options->audio_jitter_buffer_max_packets.value(); });
}

WEBRTC_EXPORT void webrtc_AudioOptions_set_audio_jitter_buffer_max_packets(
    struct webrtc_AudioOptions* self,
    int has,
    const int* value) {
  auto options = reinterpret_cast<webrtc::AudioOptions*>(self);
  webrtc_c::OptionalSetAs(options->audio_jitter_buffer_max_packets, has, value,
                          [&]() { return *value; });
}

WEBRTC_EXPORT void webrtc_AudioOptions_get_audio_jitter_buffer_fast_accelerate(
    struct webrtc_AudioOptions* self,
    int* out_has,
    int* out_value) {
  auto options = reinterpret_cast<webrtc::AudioOptions*>(self);
  webrtc_c::OptionalGetAs(
      options->audio_jitter_buffer_fast_accelerate, out_has, out_value, [&]() {
        return options->audio_jitter_buffer_fast_accelerate.value() ? 1 : 0;
      });
}

WEBRTC_EXPORT void webrtc_AudioOptions_set_audio_jitter_buffer_fast_accelerate(
    struct webrtc_AudioOptions* self,
    int has,
    const int* value) {
  auto options = reinterpret_cast<webrtc::AudioOptions*>(self);
  webrtc_c::OptionalSetAs(options->audio_jitter_buffer_fast_accelerate, has,
                          value, [&]() { return *value != 0; });
}

WEBRTC_EXPORT void webrtc_AudioOptions_get_audio_jitter_buffer_min_delay_ms(
    struct webrtc_AudioOptions* self,
    int* out_has,
    int* out_value) {
  auto options = reinterpret_cast<webrtc::AudioOptions*>(self);
  webrtc_c::OptionalGetAs(
      options->audio_jitter_buffer_min_delay_ms, out_has, out_value,
      [&]() { return options->audio_jitter_buffer_min_delay_ms.value(); });
}

WEBRTC_EXPORT void webrtc_AudioOptions_set_audio_jitter_buffer_min_delay_ms(
    struct webrtc_AudioOptions* self,
    int has,
    const int* value) {
  auto options = reinterpret_cast<webrtc::AudioOptions*>(self);
  webrtc_c::OptionalSetAs(options->audio_jitter_buffer_min_delay_ms, has, value,
                          [&]() { return *value; });
}
