#pragma once

#include "../../common.h"
#include "../../std.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::ColorSpace
// -------------------------

WEBRTC_DECLARE_UNIQUE(webrtc_ColorSpace);
WEBRTC_EXPORT struct webrtc_ColorSpace_unique* webrtc_ColorSpace_new();
WEBRTC_EXPORT struct std_string_unique* webrtc_ColorSpace_AsString(
    const struct webrtc_ColorSpace* self);

#if defined(__cplusplus)
}
#endif
