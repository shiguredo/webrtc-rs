#pragma once

#include "video_frame.h"

#if defined(__cplusplus)
extern "C" {
#endif

struct webrtc_VideoSinkInterface;

struct webrtc_VideoSinkInterface_cbs {
  void (*OnFrame)(const struct webrtc_VideoFrame* frame, void* user_data);
  void (*OnDiscardedFrame)(void* user_data);
};

struct webrtc_VideoSinkInterface* webrtc_VideoSinkInterface_new(
    struct webrtc_VideoSinkInterface_cbs* cbs,
    void* user_data);
void webrtc_VideoSinkInterface_delete(struct webrtc_VideoSinkInterface* self);

#if defined(__cplusplus)
}
#endif
