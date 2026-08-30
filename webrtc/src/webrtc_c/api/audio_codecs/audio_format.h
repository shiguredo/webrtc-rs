#pragma once

#include <stddef.h>

#include "../../common.h"
#include "../../std.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::SdpAudioFormat
// -------------------------

WEBRTC_DECLARE_UNIQUE(webrtc_SdpAudioFormat);
WEBRTC_DECLARE_VECTOR_NO_DEFAULT_CTOR(webrtc_SdpAudioFormat);
WEBRTC_EXPORT struct webrtc_SdpAudioFormat_unique* webrtc_SdpAudioFormat_new(
    const char* name,
    size_t name_len,
    int clockrate_hz,
    size_t num_channels);
WEBRTC_EXPORT struct webrtc_SdpAudioFormat_unique*
webrtc_SdpAudioFormat_new_with_parameters(
    const char* name,
    size_t name_len,
    int clockrate_hz,
    size_t num_channels,
    struct std_map_string_string* parameters);
WEBRTC_EXPORT struct webrtc_SdpAudioFormat_unique* webrtc_SdpAudioFormat_copy(
    const struct webrtc_SdpAudioFormat* self);
WEBRTC_EXPORT struct std_string* webrtc_SdpAudioFormat_get_name(
    struct webrtc_SdpAudioFormat* self);
WEBRTC_EXPORT void webrtc_SdpAudioFormat_set_name(
    struct webrtc_SdpAudioFormat* self,
    const struct std_string* name);
WEBRTC_EXPORT int webrtc_SdpAudioFormat_get_clockrate_hz(
    const struct webrtc_SdpAudioFormat* self);
WEBRTC_EXPORT void webrtc_SdpAudioFormat_set_clockrate_hz(
    struct webrtc_SdpAudioFormat* self,
    int value);
WEBRTC_EXPORT size_t webrtc_SdpAudioFormat_get_num_channels(
    const struct webrtc_SdpAudioFormat* self);
WEBRTC_EXPORT void webrtc_SdpAudioFormat_set_num_channels(
    struct webrtc_SdpAudioFormat* self,
    size_t value);
WEBRTC_EXPORT struct std_map_string_string*
webrtc_SdpAudioFormat_get_parameters(struct webrtc_SdpAudioFormat* self);
WEBRTC_EXPORT void webrtc_SdpAudioFormat_set_parameters(
    struct webrtc_SdpAudioFormat* self,
    struct std_map_string_string* parameters);
WEBRTC_EXPORT int webrtc_SdpAudioFormat_is_equal(
    const struct webrtc_SdpAudioFormat* lhs,
    const struct webrtc_SdpAudioFormat* rhs);
WEBRTC_EXPORT int webrtc_SdpAudioFormat_Matches(
    const struct webrtc_SdpAudioFormat* self,
    const struct webrtc_SdpAudioFormat* other);

// -------------------------
// webrtc::AudioCodecInfo
// -------------------------

struct webrtc_AudioCodecInfo;
WEBRTC_EXPORT struct webrtc_AudioCodecInfo* webrtc_AudioCodecInfo_new(
    int sample_rate_hz,
    size_t num_channels,
    int default_bitrate_bps,
    int min_bitrate_bps,
    int max_bitrate_bps);
WEBRTC_EXPORT void webrtc_AudioCodecInfo_delete(
    struct webrtc_AudioCodecInfo* self);
WEBRTC_EXPORT struct webrtc_AudioCodecInfo* webrtc_AudioCodecInfo_copy(
    const struct webrtc_AudioCodecInfo* self);
WEBRTC_EXPORT int webrtc_AudioCodecInfo_get_sample_rate_hz(
    const struct webrtc_AudioCodecInfo* self);
WEBRTC_EXPORT void webrtc_AudioCodecInfo_set_sample_rate_hz(
    struct webrtc_AudioCodecInfo* self,
    int value);
WEBRTC_EXPORT size_t webrtc_AudioCodecInfo_get_num_channels(
    const struct webrtc_AudioCodecInfo* self);
WEBRTC_EXPORT void webrtc_AudioCodecInfo_set_num_channels(
    struct webrtc_AudioCodecInfo* self,
    size_t value);
WEBRTC_EXPORT int webrtc_AudioCodecInfo_get_default_bitrate_bps(
    const struct webrtc_AudioCodecInfo* self);
WEBRTC_EXPORT void webrtc_AudioCodecInfo_set_default_bitrate_bps(
    struct webrtc_AudioCodecInfo* self,
    int value);
WEBRTC_EXPORT int webrtc_AudioCodecInfo_get_min_bitrate_bps(
    const struct webrtc_AudioCodecInfo* self);
WEBRTC_EXPORT void webrtc_AudioCodecInfo_set_min_bitrate_bps(
    struct webrtc_AudioCodecInfo* self,
    int value);
WEBRTC_EXPORT int webrtc_AudioCodecInfo_get_max_bitrate_bps(
    const struct webrtc_AudioCodecInfo* self);
WEBRTC_EXPORT void webrtc_AudioCodecInfo_set_max_bitrate_bps(
    struct webrtc_AudioCodecInfo* self,
    int value);
WEBRTC_EXPORT int webrtc_AudioCodecInfo_get_allow_comfort_noise(
    const struct webrtc_AudioCodecInfo* self);
WEBRTC_EXPORT void webrtc_AudioCodecInfo_set_allow_comfort_noise(
    struct webrtc_AudioCodecInfo* self,
    int value);
WEBRTC_EXPORT int webrtc_AudioCodecInfo_get_supports_network_adaption(
    const struct webrtc_AudioCodecInfo* self);
WEBRTC_EXPORT void webrtc_AudioCodecInfo_set_supports_network_adaption(
    struct webrtc_AudioCodecInfo* self,
    int value);

// -------------------------
// webrtc::AudioCodecSpec
// -------------------------

struct webrtc_AudioCodecSpec;
WEBRTC_DECLARE_VECTOR_NO_DEFAULT_CTOR(webrtc_AudioCodecSpec);
WEBRTC_EXPORT struct webrtc_AudioCodecSpec* webrtc_AudioCodecSpec_new(
    struct webrtc_SdpAudioFormat* format,
    struct webrtc_AudioCodecInfo* info);
WEBRTC_EXPORT void webrtc_AudioCodecSpec_delete(
    struct webrtc_AudioCodecSpec* self);
WEBRTC_EXPORT struct webrtc_AudioCodecSpec* webrtc_AudioCodecSpec_copy(
    const struct webrtc_AudioCodecSpec* self);
WEBRTC_EXPORT void webrtc_AudioCodecSpec_set_format(
    struct webrtc_AudioCodecSpec* self,
    struct webrtc_SdpAudioFormat* format);
WEBRTC_EXPORT struct webrtc_SdpAudioFormat* webrtc_AudioCodecSpec_get_format(
    struct webrtc_AudioCodecSpec* self);
WEBRTC_EXPORT void webrtc_AudioCodecSpec_set_info(
    struct webrtc_AudioCodecSpec* self,
    struct webrtc_AudioCodecInfo* info);
WEBRTC_EXPORT struct webrtc_AudioCodecInfo* webrtc_AudioCodecSpec_get_info(
    struct webrtc_AudioCodecSpec* self);

#if defined(__cplusplus)
}
#endif
