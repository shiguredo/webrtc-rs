#pragma once

#include "../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::RtpTransceiverDirection
// -------------------------

WEBRTC_EXPORT extern const int webrtc_RtpTransceiverDirection_kSendRecv;
WEBRTC_EXPORT extern const int webrtc_RtpTransceiverDirection_kSendOnly;
WEBRTC_EXPORT extern const int webrtc_RtpTransceiverDirection_kRecvOnly;

#if defined(__cplusplus)
}
#endif
