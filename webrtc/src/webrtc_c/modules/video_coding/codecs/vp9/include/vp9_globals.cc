#include "vp9_globals.h"

#include <assert.h>
#include <memory>

#include <modules/video_coding/codecs/vp9/include/vp9_globals.h>

#include "../../../../../common.h"

extern "C" {
WEBRTC_EXPORT struct webrtc_GofInfoVP9* webrtc_GofInfoVP9_new() {
  auto gof = std::make_unique<webrtc::GofInfoVP9>();
  return reinterpret_cast<struct webrtc_GofInfoVP9*>(gof.release());
}

WEBRTC_EXPORT void webrtc_GofInfoVP9_delete(struct webrtc_GofInfoVP9* self) {
  auto gof = reinterpret_cast<webrtc::GofInfoVP9*>(self);
  delete gof;
}

WEBRTC_EXPORT struct webrtc_GofInfoVP9* webrtc_GofInfoVP9_copy(
    const struct webrtc_GofInfoVP9* self) {
  auto gof = reinterpret_cast<const webrtc::GofInfoVP9*>(self);
  auto copy = std::make_unique<webrtc::GofInfoVP9>(*gof);
  return reinterpret_cast<struct webrtc_GofInfoVP9*>(copy.release());
}

WEBRTC_EXPORT size_t
webrtc_GofInfoVP9_get_num_frames_in_gof(struct webrtc_GofInfoVP9* self) {
  auto gof = reinterpret_cast<webrtc::GofInfoVP9*>(self);
  return gof->num_frames_in_gof;
}

WEBRTC_EXPORT void webrtc_GofInfoVP9_set_num_frames_in_gof(
    struct webrtc_GofInfoVP9* self,
    size_t value) {
  auto gof = reinterpret_cast<webrtc::GofInfoVP9*>(self);
  gof->num_frames_in_gof = value;
}

WEBRTC_EXPORT uint8_t
webrtc_GofInfoVP9_get_temporal_idx(struct webrtc_GofInfoVP9* self,
                                   size_t index) {
  auto gof = reinterpret_cast<webrtc::GofInfoVP9*>(self);
  return gof->temporal_idx[index];
}

WEBRTC_EXPORT void webrtc_GofInfoVP9_set_temporal_idx(
    struct webrtc_GofInfoVP9* self,
    size_t index,
    uint8_t value) {
  auto gof = reinterpret_cast<webrtc::GofInfoVP9*>(self);
  gof->temporal_idx[index] = value;
}

WEBRTC_EXPORT int webrtc_GofInfoVP9_get_temporal_up_switch(
    struct webrtc_GofInfoVP9* self,
    size_t index) {
  auto gof = reinterpret_cast<webrtc::GofInfoVP9*>(self);
  return gof->temporal_up_switch[index] ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_GofInfoVP9_set_temporal_up_switch(
    struct webrtc_GofInfoVP9* self,
    size_t index,
    int value) {
  auto gof = reinterpret_cast<webrtc::GofInfoVP9*>(self);
  gof->temporal_up_switch[index] = value != 0;
}

WEBRTC_EXPORT uint8_t
webrtc_GofInfoVP9_get_num_ref_pics(struct webrtc_GofInfoVP9* self,
                                   size_t index) {
  auto gof = reinterpret_cast<webrtc::GofInfoVP9*>(self);
  return gof->num_ref_pics[index];
}

WEBRTC_EXPORT void webrtc_GofInfoVP9_set_num_ref_pics(
    struct webrtc_GofInfoVP9* self,
    size_t index,
    uint8_t value) {
  auto gof = reinterpret_cast<webrtc::GofInfoVP9*>(self);
  gof->num_ref_pics[index] = value;
}

WEBRTC_EXPORT uint8_t
webrtc_GofInfoVP9_get_pid_diff(struct webrtc_GofInfoVP9* self,
                               size_t index,
                               size_t ref_index) {
  auto gof = reinterpret_cast<webrtc::GofInfoVP9*>(self);
  return gof->pid_diff[index][ref_index];
}

WEBRTC_EXPORT void webrtc_GofInfoVP9_set_pid_diff(
    struct webrtc_GofInfoVP9* self,
    size_t index,
    size_t ref_index,
    uint8_t value) {
  auto gof = reinterpret_cast<webrtc::GofInfoVP9*>(self);
  gof->pid_diff[index][ref_index] = value;
}

WEBRTC_EXPORT uint16_t
webrtc_GofInfoVP9_get_pid_start(struct webrtc_GofInfoVP9* self) {
  auto gof = reinterpret_cast<webrtc::GofInfoVP9*>(self);
  return gof->pid_start;
}

WEBRTC_EXPORT void webrtc_GofInfoVP9_set_pid_start(
    struct webrtc_GofInfoVP9* self,
    uint16_t value) {
  auto gof = reinterpret_cast<webrtc::GofInfoVP9*>(self);
  gof->pid_start = value;
}

WEBRTC_EXPORT struct webrtc_RTPVideoHeaderVP9* webrtc_RTPVideoHeaderVP9_new() {
  auto header = std::make_unique<webrtc::RTPVideoHeaderVP9>();
  header->InitRTPVideoHeaderVP9();
  return reinterpret_cast<struct webrtc_RTPVideoHeaderVP9*>(header.release());
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_delete(
    struct webrtc_RTPVideoHeaderVP9* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  delete header;
}

WEBRTC_EXPORT struct webrtc_RTPVideoHeaderVP9* webrtc_RTPVideoHeaderVP9_copy(
    const struct webrtc_RTPVideoHeaderVP9* self) {
  auto header = reinterpret_cast<const webrtc::RTPVideoHeaderVP9*>(self);
  auto copy = std::make_unique<webrtc::RTPVideoHeaderVP9>(*header);
  return reinterpret_cast<struct webrtc_RTPVideoHeaderVP9*>(copy.release());
}

WEBRTC_EXPORT int webrtc_RTPVideoHeaderVP9_get_inter_pic_predicted(
    struct webrtc_RTPVideoHeaderVP9* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  return header->inter_pic_predicted ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_inter_pic_predicted(
    struct webrtc_RTPVideoHeaderVP9* self,
    int value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  header->inter_pic_predicted = value != 0;
}

WEBRTC_EXPORT int webrtc_RTPVideoHeaderVP9_get_flexible_mode(
    struct webrtc_RTPVideoHeaderVP9* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  return header->flexible_mode ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_flexible_mode(
    struct webrtc_RTPVideoHeaderVP9* self,
    int value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  header->flexible_mode = value != 0;
}

WEBRTC_EXPORT int webrtc_RTPVideoHeaderVP9_get_beginning_of_frame(
    struct webrtc_RTPVideoHeaderVP9* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  return header->beginning_of_frame ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_beginning_of_frame(
    struct webrtc_RTPVideoHeaderVP9* self,
    int value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  header->beginning_of_frame = value != 0;
}

WEBRTC_EXPORT int webrtc_RTPVideoHeaderVP9_get_end_of_frame(
    struct webrtc_RTPVideoHeaderVP9* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  return header->end_of_frame ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_end_of_frame(
    struct webrtc_RTPVideoHeaderVP9* self,
    int value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  header->end_of_frame = value != 0;
}

WEBRTC_EXPORT int webrtc_RTPVideoHeaderVP9_get_ss_data_available(
    struct webrtc_RTPVideoHeaderVP9* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  return header->ss_data_available ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_ss_data_available(
    struct webrtc_RTPVideoHeaderVP9* self,
    int value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  header->ss_data_available = value != 0;
}

WEBRTC_EXPORT int webrtc_RTPVideoHeaderVP9_get_non_ref_for_inter_layer_pred(
    struct webrtc_RTPVideoHeaderVP9* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  return header->non_ref_for_inter_layer_pred ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_non_ref_for_inter_layer_pred(
    struct webrtc_RTPVideoHeaderVP9* self,
    int value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  header->non_ref_for_inter_layer_pred = value != 0;
}

WEBRTC_EXPORT int16_t
webrtc_RTPVideoHeaderVP9_get_picture_id(struct webrtc_RTPVideoHeaderVP9* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  return header->picture_id;
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_picture_id(
    struct webrtc_RTPVideoHeaderVP9* self,
    int16_t value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  header->picture_id = value;
}

WEBRTC_EXPORT int16_t webrtc_RTPVideoHeaderVP9_get_max_picture_id(
    struct webrtc_RTPVideoHeaderVP9* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  return header->max_picture_id;
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_max_picture_id(
    struct webrtc_RTPVideoHeaderVP9* self,
    int16_t value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  header->max_picture_id = value;
}

WEBRTC_EXPORT int16_t webrtc_RTPVideoHeaderVP9_get_tl0_pic_idx(
    struct webrtc_RTPVideoHeaderVP9* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  return header->tl0_pic_idx;
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_tl0_pic_idx(
    struct webrtc_RTPVideoHeaderVP9* self,
    int16_t value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  header->tl0_pic_idx = value;
}

WEBRTC_EXPORT uint8_t webrtc_RTPVideoHeaderVP9_get_temporal_idx(
    struct webrtc_RTPVideoHeaderVP9* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  return header->temporal_idx;
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_temporal_idx(
    struct webrtc_RTPVideoHeaderVP9* self,
    uint8_t value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  header->temporal_idx = value;
}

WEBRTC_EXPORT uint8_t webrtc_RTPVideoHeaderVP9_get_spatial_idx(
    struct webrtc_RTPVideoHeaderVP9* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  return header->spatial_idx;
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_spatial_idx(
    struct webrtc_RTPVideoHeaderVP9* self,
    uint8_t value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  header->spatial_idx = value;
}

WEBRTC_EXPORT int webrtc_RTPVideoHeaderVP9_get_temporal_up_switch(
    struct webrtc_RTPVideoHeaderVP9* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  return header->temporal_up_switch ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_temporal_up_switch(
    struct webrtc_RTPVideoHeaderVP9* self,
    int value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  header->temporal_up_switch = value != 0;
}

WEBRTC_EXPORT int webrtc_RTPVideoHeaderVP9_get_inter_layer_predicted(
    struct webrtc_RTPVideoHeaderVP9* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  return header->inter_layer_predicted ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_inter_layer_predicted(
    struct webrtc_RTPVideoHeaderVP9* self,
    int value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  header->inter_layer_predicted = value != 0;
}

WEBRTC_EXPORT uint8_t
webrtc_RTPVideoHeaderVP9_get_gof_idx(struct webrtc_RTPVideoHeaderVP9* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  return header->gof_idx;
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_gof_idx(
    struct webrtc_RTPVideoHeaderVP9* self,
    uint8_t value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  header->gof_idx = value;
}

WEBRTC_EXPORT uint8_t webrtc_RTPVideoHeaderVP9_get_num_ref_pics(
    struct webrtc_RTPVideoHeaderVP9* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  return header->num_ref_pics;
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_num_ref_pics(
    struct webrtc_RTPVideoHeaderVP9* self,
    uint8_t value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  header->num_ref_pics = value;
}

WEBRTC_EXPORT uint8_t
webrtc_RTPVideoHeaderVP9_get_pid_diff(struct webrtc_RTPVideoHeaderVP9* self,
                                      size_t index) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  return header->pid_diff[index];
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_pid_diff(
    struct webrtc_RTPVideoHeaderVP9* self,
    size_t index,
    uint8_t value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  header->pid_diff[index] = value;
}

WEBRTC_EXPORT int16_t webrtc_RTPVideoHeaderVP9_get_ref_picture_id(
    struct webrtc_RTPVideoHeaderVP9* self,
    size_t index) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  return header->ref_picture_id[index];
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_ref_picture_id(
    struct webrtc_RTPVideoHeaderVP9* self,
    size_t index,
    int16_t value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  header->ref_picture_id[index] = value;
}

WEBRTC_EXPORT size_t webrtc_RTPVideoHeaderVP9_get_num_spatial_layers(
    struct webrtc_RTPVideoHeaderVP9* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  return header->num_spatial_layers;
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_num_spatial_layers(
    struct webrtc_RTPVideoHeaderVP9* self,
    size_t value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  header->num_spatial_layers = value;
}

WEBRTC_EXPORT size_t webrtc_RTPVideoHeaderVP9_get_first_active_layer(
    struct webrtc_RTPVideoHeaderVP9* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  return header->first_active_layer;
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_first_active_layer(
    struct webrtc_RTPVideoHeaderVP9* self,
    size_t value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  header->first_active_layer = value;
}

WEBRTC_EXPORT int webrtc_RTPVideoHeaderVP9_get_spatial_layer_resolution_present(
    struct webrtc_RTPVideoHeaderVP9* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  return header->spatial_layer_resolution_present ? 1 : 0;
}

WEBRTC_EXPORT void
webrtc_RTPVideoHeaderVP9_set_spatial_layer_resolution_present(
    struct webrtc_RTPVideoHeaderVP9* self,
    int value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  header->spatial_layer_resolution_present = value != 0;
}

WEBRTC_EXPORT uint16_t
webrtc_RTPVideoHeaderVP9_get_width(struct webrtc_RTPVideoHeaderVP9* self,
                                   size_t index) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  return header->width[index];
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_width(
    struct webrtc_RTPVideoHeaderVP9* self,
    size_t index,
    uint16_t value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  header->width[index] = value;
}

WEBRTC_EXPORT uint16_t
webrtc_RTPVideoHeaderVP9_get_height(struct webrtc_RTPVideoHeaderVP9* self,
                                    size_t index) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  return header->height[index];
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_height(
    struct webrtc_RTPVideoHeaderVP9* self,
    size_t index,
    uint16_t value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  header->height[index] = value;
}

WEBRTC_EXPORT struct webrtc_GofInfoVP9* webrtc_RTPVideoHeaderVP9_get_gof(
    struct webrtc_RTPVideoHeaderVP9* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  return reinterpret_cast<struct webrtc_GofInfoVP9*>(&header->gof);
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_gof(
    struct webrtc_RTPVideoHeaderVP9* self,
    const struct webrtc_GofInfoVP9* value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  auto gof = reinterpret_cast<const webrtc::GofInfoVP9*>(value);
  header->gof = *gof;
}

WEBRTC_EXPORT int webrtc_RTPVideoHeaderVP9_get_end_of_picture(
    struct webrtc_RTPVideoHeaderVP9* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  return header->end_of_picture ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP9_set_end_of_picture(
    struct webrtc_RTPVideoHeaderVP9* self,
    int value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP9*>(self);
  header->end_of_picture = value != 0;
}
}
