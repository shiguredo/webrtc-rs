#include "simulcast_encoder_adapter.h"

// WebRTC
#include <api/environment/environment.h>
#include <api/video_codecs/sdp_video_format.h>
#include <api/video_codecs/video_encoder.h>
#include <api/video_codecs/video_encoder_factory.h>
#include <media/engine/simulcast_encoder_adapter.h>

#include "../../common.h"
#include "../../common.impl.h"

extern "C" {
WEBRTC_DEFINE_UNIQUE(webrtc_SimulcastEncoderAdapter,
                     webrtc::SimulcastEncoderAdapter);
WEBRTC_DEFINE_CAST(webrtc_SimulcastEncoderAdapter,
                   webrtc_VideoEncoder,
                   webrtc::SimulcastEncoderAdapter,
                   webrtc::VideoEncoder);

WEBRTC_EXPORT struct webrtc_SimulcastEncoderAdapter_unique*
webrtc_SimulcastEncoderAdapter_new(
    struct webrtc_Environment* env,
    struct webrtc_VideoEncoderFactory* primary_factory,
    struct webrtc_VideoEncoderFactory* fallback_factory,
    struct webrtc_SdpVideoFormat* format) {
  auto cpp_env = reinterpret_cast<webrtc::Environment*>(env);
  auto cpp_primary_factory =
      reinterpret_cast<webrtc::VideoEncoderFactory*>(primary_factory);
  auto cpp_fallback_factory =
      reinterpret_cast<webrtc::VideoEncoderFactory*>(fallback_factory);
  auto cpp_format = reinterpret_cast<webrtc::SdpVideoFormat*>(format);

  auto adapter = new webrtc::SimulcastEncoderAdapter(
      *cpp_env, cpp_primary_factory, cpp_fallback_factory, *cpp_format);
  return reinterpret_cast<struct webrtc_SimulcastEncoderAdapter_unique*>(
      adapter);
}
}
