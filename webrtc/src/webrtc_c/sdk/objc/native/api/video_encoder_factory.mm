#include "video_encoder_factory.h"

#if !defined(WEBRTC_IOS) && !defined(WEBRTC_MAC)

extern "C" {

WEBRTC_EXPORT struct webrtc_VideoEncoderFactory_unique*
webrtc_ObjCToNativeVideoEncoderFactory(
    struct webrtc_objc_RTCVideoEncoderFactory* objc_video_encoder_factory) {
  (void)objc_video_encoder_factory;
  return nullptr;
}
}

#else

#include <sdk/objc/native/api/video_encoder_factory.h>

namespace {

id<RTC_OBJC_TYPE(RTCVideoEncoderFactory)> ToRTCVideoEncoderFactory(
    struct webrtc_objc_RTCVideoEncoderFactory* self) {
  return (__bridge id<RTC_OBJC_TYPE(RTCVideoEncoderFactory)>)self;
}

}  // namespace

extern "C" {
WEBRTC_EXPORT struct webrtc_VideoEncoderFactory_unique*
webrtc_ObjCToNativeVideoEncoderFactory(
    struct webrtc_objc_RTCVideoEncoderFactory* objc_video_encoder_factory) {
  if (objc_video_encoder_factory == nullptr) {
    return nullptr;
  }
  auto factory = webrtc::ObjCToNativeVideoEncoderFactory(
      ToRTCVideoEncoderFactory(objc_video_encoder_factory));
  return reinterpret_cast<struct webrtc_VideoEncoderFactory_unique*>(
      factory.release());
}
}

#endif
