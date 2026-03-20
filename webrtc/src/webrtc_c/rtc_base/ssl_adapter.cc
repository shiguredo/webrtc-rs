#include "ssl_adapter.h"

#include <stdarg.h>
#include <stddef.h>

// WebRTC
#include <rtc_base/ssl_adapter.h>

#include "../common.h"

// -------------------------
// webrtc::InitializeSSL()
// -------------------------

extern "C" {
WEBRTC_EXPORT void webrtc_InitializeSSL() {
  webrtc::InitializeSSL();
}
}
