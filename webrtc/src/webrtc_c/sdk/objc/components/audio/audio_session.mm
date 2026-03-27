#include "audio_session.h"

#import <AVFoundation/AVFoundation.h>
#import <Foundation/Foundation.h>

#import <sdk/objc/components/audio/RTCAudioSession.h>
#import <sdk/objc/components/audio/RTCAudioSessionConfiguration.h>

namespace {

RTCAudioSession* ToRTCAudioSession(struct webrtc_objc_RTCAudioSession* self) {
  return (__bridge RTCAudioSession*)(void*)self;
}

RTCAudioSessionConfiguration* ToRTCAudioSessionConfiguration(
    struct webrtc_objc_RTCAudioSessionConfiguration* self) {
  return (__bridge RTCAudioSessionConfiguration*)(void*)self;
}

struct webrtc_objc_RTCAudioSession* RetainRTCAudioSession(
    RTCAudioSession* self) {
  return (struct webrtc_objc_RTCAudioSession*)CFBridgingRetain(self);
}

struct webrtc_objc_RTCAudioSessionConfiguration*
RetainRTCAudioSessionConfiguration(RTCAudioSessionConfiguration* self) {
  return (struct webrtc_objc_RTCAudioSessionConfiguration*)CFBridgingRetain(
      self);
}

struct objc_NSError* RetainNSError(NSError* self) {
  return (struct objc_NSError*)CFBridgingRetain(self);
}

}  // namespace

extern "C" {

WEBRTC_EXPORT struct webrtc_objc_RTCAudioSession*
webrtc_objc_RTCAudioSession_sharedInstance(void) {
  return RetainRTCAudioSession([RTCAudioSession sharedInstance]);
}

WEBRTC_EXPORT void webrtc_objc_RTCAudioSession_release(
    struct webrtc_objc_RTCAudioSession* self) {
  CFBridgingRelease(reinterpret_cast<CFTypeRef>(self));
}

WEBRTC_EXPORT void webrtc_objc_RTCAudioSession_lockForConfiguration(
    struct webrtc_objc_RTCAudioSession* self) {
  [ToRTCAudioSession(self) lockForConfiguration];
}

WEBRTC_EXPORT void webrtc_objc_RTCAudioSession_unlockForConfiguration(
    struct webrtc_objc_RTCAudioSession* self) {
  [ToRTCAudioSession(self) unlockForConfiguration];
}

WEBRTC_EXPORT int webrtc_objc_RTCAudioSession_setConfiguration_active_error(
    struct webrtc_objc_RTCAudioSession* self,
    struct webrtc_objc_RTCAudioSessionConfiguration* configuration,
    int active,
    struct objc_NSError** out_error) {
  NSError* error = nil;
  BOOL ok = [ToRTCAudioSession(self)
      setConfiguration:ToRTCAudioSessionConfiguration(configuration)
                active:active != 0
                 error:&error];
  if (out_error != nullptr) {
    *out_error = error == nil ? nullptr : RetainNSError(error);
  }
  return ok ? 1 : 0;
}

WEBRTC_EXPORT int webrtc_objc_RTCAudioSession_setActive_error(
    struct webrtc_objc_RTCAudioSession* self,
    int active,
    struct objc_NSError** out_error) {
  NSError* error = nil;
  BOOL ok = [ToRTCAudioSession(self) setActive:active != 0 error:&error];
  if (out_error != nullptr) {
    *out_error = error == nil ? nullptr : RetainNSError(error);
  }
  return ok ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_objc_RTCAudioSession_initializeInput(
    struct webrtc_objc_RTCAudioSession* self,
    webrtc_objc_RTCAudioSession_initializeInput_callback callback,
    void* user_data) {
  RTCAudioSession* session = ToRTCAudioSession(self);
  if (callback == nullptr) {
    [session initializeInput:nil];
    return;
  }
  [session initializeInput:^(NSError* _Nullable error) {
    callback(error == nil ? nullptr : RetainNSError(error), user_data);
  }];
}

WEBRTC_EXPORT struct webrtc_objc_RTCAudioSessionConfiguration*
webrtc_objc_RTCAudioSessionConfiguration_webRTCConfiguration(void) {
  return RetainRTCAudioSessionConfiguration(
      [RTCAudioSessionConfiguration webRTCConfiguration]);
}

WEBRTC_EXPORT void webrtc_objc_RTCAudioSessionConfiguration_release(
    struct webrtc_objc_RTCAudioSessionConfiguration* self) {
  CFBridgingRelease(reinterpret_cast<CFTypeRef>(self));
}

WEBRTC_EXPORT void
webrtc_objc_RTCAudioSessionConfiguration_setWebRTCConfiguration(
    struct webrtc_objc_RTCAudioSessionConfiguration* configuration) {
  [RTCAudioSessionConfiguration
      setWebRTCConfiguration:ToRTCAudioSessionConfiguration(configuration)];
}

WEBRTC_EXPORT void webrtc_objc_RTCAudioSessionConfiguration_setCategory(
    struct webrtc_objc_RTCAudioSessionConfiguration* self,
    const void* category) {
  ToRTCAudioSessionConfiguration(self).category = (__bridge NSString*)category;
}

WEBRTC_EXPORT void webrtc_objc_RTCAudioSessionConfiguration_setMode(
    struct webrtc_objc_RTCAudioSessionConfiguration* self,
    const void* mode) {
  ToRTCAudioSessionConfiguration(self).mode = (__bridge NSString*)mode;
}

WEBRTC_EXPORT void webrtc_objc_RTCAudioSessionConfiguration_setCategoryOptions(
    struct webrtc_objc_RTCAudioSessionConfiguration* self,
    uint64_t category_options) {
  ToRTCAudioSessionConfiguration(self).categoryOptions =
      static_cast<AVAudioSessionCategoryOptions>(category_options);
}

}
