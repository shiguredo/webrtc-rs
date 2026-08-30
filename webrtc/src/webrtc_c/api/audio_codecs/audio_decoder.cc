#include "audio_decoder.h"

#include <stdint.h>
#include <cassert>
#include <cstddef>
#include <memory>

// WebRTC
#include <api/audio_codecs/audio_decoder.h>
#include <rtc_base/buffer.h>

#include "../../common.h"
#include "../../common.impl.h"

namespace {

class AudioDecoderImpl : public webrtc::AudioDecoder {
 public:
  AudioDecoderImpl(const webrtc_AudioDecoder_cbs* cbs, void* user_data)
      : user_data_(user_data) {
    assert(cbs != nullptr);
    assert(cbs->Decode != nullptr);
    assert(cbs->DecodeRedundant != nullptr);
    assert(cbs->HasDecodePlc != nullptr);
    assert(cbs->DecodePlc != nullptr);
    assert(cbs->GeneratePlc != nullptr);
    assert(cbs->Reset != nullptr);
    assert(cbs->ErrorCode != nullptr);
    assert(cbs->PacketDuration != nullptr);
    assert(cbs->PacketDurationRedundant != nullptr);
    assert(cbs->PacketHasFec != nullptr);
    assert(cbs->SampleRateHz != nullptr);
    assert(cbs->Channels != nullptr);
    assert(cbs->OnDestroy != nullptr);
    cbs_ = *cbs;
  }

  ~AudioDecoderImpl() override { cbs_.OnDestroy(user_data_); }

  bool HasDecodePlc() const override {
    return cbs_.HasDecodePlc(user_data_) != 0;
  }

  size_t DecodePlc(size_t num_frames, int16_t* decoded) override {
    return cbs_.DecodePlc(num_frames, decoded, user_data_);
  }

  void GeneratePlc(size_t requested_samples_per_channel,
                   webrtc::BufferT<int16_t>* concealment_audio) override {
    cbs_.GeneratePlc(
        requested_samples_per_channel,
        reinterpret_cast<struct webrtc_BufferS16*>(concealment_audio),
        user_data_);
  }

  void Reset() override { cbs_.Reset(user_data_); }

  int ErrorCode() override { return cbs_.ErrorCode(user_data_); }

  int PacketDuration(const uint8_t* encoded,
                     size_t encoded_len) const override {
    return cbs_.PacketDuration(encoded, encoded_len, user_data_);
  }

  int PacketDurationRedundant(const uint8_t* encoded,
                              size_t encoded_len) const override {
    return cbs_.PacketDurationRedundant(encoded, encoded_len, user_data_);
  }

  bool PacketHasFec(const uint8_t* encoded, size_t encoded_len) const override {
    return cbs_.PacketHasFec(encoded, encoded_len, user_data_) != 0;
  }

  int SampleRateHz() const override { return cbs_.SampleRateHz(user_data_); }

  size_t Channels() const override { return cbs_.Channels(user_data_); }

 protected:
  int DecodeInternal(const uint8_t* encoded,
                     size_t encoded_len,
                     int sample_rate_hz,
                     int16_t* decoded,
                     SpeechType* speech_type) override {
    int st = static_cast<int>(SpeechType::kSpeech);
    int result = cbs_.Decode(encoded, encoded_len, sample_rate_hz, decoded, &st,
                             user_data_);
    *speech_type = static_cast<SpeechType>(st);
    return result;
  }

  int DecodeRedundantInternal(const uint8_t* encoded,
                              size_t encoded_len,
                              int sample_rate_hz,
                              int16_t* decoded,
                              SpeechType* speech_type) override {
    int st = static_cast<int>(SpeechType::kSpeech);
    int result = cbs_.DecodeRedundant(encoded, encoded_len, sample_rate_hz,
                                      decoded, &st, user_data_);
    *speech_type = static_cast<SpeechType>(st);
    return result;
  }

 private:
  webrtc_AudioDecoder_cbs cbs_{};
  void* user_data_ = nullptr;
};

}  // namespace

