#include "rtp_parameters.h"

#include <assert.h>
#include <stddef.h>
#include <stdint.h>

#include <string>
#include <vector>

// WebRTC
#include <api/media_types.h>
#include <api/rtp_parameters.h>
#include <api/video/resolution.h>

#include "../common.impl.h"
#include "../std.h"
#include "../std.impl.h"

// -------------------------
// webrtc::RtpCodec
// -------------------------

struct webrtc_RtpCodec* webrtc_RtpCodec_new() {
  auto codec = new webrtc::RtpCodec();
  return reinterpret_cast<struct webrtc_RtpCodec*>(codec);
}
void webrtc_RtpCodec_delete(struct webrtc_RtpCodec* self) {
  auto codec = reinterpret_cast<webrtc::RtpCodec*>(self);
  delete codec;
}
void webrtc_RtpCodec_set_kind(struct webrtc_RtpCodec* self, int kind) {
  auto codec = reinterpret_cast<webrtc::RtpCodec*>(self);
  codec->kind = static_cast<webrtc::MediaType>(kind);
}
struct std_string* webrtc_RtpCodec_get_name(struct webrtc_RtpCodec* self) {
  auto codec = reinterpret_cast<webrtc::RtpCodec*>(self);
  return reinterpret_cast<struct std_string*>(&codec->name);
}
void webrtc_RtpCodec_set_name(struct webrtc_RtpCodec* self,
                              const char* name,
                              size_t name_len) {
  auto codec = reinterpret_cast<webrtc::RtpCodec*>(self);
  codec->name = name != nullptr ? std::string(name, name_len) : std::string();
}
void webrtc_RtpCodec_get_clock_rate(struct webrtc_RtpCodec* self,
                                    int* out_has,
                                    int* out_value) {
  auto codec = reinterpret_cast<webrtc::RtpCodec*>(self);
  webrtc_c::OptionalGet(codec->clock_rate, out_has, out_value);
}
void webrtc_RtpCodec_set_clock_rate(struct webrtc_RtpCodec* self,
                                    int has,
                                    const int* value) {
  auto codec = reinterpret_cast<webrtc::RtpCodec*>(self);
  webrtc_c::OptionalSet(codec->clock_rate, has, value);
}
void webrtc_RtpCodec_get_num_channels(struct webrtc_RtpCodec* self,
                                      int* out_has,
                                      int* out_value) {
  auto codec = reinterpret_cast<webrtc::RtpCodec*>(self);
  webrtc_c::OptionalGet(codec->num_channels, out_has, out_value);
}
void webrtc_RtpCodec_set_num_channels(struct webrtc_RtpCodec* self,
                                      int has,
                                      const int* value) {
  auto codec = reinterpret_cast<webrtc::RtpCodec*>(self);
  webrtc_c::OptionalSet(codec->num_channels, has, value);
}
struct std_map_string_string* webrtc_RtpCodec_get_parameters(
    struct webrtc_RtpCodec* self) {
  auto codec = reinterpret_cast<webrtc::RtpCodec*>(self);
  return reinterpret_cast<struct std_map_string_string*>(&codec->parameters);
}

// -------------------------
// webrtc::RtpCodecCapability
// -------------------------

WEBRTC_DEFINE_VECTOR(webrtc_RtpCodecCapability, webrtc::RtpCodecCapability);
WEBRTC_DEFINE_CAST(webrtc_RtpCodecCapability,
                   webrtc_RtpCodec,
                   webrtc::RtpCodecCapability,
                   webrtc::RtpCodec);

