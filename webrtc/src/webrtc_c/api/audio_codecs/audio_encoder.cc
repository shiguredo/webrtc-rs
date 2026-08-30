#include "audio_encoder.h"

#include <stdint.h>
#include <cassert>
#include <cstddef>
#include <memory>
#include <optional>
#include <utility>
#include <vector>

// WebRTC
#include <api/audio_codecs/audio_encoder.h>
#include <api/call/bitrate_allocation.h>
#include <api/units/data_rate.h>
#include <api/units/time_delta.h>
#include <rtc_base/buffer.h>

#include "../../common.h"
#include "../../common.impl.h"
#include "../../std.impl.h"

namespace {

class AudioEncoderImpl : public webrtc::AudioEncoder {
 public:
  AudioEncoderImpl(const webrtc_AudioEncoder_cbs* cbs, void* user_data)
      : user_data_(user_data) {
    assert(cbs != nullptr);
    assert(cbs->SampleRateHz != nullptr);
    assert(cbs->NumChannels != nullptr);
    assert(cbs->RtpTimestampRateHz != nullptr);
    assert(cbs->Num10MsFramesInNextPacket != nullptr);
    assert(cbs->Max10MsFramesInAPacket != nullptr);
    assert(cbs->GetTargetBitrate != nullptr);
    assert(cbs->Encode != nullptr);
    assert(cbs->Reset != nullptr);
    assert(cbs->SetFec != nullptr);
    assert(cbs->SetDtx != nullptr);
    assert(cbs->GetDtx != nullptr);
    assert(cbs->SetApplication != nullptr);
    assert(cbs->SetMaxPlaybackRate != nullptr);
    assert(cbs->ReclaimContainedEncoders != nullptr);
    assert(cbs->EnableAudioNetworkAdaptor != nullptr);
    assert(cbs->DisableAudioNetworkAdaptor != nullptr);
    assert(cbs->OnReceivedUplinkPacketLossFraction != nullptr);
    assert(cbs->OnReceivedTargetAudioBitrate != nullptr);
    assert(cbs->OnReceivedUplinkAllocation != nullptr);
    assert(cbs->OnReceivedRtt != nullptr);
    assert(cbs->OnReceivedOverhead != nullptr);
    assert(cbs->SetReceiverFrameLengthRange != nullptr);
    assert(cbs->GetANAStats != nullptr);
    assert(cbs->GetFrameLengthRange != nullptr);
    assert(cbs->GetBitrateRange != nullptr);
    assert(cbs->OnDestroy != nullptr);
    cbs_ = *cbs;
  }

  ~AudioEncoderImpl() override { cbs_.OnDestroy(user_data_); }

  int SampleRateHz() const override { return cbs_.SampleRateHz(user_data_); }

  size_t NumChannels() const override { return cbs_.NumChannels(user_data_); }

  int RtpTimestampRateHz() const override {
    return cbs_.RtpTimestampRateHz(user_data_);
  }

  size_t Num10MsFramesInNextPacket() const override {
    return cbs_.Num10MsFramesInNextPacket(user_data_);
  }

  size_t Max10MsFramesInAPacket() const override {
    return cbs_.Max10MsFramesInAPacket(user_data_);
  }

  int GetTargetBitrate() const override {
    return cbs_.GetTargetBitrate(user_data_);
  }

  void Reset() override { cbs_.Reset(user_data_); }

  bool SetFec(bool enable) override {
    return cbs_.SetFec(enable ? 1 : 0, user_data_) != 0;
  }

  bool SetDtx(bool enable) override {
    return cbs_.SetDtx(enable ? 1 : 0, user_data_) != 0;
  }

  bool GetDtx() const override { return cbs_.GetDtx(user_data_) != 0; }

  bool SetApplication(Application application) override {
    return cbs_.SetApplication(static_cast<int>(application), user_data_) != 0;
  }

  void SetMaxPlaybackRate(int frequency_hz) override {
    cbs_.SetMaxPlaybackRate(frequency_hz, user_data_);
  }

