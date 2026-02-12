#include "rtc_event_log.h"

#include <stdarg.h>
#include <stddef.h>
#include <memory>

// WebRTC
#include <api/rtc_event_log/rtc_event_log_factory.h>

#include "../common.impl.h"

// -------------------------
// webrtc::RtcEventLogFactory
// -------------------------

extern "C" {
WEBRTC_DEFINE_UNIQUE(webrtc_RtcEventLogFactory, webrtc::RtcEventLogFactory);
webrtc_RtcEventLogFactory_unique* webrtc_RtcEventLogFactory_Create() {
  auto factory = std::make_unique<webrtc::RtcEventLogFactory>();
  return reinterpret_cast<struct webrtc_RtcEventLogFactory_unique*>(
      factory.release());
}
}
