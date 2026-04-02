#pragma once

#include "../../../../api/video_codecs/video_decoder_factory.h"
#include "../../../../api/video_codecs/video_encoder_factory.h"
#include "../../../../common.h"
#include "../../../../jni_export.h"

#if defined(__cplusplus)
extern "C" {
#endif

WEBRTC_EXPORT struct webrtc_VideoDecoderFactory_unique*
webrtc_JavaToNativeVideoDecoderFactory(JNIEnv* env, jobject decoder_factory);

WEBRTC_EXPORT struct webrtc_VideoEncoderFactory_unique*
webrtc_JavaToNativeVideoEncoderFactory(JNIEnv* env, jobject encoder_factory);

#if defined(__cplusplus)
}
#endif
