#pragma once

#include "../../../../api/audio/audio_device.h"
#include "../../../../api/environment.h"
#include "../../../../common.h"
#include "../../../../jni_export.h"

#if defined(__cplusplus)
extern "C" {
#endif

WEBRTC_EXPORT struct webrtc_AudioDeviceModule_refcounted*
webrtc_CreateJavaAudioDeviceModule(JNIEnv* env,
                                   struct webrtc_Environment* webrtc_env,
                                   jobject application_context);

#if defined(__cplusplus)
}
#endif
