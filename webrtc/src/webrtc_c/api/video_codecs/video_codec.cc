#include "video_codec.h"

#include <stddef.h>
#include <stdint.h>
#include <vector>

// WebRTC
#include <api/video/video_codec_type.h>
#include <api/video/video_frame_type.h>
#include <api/video_codecs/video_codec.h>
#include <api/video_codecs/video_encoder.h>
#include <modules/video_coding/include/video_error_codes.h>

#include "../../common.h"
#include "../../common.impl.h"

extern "C" {
WEBRTC_EXPORT const int webrtc_VideoCodecType_Generic =
    static_cast<int>(webrtc::kVideoCodecGeneric);
WEBRTC_EXPORT const int webrtc_VideoCodecType_VP8 =
    static_cast<int>(webrtc::kVideoCodecVP8);
WEBRTC_EXPORT const int webrtc_VideoCodecType_VP9 =
    static_cast<int>(webrtc::kVideoCodecVP9);
WEBRTC_EXPORT const int webrtc_VideoCodecType_AV1 =
    static_cast<int>(webrtc::kVideoCodecAV1);
WEBRTC_EXPORT const int webrtc_VideoCodecType_H264 =
    static_cast<int>(webrtc::kVideoCodecH264);
WEBRTC_EXPORT const int webrtc_VideoCodecType_H265 =
    static_cast<int>(webrtc::kVideoCodecH265);

WEBRTC_EXPORT const int webrtc_VideoCodecStatus_TargetBitrateOvershoot =
    WEBRTC_VIDEO_CODEC_TARGET_BITRATE_OVERSHOOT;
WEBRTC_EXPORT const int webrtc_VideoCodecStatus_OkRequestKeyframe =
    WEBRTC_VIDEO_CODEC_OK_REQUEST_KEYFRAME;
WEBRTC_EXPORT const int webrtc_VideoCodecStatus_NoOutput =
    WEBRTC_VIDEO_CODEC_NO_OUTPUT;
WEBRTC_EXPORT const int webrtc_VideoCodecStatus_Ok = WEBRTC_VIDEO_CODEC_OK;
WEBRTC_EXPORT const int webrtc_VideoCodecStatus_Error =
    WEBRTC_VIDEO_CODEC_ERROR;
WEBRTC_EXPORT const int webrtc_VideoCodecStatus_Memory =
    WEBRTC_VIDEO_CODEC_MEMORY;
WEBRTC_EXPORT const int webrtc_VideoCodecStatus_ErrParameter =
    WEBRTC_VIDEO_CODEC_ERR_PARAMETER;
WEBRTC_EXPORT const int webrtc_VideoCodecStatus_Timeout =
    WEBRTC_VIDEO_CODEC_TIMEOUT;
WEBRTC_EXPORT const int webrtc_VideoCodecStatus_Uninitialized =
    WEBRTC_VIDEO_CODEC_UNINITIALIZED;
WEBRTC_EXPORT const int webrtc_VideoCodecStatus_FallbackSoftware =
    WEBRTC_VIDEO_CODEC_FALLBACK_SOFTWARE;
WEBRTC_EXPORT const int
    webrtc_VideoCodecStatus_ErrSimulcastParametersNotSupported =
        WEBRTC_VIDEO_CODEC_ERR_SIMULCAST_PARAMETERS_NOT_SUPPORTED;
WEBRTC_EXPORT const int webrtc_VideoCodecStatus_EncoderFailure =
    WEBRTC_VIDEO_CODEC_ENCODER_FAILURE;

WEBRTC_EXPORT const int webrtc_VideoFrameType_Empty =
    static_cast<int>(webrtc::VideoFrameType::kEmptyFrame);
WEBRTC_EXPORT const int webrtc_VideoFrameType_Key =
    static_cast<int>(webrtc::VideoFrameType::kVideoFrameKey);
WEBRTC_EXPORT const int webrtc_VideoFrameType_Delta =
    static_cast<int>(webrtc::VideoFrameType::kVideoFrameDelta);

WEBRTC_EXPORT struct webrtc_VideoCodec* webrtc_VideoCodec_new(void) {
  auto codec = new webrtc::VideoCodec();
  return reinterpret_cast<struct webrtc_VideoCodec*>(codec);
}

WEBRTC_EXPORT void webrtc_VideoCodec_delete(struct webrtc_VideoCodec* self) {
  delete reinterpret_cast<webrtc::VideoCodec*>(self);
}

WEBRTC_EXPORT int webrtc_VideoCodec_codec_type(struct webrtc_VideoCodec* self) {
  auto codec = reinterpret_cast<webrtc::VideoCodec*>(self);
  return static_cast<int>(codec->codecType);
}

WEBRTC_EXPORT void webrtc_VideoCodec_set_codec_type(
    struct webrtc_VideoCodec* self,
    int codec_type) {
  auto codec = reinterpret_cast<webrtc::VideoCodec*>(self);
  codec->codecType = static_cast<webrtc::VideoCodecType>(codec_type);
}

WEBRTC_EXPORT int webrtc_VideoCodec_width(struct webrtc_VideoCodec* self) {
  auto codec = reinterpret_cast<webrtc::VideoCodec*>(self);
  return static_cast<int>(codec->width);
}

WEBRTC_EXPORT void webrtc_VideoCodec_set_width(struct webrtc_VideoCodec* self,
                                               int width) {
  auto codec = reinterpret_cast<webrtc::VideoCodec*>(self);
  codec->width = static_cast<uint16_t>(width);
}

WEBRTC_EXPORT int webrtc_VideoCodec_height(struct webrtc_VideoCodec* self) {
  auto codec = reinterpret_cast<webrtc::VideoCodec*>(self);
  return static_cast<int>(codec->height);
}

WEBRTC_EXPORT void webrtc_VideoCodec_set_height(struct webrtc_VideoCodec* self,
                                                int height) {
  auto codec = reinterpret_cast<webrtc::VideoCodec*>(self);
  codec->height = static_cast<uint16_t>(height);
}

WEBRTC_EXPORT unsigned int webrtc_VideoCodec_start_bitrate_kbps(
    struct webrtc_VideoCodec* self) {
  auto codec = reinterpret_cast<webrtc::VideoCodec*>(self);
  return codec->startBitrate;
}

WEBRTC_EXPORT void webrtc_VideoCodec_set_start_bitrate_kbps(
    struct webrtc_VideoCodec* self,
    unsigned int start_bitrate) {
  auto codec = reinterpret_cast<webrtc::VideoCodec*>(self);
  codec->startBitrate = start_bitrate;
}

WEBRTC_EXPORT unsigned int webrtc_VideoCodec_max_bitrate_kbps(
    struct webrtc_VideoCodec* self) {
  auto codec = reinterpret_cast<webrtc::VideoCodec*>(self);
  return codec->maxBitrate;
}

WEBRTC_EXPORT void webrtc_VideoCodec_set_max_bitrate_kbps(
    struct webrtc_VideoCodec* self,
    unsigned int max_bitrate) {
  auto codec = reinterpret_cast<webrtc::VideoCodec*>(self);
  codec->maxBitrate = max_bitrate;
}

WEBRTC_EXPORT unsigned int webrtc_VideoCodec_min_bitrate_kbps(
    struct webrtc_VideoCodec* self) {
  auto codec = reinterpret_cast<webrtc::VideoCodec*>(self);
  return codec->minBitrate;
}

WEBRTC_EXPORT void webrtc_VideoCodec_set_min_bitrate_kbps(
    struct webrtc_VideoCodec* self,
    unsigned int min_bitrate) {
  auto codec = reinterpret_cast<webrtc::VideoCodec*>(self);
  codec->minBitrate = min_bitrate;
}

WEBRTC_EXPORT uint32_t
webrtc_VideoCodec_max_framerate(struct webrtc_VideoCodec* self) {
  auto codec = reinterpret_cast<webrtc::VideoCodec*>(self);
  return codec->maxFramerate;
}

WEBRTC_EXPORT void webrtc_VideoCodec_set_max_framerate(
    struct webrtc_VideoCodec* self,
    uint32_t max_framerate) {
  auto codec = reinterpret_cast<webrtc::VideoCodec*>(self);
  codec->maxFramerate = max_framerate;
}

WEBRTC_EXPORT struct webrtc_VideoEncoder_Settings*
webrtc_VideoEncoder_Settings_new(int number_of_cores, size_t max_payload_size) {
  auto settings = new webrtc::VideoEncoder::Settings(
      webrtc::VideoEncoder::Capabilities(false), number_of_cores,
      max_payload_size);
  return reinterpret_cast<struct webrtc_VideoEncoder_Settings*>(settings);
}

WEBRTC_EXPORT void webrtc_VideoEncoder_Settings_delete(
    struct webrtc_VideoEncoder_Settings* self) {
  delete reinterpret_cast<webrtc::VideoEncoder::Settings*>(self);
}

WEBRTC_EXPORT int webrtc_VideoEncoder_Settings_number_of_cores(
    struct webrtc_VideoEncoder_Settings* self) {
  auto settings = reinterpret_cast<webrtc::VideoEncoder::Settings*>(self);
  return settings->number_of_cores;
}

WEBRTC_EXPORT size_t webrtc_VideoEncoder_Settings_max_payload_size(
    struct webrtc_VideoEncoder_Settings* self) {
  auto settings = reinterpret_cast<webrtc::VideoEncoder::Settings*>(self);
  return settings->max_payload_size;
}

WEBRTC_EXPORT int webrtc_VideoEncoder_Settings_loss_notification(
    struct webrtc_VideoEncoder_Settings* self) {
  auto settings = reinterpret_cast<webrtc::VideoEncoder::Settings*>(self);
  return settings->capabilities.loss_notification ? 1 : 0;
}

WEBRTC_EXPORT int webrtc_VideoEncoder_Settings_has_encoder_thread_limit(
    struct webrtc_VideoEncoder_Settings* self) {
  auto settings = reinterpret_cast<webrtc::VideoEncoder::Settings*>(self);
  return settings->encoder_thread_limit.has_value() ? 1 : 0;
}

WEBRTC_EXPORT int webrtc_VideoEncoder_Settings_encoder_thread_limit(
    struct webrtc_VideoEncoder_Settings* self) {
  auto settings = reinterpret_cast<webrtc::VideoEncoder::Settings*>(self);
  return settings->encoder_thread_limit.value_or(0);
}

WEBRTC_EXPORT double webrtc_VideoEncoder_RateControlParameters_framerate_fps(
    struct webrtc_VideoEncoder_RateControlParameters* self) {
  auto parameters =
      reinterpret_cast<webrtc::VideoEncoder::RateControlParameters*>(self);
  return parameters->framerate_fps;
}

WEBRTC_EXPORT uint32_t
webrtc_VideoEncoder_RateControlParameters_target_bitrate_sum_bps(
    struct webrtc_VideoEncoder_RateControlParameters* self) {
  auto parameters =
      reinterpret_cast<webrtc::VideoEncoder::RateControlParameters*>(self);
  return parameters->target_bitrate.get_sum_bps();
}

WEBRTC_EXPORT uint32_t
webrtc_VideoEncoder_RateControlParameters_bitrate_sum_bps(
    struct webrtc_VideoEncoder_RateControlParameters* self) {
  auto parameters =
      reinterpret_cast<webrtc::VideoEncoder::RateControlParameters*>(self);
  return parameters->bitrate.get_sum_bps();
}

WEBRTC_EXPORT int64_t
webrtc_VideoEncoder_RateControlParameters_bandwidth_allocation_bps(
    struct webrtc_VideoEncoder_RateControlParameters* self) {
  auto parameters =
      reinterpret_cast<webrtc::VideoEncoder::RateControlParameters*>(self);
  return parameters->bandwidth_allocation.bps();
}

WEBRTC_DEFINE_VECTOR(webrtc_VideoFrameType, webrtc::VideoFrameType);

WEBRTC_EXPORT int webrtc_VideoFrameType_value(
    struct webrtc_VideoFrameType* self) {
  auto value = reinterpret_cast<webrtc::VideoFrameType*>(self);
  return static_cast<int>(*value);
}

WEBRTC_EXPORT void webrtc_VideoFrameType_vector_push_back_value(
    struct webrtc_VideoFrameType_vector* self,
    int value) {
  auto vec = reinterpret_cast<std::vector<webrtc::VideoFrameType>*>(self);
  vec->push_back(static_cast<webrtc::VideoFrameType>(value));
}
}
