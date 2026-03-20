#pragma once

#include <stddef.h>
#include <stdint.h>

#include "../common.h"
#include "../std.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::RtpCodec
// -------------------------

struct webrtc_RtpCodec;
struct webrtc_RtpCodec* WEBRTC_EXPORT webrtc_RtpCodec_new();
void WEBRTC_EXPORT webrtc_RtpCodec_delete(struct webrtc_RtpCodec* self);
void WEBRTC_EXPORT webrtc_RtpCodec_set_kind(struct webrtc_RtpCodec* self,
                                            int kind);
struct std_string* WEBRTC_EXPORT
webrtc_RtpCodec_get_name(struct webrtc_RtpCodec* self);
void WEBRTC_EXPORT webrtc_RtpCodec_set_name(struct webrtc_RtpCodec* self,
                                            const char* name,
                                            size_t name_len);
void WEBRTC_EXPORT webrtc_RtpCodec_get_clock_rate(struct webrtc_RtpCodec* self,
                                                  int* out_has,
                                                  int* out_value);
void WEBRTC_EXPORT webrtc_RtpCodec_set_clock_rate(struct webrtc_RtpCodec* self,
                                                  int has,
                                                  const int* value);
void WEBRTC_EXPORT
webrtc_RtpCodec_get_num_channels(struct webrtc_RtpCodec* self,
                                 int* out_has,
                                 int* out_value);
void WEBRTC_EXPORT
webrtc_RtpCodec_set_num_channels(struct webrtc_RtpCodec* self,
                                 int has,
                                 const int* value);
struct std_map_string_string* WEBRTC_EXPORT
webrtc_RtpCodec_get_parameters(struct webrtc_RtpCodec* self);

// -------------------------
// webrtc::RtpCodecCapability
// -------------------------

WEBRTC_DECLARE_VECTOR(webrtc_RtpCodecCapability);
WEBRTC_DECLARE_CAST(webrtc_RtpCodecCapability, webrtc_RtpCodec);
struct webrtc_RtpCodecCapability* WEBRTC_EXPORT webrtc_RtpCodecCapability_new();
void WEBRTC_EXPORT
webrtc_RtpCodecCapability_delete(struct webrtc_RtpCodecCapability* self);

// -------------------------
// webrtc::RtpCapabilities
// -------------------------

struct webrtc_RtpCapabilities;
void WEBRTC_EXPORT
webrtc_RtpCapabilities_delete(struct webrtc_RtpCapabilities* self);
struct webrtc_RtpCodecCapability_vector* WEBRTC_EXPORT
webrtc_RtpCapabilities_get_codecs(struct webrtc_RtpCapabilities* self);

// -------------------------
// webrtc::Resolution
// -------------------------

struct webrtc_Resolution;
struct webrtc_Resolution* WEBRTC_EXPORT webrtc_Resolution_new();
void WEBRTC_EXPORT webrtc_Resolution_delete(struct webrtc_Resolution* self);
int WEBRTC_EXPORT webrtc_Resolution_get_width(struct webrtc_Resolution* self);
void WEBRTC_EXPORT webrtc_Resolution_set_width(struct webrtc_Resolution* self,
                                               int width);
int WEBRTC_EXPORT webrtc_Resolution_get_height(struct webrtc_Resolution* self);
void WEBRTC_EXPORT webrtc_Resolution_set_height(struct webrtc_Resolution* self,
                                                int height);

// -------------------------
// webrtc::RtpEncodingParameters
// -------------------------

WEBRTC_DECLARE_VECTOR(webrtc_RtpEncodingParameters);
struct webrtc_RtpEncodingParameters* WEBRTC_EXPORT
webrtc_RtpEncodingParameters_new();
void WEBRTC_EXPORT
webrtc_RtpEncodingParameters_delete(struct webrtc_RtpEncodingParameters*);

void WEBRTC_EXPORT
webrtc_RtpEncodingParameters_set_rid(struct webrtc_RtpEncodingParameters*,
                                     const char* rid,
                                     size_t rid_len);
struct std_string* WEBRTC_EXPORT
webrtc_RtpEncodingParameters_get_rid(struct webrtc_RtpEncodingParameters* self);

void WEBRTC_EXPORT
webrtc_RtpEncodingParameters_get_ssrc(struct webrtc_RtpEncodingParameters* self,
                                      int* out_has,
                                      uint32_t* out_value);
void WEBRTC_EXPORT
webrtc_RtpEncodingParameters_set_ssrc(struct webrtc_RtpEncodingParameters* self,
                                      int has,
                                      const uint32_t* value);

void WEBRTC_EXPORT webrtc_RtpEncodingParameters_get_max_bitrate_bps(
    struct webrtc_RtpEncodingParameters* self,
    int* out_has,
    int* out_value);
void WEBRTC_EXPORT webrtc_RtpEncodingParameters_set_max_bitrate_bps(
    struct webrtc_RtpEncodingParameters* self,
    int has,
    const int* value);

void WEBRTC_EXPORT webrtc_RtpEncodingParameters_get_min_bitrate_bps(
    struct webrtc_RtpEncodingParameters* self,
    int* out_has,
    int* out_value);
