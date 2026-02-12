#pragma once

#if defined(__cplusplus)
extern "C" {
#endif

struct webrtc_VideoSinkWants;
struct webrtc_VideoSinkWants* webrtc_VideoSinkWants_new();
void webrtc_VideoSinkWants_delete(struct webrtc_VideoSinkWants* self);

#if defined(__cplusplus)
}
#endif
