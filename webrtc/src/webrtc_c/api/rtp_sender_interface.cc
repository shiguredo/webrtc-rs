#include "rtp_sender_interface.h"

#include <assert.h>

#include <api/rtp_sender_interface.h>

#include "../common.impl.h"

extern "C" {
WEBRTC_DEFINE_REFCOUNTED(webrtc_RtpSenderInterface, webrtc::RtpSenderInterface);

struct webrtc_RtpParameters* webrtc_RtpSenderInterface_GetParameters(
    struct webrtc_RtpSenderInterface* self) {
  auto sender = reinterpret_cast<webrtc::RtpSenderInterface*>(self);
  auto parameters = new webrtc::RtpParameters(sender->GetParameters());
  return reinterpret_cast<struct webrtc_RtpParameters*>(parameters);
}

struct webrtc_RTCError_unique* webrtc_RtpSenderInterface_SetParameters(
    struct webrtc_RtpSenderInterface* self,
    const struct webrtc_RtpParameters* parameters) {
  auto sender = reinterpret_cast<webrtc::RtpSenderInterface*>(self);
  assert(parameters != nullptr);
  if (parameters == nullptr) {
    return reinterpret_cast<struct webrtc_RTCError_unique*>(
        new webrtc::RTCError(webrtc::RTCErrorType::INVALID_PARAMETER,
                             "parameters is null"));
  }
  auto p = reinterpret_cast<const webrtc::RtpParameters*>(parameters);
  auto result = sender->SetParameters(*p);
  if (result.ok()) {
    return nullptr;
  }
  return reinterpret_cast<struct webrtc_RTCError_unique*>(
      new webrtc::RTCError(result));
}
}
