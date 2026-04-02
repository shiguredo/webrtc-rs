#include "rtp_parameters.h"

#include <assert.h>
#include <stddef.h>
#include <stdint.h>

#include <string>
#include <vector>

// WebRTC
#include <api/media_types.h>
#include <api/priority.h>
#include <api/rtp_parameters.h>
#include <api/video/resolution.h>

#include "../common.h"
#include "../common.impl.h"
#include "../std.h"
#include "../std.impl.h"

// -------------------------
// webrtc::RtpCodec
// -------------------------

WEBRTC_EXPORT struct webrtc_RtpCodec* webrtc_RtpCodec_new() {
  auto codec = new webrtc::RtpCodec();
  return reinterpret_cast<struct webrtc_RtpCodec*>(codec);
}
WEBRTC_EXPORT void webrtc_RtpCodec_delete(struct webrtc_RtpCodec* self) {
  auto codec = reinterpret_cast<webrtc::RtpCodec*>(self);
  delete codec;
}
WEBRTC_EXPORT void webrtc_RtpCodec_set_kind(struct webrtc_RtpCodec* self,
                                            int kind) {
  auto codec = reinterpret_cast<webrtc::RtpCodec*>(self);
  codec->kind = static_cast<webrtc::MediaType>(kind);
}
WEBRTC_EXPORT struct std_string* webrtc_RtpCodec_get_name(
    struct webrtc_RtpCodec* self) {
  auto codec = reinterpret_cast<webrtc::RtpCodec*>(self);
  return reinterpret_cast<struct std_string*>(&codec->name);
}
WEBRTC_EXPORT void webrtc_RtpCodec_set_name(struct webrtc_RtpCodec* self,
                                            const char* name,
                                            size_t name_len) {
  auto codec = reinterpret_cast<webrtc::RtpCodec*>(self);
  codec->name = name != nullptr ? std::string(name, name_len) : std::string();
}
WEBRTC_EXPORT void webrtc_RtpCodec_get_clock_rate(struct webrtc_RtpCodec* self,
                                                  int* out_has,
                                                  int* out_value) {
  auto codec = reinterpret_cast<webrtc::RtpCodec*>(self);
  webrtc_c::OptionalGet(codec->clock_rate, out_has, out_value);
}
WEBRTC_EXPORT void webrtc_RtpCodec_set_clock_rate(struct webrtc_RtpCodec* self,
                                                  int has,
                                                  const int* value) {
  auto codec = reinterpret_cast<webrtc::RtpCodec*>(self);
  webrtc_c::OptionalSet(codec->clock_rate, has, value);
}
WEBRTC_EXPORT void webrtc_RtpCodec_get_num_channels(
    struct webrtc_RtpCodec* self,
    int* out_has,
    int* out_value) {
  auto codec = reinterpret_cast<webrtc::RtpCodec*>(self);
  webrtc_c::OptionalGet(codec->num_channels, out_has, out_value);
}
WEBRTC_EXPORT void webrtc_RtpCodec_set_num_channels(
    struct webrtc_RtpCodec* self,
    int has,
    const int* value) {
  auto codec = reinterpret_cast<webrtc::RtpCodec*>(self);
  webrtc_c::OptionalSet(codec->num_channels, has, value);
}
WEBRTC_EXPORT struct std_map_string_string* webrtc_RtpCodec_get_parameters(
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

WEBRTC_EXPORT struct webrtc_RtpCodecCapability*
webrtc_RtpCodecCapability_new() {
  auto cap = new webrtc::RtpCodecCapability();
  return reinterpret_cast<struct webrtc_RtpCodecCapability*>(cap);
}
WEBRTC_EXPORT void webrtc_RtpCodecCapability_delete(
    struct webrtc_RtpCodecCapability* self) {
  auto cap = reinterpret_cast<webrtc::RtpCodecCapability*>(self);
  delete cap;
}

// -------------------------
// webrtc::RtpCapabilities
// -------------------------

WEBRTC_EXPORT void webrtc_RtpCapabilities_delete(
    struct webrtc_RtpCapabilities* self) {
  auto caps = reinterpret_cast<webrtc::RtpCapabilities*>(self);
  delete caps;
}
WEBRTC_EXPORT struct webrtc_RtpCodecCapability_vector*
webrtc_RtpCapabilities_get_codecs(struct webrtc_RtpCapabilities* self) {
  auto caps = reinterpret_cast<webrtc::RtpCapabilities*>(self);
  return reinterpret_cast<struct webrtc_RtpCodecCapability_vector*>(
      &caps->codecs);
}

// -------------------------
// webrtc::Resolution
// -------------------------

WEBRTC_EXPORT struct webrtc_Resolution* webrtc_Resolution_new() {
  auto resolution = new webrtc::Resolution();
  return reinterpret_cast<struct webrtc_Resolution*>(resolution);
}
WEBRTC_EXPORT void webrtc_Resolution_delete(struct webrtc_Resolution* self) {
  auto resolution = reinterpret_cast<webrtc::Resolution*>(self);
  delete resolution;
}
WEBRTC_EXPORT int webrtc_Resolution_get_width(struct webrtc_Resolution* self) {
  auto resolution = reinterpret_cast<webrtc::Resolution*>(self);
  return resolution->width;
}
WEBRTC_EXPORT void webrtc_Resolution_set_width(struct webrtc_Resolution* self,
                                               int width) {
  auto resolution = reinterpret_cast<webrtc::Resolution*>(self);
  resolution->width = width;
}
WEBRTC_EXPORT int webrtc_Resolution_get_height(struct webrtc_Resolution* self) {
  auto resolution = reinterpret_cast<webrtc::Resolution*>(self);
  return resolution->height;
}
WEBRTC_EXPORT void webrtc_Resolution_set_height(struct webrtc_Resolution* self,
                                                int height) {
  auto resolution = reinterpret_cast<webrtc::Resolution*>(self);
  resolution->height = height;
}

// -------------------------
// webrtc::RtpEncodingParameters
// -------------------------

WEBRTC_DEFINE_VECTOR(webrtc_RtpEncodingParameters,
                     webrtc::RtpEncodingParameters);

WEBRTC_EXPORT struct webrtc_RtpEncodingParameters*
webrtc_RtpEncodingParameters_new() {
  auto params = new webrtc::RtpEncodingParameters();
  return reinterpret_cast<struct webrtc_RtpEncodingParameters*>(params);
}
WEBRTC_EXPORT void webrtc_RtpEncodingParameters_delete(
    struct webrtc_RtpEncodingParameters* self) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  delete params;
}
WEBRTC_EXPORT void webrtc_RtpEncodingParameters_set_rid(
    struct webrtc_RtpEncodingParameters* self,
    const char* rid,
    size_t rid_len) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  params->rid = rid != nullptr ? std::string(rid, rid_len) : std::string();
}
WEBRTC_EXPORT struct std_string* webrtc_RtpEncodingParameters_get_rid(
    struct webrtc_RtpEncodingParameters* self) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  return reinterpret_cast<struct std_string*>(&params->rid);
}