extern "C" {
// -------------------------
// webrtc::AudioDecoder::SpeechType
// -------------------------

WEBRTC_EXPORT const int webrtc_AudioDecoder_SpeechType_kSpeech =
    static_cast<int>(webrtc::AudioDecoder::SpeechType::kSpeech);
WEBRTC_EXPORT const int webrtc_AudioDecoder_SpeechType_kComfortNoise =
    static_cast<int>(webrtc::AudioDecoder::SpeechType::kComfortNoise);

// -------------------------
// webrtc::AudioDecoder
// -------------------------

WEBRTC_DEFINE_UNIQUE(webrtc_AudioDecoder, webrtc::AudioDecoder);

WEBRTC_EXPORT struct webrtc_AudioDecoder_unique* webrtc_AudioDecoder_new(
    const struct webrtc_AudioDecoder_cbs* cbs,
    void* user_data) {
  auto decoder = new AudioDecoderImpl(cbs, user_data);
  return reinterpret_cast<struct webrtc_AudioDecoder_unique*>(decoder);
}

WEBRTC_EXPORT int webrtc_AudioDecoder_Decode(struct webrtc_AudioDecoder* self,
                                             const uint8_t* encoded,
                                             size_t encoded_len,
                                             int sample_rate_hz,
                                             int16_t* decoded,
                                             size_t max_decoded_bytes,
                                             int* speech_type) {
  assert(self != nullptr);
  auto decoder = reinterpret_cast<webrtc::AudioDecoder*>(self);
  auto speech = static_cast<webrtc::AudioDecoder::SpeechType>(*speech_type);
  auto result = decoder->Decode(encoded, encoded_len, sample_rate_hz,
                                max_decoded_bytes, decoded, &speech);
  *speech_type = static_cast<int>(speech);
  return result;
}

WEBRTC_EXPORT int webrtc_AudioDecoder_DecodeRedundant(
    struct webrtc_AudioDecoder* self,
    const uint8_t* encoded,
    size_t encoded_len,
    int sample_rate_hz,
    int16_t* decoded,
    size_t max_decoded_bytes,
    int* speech_type) {
  assert(self != nullptr);
  auto decoder = reinterpret_cast<webrtc::AudioDecoder*>(self);
  auto speech = static_cast<webrtc::AudioDecoder::SpeechType>(*speech_type);
  auto result = decoder->DecodeRedundant(encoded, encoded_len, sample_rate_hz,
                                         max_decoded_bytes, decoded, &speech);
  *speech_type = static_cast<int>(speech);
  return result;
}

WEBRTC_EXPORT int webrtc_AudioDecoder_HasDecodePlc(
    const struct webrtc_AudioDecoder* self) {
  assert(self != nullptr);
  auto decoder = reinterpret_cast<const webrtc::AudioDecoder*>(self);
  return decoder->HasDecodePlc() ? 1 : 0;
}

WEBRTC_EXPORT size_t
webrtc_AudioDecoder_DecodePlc(struct webrtc_AudioDecoder* self,
                              size_t num_frames,
                              int16_t* decoded) {
  assert(self != nullptr);
  auto decoder = reinterpret_cast<webrtc::AudioDecoder*>(self);
  return decoder->DecodePlc(num_frames, decoded);
}

WEBRTC_EXPORT void webrtc_AudioDecoder_GeneratePlc(
    struct webrtc_AudioDecoder* self,
    size_t requested_samples_per_channel,
    struct webrtc_BufferS16* concealment_audio) {
  assert(self != nullptr);
  auto decoder = reinterpret_cast<webrtc::AudioDecoder*>(self);
  decoder->GeneratePlc(
      requested_samples_per_channel,
      reinterpret_cast<webrtc::BufferT<int16_t>*>(concealment_audio));
}

WEBRTC_EXPORT void webrtc_AudioDecoder_Reset(struct webrtc_AudioDecoder* self) {
  assert(self != nullptr);
  auto decoder = reinterpret_cast<webrtc::AudioDecoder*>(self);
  decoder->Reset();
}

WEBRTC_EXPORT int webrtc_AudioDecoder_ErrorCode(
    struct webrtc_AudioDecoder* self) {
  assert(self != nullptr);
  auto decoder = reinterpret_cast<webrtc::AudioDecoder*>(self);
  return decoder->ErrorCode();
}

WEBRTC_EXPORT int webrtc_AudioDecoder_PacketDuration(
    const struct webrtc_AudioDecoder* self,
    const uint8_t* encoded,
    size_t encoded_len) {
  assert(self != nullptr);
  auto decoder = reinterpret_cast<const webrtc::AudioDecoder*>(self);
  return decoder->PacketDuration(encoded, encoded_len);
}

WEBRTC_EXPORT int webrtc_AudioDecoder_PacketDurationRedundant(
    const struct webrtc_AudioDecoder* self,
    const uint8_t* encoded,
    size_t encoded_len) {
  assert(self != nullptr);
  auto decoder = reinterpret_cast<const webrtc::AudioDecoder*>(self);
  return decoder->PacketDurationRedundant(encoded, encoded_len);
}

WEBRTC_EXPORT int webrtc_AudioDecoder_PacketHasFec(
    const struct webrtc_AudioDecoder* self,
    const uint8_t* encoded,
    size_t encoded_len) {
  assert(self != nullptr);
  auto decoder = reinterpret_cast<const webrtc::AudioDecoder*>(self);
  return decoder->PacketHasFec(encoded, encoded_len) ? 1 : 0;
}

WEBRTC_EXPORT int webrtc_AudioDecoder_SampleRateHz(
    const struct webrtc_AudioDecoder* self) {
  assert(self != nullptr);
  auto decoder = reinterpret_cast<const webrtc::AudioDecoder*>(self);
  return decoder->SampleRateHz();
}

WEBRTC_EXPORT size_t
webrtc_AudioDecoder_Channels(const struct webrtc_AudioDecoder* self) {
  assert(self != nullptr);
  auto decoder = reinterpret_cast<const webrtc::AudioDecoder*>(self);
  return decoder->Channels();
}
}
