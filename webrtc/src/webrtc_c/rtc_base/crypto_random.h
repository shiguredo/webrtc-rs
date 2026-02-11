#pragma once

#include "../std.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::CreateRandomString
// -------------------------

struct std_string_unique* webrtc_CreateRandomString(int length);

#if defined(__cplusplus)
}
#endif