WEBRTC_EXPORT void webrtc_RtpEncodingParameters_get_ssrc(
    struct webrtc_RtpEncodingParameters* self,
    int* out_has,
    uint32_t* out_value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  webrtc_c::OptionalGet(params->ssrc, out_has, out_value);
}
WEBRTC_EXPORT void webrtc_RtpEncodingParameters_set_ssrc(
    struct webrtc_RtpEncodingParameters* self,
    int has,
    const uint32_t* value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  webrtc_c::OptionalSet(params->ssrc, has, value);
}

WEBRTC_EXPORT void webrtc_RtpEncodingParameters_get_max_bitrate_bps(
    struct webrtc_RtpEncodingParameters* self,
    int* out_has,
    int* out_value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  webrtc_c::OptionalGet(params->max_bitrate_bps, out_has, out_value);
}
WEBRTC_EXPORT void webrtc_RtpEncodingParameters_set_max_bitrate_bps(
    struct webrtc_RtpEncodingParameters* self,
    int has,
    const int* value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  webrtc_c::OptionalSet(params->max_bitrate_bps, has, value);
}

WEBRTC_EXPORT void webrtc_RtpEncodingParameters_get_min_bitrate_bps(
    struct webrtc_RtpEncodingParameters* self,
    int* out_has,
    int* out_value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  webrtc_c::OptionalGet(params->min_bitrate_bps, out_has, out_value);
}
WEBRTC_EXPORT void webrtc_RtpEncodingParameters_set_min_bitrate_bps(
    struct webrtc_RtpEncodingParameters* self,
    int has,
    const int* value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  webrtc_c::OptionalSet(params->min_bitrate_bps, has, value);
}