struct webrtc_RtpCodecCapability* webrtc_RtpCodecCapability_new() {
  auto cap = new webrtc::RtpCodecCapability();
  return reinterpret_cast<struct webrtc_RtpCodecCapability*>(cap);
}
void webrtc_RtpCodecCapability_delete(struct webrtc_RtpCodecCapability* self) {
  auto cap = reinterpret_cast<webrtc::RtpCodecCapability*>(self);
  delete cap;
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
// webrtc::Resolution
// -------------------------

struct webrtc_Resolution* webrtc_Resolution_new() {
  auto resolution = new webrtc::Resolution();
  return reinterpret_cast<struct webrtc_Resolution*>(resolution);
}
void webrtc_Resolution_delete(struct webrtc_Resolution* self) {
  auto resolution = reinterpret_cast<webrtc::Resolution*>(self);
  delete resolution;
}
int webrtc_Resolution_get_width(struct webrtc_Resolution* self) {
  auto resolution = reinterpret_cast<webrtc::Resolution*>(self);
  return resolution->width;
}
void webrtc_Resolution_set_width(struct webrtc_Resolution* self, int width) {
  auto resolution = reinterpret_cast<webrtc::Resolution*>(self);
  resolution->width = width;
}
int webrtc_Resolution_get_height(struct webrtc_Resolution* self) {
  auto resolution = reinterpret_cast<webrtc::Resolution*>(self);
  return resolution->height;
}
void webrtc_Resolution_set_height(struct webrtc_Resolution* self, int height) {
  auto resolution = reinterpret_cast<webrtc::Resolution*>(self);
  resolution->height = height;
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

void webrtc_RtpEncodingParameters_get_ssrc(
    struct webrtc_RtpEncodingParameters* self,
    int* out_has,
    uint32_t* out_value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  webrtc_c::OptionalGet(params->ssrc, out_has, out_value);
}
void webrtc_RtpEncodingParameters_set_ssrc(
    struct webrtc_RtpEncodingParameters* self,
    int has,
    const uint32_t* value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  webrtc_c::OptionalSet(params->ssrc, has, value);
}

void webrtc_RtpEncodingParameters_get_max_bitrate_bps(
    struct webrtc_RtpEncodingParameters* self,
    int* out_has,
    int* out_value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  webrtc_c::OptionalGet(params->max_bitrate_bps, out_has, out_value);
}
void webrtc_RtpEncodingParameters_set_max_bitrate_bps(
    struct webrtc_RtpEncodingParameters* self,
    int has,
    const int* value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  webrtc_c::OptionalSet(params->max_bitrate_bps, has, value);
}

void webrtc_RtpEncodingParameters_get_min_bitrate_bps(
    struct webrtc_RtpEncodingParameters* self,
    int* out_has,
    int* out_value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  webrtc_c::OptionalGet(params->min_bitrate_bps, out_has, out_value);
}
void webrtc_RtpEncodingParameters_set_min_bitrate_bps(
    struct webrtc_RtpEncodingParameters* self,
    int has,
    const int* value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  webrtc_c::OptionalSet(params->min_bitrate_bps, has, value);
}

void webrtc_RtpEncodingParameters_get_max_framerate(
    struct webrtc_RtpEncodingParameters* self,
    int* out_has,
    double* out_value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  webrtc_c::OptionalGet(params->max_framerate, out_has, out_value);
}
void webrtc_RtpEncodingParameters_set_max_framerate(
    struct webrtc_RtpEncodingParameters* self,
    int has,
    const double* value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  webrtc_c::OptionalSet(params->max_framerate, has, value);
}

void webrtc_RtpEncodingParameters_get_scale_resolution_down_by(
    struct webrtc_RtpEncodingParameters* self,
    int* out_has,
    double* out_value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  webrtc_c::OptionalGet(params->scale_resolution_down_by, out_has, out_value);
}
void webrtc_RtpEncodingParameters_set_scale_resolution_down_by(
    struct webrtc_RtpEncodingParameters* self,
    int has,
    const double* value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  webrtc_c::OptionalSet(params->scale_resolution_down_by, has, value);
}

void webrtc_RtpEncodingParameters_get_scale_resolution_down_to(
    struct webrtc_RtpEncodingParameters* self,
    int* out_has,
    struct webrtc_Resolution* out_value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  auto value = reinterpret_cast<webrtc::Resolution*>(out_value);
  webrtc_c::OptionalGet(params->scale_resolution_down_to, out_has, value);
}
void webrtc_RtpEncodingParameters_set_scale_resolution_down_to(
    struct webrtc_RtpEncodingParameters* self,
    int has,
    const struct webrtc_Resolution* value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  auto v = reinterpret_cast<const webrtc::Resolution*>(value);
  webrtc_c::OptionalSet(params->scale_resolution_down_to, has, v);
}

int webrtc_RtpEncodingParameters_get_active(
    struct webrtc_RtpEncodingParameters* self) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  return params->active ? 1 : 0;
}
void webrtc_RtpEncodingParameters_set_active(
    struct webrtc_RtpEncodingParameters* self,
    int active) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  params->active = active != 0;
}

int webrtc_RtpEncodingParameters_get_adaptive_ptime(
    struct webrtc_RtpEncodingParameters* self) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  return params->adaptive_ptime ? 1 : 0;
}
void webrtc_RtpEncodingParameters_set_adaptive_ptime(
    struct webrtc_RtpEncodingParameters* self,
    int adaptive_ptime) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  params->adaptive_ptime = adaptive_ptime != 0;
}

void webrtc_RtpEncodingParameters_get_scalability_mode(
    struct webrtc_RtpEncodingParameters* self,
    int* out_has,
    struct std_string** out_value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  webrtc_c::OptionalGetAs(params->scalability_mode, out_has, out_value, [&]() {
    return reinterpret_cast<struct std_string*>(
        const_cast<std::string*>(&params->scalability_mode.value()));
  });
}
void webrtc_RtpEncodingParameters_set_scalability_mode(
    struct webrtc_RtpEncodingParameters* self,
    int has,
    const char* value,
    size_t value_len) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  webrtc_c::OptionalSetAs(params->scalability_mode, has, value,
                          [&]() { return std::string(value, value_len); });
}

