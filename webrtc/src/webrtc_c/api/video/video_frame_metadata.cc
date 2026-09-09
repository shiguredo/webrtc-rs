#include "video_frame_metadata.h"

#include <assert.h>
#include <cstddef>
#include <cstdint>
#include <memory>
#include <optional>
#include <span>
#include <utility>
#include <variant>
#include <vector>  // IWYU pragma: keep

#include <api/transport/rtp/dependency_descriptor.h>
#include <api/video/video_content_type.h>
#include <api/video/video_frame_metadata.h>
#include <modules/video_coding/codecs/h264/include/h264_globals.h>
#include <modules/video_coding/codecs/vp8/include/vp8_globals.h>
#include <modules/video_coding/codecs/vp9/include/vp9_globals.h>

#include "../../common.h"
#include "../../common.impl.h"
#include "../../modules/video_coding/codecs/h264/include/h264_globals.h"
#include "../../modules/video_coding/codecs/vp8/include/vp8_globals.h"
#include "../../modules/video_coding/codecs/vp9/include/vp9_globals.h"
#include "../../std.h"
#include "../../std.impl.h"
#include "api/video/video_codec_type.h"
#include "api/video/video_frame_type.h"
#include "api/video/video_rotation.h"

extern "C" {
WEBRTC_DEFINE_VARIANT(webrtc_RTPVideoHeaderCodecSpecifics,
                      webrtc::RTPVideoHeaderCodecSpecifics);

// -------------------------
// webrtc::RTPVideoHeaderCodecSpecifics
// -------------------------

WEBRTC_EXPORT struct webrtc_RTPVideoHeaderVP8*
webrtc_RTPVideoHeaderCodecSpecifics_get_RTPVideoHeaderVP8(
    struct webrtc_RTPVideoHeaderCodecSpecifics* self) {
  auto variant = reinterpret_cast<webrtc::RTPVideoHeaderCodecSpecifics*>(self);
  auto* value = std::get_if<webrtc::RTPVideoHeaderVP8>(variant);
  return value == nullptr
             ? nullptr
             : reinterpret_cast<struct webrtc_RTPVideoHeaderVP8*>(value);
}

WEBRTC_EXPORT struct webrtc_RTPVideoHeaderVP9*
webrtc_RTPVideoHeaderCodecSpecifics_get_RTPVideoHeaderVP9(
    struct webrtc_RTPVideoHeaderCodecSpecifics* self) {
  auto variant = reinterpret_cast<webrtc::RTPVideoHeaderCodecSpecifics*>(self);
  auto* value = std::get_if<webrtc::RTPVideoHeaderVP9>(variant);
  return value == nullptr
             ? nullptr
             : reinterpret_cast<struct webrtc_RTPVideoHeaderVP9*>(value);
}

WEBRTC_EXPORT struct webrtc_RTPVideoHeaderH264*
webrtc_RTPVideoHeaderCodecSpecifics_get_RTPVideoHeaderH264(
    struct webrtc_RTPVideoHeaderCodecSpecifics* self) {
  auto variant = reinterpret_cast<webrtc::RTPVideoHeaderCodecSpecifics*>(self);
  auto* value = std::get_if<webrtc::RTPVideoHeaderH264>(variant);
  return value == nullptr
             ? nullptr
             : reinterpret_cast<struct webrtc_RTPVideoHeaderH264*>(value);
}

WEBRTC_EXPORT struct webrtc_RTPVideoHeaderCodecSpecifics_unique*
webrtc_RTPVideoHeaderCodecSpecifics_new_monostate() {
  auto variant =
      std::make_unique<webrtc::RTPVideoHeaderCodecSpecifics>(std::monostate{});
  return reinterpret_cast<struct webrtc_RTPVideoHeaderCodecSpecifics_unique*>(
      variant.release());
}

WEBRTC_EXPORT struct webrtc_RTPVideoHeaderCodecSpecifics_unique*
webrtc_RTPVideoHeaderCodecSpecifics_new_RTPVideoHeaderVP8(
    const struct webrtc_RTPVideoHeaderVP8* value) {
  assert(value != nullptr);
  auto header = reinterpret_cast<const webrtc::RTPVideoHeaderVP8*>(value);
  auto variant = std::make_unique<webrtc::RTPVideoHeaderCodecSpecifics>(
      std::in_place_type<webrtc::RTPVideoHeaderVP8>, *header);
  return reinterpret_cast<struct webrtc_RTPVideoHeaderCodecSpecifics_unique*>(
      variant.release());
}

WEBRTC_EXPORT struct webrtc_RTPVideoHeaderCodecSpecifics_unique*
webrtc_RTPVideoHeaderCodecSpecifics_new_RTPVideoHeaderVP9(
    const struct webrtc_RTPVideoHeaderVP9* value) {
  assert(value != nullptr);
  auto header = reinterpret_cast<const webrtc::RTPVideoHeaderVP9*>(value);
  auto variant = std::make_unique<webrtc::RTPVideoHeaderCodecSpecifics>(
      std::in_place_type<webrtc::RTPVideoHeaderVP9>, *header);
  return reinterpret_cast<struct webrtc_RTPVideoHeaderCodecSpecifics_unique*>(
      variant.release());
}

WEBRTC_EXPORT struct webrtc_RTPVideoHeaderCodecSpecifics_unique*
webrtc_RTPVideoHeaderCodecSpecifics_new_RTPVideoHeaderH264(
    const struct webrtc_RTPVideoHeaderH264* value) {
  assert(value != nullptr);
  auto header = reinterpret_cast<const webrtc::RTPVideoHeaderH264*>(value);
  auto variant = std::make_unique<webrtc::RTPVideoHeaderCodecSpecifics>(
      std::in_place_type<webrtc::RTPVideoHeaderH264>, *header);
  return reinterpret_cast<struct webrtc_RTPVideoHeaderCodecSpecifics_unique*>(
      variant.release());
}

WEBRTC_EXPORT struct webrtc_VideoFrameMetadata*
webrtc_VideoFrameMetadata_new() {
  auto metadata = std::make_unique<webrtc::VideoFrameMetadata>();
  return reinterpret_cast<struct webrtc_VideoFrameMetadata*>(
      metadata.release());
}

WEBRTC_EXPORT void webrtc_VideoFrameMetadata_delete(
    struct webrtc_VideoFrameMetadata* self) {
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  delete metadata;
}

WEBRTC_EXPORT struct webrtc_VideoFrameMetadata* webrtc_VideoFrameMetadata_copy(
    const struct webrtc_VideoFrameMetadata* self) {
  auto metadata = reinterpret_cast<const webrtc::VideoFrameMetadata*>(self);
  auto copy = std::make_unique<webrtc::VideoFrameMetadata>(*metadata);
  return reinterpret_cast<struct webrtc_VideoFrameMetadata*>(copy.release());
}

WEBRTC_EXPORT int webrtc_VideoFrameMetadata_GetFrameType(
    struct webrtc_VideoFrameMetadata* self) {
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  return static_cast<int>(metadata->GetFrameType());
}

WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetFrameType(
    struct webrtc_VideoFrameMetadata* self,
    int frame_type) {
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  metadata->SetFrameType(static_cast<webrtc::VideoFrameType>(frame_type));
}

WEBRTC_EXPORT uint16_t
webrtc_VideoFrameMetadata_GetWidth(struct webrtc_VideoFrameMetadata* self) {
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  return metadata->GetWidth();
}

WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetWidth(
    struct webrtc_VideoFrameMetadata* self,
    uint16_t width) {
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  metadata->SetWidth(width);
}

WEBRTC_EXPORT uint16_t
webrtc_VideoFrameMetadata_GetHeight(struct webrtc_VideoFrameMetadata* self) {
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  return metadata->GetHeight();
}

WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetHeight(
    struct webrtc_VideoFrameMetadata* self,
    uint16_t height) {
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  metadata->SetHeight(height);
}

WEBRTC_EXPORT void webrtc_VideoFrameMetadata_GetFrameId(
    struct webrtc_VideoFrameMetadata* self,
    int* has,
    int64_t* frame_id) {
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  auto value = metadata->GetFrameId();
  webrtc_c::OptionalGet(value, has, frame_id);
}

WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetFrameId(
    struct webrtc_VideoFrameMetadata* self,
    int has,
    const int64_t* frame_id) {
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  std::optional<int64_t> value;
  webrtc_c::OptionalSet(value, has, frame_id);
  metadata->SetFrameId(value);
}

WEBRTC_EXPORT int webrtc_VideoFrameMetadata_GetSpatialIndex(
    struct webrtc_VideoFrameMetadata* self) {
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  return metadata->GetSpatialIndex();
}

WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetSpatialIndex(
    struct webrtc_VideoFrameMetadata* self,
    int spatial_index) {
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  metadata->SetSpatialIndex(spatial_index);
}

WEBRTC_EXPORT int webrtc_VideoFrameMetadata_GetTemporalIndex(
    struct webrtc_VideoFrameMetadata* self) {
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  return metadata->GetTemporalIndex();
}

WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetTemporalIndex(
    struct webrtc_VideoFrameMetadata* self,
    int temporal_index) {
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  metadata->SetTemporalIndex(temporal_index);
}

WEBRTC_EXPORT void webrtc_VideoFrameMetadata_GetDependencies(
    struct webrtc_VideoFrameMetadata* self,
    int* has,
    const int64_t** data,
    size_t* len) {
  assert(has != nullptr);
  assert(data != nullptr);
  assert(len != nullptr);
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  auto value = metadata->GetDependencies();
  if (value.has_value()) {
    *has = 1;
    *data = value->data();
    *len = value->size();
  } else {
    *has = 0;
  }
}

WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetDependencies(
    struct webrtc_VideoFrameMetadata* self,
    int has,
    const int64_t* data,
    size_t len) {
  assert(has == 0 || data != nullptr);
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  if (has != 0) {
    metadata->SetDependencies(std::span<const int64_t>(data, len));
  } else {
    metadata->SetDependencies(std::nullopt);
  }
}

WEBRTC_EXPORT int webrtc_VideoFrameMetadata_GetIsLastFrameInPicture(
    struct webrtc_VideoFrameMetadata* self) {
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  return metadata->GetIsLastFrameInPicture() ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetIsLastFrameInPicture(
    struct webrtc_VideoFrameMetadata* self,
    int is_last_frame_in_picture) {
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  metadata->SetIsLastFrameInPicture(is_last_frame_in_picture != 0);
}

WEBRTC_EXPORT uint8_t webrtc_VideoFrameMetadata_GetSimulcastIdx(
    struct webrtc_VideoFrameMetadata* self) {
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  return metadata->GetSimulcastIdx();
}

WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetSimulcastIdx(
    struct webrtc_VideoFrameMetadata* self,
    uint8_t simulcast_idx) {
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  metadata->SetSimulcastIdx(simulcast_idx);
}

WEBRTC_EXPORT int webrtc_VideoFrameMetadata_GetCodec(
    struct webrtc_VideoFrameMetadata* self) {
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  return static_cast<int>(metadata->GetCodec());
}

WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetCodec(
    struct webrtc_VideoFrameMetadata* self,
    int codec) {
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  metadata->SetCodec(static_cast<webrtc::VideoCodecType>(codec));
}

WEBRTC_EXPORT uint32_t
webrtc_VideoFrameMetadata_GetSsrc(struct webrtc_VideoFrameMetadata* self) {
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  return metadata->GetSsrc();
}

WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetSsrc(
    struct webrtc_VideoFrameMetadata* self,
    uint32_t ssrc) {
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  metadata->SetSsrc(ssrc);
}

WEBRTC_EXPORT int webrtc_VideoFrameMetadata_GetRotation(
    struct webrtc_VideoFrameMetadata* self) {
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  return static_cast<int>(metadata->GetRotation());
}

WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetRotation(
    struct webrtc_VideoFrameMetadata* self,
    int rotation) {
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  metadata->SetRotation(static_cast<webrtc::VideoRotation>(rotation));
}

WEBRTC_EXPORT int webrtc_VideoFrameMetadata_GetContentType(
    struct webrtc_VideoFrameMetadata* self) {
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  return static_cast<int>(metadata->GetContentType());
}

WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetContentType(
    struct webrtc_VideoFrameMetadata* self,
    int content_type) {
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  metadata->SetContentType(static_cast<webrtc::VideoContentType>(content_type));
}

WEBRTC_EXPORT void webrtc_VideoFrameMetadata_GetDecodeTargetIndications(
    struct webrtc_VideoFrameMetadata* self,
    const int** out_data,
    size_t* out_len) {
  assert(out_data != nullptr);
  assert(out_len != nullptr);
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  auto indications = metadata->GetDecodeTargetIndications();
  *out_data = reinterpret_cast<const int*>(indications.data());
  *out_len = indications.size();
}

WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetDecodeTargetIndications(
    struct webrtc_VideoFrameMetadata* self,
    const int* data,
    size_t len) {
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  std::span<const webrtc::DecodeTargetIndication> indications;
  if (len > 0) {
    assert(data != nullptr);
    indications = std::span<const webrtc::DecodeTargetIndication>(
        reinterpret_cast<const webrtc::DecodeTargetIndication*>(data), len);
  }
  metadata->SetDecodeTargetIndications(indications);
}

WEBRTC_EXPORT struct webrtc_uint32_vector* webrtc_VideoFrameMetadata_GetCsrcs(
    struct webrtc_VideoFrameMetadata* self) {
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  auto csrcs = std::make_unique<std::vector<uint32_t>>(metadata->GetCsrcs());
  return reinterpret_cast<struct webrtc_uint32_vector*>(csrcs.release());
}

WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetCsrcs(
    struct webrtc_VideoFrameMetadata* self,
    const struct webrtc_uint32_vector* csrcs) {
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  auto v = reinterpret_cast<const std::vector<uint32_t>*>(csrcs);
  metadata->SetCsrcs(*v);
}

WEBRTC_EXPORT struct webrtc_RTPVideoHeaderCodecSpecifics_unique*
webrtc_VideoFrameMetadata_GetRTPVideoHeaderCodecSpecifics(
    struct webrtc_VideoFrameMetadata* self) {
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  auto variant = std::make_unique<webrtc::RTPVideoHeaderCodecSpecifics>(
      metadata->GetRTPVideoHeaderCodecSpecifics());
  return reinterpret_cast<struct webrtc_RTPVideoHeaderCodecSpecifics_unique*>(
      variant.release());
}

WEBRTC_EXPORT void webrtc_VideoFrameMetadata_SetRTPVideoHeaderCodecSpecifics(
    struct webrtc_VideoFrameMetadata* self,
    const struct webrtc_RTPVideoHeaderCodecSpecifics* value) {
  auto metadata = reinterpret_cast<webrtc::VideoFrameMetadata*>(self);
  auto variant =
      reinterpret_cast<const webrtc::RTPVideoHeaderCodecSpecifics*>(value);
  metadata->SetRTPVideoHeaderCodecSpecifics(*variant);
}
}
