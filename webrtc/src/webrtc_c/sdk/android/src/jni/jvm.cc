#include "jvm.h"

#include <sdk/android/src/jni/jvm.h>

extern "C" {

WEBRTC_EXPORT int webrtc_jni_InitGlobalJniVariables(JavaVM* jvm) {
  return static_cast<int>(webrtc::jni::InitGlobalJniVariables(jvm));
}

WEBRTC_EXPORT JNIEnv* webrtc_jni_GetEnv(void) {
  return webrtc::jni::GetEnv();
}

WEBRTC_EXPORT JavaVM* webrtc_jni_GetJVM(void) {
  return webrtc::jni::GetJVM();
}

WEBRTC_EXPORT JNIEnv* webrtc_jni_AttachCurrentThreadIfNeeded(void) {
  return webrtc::jni::AttachCurrentThreadIfNeeded();
}
}
