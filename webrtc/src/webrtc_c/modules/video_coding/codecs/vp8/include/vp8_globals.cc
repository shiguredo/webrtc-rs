#include "vp8_globals.h"

#include <assert.h>
#include <cstdint>
#include <memory>

// WebRTC
#include <modules/video_coding/codecs/vp8/include/vp8_globals.h>

#include "../../../../../common.h"

extern "C" {
WEBRTC_EXPORT struct webrtc_RTPVideoHeaderVP8* webrtc_RTPVideoHeaderVP8_new() {
  auto header = std::make_unique<webrtc::RTPVideoHeaderVP8>();
  header->InitRTPVideoHeaderVP8();
  return reinterpret_cast<struct webrtc_RTPVideoHeaderVP8*>(header.release());
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP8_delete(
    struct webrtc_RTPVideoHeaderVP8* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP8*>(self);
  delete header;
}

WEBRTC_EXPORT struct webrtc_RTPVideoHeaderVP8* webrtc_RTPVideoHeaderVP8_copy(
    const struct webrtc_RTPVideoHeaderVP8* self) {
  auto header = reinterpret_cast<const webrtc::RTPVideoHeaderVP8*>(self);
  auto copy = std::make_unique<webrtc::RTPVideoHeaderVP8>(*header);
  return reinterpret_cast<struct webrtc_RTPVideoHeaderVP8*>(copy.release());
}

WEBRTC_EXPORT int webrtc_RTPVideoHeaderVP8_get_nonReference(
    struct webrtc_RTPVideoHeaderVP8* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP8*>(self);
  return header->nonReference ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP8_set_nonReference(
    struct webrtc_RTPVideoHeaderVP8* self,
    int value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP8*>(self);
  header->nonReference = value != 0;
}

WEBRTC_EXPORT int16_t
webrtc_RTPVideoHeaderVP8_get_pictureId(struct webrtc_RTPVideoHeaderVP8* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP8*>(self);
  return header->pictureId;
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP8_set_pictureId(
    struct webrtc_RTPVideoHeaderVP8* self,
    int16_t value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP8*>(self);
  header->pictureId = value;
}

WEBRTC_EXPORT int16_t
webrtc_RTPVideoHeaderVP8_get_tl0PicIdx(struct webrtc_RTPVideoHeaderVP8* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP8*>(self);
  return header->tl0PicIdx;
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP8_set_tl0PicIdx(
    struct webrtc_RTPVideoHeaderVP8* self,
    int16_t value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP8*>(self);
  header->tl0PicIdx = value;
}

WEBRTC_EXPORT uint8_t webrtc_RTPVideoHeaderVP8_get_temporalIdx(
    struct webrtc_RTPVideoHeaderVP8* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP8*>(self);
  return header->temporalIdx;
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP8_set_temporalIdx(
    struct webrtc_RTPVideoHeaderVP8* self,
    uint8_t value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP8*>(self);
  header->temporalIdx = value;
}

WEBRTC_EXPORT int webrtc_RTPVideoHeaderVP8_get_layerSync(
    struct webrtc_RTPVideoHeaderVP8* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP8*>(self);
  return header->layerSync ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP8_set_layerSync(
    struct webrtc_RTPVideoHeaderVP8* self,
    int value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP8*>(self);
  header->layerSync = value != 0;
}

WEBRTC_EXPORT int webrtc_RTPVideoHeaderVP8_get_keyIdx(
    struct webrtc_RTPVideoHeaderVP8* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP8*>(self);
  return header->keyIdx;
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP8_set_keyIdx(
    struct webrtc_RTPVideoHeaderVP8* self,
    int value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP8*>(self);
  header->keyIdx = value;
}

WEBRTC_EXPORT int webrtc_RTPVideoHeaderVP8_get_partitionId(
    struct webrtc_RTPVideoHeaderVP8* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP8*>(self);
  return header->partitionId;
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP8_set_partitionId(
    struct webrtc_RTPVideoHeaderVP8* self,
    int value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP8*>(self);
  header->partitionId = value;
}

WEBRTC_EXPORT int webrtc_RTPVideoHeaderVP8_get_beginningOfPartition(
    struct webrtc_RTPVideoHeaderVP8* self) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP8*>(self);
  return header->beginningOfPartition ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_RTPVideoHeaderVP8_set_beginningOfPartition(
    struct webrtc_RTPVideoHeaderVP8* self,
    int value) {
  auto header = reinterpret_cast<webrtc::RTPVideoHeaderVP8*>(self);
  header->beginningOfPartition = value != 0;
}
}
