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

WEBRTC_DECLARE_UNIQUE(webrtc_SdpVideoFormat);
WEBRTC_DECLARE_VECTOR_NO_DEFAULT_CTOR(webrtc_SdpVideoFormat);
struct webrtc_SdpVideoFormat_unique* webrtc_SdpVideoFormat_new(
    const char* name,
    size_t name_len,
    struct std_map_string_string* parameters);
struct std_string* webrtc_SdpVideoFormat_get_name(
    struct webrtc_SdpVideoFormat* self);
struct std_map_string_string* webrtc_SdpVideoFormat_get_parameters(
    struct webrtc_SdpVideoFormat* self);
int webrtc_SdpVideoFormat_is_equal(struct webrtc_SdpVideoFormat* lhs,
                                   struct webrtc_SdpVideoFormat* rhs);

#if defined(__cplusplus)
}
#endif
