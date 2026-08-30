#pragma once

#include <stddef.h>
#include <stdint.h>

#include "../../common.h"
#include "../../rtc_base/buffer.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::AudioEncoder
// -------------------------

WEBRTC_DECLARE_UNIQUE(webrtc_AudioEncoder);

// std::vector<std::unique_ptr<webrtc::AudioEncoder>>
struct webrtc_AudioEncoder_unique_vector;
WEBRTC_EXPORT struct webrtc_AudioEncoder_unique_vector*
webrtc_AudioEncoder_unique_vector_new();
WEBRTC_EXPORT void webrtc_AudioEncoder_unique_vector_delete(
    struct webrtc_AudioEncoder_unique_vector* self);
WEBRTC_EXPORT size_t webrtc_AudioEncoder_unique_vector_size(
    struct webrtc_AudioEncoder_unique_vector* self);
WEBRTC_EXPORT void webrtc_AudioEncoder_unique_vector_push_back(
    struct webrtc_AudioEncoder_unique_vector* self,
    struct webrtc_AudioEncoder_unique* value);

// webrtc::AudioEncoder::EncodedInfo
WEBRTC_DECLARE_UNIQUE(webrtc_AudioEncoder_EncodedInfo);
WEBRTC_EXPORT struct webrtc_AudioEncoder_EncodedInfo_unique*
webrtc_AudioEncoder_EncodedInfo_new();
WEBRTC_EXPORT size_t webrtc_AudioEncoder_EncodedInfo_get_encoded_bytes(
    struct webrtc_AudioEncoder_EncodedInfo* self);
WEBRTC_EXPORT void webrtc_AudioEncoder_EncodedInfo_set_encoded_bytes(
    struct webrtc_AudioEncoder_EncodedInfo* self,
    size_t value);
WEBRTC_EXPORT uint32_t webrtc_AudioEncoder_EncodedInfo_get_encoded_timestamp(
    struct webrtc_AudioEncoder_EncodedInfo* self);
WEBRTC_EXPORT void webrtc_AudioEncoder_EncodedInfo_set_encoded_timestamp(
    struct webrtc_AudioEncoder_EncodedInfo* self,
    uint32_t value);
WEBRTC_EXPORT int webrtc_AudioEncoder_EncodedInfo_get_payload_type(
    struct webrtc_AudioEncoder_EncodedInfo* self);
WEBRTC_EXPORT void webrtc_AudioEncoder_EncodedInfo_set_payload_type(
    struct webrtc_AudioEncoder_EncodedInfo* self,
    int value);
WEBRTC_EXPORT int webrtc_AudioEncoder_EncodedInfo_get_send_even_if_empty(
    struct webrtc_AudioEncoder_EncodedInfo* self);
WEBRTC_EXPORT void webrtc_AudioEncoder_EncodedInfo_set_send_even_if_empty(
    struct webrtc_AudioEncoder_EncodedInfo* self,
    int value);
WEBRTC_EXPORT int webrtc_AudioEncoder_EncodedInfo_get_speech(
    struct webrtc_AudioEncoder_EncodedInfo* self);
WEBRTC_EXPORT void webrtc_AudioEncoder_EncodedInfo_set_speech(
    struct webrtc_AudioEncoder_EncodedInfo* self,
    int value);
WEBRTC_EXPORT int webrtc_AudioEncoder_EncodedInfo_get_encoder_type(
    struct webrtc_AudioEncoder_EncodedInfo* self);
WEBRTC_EXPORT void webrtc_AudioEncoder_EncodedInfo_set_encoder_type(
    struct webrtc_AudioEncoder_EncodedInfo* self,
    int value);

// webrtc::ANAStats (Audio Network Adaptation)
struct webrtc_AudioEncoder_ANAStats;
WEBRTC_EXPORT struct webrtc_AudioEncoder_ANAStats*
webrtc_AudioEncoder_ANAStats_new();
WEBRTC_EXPORT void webrtc_AudioEncoder_ANAStats_delete(
    struct webrtc_AudioEncoder_ANAStats* self);
WEBRTC_EXPORT struct webrtc_AudioEncoder_ANAStats*
webrtc_AudioEncoder_ANAStats_copy(
    const struct webrtc_AudioEncoder_ANAStats* self);
WEBRTC_EXPORT void webrtc_AudioEncoder_ANAStats_get_bitrate_action_counter(
    const struct webrtc_AudioEncoder_ANAStats* self,
    int* out_has,
    uint32_t* out_value);
WEBRTC_EXPORT void webrtc_AudioEncoder_ANAStats_set_bitrate_action_counter(
    struct webrtc_AudioEncoder_ANAStats* self,
    int has,
    const uint32_t* value);
WEBRTC_EXPORT void webrtc_AudioEncoder_ANAStats_get_channel_action_counter(
    const struct webrtc_AudioEncoder_ANAStats* self,
    int* out_has,
    uint32_t* out_value);
WEBRTC_EXPORT void webrtc_AudioEncoder_ANAStats_set_channel_action_counter(
    struct webrtc_AudioEncoder_ANAStats* self,
    int has,
    const uint32_t* value);
