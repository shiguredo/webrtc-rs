#include "rtp_parameters.h"

#include <stddef.h>
#include <string>
#include <vector>

// WebRTC
#include <api/rtp_parameters.h>

#include "../common.impl.h"
#include "../std.h"
#include "api/media_types.h"

// -------------------------
// webrtc::RtpCodecCapability
// -------------------------

WEBRTC_DEFINE_VECTOR(webrtc_RtpCodecCapability, webrtc::RtpCodecCapability);
struct std_string* webrtc_RtpCodecCapability_get_name(
    struct webrtc_RtpCodecCapability* self) {
  auto cap = reinterpret_cast<webrtc::RtpCodecCapability*>(self);
  return reinterpret_cast<std_string*>(&cap->name);
}
struct webrtc_RtpCodecCapability* webrtc_RtpCodecCapability_new() {
  auto cap = new webrtc::RtpCodecCapability();
  return reinterpret_cast<struct webrtc_RtpCodecCapability*>(cap);
}
void webrtc_RtpCodecCapability_delete(struct webrtc_RtpCodecCapability* self) {
  auto cap = reinterpret_cast<webrtc::RtpCodecCapability*>(self);
  delete cap;
}
void webrtc_RtpCodecCapability_set_kind(struct webrtc_RtpCodecCapability* self,
                                        int kind) {
  auto cap = reinterpret_cast<webrtc::RtpCodecCapability*>(self);
  cap->kind = static_cast<webrtc::MediaType>(kind);
}
void webrtc_RtpCodecCapability_set_name(struct webrtc_RtpCodecCapability* self,
                                        const char* name,
                                        size_t name_len) {
  auto cap = reinterpret_cast<webrtc::RtpCodecCapability*>(self);
  cap->name = name != nullptr ? std::string(name, name_len) : std::string();
}
void webrtc_RtpCodecCapability_set_clock_rate(
    struct webrtc_RtpCodecCapability* self,
    int clock_rate) {
  auto cap = reinterpret_cast<webrtc::RtpCodecCapability*>(self);
  cap->clock_rate = clock_rate;
}
struct std_map_string_string* webrtc_RtpCodecCapability_get_parameters(
    struct webrtc_RtpCodecCapability* self) {
  auto cap = reinterpret_cast<webrtc::RtpCodecCapability*>(self);
  return reinterpret_cast<struct std_map_string_string*>(&cap->parameters);
}

// -------------------------
// webrtc::RtpCapabilities
// -------------------------

void webrtc_RtpCapabilities_delete(struct webrtc_RtpCapabilities* self) {
  auto caps = reinterpret_cast<webrtc::RtpCapabilities*>(self);
  delete caps;
}
struct webrtc_RtpCodecCapability_vector* webrtc_RtpCapabilities_get_codecs(
    struct webrtc_RtpCapabilities* self) {
  auto caps = reinterpret_cast<webrtc::RtpCapabilities*>(self);
  return reinterpret_cast<struct webrtc_RtpCodecCapability_vector*>(
      &caps->codecs);
}

// -------------------------
// webrtc::RtpEncodingParameters
// -------------------------

WEBRTC_DEFINE_VECTOR(webrtc_RtpEncodingParameters,
                     webrtc::RtpEncodingParameters);

struct webrtc_RtpEncodingParameters* webrtc_RtpEncodingParameters_new() {
  auto params = new webrtc::RtpEncodingParameters();
  return reinterpret_cast<struct webrtc_RtpEncodingParameters*>(params);
}
void webrtc_RtpEncodingParameters_delete(
    struct webrtc_RtpEncodingParameters* self) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  delete params;
}
void webrtc_RtpEncodingParameters_set_rid(
    struct webrtc_RtpEncodingParameters* self,
    const char* rid,
    size_t rid_len) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  params->rid = rid != nullptr ? std::string(rid, rid_len) : std::string();
}
struct std_string* webrtc_RtpEncodingParameters_get_rid(
    struct webrtc_RtpEncodingParameters* self) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  return reinterpret_cast<struct std_string*>(&params->rid);
}
void webrtc_RtpEncodingParameters_set_scale_resolution_down_by(
    struct webrtc_RtpEncodingParameters* self,
    double scale_resolution_down_by) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  params->scale_resolution_down_by = scale_resolution_down_by;
}
int webrtc_RtpEncodingParameters_has_codec(
    struct webrtc_RtpEncodingParameters* self) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  return params->codec.has_value() ? 1 : 0;
}
struct webrtc_RtpCodecCapability* webrtc_RtpEncodingParameters_get_codec(
    struct webrtc_RtpEncodingParameters* self) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  if (!params->codec.has_value()) {
    return nullptr;
  }
  return reinterpret_cast<struct webrtc_RtpCodecCapability*>(
      &params->codec.value());
}
void webrtc_RtpEncodingParameters_set_codec(
    struct webrtc_RtpEncodingParameters* self,
    struct webrtc_RtpCodecCapability* codec) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  auto cap = reinterpret_cast<webrtc::RtpCodecCapability*>(codec);
  params->codec = *cap;
}
struct webrtc_RtpEncodingParameters_vector*
webrtc_RtpEncodingParameters_vector_clone(
    struct webrtc_RtpEncodingParameters_vector* src) {
  auto vec = reinterpret_cast<std::vector<webrtc::RtpEncodingParameters>*>(src);
  auto copy = new std::vector<webrtc::RtpEncodingParameters>(*vec);
  return reinterpret_cast<struct webrtc_RtpEncodingParameters_vector*>(copy);
}
