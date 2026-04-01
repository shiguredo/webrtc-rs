#pragma once

#include "../../../../common.h"
#include "../../../../jni_export.h"

#if defined(__cplusplus)
extern "C" {
#endif

WEBRTC_EXPORT void webrtc_InitClassLoader(JNIEnv* env);
WEBRTC_EXPORT jclass webrtc_GetClass(JNIEnv* env, const char* name);

#if defined(__cplusplus)
}
#endif
