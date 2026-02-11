#include "thread.h"

#include <stdarg.h>
#include <stddef.h>
#include <memory>

// WebRTC
#include <rtc_base/thread.h>

#include "../common.impl.h"

// -------------------------
// webrtc::Thread
// -------------------------

extern "C" {
WEBRTC_DEFINE_UNIQUE(webrtc_Thread, webrtc::Thread);

void webrtc_Thread_Stop(struct webrtc_Thread* self) {
  auto p = reinterpret_cast<webrtc::Thread*>(self);
  p->Stop();
}
void webrtc_Thread_Start(struct webrtc_Thread* self) {
  auto p = reinterpret_cast<webrtc::Thread*>(self);
  p->Start();
}
struct webrtc_Thread_unique* webrtc_Thread_Create() {
  auto p = webrtc::Thread::Create();
  return reinterpret_cast<struct webrtc_Thread_unique*>(p.release());
}
struct webrtc_Thread_unique* webrtc_Thread_CreateWithSocketServer() {
  auto p = webrtc::Thread::CreateWithSocketServer();
  return reinterpret_cast<struct webrtc_Thread_unique*>(p.release());
}
void webrtc_Thread_BlockingCall(struct webrtc_Thread* self,
                                void (*func)(void*),
                                void* arg) {
  auto p = reinterpret_cast<webrtc::Thread*>(self);
  p->BlockingCall([func, arg]() { func(arg); });
}
void* webrtc_Thread_BlockingCall_r(struct webrtc_Thread* self,
                                   void* (*func)(void*),
                                   void* arg) {
  auto p = reinterpret_cast<webrtc::Thread*>(self);
  return p->BlockingCall([func, arg]() { return func(arg); });
}
void webrtc_Thread_SleepMs(int millis) {
  webrtc::Thread::SleepMs(millis);
}
}
