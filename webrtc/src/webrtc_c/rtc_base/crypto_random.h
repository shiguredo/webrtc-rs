#pragma once

#include "../common.h"
#include "../std.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::CreateRandomString
// -------------------------

WEBRTC_EXPORT struct std_string_unique* webrtc_CreateRandomString(
    size_t length);

#if defined(__cplusplus)
}
#endif
