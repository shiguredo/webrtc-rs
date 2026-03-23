#pragma once

#include <stddef.h>
#include <stdint.h>

#include "../../common.h"
#include "../../modules/video_coding/include/video_codec_interface.h"
#include "../../std.h"
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
WEBRTC_DECLARE_INLINED_VECTOR(webrtc_VideoEncoder_FramerateFraction);
WEBRTC_DECLARE_INLINED_VECTOR(webrtc_VideoFrameBuffer_Type);
WEBRTC_DECLARE_VECTOR_NO_DEFAULT_CTOR(
    webrtc_VideoEncoder_ResolutionBitrateLimits);

struct webrtc_VideoCodec;
struct webrtc_EncodedImage;
struct webrtc_CodecSpecificInfo;
struct webrtc_VideoEncoder_Settings;
struct webrtc_VideoEncoder_RateControlParameters;
struct webrtc_VideoEncoder_EncodedImageCallback;
struct webrtc_VideoFrame;
struct webrtc_VideoFrameType_vector;
struct webrtc_VideoEncoder_QpThresholds;
struct webrtc_VideoEncoder_ScalingSettings;
struct webrtc_VideoEncoder_ResolutionBitrateLimits;
struct webrtc_VideoEncoder_Resolution;

enum webrtc_VideoEncoder_EncodedImageCallback_Result_Error {
  webrtc_VideoEncoder_EncodedImageCallback_Result_Error_OK = 0,
  webrtc_VideoEncoder_EncodedImageCallback_Result_Error_ERROR_SEND_FAILED = 1,
};

WEBRTC_EXPORT extern const int
    webrtc_VideoEncoder_EncoderInfo_MaxFramerateFraction;

WEBRTC_EXPORT extern const int webrtc_VideoFrameBuffer_Type_kNative;
WEBRTC_EXPORT extern const int webrtc_VideoFrameBuffer_Type_kI420;
WEBRTC_EXPORT extern const int webrtc_VideoFrameBuffer_Type_kI420A;
WEBRTC_EXPORT extern const int webrtc_VideoFrameBuffer_Type_kI422;
WEBRTC_EXPORT extern const int webrtc_VideoFrameBuffer_Type_kI444;
WEBRTC_EXPORT extern const int webrtc_VideoFrameBuffer_Type_kI010;
WEBRTC_EXPORT extern const int webrtc_VideoFrameBuffer_Type_kI210;
WEBRTC_EXPORT extern const int webrtc_VideoFrameBuffer_Type_kI410;
WEBRTC_EXPORT extern const int webrtc_VideoFrameBuffer_Type_kNV12;

WEBRTC_EXPORT int webrtc_VideoEncoder_FramerateFraction_value(
    struct webrtc_VideoEncoder_FramerateFraction* self);
WEBRTC_EXPORT void
webrtc_VideoEncoder_FramerateFraction_inlined_vector_push_back_value(
    struct webrtc_VideoEncoder_FramerateFraction_inlined_vector* self,
    int value);
WEBRTC_EXPORT void
webrtc_VideoEncoder_FramerateFraction_inlined_vector_set_value(
    struct webrtc_VideoEncoder_FramerateFraction_inlined_vector* self,
    int index,
    int value);

WEBRTC_EXPORT int webrtc_VideoFrameBuffer_Type_value(
    struct webrtc_VideoFrameBuffer_Type* self);
WEBRTC_EXPORT void webrtc_VideoFrameBuffer_Type_inlined_vector_push_back_value(
    struct webrtc_VideoFrameBuffer_Type_inlined_vector* self,
    int value);
WEBRTC_EXPORT void webrtc_VideoFrameBuffer_Type_inlined_vector_set_value(
    struct webrtc_VideoFrameBuffer_Type_inlined_vector* self,
    int index,
    int value);

WEBRTC_EXPORT struct webrtc_VideoEncoder_QpThresholds*
webrtc_VideoEncoder_QpThresholds_new();
WEBRTC_EXPORT void webrtc_VideoEncoder_QpThresholds_delete(
    struct webrtc_VideoEncoder_QpThresholds* self);
WEBRTC_EXPORT int webrtc_VideoEncoder_QpThresholds_get_low(
    struct webrtc_VideoEncoder_QpThresholds* self);
WEBRTC_EXPORT void webrtc_VideoEncoder_QpThresholds_set_low(
    struct webrtc_VideoEncoder_QpThresholds* self,
    int value);
WEBRTC_EXPORT int webrtc_VideoEncoder_QpThresholds_get_high(
    struct webrtc_VideoEncoder_QpThresholds* self);