  std::span<std::unique_ptr<webrtc::AudioEncoder>> ReclaimContainedEncoders()
      override {
    reclaimed_.clear();
    auto raw = cbs_.ReclaimContainedEncoders(user_data_);
    if (raw != nullptr) {
      auto vec =
          reinterpret_cast<std::vector<std::unique_ptr<webrtc::AudioEncoder>>*>(
              raw);
      reclaimed_ = std::move(*vec);
      webrtc_AudioEncoder_unique_vector_delete(raw);
    }
    return reclaimed_;
  }

  bool EnableAudioNetworkAdaptor(absl::string_view config) override {
    return cbs_.EnableAudioNetworkAdaptor(
               reinterpret_cast<const uint8_t*>(config.data()), config.size(),
               user_data_) != 0;
  }

  void DisableAudioNetworkAdaptor() override {
    cbs_.DisableAudioNetworkAdaptor(user_data_);
  }

  void OnReceivedUplinkPacketLossFraction(float fraction) override {
    cbs_.OnReceivedUplinkPacketLossFraction(fraction, user_data_);
  }

  void OnReceivedTargetAudioBitrate(int target_bps) override {
    cbs_.OnReceivedTargetAudioBitrate(target_bps, user_data_);
  }

  void OnReceivedUplinkAllocation(
      webrtc::BitrateAllocationUpdate update) override {
    cbs_.OnReceivedUplinkAllocation(update.target_bitrate.bps(), -1,
                                    user_data_);
  }

  void OnReceivedRtt(int rtt_ms) override {
    cbs_.OnReceivedRtt(rtt_ms, user_data_);
  }

  void OnReceivedOverhead(size_t overhead_bytes_per_packet) override {
    cbs_.OnReceivedOverhead(overhead_bytes_per_packet, user_data_);
  }

  void SetReceiverFrameLengthRange(int min_frame_length_ms,
                                   int max_frame_length_ms) override {
    cbs_.SetReceiverFrameLengthRange(min_frame_length_ms, max_frame_length_ms,
                                     user_data_);
  }

  webrtc::ANAStats GetANAStats() const override {
    auto raw = cbs_.GetANAStats(user_data_);
    webrtc::ANAStats stats = *reinterpret_cast<webrtc::ANAStats*>(raw);
    webrtc_AudioEncoder_ANAStats_delete(raw);
    return stats;
  }

  std::optional<std::pair<webrtc::TimeDelta, webrtc::TimeDelta>>
  GetFrameLengthRange() const override {
    int has = 0;
    int64_t min_us = 0;
    int64_t max_us = 0;
    cbs_.GetFrameLengthRange(&has, &min_us, &max_us, user_data_);
    if (!has) {
      return std::nullopt;
    }
    return std::make_pair(webrtc::TimeDelta::Micros(min_us),
                          webrtc::TimeDelta::Micros(max_us));
  }

  std::optional<std::pair<webrtc::DataRate, webrtc::DataRate>> GetBitrateRange()
      const override {
    int has = 0;
    int64_t min_bps = 0;
    int64_t max_bps = 0;
    cbs_.GetBitrateRange(&has, &min_bps, &max_bps, user_data_);
    if (!has) {
      return std::nullopt;
    }
    return std::make_pair(webrtc::DataRate::BitsPerSec(min_bps),
                          webrtc::DataRate::BitsPerSec(max_bps));
  }

 protected:
  webrtc::AudioEncoder::EncodedInfo EncodeImpl(
      uint32_t rtp_timestamp,
      std::span<const int16_t> audio,
      webrtc::Buffer* encoded) override {
    encoded->Clear();
    auto raw = cbs_.Encode(rtp_timestamp, audio.data(), audio.size(),
                           reinterpret_cast<struct webrtc_Buffer*>(encoded),
                           user_data_);
    if (raw == nullptr) {
      return webrtc::AudioEncoder::EncodedInfo{};
    }
    auto cpp = reinterpret_cast<webrtc::AudioEncoder::EncodedInfo*>(
        webrtc_AudioEncoder_EncodedInfo_unique_get(raw));
    webrtc::AudioEncoder::EncodedInfo result = *cpp;
    webrtc_AudioEncoder_EncodedInfo_unique_delete(raw);
    return result;
  }

