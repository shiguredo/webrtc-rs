#pragma once

#include <stddef.h>
#include <stdint.h>

#include "../../common.h"
#include "../../std.h"
#include "codec_specific_info.h"
#include "video_codec.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::VideoEncoder
// -------------------------

WEBRTC_DECLARE_UNIQUE(webrtc_VideoEncoder);
WEBRTC_DECLARE_UNIQUE(webrtc_VideoEncoder_EncoderInfo);
WEBRTC_DECLARE_UNIQUE(webrtc_VideoEncoder_EncodedImageCallback_Result);

struct webrtc_VideoCodec;
struct webrtc_EncodedImage;
struct webrtc_CodecSpecificInfo;
struct webrtc_VideoEncoder_Settings;
struct webrtc_VideoEncoder_RateControlParameters;
struct webrtc_VideoEncoder_EncodedImageCallback;
struct webrtc_VideoFrame;
struct webrtc_VideoFrameType_vector;

enum webrtc_VideoEncoder_EncodedImageCallback_Result_Error {
  webrtc_VideoEncoder_EncodedImageCallback_Result_Error_OK = 0,
  webrtc_VideoEncoder_EncodedImageCallback_Result_Error_ERROR_SEND_FAILED = 1,
};

struct webrtc_VideoEncoder_EncoderInfo_unique*
webrtc_VideoEncoder_EncoderInfo_new();
struct std_string_unique* webrtc_VideoEncoder_EncoderInfo_get_implementation_name(
    struct webrtc_VideoEncoder_EncoderInfo* self);
void webrtc_VideoEncoder_EncoderInfo_set_implementation_name(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    struct std_string_unique* name);
int webrtc_VideoEncoder_EncoderInfo_get_is_hardware_accelerated(
    struct webrtc_VideoEncoder_EncoderInfo* self);
void webrtc_VideoEncoder_EncoderInfo_set_is_hardware_accelerated(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    int value);

struct webrtc_VideoEncoder_EncodedImageCallback_Result_unique*
webrtc_VideoEncoder_EncodedImageCallback_Result_new(
    int error);
struct webrtc_VideoEncoder_EncodedImageCallback_Result_unique*
webrtc_VideoEncoder_EncodedImageCallback_Result_new_with_frame_id(
    int error,
    uint32_t frame_id);
int webrtc_VideoEncoder_EncodedImageCallback_Result_error(
    struct webrtc_VideoEncoder_EncodedImageCallback_Result* self);
void webrtc_VideoEncoder_EncodedImageCallback_Result_set_error(
    struct webrtc_VideoEncoder_EncodedImageCallback_Result* self,
    int error);
uint32_t webrtc_VideoEncoder_EncodedImageCallback_Result_frame_id(
    struct webrtc_VideoEncoder_EncodedImageCallback_Result* self);
void webrtc_VideoEncoder_EncodedImageCallback_Result_set_frame_id(
    struct webrtc_VideoEncoder_EncodedImageCallback_Result* self,
    uint32_t frame_id);
int webrtc_VideoEncoder_EncodedImageCallback_Result_drop_next_frame(
    struct webrtc_VideoEncoder_EncodedImageCallback_Result* self);
void webrtc_VideoEncoder_EncodedImageCallback_Result_set_drop_next_frame(
    struct webrtc_VideoEncoder_EncodedImageCallback_Result* self,
    int drop_next_frame);

struct webrtc_VideoEncoder_EncodedImageCallback_cbs {
  struct webrtc_VideoEncoder_EncodedImageCallback_Result_unique* (*OnEncodedImage)(
      struct webrtc_EncodedImage* encoded_image,
      struct webrtc_CodecSpecificInfo* codec_specific_info,
      void* user_data);
  void (*OnDestroy)(void* user_data);
};

struct webrtc_VideoEncoder_EncodedImageCallback*
webrtc_VideoEncoder_EncodedImageCallback_new(
    const struct webrtc_VideoEncoder_EncodedImageCallback_cbs* cbs,
    void* user_data);
void webrtc_VideoEncoder_EncodedImageCallback_delete(
    struct webrtc_VideoEncoder_EncodedImageCallback* self);
struct webrtc_VideoEncoder_EncodedImageCallback_Result_unique*
webrtc_VideoEncoder_EncodedImageCallback_OnEncodedImage(
    struct webrtc_VideoEncoder_EncodedImageCallback* self,
    struct webrtc_EncodedImage* encoded_image,
    struct webrtc_CodecSpecificInfo* codec_specific_info);

struct webrtc_VideoEncoder_cbs {
  int32_t (*InitEncode)(struct webrtc_VideoCodec* codec_settings,
                        struct webrtc_VideoEncoder_Settings* settings,
                        void* user_data);
  int32_t (*Encode)(struct webrtc_VideoFrame* frame,
                    struct webrtc_VideoFrameType_vector* frame_types,
                    void* user_data);
  int32_t (*RegisterEncodeCompleteCallback)(
      struct webrtc_VideoEncoder_EncodedImageCallback* callback,
      void* user_data);
  int32_t (*Release)(void* user_data);
  void (*SetRates)(struct webrtc_VideoEncoder_RateControlParameters* parameters,
                   void* user_data);
  struct webrtc_VideoEncoder_EncoderInfo_unique* (*GetEncoderInfo)(
      void* user_data);
  void (*OnDestroy)(void* user_data);
};

struct webrtc_VideoEncoder_unique* webrtc_VideoEncoder_new(
    const struct webrtc_VideoEncoder_cbs* cbs,
    void* user_data);
int32_t webrtc_VideoEncoder_InitEncode(
    struct webrtc_VideoEncoder* self,
    struct webrtc_VideoCodec* codec_settings,
    struct webrtc_VideoEncoder_Settings* settings);
int32_t webrtc_VideoEncoder_Encode(
    struct webrtc_VideoEncoder* self,
    struct webrtc_VideoFrame* frame,
    struct webrtc_VideoFrameType_vector* frame_types);
int32_t webrtc_VideoEncoder_RegisterEncodeCompleteCallback(
    struct webrtc_VideoEncoder* self,
    struct webrtc_VideoEncoder_EncodedImageCallback* callback);
void webrtc_VideoEncoder_SetRates(
    struct webrtc_VideoEncoder* self,
    struct webrtc_VideoEncoder_RateControlParameters* parameters);
struct webrtc_VideoEncoder_EncoderInfo_unique* webrtc_VideoEncoder_GetEncoderInfo(
    struct webrtc_VideoEncoder* self);

#if defined(__cplusplus)
}
#endif
