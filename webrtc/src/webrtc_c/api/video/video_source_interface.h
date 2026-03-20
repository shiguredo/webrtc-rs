#pragma once

#include "../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

struct webrtc_VideoSinkWants;
WEBRTC_EXPORT struct webrtc_VideoSinkWants* webrtc_VideoSinkWants_new();
WEBRTC_EXPORT void webrtc_VideoSinkWants_delete(
    struct webrtc_VideoSinkWants* self);

#if defined(__cplusplus)
}
#endif
