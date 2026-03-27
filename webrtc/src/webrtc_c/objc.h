#pragma once

#include <stdint.h>

#include "common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// Foundation/NSString
// -------------------------

struct objc_NSString;

WEBRTC_EXPORT const void* objc_NSString_class(void);
WEBRTC_EXPORT struct objc_NSString* objc_NSString_stringWithUTF8String(
    const char* utf8);
WEBRTC_EXPORT const char* objc_NSString_UTF8String(
    const struct objc_NSString* self);
WEBRTC_EXPORT void objc_NSString_release(struct objc_NSString* self);

// -------------------------
// Foundation/NSError
// -------------------------

struct objc_NSError;

WEBRTC_EXPORT const void* objc_NSError_class(void);
WEBRTC_EXPORT int64_t objc_NSError_code(const struct objc_NSError* self);
WEBRTC_EXPORT struct objc_NSString* objc_NSError_domain(
    const struct objc_NSError* self);
WEBRTC_EXPORT struct objc_NSString* objc_NSError_localizedDescription(
    const struct objc_NSError* self);
WEBRTC_EXPORT void objc_NSError_release(struct objc_NSError* self);

// -------------------------
// AVFoundation/AVAudioSession
// -------------------------

typedef struct objc_NSString* objc_AVAudioSessionCategory;
typedef struct objc_NSString* objc_AVAudioSessionMode;

WEBRTC_EXPORT objc_AVAudioSessionCategory
objc_AVAudioSessionCategory_Ambient(void);
WEBRTC_EXPORT objc_AVAudioSessionCategory
objc_AVAudioSessionCategory_SoloAmbient(void);
WEBRTC_EXPORT objc_AVAudioSessionCategory
objc_AVAudioSessionCategory_Playback(void);
WEBRTC_EXPORT objc_AVAudioSessionCategory
objc_AVAudioSessionCategory_Record(void);
WEBRTC_EXPORT objc_AVAudioSessionCategory
objc_AVAudioSessionCategory_PlayAndRecord(void);
WEBRTC_EXPORT objc_AVAudioSessionCategory
objc_AVAudioSessionCategory_MultiRoute(void);
WEBRTC_EXPORT objc_AVAudioSessionMode objc_AVAudioSessionMode_Default(void);
WEBRTC_EXPORT objc_AVAudioSessionMode objc_AVAudioSessionMode_VoiceChat(void);
WEBRTC_EXPORT objc_AVAudioSessionMode objc_AVAudioSessionMode_VideoChat(void);
WEBRTC_EXPORT objc_AVAudioSessionMode objc_AVAudioSessionMode_GameChat(void);
WEBRTC_EXPORT objc_AVAudioSessionMode
objc_AVAudioSessionMode_VideoRecording(void);
WEBRTC_EXPORT objc_AVAudioSessionMode objc_AVAudioSessionMode_Measurement(void);
WEBRTC_EXPORT objc_AVAudioSessionMode
objc_AVAudioSessionMode_MoviePlayback(void);
WEBRTC_EXPORT objc_AVAudioSessionMode objc_AVAudioSessionMode_SpokenAudio(void);
WEBRTC_EXPORT uint64_t
objc_AVAudioSessionCategoryOption_AllowBluetoothHFP(void);
WEBRTC_EXPORT uint64_t
objc_AVAudioSessionCategoryOption_AllowBluetoothA2DP(void);
WEBRTC_EXPORT uint64_t objc_AVAudioSessionCategoryOption_AllowBluetooth(void);
WEBRTC_EXPORT uint64_t objc_AVAudioSessionCategoryOption_AllowAirPlay(void);
WEBRTC_EXPORT uint64_t objc_AVAudioSessionCategoryOption_DefaultToSpeaker(void);
WEBRTC_EXPORT uint64_t objc_AVAudioSessionCategoryOption_MixWithOthers(void);
WEBRTC_EXPORT uint64_t objc_AVAudioSessionCategoryOption_DuckOthers(void);
WEBRTC_EXPORT uint64_t
objc_AVAudioSessionCategoryOption_InterruptSpokenAudioAndMixWithOthers(void);
WEBRTC_EXPORT uint64_t
objc_AVAudioSessionCategoryOption_OverrideMutedMicrophoneInterruption(void);

#if defined(__cplusplus)
}
#endif