void WEBRTC_EXPORT webrtc_RtpEncodingParameters_set_min_bitrate_bps(
    struct webrtc_RtpEncodingParameters* self,
    int has,
    const int* value);

void WEBRTC_EXPORT webrtc_RtpEncodingParameters_get_max_framerate(
    struct webrtc_RtpEncodingParameters* self,
    int* out_has,
    double* out_value);
void WEBRTC_EXPORT webrtc_RtpEncodingParameters_set_max_framerate(
    struct webrtc_RtpEncodingParameters* self,
    int has,
    const double* value);

void WEBRTC_EXPORT webrtc_RtpEncodingParameters_get_scale_resolution_down_by(
    struct webrtc_RtpEncodingParameters* self,
    int* out_has,
    double* out_value);
void WEBRTC_EXPORT webrtc_RtpEncodingParameters_set_scale_resolution_down_by(
    struct webrtc_RtpEncodingParameters* self,
    int has,
    const double* value);

void WEBRTC_EXPORT webrtc_RtpEncodingParameters_get_scale_resolution_down_to(
    struct webrtc_RtpEncodingParameters* self,
    int* out_has,
    struct webrtc_Resolution* out_value);
void WEBRTC_EXPORT webrtc_RtpEncodingParameters_set_scale_resolution_down_to(
    struct webrtc_RtpEncodingParameters* self,
    int has,
    const struct webrtc_Resolution* value);

int WEBRTC_EXPORT webrtc_RtpEncodingParameters_get_active(
    struct webrtc_RtpEncodingParameters* self);
void WEBRTC_EXPORT webrtc_RtpEncodingParameters_set_active(
    struct webrtc_RtpEncodingParameters* self,
    int active);

int WEBRTC_EXPORT webrtc_RtpEncodingParameters_get_adaptive_ptime(
    struct webrtc_RtpEncodingParameters* self);
void WEBRTC_EXPORT webrtc_RtpEncodingParameters_set_adaptive_ptime(
    struct webrtc_RtpEncodingParameters* self,
    int adaptive_ptime);

void WEBRTC_EXPORT webrtc_RtpEncodingParameters_get_scalability_mode(
    struct webrtc_RtpEncodingParameters* self,
    int* out_has,
    struct std_string** out_value);
void WEBRTC_EXPORT webrtc_RtpEncodingParameters_set_scalability_mode(
    struct webrtc_RtpEncodingParameters* self,
    int has,
    const char* value,
    size_t value_len);

void WEBRTC_EXPORT webrtc_RtpEncodingParameters_get_codec(
    struct webrtc_RtpEncodingParameters* self,
    int* out_has,
    struct webrtc_RtpCodec** out_value);
void WEBRTC_EXPORT webrtc_RtpEncodingParameters_set_codec(
    struct webrtc_RtpEncodingParameters* self,
    int has,
    const struct webrtc_RtpCodec* value);

struct webrtc_RtpEncodingParameters_vector* WEBRTC_EXPORT
webrtc_RtpEncodingParameters_vector_clone(
    struct webrtc_RtpEncodingParameters_vector* src);

// -------------------------
// webrtc::DegradationPreference
// -------------------------

WEBRTC_EXPORT extern const int webrtc_DegradationPreference_DISABLED;
WEBRTC_EXPORT extern const int webrtc_DegradationPreference_MAINTAIN_FRAMERATE;
WEBRTC_EXPORT extern const int webrtc_DegradationPreference_MAINTAIN_RESOLUTION;
WEBRTC_EXPORT extern const int webrtc_DegradationPreference_BALANCED;

// -------------------------
// webrtc::RtpParameters
// -------------------------

struct webrtc_RtpParameters;
struct webrtc_RtpParameters* WEBRTC_EXPORT webrtc_RtpParameters_new();
void WEBRTC_EXPORT
webrtc_RtpParameters_delete(struct webrtc_RtpParameters* self);

struct std_string* WEBRTC_EXPORT
webrtc_RtpParameters_get_transaction_id(struct webrtc_RtpParameters* self);
void WEBRTC_EXPORT
webrtc_RtpParameters_set_transaction_id(struct webrtc_RtpParameters* self,
                                        const char* value,
                                        size_t value_len);

struct std_string* WEBRTC_EXPORT
webrtc_RtpParameters_get_mid(struct webrtc_RtpParameters* self);
void WEBRTC_EXPORT
webrtc_RtpParameters_set_mid(struct webrtc_RtpParameters* self,
                             const char* value,
                             size_t value_len);

struct webrtc_RtpEncodingParameters_vector* WEBRTC_EXPORT
webrtc_RtpParameters_get_encodings(struct webrtc_RtpParameters* self);
void WEBRTC_EXPORT webrtc_RtpParameters_set_encodings(
    struct webrtc_RtpParameters* self,
    struct webrtc_RtpEncodingParameters_vector* encodings);

void WEBRTC_EXPORT webrtc_RtpParameters_get_degradation_preference(
    struct webrtc_RtpParameters* self,
    int* out_has,
    int* out_value);
void WEBRTC_EXPORT webrtc_RtpParameters_set_degradation_preference(
    struct webrtc_RtpParameters* self,
    int has,
    const int* value);

#if defined(__cplusplus)
}
#endif