 private:
  webrtc_AudioEncoder_cbs cbs_{};
  void* user_data_ = nullptr;
  std::vector<std::unique_ptr<webrtc::AudioEncoder>> reclaimed_;
};

}  // namespace

extern "C" {
WEBRTC_DEFINE_UNIQUE(webrtc_AudioEncoder, webrtc::AudioEncoder);
WEBRTC_DEFINE_UNIQUE(webrtc_AudioEncoder_EncodedInfo,
                     webrtc::AudioEncoder::EncodedInfo);

// -------------------------
// std::vector<std::unique_ptr<webrtc::AudioEncoder>>
// -------------------------

WEBRTC_EXPORT struct webrtc_AudioEncoder_unique_vector*
webrtc_AudioEncoder_unique_vector_new() {
  auto vec = new std::vector<std::unique_ptr<webrtc::AudioEncoder>>();
  return reinterpret_cast<struct webrtc_AudioEncoder_unique_vector*>(vec);
}

WEBRTC_EXPORT void webrtc_AudioEncoder_unique_vector_delete(
    struct webrtc_AudioEncoder_unique_vector* self) {
  auto vec =
      reinterpret_cast<std::vector<std::unique_ptr<webrtc::AudioEncoder>>*>(
          self);
  delete vec;
}

WEBRTC_EXPORT size_t webrtc_AudioEncoder_unique_vector_size(
    struct webrtc_AudioEncoder_unique_vector* self) {
  auto vec =
      reinterpret_cast<std::vector<std::unique_ptr<webrtc::AudioEncoder>>*>(
          self);
  return vec->size();
}

WEBRTC_EXPORT void webrtc_AudioEncoder_unique_vector_push_back(
    struct webrtc_AudioEncoder_unique_vector* self,
    struct webrtc_AudioEncoder_unique* value) {
  auto vec =
      reinterpret_cast<std::vector<std::unique_ptr<webrtc::AudioEncoder>>*>(
          self);
  auto cpp = reinterpret_cast<webrtc::AudioEncoder*>(value);
  vec->push_back(std::unique_ptr<webrtc::AudioEncoder>(cpp));
}

// -------------------------
// webrtc::AudioEncoder::EncodedInfo
// -------------------------

WEBRTC_EXPORT struct webrtc_AudioEncoder_EncodedInfo_unique*
webrtc_AudioEncoder_EncodedInfo_new() {
  auto info = std::make_unique<webrtc::AudioEncoder::EncodedInfo>();
  return reinterpret_cast<struct webrtc_AudioEncoder_EncodedInfo_unique*>(
      info.release());
}

WEBRTC_EXPORT size_t webrtc_AudioEncoder_EncodedInfo_get_encoded_bytes(
    struct webrtc_AudioEncoder_EncodedInfo* self) {
  auto info = reinterpret_cast<webrtc::AudioEncoder::EncodedInfo*>(self);
  return info->encoded_bytes;
}

WEBRTC_EXPORT void webrtc_AudioEncoder_EncodedInfo_set_encoded_bytes(
    struct webrtc_AudioEncoder_EncodedInfo* self,
    size_t value) {
  auto info = reinterpret_cast<webrtc::AudioEncoder::EncodedInfo*>(self);
  info->encoded_bytes = value;
}

WEBRTC_EXPORT uint32_t webrtc_AudioEncoder_EncodedInfo_get_encoded_timestamp(
    struct webrtc_AudioEncoder_EncodedInfo* self) {
  auto info = reinterpret_cast<webrtc::AudioEncoder::EncodedInfo*>(self);
  return info->encoded_timestamp;
}

WEBRTC_EXPORT void webrtc_AudioEncoder_EncodedInfo_set_encoded_timestamp(
    struct webrtc_AudioEncoder_EncodedInfo* self,
    uint32_t value) {
  auto info = reinterpret_cast<webrtc::AudioEncoder::EncodedInfo*>(self);
  info->encoded_timestamp = value;
}

WEBRTC_EXPORT int webrtc_AudioEncoder_EncodedInfo_get_payload_type(
    struct webrtc_AudioEncoder_EncodedInfo* self) {
  auto info = reinterpret_cast<webrtc::AudioEncoder::EncodedInfo*>(self);
  return info->payload_type;
}

WEBRTC_EXPORT void webrtc_AudioEncoder_EncodedInfo_set_payload_type(
    struct webrtc_AudioEncoder_EncodedInfo* self,
    int value) {
  auto info = reinterpret_cast<webrtc::AudioEncoder::EncodedInfo*>(self);
  info->payload_type = value;
}

WEBRTC_EXPORT int webrtc_AudioEncoder_EncodedInfo_get_send_even_if_empty(
    struct webrtc_AudioEncoder_EncodedInfo* self) {
  auto info = reinterpret_cast<webrtc::AudioEncoder::EncodedInfo*>(self);
  return info->send_even_if_empty ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_AudioEncoder_EncodedInfo_set_send_even_if_empty(
    struct webrtc_AudioEncoder_EncodedInfo* self,
    int value) {
  auto info = reinterpret_cast<webrtc::AudioEncoder::EncodedInfo*>(self);
  info->send_even_if_empty = value != 0;
}

WEBRTC_EXPORT int webrtc_AudioEncoder_EncodedInfo_get_speech(
    struct webrtc_AudioEncoder_EncodedInfo* self) {
  auto info = reinterpret_cast<webrtc::AudioEncoder::EncodedInfo*>(self);
  return info->speech ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_AudioEncoder_EncodedInfo_set_speech(
    struct webrtc_AudioEncoder_EncodedInfo* self,
    int value) {
  auto info = reinterpret_cast<webrtc::AudioEncoder::EncodedInfo*>(self);
  info->speech = value != 0;
}

WEBRTC_EXPORT int webrtc_AudioEncoder_EncodedInfo_get_encoder_type(
    struct webrtc_AudioEncoder_EncodedInfo* self) {
  auto info = reinterpret_cast<webrtc::AudioEncoder::EncodedInfo*>(self);
  return static_cast<int>(info->encoder_type);
}

WEBRTC_EXPORT void webrtc_AudioEncoder_EncodedInfo_set_encoder_type(
    struct webrtc_AudioEncoder_EncodedInfo* self,
    int value) {
  auto info = reinterpret_cast<webrtc::AudioEncoder::EncodedInfo*>(self);
  info->encoder_type = static_cast<webrtc::AudioEncoder::CodecType>(value);
}

// -------------------------
// webrtc::AudioEncoder
// -------------------------

WEBRTC_EXPORT struct webrtc_AudioEncoder_unique* webrtc_AudioEncoder_new(
    const struct webrtc_AudioEncoder_cbs* cbs,
    void* user_data) {
  auto encoder = new AudioEncoderImpl(cbs, user_data);
  return reinterpret_cast<struct webrtc_AudioEncoder_unique*>(encoder);
}

WEBRTC_EXPORT int webrtc_AudioEncoder_SampleRateHz(
    const struct webrtc_AudioEncoder* self) {
  assert(self != nullptr);
  auto encoder = reinterpret_cast<const webrtc::AudioEncoder*>(self);
  return encoder->SampleRateHz();
}

WEBRTC_EXPORT size_t
webrtc_AudioEncoder_NumChannels(const struct webrtc_AudioEncoder* self) {
  assert(self != nullptr);
  auto encoder = reinterpret_cast<const webrtc::AudioEncoder*>(self);
  return encoder->NumChannels();
}

WEBRTC_EXPORT int webrtc_AudioEncoder_RtpTimestampRateHz(
    const struct webrtc_AudioEncoder* self) {
  assert(self != nullptr);
  auto encoder = reinterpret_cast<const webrtc::AudioEncoder*>(self);
  return encoder->RtpTimestampRateHz();
}

WEBRTC_EXPORT size_t webrtc_AudioEncoder_Num10MsFramesInNextPacket(
    const struct webrtc_AudioEncoder* self) {
  assert(self != nullptr);
  auto encoder = reinterpret_cast<const webrtc::AudioEncoder*>(self);
  return encoder->Num10MsFramesInNextPacket();
}

WEBRTC_EXPORT size_t webrtc_AudioEncoder_Max10MsFramesInAPacket(
    const struct webrtc_AudioEncoder* self) {
  assert(self != nullptr);
  auto encoder = reinterpret_cast<const webrtc::AudioEncoder*>(self);
  return encoder->Max10MsFramesInAPacket();
}

WEBRTC_EXPORT int webrtc_AudioEncoder_GetTargetBitrate(
    const struct webrtc_AudioEncoder* self) {
  assert(self != nullptr);
  auto encoder = reinterpret_cast<const webrtc::AudioEncoder*>(self);
  return encoder->GetTargetBitrate();
}

WEBRTC_EXPORT struct webrtc_AudioEncoder_EncodedInfo_unique*
webrtc_AudioEncoder_Encode(struct webrtc_AudioEncoder* self,
                           uint32_t rtp_timestamp,
                           const int16_t* audio,
                           size_t audio_size,
                           struct webrtc_Buffer* encoded) {
  assert(self != nullptr);
  auto encoder = reinterpret_cast<webrtc::AudioEncoder*>(self);
  auto buffer = reinterpret_cast<webrtc::Buffer*>(encoded);
  std::span<const int16_t> audio_span(audio, audio_size);
  auto info = std::make_unique<webrtc::AudioEncoder::EncodedInfo>(
      encoder->Encode(rtp_timestamp, audio_span, buffer));
  return reinterpret_cast<struct webrtc_AudioEncoder_EncodedInfo_unique*>(
      info.release());
}

WEBRTC_EXPORT void webrtc_AudioEncoder_Reset(struct webrtc_AudioEncoder* self) {
  assert(self != nullptr);
  auto encoder = reinterpret_cast<webrtc::AudioEncoder*>(self);
  encoder->Reset();
}

WEBRTC_EXPORT int webrtc_AudioEncoder_SetFec(struct webrtc_AudioEncoder* self,
                                             int enable) {
  assert(self != nullptr);
  auto encoder = reinterpret_cast<webrtc::AudioEncoder*>(self);
  return encoder->SetFec(enable != 0) ? 1 : 0;
}

WEBRTC_EXPORT int webrtc_AudioEncoder_SetDtx(struct webrtc_AudioEncoder* self,
                                             int enable) {
  assert(self != nullptr);
  auto encoder = reinterpret_cast<webrtc::AudioEncoder*>(self);
  return encoder->SetDtx(enable != 0) ? 1 : 0;
}

WEBRTC_EXPORT int webrtc_AudioEncoder_GetDtx(
    const struct webrtc_AudioEncoder* self) {
  assert(self != nullptr);
  auto encoder = reinterpret_cast<const webrtc::AudioEncoder*>(self);
  return encoder->GetDtx() ? 1 : 0;
}

WEBRTC_EXPORT int webrtc_AudioEncoder_SetApplication(
    struct webrtc_AudioEncoder* self,
    int application) {
  assert(self != nullptr);
  auto encoder = reinterpret_cast<webrtc::AudioEncoder*>(self);
  return encoder->SetApplication(
             static_cast<webrtc::AudioEncoder::Application>(application))
             ? 1
             : 0;
}

WEBRTC_EXPORT void webrtc_AudioEncoder_SetMaxPlaybackRate(
    struct webrtc_AudioEncoder* self,
    int frequency_hz) {
  assert(self != nullptr);
  auto encoder = reinterpret_cast<webrtc::AudioEncoder*>(self);
  encoder->SetMaxPlaybackRate(frequency_hz);
}

WEBRTC_EXPORT struct webrtc_AudioEncoder_unique_vector*
webrtc_AudioEncoder_ReclaimContainedEncoders(struct webrtc_AudioEncoder* self) {
  assert(self != nullptr);
  auto encoder = reinterpret_cast<webrtc::AudioEncoder*>(self);
  auto reclaimed = encoder->ReclaimContainedEncoders();
  auto vec = new std::vector<std::unique_ptr<webrtc::AudioEncoder>>();
  for (auto& ptr : reclaimed) {
    vec->push_back(std::move(ptr));
  }
  return reinterpret_cast<struct webrtc_AudioEncoder_unique_vector*>(vec);
}

WEBRTC_EXPORT int webrtc_AudioEncoder_EnableAudioNetworkAdaptor(
    struct webrtc_AudioEncoder* self,
    const uint8_t* config,
    size_t config_len) {
  assert(self != nullptr);
  auto encoder = reinterpret_cast<webrtc::AudioEncoder*>(self);
  return encoder->EnableAudioNetworkAdaptor(absl::string_view(
             reinterpret_cast<const char*>(config), config_len))
             ? 1
             : 0;
}

WEBRTC_EXPORT void webrtc_AudioEncoder_DisableAudioNetworkAdaptor(
    struct webrtc_AudioEncoder* self) {
  assert(self != nullptr);
  auto encoder = reinterpret_cast<webrtc::AudioEncoder*>(self);
  encoder->DisableAudioNetworkAdaptor();
}

WEBRTC_EXPORT void webrtc_AudioEncoder_OnReceivedUplinkPacketLossFraction(
    struct webrtc_AudioEncoder* self,
    float fraction) {
  assert(self != nullptr);
  auto encoder = reinterpret_cast<webrtc::AudioEncoder*>(self);
  encoder->OnReceivedUplinkPacketLossFraction(fraction);
}

WEBRTC_EXPORT void webrtc_AudioEncoder_OnReceivedTargetAudioBitrate(
    struct webrtc_AudioEncoder* self,
    int target_bps) {
  assert(self != nullptr);
  auto encoder = reinterpret_cast<webrtc::AudioEncoder*>(self);
  encoder->OnReceivedTargetAudioBitrate(target_bps);
}

WEBRTC_EXPORT void webrtc_AudioEncoder_OnReceivedUplinkAllocation(
    struct webrtc_AudioEncoder* self,
    int64_t target_bitrate_bps,
    int64_t prediction_interval_us) {
  assert(self != nullptr);
  auto encoder = reinterpret_cast<webrtc::AudioEncoder*>(self);
  webrtc::BitrateAllocationUpdate update;
  update.target_bitrate = webrtc::DataRate::BitsPerSec(target_bitrate_bps);
  (void)prediction_interval_us;
  encoder->OnReceivedUplinkAllocation(update);
}

WEBRTC_EXPORT void webrtc_AudioEncoder_OnReceivedRtt(
    struct webrtc_AudioEncoder* self,
    int rtt_ms) {
  assert(self != nullptr);
  auto encoder = reinterpret_cast<webrtc::AudioEncoder*>(self);
  encoder->OnReceivedRtt(rtt_ms);
}

WEBRTC_EXPORT void webrtc_AudioEncoder_OnReceivedOverhead(
    struct webrtc_AudioEncoder* self,
    size_t overhead_bytes_per_packet) {
  assert(self != nullptr);
  auto encoder = reinterpret_cast<webrtc::AudioEncoder*>(self);
  encoder->OnReceivedOverhead(overhead_bytes_per_packet);
}

WEBRTC_EXPORT void webrtc_AudioEncoder_SetReceiverFrameLengthRange(
    struct webrtc_AudioEncoder* self,
    int min_frame_length_ms,
    int max_frame_length_ms) {
  assert(self != nullptr);
  auto encoder = reinterpret_cast<webrtc::AudioEncoder*>(self);
  encoder->SetReceiverFrameLengthRange(min_frame_length_ms,
                                       max_frame_length_ms);
}

WEBRTC_EXPORT struct webrtc_AudioEncoder_ANAStats*
webrtc_AudioEncoder_ANAStats_new() {
  auto stats = new webrtc::ANAStats();
  return reinterpret_cast<struct webrtc_AudioEncoder_ANAStats*>(stats);
}

WEBRTC_EXPORT void webrtc_AudioEncoder_ANAStats_delete(
    struct webrtc_AudioEncoder_ANAStats* self) {
  auto stats = reinterpret_cast<webrtc::ANAStats*>(self);
  delete stats;
}

WEBRTC_EXPORT struct webrtc_AudioEncoder_ANAStats*
webrtc_AudioEncoder_ANAStats_copy(
    const struct webrtc_AudioEncoder_ANAStats* self) {
  auto stats = reinterpret_cast<const webrtc::ANAStats*>(self);
  auto copied = new webrtc::ANAStats(*stats);
  return reinterpret_cast<struct webrtc_AudioEncoder_ANAStats*>(copied);
}

// ANAStats の各フィールドは std::optional なので、getter は has + value、
// setter は has と value を受け取って optional を設定・解除する。
WEBRTC_EXPORT void webrtc_AudioEncoder_ANAStats_get_bitrate_action_counter(
    const struct webrtc_AudioEncoder_ANAStats* self,
    int* out_has,
    uint32_t* out_value) {
  auto stats = reinterpret_cast<const webrtc::ANAStats*>(self);
  webrtc_c::OptionalGet(stats->bitrate_action_counter, out_has, out_value);
}

WEBRTC_EXPORT void webrtc_AudioEncoder_ANAStats_set_bitrate_action_counter(
    struct webrtc_AudioEncoder_ANAStats* self,
    int has,
    const uint32_t* value) {
  auto stats = reinterpret_cast<webrtc::ANAStats*>(self);
  webrtc_c::OptionalSet(stats->bitrate_action_counter, has, value);
}

WEBRTC_EXPORT void webrtc_AudioEncoder_ANAStats_get_channel_action_counter(
    const struct webrtc_AudioEncoder_ANAStats* self,
    int* out_has,
    uint32_t* out_value) {
  auto stats = reinterpret_cast<const webrtc::ANAStats*>(self);
  webrtc_c::OptionalGet(stats->channel_action_counter, out_has, out_value);
}

WEBRTC_EXPORT void webrtc_AudioEncoder_ANAStats_set_channel_action_counter(
    struct webrtc_AudioEncoder_ANAStats* self,
    int has,
    const uint32_t* value) {
  auto stats = reinterpret_cast<webrtc::ANAStats*>(self);
  webrtc_c::OptionalSet(stats->channel_action_counter, has, value);
}

WEBRTC_EXPORT void webrtc_AudioEncoder_ANAStats_get_dtx_action_counter(
    const struct webrtc_AudioEncoder_ANAStats* self,
    int* out_has,
    uint32_t* out_value) {
  auto stats = reinterpret_cast<const webrtc::ANAStats*>(self);
  webrtc_c::OptionalGet(stats->dtx_action_counter, out_has, out_value);
}

WEBRTC_EXPORT void webrtc_AudioEncoder_ANAStats_set_dtx_action_counter(
    struct webrtc_AudioEncoder_ANAStats* self,
    int has,
    const uint32_t* value) {
  auto stats = reinterpret_cast<webrtc::ANAStats*>(self);
  webrtc_c::OptionalSet(stats->dtx_action_counter, has, value);
}

WEBRTC_EXPORT void webrtc_AudioEncoder_ANAStats_get_fec_action_counter(
    const struct webrtc_AudioEncoder_ANAStats* self,
    int* out_has,
    uint32_t* out_value) {
  auto stats = reinterpret_cast<const webrtc::ANAStats*>(self);
  webrtc_c::OptionalGet(stats->fec_action_counter, out_has, out_value);
}

WEBRTC_EXPORT void webrtc_AudioEncoder_ANAStats_set_fec_action_counter(
    struct webrtc_AudioEncoder_ANAStats* self,
    int has,
    const uint32_t* value) {
  auto stats = reinterpret_cast<webrtc::ANAStats*>(self);
  webrtc_c::OptionalSet(stats->fec_action_counter, has, value);
}

WEBRTC_EXPORT void
webrtc_AudioEncoder_ANAStats_get_frame_length_increase_counter(
    const struct webrtc_AudioEncoder_ANAStats* self,
    int* out_has,
    uint32_t* out_value) {
  auto stats = reinterpret_cast<const webrtc::ANAStats*>(self);
  webrtc_c::OptionalGet(stats->frame_length_increase_counter, out_has,
                        out_value);
}

WEBRTC_EXPORT void
webrtc_AudioEncoder_ANAStats_set_frame_length_increase_counter(
    struct webrtc_AudioEncoder_ANAStats* self,
    int has,
    const uint32_t* value) {
  auto stats = reinterpret_cast<webrtc::ANAStats*>(self);
  webrtc_c::OptionalSet(stats->frame_length_increase_counter, has, value);
}

WEBRTC_EXPORT void
webrtc_AudioEncoder_ANAStats_get_frame_length_decrease_counter(
    const struct webrtc_AudioEncoder_ANAStats* self,
    int* out_has,
    uint32_t* out_value) {
  auto stats = reinterpret_cast<const webrtc::ANAStats*>(self);
  webrtc_c::OptionalGet(stats->frame_length_decrease_counter, out_has,
                        out_value);
}

WEBRTC_EXPORT void
webrtc_AudioEncoder_ANAStats_set_frame_length_decrease_counter(
    struct webrtc_AudioEncoder_ANAStats* self,
    int has,
    const uint32_t* value) {
  auto stats = reinterpret_cast<webrtc::ANAStats*>(self);
  webrtc_c::OptionalSet(stats->frame_length_decrease_counter, has, value);
}

WEBRTC_EXPORT void webrtc_AudioEncoder_ANAStats_get_uplink_packet_loss_fraction(
    const struct webrtc_AudioEncoder_ANAStats* self,
    int* out_has,
    float* out_value) {
  auto stats = reinterpret_cast<const webrtc::ANAStats*>(self);
  webrtc_c::OptionalGet(stats->uplink_packet_loss_fraction, out_has, out_value);
}

WEBRTC_EXPORT void webrtc_AudioEncoder_ANAStats_set_uplink_packet_loss_fraction(
    struct webrtc_AudioEncoder_ANAStats* self,
    int has,
    const float* value) {
  auto stats = reinterpret_cast<webrtc::ANAStats*>(self);
  webrtc_c::OptionalSet(stats->uplink_packet_loss_fraction, has, value);
}

WEBRTC_EXPORT int webrtc_AudioEncoder_GetANAStats(
    const struct webrtc_AudioEncoder* self,
    struct webrtc_AudioEncoder_ANAStats* out) {
  assert(self != nullptr);
  auto encoder = reinterpret_cast<const webrtc::AudioEncoder*>(self);
  *reinterpret_cast<webrtc::ANAStats*>(out) = encoder->GetANAStats();
  return 1;
}

WEBRTC_EXPORT void webrtc_AudioEncoder_GetFrameLengthRange(
    const struct webrtc_AudioEncoder* self,
    int* out_has,
    int64_t* out_min_us,
    int64_t* out_max_us) {
  assert(self != nullptr);
  auto encoder = reinterpret_cast<const webrtc::AudioEncoder*>(self);
  auto range = encoder->GetFrameLengthRange();
  if (!range.has_value()) {
    *out_has = 0;
    return;
  }
  *out_has = 1;
  *out_min_us = range->first.us();
  *out_max_us = range->second.us();
}

WEBRTC_EXPORT void webrtc_AudioEncoder_GetBitrateRange(
    const struct webrtc_AudioEncoder* self,
    int* out_has,
    int64_t* out_min_bps,
    int64_t* out_max_bps) {
  assert(self != nullptr);
  auto encoder = reinterpret_cast<const webrtc::AudioEncoder*>(self);
  auto range = encoder->GetBitrateRange();
  if (!range.has_value()) {
    *out_has = 0;
    return;
  }
  *out_has = 1;
  *out_min_bps = range->first.bps();
  *out_max_bps = range->second.bps();
}
}
