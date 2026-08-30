#pragma once

#include <stddef.h>
#include <stdint.h>

#include "../../common.h"
#include "../environment.h"
#include "audio_codec_pair_id.h"
#include "audio_encoder.h"
#include "audio_format.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::AudioEncoderFactory::Options
// -------------------------

struct webrtc_AudioEncoderFactory_Options;
WEBRTC_EXPORT struct webrtc_AudioEncoderFactory_Options*
webrtc_AudioEncoderFactory_Options_new();
WEBRTC_EXPORT void webrtc_AudioEncoderFactory_Options_delete(
    struct webrtc_AudioEncoderFactory_Options* self);
WEBRTC_EXPORT int webrtc_AudioEncoderFactory_Options_get_payload_type(
    const struct webrtc_AudioEncoderFactory_Options* self);
WEBRTC_EXPORT void webrtc_AudioEncoderFactory_Options_set_payload_type(
    struct webrtc_AudioEncoderFactory_Options* self,
    int value);
// 返り値は Options が保持する codec_pair_id の所有コピー。未設定の場合は null。
// AudioCodecPairId はデフォルト構築不可のため out パラメータ方式が使えない。
WEBRTC_EXPORT struct webrtc_AudioCodecPairId*
webrtc_AudioEncoderFactory_Options_get_codec_pair_id(
    const struct webrtc_AudioEncoderFactory_Options* self);
WEBRTC_EXPORT void webrtc_AudioEncoderFactory_Options_set_codec_pair_id(
    struct webrtc_AudioEncoderFactory_Options* self,
    int has,
    const struct webrtc_AudioCodecPairId* value);

// -------------------------
// webrtc::AudioEncoderFactory
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_AudioEncoderFactory);

// 全コールバックは必須（null 非許容）。
// 呼び出し側は全関数ポインタを非 null で設定しなければならない。
struct webrtc_AudioEncoderFactory_cbs {
  struct webrtc_AudioCodecSpec_vector* (*GetSupportedEncoders)(void* user_data);
  struct webrtc_AudioCodecInfo* (*QueryAudioEncoder)(
      const struct webrtc_SdpAudioFormat* format,
      void* user_data);
  struct webrtc_AudioEncoder_unique* (*Create)(
      const struct webrtc_Environment* env,
      const struct webrtc_SdpAudioFormat* format,
      struct webrtc_AudioEncoderFactory_Options* options,
      void* user_data);
  void (*OnDestroy)(void* user_data);
};
WEBRTC_EXPORT struct webrtc_AudioEncoderFactory_refcounted*
webrtc_AudioEncoderFactory_make_ref_counted(
    const struct webrtc_AudioEncoderFactory_cbs* cbs,
    void* user_data);
WEBRTC_EXPORT struct webrtc_AudioCodecSpec_vector*
webrtc_AudioEncoderFactory_GetSupportedEncoders(
    struct webrtc_AudioEncoderFactory* self);
WEBRTC_EXPORT struct webrtc_AudioCodecInfo*
webrtc_AudioEncoderFactory_QueryAudioEncoder(
    struct webrtc_AudioEncoderFactory* self,
    const struct webrtc_SdpAudioFormat* format);
WEBRTC_EXPORT struct webrtc_AudioEncoder_unique*
webrtc_AudioEncoderFactory_MakeAudioEncoder(
    struct webrtc_AudioEncoderFactory* self,
    const struct webrtc_Environment* env,
    const struct webrtc_SdpAudioFormat* format,
    struct webrtc_AudioEncoderFactory_Options* options);

WEBRTC_EXPORT struct webrtc_AudioEncoderFactory_refcounted*
webrtc_CreateBuiltinAudioEncoderFactory();

#if defined(__cplusplus)
}
#endif
