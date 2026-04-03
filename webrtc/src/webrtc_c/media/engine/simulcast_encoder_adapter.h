#pragma once

#include "../../api/environment.h"
#include "../../api/video_codecs/sdp_video_format.h"
#include "../../api/video_codecs/video_encoder.h"
#include "../../api/video_codecs/video_encoder_factory.h"
#include "../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::SimulcastEncoderAdapter
// -------------------------

WEBRTC_DECLARE_UNIQUE(webrtc_SimulcastEncoderAdapter);
WEBRTC_DECLARE_CAST(webrtc_SimulcastEncoderAdapter, webrtc_VideoEncoder);

WEBRTC_EXPORT struct webrtc_SimulcastEncoderAdapter_unique*
webrtc_SimulcastEncoderAdapter_new(
    struct webrtc_Environment* env,
    struct webrtc_VideoEncoderFactory* primary_factory,
    struct webrtc_VideoEncoderFactory* fallback_factory,
    struct webrtc_SdpVideoFormat* format);

#if defined(__cplusplus)
}
#endif
