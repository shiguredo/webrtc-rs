#pragma once

#include <stddef.h>
#include <stdint.h>

#include "../../common.h"
#include "../../rtc_base/buffer.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::AudioDecoder::SpeechType
// -------------------------

WEBRTC_EXPORT extern const int webrtc_AudioDecoder_SpeechType_kSpeech;
WEBRTC_EXPORT extern const int webrtc_AudioDecoder_SpeechType_kComfortNoise;

// -------------------------
// webrtc::AudioDecoder
// -------------------------

WEBRTC_DECLARE_UNIQUE(webrtc_AudioDecoder);

// 全コールバックは必須（null 非許容）。
// 呼び出し側は全関数ポインタを非 null で設定しなければならない。
struct webrtc_AudioDecoder_cbs {
  int (*Decode)(const uint8_t* encoded,
                size_t encoded_len,
                int sample_rate_hz,
                int16_t* decoded,
                int* speech_type,
                void* user_data);
  int (*DecodeRedundant)(const uint8_t* encoded,
                         size_t encoded_len,
                         int sample_rate_hz,
                         int16_t* decoded,
                         int* speech_type,
                         void* user_data);
  int (*HasDecodePlc)(void* user_data);
  size_t (*DecodePlc)(size_t num_frames, int16_t* decoded, void* user_data);
  int (*GeneratePlc)(size_t requested_samples_per_channel,
                     struct webrtc_BufferS16* concealment_audio,
                     void* user_data);
  void (*Reset)(void* user_data);
  int (*ErrorCode)(void* user_data);
  int (*PacketDuration)(const uint8_t* encoded,
                        size_t encoded_len,
                        void* user_data);
  int (*PacketDurationRedundant)(const uint8_t* encoded,
                                 size_t encoded_len,
                                 void* user_data);
  int (*PacketHasFec)(const uint8_t* encoded,
                      size_t encoded_len,
                      void* user_data);
  int (*SampleRateHz)(void* user_data);
  size_t (*Channels)(void* user_data);
  void (*OnDestroy)(void* user_data);
};

WEBRTC_EXPORT struct webrtc_AudioDecoder_unique* webrtc_AudioDecoder_new(
    const struct webrtc_AudioDecoder_cbs* cbs,
    void* user_data);
WEBRTC_EXPORT int webrtc_AudioDecoder_Decode(struct webrtc_AudioDecoder* self,
                                             const uint8_t* encoded,
                                             size_t encoded_len,
                                             int sample_rate_hz,
                                             int16_t* decoded,
                                             size_t max_decoded_bytes,
                                             int* speech_type);
WEBRTC_EXPORT int webrtc_AudioDecoder_DecodeRedundant(
    struct webrtc_AudioDecoder* self,
    const uint8_t* encoded,
    size_t encoded_len,
    int sample_rate_hz,
    int16_t* decoded,
    size_t max_decoded_bytes,
    int* speech_type);
WEBRTC_EXPORT int webrtc_AudioDecoder_HasDecodePlc(
    const struct webrtc_AudioDecoder* self);
WEBRTC_EXPORT size_t
webrtc_AudioDecoder_DecodePlc(struct webrtc_AudioDecoder* self,
                              size_t num_frames,
                              int16_t* decoded);
WEBRTC_EXPORT void webrtc_AudioDecoder_GeneratePlc(
    struct webrtc_AudioDecoder* self,
    size_t requested_samples_per_channel,
    struct webrtc_BufferS16* concealment_audio);
WEBRTC_EXPORT void webrtc_AudioDecoder_Reset(struct webrtc_AudioDecoder* self);
WEBRTC_EXPORT int webrtc_AudioDecoder_ErrorCode(
    struct webrtc_AudioDecoder* self);
WEBRTC_EXPORT int webrtc_AudioDecoder_PacketDuration(
    const struct webrtc_AudioDecoder* self,
    const uint8_t* encoded,
    size_t encoded_len);
WEBRTC_EXPORT int webrtc_AudioDecoder_PacketDurationRedundant(
    const struct webrtc_AudioDecoder* self,
    const uint8_t* encoded,
    size_t encoded_len);
WEBRTC_EXPORT int webrtc_AudioDecoder_PacketHasFec(
    const struct webrtc_AudioDecoder* self,
    const uint8_t* encoded,
    size_t encoded_len);
WEBRTC_EXPORT int webrtc_AudioDecoder_SampleRateHz(
    const struct webrtc_AudioDecoder* self);
WEBRTC_EXPORT size_t
webrtc_AudioDecoder_Channels(const struct webrtc_AudioDecoder* self);

#if defined(__cplusplus)
}
#endif
