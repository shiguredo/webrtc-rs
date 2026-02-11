#pragma once

#include "../common.h"
#include "../std.h"
#include "rtp_parameters.h"
#include "rtp_receiver_interface.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::RtpTransceiverInit
// -------------------------

struct webrtc_RtpTransceiverInit;
struct webrtc_RtpTransceiverInit* webrtc_RtpTransceiverInit_new();
void webrtc_RtpTransceiverInit_delete(struct webrtc_RtpTransceiverInit* self);
void webrtc_RtpTransceiverInit_set_direction(
    struct webrtc_RtpTransceiverInit* self,
    int direction);
struct std_string_vector* webrtc_RtpTransceiverInit_get_stream_ids(
    struct webrtc_RtpTransceiverInit* self);
void webrtc_RtpTransceiverInit_set_send_encodings(
    struct webrtc_RtpTransceiverInit* self,
    struct webrtc_RtpEncodingParameters_vector* encodings);

// -------------------------
// webrtc::RtpTransceiverInterface
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_RtpTransceiverInterface);

struct webrtc_RTCError_unique*
webrtc_RtpTransceiverInterface_SetCodecPreferences(
    struct webrtc_RtpTransceiverInterface* self,
    struct webrtc_RtpCodecCapability_vector* codecs);
struct webrtc_RtpReceiverInterface_refcounted*
webrtc_RtpTransceiverInterface_receiver(
    struct webrtc_RtpTransceiverInterface* self);
#if defined(__cplusplus)
}
#endif
