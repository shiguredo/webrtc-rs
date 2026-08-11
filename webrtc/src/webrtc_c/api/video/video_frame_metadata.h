#pragma once

#include <stddef.h>
#include <stdint.h>

#include "../../common.h"
#include "../../modules/video_coding/codecs/h264/include/h264_globals.h"
#include "../../modules/video_coding/codecs/vp8/include/vp8_globals.h"
#include "../../modules/video_coding/codecs/vp9/include/vp9_globals.h"
#include "../../std.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::RTPVideoHeaderCodecSpecifics
// (std::variant<std::monostate, RTPVideoHeaderVP8, RTPVideoHeaderVP9,
//               RTPVideoHeaderH264>)
// -------------------------

// ヒープ確保したコピーを返し、webrtc_RTPVideoHeaderCodecSpecifics_unique_delete
// で破棄する。
WEBRTC_DECLARE_VARIANT(webrtc_RTPVideoHeaderCodecSpecifics);

// 各 alternative の値。アクティブでない場合は null。戻り値は借用 (delete しない)。
WEBRTC_EXPORT struct webrtc_RTPVideoHeaderVP8*
webrtc_RTPVideoHeaderCodecSpecifics_get_RTPVideoHeaderVP8(
    struct webrtc_RTPVideoHeaderCodecSpecifics* self);
WEBRTC_EXPORT struct webrtc_RTPVideoHeaderVP9*
webrtc_RTPVideoHeaderCodecSpecifics_get_RTPVideoHeaderVP9(
    struct webrtc_RTPVideoHeaderCodecSpecifics* self);
WEBRTC_EXPORT struct webrtc_RTPVideoHeaderH264*
webrtc_RTPVideoHeaderCodecSpecifics_get_RTPVideoHeaderH264(
    struct webrtc_RTPVideoHeaderCodecSpecifics* self);

// 各 alternative から variant を構築する。
WEBRTC_EXPORT struct webrtc_RTPVideoHeaderCodecSpecifics_unique*
webrtc_RTPVideoHeaderCodecSpecifics_new_monostate();
WEBRTC_EXPORT struct webrtc_RTPVideoHeaderCodecSpecifics_unique*
webrtc_RTPVideoHeaderCodecSpecifics_new_RTPVideoHeaderVP8(
    const struct webrtc_RTPVideoHeaderVP8* value);
WEBRTC_EXPORT struct webrtc_RTPVideoHeaderCodecSpecifics_unique*
webrtc_RTPVideoHeaderCodecSpecifics_new_RTPVideoHeaderVP9(
    const struct webrtc_RTPVideoHeaderVP9* value);
WEBRTC_EXPORT struct webrtc_RTPVideoHeaderCodecSpecifics_unique*
webrtc_RTPVideoHeaderCodecSpecifics_new_RTPVideoHeaderH264(
    const struct webrtc_RTPVideoHeaderH264* value);

// -------------------------
// webrtc::VideoFrameMetadata
// -------------------------

struct webrtc_VideoFrameMetadata;

WEBRTC_EXPORT struct webrtc_VideoFrameMetadata* webrtc_VideoFrameMetadata_new();
WEBRTC_EXPORT void webrtc_VideoFrameMetadata_delete(
    struct webrtc_VideoFrameMetadata* self);

WEBRTC_EXPORT int webrtc_VideoFrameMetadata_GetFrameType(
    struct webrtc_VideoFrameMetadata* self);
WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetFrameType(
    struct webrtc_VideoFrameMetadata* self,
    int frame_type);
WEBRTC_EXPORT uint16_t
webrtc_VideoFrameMetadata_GetWidth(struct webrtc_VideoFrameMetadata* self);
WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetWidth(
    struct webrtc_VideoFrameMetadata* self,
    uint16_t width);
WEBRTC_EXPORT uint16_t
webrtc_VideoFrameMetadata_GetHeight(struct webrtc_VideoFrameMetadata* self);
WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetHeight(
    struct webrtc_VideoFrameMetadata* self,
    uint16_t height);
// optional は int* has に 0 か 1 を設定する。
WEBRTC_EXPORT void webrtc_VideoFrameMetadata_GetFrameId(
    struct webrtc_VideoFrameMetadata* self,
    int* has,
    int64_t* frame_id);
WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetFrameId(
    struct webrtc_VideoFrameMetadata* self,
    int has,
    const int64_t* frame_id);
WEBRTC_EXPORT int webrtc_VideoFrameMetadata_GetSpatialIndex(
    struct webrtc_VideoFrameMetadata* self);
WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetSpatialIndex(
    struct webrtc_VideoFrameMetadata* self,
    int spatial_index);
WEBRTC_EXPORT int webrtc_VideoFrameMetadata_GetTemporalIndex(
    struct webrtc_VideoFrameMetadata* self);
WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetTemporalIndex(
    struct webrtc_VideoFrameMetadata* self,
    int temporal_index);
// optional<span<const int64_t>> は int* has と data/len に展開する。
WEBRTC_EXPORT void webrtc_VideoFrameMetadata_GetDependencies(
    struct webrtc_VideoFrameMetadata* self,
    int* has,
    const int64_t** data,
    size_t* len);
WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetDependencies(
    struct webrtc_VideoFrameMetadata* self,
    int has,
    const int64_t* data,
    size_t len);
WEBRTC_EXPORT int webrtc_VideoFrameMetadata_GetIsLastFrameInPicture(
    struct webrtc_VideoFrameMetadata* self);
WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetIsLastFrameInPicture(
    struct webrtc_VideoFrameMetadata* self,
    int is_last_frame_in_picture);
WEBRTC_EXPORT uint8_t webrtc_VideoFrameMetadata_GetSimulcastIdx(
    struct webrtc_VideoFrameMetadata* self);
WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetSimulcastIdx(
    struct webrtc_VideoFrameMetadata* self,
    uint8_t simulcast_idx);
WEBRTC_EXPORT int webrtc_VideoFrameMetadata_GetCodec(
    struct webrtc_VideoFrameMetadata* self);
WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetCodec(
    struct webrtc_VideoFrameMetadata* self,
    int codec);
WEBRTC_EXPORT uint32_t
webrtc_VideoFrameMetadata_GetSsrc(struct webrtc_VideoFrameMetadata* self);
WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetSsrc(
    struct webrtc_VideoFrameMetadata* self,
    uint32_t ssrc);

WEBRTC_EXPORT int webrtc_VideoFrameMetadata_GetRotation(
    struct webrtc_VideoFrameMetadata* self);
WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetRotation(
    struct webrtc_VideoFrameMetadata* self,
    int rotation);
WEBRTC_EXPORT int webrtc_VideoFrameMetadata_GetContentType(
    struct webrtc_VideoFrameMetadata* self);
WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetContentType(
    struct webrtc_VideoFrameMetadata* self,
    int content_type);
// DecodeTargetIndications は metadata 内を借用した配列を返す。
WEBRTC_EXPORT void webrtc_VideoFrameMetadata_GetDecodeTargetIndications(
    struct webrtc_VideoFrameMetadata* self,
    const int** out_data,
    size_t* out_len);
WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetDecodeTargetIndications(
    struct webrtc_VideoFrameMetadata* self,
    const int* data,
    size_t len);
// Csrcs はヒープ確保したコピーを返し、webrtc_uint32_vector_delete で破棄する。
WEBRTC_EXPORT struct webrtc_uint32_vector* webrtc_VideoFrameMetadata_GetCsrcs(
    struct webrtc_VideoFrameMetadata* self);
WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetCsrcs(
    struct webrtc_VideoFrameMetadata* self,
    const struct webrtc_uint32_vector* csrcs);
// RTPVideoHeaderCodecSpecifics はヒープ確保したコピーを返し、
// webrtc_RTPVideoHeaderCodecSpecifics_unique_delete で破棄する。
WEBRTC_EXPORT struct webrtc_RTPVideoHeaderCodecSpecifics_unique*
webrtc_VideoFrameMetadata_GetRTPVideoHeaderCodecSpecifics(
    struct webrtc_VideoFrameMetadata* self);
WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetRTPVideoHeaderCodecSpecifics(
    struct webrtc_VideoFrameMetadata* self,
    const struct webrtc_RTPVideoHeaderCodecSpecifics* value);

#if defined(__cplusplus)
}
#endif
