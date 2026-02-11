#pragma once

#include <stddef.h>

#include "../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::RTCError
// -------------------------

WEBRTC_DECLARE_UNIQUE(webrtc_RTCError);
int webrtc_RTCError_ok(struct webrtc_RTCError* self);
void webrtc_RTCError_message(struct webrtc_RTCError* self,
                             const char** out_message,
                             size_t* out_len);

#if defined(__cplusplus)
}
#endif
