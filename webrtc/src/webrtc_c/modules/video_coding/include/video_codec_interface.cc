#include "video_codec_interface.h"

#include <cstdint>
#include <memory>

// WebRTC
#include <api/video/video_codec_type.h>
#include <modules/video_coding/codecs/h264/include/h264_globals.h>
#include <modules/video_coding/include/video_codec_interface.h>

#include "../../../common.h"
#include "../../../common.impl.h"

extern "C" {
WEBRTC_DEFINE_UNIQUE(webrtc_CodecSpecificInfo, webrtc::CodecSpecificInfo);

WEBRTC_EXPORT const int webrtc_H264PacketizationMode_NonInterleaved =
    static_cast<int>(webrtc::H264PacketizationMode::NonInterleaved);
WEBRTC_EXPORT const int webrtc_H264PacketizationMode_SingleNalUnit =
    static_cast<int>(webrtc::H264PacketizationMode::SingleNalUnit);

WEBRTC_EXPORT struct webrtc_CodecSpecificInfo_unique*
webrtc_CodecSpecificInfo_new() {
  auto info = std::make_unique<webrtc::CodecSpecificInfo>();
  return reinterpret_cast<struct webrtc_CodecSpecificInfo_unique*>(
      info.release());
}

WEBRTC_EXPORT int webrtc_CodecSpecificInfo_codec_type(
    struct webrtc_CodecSpecificInfo* self) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  return static_cast<int>(info->codecType);
}

WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_codec_type(
    struct webrtc_CodecSpecificInfo* self,
    int codec_type) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  info->codecType = static_cast<webrtc::VideoCodecType>(codec_type);
}

WEBRTC_EXPORT int webrtc_CodecSpecificInfo_end_of_picture(
    struct webrtc_CodecSpecificInfo* self) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  return info->end_of_picture ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_end_of_picture(
    struct webrtc_CodecSpecificInfo* self,
    int end_of_picture) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  info->end_of_picture = end_of_picture != 0;
}

WEBRTC_EXPORT int webrtc_CodecSpecificInfo_vp8_non_reference(
    struct webrtc_CodecSpecificInfo* self) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  return info->codecSpecific.VP8.nonReference ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_vp8_non_reference(
    struct webrtc_CodecSpecificInfo* self,
    int non_reference) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  info->codecSpecific.VP8.nonReference = non_reference != 0;
}

WEBRTC_EXPORT int webrtc_CodecSpecificInfo_vp8_temporal_idx(
    struct webrtc_CodecSpecificInfo* self) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  return static_cast<int>(info->codecSpecific.VP8.temporalIdx);
}

WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_vp8_temporal_idx(
    struct webrtc_CodecSpecificInfo* self,
    int temporal_idx) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  info->codecSpecific.VP8.temporalIdx = static_cast<uint8_t>(temporal_idx);
}

WEBRTC_EXPORT int webrtc_CodecSpecificInfo_vp8_layer_sync(
    struct webrtc_CodecSpecificInfo* self) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  return info->codecSpecific.VP8.layerSync ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_vp8_layer_sync(
    struct webrtc_CodecSpecificInfo* self,
    int layer_sync) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  info->codecSpecific.VP8.layerSync = layer_sync != 0;
}

WEBRTC_EXPORT int webrtc_CodecSpecificInfo_vp8_key_idx(
    struct webrtc_CodecSpecificInfo* self) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  return static_cast<int>(info->codecSpecific.VP8.keyIdx);
}

WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_vp8_key_idx(
    struct webrtc_CodecSpecificInfo* self,
    int key_idx) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  info->codecSpecific.VP8.keyIdx = static_cast<int8_t>(key_idx);
}

WEBRTC_EXPORT int webrtc_CodecSpecificInfo_vp9_temporal_idx(
    struct webrtc_CodecSpecificInfo* self) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  return static_cast<int>(info->codecSpecific.VP9.temporal_idx);
}

WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_vp9_temporal_idx(
    struct webrtc_CodecSpecificInfo* self,
    int temporal_idx) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  info->codecSpecific.VP9.temporal_idx = static_cast<uint8_t>(temporal_idx);
}

WEBRTC_EXPORT int webrtc_CodecSpecificInfo_vp9_inter_pic_predicted(
    struct webrtc_CodecSpecificInfo* self) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  return info->codecSpecific.VP9.inter_pic_predicted ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_vp9_inter_pic_predicted(
    struct webrtc_CodecSpecificInfo* self,
    int inter_pic_predicted) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  info->codecSpecific.VP9.inter_pic_predicted = inter_pic_predicted != 0;
}

WEBRTC_EXPORT int webrtc_CodecSpecificInfo_vp9_flexible_mode(
    struct webrtc_CodecSpecificInfo* self) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  return info->codecSpecific.VP9.flexible_mode ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_vp9_flexible_mode(
    struct webrtc_CodecSpecificInfo* self,
    int flexible_mode) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  info->codecSpecific.VP9.flexible_mode = flexible_mode != 0;
}

