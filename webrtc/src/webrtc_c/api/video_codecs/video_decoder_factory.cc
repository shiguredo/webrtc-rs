#include "video_decoder_factory.h"

#include <stdarg.h>
#include <stddef.h>
#include <memory>

// WebRTC
#include <api/video_codecs/builtin_video_decoder_factory.h>
#include <api/video_codecs/video_decoder_factory.h>

#include "../../common.impl.h"

// -------------------------
// webrtc::VideoDecoderFactory
// -------------------------

extern "C" {
WEBRTC_DEFINE_UNIQUE(webrtc_VideoDecoderFactory, webrtc::VideoDecoderFactory);
struct webrtc_VideoDecoderFactory_unique*
webrtc_CreateBuiltinVideoDecoderFactory() {
  auto factory = webrtc::CreateBuiltinVideoDecoderFactory();
  return reinterpret_cast<struct webrtc_VideoDecoderFactory_unique*>(
      factory.release());
}
}
