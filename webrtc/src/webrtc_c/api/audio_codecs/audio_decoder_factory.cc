#include "audio_decoder_factory.h"

#include <stdarg.h>
#include <stddef.h>

// WebRTC
#include <api/audio_codecs/audio_decoder_factory.h>
#include <api/audio_codecs/builtin_audio_decoder_factory.h>
#include <api/scoped_refptr.h>

#include "../../common.impl.h"

// -------------------------
// webrtc::AudioDecoderFactory
// -------------------------

extern "C" {
WEBRTC_DEFINE_REFCOUNTED(webrtc_AudioDecoderFactory,
                         webrtc::AudioDecoderFactory);
struct webrtc_AudioDecoderFactory_refcounted*
webrtc_CreateBuiltinAudioDecoderFactory() {
  auto factory = webrtc::CreateBuiltinAudioDecoderFactory();
  return reinterpret_cast<struct webrtc_AudioDecoderFactory_refcounted*>(
      factory.release());
}
}
