#include "sdp_video_format.h"

#include <cstddef>
#include <map>
#include <memory>
#include <string>

// WebRTC
#include <api/video_codecs/sdp_video_format.h>

#include "../../common.impl.h"
#include "../../std.h"

extern "C" {
WEBRTC_DEFINE_UNIQUE(webrtc_SdpVideoFormat, webrtc::SdpVideoFormat);

struct webrtc_SdpVideoFormat_unique* webrtc_SdpVideoFormat_new(
    const char* name,
    size_t name_len,
    struct std_map_string_string* parameters) {
  std::string n = name != nullptr ? std::string(name, name_len) : std::string();
  if (parameters != nullptr) {
    auto params =
        reinterpret_cast<std::map<std::string, std::string>*>(parameters);
    auto fmt = std::make_unique<webrtc::SdpVideoFormat>(n, *params);
    return reinterpret_cast<struct webrtc_SdpVideoFormat_unique*>(
        fmt.release());
  }
  auto fmt = std::make_unique<webrtc::SdpVideoFormat>(n);
  return reinterpret_cast<struct webrtc_SdpVideoFormat_unique*>(fmt.release());
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
