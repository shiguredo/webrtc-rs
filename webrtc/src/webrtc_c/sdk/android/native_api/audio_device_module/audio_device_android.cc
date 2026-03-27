#include "audio_device_android.h"

#include <api/environment/environment.h>
#include <sdk/android/native_api/audio_device_module/audio_device_android.h>

extern "C" {

WEBRTC_EXPORT struct webrtc_AudioDeviceModule_refcounted*
webrtc_CreateJavaAudioDeviceModule(JNIEnv* env,
                                   struct webrtc_Environment* webrtc_env,
                                   jobject application_context) {
  auto cpp_webrtc_env = reinterpret_cast<webrtc::Environment*>(webrtc_env);
  auto adm = webrtc::CreateJavaAudioDeviceModule(env, *cpp_webrtc_env,
                                                 application_context);
  return reinterpret_cast<struct webrtc_AudioDeviceModule_refcounted*>(
      adm.release());
}
}
