#pragma once

#include <stddef.h>
#include <stdint.h>

#include "../../common.h"
#include "../../std.h"
#include "../video/encoded_image.h"
#include "../video/video_frame.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::VideoDecoder
// -------------------------

WEBRTC_DECLARE_UNIQUE(webrtc_VideoDecoder);
WEBRTC_DECLARE_UNIQUE(webrtc_VideoDecoder_DecoderInfo);

struct webrtc_EncodedImage;
struct webrtc_VideoDecoder_Settings;
struct webrtc_VideoDecoder_DecodedImageCallback;

WEBRTC_EXPORT struct webrtc_VideoDecoder_DecoderInfo_unique*
webrtc_VideoDecoder_DecoderInfo_new();
WEBRTC_EXPORT struct std_string_unique*
webrtc_VideoDecoder_DecoderInfo_get_implementation_name(
    struct webrtc_VideoDecoder_DecoderInfo* self);
WEBRTC_EXPORT void webrtc_VideoDecoder_DecoderInfo_set_implementation_name(
    struct webrtc_VideoDecoder_DecoderInfo* self,
    struct std_string_unique* name);
WEBRTC_EXPORT int webrtc_VideoDecoder_DecoderInfo_get_is_hardware_accelerated(
    struct webrtc_VideoDecoder_DecoderInfo* self);
WEBRTC_EXPORT void webrtc_VideoDecoder_DecoderInfo_set_is_hardware_accelerated(
    struct webrtc_VideoDecoder_DecoderInfo* self,
    int value);
WEBRTC_EXPORT int webrtc_VideoDecoder_Settings_number_of_cores(
    struct webrtc_VideoDecoder_Settings* self);
WEBRTC_EXPORT int webrtc_VideoDecoder_Settings_codec_type(
    struct webrtc_VideoDecoder_Settings* self);
WEBRTC_EXPORT int webrtc_VideoDecoder_Settings_has_buffer_pool_size(
    struct webrtc_VideoDecoder_Settings* self);
WEBRTC_EXPORT int webrtc_VideoDecoder_Settings_buffer_pool_size(
    struct webrtc_VideoDecoder_Settings* self);
WEBRTC_EXPORT int webrtc_VideoDecoder_Settings_max_render_resolution_width(
    struct webrtc_VideoDecoder_Settings* self);
WEBRTC_EXPORT int webrtc_VideoDecoder_Settings_max_render_resolution_height(
    struct webrtc_VideoDecoder_Settings* self);
WEBRTC_EXPORT void webrtc_VideoDecoder_DecodedImageCallback_Decoded(
    struct webrtc_VideoDecoder_DecodedImageCallback* self,
    struct webrtc_VideoFrame* decoded_image);

struct webrtc_VideoDecoder_cbs {
  int (*Configure)(struct webrtc_VideoDecoder_Settings* settings,
                   void* user_data);
  int32_t (*Decode)(struct webrtc_EncodedImage* input_image,
                    int64_t render_time_ms,
                    void* user_data);
  int32_t (*RegisterDecodeCompleteCallback)(
      struct webrtc_VideoDecoder_DecodedImageCallback* callback,
      void* user_data);
  int32_t (*Release)(void* user_data);
  struct webrtc_VideoDecoder_DecoderInfo_unique* (*GetDecoderInfo)(
      void* user_data);
  void (*OnDestroy)(void* user_data);
};

WEBRTC_EXPORT struct webrtc_VideoDecoder_unique* webrtc_VideoDecoder_new(
    const struct webrtc_VideoDecoder_cbs* cbs,
    void* user_data);
WEBRTC_EXPORT int webrtc_VideoDecoder_Configure(
    struct webrtc_VideoDecoder* self,
    struct webrtc_VideoDecoder_Settings* settings);
WEBRTC_EXPORT int32_t
webrtc_VideoDecoder_Decode(struct webrtc_VideoDecoder* self,
                           struct webrtc_EncodedImage* input_image,
                           int64_t render_time_ms);
WEBRTC_EXPORT int32_t webrtc_VideoDecoder_RegisterDecodeCompleteCallback(
    struct webrtc_VideoDecoder* self,
    struct webrtc_VideoDecoder_DecodedImageCallback* callback);
WEBRTC_EXPORT int32_t
webrtc_VideoDecoder_Release(struct webrtc_VideoDecoder* self);
WEBRTC_EXPORT struct webrtc_VideoDecoder_DecoderInfo_unique*
webrtc_VideoDecoder_GetDecoderInfo(struct webrtc_VideoDecoder* self);

#if defined(__cplusplus)
}
#endif
