#pragma once

#include "../common.h"
#include "frame_transformer_interface.h"
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
WEBRTC_EXPORT void webrtc_RtpSenderInterface_SetParameters(
    struct webrtc_RtpSenderInterface* self,
    const struct webrtc_RtpParameters* parameters,
    struct webrtc_RTCError_unique** out_rtc_error);
WEBRTC_EXPORT int webrtc_RtpSenderInterface_SetTrack(
    struct webrtc_RtpSenderInterface* self,
    struct webrtc_MediaStreamTrackInterface* track);
WEBRTC_EXPORT void webrtc_RtpSenderInterface_SetFrameTransformer(
    struct webrtc_RtpSenderInterface* self,
    struct webrtc_FrameTransformerInterface_refcounted* frame_transformer);

#if defined(__cplusplus)
}
#endif
