#pragma once

#include "rtc_stats_report.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::RTCStatsCollectorCallback
// -------------------------

// 全コールバックは必須（null 非許容）。
// 呼び出し側は全関数ポインタを非 null で設定しなければならない。
struct webrtc_RTCStatsCollectorCallback_cbs {
  void (*OnStatsDelivered)(
      const struct webrtc_RTCStatsReport_refcounted* report,
      void* user_data);
};

#if defined(__cplusplus)
}
#endif
