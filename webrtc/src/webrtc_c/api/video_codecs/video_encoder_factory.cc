#include "video_encoder_factory.h"

#include <stdarg.h>
#include <stddef.h>
#include <memory>

// WebRTC
#include <api/video_codecs/builtin_video_encoder_factory.h>
#include <api/video_codecs/video_encoder_factory.h>

#include "../../common.impl.h"

// -------------------------
// webrtc::VideoEncoderFactory
// -------------------------

extern "C" {
WEBRTC_DEFINE_UNIQUE(webrtc_VideoEncoderFactory, webrtc::VideoEncoderFactory);
struct webrtc_VideoEncoderFactory_unique*
webrtc_CreateBuiltinVideoEncoderFactory() {
  auto factory = webrtc::CreateBuiltinVideoEncoderFactory();
  return reinterpret_cast<struct webrtc_VideoEncoderFactory_unique*>(
      factory.release());
}
}
