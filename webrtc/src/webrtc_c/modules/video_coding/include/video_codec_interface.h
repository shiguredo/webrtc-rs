#pragma once

#include <stdint.h>

#include "../../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::CodecSpecificInfo
// -------------------------

WEBRTC_DECLARE_UNIQUE(webrtc_CodecSpecificInfo);

WEBRTC_EXPORT struct webrtc_CodecSpecificInfo_unique*
webrtc_CodecSpecificInfo_new();

WEBRTC_EXPORT int webrtc_CodecSpecificInfo_codec_type(
    struct webrtc_CodecSpecificInfo* self);
WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_codec_type(
    struct webrtc_CodecSpecificInfo* self,
    int codec_type);

WEBRTC_EXPORT int webrtc_CodecSpecificInfo_end_of_picture(
    struct webrtc_CodecSpecificInfo* self);
WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_end_of_picture(
    struct webrtc_CodecSpecificInfo* self,
    int end_of_picture);

WEBRTC_EXPORT int webrtc_CodecSpecificInfo_vp8_non_reference(
    struct webrtc_CodecSpecificInfo* self);
WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_vp8_non_reference(
    struct webrtc_CodecSpecificInfo* self,
    int non_reference);
WEBRTC_EXPORT int webrtc_CodecSpecificInfo_vp8_temporal_idx(
    struct webrtc_CodecSpecificInfo* self);
WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_vp8_temporal_idx(
    struct webrtc_CodecSpecificInfo* self,
    int temporal_idx);
WEBRTC_EXPORT int webrtc_CodecSpecificInfo_vp8_layer_sync(
    struct webrtc_CodecSpecificInfo* self);
WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_vp8_layer_sync(
    struct webrtc_CodecSpecificInfo* self,
    int layer_sync);
WEBRTC_EXPORT int webrtc_CodecSpecificInfo_vp8_key_idx(
    struct webrtc_CodecSpecificInfo* self);
WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_vp8_key_idx(
    struct webrtc_CodecSpecificInfo* self,
    int key_idx);

WEBRTC_EXPORT int webrtc_CodecSpecificInfo_vp9_temporal_idx(
    struct webrtc_CodecSpecificInfo* self);
WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_vp9_temporal_idx(
    struct webrtc_CodecSpecificInfo* self,
    int temporal_idx);
WEBRTC_EXPORT int webrtc_CodecSpecificInfo_vp9_inter_pic_predicted(
    struct webrtc_CodecSpecificInfo* self);
WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_vp9_inter_pic_predicted(
    struct webrtc_CodecSpecificInfo* self,
    int inter_pic_predicted);
WEBRTC_EXPORT int webrtc_CodecSpecificInfo_vp9_flexible_mode(
    struct webrtc_CodecSpecificInfo* self);
WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_vp9_flexible_mode(
    struct webrtc_CodecSpecificInfo* self,
    int flexible_mode);
WEBRTC_EXPORT int webrtc_CodecSpecificInfo_vp9_inter_layer_predicted(
    struct webrtc_CodecSpecificInfo* self);
WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_vp9_inter_layer_predicted(
    struct webrtc_CodecSpecificInfo* self,
    int inter_layer_predicted);
WEBRTC_EXPORT int webrtc_CodecSpecificInfo_vp9_ss_data_available(
    struct webrtc_CodecSpecificInfo* self);
WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_vp9_ss_data_available(
    struct webrtc_CodecSpecificInfo* self,
    int ss_data_available);
WEBRTC_EXPORT int webrtc_CodecSpecificInfo_vp9_temporal_up_switch(
    struct webrtc_CodecSpecificInfo* self);
WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_vp9_temporal_up_switch(
    struct webrtc_CodecSpecificInfo* self,
    int temporal_up_switch);
WEBRTC_EXPORT int webrtc_CodecSpecificInfo_vp9_num_spatial_layers(
    struct webrtc_CodecSpecificInfo* self);
WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_vp9_num_spatial_layers(
    struct webrtc_CodecSpecificInfo* self,
    int num_spatial_layers);
WEBRTC_EXPORT int webrtc_CodecSpecificInfo_vp9_first_frame_in_picture(
    struct webrtc_CodecSpecificInfo* self);
WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_vp9_first_frame_in_picture(
    struct webrtc_CodecSpecificInfo* self,
    int first_frame_in_picture);
WEBRTC_EXPORT int webrtc_CodecSpecificInfo_vp9_spatial_layer_resolution_present(
    struct webrtc_CodecSpecificInfo* self);
WEBRTC_EXPORT void
webrtc_CodecSpecificInfo_set_vp9_spatial_layer_resolution_present(
    struct webrtc_CodecSpecificInfo* self,
    int spatial_layer_resolution_present);

WEBRTC_EXPORT extern const int webrtc_H264PacketizationMode_NonInterleaved;
WEBRTC_EXPORT extern const int webrtc_H264PacketizationMode_SingleNalUnit;

WEBRTC_EXPORT int webrtc_CodecSpecificInfo_h264_packetization_mode(
    struct webrtc_CodecSpecificInfo* self);
WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_h264_packetization_mode(
    struct webrtc_CodecSpecificInfo* self,
    int packetization_mode);
WEBRTC_EXPORT int webrtc_CodecSpecificInfo_h264_temporal_idx(
    struct webrtc_CodecSpecificInfo* self);
WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_h264_temporal_idx(
    struct webrtc_CodecSpecificInfo* self,
    int temporal_idx);
WEBRTC_EXPORT int webrtc_CodecSpecificInfo_h264_base_layer_sync(
    struct webrtc_CodecSpecificInfo* self);
WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_h264_base_layer_sync(
    struct webrtc_CodecSpecificInfo* self,
    int base_layer_sync);
WEBRTC_EXPORT int webrtc_CodecSpecificInfo_h264_idr_frame(
    struct webrtc_CodecSpecificInfo* self);
WEBRTC_EXPORT void webrtc_CodecSpecificInfo_set_h264_idr_frame(
    struct webrtc_CodecSpecificInfo* self,
    int idr_frame);

#if defined(__cplusplus)
}
#endif
