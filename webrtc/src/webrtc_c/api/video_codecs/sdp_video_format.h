#pragma once

#include <stddef.h>

#include "../../common.h"
#include "../../std.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::SdpVideoFormat
// -------------------------

WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_L1T1;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_L1T2;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_L1T3;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_L2T1;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_L2T1h;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_L2T1_KEY;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_L2T2;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_L2T2h;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_L2T2_KEY;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_L2T2_KEY_SHIFT;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_L2T3;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_L2T3h;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_L2T3_KEY;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_L3T1;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_L3T1h;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_L3T1_KEY;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_L3T2;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_L3T2h;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_L3T2_KEY;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_L3T3;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_L3T3h;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_L3T3_KEY;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_S2T1;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_S2T1h;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_S2T2;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_S2T2h;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_S2T3;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_S2T3h;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_S3T1;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_S3T1h;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_S3T2;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_S3T2h;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_S3T3;
WEBRTC_EXPORT extern const int webrtc_ScalabilityMode_S3T3h;
WEBRTC_EXPORT struct std_string_unique* webrtc_ScalabilityModeToString(
    int mode);

WEBRTC_DECLARE_UNIQUE(webrtc_SdpVideoFormat);
WEBRTC_DECLARE_VECTOR_NO_DEFAULT_CTOR(webrtc_SdpVideoFormat);
WEBRTC_EXPORT struct webrtc_SdpVideoFormat_unique* webrtc_SdpVideoFormat_new(
    const char* name,
    size_t name_len);
WEBRTC_EXPORT struct webrtc_SdpVideoFormat_unique*
webrtc_SdpVideoFormat_new_with_parameters(
    const char* name,
    size_t name_len,
    struct std_map_string_string* parameters,
    const int* scalability_modes,
    size_t scalability_modes_len);
WEBRTC_EXPORT struct webrtc_SdpVideoFormat_unique* webrtc_SdpVideoFormat_copy(
    struct webrtc_SdpVideoFormat* self);
WEBRTC_EXPORT struct std_string* webrtc_SdpVideoFormat_get_name(
    struct webrtc_SdpVideoFormat* self);
WEBRTC_EXPORT struct std_map_string_string*
webrtc_SdpVideoFormat_get_parameters(struct webrtc_SdpVideoFormat* self);
WEBRTC_EXPORT size_t webrtc_SdpVideoFormat_get_scalability_modes_size(
    struct webrtc_SdpVideoFormat* self);
WEBRTC_EXPORT size_t
webrtc_SdpVideoFormat_copy_scalability_modes(struct webrtc_SdpVideoFormat* self,
                                             int* out_modes,
                                             size_t out_modes_len);
WEBRTC_EXPORT int webrtc_SdpVideoFormat_is_equal(
    struct webrtc_SdpVideoFormat* lhs,
    struct webrtc_SdpVideoFormat* rhs);

#if defined(__cplusplus)
}
#endif