WEBRTC_EXPORT void webrtc_VideoEncoder_QpThresholds_set_high(
    struct webrtc_VideoEncoder_QpThresholds* self,
    int value);

WEBRTC_EXPORT struct webrtc_VideoEncoder_ScalingSettings*
webrtc_VideoEncoder_ScalingSettings_new();
WEBRTC_EXPORT void webrtc_VideoEncoder_ScalingSettings_delete(
    struct webrtc_VideoEncoder_ScalingSettings* self);
WEBRTC_EXPORT void webrtc_VideoEncoder_ScalingSettings_get_thresholds(
    struct webrtc_VideoEncoder_ScalingSettings* self,
    int* out_has,
    struct webrtc_VideoEncoder_QpThresholds* out_value);
WEBRTC_EXPORT void webrtc_VideoEncoder_ScalingSettings_set_thresholds(
    struct webrtc_VideoEncoder_ScalingSettings* self,
    int has,
    const struct webrtc_VideoEncoder_QpThresholds* value);
WEBRTC_EXPORT int webrtc_VideoEncoder_ScalingSettings_get_min_pixels_per_frame(
    struct webrtc_VideoEncoder_ScalingSettings* self);
WEBRTC_EXPORT void webrtc_VideoEncoder_ScalingSettings_set_min_pixels_per_frame(
    struct webrtc_VideoEncoder_ScalingSettings* self,
    int value);

WEBRTC_EXPORT struct webrtc_VideoEncoder_ResolutionBitrateLimits*
webrtc_VideoEncoder_ResolutionBitrateLimits_new(int frame_size_pixels,
                                                int min_start_bitrate_bps,
                                                int min_bitrate_bps,
                                                int max_bitrate_bps);
WEBRTC_EXPORT void webrtc_VideoEncoder_ResolutionBitrateLimits_delete(
    struct webrtc_VideoEncoder_ResolutionBitrateLimits* self);
WEBRTC_EXPORT int
webrtc_VideoEncoder_ResolutionBitrateLimits_get_frame_size_pixels(
    struct webrtc_VideoEncoder_ResolutionBitrateLimits* self);
WEBRTC_EXPORT void
webrtc_VideoEncoder_ResolutionBitrateLimits_set_frame_size_pixels(
    struct webrtc_VideoEncoder_ResolutionBitrateLimits* self,
    int value);
WEBRTC_EXPORT int
webrtc_VideoEncoder_ResolutionBitrateLimits_get_min_start_bitrate_bps(
    struct webrtc_VideoEncoder_ResolutionBitrateLimits* self);
WEBRTC_EXPORT void
webrtc_VideoEncoder_ResolutionBitrateLimits_set_min_start_bitrate_bps(
    struct webrtc_VideoEncoder_ResolutionBitrateLimits* self,
    int value);
WEBRTC_EXPORT int
webrtc_VideoEncoder_ResolutionBitrateLimits_get_min_bitrate_bps(
    struct webrtc_VideoEncoder_ResolutionBitrateLimits* self);
WEBRTC_EXPORT void
webrtc_VideoEncoder_ResolutionBitrateLimits_set_min_bitrate_bps(
    struct webrtc_VideoEncoder_ResolutionBitrateLimits* self,
    int value);
WEBRTC_EXPORT int
webrtc_VideoEncoder_ResolutionBitrateLimits_get_max_bitrate_bps(
    struct webrtc_VideoEncoder_ResolutionBitrateLimits* self);
WEBRTC_EXPORT void
webrtc_VideoEncoder_ResolutionBitrateLimits_set_max_bitrate_bps(
    struct webrtc_VideoEncoder_ResolutionBitrateLimits* self,
    int value);

WEBRTC_EXPORT struct webrtc_VideoEncoder_Resolution*
webrtc_VideoEncoder_Resolution_new(int width, int height);
WEBRTC_EXPORT void webrtc_VideoEncoder_Resolution_delete(
    struct webrtc_VideoEncoder_Resolution* self);
WEBRTC_EXPORT int webrtc_VideoEncoder_Resolution_get_width(
    struct webrtc_VideoEncoder_Resolution* self);
WEBRTC_EXPORT void webrtc_VideoEncoder_Resolution_set_width(
    struct webrtc_VideoEncoder_Resolution* self,
    int value);
WEBRTC_EXPORT int webrtc_VideoEncoder_Resolution_get_height(
    struct webrtc_VideoEncoder_Resolution* self);
WEBRTC_EXPORT void webrtc_VideoEncoder_Resolution_set_height(
    struct webrtc_VideoEncoder_Resolution* self,
    int value);

