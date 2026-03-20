#pragma once

#include "../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::Environment
// -------------------------

struct webrtc_Environment;
WEBRTC_EXPORT struct webrtc_Environment* webrtc_CreateEnvironment();
WEBRTC_EXPORT void webrtc_Environment_delete(struct webrtc_Environment* self);

#if defined(__cplusplus)
}
#endif
