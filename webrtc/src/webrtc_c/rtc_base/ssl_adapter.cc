#include "ssl_adapter.h"

#include <stdarg.h>
#include <stddef.h>

// WebRTC
#include <rtc_base/ssl_adapter.h>

// -------------------------
// webrtc::InitializeSSL()
// -------------------------

extern "C" {
void webrtc_InitializeSSL() {
  webrtc::InitializeSSL();
}
}
