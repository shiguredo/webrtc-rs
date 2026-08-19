#include "h264_globals.h"

#include <assert.h>
#include <memory>
#include <vector>  // IWYU pragma: keep

#include <modules/video_coding/codecs/h264/include/h264_globals.h>

#include "../../../../../common.h"
#include "../../../../../common.impl.h"

extern "C" {
WEBRTC_EXPORT const int webrtc_H264PacketizationType_SingleNalu =
    static_cast<int>(webrtc::kH264SingleNalu);
WEBRTC_EXPORT const int webrtc_H264PacketizationType_StapA =
    static_cast<int>(webrtc::kH264StapA);
WEBRTC_EXPORT const int webrtc_H264PacketizationType_FuA =
    static_cast<int>(webrtc::kH264FuA);

WEBRTC_EXPORT struct webrtc_NaluInfo* webrtc_NaluInfo_new() {
  auto info = std::make_unique<webrtc::NaluInfo>();
  return reinterpret_cast<struct webrtc_NaluInfo*>(info.release());
}

WEBRTC_EXPORT void webrtc_NaluInfo_delete(struct webrtc_NaluInfo* self) {
  auto info = reinterpret_cast<webrtc::NaluInfo*>(self);
  delete info;
}

WEBRTC_EXPORT struct webrtc_NaluInfo* webrtc_NaluInfo_copy(
    const struct webrtc_NaluInfo* self) {
  auto info = reinterpret_cast<const webrtc::NaluInfo*>(self);
  auto copy = std::make_unique<webrtc::NaluInfo>(*info);
  return reinterpret_cast<struct webrtc_NaluInfo*>(copy.release());
}

WEBRTC_EXPORT uint8_t webrtc_NaluInfo_get_type(struct webrtc_NaluInfo* self) {
  auto info = reinterpret_cast<webrtc::NaluInfo*>(self);
  return info->type;
}

WEBRTC_EXPORT void webrtc_NaluInfo_set_type(struct webrtc_NaluInfo* self,
                                            uint8_t value) {
  auto info = reinterpret_cast<webrtc::NaluInfo*>(self);
  info->type = value;
}

WEBRTC_EXPORT int webrtc_NaluInfo_get_sps_id(struct webrtc_NaluInfo* self) {
  auto info = reinterpret_cast<webrtc::NaluInfo*>(self);
  return info->sps_id;
}

WEBRTC_EXPORT void webrtc_NaluInfo_set_sps_id(struct webrtc_NaluInfo* self,
                                              int value) {
  auto info = reinterpret_cast<webrtc::NaluInfo*>(self);
  info->sps_id = value;
}

WEBRTC_EXPORT int webrtc_NaluInfo_get_pps_id(struct webrtc_NaluInfo* self) {
  auto info = reinterpret_cast<webrtc::NaluInfo*>(self);
  return info->pps_id;
}

WEBRTC_EXPORT void webrtc_NaluInfo_set_pps_id(struct webrtc_NaluInfo* self,
                                              int value) {
  auto info = reinterpret_cast<webrtc::NaluInfo*>(self);
  info->pps_id = value;
}

WEBRTC_DEFINE_VECTOR(webrtc_NaluInfo, webrtc::NaluInfo);

WEBRTC_EXPORT struct webrtc_RTPVideoHeaderH264*
webrtc_RTPVideoHeaderH264_new() {
  auto header = std::make_unique<webrtc::RTPVideoHeaderH264>();
  return reinterpret_cast<struct webrtc_RTPVideoHeaderH264*>(header.release());
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderH264_delete(
    struct webrtc_RTPVideoHeaderH264* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderH264*>(self);
  delete header;
}

WEBRTC_EXPORT struct webrtc_RTPVideoHeaderH264* webrtc_RTPVideoHeaderH264_copy(
    const struct webrtc_RTPVideoHeaderH264* self) {
  auto header = reinterpret_cast<const webrtc::RTPVideoHeaderH264*>(self);
  auto copy = std::make_unique<webrtc::RTPVideoHeaderH264>(*header);
  return reinterpret_cast<struct webrtc_RTPVideoHeaderH264*>(copy.release());
}

WEBRTC_EXPORT uint8_t webrtc_RTPVideoHeaderH264_get_nalu_type(
    struct webrtc_RTPVideoHeaderH264* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderH264*>(self);
  return header->nalu_type;
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderH264_set_nalu_type(
    struct webrtc_RTPVideoHeaderH264* self,
    uint8_t value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderH264*>(self);
  header->nalu_type = value;
}

WEBRTC_EXPORT int webrtc_RTPVideoHeaderH264_get_packetization_type(
    struct webrtc_RTPVideoHeaderH264* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderH264*>(self);
  return static_cast<int>(header->packetization_type);
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderH264_set_packetization_type(
    struct webrtc_RTPVideoHeaderH264* self,
    int value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderH264*>(self);
  header->packetization_type =
      static_cast<webrtc::H264PacketizationTypes>(value);
}

WEBRTC_EXPORT struct webrtc_NaluInfo_vector*
webrtc_RTPVideoHeaderH264_get_nalus(struct webrtc_RTPVideoHeaderH264* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderH264*>(self);
  return reinterpret_cast<struct webrtc_NaluInfo_vector*>(&header->nalus);
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderH264_set_nalus(
    struct webrtc_RTPVideoHeaderH264* self,
    const struct webrtc_NaluInfo_vector* value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderH264*>(self);
  auto nalus = reinterpret_cast<const std::vector<webrtc::NaluInfo>*>(value);
  header->nalus = *nalus;
}

WEBRTC_EXPORT int webrtc_RTPVideoHeaderH264_get_packetization_mode(
    struct webrtc_RTPVideoHeaderH264* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderH264*>(self);
  return static_cast<int>(header->packetization_mode);
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderH264_set_packetization_mode(
    struct webrtc_RTPVideoHeaderH264* self,
    int value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderH264*>(self);
  header->packetization_mode =
      static_cast<webrtc::H264PacketizationMode>(value);
}
}
