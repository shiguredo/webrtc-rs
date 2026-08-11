#pragma once

#include <stddef.h>
#include <stdint.h>

#include "../../../../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::GofInfoVP9
// -------------------------

struct webrtc_GofInfoVP9;

WEBRTC_EXPORT struct webrtc_GofInfoVP9* webrtc_GofInfoVP9_new();
WEBRTC_EXPORT void webrtc_GofInfoVP9_delete(struct webrtc_GofInfoVP9* self);
WEBRTC_EXPORT struct webrtc_GofInfoVP9* webrtc_GofInfoVP9_copy(
    const struct webrtc_GofInfoVP9* self);

WEBRTC_EXPORT size_t
webrtc_GofInfoVP9_get_num_frames_in_gof(struct webrtc_GofInfoVP9* self);
WEBRTC_EXPORT void webrtc_GofInfoVP9_set_num_frames_in_gof(
    struct webrtc_GofInfoVP9* self,
    size_t value);
// 各配列は index を指定してアクセスする。
WEBRTC_EXPORT uint8_t
webrtc_GofInfoVP9_get_temporal_idx(struct webrtc_GofInfoVP9* self,
                                   size_t index);
WEBRTC_EXPORT void webrtc_GofInfoVP9_set_temporal_idx(
    struct webrtc_GofInfoVP9* self,
    size_t index,
    uint8_t value);
WEBRTC_EXPORT int webrtc_GofInfoVP9_get_temporal_up_switch(
    struct webrtc_GofInfoVP9* self,
    size_t index);
WEBRTC_EXPORT void webrtc_GofInfoVP9_set_temporal_up_switch(
    struct webrtc_GofInfoVP9* self,
    size_t index,
    int value);
WEBRTC_EXPORT uint8_t
webrtc_GofInfoVP9_get_num_ref_pics(struct webrtc_GofInfoVP9* self,
                                   size_t index);
WEBRTC_EXPORT void webrtc_GofInfoVP9_set_num_ref_pics(
    struct webrtc_GofInfoVP9* self,
    size_t index,
    uint8_t value);
WEBRTC_EXPORT uint8_t
webrtc_GofInfoVP9_get_pid_diff(struct webrtc_GofInfoVP9* self,
                               size_t index,
                               size_t ref_index);
WEBRTC_EXPORT void webrtc_GofInfoVP9_set_pid_diff(
    struct webrtc_GofInfoVP9* self,
    size_t index,
    size_t ref_index,
    uint8_t value);
WEBRTC_EXPORT uint16_t
webrtc_GofInfoVP9_get_pid_start(struct webrtc_GofInfoVP9* self);
WEBRTC_EXPORT void webrtc_GofInfoVP9_set_pid_start(
    struct webrtc_GofInfoVP9* self,
    uint16_t value);

// -------------------------
// webrtc::RTPVideoHeaderVP9
// -------------------------

struct webrtc_RTPVideoHeaderVP9;

WEBRTC_EXPORT struct webrtc_RTPVideoHeaderVP9* webrtc_RTPVideoHeaderVP9_new();
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_delete(
    struct webrtc_RTPVideoHeaderVP9* self);
WEBRTC_EXPORT struct webrtc_RTPVideoHeaderVP9* webrtc_RTPVideoHeaderVP9_copy(
    const struct webrtc_RTPVideoHeaderVP9* self);

WEBRTC_EXPORT int webrtc_RTPVideoHeaderVP9_get_inter_pic_predicted(
    struct webrtc_RTPVideoHeaderVP9* self);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_inter_pic_predicted(
    struct webrtc_RTPVideoHeaderVP9* self,
    int value);
WEBRTC_EXPORT int webrtc_RTPVideoHeaderVP9_get_flexible_mode(
    struct webrtc_RTPVideoHeaderVP9* self);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_flexible_mode(
    struct webrtc_RTPVideoHeaderVP9* self,
    int value);
WEBRTC_EXPORT int webrtc_RTPVideoHeaderVP9_get_beginning_of_frame(
    struct webrtc_RTPVideoHeaderVP9* self);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_beginning_of_frame(
    struct webrtc_RTPVideoHeaderVP9* self,
    int value);
WEBRTC_EXPORT int webrtc_RTPVideoHeaderVP9_get_end_of_frame(
    struct webrtc_RTPVideoHeaderVP9* self);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_end_of_frame(
    struct webrtc_RTPVideoHeaderVP9* self,
    int value);
WEBRTC_EXPORT int webrtc_RTPVideoHeaderVP9_get_ss_data_available(
    struct webrtc_RTPVideoHeaderVP9* self);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_ss_data_available(
    struct webrtc_RTPVideoHeaderVP9* self,
    int value);
WEBRTC_EXPORT int webrtc_RTPVideoHeaderVP9_get_non_ref_for_inter_layer_pred(
    struct webrtc_RTPVideoHeaderVP9* self);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_non_ref_for_inter_layer_pred(
    struct webrtc_RTPVideoHeaderVP9* self,
    int value);
WEBRTC_EXPORT int16_t
webrtc_RTPVideoHeaderVP9_get_picture_id(struct webrtc_RTPVideoHeaderVP9* self);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_picture_id(
    struct webrtc_RTPVideoHeaderVP9* self,
    int16_t value);
