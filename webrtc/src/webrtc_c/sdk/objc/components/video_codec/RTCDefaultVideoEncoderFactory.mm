#include "RTCDefaultVideoEncoderFactory.h"

#if !defined(WEBRTC_IOS) && !defined(WEBRTC_MAC)

extern "C" {

WEBRTC_EXPORT struct webrtc_objc_RTCVideoEncoderFactory*
webrtc_objc_RTCDefaultVideoEncoderFactory_new(void) {
  return nullptr;
}

WEBRTC_EXPORT void webrtc_objc_RTCVideoEncoderFactory_release(
    struct webrtc_objc_RTCVideoEncoderFactory* self) {
  (void)self;
}
}

#else

#import <Foundation/Foundation.h>

#import <sdk/objc/components/video_codec/RTCDefaultVideoEncoderFactory.h>

namespace {

struct webrtc_objc_RTCVideoEncoderFactory* RetainRTCVideoEncoderFactory(
    id<RTC_OBJC_TYPE(RTCVideoEncoderFactory)> self) {
  return (struct webrtc_objc_RTCVideoEncoderFactory*)CFBridgingRetain(self);
}

}  // namespace

extern "C" {

WEBRTC_EXPORT struct webrtc_objc_RTCVideoEncoderFactory*
webrtc_objc_RTCDefaultVideoEncoderFactory_new(void) {
  return RetainRTCVideoEncoderFactory(
      [[RTC_OBJC_TYPE(RTCDefaultVideoEncoderFactory) alloc] init]);
}

WEBRTC_EXPORT void webrtc_objc_RTCVideoEncoderFactory_release(
    struct webrtc_objc_RTCVideoEncoderFactory* self) {
  if (self == nullptr) {
    return;
  }
  CFBridgingRelease(reinterpret_cast<CFTypeRef>(self));
}
}

#endif
