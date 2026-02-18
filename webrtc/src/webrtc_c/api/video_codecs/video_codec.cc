#include "video_codec.h"

#include <stddef.h>
#include <stdint.h>
#include <vector>

// WebRTC
#include <api/video/video_codec_type.h>
#include <api/video/video_frame_type.h>
#include <api/video_codecs/video_codec.h>
#include <api/video_codecs/video_encoder.h>

#include "../../common.impl.h"

extern "C" {
const int webrtc_VideoCodecType_Generic =
    static_cast<int>(webrtc::kVideoCodecGeneric);
const int webrtc_VideoCodecType_VP8 = static_cast<int>(webrtc::kVideoCodecVP8);
const int webrtc_VideoCodecType_VP9 = static_cast<int>(webrtc::kVideoCodecVP9);
const int webrtc_VideoCodecType_AV1 = static_cast<int>(webrtc::kVideoCodecAV1);
const int webrtc_VideoCodecType_H264 =
    static_cast<int>(webrtc::kVideoCodecH264);
const int webrtc_VideoCodecType_H265 =
    static_cast<int>(webrtc::kVideoCodecH265);

const int webrtc_VideoFrameType_Empty =
    static_cast<int>(webrtc::VideoFrameType::kEmptyFrame);
const int webrtc_VideoFrameType_Key =
    static_cast<int>(webrtc::VideoFrameType::kVideoFrameKey);
const int webrtc_VideoFrameType_Delta =
    static_cast<int>(webrtc::VideoFrameType::kVideoFrameDelta);

int webrtc_VideoCodec_codec_type(struct webrtc_VideoCodec* self) {
  auto codec = reinterpret_cast<webrtc::VideoCodec*>(self);
  return static_cast<int>(codec->codecType);
}

int webrtc_VideoCodec_width(struct webrtc_VideoCodec* self) {
  auto codec = reinterpret_cast<webrtc::VideoCodec*>(self);
  return static_cast<int>(codec->width);
}

int webrtc_VideoCodec_height(struct webrtc_VideoCodec* self) {
  auto codec = reinterpret_cast<webrtc::VideoCodec*>(self);
  return static_cast<int>(codec->height);
}

unsigned int webrtc_VideoCodec_start_bitrate_kbps(
    struct webrtc_VideoCodec* self) {
  auto codec = reinterpret_cast<webrtc::VideoCodec*>(self);
  return codec->startBitrate;
}

unsigned int webrtc_VideoCodec_max_bitrate_kbps(
    struct webrtc_VideoCodec* self) {
  auto codec = reinterpret_cast<webrtc::VideoCodec*>(self);
  return codec->maxBitrate;
}

unsigned int webrtc_VideoCodec_min_bitrate_kbps(
    struct webrtc_VideoCodec* self) {
  auto codec = reinterpret_cast<webrtc::VideoCodec*>(self);
  return codec->minBitrate;
}

uint32_t webrtc_VideoCodec_max_framerate(struct webrtc_VideoCodec* self) {
  auto codec = reinterpret_cast<webrtc::VideoCodec*>(self);
  return codec->maxFramerate;
}

int webrtc_VideoEncoder_Settings_number_of_cores(
    struct webrtc_VideoEncoder_Settings* self) {
  auto settings = reinterpret_cast<webrtc::VideoEncoder::Settings*>(self);
  return settings->number_of_cores;
}

size_t webrtc_VideoEncoder_Settings_max_payload_size(
    struct webrtc_VideoEncoder_Settings* self) {
  auto settings = reinterpret_cast<webrtc::VideoEncoder::Settings*>(self);
  return settings->max_payload_size;
}

int webrtc_VideoEncoder_Settings_loss_notification(
    struct webrtc_VideoEncoder_Settings* self) {
  auto settings = reinterpret_cast<webrtc::VideoEncoder::Settings*>(self);
  return settings->capabilities.loss_notification ? 1 : 0;
}

int webrtc_VideoEncoder_Settings_has_encoder_thread_limit(
    struct webrtc_VideoEncoder_Settings* self) {
  auto settings = reinterpret_cast<webrtc::VideoEncoder::Settings*>(self);
  return settings->encoder_thread_limit.has_value() ? 1 : 0;
}

int webrtc_VideoEncoder_Settings_encoder_thread_limit(
    struct webrtc_VideoEncoder_Settings* self) {
  auto settings = reinterpret_cast<webrtc::VideoEncoder::Settings*>(self);
  return settings->encoder_thread_limit.value_or(0);
}

double webrtc_VideoEncoder_RateControlParameters_framerate_fps(
    struct webrtc_VideoEncoder_RateControlParameters* self) {
  auto parameters =
      reinterpret_cast<webrtc::VideoEncoder::RateControlParameters*>(self);
  return parameters->framerate_fps;
}

uint32_t webrtc_VideoEncoder_RateControlParameters_target_bitrate_sum_bps(
    struct webrtc_VideoEncoder_RateControlParameters* self) {
  auto parameters =
      reinterpret_cast<webrtc::VideoEncoder::RateControlParameters*>(self);
  return parameters->target_bitrate.get_sum_bps();
}

uint32_t webrtc_VideoEncoder_RateControlParameters_bitrate_sum_bps(
    struct webrtc_VideoEncoder_RateControlParameters* self) {
  auto parameters =
      reinterpret_cast<webrtc::VideoEncoder::RateControlParameters*>(self);
  return parameters->bitrate.get_sum_bps();
}

int64_t webrtc_VideoEncoder_RateControlParameters_bandwidth_allocation_bps(
    struct webrtc_VideoEncoder_RateControlParameters* self) {
  auto parameters =
      reinterpret_cast<webrtc::VideoEncoder::RateControlParameters*>(self);
  return parameters->bandwidth_allocation.bps();
}

WEBRTC_DEFINE_VECTOR(webrtc_VideoFrameType, webrtc::VideoFrameType);

int webrtc_VideoFrameType_value(struct webrtc_VideoFrameType* self) {
  auto value = reinterpret_cast<webrtc::VideoFrameType*>(self);
  return static_cast<int>(*value);
}

void webrtc_VideoFrameType_vector_push_back_value(
    struct webrtc_VideoFrameType_vector* self,
    int value) {
  auto vec = reinterpret_cast<std::vector<webrtc::VideoFrameType>*>(self);
  vec->push_back(static_cast<webrtc::VideoFrameType>(value));
}
}
