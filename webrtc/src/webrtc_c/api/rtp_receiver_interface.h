#pragma once

#include "../common.h"
#include "media_stream_interface.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::RtpReceiverInterface
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_RtpReceiverInterface);

struct webrtc_MediaStreamTrackInterface_refcounted*
webrtc_RtpReceiverInterface_track(struct webrtc_RtpReceiverInterface* self);

#if defined(__cplusplus)
}
#endif
