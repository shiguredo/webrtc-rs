#include "audio_codec_pair_id.h"

#include <cstdint>

// WebRTC
#include <api/audio_codecs/audio_codec_pair_id.h>

#include "../../common.h"

extern "C" {
WEBRTC_EXPORT struct webrtc_AudioCodecPairId* webrtc_AudioCodecPairId_Create() {
  auto id = new webrtc::AudioCodecPairId(webrtc::AudioCodecPairId::Create());
  return reinterpret_cast<struct webrtc_AudioCodecPairId*>(id);
}

WEBRTC_EXPORT struct webrtc_AudioCodecPairId* webrtc_AudioCodecPairId_copy(
    const struct webrtc_AudioCodecPairId* self) {
  auto id = reinterpret_cast<const webrtc::AudioCodecPairId*>(self);
  auto copied = new webrtc::AudioCodecPairId(*id);
  return reinterpret_cast<struct webrtc_AudioCodecPairId*>(copied);
}

WEBRTC_EXPORT void webrtc_AudioCodecPairId_delete(
    struct webrtc_AudioCodecPairId* self) {
  auto id = reinterpret_cast<webrtc::AudioCodecPairId*>(self);
  delete id;
}

WEBRTC_EXPORT uint64_t webrtc_AudioCodecPairId_NumericRepresentation(
    const struct webrtc_AudioCodecPairId* self) {
  auto id = reinterpret_cast<const webrtc::AudioCodecPairId*>(self);
  return id->NumericRepresentation();
}

WEBRTC_EXPORT int webrtc_AudioCodecPairId_is_equal(
    const struct webrtc_AudioCodecPairId* a,
    const struct webrtc_AudioCodecPairId* b) {
  auto lhs = reinterpret_cast<const webrtc::AudioCodecPairId*>(a);
  auto rhs = reinterpret_cast<const webrtc::AudioCodecPairId*>(b);
  return *lhs == *rhs ? 1 : 0;
}

WEBRTC_EXPORT int webrtc_AudioCodecPairId_less(
    const struct webrtc_AudioCodecPairId* a,
    const struct webrtc_AudioCodecPairId* b) {
  auto lhs = reinterpret_cast<const webrtc::AudioCodecPairId*>(a);
  auto rhs = reinterpret_cast<const webrtc::AudioCodecPairId*>(b);
  return *lhs < *rhs ? 1 : 0;
}
}
