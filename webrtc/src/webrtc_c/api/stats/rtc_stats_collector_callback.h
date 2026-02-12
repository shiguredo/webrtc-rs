#pragma once

#include "rtc_stats_report.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::RTCStatsCollectorCallback
// -------------------------

struct webrtc_RTCStatsCollectorCallback_cbs {
  void (*OnStatsDelivered)(
      const struct webrtc_RTCStatsReport_refcounted* report,
      void* user_data);
};

#if defined(__cplusplus)
}
#endif
