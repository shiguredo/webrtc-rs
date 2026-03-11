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

namespace {}  // namespace

extern "C" {
WEBRTC_DEFINE_UNIQUE(webrtc_SdpVideoFormat, webrtc::SdpVideoFormat);
WEBRTC_DEFINE_VECTOR_NO_DEFAULT_CTOR(webrtc_SdpVideoFormat,
                                     webrtc::SdpVideoFormat);

struct std_string_unique* webrtc_ScalabilityModeToString(
    enum webrtc_ScalabilityMode mode) {
  if (static_cast<size_t>(mode) >= webrtc::kScalabilityModeCount) {
    return nullptr;
  }
  auto cpp_mode = static_cast<webrtc::ScalabilityMode>(mode);
  auto mode_string =
      std::make_unique<std::string>(webrtc::ScalabilityModeToString(cpp_mode));
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
    const enum webrtc_ScalabilityMode* scalability_modes,
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
      if (static_cast<size_t>(scalability_modes[i]) <
          webrtc::kScalabilityModeCount) {
        modes.push_back(
            static_cast<webrtc::ScalabilityMode>(scalability_modes[i]));
      }
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
    enum webrtc_ScalabilityMode* out_modes,
    size_t out_modes_len) {
  auto fmt = reinterpret_cast<webrtc::SdpVideoFormat*>(self);
  if (fmt == nullptr || out_modes == nullptr) {
    return 0;
  }
  const size_t copied = std::min(out_modes_len, fmt->scalability_modes.size());
  for (size_t i = 0; i < copied; ++i) {
    out_modes[i] =
        static_cast<enum webrtc_ScalabilityMode>(fmt->scalability_modes[i]);
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
