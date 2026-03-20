#pragma once

#include "../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::Environment
// -------------------------

struct webrtc_Environment;
struct webrtc_Environment* WEBRTC_EXPORT webrtc_CreateEnvironment();
void WEBRTC_EXPORT webrtc_Environment_delete(struct webrtc_Environment* self);

#if defined(__cplusplus)
}
#endif