WEBRTC_EXPORT struct webrtc_VideoEncoder_EncoderInfo_unique*
webrtc_VideoEncoder_EncoderInfo_new();
WEBRTC_EXPORT struct std_string_unique*
webrtc_VideoEncoder_EncoderInfo_get_implementation_name(
    struct webrtc_VideoEncoder_EncoderInfo* self);
WEBRTC_EXPORT void webrtc_VideoEncoder_EncoderInfo_set_implementation_name(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    struct std_string_unique* name);
WEBRTC_EXPORT int webrtc_VideoEncoder_EncoderInfo_get_is_hardware_accelerated(
    struct webrtc_VideoEncoder_EncoderInfo* self);
WEBRTC_EXPORT void webrtc_VideoEncoder_EncoderInfo_set_is_hardware_accelerated(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    int value);
WEBRTC_EXPORT struct webrtc_VideoEncoder_ScalingSettings*
webrtc_VideoEncoder_EncoderInfo_get_scaling_settings(
    struct webrtc_VideoEncoder_EncoderInfo* self);
WEBRTC_EXPORT void webrtc_VideoEncoder_EncoderInfo_set_scaling_settings(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    const struct webrtc_VideoEncoder_ScalingSettings* value);
WEBRTC_EXPORT uint32_t
webrtc_VideoEncoder_EncoderInfo_get_requested_resolution_alignment(
    struct webrtc_VideoEncoder_EncoderInfo* self);
WEBRTC_EXPORT void
webrtc_VideoEncoder_EncoderInfo_set_requested_resolution_alignment(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    uint32_t value);
WEBRTC_EXPORT int
webrtc_VideoEncoder_EncoderInfo_get_apply_alignment_to_all_simulcast_layers(
    struct webrtc_VideoEncoder_EncoderInfo* self);
WEBRTC_EXPORT void
webrtc_VideoEncoder_EncoderInfo_set_apply_alignment_to_all_simulcast_layers(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    int value);
WEBRTC_EXPORT int webrtc_VideoEncoder_EncoderInfo_get_supports_native_handle(
    struct webrtc_VideoEncoder_EncoderInfo* self);
WEBRTC_EXPORT void webrtc_VideoEncoder_EncoderInfo_set_supports_native_handle(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    int value);
WEBRTC_EXPORT int
webrtc_VideoEncoder_EncoderInfo_get_has_trusted_rate_controller(
    struct webrtc_VideoEncoder_EncoderInfo* self);
WEBRTC_EXPORT void
webrtc_VideoEncoder_EncoderInfo_set_has_trusted_rate_controller(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    int value);
WEBRTC_EXPORT struct webrtc_VideoEncoder_FramerateFraction_inlined_vector*
webrtc_VideoEncoder_EncoderInfo_get_fps_allocation(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    int spatial_index);
WEBRTC_EXPORT struct webrtc_VideoEncoder_ResolutionBitrateLimits_vector*
webrtc_VideoEncoder_EncoderInfo_get_resolution_bitrate_limits(
    struct webrtc_VideoEncoder_EncoderInfo* self);
WEBRTC_EXPORT int webrtc_VideoEncoder_EncoderInfo_get_supports_simulcast(
    struct webrtc_VideoEncoder_EncoderInfo* self);
WEBRTC_EXPORT void webrtc_VideoEncoder_EncoderInfo_set_supports_simulcast(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    int value);
WEBRTC_EXPORT struct webrtc_VideoFrameBuffer_Type_inlined_vector*
webrtc_VideoEncoder_EncoderInfo_get_preferred_pixel_formats(
    struct webrtc_VideoEncoder_EncoderInfo* self);
WEBRTC_EXPORT void webrtc_VideoEncoder_EncoderInfo_get_is_qp_trusted(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    int* out_has,
    int* out_value);
WEBRTC_EXPORT void webrtc_VideoEncoder_EncoderInfo_set_is_qp_trusted(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    int has,
    const int* value);
WEBRTC_EXPORT void webrtc_VideoEncoder_EncoderInfo_get_min_qp(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    int* out_has,
    int* out_value);
WEBRTC_EXPORT void webrtc_VideoEncoder_EncoderInfo_set_min_qp(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    int has,
    const int* value);
WEBRTC_EXPORT void webrtc_VideoEncoder_EncoderInfo_get_mapped_resolution(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    int* out_has,
    struct webrtc_VideoEncoder_Resolution* out_value);
WEBRTC_EXPORT void webrtc_VideoEncoder_EncoderInfo_set_mapped_resolution(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    int has,
    const struct webrtc_VideoEncoder_Resolution* value);
