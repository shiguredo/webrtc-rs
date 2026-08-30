#pragma once

#include "../../common.h"
#include "../environment.h"
#include "audio_decoder.h"
#include "audio_format.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::AudioDecoderFactory
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_AudioDecoderFactory);

// 全コールバックは必須（null 非許容）。
// 呼び出し側は全関数ポインタを非 null で設定しなければならない。
struct webrtc_AudioDecoderFactory_cbs {
  struct webrtc_AudioCodecSpec_vector* (*GetSupportedDecoders)(void* user_data);
  int (*IsSupportedDecoder)(const struct webrtc_SdpAudioFormat* format,
                            void* user_data);
  struct webrtc_AudioDecoder_unique* (*Create)(
      const struct webrtc_Environment* env,
      const struct webrtc_SdpAudioFormat* format,
      void* user_data);
  void (*OnDestroy)(void* user_data);
};
WEBRTC_EXPORT struct webrtc_AudioDecoderFactory_refcounted*
webrtc_AudioDecoderFactory_make_ref_counted(
    const struct webrtc_AudioDecoderFactory_cbs* cbs,
    void* user_data);
WEBRTC_EXPORT struct webrtc_AudioCodecSpec_vector*
webrtc_AudioDecoderFactory_GetSupportedDecoders(
    struct webrtc_AudioDecoderFactory* self);
WEBRTC_EXPORT int webrtc_AudioDecoderFactory_IsSupportedDecoder(
    struct webrtc_AudioDecoderFactory* self,
    const struct webrtc_SdpAudioFormat* format);
WEBRTC_EXPORT struct webrtc_AudioDecoder_unique*
webrtc_AudioDecoderFactory_MakeAudioDecoder(
    struct webrtc_AudioDecoderFactory* self,
    const struct webrtc_Environment* env,
    const struct webrtc_SdpAudioFormat* format);

WEBRTC_EXPORT struct webrtc_AudioDecoderFactory_refcounted*
webrtc_CreateBuiltinAudioDecoderFactory();

#if defined(__cplusplus)
}
#endif