WEBRTC_EXPORT void webrtc_RtpEncodingParameters_get_max_framerate(
    struct webrtc_RtpEncodingParameters* self,
    int* out_has,
    double* out_value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  webrtc_c::OptionalGet(params->max_framerate, out_has, out_value);
}
WEBRTC_EXPORT void webrtc_RtpEncodingParameters_set_max_framerate(
    struct webrtc_RtpEncodingParameters* self,
    int has,
    const double* value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  webrtc_c::OptionalSet(params->max_framerate, has, value);
}

WEBRTC_EXPORT void webrtc_RtpEncodingParameters_get_scale_resolution_down_by(
    struct webrtc_RtpEncodingParameters* self,
    int* out_has,
    double* out_value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  webrtc_c::OptionalGet(params->scale_resolution_down_by, out_has, out_value);
}
WEBRTC_EXPORT void webrtc_RtpEncodingParameters_set_scale_resolution_down_by(
    struct webrtc_RtpEncodingParameters* self,
    int has,
    const double* value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  webrtc_c::OptionalSet(params->scale_resolution_down_by, has, value);
}

WEBRTC_EXPORT void webrtc_RtpEncodingParameters_get_scale_resolution_down_to(
    struct webrtc_RtpEncodingParameters* self,
    int* out_has,
    struct webrtc_Resolution* out_value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  auto value = reinterpret_cast<webrtc::Resolution*>(out_value);
  webrtc_c::OptionalGet(params->scale_resolution_down_to, out_has, value);
}
WEBRTC_EXPORT void webrtc_RtpEncodingParameters_set_scale_resolution_down_to(
    struct webrtc_RtpEncodingParameters* self,
    int has,
    const struct webrtc_Resolution* value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  auto v = reinterpret_cast<const webrtc::Resolution*>(value);
  webrtc_c::OptionalSet(params->scale_resolution_down_to, has, v);
}

WEBRTC_EXPORT int webrtc_RtpEncodingParameters_get_active(
    struct webrtc_RtpEncodingParameters* self) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  return params->active ? 1 : 0;
}
WEBRTC_EXPORT void webrtc_RtpEncodingParameters_set_active(
    struct webrtc_RtpEncodingParameters* self,
    int active) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  params->active = active != 0;
}

WEBRTC_EXPORT int webrtc_RtpEncodingParameters_get_adaptive_ptime(
    struct webrtc_RtpEncodingParameters* self) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  return params->adaptive_ptime ? 1 : 0;
}
WEBRTC_EXPORT void webrtc_RtpEncodingParameters_set_adaptive_ptime(
    struct webrtc_RtpEncodingParameters* self,
    int adaptive_ptime) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  params->adaptive_ptime = adaptive_ptime != 0;
}

WEBRTC_EXPORT void webrtc_RtpEncodingParameters_get_scalability_mode(
    struct webrtc_RtpEncodingParameters* self,
    int* out_has,
    struct std_string** out_value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  webrtc_c::OptionalGetAs(params->scalability_mode, out_has, out_value, [&]() {
    return reinterpret_cast<struct std_string*>(
        &params->scalability_mode.value());
  });
}
WEBRTC_EXPORT void webrtc_RtpEncodingParameters_set_scalability_mode(
    struct webrtc_RtpEncodingParameters* self,
    int has,
    const char* value,
    size_t value_len) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  webrtc_c::OptionalSetAs(params->scalability_mode, has, value,
                          [&]() { return std::string(value, value_len); });
}

