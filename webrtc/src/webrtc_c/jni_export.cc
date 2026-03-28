#include "jni_export.h"

extern "C" {

WEBRTC_EXPORT jint jni_JavaVM_GetEnv(JavaVM* vm,
                                     JNIEnv** out_env,
                                     jint version) {
  return vm->GetEnv(reinterpret_cast<void**>(out_env), version);
}

WEBRTC_EXPORT jint jni_JavaVM_AttachCurrentThread(JavaVM* vm,
                                                  JNIEnv** out_env,
                                                  void* args) {
  return vm->AttachCurrentThread(out_env, args);
}

WEBRTC_EXPORT jint jni_JavaVM_DetachCurrentThread(JavaVM* vm) {
  return vm->DetachCurrentThread();
}

WEBRTC_EXPORT jint jni_JNIEnv_GetJavaVM(JNIEnv* env, JavaVM** out_vm) {
  return env->GetJavaVM(out_vm);
}

WEBRTC_EXPORT jobject jni_JNIEnv_NewGlobalRef(JNIEnv* env, jobject obj) {
  return env->NewGlobalRef(obj);
}

WEBRTC_EXPORT void jni_JNIEnv_DeleteGlobalRef(JNIEnv* env, jobject global_ref) {
  env->DeleteGlobalRef(global_ref);
}

WEBRTC_EXPORT void jni_JNIEnv_DeleteLocalRef(JNIEnv* env, jobject local_ref) {
  env->DeleteLocalRef(local_ref);
}

WEBRTC_EXPORT jboolean jni_JNIEnv_ExceptionCheck(JNIEnv* env) {
  return env->ExceptionCheck();
}

WEBRTC_EXPORT void jni_JNIEnv_ExceptionClear(JNIEnv* env) {
  env->ExceptionClear();
}
}
