#pragma once

#include <stdint.h>

#include "../../../../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::H264PacketizationTypes
// -------------------------

WEBRTC_EXPORT extern const int webrtc_H264PacketizationType_SingleNalu;
WEBRTC_EXPORT extern const int webrtc_H264PacketizationType_StapA;
WEBRTC_EXPORT extern const int webrtc_H264PacketizationType_FuA;

// -------------------------
// webrtc::NaluInfo
// -------------------------

struct webrtc_NaluInfo;

WEBRTC_EXPORT struct webrtc_NaluInfo* webrtc_NaluInfo_new();
WEBRTC_EXPORT void webrtc_NaluInfo_delete(struct webrtc_NaluInfo* self);
WEBRTC_EXPORT struct webrtc_NaluInfo* webrtc_NaluInfo_copy(
    const struct webrtc_NaluInfo* self);
WEBRTC_EXPORT uint8_t webrtc_NaluInfo_get_type(struct webrtc_NaluInfo* self);
WEBRTC_EXPORT void webrtc_NaluInfo_set_type(struct webrtc_NaluInfo* self,
                                            uint8_t value);
WEBRTC_EXPORT int webrtc_NaluInfo_get_sps_id(struct webrtc_NaluInfo* self);
WEBRTC_EXPORT void webrtc_NaluInfo_set_sps_id(struct webrtc_NaluInfo* self,
                                              int value);
WEBRTC_EXPORT int webrtc_NaluInfo_get_pps_id(struct webrtc_NaluInfo* self);
WEBRTC_EXPORT void webrtc_NaluInfo_set_pps_id(struct webrtc_NaluInfo* self,
                                              int value);

// -------------------------
// std::vector<webrtc::NaluInfo>
// -------------------------

WEBRTC_DECLARE_VECTOR(webrtc_NaluInfo);

// -------------------------
// webrtc::RTPVideoHeaderH264
// -------------------------

struct webrtc_RTPVideoHeaderH264;

WEBRTC_EXPORT struct webrtc_RTPVideoHeaderH264* webrtc_RTPVideoHeaderH264_new();
WEBRTC_EXPORT void webrtc_RTPVideoHeaderH264_delete(
    struct webrtc_RTPVideoHeaderH264* self);
WEBRTC_EXPORT struct webrtc_RTPVideoHeaderH264* webrtc_RTPVideoHeaderH264_copy(
    const struct webrtc_RTPVideoHeaderH264* self);

WEBRTC_EXPORT uint8_t
webrtc_RTPVideoHeaderH264_get_nalu_type(struct webrtc_RTPVideoHeaderH264* self);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderH264_set_nalu_type(
    struct webrtc_RTPVideoHeaderH264* self,
    uint8_t value);
WEBRTC_EXPORT int webrtc_RTPVideoHeaderH264_get_packetization_type(
    struct webrtc_RTPVideoHeaderH264* self);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderH264_set_packetization_type(
    struct webrtc_RTPVideoHeaderH264* self,
    int value);
// nalus は借用ポインタで取得し、set はコピーする。
WEBRTC_EXPORT struct webrtc_NaluInfo_vector*
webrtc_RTPVideoHeaderH264_get_nalus(struct webrtc_RTPVideoHeaderH264* self);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderH264_set_nalus(
    struct webrtc_RTPVideoHeaderH264* self,
    const struct webrtc_NaluInfo_vector* value);
WEBRTC_EXPORT int webrtc_RTPVideoHeaderH264_get_packetization_mode(
    struct webrtc_RTPVideoHeaderH264* self);
WEBRTC_EXPORT void webrtc_RTPVideoHeaderH264_set_packetization_mode(
    struct webrtc_RTPVideoHeaderH264* self,
    int value);

#if defined(__cplusplus)
}
#endif
