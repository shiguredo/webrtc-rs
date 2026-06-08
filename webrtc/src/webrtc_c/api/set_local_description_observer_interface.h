#pragma once

#include "../common.h"
#include "rtc_error.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::SetLocalDescriptionObserverInterface
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_SetLocalDescriptionObserverInterface);

// 全コールバックは必須（null 非許容）。
// 呼び出し側は全関数ポインタを非 null で設定しなければならない。
struct webrtc_SetLocalDescriptionObserverInterface_cbs {
  void (*OnSetLocalDescriptionComplete)(struct webrtc_RTCError_unique* error,
                                        void* user_data);
  void (*OnDestroy)(void* user_data);
};

WEBRTC_EXPORT struct webrtc_SetLocalDescriptionObserverInterface_refcounted*
webrtc_SetLocalDescriptionObserverInterface_make_ref_counted(
    const struct webrtc_SetLocalDescriptionObserverInterface_cbs* cbs,
    void* user_data);

#if defined(__cplusplus)
}
#endif
