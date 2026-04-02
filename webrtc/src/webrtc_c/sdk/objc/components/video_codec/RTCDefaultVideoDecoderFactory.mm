#include "RTCDefaultVideoDecoderFactory.h"

#if !defined(WEBRTC_IOS) && !defined(WEBRTC_MAC)

extern "C" {

WEBRTC_EXPORT struct webrtc_objc_RTCVideoDecoderFactory*
webrtc_objc_RTCDefaultVideoDecoderFactory_new(void) {
  return nullptr;
}

WEBRTC_EXPORT void webrtc_objc_RTCVideoDecoderFactory_release(
    struct webrtc_objc_RTCVideoDecoderFactory* self) {
  (void)self;
}
}

#else

#import <Foundation/Foundation.h>

#import <sdk/objc/components/video_codec/RTCDefaultVideoDecoderFactory.h>

namespace {

struct webrtc_objc_RTCVideoDecoderFactory* RetainRTCVideoDecoderFactory(
    id<RTC_OBJC_TYPE(RTCVideoDecoderFactory)> self) {
  return (struct webrtc_objc_RTCVideoDecoderFactory*)CFBridgingRetain(self);
}

}  // namespace

extern "C" {

WEBRTC_EXPORT struct webrtc_objc_RTCVideoDecoderFactory*
webrtc_objc_RTCDefaultVideoDecoderFactory_new(void) {
  return RetainRTCVideoDecoderFactory(
      [[RTC_OBJC_TYPE(RTCDefaultVideoDecoderFactory) alloc] init]);
}

WEBRTC_EXPORT void webrtc_objc_RTCVideoDecoderFactory_release(
    struct webrtc_objc_RTCVideoDecoderFactory* self) {
  if (self == nullptr) {
    return;
  }
  CFBridgingRelease(reinterpret_cast<CFTypeRef>(self));
}
}

#endif
