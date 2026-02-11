#include "audio_processing.h"

#include <stdarg.h>
#include <stddef.h>
#include <memory>

// WebRTC
#include <api/audio/audio_processing.h>
#include <api/audio/builtin_audio_processing_builder.h>

#include "../../common.impl.h"

// -------------------------
// webrtc::AudioProcessingBuilderInterface
// -------------------------

extern "C" {
WEBRTC_DEFINE_UNIQUE(webrtc_AudioProcessingBuilderInterface,
                     webrtc::AudioProcessingBuilderInterface);
struct webrtc_AudioProcessingBuilderInterface_unique*
webrtc_BuiltinAudioProcessingBuilder_Create() {
  auto builder = std::make_unique<webrtc::BuiltinAudioProcessingBuilder>();
  return reinterpret_cast<
      struct webrtc_AudioProcessingBuilderInterface_unique*>(builder.release());
}
}