void webrtc_RtpEncodingParameters_get_codec(
    struct webrtc_RtpEncodingParameters* self,
    int* out_has,
    struct webrtc_RtpCodec** out_value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  webrtc_c::OptionalGetAs(params->codec, out_has, out_value, [&]() {
    return reinterpret_cast<struct webrtc_RtpCodec*>(&params->codec.value());
  });
}
void webrtc_RtpEncodingParameters_set_codec(
    struct webrtc_RtpEncodingParameters* self,
    int has,
    const struct webrtc_RtpCodec* value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  auto codec = reinterpret_cast<const webrtc::RtpCodec*>(value);
  webrtc_c::OptionalSet(params->codec, has, codec);
}

struct webrtc_RtpEncodingParameters_vector*
webrtc_RtpEncodingParameters_vector_clone(
    struct webrtc_RtpEncodingParameters_vector* src) {
  auto vec = reinterpret_cast<std::vector<webrtc::RtpEncodingParameters>*>(src);
  auto copy = new std::vector<webrtc::RtpEncodingParameters>(*vec);
  return reinterpret_cast<struct webrtc_RtpEncodingParameters_vector*>(copy);
}

// -------------------------
// webrtc::DegradationPreference
// -------------------------

extern const int webrtc_DegradationPreference_DISABLED =
    static_cast<int>(webrtc::DegradationPreference::DISABLED);
extern const int webrtc_DegradationPreference_MAINTAIN_FRAMERATE =
    static_cast<int>(webrtc::DegradationPreference::MAINTAIN_FRAMERATE);
extern const int webrtc_DegradationPreference_MAINTAIN_RESOLUTION =
    static_cast<int>(webrtc::DegradationPreference::MAINTAIN_RESOLUTION);
extern const int webrtc_DegradationPreference_BALANCED =
    static_cast<int>(webrtc::DegradationPreference::BALANCED);

// -------------------------
// webrtc::RtpParameters
// -------------------------

struct webrtc_RtpParameters* webrtc_RtpParameters_new() {
  auto params = new webrtc::RtpParameters();
  return reinterpret_cast<struct webrtc_RtpParameters*>(params);
}
void webrtc_RtpParameters_delete(struct webrtc_RtpParameters* self) {
  auto params = reinterpret_cast<webrtc::RtpParameters*>(self);
  delete params;
}

struct std_string* webrtc_RtpParameters_get_transaction_id(
    struct webrtc_RtpParameters* self) {
  auto params = reinterpret_cast<webrtc::RtpParameters*>(self);
  return reinterpret_cast<struct std_string*>(&params->transaction_id);
}
void webrtc_RtpParameters_set_transaction_id(struct webrtc_RtpParameters* self,
                                             const char* value,
                                             size_t value_len) {
  auto params = reinterpret_cast<webrtc::RtpParameters*>(self);
  params->transaction_id =
      value != nullptr ? std::string(value, value_len) : std::string();
}

struct std_string* webrtc_RtpParameters_get_mid(
    struct webrtc_RtpParameters* self) {
  auto params = reinterpret_cast<webrtc::RtpParameters*>(self);
  return reinterpret_cast<struct std_string*>(&params->mid);
}
void webrtc_RtpParameters_set_mid(struct webrtc_RtpParameters* self,
                                  const char* value,
                                  size_t value_len) {
  auto params = reinterpret_cast<webrtc::RtpParameters*>(self);
  params->mid =
      value != nullptr ? std::string(value, value_len) : std::string();
}

struct webrtc_RtpEncodingParameters_vector* webrtc_RtpParameters_get_encodings(
    struct webrtc_RtpParameters* self) {
  auto params = reinterpret_cast<webrtc::RtpParameters*>(self);
  return reinterpret_cast<struct webrtc_RtpEncodingParameters_vector*>(
      &params->encodings);
}
void webrtc_RtpParameters_set_encodings(
    struct webrtc_RtpParameters* self,
    struct webrtc_RtpEncodingParameters_vector* encodings) {
  auto params = reinterpret_cast<webrtc::RtpParameters*>(self);
  if (encodings == nullptr) {
    params->encodings.clear();
    return;
  }
  auto vec =
      reinterpret_cast<std::vector<webrtc::RtpEncodingParameters>*>(encodings);
  params->encodings = *vec;
}

void webrtc_RtpParameters_get_degradation_preference(
    struct webrtc_RtpParameters* self,
    int* out_has,
    int* out_value) {
  auto params = reinterpret_cast<webrtc::RtpParameters*>(self);
  webrtc_c::OptionalGetAs(
      params->degradation_preference, out_has, out_value, [&]() {
        return static_cast<int>(params->degradation_preference.value());
      });
}
void webrtc_RtpParameters_set_degradation_preference(
    struct webrtc_RtpParameters* self,
    int has,
    const int* value) {
  auto params = reinterpret_cast<webrtc::RtpParameters*>(self);
  webrtc_c::OptionalSetAs(params->degradation_preference, has, value, [&]() {
    return static_cast<webrtc::DegradationPreference>(*value);
  });
}
