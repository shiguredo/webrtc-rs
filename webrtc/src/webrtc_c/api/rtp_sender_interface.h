#pragma once

#include "../common.h"
#include "rtc_error.h"
#include "rtp_parameters.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::RtpSenderInterface
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_RtpSenderInterface);

WEBRTC_EXPORT struct webrtc_RtpParameters*
webrtc_RtpSenderInterface_GetParameters(struct webrtc_RtpSenderInterface* self);
WEBRTC_EXPORT struct webrtc_RTCError_unique*
webrtc_RtpSenderInterface_SetParameters(
    struct webrtc_RtpSenderInterface* self,
    const struct webrtc_RtpParameters* parameters);

#if defined(__cplusplus)
}
#endif
