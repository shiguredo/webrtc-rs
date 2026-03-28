#include "objc.h"

#if defined(WEBRTC_IOS)

#import <AVFoundation/AVFoundation.h>
#import <Foundation/Foundation.h>

namespace {

NSString* ToNSString(struct objc_NSString* self) {
  return (__bridge NSString*)self;
}

NSError* ToNSError(struct objc_NSError* self) {
  return (__bridge NSError*)self;
}

struct objc_NSString* RetainNSString(NSString* self) {
  return (struct objc_NSString*)CFBridgingRetain(self);
}

}  // namespace

extern "C" {

// -------------------------
// Foundation/NSString
// -------------------------

WEBRTC_EXPORT objc_Class objc_NSString_class(void) {
  return (objc_Class)(__bridge void*)[NSString class];
}

WEBRTC_EXPORT struct objc_NSString* objc_NSString_stringWithUTF8String(
    const char* utf8) {
  if (utf8 == nullptr) {
    return nullptr;
  }
  NSString* value = [NSString stringWithUTF8String:utf8];
  if (value == nil) {
    return nullptr;
  }
  return RetainNSString(value);
}

WEBRTC_EXPORT const char* objc_NSString_UTF8String(struct objc_NSString* self) {
  return [ToNSString(self) UTF8String];
}

WEBRTC_EXPORT void objc_NSString_release(struct objc_NSString* self) {
  if (self == nullptr) {
    return;
  }
  CFBridgingRelease(reinterpret_cast<CFTypeRef>(self));
}

// -------------------------
// Foundation/NSError
// -------------------------

WEBRTC_EXPORT objc_Class objc_NSError_class(void) {
  return (objc_Class)(__bridge void*)[NSError class];
}

WEBRTC_EXPORT int64_t objc_NSError_code(struct objc_NSError* self) {
  return static_cast<int64_t>(ToNSError(self).code);
}

WEBRTC_EXPORT struct objc_NSString* objc_NSError_domain(
    struct objc_NSError* self) {
  return RetainNSString(ToNSError(self).domain);
}

WEBRTC_EXPORT struct objc_NSString* objc_NSError_localizedDescription(
    struct objc_NSError* self) {
  return RetainNSString(ToNSError(self).localizedDescription);
}

WEBRTC_EXPORT void objc_NSError_release(struct objc_NSError* self) {
  CFBridgingRelease(reinterpret_cast<CFTypeRef>(self));
}

// -------------------------
// AVFoundation/AVAudioSession
// -------------------------

WEBRTC_EXPORT objc_AVAudioSessionCategory
objc_AVAudioSessionCategory_Ambient(void) {
  return reinterpret_cast<objc_AVAudioSessionCategory>(
      (__bridge void*)AVAudioSessionCategoryAmbient);
}

WEBRTC_EXPORT objc_AVAudioSessionCategory
objc_AVAudioSessionCategory_SoloAmbient(void) {
  return reinterpret_cast<objc_AVAudioSessionCategory>(
      (__bridge void*)AVAudioSessionCategorySoloAmbient);
}

WEBRTC_EXPORT objc_AVAudioSessionCategory
objc_AVAudioSessionCategory_Playback(void) {
  return reinterpret_cast<objc_AVAudioSessionCategory>(
      (__bridge void*)AVAudioSessionCategoryPlayback);
}

WEBRTC_EXPORT objc_AVAudioSessionCategory
objc_AVAudioSessionCategory_Record(void) {
  return reinterpret_cast<objc_AVAudioSessionCategory>(
      (__bridge void*)AVAudioSessionCategoryRecord);
}

WEBRTC_EXPORT objc_AVAudioSessionCategory
objc_AVAudioSessionCategory_PlayAndRecord(void) {
  return reinterpret_cast<objc_AVAudioSessionCategory>(
      (__bridge void*)AVAudioSessionCategoryPlayAndRecord);
}

WEBRTC_EXPORT objc_AVAudioSessionCategory
objc_AVAudioSessionCategory_MultiRoute(void) {
  return reinterpret_cast<objc_AVAudioSessionCategory>(
      (__bridge void*)AVAudioSessionCategoryMultiRoute);
}

WEBRTC_EXPORT objc_AVAudioSessionMode objc_AVAudioSessionMode_Default(void) {
  return reinterpret_cast<objc_AVAudioSessionMode>(
      (__bridge void*)AVAudioSessionModeDefault);
}

WEBRTC_EXPORT objc_AVAudioSessionMode objc_AVAudioSessionMode_VoiceChat(void) {
  return reinterpret_cast<objc_AVAudioSessionMode>(
      (__bridge void*)AVAudioSessionModeVoiceChat);
}

WEBRTC_EXPORT objc_AVAudioSessionMode objc_AVAudioSessionMode_VideoChat(void) {
  return reinterpret_cast<objc_AVAudioSessionMode>(
      (__bridge void*)AVAudioSessionModeVideoChat);
}

WEBRTC_EXPORT objc_AVAudioSessionMode objc_AVAudioSessionMode_GameChat(void) {
  return reinterpret_cast<objc_AVAudioSessionMode>(
      (__bridge void*)AVAudioSessionModeGameChat);
}

WEBRTC_EXPORT objc_AVAudioSessionMode
objc_AVAudioSessionMode_VideoRecording(void) {
  return reinterpret_cast<objc_AVAudioSessionMode>(
      (__bridge void*)AVAudioSessionModeVideoRecording);
}

WEBRTC_EXPORT objc_AVAudioSessionMode
objc_AVAudioSessionMode_Measurement(void) {
  return reinterpret_cast<objc_AVAudioSessionMode>(
      (__bridge void*)AVAudioSessionModeMeasurement);
}

WEBRTC_EXPORT objc_AVAudioSessionMode
objc_AVAudioSessionMode_MoviePlayback(void) {
  return reinterpret_cast<objc_AVAudioSessionMode>(
      (__bridge void*)AVAudioSessionModeMoviePlayback);
}

WEBRTC_EXPORT objc_AVAudioSessionMode
objc_AVAudioSessionMode_SpokenAudio(void) {
  return reinterpret_cast<objc_AVAudioSessionMode>(
      (__bridge void*)AVAudioSessionModeSpokenAudio);
}

WEBRTC_EXPORT uint64_t
objc_AVAudioSessionCategoryOption_AllowBluetoothHFP(void) {
  return static_cast<uint64_t>(AVAudioSessionCategoryOptionAllowBluetoothHFP);
}

WEBRTC_EXPORT uint64_t
objc_AVAudioSessionCategoryOption_AllowBluetoothA2DP(void) {
  return static_cast<uint64_t>(AVAudioSessionCategoryOptionAllowBluetoothA2DP);
}

WEBRTC_EXPORT uint64_t objc_AVAudioSessionCategoryOption_AllowAirPlay(void) {
  return static_cast<uint64_t>(AVAudioSessionCategoryOptionAllowAirPlay);
}

WEBRTC_EXPORT uint64_t
objc_AVAudioSessionCategoryOption_DefaultToSpeaker(void) {
  return static_cast<uint64_t>(AVAudioSessionCategoryOptionDefaultToSpeaker);
}

WEBRTC_EXPORT uint64_t objc_AVAudioSessionCategoryOption_MixWithOthers(void) {
  return static_cast<uint64_t>(AVAudioSessionCategoryOptionMixWithOthers);
}

WEBRTC_EXPORT uint64_t objc_AVAudioSessionCategoryOption_DuckOthers(void) {
  return static_cast<uint64_t>(AVAudioSessionCategoryOptionDuckOthers);
}

WEBRTC_EXPORT uint64_t
objc_AVAudioSessionCategoryOption_InterruptSpokenAudioAndMixWithOthers(void) {
  return static_cast<uint64_t>(
      AVAudioSessionCategoryOptionInterruptSpokenAudioAndMixWithOthers);
}

WEBRTC_EXPORT uint64_t
objc_AVAudioSessionCategoryOption_OverrideMutedMicrophoneInterruption(void) {
  return static_cast<uint64_t>(
      AVAudioSessionCategoryOptionOverrideMutedMicrophoneInterruption);
}

}  // extern "C"

#endif  // defined(WEBRTC_IOS)
