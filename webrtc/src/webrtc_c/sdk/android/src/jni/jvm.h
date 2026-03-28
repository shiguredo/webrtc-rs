#pragma once

#include "../../../../common.h"
#include "../../../../jni_export.h"

#if defined(__cplusplus)
extern "C" {
#endif

WEBRTC_EXPORT int webrtc_jni_InitGlobalJniVariables(JavaVM* jvm);
WEBRTC_EXPORT JNIEnv* webrtc_jni_GetEnv(void);
WEBRTC_EXPORT JavaVM* webrtc_jni_GetJVM(void);
WEBRTC_EXPORT JNIEnv* webrtc_jni_AttachCurrentThreadIfNeeded(void);

#if defined(__cplusplus)
}
#endif
