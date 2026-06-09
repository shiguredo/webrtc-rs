#include "rtp_sender_interface.h"

#include <assert.h>
#include <memory>

#include <api/rtc_error.h>
#include <api/rtp_parameters.h>
#include <api/rtp_sender_interface.h>

#include "../common.h"
#include "../common.impl.h"
#include "api/media_stream_interface.h"
#include "rtc_error.h"
#include "rtp_parameters.h"

extern "C" {
WEBRTC_DEFINE_REFCOUNTED(webrtc_RtpSenderInterface, webrtc::RtpSenderInterface);

WEBRTC_EXPORT struct webrtc_RtpParameters*
webrtc_RtpSenderInterface_GetParameters(
    struct webrtc_RtpSenderInterface* self) {
  auto sender = reinterpret_cast<webrtc::RtpSenderInterface*>(self);
  auto parameters = new webrtc::RtpParameters(sender->GetParameters());
  return reinterpret_cast<struct webrtc_RtpParameters*>(parameters);
}

WEBRTC_EXPORT void webrtc_RtpSenderInterface_SetParameters(
    struct webrtc_RtpSenderInterface* self,
    const struct webrtc_RtpParameters* parameters,
    struct webrtc_RTCError_unique** out_rtc_error) {
  assert(out_rtc_error != nullptr);
  assert(parameters != nullptr);
  auto sender = reinterpret_cast<webrtc::RtpSenderInterface*>(self);
  auto p = reinterpret_cast<const webrtc::RtpParameters*>(parameters);
  auto result = sender->SetParameters(*p);
  if (result.ok()) {
    *out_rtc_error = nullptr;
  } else {
    auto error = std::make_unique<webrtc::RTCError>(result);
    *out_rtc_error =
        reinterpret_cast<struct webrtc_RTCError_unique*>(error.release());
  }
}

WEBRTC_EXPORT int webrtc_RtpSenderInterface_SetTrack(
    struct webrtc_RtpSenderInterface* self,
    struct webrtc_MediaStreamTrackInterface* track) {
  auto sender = reinterpret_cast<webrtc::RtpSenderInterface*>(self);
  auto media_track =
      reinterpret_cast<webrtc::MediaStreamTrackInterface*>(track);
  return sender->SetTrack(media_track) ? 1 : 0;
}
}
