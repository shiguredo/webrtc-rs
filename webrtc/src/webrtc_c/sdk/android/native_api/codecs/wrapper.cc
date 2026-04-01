#include "wrapper.h"

#include <sdk/android/native_api/codecs/wrapper.h>

extern "C" {

WEBRTC_EXPORT struct webrtc_VideoDecoderFactory_unique*
webrtc_JavaToNativeVideoDecoderFactory(JNIEnv* env, jobject decoder_factory) {
  auto factory = webrtc::JavaToNativeVideoDecoderFactory(env, decoder_factory);
  return reinterpret_cast<struct webrtc_VideoDecoderFactory_unique*>(
      factory.release());
}

WEBRTC_EXPORT struct webrtc_VideoEncoderFactory_unique*
webrtc_JavaToNativeVideoEncoderFactory(JNIEnv* env, jobject encoder_factory) {
  auto factory = webrtc::JavaToNativeVideoEncoderFactory(env, encoder_factory);
  return reinterpret_cast<struct webrtc_VideoEncoderFactory_unique*>(
      factory.release());
}
}
