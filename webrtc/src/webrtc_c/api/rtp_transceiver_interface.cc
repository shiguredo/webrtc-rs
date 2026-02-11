#include "rtp_transceiver_interface.h"

#include <stdarg.h>
#include <stddef.h>
#include <vector>

// WebRTC
#include <api/rtc_error.h>
#include <api/rtp_parameters.h>
#include <api/rtp_transceiver_direction.h>
#include <api/rtp_transceiver_interface.h>

#include "../common.impl.h"
#include "../std.h"
#include "rtp_parameters.h"
#include "rtp_receiver_interface.h"

// -------------------------
// webrtc::RtpTransceiverInit
// -------------------------

extern "C" {
struct webrtc_RtpTransceiverInit* webrtc_RtpTransceiverInit_new() {
  auto init = new webrtc::RtpTransceiverInit();
  return reinterpret_cast<struct webrtc_RtpTransceiverInit*>(init);
}
void webrtc_RtpTransceiverInit_delete(struct webrtc_RtpTransceiverInit* self) {
  auto init = reinterpret_cast<webrtc::RtpTransceiverInit*>(self);
  delete init;
}
void webrtc_RtpTransceiverInit_set_direction(
    struct webrtc_RtpTransceiverInit* self,
    int direction) {
  auto init = reinterpret_cast<webrtc::RtpTransceiverInit*>(self);
  init->direction = static_cast<webrtc::RtpTransceiverDirection>(direction);
}
struct std_string_vector* webrtc_RtpTransceiverInit_get_stream_ids(
    struct webrtc_RtpTransceiverInit* self) {
  auto init = reinterpret_cast<webrtc::RtpTransceiverInit*>(self);
  return reinterpret_cast<struct std_string_vector*>(&init->stream_ids);
}
void webrtc_RtpTransceiverInit_set_send_encodings(
    struct webrtc_RtpTransceiverInit* self,
    struct webrtc_RtpEncodingParameters_vector* encodings) {
  auto init = reinterpret_cast<webrtc::RtpTransceiverInit*>(self);
  auto vec =
      reinterpret_cast<std::vector<webrtc::RtpEncodingParameters>*>(encodings);
  init->send_encodings = *vec;
}
}

// -------------------------
// webrtc::RtpTransceiverInterface
// -------------------------

WEBRTC_DEFINE_REFCOUNTED(webrtc_RtpTransceiverInterface,
                         webrtc::RtpTransceiverInterface);

webrtc_RTCError_unique* webrtc_RtpTransceiverInterface_SetCodecPreferences(
    struct webrtc_RtpTransceiverInterface* self,
    struct webrtc_RtpCodecCapability_vector* codecs) {
  auto transceiver = reinterpret_cast<webrtc::RtpTransceiverInterface*>(self);
  auto vec = reinterpret_cast<std::vector<webrtc::RtpCodecCapability>*>(codecs);
  auto result = transceiver->SetCodecPreferences(*vec);
  if (result.ok()) {
    return nullptr;
  } else {
    return reinterpret_cast<webrtc_RTCError_unique*>(
        new webrtc::RTCError(result));
  }
}

struct webrtc_RtpReceiverInterface_refcounted*
webrtc_RtpTransceiverInterface_receiver(
    struct webrtc_RtpTransceiverInterface* self) {
  auto transceiver = reinterpret_cast<webrtc::RtpTransceiverInterface*>(self);
  auto receiver = transceiver->receiver();
  return reinterpret_cast<struct webrtc_RtpReceiverInterface_refcounted*>(
      receiver.release());
}