WEBRTC_EXPORT int16_t webrtc_RTPVideoHeaderVP9_get_max_picture_id(
    struct webrtc_RTPVideoHeaderVP9* self);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_max_picture_id(
    struct webrtc_RTPVideoHeaderVP9* self,
    int16_t value);
WEBRTC_EXPORT int16_t
webrtc_RTPVideoHeaderVP9_get_tl0_pic_idx(struct webrtc_RTPVideoHeaderVP9* self);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_tl0_pic_idx(
    struct webrtc_RTPVideoHeaderVP9* self,
    int16_t value);
WEBRTC_EXPORT uint8_t webrtc_RTPVideoHeaderVP9_get_temporal_idx(
    struct webrtc_RTPVideoHeaderVP9* self);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_temporal_idx(
    struct webrtc_RTPVideoHeaderVP9* self,
    uint8_t value);
WEBRTC_EXPORT uint8_t
webrtc_RTPVideoHeaderVP9_get_spatial_idx(struct webrtc_RTPVideoHeaderVP9* self);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_spatial_idx(
    struct webrtc_RTPVideoHeaderVP9* self,
    uint8_t value);
WEBRTC_EXPORT int webrtc_RTPVideoHeaderVP9_get_temporal_up_switch(
    struct webrtc_RTPVideoHeaderVP9* self);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_temporal_up_switch(
    struct webrtc_RTPVideoHeaderVP9* self,
    int value);
WEBRTC_EXPORT int webrtc_RTPVideoHeaderVP9_get_inter_layer_predicted(
    struct webrtc_RTPVideoHeaderVP9* self);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_inter_layer_predicted(
    struct webrtc_RTPVideoHeaderVP9* self,
    int value);
WEBRTC_EXPORT uint8_t
webrtc_RTPVideoHeaderVP9_get_gof_idx(struct webrtc_RTPVideoHeaderVP9* self);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_gof_idx(
    struct webrtc_RTPVideoHeaderVP9* self,
    uint8_t value);
WEBRTC_EXPORT uint8_t webrtc_RTPVideoHeaderVP9_get_num_ref_pics(
    struct webrtc_RTPVideoHeaderVP9* self);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_num_ref_pics(
    struct webrtc_RTPVideoHeaderVP9* self,
    uint8_t value);
// 各配列は index を指定してアクセスする。
WEBRTC_EXPORT uint8_t
webrtc_RTPVideoHeaderVP9_get_pid_diff(struct webrtc_RTPVideoHeaderVP9* self,
                                      size_t index);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_pid_diff(
    struct webrtc_RTPVideoHeaderVP9* self,
    size_t index,
    uint8_t value);
WEBRTC_EXPORT int16_t webrtc_RTPVideoHeaderVP9_get_ref_picture_id(
    struct webrtc_RTPVideoHeaderVP9* self,
    size_t index);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_ref_picture_id(
    struct webrtc_RTPVideoHeaderVP9* self,
    size_t index,
    int16_t value);
WEBRTC_EXPORT size_t webrtc_RTPVideoHeaderVP9_get_num_spatial_layers(
    struct webrtc_RTPVideoHeaderVP9* self);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_num_spatial_layers(
    struct webrtc_RTPVideoHeaderVP9* self,
    size_t value);
WEBRTC_EXPORT size_t webrtc_RTPVideoHeaderVP9_get_first_active_layer(
    struct webrtc_RTPVideoHeaderVP9* self);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_first_active_layer(
    struct webrtc_RTPVideoHeaderVP9* self,
    size_t value);
WEBRTC_EXPORT int webrtc_RTPVideoHeaderVP9_get_spatial_layer_resolution_present(
    struct webrtc_RTPVideoHeaderVP9* self);
WEBRTC_EXPORT void
webrtc_RTPVideoHeaderVP9_set_spatial_layer_resolution_present(
    struct webrtc_RTPVideoHeaderVP9* self,
    int value);
WEBRTC_EXPORT uint16_t
webrtc_RTPVideoHeaderVP9_get_width(struct webrtc_RTPVideoHeaderVP9* self,
                                   size_t index);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_width(
    struct webrtc_RTPVideoHeaderVP9* self,
    size_t index,
    uint16_t value);
WEBRTC_EXPORT uint16_t
webrtc_RTPVideoHeaderVP9_get_height(struct webrtc_RTPVideoHeaderVP9* self,
                                    size_t index);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_height(
    struct webrtc_RTPVideoHeaderVP9* self,
    size_t index,
    uint16_t value);
// gof は借用ポインタで取得し、set はコピーする。
WEBRTC_EXPORT struct webrtc_GofInfoVP9* webrtc_RTPVideoHeaderVP9_get_gof(
    struct webrtc_RTPVideoHeaderVP9* self);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_gof(
    struct webrtc_RTPVideoHeaderVP9* self,
    const struct webrtc_GofInfoVP9* value);
WEBRTC_EXPORT int webrtc_RTPVideoHeaderVP9_get_end_of_picture(
    struct webrtc_RTPVideoHeaderVP9* self);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_end_of_picture(
    struct webrtc_RTPVideoHeaderVP9* self,
    int value);

#if defined(__cplusplus)
}
#endif
