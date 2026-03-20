#include "rtp_transceiver_direction.h"

#include <stdarg.h>
#include <stddef.h>

// WebRTC
#include <api/rtp_transceiver_direction.h>

#include "../common.h"

// -------------------------
// webrtc::RtpTransceiverDirection
// -------------------------

extern "C" {
WEBRTC_EXPORT extern const int webrtc_RtpTransceiverDirection_kSendRecv =
    static_cast<int>(webrtc::RtpTransceiverDirection::kSendRecv);
WEBRTC_EXPORT extern const int webrtc_RtpTransceiverDirection_kSendOnly =
    static_cast<int>(webrtc::RtpTransceiverDirection::kSendOnly);
WEBRTC_EXPORT extern const int webrtc_RtpTransceiverDirection_kRecvOnly =
    static_cast<int>(webrtc::RtpTransceiverDirection::kRecvOnly);
}
