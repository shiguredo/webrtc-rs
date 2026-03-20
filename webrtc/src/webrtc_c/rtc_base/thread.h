#pragma once

#include "../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::Thread
// -------------------------

WEBRTC_DECLARE_UNIQUE(webrtc_Thread);
void WEBRTC_EXPORT webrtc_Thread_Start(struct webrtc_Thread* self);
void WEBRTC_EXPORT webrtc_Thread_Stop(struct webrtc_Thread* self);
struct webrtc_Thread_unique* WEBRTC_EXPORT webrtc_Thread_Create();
struct webrtc_Thread_unique* WEBRTC_EXPORT
webrtc_Thread_CreateWithSocketServer();
void WEBRTC_EXPORT webrtc_Thread_BlockingCall(struct webrtc_Thread* self,
                                              void (*func)(void*),
                                              void* arg);
void* WEBRTC_EXPORT webrtc_Thread_BlockingCall_r(struct webrtc_Thread* self,
                                                 void* (*func)(void*),
                                                 void* arg);
void WEBRTC_EXPORT webrtc_Thread_SleepMs(int millis);

#if defined(__cplusplus)
}
#endif
