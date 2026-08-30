#include "audio_format.h"

#include <cassert>
#include <cstddef>
#include <map>
#include <memory>
#include <string>
#include <utility>

// WebRTC
#include <api/audio_codecs/audio_format.h>

#include "../../common.h"
#include "../../common.impl.h"
#include "../../std.h"

extern "C" {
// -------------------------
// webrtc::SdpAudioFormat
// -------------------------

WEBRTC_DEFINE_UNIQUE(webrtc_SdpAudioFormat, webrtc::SdpAudioFormat);
WEBRTC_DEFINE_VECTOR_NO_DEFAULT_CTOR(webrtc_SdpAudioFormat,
                                     webrtc::SdpAudioFormat);

WEBRTC_EXPORT struct webrtc_SdpAudioFormat_unique* webrtc_SdpAudioFormat_new(
    const char* name,
    size_t name_len,
    int clockrate_hz,
    size_t num_channels) {
  assert(name != nullptr);
  std::string n(name, name_len);
  auto fmt =
      std::make_unique<webrtc::SdpAudioFormat>(n, clockrate_hz, num_channels);
  return reinterpret_cast<struct webrtc_SdpAudioFormat_unique*>(fmt.release());
}

WEBRTC_EXPORT struct webrtc_SdpAudioFormat_unique*
webrtc_SdpAudioFormat_new_with_parameters(
    const char* name,
    size_t name_len,
    int clockrate_hz,
    size_t num_channels,
    struct std_map_string_string* parameters) {
  assert(name != nullptr);
  std::string n(name, name_len);
  webrtc::CodecParameterMap params;
  if (parameters != nullptr) {
    auto parameter_map =
        reinterpret_cast<std::map<std::string, std::string>*>(parameters);
    params = *parameter_map;
  }
  auto fmt = std::make_unique<webrtc::SdpAudioFormat>(
      n, clockrate_hz, num_channels, std::move(params));
  return reinterpret_cast<struct webrtc_SdpAudioFormat_unique*>(fmt.release());
}

WEBRTC_EXPORT struct webrtc_SdpAudioFormat_unique* webrtc_SdpAudioFormat_copy(
    const struct webrtc_SdpAudioFormat* self) {
  assert(self != nullptr);
  auto fmt = reinterpret_cast<const webrtc::SdpAudioFormat*>(self);
  auto copied = std::make_unique<webrtc::SdpAudioFormat>(*fmt);
  return reinterpret_cast<struct webrtc_SdpAudioFormat_unique*>(
      copied.release());
}

WEBRTC_EXPORT struct std_string* webrtc_SdpAudioFormat_get_name(
    struct webrtc_SdpAudioFormat* self) {
  auto fmt = reinterpret_cast<webrtc::SdpAudioFormat*>(self);
  return reinterpret_cast<struct std_string*>(&fmt->name);
}

WEBRTC_EXPORT void webrtc_SdpAudioFormat_set_name(
    struct webrtc_SdpAudioFormat* self,
    const struct std_string* name) {
  auto fmt = reinterpret_cast<webrtc::SdpAudioFormat*>(self);
  auto cpp_name = reinterpret_cast<const std::string*>(name);
  fmt->name = *cpp_name;
}

WEBRTC_EXPORT int webrtc_SdpAudioFormat_get_clockrate_hz(
    const struct webrtc_SdpAudioFormat* self) {
  auto fmt = reinterpret_cast<const webrtc::SdpAudioFormat*>(self);
  return fmt->clockrate_hz;
}

WEBRTC_EXPORT void webrtc_SdpAudioFormat_set_clockrate_hz(
    struct webrtc_SdpAudioFormat* self,
    int value) {
  auto fmt = reinterpret_cast<webrtc::SdpAudioFormat*>(self);
  fmt->clockrate_hz = value;
}

WEBRTC_EXPORT size_t webrtc_SdpAudioFormat_get_num_channels(
    const struct webrtc_SdpAudioFormat* self) {
  auto fmt = reinterpret_cast<const webrtc::SdpAudioFormat*>(self);
  return fmt->num_channels;
}

WEBRTC_EXPORT void webrtc_SdpAudioFormat_set_num_channels(
    struct webrtc_SdpAudioFormat* self,
    size_t value) {
  auto fmt = reinterpret_cast<webrtc::SdpAudioFormat*>(self);
  fmt->num_channels = value;
}

WEBRTC_EXPORT struct std_map_string_string*
webrtc_SdpAudioFormat_get_parameters(struct webrtc_SdpAudioFormat* self) {
  auto fmt = reinterpret_cast<webrtc::SdpAudioFormat*>(self);
  return reinterpret_cast<struct std_map_string_string*>(&fmt->parameters);
}

WEBRTC_EXPORT void webrtc_SdpAudioFormat_set_parameters(
    struct webrtc_SdpAudioFormat* self,
    struct std_map_string_string* parameters) {
  auto fmt = reinterpret_cast<webrtc::SdpAudioFormat*>(self);
  auto cpp_params =
      reinterpret_cast<std::map<std::string, std::string>*>(parameters);
  fmt->parameters = *cpp_params;
}

WEBRTC_EXPORT int webrtc_SdpAudioFormat_is_equal(
    const struct webrtc_SdpAudioFormat* lhs,
    const struct webrtc_SdpAudioFormat* rhs) {
  auto a = reinterpret_cast<const webrtc::SdpAudioFormat*>(lhs);
  auto b = reinterpret_cast<const webrtc::SdpAudioFormat*>(rhs);
  assert(a != nullptr);
  assert(b != nullptr);
  return *a == *b;
}

WEBRTC_EXPORT int webrtc_SdpAudioFormat_Matches(
    const struct webrtc_SdpAudioFormat* self,
    const struct webrtc_SdpAudioFormat* other) {
  auto a = reinterpret_cast<const webrtc::SdpAudioFormat*>(self);
  auto b = reinterpret_cast<const webrtc::SdpAudioFormat*>(other);
  assert(a != nullptr);
  assert(b != nullptr);
  return a->Matches(*b);
}

// -------------------------
// webrtc::AudioCodecInfo
// -------------------------

WEBRTC_EXPORT struct webrtc_AudioCodecInfo* webrtc_AudioCodecInfo_new(
    int sample_rate_hz,
    size_t num_channels,
    int default_bitrate_bps,
    int min_bitrate_bps,
    int max_bitrate_bps) {
  auto info = new webrtc::AudioCodecInfo(sample_rate_hz, num_channels,
                                         default_bitrate_bps, min_bitrate_bps,
                                         max_bitrate_bps);
  return reinterpret_cast<struct webrtc_AudioCodecInfo*>(info);
}

WEBRTC_EXPORT void webrtc_AudioCodecInfo_delete(
    struct webrtc_AudioCodecInfo* self) {
  auto info = reinterpret_cast<webrtc::AudioCodecInfo*>(self);
  delete info;
}

WEBRTC_EXPORT struct webrtc_AudioCodecInfo* webrtc_AudioCodecInfo_copy(
    const struct webrtc_AudioCodecInfo* self) {
  auto info = reinterpret_cast<const webrtc::AudioCodecInfo*>(self);
  auto copied = new webrtc::AudioCodecInfo(*info);
  return reinterpret_cast<struct webrtc_AudioCodecInfo*>(copied);
}

WEBRTC_EXPORT int webrtc_AudioCodecInfo_get_sample_rate_hz(
    const struct webrtc_AudioCodecInfo* self) {
  auto info = reinterpret_cast<const webrtc::AudioCodecInfo*>(self);
  return info->sample_rate_hz;
}

WEBRTC_EXPORT void webrtc_AudioCodecInfo_set_sample_rate_hz(
    struct webrtc_AudioCodecInfo* self,
    int value) {
  auto info = reinterpret_cast<webrtc::AudioCodecInfo*>(self);
  info->sample_rate_hz = value;
}

WEBRTC_EXPORT size_t webrtc_AudioCodecInfo_get_num_channels(
    const struct webrtc_AudioCodecInfo* self) {
  auto info = reinterpret_cast<const webrtc::AudioCodecInfo*>(self);
  return info->num_channels;
}

WEBRTC_EXPORT void webrtc_AudioCodecInfo_set_num_channels(
    struct webrtc_AudioCodecInfo* self,
    size_t value) {
  auto info = reinterpret_cast<webrtc::AudioCodecInfo*>(self);
  info->num_channels = value;
}

WEBRTC_EXPORT int webrtc_AudioCodecInfo_get_default_bitrate_bps(
    const struct webrtc_AudioCodecInfo* self) {
  auto info = reinterpret_cast<const webrtc::AudioCodecInfo*>(self);
  return info->default_bitrate_bps;
}

WEBRTC_EXPORT void webrtc_AudioCodecInfo_set_default_bitrate_bps(
    struct webrtc_AudioCodecInfo* self,
    int value) {
  auto info = reinterpret_cast<webrtc::AudioCodecInfo*>(self);
  info->default_bitrate_bps = value;
}

WEBRTC_EXPORT int webrtc_AudioCodecInfo_get_min_bitrate_bps(
    const struct webrtc_AudioCodecInfo* self) {
  auto info = reinterpret_cast<const webrtc::AudioCodecInfo*>(self);
  return info->min_bitrate_bps;
}

WEBRTC_EXPORT void webrtc_AudioCodecInfo_set_min_bitrate_bps(
    struct webrtc_AudioCodecInfo* self,
    int value) {
  auto info = reinterpret_cast<webrtc::AudioCodecInfo*>(self);
  info->min_bitrate_bps = value;
}

WEBRTC_EXPORT int webrtc_AudioCodecInfo_get_max_bitrate_bps(
    const struct webrtc_AudioCodecInfo* self) {
  auto info = reinterpret_cast<const webrtc::AudioCodecInfo*>(self);
  return info->max_bitrate_bps;
}

WEBRTC_EXPORT void webrtc_AudioCodecInfo_set_max_bitrate_bps(
    struct webrtc_AudioCodecInfo* self,
    int value) {
  auto info = reinterpret_cast<webrtc::AudioCodecInfo*>(self);
  info->max_bitrate_bps = value;
}

WEBRTC_EXPORT int webrtc_AudioCodecInfo_get_allow_comfort_noise(
    const struct webrtc_AudioCodecInfo* self) {
  auto info = reinterpret_cast<const webrtc::AudioCodecInfo*>(self);
  return info->allow_comfort_noise ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_AudioCodecInfo_set_allow_comfort_noise(
    struct webrtc_AudioCodecInfo* self,
    int value) {
  auto info = reinterpret_cast<webrtc::AudioCodecInfo*>(self);
  info->allow_comfort_noise = value != 0;
}

WEBRTC_EXPORT int webrtc_AudioCodecInfo_get_supports_network_adaption(
    const struct webrtc_AudioCodecInfo* self) {
  auto info = reinterpret_cast<const webrtc::AudioCodecInfo*>(self);
  return info->supports_network_adaption ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_AudioCodecInfo_set_supports_network_adaption(
    struct webrtc_AudioCodecInfo* self,
    int value) {
  auto info = reinterpret_cast<webrtc::AudioCodecInfo*>(self);
  info->supports_network_adaption = value != 0;
}

// -------------------------
// webrtc::AudioCodecSpec
// -------------------------

WEBRTC_DEFINE_VECTOR_NO_DEFAULT_CTOR(webrtc_AudioCodecSpec,
                                     webrtc::AudioCodecSpec);
WEBRTC_EXPORT struct webrtc_AudioCodecSpec* webrtc_AudioCodecSpec_new(
    struct webrtc_SdpAudioFormat* format,
    struct webrtc_AudioCodecInfo* info) {
  auto cpp_format = reinterpret_cast<webrtc::SdpAudioFormat*>(format);
  auto cpp_info = reinterpret_cast<webrtc::AudioCodecInfo*>(info);
  auto spec = new webrtc::AudioCodecSpec(
      webrtc::AudioCodecSpec{*cpp_format, *cpp_info});
  return reinterpret_cast<struct webrtc_AudioCodecSpec*>(spec);
}

WEBRTC_EXPORT void webrtc_AudioCodecSpec_delete(
    struct webrtc_AudioCodecSpec* self) {
  auto spec = reinterpret_cast<webrtc::AudioCodecSpec*>(self);
  delete spec;
}

WEBRTC_EXPORT struct webrtc_AudioCodecSpec* webrtc_AudioCodecSpec_copy(
    const struct webrtc_AudioCodecSpec* self) {
  assert(self != nullptr);
  auto spec = reinterpret_cast<const webrtc::AudioCodecSpec*>(self);
  auto copied = new webrtc::AudioCodecSpec(*spec);
  return reinterpret_cast<struct webrtc_AudioCodecSpec*>(copied);
}

WEBRTC_EXPORT void webrtc_AudioCodecSpec_set_format(
    struct webrtc_AudioCodecSpec* self,
    struct webrtc_SdpAudioFormat* format) {
  auto spec = reinterpret_cast<webrtc::AudioCodecSpec*>(self);
  auto cpp_format = reinterpret_cast<webrtc::SdpAudioFormat*>(format);
  spec->format = *cpp_format;
}

WEBRTC_EXPORT struct webrtc_SdpAudioFormat* webrtc_AudioCodecSpec_get_format(
    struct webrtc_AudioCodecSpec* self) {
  auto spec = reinterpret_cast<webrtc::AudioCodecSpec*>(self);
  return reinterpret_cast<struct webrtc_SdpAudioFormat*>(&spec->format);
}

WEBRTC_EXPORT void webrtc_AudioCodecSpec_set_info(
    struct webrtc_AudioCodecSpec* self,
    struct webrtc_AudioCodecInfo* info) {
  auto spec = reinterpret_cast<webrtc::AudioCodecSpec*>(self);
  auto cpp_info = reinterpret_cast<webrtc::AudioCodecInfo*>(info);
  spec->info = *cpp_info;
}

WEBRTC_EXPORT struct webrtc_AudioCodecInfo* webrtc_AudioCodecSpec_get_info(
    struct webrtc_AudioCodecSpec* self) {
  auto spec = reinterpret_cast<webrtc::AudioCodecSpec*>(self);
  return reinterpret_cast<struct webrtc_AudioCodecInfo*>(&spec->info);
}
}
