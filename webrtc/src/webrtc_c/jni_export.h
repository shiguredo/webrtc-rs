#pragma once

#include "common.h"

#include <jni.h>

#if defined(__cplusplus)
extern "C" {
#endif

WEBRTC_EXPORT jint jni_JavaVM_GetEnv(JavaVM* vm,
                                     JNIEnv** out_env,
                                     jint version);
WEBRTC_EXPORT jint jni_JavaVM_AttachCurrentThread(JavaVM* vm,
                                                  JNIEnv** out_env,
                                                  void* args);
WEBRTC_EXPORT jint jni_JavaVM_DetachCurrentThread(JavaVM* vm);

WEBRTC_EXPORT jint jni_JNIEnv_GetJavaVM(JNIEnv* env, JavaVM** out_vm);
WEBRTC_EXPORT jmethodID jni_JNIEnv_GetMethodID(JNIEnv* env,
                                               jclass clazz,
                                               const char* name,
                                               const char* sig);
WEBRTC_EXPORT jobject jni_JNIEnv_NewObjectA(JNIEnv* env,
                                            jclass clazz,
                                            jmethodID method_id,
                                            const jvalue* args);
WEBRTC_EXPORT jobject jni_JNIEnv_NewGlobalRef(JNIEnv* env, jobject obj);
WEBRTC_EXPORT void jni_JNIEnv_DeleteGlobalRef(JNIEnv* env, jobject global_ref);
WEBRTC_EXPORT void jni_JNIEnv_DeleteLocalRef(JNIEnv* env, jobject local_ref);
WEBRTC_EXPORT jboolean jni_JNIEnv_ExceptionCheck(JNIEnv* env);
WEBRTC_EXPORT void jni_JNIEnv_ExceptionClear(JNIEnv* env);

#if defined(__cplusplus)
}
#endif
