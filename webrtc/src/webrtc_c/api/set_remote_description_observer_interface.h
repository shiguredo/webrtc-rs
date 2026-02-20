#pragma once

#include "../common.h"
#include "rtc_error.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::SetRemoteDescriptionObserverInterface
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_SetRemoteDescriptionObserverInterface);

struct webrtc_SetRemoteDescriptionObserverInterface_cbs {
  void (*OnSetRemoteDescriptionComplete)(struct webrtc_RTCError_unique* error,
                                         void* user_data);
  void (*OnDestroy)(void* user_data);
};

struct webrtc_SetRemoteDescriptionObserverInterface_refcounted*
webrtc_SetRemoteDescriptionObserverInterface_make_ref_counted(
    const struct webrtc_SetRemoteDescriptionObserverInterface_cbs* cbs,
    void* user_data);

#if defined(__cplusplus)
}
#endif
