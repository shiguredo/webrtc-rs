#include "rtp_receiver_interface.h"

#include <api/rtp_receiver_interface.h>
#include <api/scoped_refptr.h>

#include "../common.h"
#include "../common.impl.h"
#include "api/frame_transformer_interface.h"
#include "frame_transformer_interface.h"
#include "media_stream_interface.h"

extern "C" {
WEBRTC_DEFINE_REFCOUNTED(webrtc_RtpReceiverInterface,
                         webrtc::RtpReceiverInterface);

WEBRTC_EXPORT struct webrtc_MediaStreamTrackInterface_refcounted*
webrtc_RtpReceiverInterface_track(struct webrtc_RtpReceiverInterface* self) {
  auto receiver = reinterpret_cast<webrtc::RtpReceiverInterface*>(self);
  auto track = receiver->track();
  return reinterpret_cast<struct webrtc_MediaStreamTrackInterface_refcounted*>(
      track.release());
}

WEBRTC_EXPORT void webrtc_RtpReceiverInterface_SetFrameTransformer(
    struct webrtc_RtpReceiverInterface* self,
    struct webrtc_FrameTransformerInterface_refcounted* frame_transformer) {
  auto receiver = reinterpret_cast<webrtc::RtpReceiverInterface*>(self);
  auto transformer = reinterpret_cast<webrtc::FrameTransformerInterface*>(
      webrtc_FrameTransformerInterface_refcounted_get(frame_transformer));
  receiver->SetFrameTransformer(
      webrtc::scoped_refptr<webrtc::FrameTransformerInterface>(transformer));
}
}
