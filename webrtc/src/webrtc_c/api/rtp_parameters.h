#pragma once

#include <stddef.h>

#include "../common.h"
#include "../std.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::RtpCodecCapability
// -------------------------

WEBRTC_DECLARE_VECTOR(webrtc_RtpCodecCapability);
struct std_string* webrtc_RtpCodecCapability_get_name(
    struct webrtc_RtpCodecCapability* self);
struct webrtc_RtpCodecCapability* webrtc_RtpCodecCapability_new();
void webrtc_RtpCodecCapability_delete(struct webrtc_RtpCodecCapability* self);
void webrtc_RtpCodecCapability_set_kind(struct webrtc_RtpCodecCapability* self,
                                        int kind);
void webrtc_RtpCodecCapability_set_name(struct webrtc_RtpCodecCapability* self,
                                        const char* name,
                                        size_t name_len);
void webrtc_RtpCodecCapability_set_clock_rate(
    struct webrtc_RtpCodecCapability* self,
    int clock_rate);
struct std_map_string_string* webrtc_RtpCodecCapability_get_parameters(
    struct webrtc_RtpCodecCapability* self);

// -------------------------
// webrtc::RtpCapabilities
// -------------------------

struct webrtc_RtpCapabilities;
void webrtc_RtpCapabilities_delete(struct webrtc_RtpCapabilities* self);
struct webrtc_RtpCodecCapability_vector* webrtc_RtpCapabilities_get_codecs(
    struct webrtc_RtpCapabilities* self);

// -------------------------
// webrtc::RtpEncodingParameters
// -------------------------

WEBRTC_DECLARE_VECTOR(webrtc_RtpEncodingParameters);
struct webrtc_RtpEncodingParameters* webrtc_RtpEncodingParameters_new();
void webrtc_RtpEncodingParameters_delete(struct webrtc_RtpEncodingParameters*);
void webrtc_RtpEncodingParameters_set_rid(struct webrtc_RtpEncodingParameters*,
                                          const char* rid,
                                          size_t rid_len);
struct std_string* webrtc_RtpEncodingParameters_get_rid(
    struct webrtc_RtpEncodingParameters* self);
void webrtc_RtpEncodingParameters_set_scale_resolution_down_by(
    struct webrtc_RtpEncodingParameters* self,
    double scale_resolution_down_by);
int webrtc_RtpEncodingParameters_has_codec(
    struct webrtc_RtpEncodingParameters* self);
struct webrtc_RtpCodecCapability* webrtc_RtpEncodingParameters_get_codec(
    struct webrtc_RtpEncodingParameters* self);
void webrtc_RtpEncodingParameters_set_codec(
    struct webrtc_RtpEncodingParameters* self,
    struct webrtc_RtpCodecCapability* codec);
struct webrtc_RtpEncodingParameters_vector*
webrtc_RtpEncodingParameters_vector_clone(
    struct webrtc_RtpEncodingParameters_vector* src);

#if defined(__cplusplus)
}
#endif
