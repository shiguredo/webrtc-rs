#pragma once

#include "../environment.h"
#include "sdp_video_format.h"
#include "video_encoder.h"

#include "../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::VideoEncoderFactory
// -------------------------

WEBRTC_DECLARE_UNIQUE(webrtc_VideoEncoderFactory);
struct webrtc_VideoEncoderFactory_unique*
webrtc_CreateBuiltinVideoEncoderFactory();
struct webrtc_VideoEncoderFactory_cbs {
  struct webrtc_SdpVideoFormat_vector* (*GetSupportedFormats)(void* user_data);
  struct webrtc_VideoEncoder_unique* (*Create)(
      struct webrtc_Environment* env,
      struct webrtc_SdpVideoFormat* format,
      void* user_data);
  void (*OnDestroy)(void* user_data);
};
struct webrtc_VideoEncoderFactory_unique* webrtc_VideoEncoderFactory_new(
    const struct webrtc_VideoEncoderFactory_cbs* cbs,
    void* user_data);
struct webrtc_VideoEncoder_unique* webrtc_VideoEncoderFactory_Create(
    struct webrtc_VideoEncoderFactory* self,
    struct webrtc_Environment* env,
    struct webrtc_SdpVideoFormat* format);

#if defined(__cplusplus)
}
#endif
