#pragma once

#include <stdint.h>

#include "../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::CodecSpecificInfo
// -------------------------

WEBRTC_DECLARE_UNIQUE(webrtc_CodecSpecificInfo);

struct webrtc_CodecSpecificInfo_unique* webrtc_CodecSpecificInfo_new();

int webrtc_CodecSpecificInfo_codec_type(struct webrtc_CodecSpecificInfo* self);
void webrtc_CodecSpecificInfo_set_codec_type(struct webrtc_CodecSpecificInfo* self,
                                             int codec_type);

int webrtc_CodecSpecificInfo_end_of_picture(struct webrtc_CodecSpecificInfo* self);
void webrtc_CodecSpecificInfo_set_end_of_picture(struct webrtc_CodecSpecificInfo* self,
                                                 int end_of_picture);

int webrtc_CodecSpecificInfo_vp8_non_reference(struct webrtc_CodecSpecificInfo* self);
void webrtc_CodecSpecificInfo_set_vp8_non_reference(struct webrtc_CodecSpecificInfo* self,
                                                    int non_reference);
int webrtc_CodecSpecificInfo_vp8_temporal_idx(struct webrtc_CodecSpecificInfo* self);
void webrtc_CodecSpecificInfo_set_vp8_temporal_idx(struct webrtc_CodecSpecificInfo* self,
                                                   int temporal_idx);
int webrtc_CodecSpecificInfo_vp8_layer_sync(struct webrtc_CodecSpecificInfo* self);
void webrtc_CodecSpecificInfo_set_vp8_layer_sync(struct webrtc_CodecSpecificInfo* self,
                                                 int layer_sync);
int webrtc_CodecSpecificInfo_vp8_key_idx(struct webrtc_CodecSpecificInfo* self);
void webrtc_CodecSpecificInfo_set_vp8_key_idx(struct webrtc_CodecSpecificInfo* self,
                                              int key_idx);

int webrtc_CodecSpecificInfo_vp9_temporal_idx(struct webrtc_CodecSpecificInfo* self);
void webrtc_CodecSpecificInfo_set_vp9_temporal_idx(struct webrtc_CodecSpecificInfo* self,
                                                   int temporal_idx);
int webrtc_CodecSpecificInfo_vp9_inter_pic_predicted(struct webrtc_CodecSpecificInfo* self);
void webrtc_CodecSpecificInfo_set_vp9_inter_pic_predicted(struct webrtc_CodecSpecificInfo* self,
                                                          int inter_pic_predicted);
int webrtc_CodecSpecificInfo_vp9_flexible_mode(struct webrtc_CodecSpecificInfo* self);
void webrtc_CodecSpecificInfo_set_vp9_flexible_mode(struct webrtc_CodecSpecificInfo* self,
                                                    int flexible_mode);
int webrtc_CodecSpecificInfo_vp9_inter_layer_predicted(struct webrtc_CodecSpecificInfo* self);
void webrtc_CodecSpecificInfo_set_vp9_inter_layer_predicted(
    struct webrtc_CodecSpecificInfo* self,
    int inter_layer_predicted);

extern const int webrtc_H264PacketizationMode_NonInterleaved;
extern const int webrtc_H264PacketizationMode_SingleNalUnit;

int webrtc_CodecSpecificInfo_h264_packetization_mode(struct webrtc_CodecSpecificInfo* self);
void webrtc_CodecSpecificInfo_set_h264_packetization_mode(
    struct webrtc_CodecSpecificInfo* self,
    int packetization_mode);
int webrtc_CodecSpecificInfo_h264_temporal_idx(struct webrtc_CodecSpecificInfo* self);
void webrtc_CodecSpecificInfo_set_h264_temporal_idx(struct webrtc_CodecSpecificInfo* self,
                                                    int temporal_idx);
int webrtc_CodecSpecificInfo_h264_base_layer_sync(struct webrtc_CodecSpecificInfo* self);
void webrtc_CodecSpecificInfo_set_h264_base_layer_sync(struct webrtc_CodecSpecificInfo* self,
                                                       int base_layer_sync);
int webrtc_CodecSpecificInfo_h264_idr_frame(struct webrtc_CodecSpecificInfo* self);
void webrtc_CodecSpecificInfo_set_h264_idr_frame(struct webrtc_CodecSpecificInfo* self,
                                                 int idr_frame);

#if defined(__cplusplus)
}
#endif
