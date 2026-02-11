#include "rtp_receiver_interface.h"

#include <api/rtp_receiver_interface.h>
#include <api/scoped_refptr.h>

#include "../common.impl.h"
#include "media_stream_interface.h"

extern "C" {
WEBRTC_DEFINE_REFCOUNTED(webrtc_RtpReceiverInterface,
                         webrtc::RtpReceiverInterface);

struct webrtc_MediaStreamTrackInterface_refcounted*
webrtc_RtpReceiverInterface_track(struct webrtc_RtpReceiverInterface* self) {
  auto receiver = reinterpret_cast<webrtc::RtpReceiverInterface*>(self);
  auto track = receiver->track();
  return reinterpret_cast<struct webrtc_MediaStreamTrackInterface_refcounted*>(
      track.release());
}
}