WEBRTC_EXPORT void webrtc_AudioEncoder_ANAStats_get_dtx_action_counter(
    const struct webrtc_AudioEncoder_ANAStats* self,
    int* out_has,
    uint32_t* out_value);
WEBRTC_EXPORT void webrtc_AudioEncoder_ANAStats_set_dtx_action_counter(
    struct webrtc_AudioEncoder_ANAStats* self,
    int has,
    const uint32_t* value);
WEBRTC_EXPORT void webrtc_AudioEncoder_ANAStats_get_fec_action_counter(
    const struct webrtc_AudioEncoder_ANAStats* self,
    int* out_has,
    uint32_t* out_value);
WEBRTC_EXPORT void webrtc_AudioEncoder_ANAStats_set_fec_action_counter(
    struct webrtc_AudioEncoder_ANAStats* self,
    int has,
    const uint32_t* value);
WEBRTC_EXPORT void
webrtc_AudioEncoder_ANAStats_get_frame_length_increase_counter(
    const struct webrtc_AudioEncoder_ANAStats* self,
    int* out_has,
    uint32_t* out_value);
WEBRTC_EXPORT void
webrtc_AudioEncoder_ANAStats_set_frame_length_increase_counter(
    struct webrtc_AudioEncoder_ANAStats* self,
    int has,
    const uint32_t* value);
WEBRTC_EXPORT void
webrtc_AudioEncoder_ANAStats_get_frame_length_decrease_counter(
    const struct webrtc_AudioEncoder_ANAStats* self,
    int* out_has,
    uint32_t* out_value);
WEBRTC_EXPORT void
webrtc_AudioEncoder_ANAStats_set_frame_length_decrease_counter(
    struct webrtc_AudioEncoder_ANAStats* self,
    int has,
    const uint32_t* value);
WEBRTC_EXPORT void webrtc_AudioEncoder_ANAStats_get_uplink_packet_loss_fraction(
    const struct webrtc_AudioEncoder_ANAStats* self,
    int* out_has,
    float* out_value);
WEBRTC_EXPORT void webrtc_AudioEncoder_ANAStats_set_uplink_packet_loss_fraction(
    struct webrtc_AudioEncoder_ANAStats* self,
    int has,
    const float* value);

// 全コールバックは必須（null 非許容）。
// 呼び出し側は全関数ポインタを非 null で設定しなければならない。
struct webrtc_AudioEncoder_cbs {
  int (*SampleRateHz)(void* user_data);
  size_t (*NumChannels)(void* user_data);
  int (*RtpTimestampRateHz)(void* user_data);
  size_t (*Num10MsFramesInNextPacket)(void* user_data);
  size_t (*Max10MsFramesInAPacket)(void* user_data);
  int (*GetTargetBitrate)(void* user_data);
  struct webrtc_AudioEncoder_EncodedInfo_unique* (*Encode)(
      uint32_t rtp_timestamp,
      const int16_t* audio,
      size_t audio_size,
      struct webrtc_Buffer* encoded,
      void* user_data);
  void (*Reset)(void* user_data);
  int (*SetFec)(int enable, void* user_data);
  int (*SetDtx)(int enable, void* user_data);
  int (*GetDtx)(void* user_data);
  int (*SetApplication)(int application, void* user_data);
  void (*SetMaxPlaybackRate)(int frequency_hz, void* user_data);
  struct webrtc_AudioEncoder_unique_vector* (*ReclaimContainedEncoders)(
      void* user_data);
  int (*EnableAudioNetworkAdaptor)(const uint8_t* config,
                                   size_t config_len,
                                   void* user_data);
  void (*DisableAudioNetworkAdaptor)(void* user_data);
  void (*OnReceivedUplinkPacketLossFraction)(float fraction, void* user_data);
  void (*OnReceivedTargetAudioBitrate)(int target_bps, void* user_data);
  void (*OnReceivedUplinkAllocation)(int64_t target_bitrate_bps,
                                     int64_t prediction_interval_us,
                                     void* user_data);
  void (*OnReceivedRtt)(int rtt_ms, void* user_data);
  void (*OnReceivedOverhead)(size_t overhead_bytes_per_packet, void* user_data);
  void (*SetReceiverFrameLengthRange)(int min_frame_length_ms,
                                      int max_frame_length_ms,
                                      void* user_data);
  struct webrtc_AudioEncoder_ANAStats* (*GetANAStats)(void* user_data);
  void (*GetFrameLengthRange)(int* out_has,
                              int64_t* out_min_us,
                              int64_t* out_max_us,
                              void* user_data);
  void (*GetBitrateRange)(int* out_has,
                          int64_t* out_min_bps,
                          int64_t* out_max_bps,
                          void* user_data);
  void (*OnDestroy)(void* user_data);
};

WEBRTC_EXPORT struct webrtc_AudioEncoder_unique* webrtc_AudioEncoder_new(
    const struct webrtc_AudioEncoder_cbs* cbs,
    void* user_data);
WEBRTC_EXPORT int webrtc_AudioEncoder_SampleRateHz(
    const struct webrtc_AudioEncoder* self);
