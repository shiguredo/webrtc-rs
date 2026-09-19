#pragma once

#include <stddef.h>

#include "../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::FieldTrialsView
// -------------------------

struct webrtc_FieldTrialsView;
WEBRTC_EXPORT int webrtc_FieldTrialsView_IsEnabled(
    const struct webrtc_FieldTrialsView* self,
    const char* key,
    size_t key_len);

#if defined(__cplusplus)
}
#endif
