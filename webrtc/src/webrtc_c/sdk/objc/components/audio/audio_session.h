#pragma once

#include <stdint.h>

#include "../../../../objc.h"

#if defined(__cplusplus)
extern "C" {
#endif

struct webrtc_objc_RTCAudioSession;
struct webrtc_objc_RTCAudioSessionConfiguration;

WEBRTC_EXPORT struct webrtc_objc_RTCAudioSession*
webrtc_objc_RTCAudioSession_sharedInstance(void);
WEBRTC_EXPORT void webrtc_objc_RTCAudioSession_release(
    struct webrtc_objc_RTCAudioSession* self);
WEBRTC_EXPORT void webrtc_objc_RTCAudioSession_lockForConfiguration(
    struct webrtc_objc_RTCAudioSession* self);
WEBRTC_EXPORT void webrtc_objc_RTCAudioSession_unlockForConfiguration(
    struct webrtc_objc_RTCAudioSession* self);
WEBRTC_EXPORT int webrtc_objc_RTCAudioSession_setConfiguration_active_error(
    struct webrtc_objc_RTCAudioSession* self,
    struct webrtc_objc_RTCAudioSessionConfiguration* configuration,
    int active,
    struct objc_NSError** out_error);
WEBRTC_EXPORT int webrtc_objc_RTCAudioSession_setActive_error(
    struct webrtc_objc_RTCAudioSession* self,
    int active,
    struct objc_NSError** out_error);

typedef void (*webrtc_objc_RTCAudioSession_initializeInput_callback)(
    struct objc_NSError* error,
    void* user_data);

WEBRTC_EXPORT void webrtc_objc_RTCAudioSession_initializeInput(
    struct webrtc_objc_RTCAudioSession* self,
    webrtc_objc_RTCAudioSession_initializeInput_callback callback,
    void* user_data);

WEBRTC_EXPORT struct webrtc_objc_RTCAudioSessionConfiguration*
webrtc_objc_RTCAudioSessionConfiguration_webRTCConfiguration(void);
WEBRTC_EXPORT void webrtc_objc_RTCAudioSessionConfiguration_release(
    struct webrtc_objc_RTCAudioSessionConfiguration* self);
WEBRTC_EXPORT void
webrtc_objc_RTCAudioSessionConfiguration_setWebRTCConfiguration(
    struct webrtc_objc_RTCAudioSessionConfiguration* configuration);
WEBRTC_EXPORT void webrtc_objc_RTCAudioSessionConfiguration_setCategory(
    struct webrtc_objc_RTCAudioSessionConfiguration* self,
    const void* category);
WEBRTC_EXPORT void webrtc_objc_RTCAudioSessionConfiguration_setMode(
    struct webrtc_objc_RTCAudioSessionConfiguration* self,
    const void* mode);
WEBRTC_EXPORT void webrtc_objc_RTCAudioSessionConfiguration_setCategoryOptions(
    struct webrtc_objc_RTCAudioSessionConfiguration* self,
    uint64_t category_options);

#if defined(__cplusplus)
}
#endif
