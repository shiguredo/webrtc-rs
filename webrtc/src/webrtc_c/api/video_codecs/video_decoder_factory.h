#pragma once

#include "../environment.h"
#include "sdp_video_format.h"
#include "video_decoder.h"

#include "../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::VideoDecoderFactory
// -------------------------

WEBRTC_DECLARE_UNIQUE(webrtc_VideoDecoderFactory);
struct webrtc_VideoDecoderFactory_unique*
webrtc_CreateBuiltinVideoDecoderFactory();
struct webrtc_VideoDecoderFactory_cbs {
  struct webrtc_SdpVideoFormat_vector* (*GetSupportedFormats)(void* user_data);
  struct webrtc_VideoDecoder_unique* (*Create)(
      struct webrtc_Environment* env,
      struct webrtc_SdpVideoFormat* format,
      void* user_data);
  void (*OnDestroy)(void* user_data);
};
struct webrtc_VideoDecoderFactory_unique* webrtc_VideoDecoderFactory_new(
    const struct webrtc_VideoDecoderFactory_cbs* cbs,
    void* user_data);
struct webrtc_VideoDecoder_unique* webrtc_VideoDecoderFactory_Create(
    struct webrtc_VideoDecoderFactory* self,
    struct webrtc_Environment* env,
    struct webrtc_SdpVideoFormat* format);

#if defined(__cplusplus)
}
#endif
