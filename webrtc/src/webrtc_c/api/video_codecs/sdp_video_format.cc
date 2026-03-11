#include "sdp_video_format.h"

#include <algorithm>
#include <cstddef>
#include <map>
#include <memory>
#include <string>

// Abseil
#include <absl/container/inlined_vector.h>

// WebRTC
#include <api/video_codecs/scalability_mode.h>
#include <api/video_codecs/sdp_video_format.h>

#include "../../common.impl.h"
#include "../../std.h"

extern "C" {
const int webrtc_ScalabilityMode_L1T1 =
    static_cast<int>(webrtc::ScalabilityMode::kL1T1);
const int webrtc_ScalabilityMode_L1T2 =
    static_cast<int>(webrtc::ScalabilityMode::kL1T2);
const int webrtc_ScalabilityMode_L1T3 =
    static_cast<int>(webrtc::ScalabilityMode::kL1T3);
const int webrtc_ScalabilityMode_L2T1 =
    static_cast<int>(webrtc::ScalabilityMode::kL2T1);
const int webrtc_ScalabilityMode_L2T1h =
    static_cast<int>(webrtc::ScalabilityMode::kL2T1h);
const int webrtc_ScalabilityMode_L2T1_KEY =
    static_cast<int>(webrtc::ScalabilityMode::kL2T1_KEY);
const int webrtc_ScalabilityMode_L2T2 =
    static_cast<int>(webrtc::ScalabilityMode::kL2T2);
const int webrtc_ScalabilityMode_L2T2h =
    static_cast<int>(webrtc::ScalabilityMode::kL2T2h);
const int webrtc_ScalabilityMode_L2T2_KEY =
    static_cast<int>(webrtc::ScalabilityMode::kL2T2_KEY);
const int webrtc_ScalabilityMode_L2T2_KEY_SHIFT =
    static_cast<int>(webrtc::ScalabilityMode::kL2T2_KEY_SHIFT);
const int webrtc_ScalabilityMode_L2T3 =
    static_cast<int>(webrtc::ScalabilityMode::kL2T3);
const int webrtc_ScalabilityMode_L2T3h =
    static_cast<int>(webrtc::ScalabilityMode::kL2T3h);
const int webrtc_ScalabilityMode_L2T3_KEY =
    static_cast<int>(webrtc::ScalabilityMode::kL2T3_KEY);
const int webrtc_ScalabilityMode_L3T1 =
    static_cast<int>(webrtc::ScalabilityMode::kL3T1);
const int webrtc_ScalabilityMode_L3T1h =
    static_cast<int>(webrtc::ScalabilityMode::kL3T1h);
const int webrtc_ScalabilityMode_L3T1_KEY =
    static_cast<int>(webrtc::ScalabilityMode::kL3T1_KEY);
const int webrtc_ScalabilityMode_L3T2 =
    static_cast<int>(webrtc::ScalabilityMode::kL3T2);
const int webrtc_ScalabilityMode_L3T2h =
    static_cast<int>(webrtc::ScalabilityMode::kL3T2h);
const int webrtc_ScalabilityMode_L3T2_KEY =
    static_cast<int>(webrtc::ScalabilityMode::kL3T2_KEY);
const int webrtc_ScalabilityMode_L3T3 =
    static_cast<int>(webrtc::ScalabilityMode::kL3T3);
const int webrtc_ScalabilityMode_L3T3h =
    static_cast<int>(webrtc::ScalabilityMode::kL3T3h);
const int webrtc_ScalabilityMode_L3T3_KEY =
    static_cast<int>(webrtc::ScalabilityMode::kL3T3_KEY);
const int webrtc_ScalabilityMode_S2T1 =
    static_cast<int>(webrtc::ScalabilityMode::kS2T1);
const int webrtc_ScalabilityMode_S2T1h =
    static_cast<int>(webrtc::ScalabilityMode::kS2T1h);
const int webrtc_ScalabilityMode_S2T2 =
    static_cast<int>(webrtc::ScalabilityMode::kS2T2);
const int webrtc_ScalabilityMode_S2T2h =
    static_cast<int>(webrtc::ScalabilityMode::kS2T2h);
const int webrtc_ScalabilityMode_S2T3 =
    static_cast<int>(webrtc::ScalabilityMode::kS2T3);
const int webrtc_ScalabilityMode_S2T3h =
    static_cast<int>(webrtc::ScalabilityMode::kS2T3h);
const int webrtc_ScalabilityMode_S3T1 =
    static_cast<int>(webrtc::ScalabilityMode::kS3T1);
const int webrtc_ScalabilityMode_S3T1h =
    static_cast<int>(webrtc::ScalabilityMode::kS3T1h);
const int webrtc_ScalabilityMode_S3T2 =
    static_cast<int>(webrtc::ScalabilityMode::kS3T2);
const int webrtc_ScalabilityMode_S3T2h =
    static_cast<int>(webrtc::ScalabilityMode::kS3T2h);
const int webrtc_ScalabilityMode_S3T3 =
    static_cast<int>(webrtc::ScalabilityMode::kS3T3);
const int webrtc_ScalabilityMode_S3T3h =
    static_cast<int>(webrtc::ScalabilityMode::kS3T3h);

WEBRTC_DEFINE_UNIQUE(webrtc_SdpVideoFormat, webrtc::SdpVideoFormat);
WEBRTC_DEFINE_VECTOR_NO_DEFAULT_CTOR(webrtc_SdpVideoFormat,
                                     webrtc::SdpVideoFormat);

struct std_string_unique* webrtc_ScalabilityModeToString(int mode) {
  auto mode_string = std::make_unique<std::string>(
      webrtc::ScalabilityModeToString(
          static_cast<webrtc::ScalabilityMode>(mode)));
  return reinterpret_cast<struct std_string_unique*>(mode_string.release());
}

struct webrtc_SdpVideoFormat_unique* webrtc_SdpVideoFormat_new(
    const char* name,
    size_t name_len) {
  std::string n = name != nullptr ? std::string(name, name_len) : std::string();
  auto fmt = std::make_unique<webrtc::SdpVideoFormat>(n);
  return reinterpret_cast<struct webrtc_SdpVideoFormat_unique*>(fmt.release());
}

struct webrtc_SdpVideoFormat_unique* webrtc_SdpVideoFormat_new_with_parameters(
    const char* name,
    size_t name_len,
    struct std_map_string_string* parameters,
    const int* scalability_modes,
    size_t scalability_modes_len) {
  std::string n = name != nullptr ? std::string(name, name_len) : std::string();
  std::map<std::string, std::string> params;
  if (parameters != nullptr) {
    auto parameter_map =
        reinterpret_cast<std::map<std::string, std::string>*>(parameters);
    params = *parameter_map;
  }

  absl::InlinedVector<webrtc::ScalabilityMode, webrtc::kScalabilityModeCount>
      modes;
  modes.reserve(scalability_modes_len);
  if (scalability_modes != nullptr) {
    for (size_t i = 0; i < scalability_modes_len; ++i) {
      modes.push_back(
          static_cast<webrtc::ScalabilityMode>(scalability_modes[i]));
    }
  }

  auto fmt = std::make_unique<webrtc::SdpVideoFormat>(n, params, modes);
  return reinterpret_cast<struct webrtc_SdpVideoFormat_unique*>(fmt.release());
}

struct webrtc_SdpVideoFormat_unique* webrtc_SdpVideoFormat_copy(
    struct webrtc_SdpVideoFormat* self) {
  auto fmt = reinterpret_cast<webrtc::SdpVideoFormat*>(self);
  if (fmt == nullptr) {
    return nullptr;
  }
  auto copied = std::make_unique<webrtc::SdpVideoFormat>(*fmt);
  return reinterpret_cast<struct webrtc_SdpVideoFormat_unique*>(
      copied.release());
}

struct std_string* webrtc_SdpVideoFormat_get_name(
    struct webrtc_SdpVideoFormat* self) {
  auto fmt = reinterpret_cast<webrtc::SdpVideoFormat*>(self);
  return reinterpret_cast<struct std_string*>(&fmt->name);
}

struct std_map_string_string* webrtc_SdpVideoFormat_get_parameters(
    struct webrtc_SdpVideoFormat* self) {
  auto fmt = reinterpret_cast<webrtc::SdpVideoFormat*>(self);
  return reinterpret_cast<struct std_map_string_string*>(&fmt->parameters);
}

size_t webrtc_SdpVideoFormat_get_scalability_modes_size(
    struct webrtc_SdpVideoFormat* self) {
  auto fmt = reinterpret_cast<webrtc::SdpVideoFormat*>(self);
  if (fmt == nullptr) {
    return 0;
  }
  return fmt->scalability_modes.size();
}

size_t webrtc_SdpVideoFormat_copy_scalability_modes(
    struct webrtc_SdpVideoFormat* self,
    int* out_modes,
    size_t out_modes_len) {
  auto fmt = reinterpret_cast<webrtc::SdpVideoFormat*>(self);
  if (fmt == nullptr || out_modes == nullptr) {
    return 0;
  }
  const size_t copied = std::min(out_modes_len, fmt->scalability_modes.size());
  for (size_t i = 0; i < copied; ++i) {
    out_modes[i] = static_cast<int>(fmt->scalability_modes[i]);
  }
  return copied;
}

int webrtc_SdpVideoFormat_is_equal(struct webrtc_SdpVideoFormat* lhs,
                                   struct webrtc_SdpVideoFormat* rhs) {
  auto a = reinterpret_cast<webrtc::SdpVideoFormat*>(lhs);
  auto b = reinterpret_cast<webrtc::SdpVideoFormat*>(rhs);
  if (a == nullptr || b == nullptr) {
    return 0;
  }
  return *a == *b;
}
}