WEBRTC_EXPORT struct std_string_unique*
webrtc_VideoEncoder_EncoderInfo_ToString(
    struct webrtc_VideoEncoder_EncoderInfo* self);
WEBRTC_EXPORT void
webrtc_VideoEncoder_EncoderInfo_GetEncoderBitrateLimitsForResolution(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    int frame_size_pixels,
    int* out_has,
    struct webrtc_VideoEncoder_ResolutionBitrateLimits* out_value);

WEBRTC_EXPORT struct webrtc_VideoEncoder_EncodedImageCallback_Result_unique*
webrtc_VideoEncoder_EncodedImageCallback_Result_new(int error);
WEBRTC_EXPORT struct webrtc_VideoEncoder_EncodedImageCallback_Result_unique*
webrtc_VideoEncoder_EncodedImageCallback_Result_new_with_frame_id(
    int error,
    uint32_t frame_id);
WEBRTC_EXPORT int webrtc_VideoEncoder_EncodedImageCallback_Result_error(
    struct webrtc_VideoEncoder_EncodedImageCallback_Result* self);
WEBRTC_EXPORT void webrtc_VideoEncoder_EncodedImageCallback_Result_set_error(
    struct webrtc_VideoEncoder_EncodedImageCallback_Result* self,
    int error);
WEBRTC_EXPORT uint32_t webrtc_VideoEncoder_EncodedImageCallback_Result_frame_id(
    struct webrtc_VideoEncoder_EncodedImageCallback_Result* self);
WEBRTC_EXPORT void webrtc_VideoEncoder_EncodedImageCallback_Result_set_frame_id(
    struct webrtc_VideoEncoder_EncodedImageCallback_Result* self,
    uint32_t frame_id);
WEBRTC_EXPORT int
webrtc_VideoEncoder_EncodedImageCallback_Result_drop_next_frame(
    struct webrtc_VideoEncoder_EncodedImageCallback_Result* self);
WEBRTC_EXPORT void
webrtc_VideoEncoder_EncodedImageCallback_Result_set_drop_next_frame(
    struct webrtc_VideoEncoder_EncodedImageCallback_Result* self,
    int drop_next_frame);

struct webrtc_VideoEncoder_EncodedImageCallback_cbs {
  struct webrtc_VideoEncoder_EncodedImageCallback_Result_unique* (
      *OnEncodedImage)(struct webrtc_EncodedImage* encoded_image,
                       struct webrtc_CodecSpecificInfo* codec_specific_info,
                       void* user_data);
  void (*OnDestroy)(void* user_data);
};

WEBRTC_EXPORT struct webrtc_VideoEncoder_EncodedImageCallback*
webrtc_VideoEncoder_EncodedImageCallback_new(
    const struct webrtc_VideoEncoder_EncodedImageCallback_cbs* cbs,
    void* user_data);
WEBRTC_EXPORT void webrtc_VideoEncoder_EncodedImageCallback_delete(
    struct webrtc_VideoEncoder_EncodedImageCallback* self);
WEBRTC_EXPORT struct webrtc_VideoEncoder_EncodedImageCallback_Result_unique*
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

WEBRTC_EXPORT struct webrtc_VideoEncoder_unique* webrtc_VideoEncoder_new(
    const struct webrtc_VideoEncoder_cbs* cbs,
    void* user_data);
WEBRTC_EXPORT int32_t
webrtc_VideoEncoder_InitEncode(struct webrtc_VideoEncoder* self,
                               struct webrtc_VideoCodec* codec_settings,
                               struct webrtc_VideoEncoder_Settings* settings);
WEBRTC_EXPORT int32_t
webrtc_VideoEncoder_Encode(struct webrtc_VideoEncoder* self,
                           struct webrtc_VideoFrame* frame,
                           struct webrtc_VideoFrameType_vector* frame_types);
WEBRTC_EXPORT int32_t webrtc_VideoEncoder_RegisterEncodeCompleteCallback(
    struct webrtc_VideoEncoder* self,
    struct webrtc_VideoEncoder_EncodedImageCallback* callback);
WEBRTC_EXPORT int32_t
webrtc_VideoEncoder_Release(struct webrtc_VideoEncoder* self);
WEBRTC_EXPORT void webrtc_VideoEncoder_SetRates(
    struct webrtc_VideoEncoder* self,
    struct webrtc_VideoEncoder_RateControlParameters* parameters);
WEBRTC_EXPORT struct webrtc_VideoEncoder_EncoderInfo_unique*
webrtc_VideoEncoder_GetEncoderInfo(struct webrtc_VideoEncoder* self);

#if defined(__cplusplus)
}
#endif
