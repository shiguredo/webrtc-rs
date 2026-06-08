#pragma once

#include "../../common.h"
#include "video_frame.h"

#if defined(__cplusplus)
extern "C" {
#endif

struct webrtc_VideoSinkInterface;

// 全コールバックは必須（null 非許容）。
// 呼び出し側は全関数ポインタを非 null で設定しなければならない。
struct webrtc_VideoSinkInterface_cbs {
  void (*OnFrame)(const struct webrtc_VideoFrame* frame, void* user_data);
  void (*OnDiscardedFrame)(void* user_data);
  void (*OnDestroy)(void* user_data);
};

WEBRTC_EXPORT struct webrtc_VideoSinkInterface* webrtc_VideoSinkInterface_new(
    const struct webrtc_VideoSinkInterface_cbs* cbs,
    void* user_data);
WEBRTC_EXPORT void webrtc_VideoSinkInterface_delete(
    struct webrtc_VideoSinkInterface* self);

#if defined(__cplusplus)
}
#endif
