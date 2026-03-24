#include "crypto_random.h"

#include <memory>
#include <string>

// WebRTC
#include <rtc_base/crypto_random.h>

#include "../common.h"
#include "../std.h"

// -------------------------
// webrtc::CreateRandomString
// -------------------------

extern "C" {
WEBRTC_EXPORT struct std_string_unique* webrtc_CreateRandomString(int length) {
  auto str = std::make_unique<std::string>(webrtc::CreateRandomString(length));
  return reinterpret_cast<struct std_string_unique*>(str.release());
}
}
