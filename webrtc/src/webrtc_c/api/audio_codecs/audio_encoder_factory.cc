#include "audio_encoder_factory.h"

#include <stdarg.h>
#include <stddef.h>

// WebRTC
#include <api/audio_codecs/audio_encoder_factory.h>
#include <api/audio_codecs/builtin_audio_encoder_factory.h>
#include <api/scoped_refptr.h>

#include "../../common.impl.h"

// -------------------------
// webrtc::AudioEncoderFactory
// -------------------------

extern "C" {
WEBRTC_DEFINE_REFCOUNTED(webrtc_AudioEncoderFactory,
                         webrtc::AudioEncoderFactory);
struct webrtc_AudioEncoderFactory_refcounted*
webrtc_CreateBuiltinAudioEncoderFactory() {
  auto factory = webrtc::CreateBuiltinAudioEncoderFactory();
  return reinterpret_cast<struct webrtc_AudioEncoderFactory_refcounted*>(
      factory.release());
}
}