WEBRTC_EXPORT void webrtc_RtpEncodingParameters_get_codec(
    struct webrtc_RtpEncodingParameters* self,
    int* out_has,
    struct webrtc_RtpCodec** out_value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  webrtc_c::OptionalGetAs(params->codec, out_has, out_value, [&]() {
    return reinterpret_cast<struct webrtc_RtpCodec*>(&params->codec.value());
  });
}
WEBRTC_EXPORT void webrtc_RtpEncodingParameters_set_codec(
    struct webrtc_RtpEncodingParameters* self,
    int has,
    const struct webrtc_RtpCodec* value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  auto codec = reinterpret_cast<const webrtc::RtpCodec*>(value);
  webrtc_c::OptionalSet(params->codec, has, codec);
}

namespace {

// 未知の int は libwebrtc の `Priority` にそのままキャストできないため、kLow に寄せる（Rust の NetworkPriority::from_int と同じ既定）。
webrtc::Priority IntToNetworkPriority(int value) {
  switch (value) {
    case static_cast<int>(webrtc::Priority::kVeryLow):
      return webrtc::Priority::kVeryLow;
    case static_cast<int>(webrtc::Priority::kLow):
      return webrtc::Priority::kLow;
    case static_cast<int>(webrtc::Priority::kMedium):
      return webrtc::Priority::kMedium;
    case static_cast<int>(webrtc::Priority::kHigh):
      return webrtc::Priority::kHigh;
    default:
      return webrtc::Priority::kLow;
  }
}

}  // namespace

WEBRTC_EXPORT void webrtc_RtpEncodingParameters_get_bitrate_priority(
    struct webrtc_RtpEncodingParameters* self,
    double* out_value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  *out_value = params->bitrate_priority;
}
WEBRTC_EXPORT void webrtc_RtpEncodingParameters_set_bitrate_priority(
    struct webrtc_RtpEncodingParameters* self,
    double value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  params->bitrate_priority = value;
}

WEBRTC_EXPORT void webrtc_RtpEncodingParameters_get_network_priority(
    struct webrtc_RtpEncodingParameters* self,
    int* out_value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  *out_value = static_cast<int>(params->network_priority);
}
WEBRTC_EXPORT void webrtc_RtpEncodingParameters_set_network_priority(
    struct webrtc_RtpEncodingParameters* self,
    int value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  params->network_priority = IntToNetworkPriority(value);
}

WEBRTC_EXPORT int webrtc_RtpEncodingParameters_get_request_key_frame(
    struct webrtc_RtpEncodingParameters* self) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  return params->request_key_frame ? 1 : 0;
}
WEBRTC_EXPORT void webrtc_RtpEncodingParameters_set_request_key_frame(
    struct webrtc_RtpEncodingParameters* self,
    int request_key_frame) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  params->request_key_frame = request_key_frame != 0;
}

WEBRTC_EXPORT void webrtc_RtpEncodingParameters_get_num_temporal_layers(
    struct webrtc_RtpEncodingParameters* self,
    int* out_has,
    int* out_value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  webrtc_c::OptionalGet(params->num_temporal_layers, out_has, out_value);
}
WEBRTC_EXPORT void webrtc_RtpEncodingParameters_set_num_temporal_layers(
    struct webrtc_RtpEncodingParameters* self,
    int has,
    const int* value) {
  auto params = reinterpret_cast<webrtc::RtpEncodingParameters*>(self);
  webrtc_c::OptionalSet(params->num_temporal_layers, has, value);
}

WEBRTC_EXPORT struct webrtc_RtpEncodingParameters_vector*
webrtc_RtpEncodingParameters_vector_clone(
    struct webrtc_RtpEncodingParameters_vector* src) {
  auto vec = reinterpret_cast<std::vector<webrtc::RtpEncodingParameters>*>(src);
  auto copy = new std::vector<webrtc::RtpEncodingParameters>(*vec);
  return reinterpret_cast<struct webrtc_RtpEncodingParameters_vector*>(copy);
}

// -------------------------
// webrtc::Priority
// -------------------------

WEBRTC_EXPORT extern const int webrtc_Priority_kVeryLow =
    static_cast<int>(webrtc::Priority::kVeryLow);
WEBRTC_EXPORT extern const int webrtc_Priority_kLow =
    static_cast<int>(webrtc::Priority::kLow);
WEBRTC_EXPORT extern const int webrtc_Priority_kMedium =
    static_cast<int>(webrtc::Priority::kMedium);
