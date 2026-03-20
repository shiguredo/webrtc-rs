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

WEBRTC_EXPORT extern const int webrtc_VideoCodecType_Generic;
WEBRTC_EXPORT extern const int webrtc_VideoCodecType_VP8;
WEBRTC_EXPORT extern const int webrtc_VideoCodecType_VP9;
WEBRTC_EXPORT extern const int webrtc_VideoCodecType_AV1;
WEBRTC_EXPORT extern const int webrtc_VideoCodecType_H264;
WEBRTC_EXPORT extern const int webrtc_VideoCodecType_H265;

WEBRTC_EXPORT extern const int webrtc_VideoCodecStatus_TargetBitrateOvershoot;
WEBRTC_EXPORT extern const int webrtc_VideoCodecStatus_OkRequestKeyframe;
WEBRTC_EXPORT extern const int webrtc_VideoCodecStatus_NoOutput;
WEBRTC_EXPORT extern const int webrtc_VideoCodecStatus_Ok;
WEBRTC_EXPORT extern const int webrtc_VideoCodecStatus_Error;
WEBRTC_EXPORT extern const int webrtc_VideoCodecStatus_Memory;
WEBRTC_EXPORT extern const int webrtc_VideoCodecStatus_ErrParameter;
WEBRTC_EXPORT extern const int webrtc_VideoCodecStatus_Timeout;
WEBRTC_EXPORT extern const int webrtc_VideoCodecStatus_Uninitialized;
WEBRTC_EXPORT extern const int webrtc_VideoCodecStatus_FallbackSoftware;
WEBRTC_EXPORT extern const int
    webrtc_VideoCodecStatus_ErrSimulcastParametersNotSupported;
WEBRTC_EXPORT extern const int webrtc_VideoCodecStatus_EncoderFailure;

WEBRTC_EXPORT extern const int webrtc_VideoFrameType_Empty;
WEBRTC_EXPORT extern const int webrtc_VideoFrameType_Key;
WEBRTC_EXPORT extern const int webrtc_VideoFrameType_Delta;

WEBRTC_EXPORT int webrtc_VideoCodec_codec_type(struct webrtc_VideoCodec* self);
WEBRTC_EXPORT int webrtc_VideoCodec_width(struct webrtc_VideoCodec* self);
WEBRTC_EXPORT int webrtc_VideoCodec_height(struct webrtc_VideoCodec* self);
WEBRTC_EXPORT unsigned int webrtc_VideoCodec_start_bitrate_kbps(
    struct webrtc_VideoCodec* self);
WEBRTC_EXPORT unsigned int webrtc_VideoCodec_max_bitrate_kbps(
    struct webrtc_VideoCodec* self);
WEBRTC_EXPORT unsigned int webrtc_VideoCodec_min_bitrate_kbps(
    struct webrtc_VideoCodec* self);
WEBRTC_EXPORT uint32_t
webrtc_VideoCodec_max_framerate(struct webrtc_VideoCodec* self);

WEBRTC_EXPORT int webrtc_VideoEncoder_Settings_number_of_cores(
    struct webrtc_VideoEncoder_Settings* self);
WEBRTC_EXPORT size_t webrtc_VideoEncoder_Settings_max_payload_size(
    struct webrtc_VideoEncoder_Settings* self);
WEBRTC_EXPORT int webrtc_VideoEncoder_Settings_loss_notification(
    struct webrtc_VideoEncoder_Settings* self);
WEBRTC_EXPORT int webrtc_VideoEncoder_Settings_has_encoder_thread_limit(
    struct webrtc_VideoEncoder_Settings* self);
WEBRTC_EXPORT int webrtc_VideoEncoder_Settings_encoder_thread_limit(
    struct webrtc_VideoEncoder_Settings* self);

WEBRTC_EXPORT double webrtc_VideoEncoder_RateControlParameters_framerate_fps(
    struct webrtc_VideoEncoder_RateControlParameters* self);
WEBRTC_EXPORT uint32_t
webrtc_VideoEncoder_RateControlParameters_target_bitrate_sum_bps(
    struct webrtc_VideoEncoder_RateControlParameters* self);
WEBRTC_EXPORT uint32_t
webrtc_VideoEncoder_RateControlParameters_bitrate_sum_bps(
    struct webrtc_VideoEncoder_RateControlParameters* self);
WEBRTC_EXPORT int64_t
webrtc_VideoEncoder_RateControlParameters_bandwidth_allocation_bps(
    struct webrtc_VideoEncoder_RateControlParameters* self);

WEBRTC_DECLARE_VECTOR(webrtc_VideoFrameType);
WEBRTC_EXPORT int webrtc_VideoFrameType_value(
    struct webrtc_VideoFrameType* self);
WEBRTC_EXPORT void webrtc_VideoFrameType_vector_push_back_value(
    struct webrtc_VideoFrameType_vector* self,
    int value);

#if defined(__cplusplus)
}
#endif
