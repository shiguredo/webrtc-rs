#include "thread.h"

#include <stdarg.h>
#include <stddef.h>
#include <memory>

// WebRTC
#include <rtc_base/thread.h>

#include "../common.h"
#include "../common.impl.h"

// -------------------------
// webrtc::Thread
// -------------------------

extern "C" {
WEBRTC_DEFINE_UNIQUE(webrtc_Thread, webrtc::Thread);

WEBRTC_EXPORT void webrtc_Thread_Stop(struct webrtc_Thread* self) {
  auto p = reinterpret_cast<webrtc::Thread*>(self);
  p->Stop();
}
WEBRTC_EXPORT int webrtc_Thread_Start(struct webrtc_Thread* self) {
  auto p = reinterpret_cast<webrtc::Thread*>(self);
  return p->Start() ? 1 : 0;
}
WEBRTC_EXPORT void webrtc_Thread_Quit(struct webrtc_Thread* self) {
  auto p = reinterpret_cast<webrtc::Thread*>(self);
  p->Quit();
}
WEBRTC_EXPORT struct webrtc_Thread_unique* webrtc_Thread_Create() {
  auto p = webrtc::Thread::Create();
  return reinterpret_cast<struct webrtc_Thread_unique*>(p.release());
}
WEBRTC_EXPORT struct webrtc_Thread_unique*
webrtc_Thread_CreateWithSocketServer() {
  auto p = webrtc::Thread::CreateWithSocketServer();
  return reinterpret_cast<struct webrtc_Thread_unique*>(p.release());
}
WEBRTC_EXPORT void webrtc_Thread_BlockingCall(struct webrtc_Thread* self,
                                              void (*func)(void*),
                                              void* arg) {
  auto p = reinterpret_cast<webrtc::Thread*>(self);
  p->BlockingCall([func, arg]() { func(arg); });
}
WEBRTC_EXPORT void* webrtc_Thread_BlockingCall_r(struct webrtc_Thread* self,
                                                 void* (*func)(void*),
                                                 void* arg) {
  auto p = reinterpret_cast<webrtc::Thread*>(self);
  // 非 void テンプレートの BlockingCall はデフォルト初期化値を返すため、停止中
  // スレッドでは未初期化ポインタ（void*）が返り得る。ここでは nullptr を
  // 初期値にし、void 版 BlockingCall で functor を実行する。
  // こうすることで functor が実行されない場合は nullptr が返ることになる
  // （C++ の不確定値の穴を正規化する）。
  void* result = nullptr;
  p->BlockingCall([func, arg, &result]() { result = func(arg); });
  return result;
}
WEBRTC_EXPORT int webrtc_Thread_SleepMs(int millis) {
  return webrtc::Thread::SleepMs(millis) ? 1 : 0;
}
}