WEBRTC_EXPORT extern const int webrtc_Priority_kHigh =
    static_cast<int>(webrtc::Priority::kHigh);

WEBRTC_EXPORT extern const double webrtc_kDefaultBitratePriority =
    webrtc::kDefaultBitratePriority;

// -------------------------
// webrtc::DegradationPreference
// -------------------------

WEBRTC_EXPORT extern const int webrtc_DegradationPreference_DISABLED =
    static_cast<int>(webrtc::DegradationPreference::DISABLED);
WEBRTC_EXPORT extern const int webrtc_DegradationPreference_MAINTAIN_FRAMERATE =
    static_cast<int>(webrtc::DegradationPreference::MAINTAIN_FRAMERATE);
WEBRTC_EXPORT extern const int
    webrtc_DegradationPreference_MAINTAIN_RESOLUTION =
        static_cast<int>(webrtc::DegradationPreference::MAINTAIN_RESOLUTION);
WEBRTC_EXPORT extern const int webrtc_DegradationPreference_BALANCED =
    static_cast<int>(webrtc::DegradationPreference::BALANCED);

// -------------------------
// webrtc::RtpParameters
// -------------------------

WEBRTC_EXPORT struct webrtc_RtpParameters* webrtc_RtpParameters_new() {
  auto params = new webrtc::RtpParameters();
  return reinterpret_cast<struct webrtc_RtpParameters*>(params);
}
WEBRTC_EXPORT void webrtc_RtpParameters_delete(
    struct webrtc_RtpParameters* self) {
  auto params = reinterpret_cast<webrtc::RtpParameters*>(self);
  delete params;
}

WEBRTC_EXPORT struct std_string* webrtc_RtpParameters_get_transaction_id(
    struct webrtc_RtpParameters* self) {
  auto params = reinterpret_cast<webrtc::RtpParameters*>(self);
  return reinterpret_cast<struct std_string*>(&params->transaction_id);
}
WEBRTC_EXPORT void webrtc_RtpParameters_set_transaction_id(
    struct webrtc_RtpParameters* self,
    const char* value,
    size_t value_len) {
  auto params = reinterpret_cast<webrtc::RtpParameters*>(self);
  params->transaction_id =
      value != nullptr ? std::string(value, value_len) : std::string();
}

WEBRTC_EXPORT struct std_string* webrtc_RtpParameters_get_mid(
    struct webrtc_RtpParameters* self) {
  auto params = reinterpret_cast<webrtc::RtpParameters*>(self);
  return reinterpret_cast<struct std_string*>(&params->mid);
}
WEBRTC_EXPORT void webrtc_RtpParameters_set_mid(
    struct webrtc_RtpParameters* self,
    const char* value,
    size_t value_len) {
  auto params = reinterpret_cast<webrtc::RtpParameters*>(self);
  params->mid =
      value != nullptr ? std::string(value, value_len) : std::string();
}

WEBRTC_EXPORT struct webrtc_RtpEncodingParameters_vector*
webrtc_RtpParameters_get_encodings(struct webrtc_RtpParameters* self) {
  auto params = reinterpret_cast<webrtc::RtpParameters*>(self);
  return reinterpret_cast<struct webrtc_RtpEncodingParameters_vector*>(
      &params->encodings);
}
WEBRTC_EXPORT void webrtc_RtpParameters_set_encodings(
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

WEBRTC_EXPORT void webrtc_RtpParameters_get_degradation_preference(
    struct webrtc_RtpParameters* self,
    int* out_has,
    int* out_value) {
  auto params = reinterpret_cast<webrtc::RtpParameters*>(self);
  webrtc_c::OptionalGetAs(
      params->degradation_preference, out_has, out_value, [&]() {
        return static_cast<int>(params->degradation_preference.value());
      });
}
WEBRTC_EXPORT void webrtc_RtpParameters_set_degradation_preference(
    struct webrtc_RtpParameters* self,
    int has,
    const int* value) {
  auto params = reinterpret_cast<webrtc::RtpParameters*>(self);
  webrtc_c::OptionalSetAs(params->degradation_preference, has, value, [&]() {
    return static_cast<webrtc::DegradationPreference>(*value);
  });
}
