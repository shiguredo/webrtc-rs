#include "rtc_stats_report.h"

#include <stddef.h>
#include <memory>
#include <string>
#include <utility>

// WebRTC
#include <api/stats/rtc_stats_report.h>

#include "../../common.impl.h"
#include "../../std.h"

// -------------------------
// webrtc::RTCStatsReport
// -------------------------

extern "C" {
WEBRTC_DEFINE_REFCOUNTED(webrtc_RTCStatsReport, webrtc::RTCStatsReport);

struct std_string_unique* webrtc_RTCStatsReport_ToJson(
    struct webrtc_RTCStatsReport* report) {
  if (report == nullptr) {
    return nullptr;
  }
  auto rtc_report = reinterpret_cast<webrtc::RTCStatsReport*>(report);
  std::string json = rtc_report->ToJson();
  auto str = std::make_unique<std::string>(std::move(json));
  return reinterpret_cast<struct std_string_unique*>(str.release());
}
}