WEBRTC_EXPORT int webrtc_CodecSpecificInfo_vp9_inter_layer_predicted(
    struct webrtc_CodecSpecificInfo* self) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  return info->codecSpecific.VP9.inter_layer_predicted ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_vp9_inter_layer_predicted(
    struct webrtc_CodecSpecificInfo* self,
    int inter_layer_predicted) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  info->codecSpecific.VP9.inter_layer_predicted = inter_layer_predicted != 0;
}

WEBRTC_EXPORT int webrtc_CodecSpecificInfo_vp9_ss_data_available(
    struct webrtc_CodecSpecificInfo* self) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  return info->codecSpecific.VP9.ss_data_available ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_vp9_ss_data_available(
    struct webrtc_CodecSpecificInfo* self,
    int ss_data_available) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  info->codecSpecific.VP9.ss_data_available = ss_data_available != 0;
}

WEBRTC_EXPORT int webrtc_CodecSpecificInfo_vp9_temporal_up_switch(
    struct webrtc_CodecSpecificInfo* self) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  return info->codecSpecific.VP9.temporal_up_switch ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_vp9_temporal_up_switch(
    struct webrtc_CodecSpecificInfo* self,
    int temporal_up_switch) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  info->codecSpecific.VP9.temporal_up_switch = temporal_up_switch != 0;
}

WEBRTC_EXPORT int webrtc_CodecSpecificInfo_vp9_num_spatial_layers(
    struct webrtc_CodecSpecificInfo* self) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  return static_cast<int>(info->codecSpecific.VP9.num_spatial_layers);
}

WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_vp9_num_spatial_layers(
    struct webrtc_CodecSpecificInfo* self,
    int num_spatial_layers) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  info->codecSpecific.VP9.num_spatial_layers =
      static_cast<uint8_t>(num_spatial_layers);
}

WEBRTC_EXPORT int webrtc_CodecSpecificInfo_vp9_first_frame_in_picture(
    struct webrtc_CodecSpecificInfo* self) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  return info->codecSpecific.VP9.first_frame_in_picture ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_vp9_first_frame_in_picture(
    struct webrtc_CodecSpecificInfo* self,
    int first_frame_in_picture) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  info->codecSpecific.VP9.first_frame_in_picture = first_frame_in_picture != 0;
}

WEBRTC_EXPORT int webrtc_CodecSpecificInfo_vp9_spatial_layer_resolution_present(
    struct webrtc_CodecSpecificInfo* self) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  return info->codecSpecific.VP9.spatial_layer_resolution_present ? 1 : 0;
}

WEBRTC_EXPORT void
webrtc_CodecSpecificInfo_set_vp9_spatial_layer_resolution_present(
    struct webrtc_CodecSpecificInfo* self,
    int spatial_layer_resolution_present) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  info->codecSpecific.VP9.spatial_layer_resolution_present =
      spatial_layer_resolution_present != 0;
}

WEBRTC_EXPORT int webrtc_CodecSpecificInfo_h264_packetization_mode(
    struct webrtc_CodecSpecificInfo* self) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  return static_cast<int>(info->codecSpecific.H264.packetization_mode);
}

WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_h264_packetization_mode(
    struct webrtc_CodecSpecificInfo* self,
    int packetization_mode) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  info->codecSpecific.H264.packetization_mode =
      static_cast<webrtc::H264PacketizationMode>(packetization_mode);
}

WEBRTC_EXPORT int webrtc_CodecSpecificInfo_h264_temporal_idx(
    struct webrtc_CodecSpecificInfo* self) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  return static_cast<int>(info->codecSpecific.H264.temporal_idx);
}

WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_h264_temporal_idx(
    struct webrtc_CodecSpecificInfo* self,
    int temporal_idx) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  info->codecSpecific.H264.temporal_idx = static_cast<uint8_t>(temporal_idx);
}

WEBRTC_EXPORT int webrtc_CodecSpecificInfo_h264_base_layer_sync(
    struct webrtc_CodecSpecificInfo* self) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  return info->codecSpecific.H264.base_layer_sync ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_h264_base_layer_sync(
    struct webrtc_CodecSpecificInfo* self,
    int base_layer_sync) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  info->codecSpecific.H264.base_layer_sync = base_layer_sync != 0;
}

WEBRTC_EXPORT int webrtc_CodecSpecificInfo_h264_idr_frame(
    struct webrtc_CodecSpecificInfo* self) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  return info->codecSpecific.H264.idr_frame ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_h264_idr_frame(
    struct webrtc_CodecSpecificInfo* self,
    int idr_frame) {
  auto info = reinterpret_cast<webrtc::CodecSpecificInfo*>(self);
  info->codecSpecific.H264.idr_frame = idr_frame != 0;
}
}
