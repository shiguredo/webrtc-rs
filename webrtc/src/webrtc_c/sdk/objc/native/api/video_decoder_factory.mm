#include "video_decoder_factory.h"

#if !defined(WEBRTC_IOS) && !defined(WEBRTC_MAC)

extern "C" {

WEBRTC_EXPORT struct webrtc_VideoDecoderFactory_unique*
webrtc_ObjCToNativeVideoDecoderFactory(
    struct webrtc_objc_RTCVideoDecoderFactory* objc_video_decoder_factory) {
  (void)objc_video_decoder_factory;
  return nullptr;
}
}

#else

#include <sdk/objc/native/api/video_decoder_factory.h>

namespace {

id<RTC_OBJC_TYPE(RTCVideoDecoderFactory)> ToRTCVideoDecoderFactory(
    struct webrtc_objc_RTCVideoDecoderFactory* self) {
  return (__bridge id<RTC_OBJC_TYPE(RTCVideoDecoderFactory)>)self;
}

}  // namespace

extern "C" {
WEBRTC_EXPORT struct webrtc_VideoDecoderFactory_unique*
webrtc_ObjCToNativeVideoDecoderFactory(
    struct webrtc_objc_RTCVideoDecoderFactory* objc_video_decoder_factory) {
  if (objc_video_decoder_factory == nullptr) {
    return nullptr;
  }
  auto factory = webrtc::ObjCToNativeVideoDecoderFactory(
      ToRTCVideoDecoderFactory(objc_video_decoder_factory));
  return reinterpret_cast<struct webrtc_VideoDecoderFactory_unique*>(
      factory.release());
}
}

#endif
