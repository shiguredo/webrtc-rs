#include "class_loader.h"

#include <sdk/android/native_api/jni/class_loader.h>

extern "C" {

WEBRTC_EXPORT void webrtc_InitClassLoader(JNIEnv* env) {
  webrtc::InitClassLoader(env);
}
}