WEBRTC_EXPORT size_t
webrtc_AudioEncoder_NumChannels(const struct webrtc_AudioEncoder* self);
WEBRTC_EXPORT int webrtc_AudioEncoder_RtpTimestampRateHz(
    const struct webrtc_AudioEncoder* self);
WEBRTC_EXPORT size_t webrtc_AudioEncoder_Num10MsFramesInNextPacket(
    const struct webrtc_AudioEncoder* self);
WEBRTC_EXPORT size_t webrtc_AudioEncoder_Max10MsFramesInAPacket(
    const struct webrtc_AudioEncoder* self);
WEBRTC_EXPORT int webrtc_AudioEncoder_GetTargetBitrate(
    const struct webrtc_AudioEncoder* self);
WEBRTC_EXPORT struct webrtc_AudioEncoder_EncodedInfo_unique*
webrtc_AudioEncoder_Encode(struct webrtc_AudioEncoder* self,
                           uint32_t rtp_timestamp,
                           const int16_t* audio,
                           size_t audio_size,
                           struct webrtc_Buffer* encoded);
WEBRTC_EXPORT void webrtc_AudioEncoder_Reset(struct webrtc_AudioEncoder* self);
WEBRTC_EXPORT int webrtc_AudioEncoder_SetFec(struct webrtc_AudioEncoder* self,
                                             int enable);
WEBRTC_EXPORT int webrtc_AudioEncoder_SetDtx(struct webrtc_AudioEncoder* self,
                                             int enable);
WEBRTC_EXPORT int webrtc_AudioEncoder_GetDtx(
    const struct webrtc_AudioEncoder* self);
WEBRTC_EXPORT int webrtc_AudioEncoder_SetApplication(
    struct webrtc_AudioEncoder* self,
    int application);
WEBRTC_EXPORT void webrtc_AudioEncoder_SetMaxPlaybackRate(
    struct webrtc_AudioEncoder* self,
    int frequency_hz);
WEBRTC_EXPORT struct webrtc_AudioEncoder_unique_vector*
webrtc_AudioEncoder_ReclaimContainedEncoders(struct webrtc_AudioEncoder* self);
WEBRTC_EXPORT int webrtc_AudioEncoder_EnableAudioNetworkAdaptor(
    struct webrtc_AudioEncoder* self,
    const uint8_t* config,
    size_t config_len);
WEBRTC_EXPORT void webrtc_AudioEncoder_DisableAudioNetworkAdaptor(
    struct webrtc_AudioEncoder* self);
WEBRTC_EXPORT void webrtc_AudioEncoder_OnReceivedUplinkPacketLossFraction(
    struct webrtc_AudioEncoder* self,
    float fraction);
WEBRTC_EXPORT void webrtc_AudioEncoder_OnReceivedTargetAudioBitrate(
    struct webrtc_AudioEncoder* self,
    int target_bps);
WEBRTC_EXPORT void webrtc_AudioEncoder_OnReceivedUplinkAllocation(
    struct webrtc_AudioEncoder* self,
    int64_t target_bitrate_bps,
    int64_t prediction_interval_us);
WEBRTC_EXPORT void webrtc_AudioEncoder_OnReceivedRtt(
    struct webrtc_AudioEncoder* self,
    int rtt_ms);
WEBRTC_EXPORT void webrtc_AudioEncoder_OnReceivedOverhead(
    struct webrtc_AudioEncoder* self,
    size_t overhead_bytes_per_packet);
WEBRTC_EXPORT void webrtc_AudioEncoder_SetReceiverFrameLengthRange(
    struct webrtc_AudioEncoder* self,
    int min_frame_length_ms,
    int max_frame_length_ms);
WEBRTC_EXPORT int webrtc_AudioEncoder_GetANAStats(
    const struct webrtc_AudioEncoder* self,
    struct webrtc_AudioEncoder_ANAStats* out);
WEBRTC_EXPORT void webrtc_AudioEncoder_GetFrameLengthRange(
    const struct webrtc_AudioEncoder* self,
    int* out_has,
    int64_t* out_min_us,
    int64_t* out_max_us);
WEBRTC_EXPORT void webrtc_AudioEncoder_GetBitrateRange(
    const struct webrtc_AudioEncoder* self,
    int* out_has,
    int64_t* out_min_bps,
    int64_t* out_max_bps);

// -------------------------
// webrtc::AudioEncoder::CodecType
// -------------------------

WEBRTC_EXPORT extern const int webrtc_AudioEncoder_CodecType_Other;
WEBRTC_EXPORT extern const int webrtc_AudioEncoder_CodecType_Opus;
WEBRTC_EXPORT extern const int webrtc_AudioEncoder_CodecType_Isac;
WEBRTC_EXPORT extern const int webrtc_AudioEncoder_CodecType_PcmA;
WEBRTC_EXPORT extern const int webrtc_AudioEncoder_CodecType_PcmU;
WEBRTC_EXPORT extern const int webrtc_AudioEncoder_CodecType_G722;

#if defined(__cplusplus)
}
#endif
