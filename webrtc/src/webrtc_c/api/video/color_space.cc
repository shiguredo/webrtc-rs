#include "color_space.h"

#include <memory>
#include <string>

// WebRTC
#include <api/video/color_space.h>

#include "../../common.h"
#include "../../common.impl.h"
#include "../../std.h"

extern "C" {

WEBRTC_DEFINE_UNIQUE(webrtc_ColorSpace, webrtc::ColorSpace);

WEBRTC_EXPORT struct webrtc_ColorSpace_unique* webrtc_ColorSpace_new() {
  auto color_space = std::make_unique<webrtc::ColorSpace>();
  return reinterpret_cast<struct webrtc_ColorSpace_unique*>(
      color_space.release());
}

WEBRTC_EXPORT struct std_string_unique* webrtc_ColorSpace_AsString(
    const struct webrtc_ColorSpace* self) {
  auto color_space = reinterpret_cast<const webrtc::ColorSpace*>(self);
  auto out = std::make_unique<std::string>(color_space->AsString());
  return reinterpret_cast<struct std_string_unique*>(out.release());
}

}  // extern "C"
