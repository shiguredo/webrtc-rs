#pragma once

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::Environment
// -------------------------

struct webrtc_Environment;
struct webrtc_Environment* webrtc_CreateEnvironment();
void webrtc_Environment_delete(struct webrtc_Environment* self);

#if defined(__cplusplus)
}
#endif
