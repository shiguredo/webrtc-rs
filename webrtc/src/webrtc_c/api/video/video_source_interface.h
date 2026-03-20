#pragma once

#include "../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

struct webrtc_VideoSinkWants;
struct webrtc_VideoSinkWants* WEBRTC_EXPORT webrtc_VideoSinkWants_new();
void WEBRTC_EXPORT
webrtc_VideoSinkWants_delete(struct webrtc_VideoSinkWants* self);

#if defined(__cplusplus)
}
#endif
