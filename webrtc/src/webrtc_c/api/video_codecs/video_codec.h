#pragma once

#include <stddef.h>
#include <stdint.h>

#include "../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::VideoCodec / VideoEncoder::Settings /
// webrtc::VideoEncoder::RateControlParameters
// -------------------------

struct webrtc_VideoCodec;
struct webrtc_VideoEncoder_Settings;
struct webrtc_VideoEncoder_RateControlParameters;
struct webrtc_VideoFrameType;

extern const int webrtc_VideoCodecType_Generic;
extern const int webrtc_VideoCodecType_VP8;
extern const int webrtc_VideoCodecType_VP9;
extern const int webrtc_VideoCodecType_AV1;
extern const int webrtc_VideoCodecType_H264;
extern const int webrtc_VideoCodecType_H265;

extern const int webrtc_VideoCodecStatus_TargetBitrateOvershoot;
extern const int webrtc_VideoCodecStatus_OkRequestKeyframe;
extern const int webrtc_VideoCodecStatus_NoOutput;
extern const int webrtc_VideoCodecStatus_Ok;
extern const int webrtc_VideoCodecStatus_Error;
extern const int webrtc_VideoCodecStatus_Memory;
extern const int webrtc_VideoCodecStatus_ErrParameter;
extern const int webrtc_VideoCodecStatus_Timeout;
extern const int webrtc_VideoCodecStatus_Uninitialized;
extern const int webrtc_VideoCodecStatus_FallbackSoftware;
extern const int webrtc_VideoCodecStatus_ErrSimulcastParametersNotSupported;
extern const int webrtc_VideoCodecStatus_EncoderFailure;

extern const int webrtc_VideoFrameType_Empty;
extern const int webrtc_VideoFrameType_Key;
extern const int webrtc_VideoFrameType_Delta;

int webrtc_VideoCodec_codec_type(struct webrtc_VideoCodec* self);
int webrtc_VideoCodec_width(struct webrtc_VideoCodec* self);
int webrtc_VideoCodec_height(struct webrtc_VideoCodec* self);
unsigned int webrtc_VideoCodec_start_bitrate_kbps(
    struct webrtc_VideoCodec* self);
unsigned int webrtc_VideoCodec_max_bitrate_kbps(struct webrtc_VideoCodec* self);
unsigned int webrtc_VideoCodec_min_bitrate_kbps(struct webrtc_VideoCodec* self);
uint32_t webrtc_VideoCodec_max_framerate(struct webrtc_VideoCodec* self);

int webrtc_VideoEncoder_Settings_number_of_cores(
    struct webrtc_VideoEncoder_Settings* self);
size_t webrtc_VideoEncoder_Settings_max_payload_size(
    struct webrtc_VideoEncoder_Settings* self);
int webrtc_VideoEncoder_Settings_loss_notification(
    struct webrtc_VideoEncoder_Settings* self);
int webrtc_VideoEncoder_Settings_has_encoder_thread_limit(
    struct webrtc_VideoEncoder_Settings* self);
int webrtc_VideoEncoder_Settings_encoder_thread_limit(
    struct webrtc_VideoEncoder_Settings* self);

double webrtc_VideoEncoder_RateControlParameters_framerate_fps(
    struct webrtc_VideoEncoder_RateControlParameters* self);
uint32_t webrtc_VideoEncoder_RateControlParameters_target_bitrate_sum_bps(
    struct webrtc_VideoEncoder_RateControlParameters* self);
uint32_t webrtc_VideoEncoder_RateControlParameters_bitrate_sum_bps(
    struct webrtc_VideoEncoder_RateControlParameters* self);
int64_t webrtc_VideoEncoder_RateControlParameters_bandwidth_allocation_bps(
    struct webrtc_VideoEncoder_RateControlParameters* self);

WEBRTC_DECLARE_VECTOR(webrtc_VideoFrameType);
int webrtc_VideoFrameType_value(struct webrtc_VideoFrameType* self);
void webrtc_VideoFrameType_vector_push_back_value(
    struct webrtc_VideoFrameType_vector* self,
    int value);

#if defined(__cplusplus)
}
#endif
