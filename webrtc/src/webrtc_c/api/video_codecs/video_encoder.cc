#include "video_encoder.h"

#include <stddef.h>
#include <stdint.h>
#include <cassert>
#include <memory>
#include <string>
#include <vector>

// WebRTC
#include <absl/container/inlined_vector.h>
#include <api/units/data_rate.h>
#include <api/video/encoded_image.h>
#include <api/video/video_bitrate_allocation.h>
#include <api/video/video_codec_constants.h>
#include <api/video/video_frame.h>
#include <api/video/video_frame_buffer.h>
#include <api/video/video_frame_type.h>
#include <api/video_codecs/video_codec.h>
#include <api/video_codecs/video_encoder.h>
#include <modules/video_coding/include/video_codec_interface.h>
#include <modules/video_coding/include/video_error_codes.h>

#include "../../common.h"
#include "../../common.impl.h"
#include "../../std.h"
#include "../../std.impl.h"

namespace {

class EncodedImageCallbackImpl : public webrtc::EncodedImageCallback {
 public:
  EncodedImageCallbackImpl(
      const webrtc_VideoEncoder_EncodedImageCallback_cbs* cbs,
      void* user_data)
      : user_data_(user_data) {
    if (cbs != nullptr) {
      cbs_ = *cbs;
    }
  }

  ~EncodedImageCallbackImpl() override {
    if (cbs_.OnDestroy != nullptr) {
      cbs_.OnDestroy(user_data_);
    }
  }

  Result OnEncodedImage(
      const webrtc::EncodedImage& encoded_image,
      const webrtc::CodecSpecificInfo* codec_specific_info) override {
    if (cbs_.OnEncodedImage == nullptr) {
      return Result(Result::OK);
    }
    auto raw_result = cbs_.OnEncodedImage(
        reinterpret_cast<struct webrtc_EncodedImage*>(
            const_cast<webrtc::EncodedImage*>(&encoded_image)),
        reinterpret_cast<struct webrtc_CodecSpecificInfo*>(
            const_cast<webrtc::CodecSpecificInfo*>(codec_specific_info)),
        user_data_);
    assert(raw_result != nullptr);
    if (raw_result == nullptr) {
      return Result(Result::OK);
    }
    auto result_unique = reinterpret_cast<
        struct webrtc_VideoEncoder_EncodedImageCallback_Result_unique*>(
        raw_result);
    auto raw = reinterpret_cast<Result*>(
        webrtc_VideoEncoder_EncodedImageCallback_Result_unique_get(
            result_unique));
    Result result(Result::OK);
    if (raw != nullptr) {
      result = *raw;
    }
    webrtc_VideoEncoder_EncodedImageCallback_Result_unique_delete(
        result_unique);
    return result;
  }

 private:
  webrtc_VideoEncoder_EncodedImageCallback_cbs cbs_{};
  void* user_data_ = nullptr;
};

class VideoEncoderImpl : public webrtc::VideoEncoder {
 public:
  VideoEncoderImpl(const webrtc_VideoEncoder_cbs* cbs, void* user_data)
      : user_data_(user_data) {
    if (cbs != nullptr) {
      cbs_ = *cbs;
    }
  }

  ~VideoEncoderImpl() override {
    if (cbs_.OnDestroy != nullptr) {
      cbs_.OnDestroy(user_data_);
    }
  }

  int InitEncode(const webrtc::VideoCodec* codec_settings,
                 const webrtc::VideoEncoder::Settings& settings) override {
    if (cbs_.InitEncode != nullptr) {
      return cbs_.InitEncode(
          reinterpret_cast<struct webrtc_VideoCodec*>(
              const_cast<webrtc::VideoCodec*>(codec_settings)),
          reinterpret_cast<struct webrtc_VideoEncoder_Settings*>(
              const_cast<webrtc::VideoEncoder::Settings*>(&settings)),
          user_data_);
    }
    return WEBRTC_VIDEO_CODEC_OK;
  }

  int32_t Encode(
      const webrtc::VideoFrame& frame,
      const std::vector<webrtc::VideoFrameType>* frame_types) override {
    if (cbs_.Encode != nullptr) {
      return cbs_.Encode(
          reinterpret_cast<struct webrtc_VideoFrame*>(
              const_cast<webrtc::VideoFrame*>(&frame)),
          reinterpret_cast<struct webrtc_VideoFrameType_vector*>(
              const_cast<std::vector<webrtc::VideoFrameType>*>(frame_types)),
          user_data_);
    }
    return WEBRTC_VIDEO_CODEC_OK;
  }

  int32_t RegisterEncodeCompleteCallback(
      webrtc::EncodedImageCallback* callback) override {
    if (cbs_.RegisterEncodeCompleteCallback != nullptr) {
      return cbs_.RegisterEncodeCompleteCallback(
          reinterpret_cast<struct webrtc_VideoEncoder_EncodedImageCallback*>(
              callback),
          user_data_);
    }
    return WEBRTC_VIDEO_CODEC_OK;
  }

  int32_t Release() override {
    if (cbs_.Release != nullptr) {
      return cbs_.Release(user_data_);
    }
    return WEBRTC_VIDEO_CODEC_OK;
  }

  void SetRates(
      const webrtc::VideoEncoder::RateControlParameters& parameters) override {
    if (cbs_.SetRates != nullptr) {
      cbs_.SetRates(
          reinterpret_cast<struct webrtc_VideoEncoder_RateControlParameters*>(
              const_cast<webrtc::VideoEncoder::RateControlParameters*>(
                  &parameters)),
          user_data_);
    }
  }

  webrtc::VideoEncoder::EncoderInfo GetEncoderInfo() const override {
    webrtc::VideoEncoder::EncoderInfo info;
    if (cbs_.GetEncoderInfo != nullptr) {
      auto raw_info = cbs_.GetEncoderInfo(user_data_);
      if (raw_info != nullptr) {
        auto raw =
            reinterpret_cast<struct webrtc_VideoEncoder_EncoderInfo_unique*>(
                raw_info);
        auto c_info = reinterpret_cast<webrtc::VideoEncoder::EncoderInfo*>(
            webrtc_VideoEncoder_EncoderInfo_unique_get(raw));
        if (c_info != nullptr) {
          info = *c_info;
        }
        webrtc_VideoEncoder_EncoderInfo_unique_delete(raw);
      }
    }
    return info;
  }

 private:
  webrtc_VideoEncoder_cbs cbs_{};
  void* user_data_ = nullptr;
};

}  // namespace

extern "C" {
WEBRTC_DEFINE_UNIQUE(webrtc_VideoEncoder, webrtc::VideoEncoder);
WEBRTC_DEFINE_UNIQUE(webrtc_VideoEncoder_EncoderInfo,
                     webrtc::VideoEncoder::EncoderInfo);
WEBRTC_DEFINE_UNIQUE(webrtc_VideoEncoder_EncodedImageCallback_Result,
                     webrtc::EncodedImageCallback::Result);
WEBRTC_DEFINE_INLINED_VECTOR(webrtc_VideoEncoder_FramerateFraction,
                             uint8_t,
                             webrtc::kMaxTemporalStreams);
WEBRTC_DEFINE_INLINED_VECTOR(webrtc_VideoFrameBuffer_Type,
                             webrtc::VideoFrameBuffer::Type,
                             webrtc::kMaxPreferredPixelFormats);
WEBRTC_DEFINE_VECTOR_NO_DEFAULT_CTOR(
    webrtc_VideoEncoder_ResolutionBitrateLimits,
    webrtc::VideoEncoder::ResolutionBitrateLimits);

WEBRTC_EXPORT const int webrtc_VideoEncoder_EncoderInfo_MaxFramerateFraction =
    static_cast<int>(webrtc::VideoEncoder::EncoderInfo::kMaxFramerateFraction);

WEBRTC_EXPORT const int webrtc_VideoFrameBuffer_Type_kNative =
    static_cast<int>(webrtc::VideoFrameBuffer::Type::kNative);
WEBRTC_EXPORT const int webrtc_VideoFrameBuffer_Type_kI420 =
    static_cast<int>(webrtc::VideoFrameBuffer::Type::kI420);
WEBRTC_EXPORT const int webrtc_VideoFrameBuffer_Type_kI420A =
    static_cast<int>(webrtc::VideoFrameBuffer::Type::kI420A);
WEBRTC_EXPORT const int webrtc_VideoFrameBuffer_Type_kI422 =
    static_cast<int>(webrtc::VideoFrameBuffer::Type::kI422);
WEBRTC_EXPORT const int webrtc_VideoFrameBuffer_Type_kI444 =
    static_cast<int>(webrtc::VideoFrameBuffer::Type::kI444);
WEBRTC_EXPORT const int webrtc_VideoFrameBuffer_Type_kI010 =
    static_cast<int>(webrtc::VideoFrameBuffer::Type::kI010);
WEBRTC_EXPORT const int webrtc_VideoFrameBuffer_Type_kI210 =
    static_cast<int>(webrtc::VideoFrameBuffer::Type::kI210);
WEBRTC_EXPORT const int webrtc_VideoFrameBuffer_Type_kI410 =
    static_cast<int>(webrtc::VideoFrameBuffer::Type::kI410);
WEBRTC_EXPORT const int webrtc_VideoFrameBuffer_Type_kNV12 =
    static_cast<int>(webrtc::VideoFrameBuffer::Type::kNV12);

WEBRTC_EXPORT int webrtc_VideoEncoder_FramerateFraction_value(
    struct webrtc_VideoEncoder_FramerateFraction* self) {
  auto value = reinterpret_cast<uint8_t*>(self);
  return static_cast<int>(*value);
}

WEBRTC_EXPORT void
webrtc_VideoEncoder_FramerateFraction_inlined_vector_push_back_value(
    struct webrtc_VideoEncoder_FramerateFraction_inlined_vector* self,
    int value) {
  auto vec = reinterpret_cast<
      absl::InlinedVector<uint8_t, webrtc::kMaxTemporalStreams>*>(self);
  vec->push_back(static_cast<uint8_t>(value));
}

WEBRTC_EXPORT void
webrtc_VideoEncoder_FramerateFraction_inlined_vector_set_value(
    struct webrtc_VideoEncoder_FramerateFraction_inlined_vector* self,
    int index,
    int value) {
  auto vec = reinterpret_cast<
      absl::InlinedVector<uint8_t, webrtc::kMaxTemporalStreams>*>(self);
  (*vec)[index] = static_cast<uint8_t>(value);
}

WEBRTC_EXPORT int webrtc_VideoFrameBuffer_Type_value(
    struct webrtc_VideoFrameBuffer_Type* self) {
  auto value = reinterpret_cast<webrtc::VideoFrameBuffer::Type*>(self);
  return static_cast<int>(*value);
}

WEBRTC_EXPORT void webrtc_VideoFrameBuffer_Type_inlined_vector_push_back_value(
    struct webrtc_VideoFrameBuffer_Type_inlined_vector* self,
    int value) {
  auto vec =
      reinterpret_cast<absl::InlinedVector<webrtc::VideoFrameBuffer::Type,
                                           webrtc::kMaxPreferredPixelFormats>*>(
          self);
  vec->push_back(static_cast<webrtc::VideoFrameBuffer::Type>(value));
}

WEBRTC_EXPORT void webrtc_VideoFrameBuffer_Type_inlined_vector_set_value(
    struct webrtc_VideoFrameBuffer_Type_inlined_vector* self,
    int index,
    int value) {
  auto vec =
      reinterpret_cast<absl::InlinedVector<webrtc::VideoFrameBuffer::Type,
                                           webrtc::kMaxPreferredPixelFormats>*>(
          self);
  (*vec)[index] = static_cast<webrtc::VideoFrameBuffer::Type>(value);
}

WEBRTC_EXPORT struct webrtc_VideoEncoder_QpThresholds*
webrtc_VideoEncoder_QpThresholds_new() {
  auto thresholds = new webrtc::VideoEncoder::QpThresholds();
  return reinterpret_cast<struct webrtc_VideoEncoder_QpThresholds*>(thresholds);
}

WEBRTC_EXPORT void webrtc_VideoEncoder_QpThresholds_delete(
    struct webrtc_VideoEncoder_QpThresholds* self) {
  auto thresholds = reinterpret_cast<webrtc::VideoEncoder::QpThresholds*>(self);
  delete thresholds;
}

WEBRTC_EXPORT int webrtc_VideoEncoder_QpThresholds_get_low(
    struct webrtc_VideoEncoder_QpThresholds* self) {
  auto thresholds = reinterpret_cast<webrtc::VideoEncoder::QpThresholds*>(self);
  return thresholds->low;
}

WEBRTC_EXPORT void webrtc_VideoEncoder_QpThresholds_set_low(
    struct webrtc_VideoEncoder_QpThresholds* self,
    int value) {
  auto thresholds = reinterpret_cast<webrtc::VideoEncoder::QpThresholds*>(self);
  thresholds->low = value;
}

WEBRTC_EXPORT int webrtc_VideoEncoder_QpThresholds_get_high(
    struct webrtc_VideoEncoder_QpThresholds* self) {
  auto thresholds = reinterpret_cast<webrtc::VideoEncoder::QpThresholds*>(self);
  return thresholds->high;
}

WEBRTC_EXPORT void webrtc_VideoEncoder_QpThresholds_set_high(
    struct webrtc_VideoEncoder_QpThresholds* self,
    int value) {
  auto thresholds = reinterpret_cast<webrtc::VideoEncoder::QpThresholds*>(self);
  thresholds->high = value;
}

WEBRTC_EXPORT struct webrtc_VideoEncoder_ScalingSettings*
webrtc_VideoEncoder_ScalingSettings_new() {
  auto settings = new webrtc::VideoEncoder::ScalingSettings(
      webrtc::VideoEncoder::ScalingSettings::kOff);
  return reinterpret_cast<struct webrtc_VideoEncoder_ScalingSettings*>(
      settings);
}

WEBRTC_EXPORT void webrtc_VideoEncoder_ScalingSettings_delete(
    struct webrtc_VideoEncoder_ScalingSettings* self) {
  auto settings =
      reinterpret_cast<webrtc::VideoEncoder::ScalingSettings*>(self);
  delete settings;
}

WEBRTC_EXPORT void webrtc_VideoEncoder_ScalingSettings_get_thresholds(
    struct webrtc_VideoEncoder_ScalingSettings* self,
    int* out_has,
    struct webrtc_VideoEncoder_QpThresholds* out_value) {
  auto settings =
      reinterpret_cast<webrtc::VideoEncoder::ScalingSettings*>(self);
  auto value = reinterpret_cast<webrtc::VideoEncoder::QpThresholds*>(out_value);
  webrtc_c::OptionalGet(settings->thresholds, out_has, value);
}

WEBRTC_EXPORT void webrtc_VideoEncoder_ScalingSettings_set_thresholds(
    struct webrtc_VideoEncoder_ScalingSettings* self,
    int has,
    const struct webrtc_VideoEncoder_QpThresholds* value) {
  auto settings =
      reinterpret_cast<webrtc::VideoEncoder::ScalingSettings*>(self);
  auto v = reinterpret_cast<const webrtc::VideoEncoder::QpThresholds*>(value);
  webrtc_c::OptionalSet(settings->thresholds, has, v);
}

WEBRTC_EXPORT int webrtc_VideoEncoder_ScalingSettings_get_min_pixels_per_frame(
    struct webrtc_VideoEncoder_ScalingSettings* self) {
  auto settings =
      reinterpret_cast<webrtc::VideoEncoder::ScalingSettings*>(self);
  return settings->min_pixels_per_frame;
}

WEBRTC_EXPORT void webrtc_VideoEncoder_ScalingSettings_set_min_pixels_per_frame(
    struct webrtc_VideoEncoder_ScalingSettings* self,
    int value) {
  auto settings =
      reinterpret_cast<webrtc::VideoEncoder::ScalingSettings*>(self);
  settings->min_pixels_per_frame = value;
}

WEBRTC_EXPORT struct webrtc_VideoEncoder_ResolutionBitrateLimits*
webrtc_VideoEncoder_ResolutionBitrateLimits_new(int frame_size_pixels,
                                                int min_start_bitrate_bps,
                                                int min_bitrate_bps,
                                                int max_bitrate_bps) {
  auto limits = new webrtc::VideoEncoder::ResolutionBitrateLimits(
      frame_size_pixels, min_start_bitrate_bps, min_bitrate_bps,
      max_bitrate_bps);
  return reinterpret_cast<struct webrtc_VideoEncoder_ResolutionBitrateLimits*>(
      limits);
}

WEBRTC_EXPORT void webrtc_VideoEncoder_ResolutionBitrateLimits_delete(
    struct webrtc_VideoEncoder_ResolutionBitrateLimits* self) {
  auto limits =
      reinterpret_cast<webrtc::VideoEncoder::ResolutionBitrateLimits*>(self);
  delete limits;
}

WEBRTC_EXPORT int
webrtc_VideoEncoder_ResolutionBitrateLimits_get_frame_size_pixels(
    struct webrtc_VideoEncoder_ResolutionBitrateLimits* self) {
  auto limits =
      reinterpret_cast<webrtc::VideoEncoder::ResolutionBitrateLimits*>(self);
  return limits->frame_size_pixels;
}

WEBRTC_EXPORT void
webrtc_VideoEncoder_ResolutionBitrateLimits_set_frame_size_pixels(
    struct webrtc_VideoEncoder_ResolutionBitrateLimits* self,
    int value) {
  auto limits =
      reinterpret_cast<webrtc::VideoEncoder::ResolutionBitrateLimits*>(self);
  limits->frame_size_pixels = value;
}

WEBRTC_EXPORT int
webrtc_VideoEncoder_ResolutionBitrateLimits_get_min_start_bitrate_bps(
    struct webrtc_VideoEncoder_ResolutionBitrateLimits* self) {
  auto limits =
      reinterpret_cast<webrtc::VideoEncoder::ResolutionBitrateLimits*>(self);
  return limits->min_start_bitrate_bps;
}

WEBRTC_EXPORT void
webrtc_VideoEncoder_ResolutionBitrateLimits_set_min_start_bitrate_bps(
    struct webrtc_VideoEncoder_ResolutionBitrateLimits* self,
    int value) {
  auto limits =
      reinterpret_cast<webrtc::VideoEncoder::ResolutionBitrateLimits*>(self);
  limits->min_start_bitrate_bps = value;
}

WEBRTC_EXPORT int
webrtc_VideoEncoder_ResolutionBitrateLimits_get_min_bitrate_bps(
    struct webrtc_VideoEncoder_ResolutionBitrateLimits* self) {
  auto limits =
      reinterpret_cast<webrtc::VideoEncoder::ResolutionBitrateLimits*>(self);
  return limits->min_bitrate_bps;
}

WEBRTC_EXPORT void
webrtc_VideoEncoder_ResolutionBitrateLimits_set_min_bitrate_bps(
    struct webrtc_VideoEncoder_ResolutionBitrateLimits* self,
    int value) {
  auto limits =
      reinterpret_cast<webrtc::VideoEncoder::ResolutionBitrateLimits*>(self);
  limits->min_bitrate_bps = value;
}

WEBRTC_EXPORT int
webrtc_VideoEncoder_ResolutionBitrateLimits_get_max_bitrate_bps(
    struct webrtc_VideoEncoder_ResolutionBitrateLimits* self) {
  auto limits =
      reinterpret_cast<webrtc::VideoEncoder::ResolutionBitrateLimits*>(self);
  return limits->max_bitrate_bps;
}

WEBRTC_EXPORT void
webrtc_VideoEncoder_ResolutionBitrateLimits_set_max_bitrate_bps(
    struct webrtc_VideoEncoder_ResolutionBitrateLimits* self,
    int value) {
  auto limits =
      reinterpret_cast<webrtc::VideoEncoder::ResolutionBitrateLimits*>(self);
  limits->max_bitrate_bps = value;
}

WEBRTC_EXPORT struct webrtc_VideoEncoder_Resolution*
webrtc_VideoEncoder_Resolution_new(int width, int height) {
  auto resolution = new webrtc::VideoEncoder::Resolution(width, height);
  return reinterpret_cast<struct webrtc_VideoEncoder_Resolution*>(resolution);
}

WEBRTC_EXPORT void webrtc_VideoEncoder_Resolution_delete(
    struct webrtc_VideoEncoder_Resolution* self) {
  auto resolution = reinterpret_cast<webrtc::VideoEncoder::Resolution*>(self);
  delete resolution;
}

WEBRTC_EXPORT int webrtc_VideoEncoder_Resolution_get_width(
    struct webrtc_VideoEncoder_Resolution* self) {
  auto resolution = reinterpret_cast<webrtc::VideoEncoder::Resolution*>(self);
  return resolution->width;
}

WEBRTC_EXPORT void webrtc_VideoEncoder_Resolution_set_width(
    struct webrtc_VideoEncoder_Resolution* self,
    int value) {
  auto resolution = reinterpret_cast<webrtc::VideoEncoder::Resolution*>(self);
  resolution->width = value;
}

WEBRTC_EXPORT int webrtc_VideoEncoder_Resolution_get_height(
    struct webrtc_VideoEncoder_Resolution* self) {
  auto resolution = reinterpret_cast<webrtc::VideoEncoder::Resolution*>(self);
  return resolution->height;
}

WEBRTC_EXPORT void webrtc_VideoEncoder_Resolution_set_height(
    struct webrtc_VideoEncoder_Resolution* self,
    int value) {
  auto resolution = reinterpret_cast<webrtc::VideoEncoder::Resolution*>(self);
  resolution->height = value;
}

WEBRTC_EXPORT struct webrtc_VideoEncoder_EncoderInfo_unique*
webrtc_VideoEncoder_EncoderInfo_new() {
  auto info = std::make_unique<webrtc::VideoEncoder::EncoderInfo>();
  return reinterpret_cast<struct webrtc_VideoEncoder_EncoderInfo_unique*>(
      info.release());
}

WEBRTC_EXPORT struct std_string_unique*
webrtc_VideoEncoder_EncoderInfo_get_implementation_name(
    struct webrtc_VideoEncoder_EncoderInfo* self) {
  auto info = reinterpret_cast<webrtc::VideoEncoder::EncoderInfo*>(self);
  auto out = std::make_unique<std::string>(info->implementation_name);
  return reinterpret_cast<struct std_string_unique*>(out.release());
}

WEBRTC_EXPORT void webrtc_VideoEncoder_EncoderInfo_set_implementation_name(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    struct std_string_unique* name) {
  auto info = reinterpret_cast<webrtc::VideoEncoder::EncoderInfo*>(self);
  if (name == nullptr) {
    info->implementation_name.clear();
    return;
  }
  auto cpp_name = reinterpret_cast<std::string*>(std_string_unique_get(name));
  if (cpp_name != nullptr) {
    info->implementation_name = *cpp_name;
  } else {
    info->implementation_name.clear();
  }
  std_string_unique_delete(name);
}

WEBRTC_EXPORT int webrtc_VideoEncoder_EncoderInfo_get_is_hardware_accelerated(
    struct webrtc_VideoEncoder_EncoderInfo* self) {
  auto info = reinterpret_cast<webrtc::VideoEncoder::EncoderInfo*>(self);
  return info->is_hardware_accelerated ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_VideoEncoder_EncoderInfo_set_is_hardware_accelerated(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    int value) {
  auto info = reinterpret_cast<webrtc::VideoEncoder::EncoderInfo*>(self);
  info->is_hardware_accelerated = value != 0;
}

WEBRTC_EXPORT struct webrtc_VideoEncoder_ScalingSettings*
webrtc_VideoEncoder_EncoderInfo_get_scaling_settings(
    struct webrtc_VideoEncoder_EncoderInfo* self) {
  auto info = reinterpret_cast<webrtc::VideoEncoder::EncoderInfo*>(self);
  return reinterpret_cast<struct webrtc_VideoEncoder_ScalingSettings*>(
      &info->scaling_settings);
}

WEBRTC_EXPORT void webrtc_VideoEncoder_EncoderInfo_set_scaling_settings(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    const struct webrtc_VideoEncoder_ScalingSettings* value) {
  auto info = reinterpret_cast<webrtc::VideoEncoder::EncoderInfo*>(self);
  assert(value != nullptr);
  if (value == nullptr) {
    return;
  }
  auto settings =
      reinterpret_cast<const webrtc::VideoEncoder::ScalingSettings*>(value);
  info->scaling_settings = *settings;
}

WEBRTC_EXPORT uint32_t
webrtc_VideoEncoder_EncoderInfo_get_requested_resolution_alignment(
    struct webrtc_VideoEncoder_EncoderInfo* self) {
  auto info = reinterpret_cast<webrtc::VideoEncoder::EncoderInfo*>(self);
  return info->requested_resolution_alignment;
}

WEBRTC_EXPORT void
webrtc_VideoEncoder_EncoderInfo_set_requested_resolution_alignment(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    uint32_t value) {
  auto info = reinterpret_cast<webrtc::VideoEncoder::EncoderInfo*>(self);
  info->requested_resolution_alignment = value;
}

WEBRTC_EXPORT int
webrtc_VideoEncoder_EncoderInfo_get_apply_alignment_to_all_simulcast_layers(
    struct webrtc_VideoEncoder_EncoderInfo* self) {
  auto info = reinterpret_cast<webrtc::VideoEncoder::EncoderInfo*>(self);
  return info->apply_alignment_to_all_simulcast_layers ? 1 : 0;
}

WEBRTC_EXPORT void
webrtc_VideoEncoder_EncoderInfo_set_apply_alignment_to_all_simulcast_layers(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    int value) {
  auto info = reinterpret_cast<webrtc::VideoEncoder::EncoderInfo*>(self);
  info->apply_alignment_to_all_simulcast_layers = value != 0;
}

WEBRTC_EXPORT int webrtc_VideoEncoder_EncoderInfo_get_supports_native_handle(
    struct webrtc_VideoEncoder_EncoderInfo* self) {
  auto info = reinterpret_cast<webrtc::VideoEncoder::EncoderInfo*>(self);
  return info->supports_native_handle ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_VideoEncoder_EncoderInfo_set_supports_native_handle(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    int value) {
  auto info = reinterpret_cast<webrtc::VideoEncoder::EncoderInfo*>(self);
  info->supports_native_handle = value != 0;
}

WEBRTC_EXPORT int
webrtc_VideoEncoder_EncoderInfo_get_has_trusted_rate_controller(
    struct webrtc_VideoEncoder_EncoderInfo* self) {
  auto info = reinterpret_cast<webrtc::VideoEncoder::EncoderInfo*>(self);
  return info->has_trusted_rate_controller ? 1 : 0;
}

WEBRTC_EXPORT void
webrtc_VideoEncoder_EncoderInfo_set_has_trusted_rate_controller(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    int value) {
  auto info = reinterpret_cast<webrtc::VideoEncoder::EncoderInfo*>(self);
  info->has_trusted_rate_controller = value != 0;
}

WEBRTC_EXPORT struct webrtc_VideoEncoder_FramerateFraction_inlined_vector*
webrtc_VideoEncoder_EncoderInfo_get_fps_allocation(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    int spatial_index) {
  auto info = reinterpret_cast<webrtc::VideoEncoder::EncoderInfo*>(self);
  if (spatial_index < 0 || spatial_index >= webrtc::kMaxSpatialLayers) {
    return nullptr;
  }
  return reinterpret_cast<
      struct webrtc_VideoEncoder_FramerateFraction_inlined_vector*>(
      &info->fps_allocation[spatial_index]);
}

WEBRTC_EXPORT struct webrtc_VideoEncoder_ResolutionBitrateLimits_vector*
webrtc_VideoEncoder_EncoderInfo_get_resolution_bitrate_limits(
    struct webrtc_VideoEncoder_EncoderInfo* self) {
  auto info = reinterpret_cast<webrtc::VideoEncoder::EncoderInfo*>(self);
  return reinterpret_cast<
      struct webrtc_VideoEncoder_ResolutionBitrateLimits_vector*>(
      &info->resolution_bitrate_limits);
}

WEBRTC_EXPORT int webrtc_VideoEncoder_EncoderInfo_get_supports_simulcast(
    struct webrtc_VideoEncoder_EncoderInfo* self) {
  auto info = reinterpret_cast<webrtc::VideoEncoder::EncoderInfo*>(self);
  return info->supports_simulcast ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_VideoEncoder_EncoderInfo_set_supports_simulcast(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    int value) {
  auto info = reinterpret_cast<webrtc::VideoEncoder::EncoderInfo*>(self);
  info->supports_simulcast = value != 0;
}

WEBRTC_EXPORT struct webrtc_VideoFrameBuffer_Type_inlined_vector*
webrtc_VideoEncoder_EncoderInfo_get_preferred_pixel_formats(
    struct webrtc_VideoEncoder_EncoderInfo* self) {
  auto info = reinterpret_cast<webrtc::VideoEncoder::EncoderInfo*>(self);
  return reinterpret_cast<struct webrtc_VideoFrameBuffer_Type_inlined_vector*>(
      &info->preferred_pixel_formats);
}

WEBRTC_EXPORT void webrtc_VideoEncoder_EncoderInfo_get_is_qp_trusted(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    int* out_has,
    int* out_value) {
  auto info = reinterpret_cast<webrtc::VideoEncoder::EncoderInfo*>(self);
  webrtc_c::OptionalGetAs(info->is_qp_trusted, out_has, out_value, [&]() {
    return info->is_qp_trusted.value() ? 1 : 0;
  });
}

WEBRTC_EXPORT void webrtc_VideoEncoder_EncoderInfo_set_is_qp_trusted(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    int has,
    const int* value) {
  auto info = reinterpret_cast<webrtc::VideoEncoder::EncoderInfo*>(self);
  webrtc_c::OptionalSetAs(info->is_qp_trusted, has, value,
                          [&]() { return *value != 0; });
}

WEBRTC_EXPORT void webrtc_VideoEncoder_EncoderInfo_get_min_qp(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    int* out_has,
    int* out_value) {
  auto info = reinterpret_cast<webrtc::VideoEncoder::EncoderInfo*>(self);
  webrtc_c::OptionalGet(info->min_qp, out_has, out_value);
}

WEBRTC_EXPORT void webrtc_VideoEncoder_EncoderInfo_set_min_qp(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    int has,
    const int* value) {
  auto info = reinterpret_cast<webrtc::VideoEncoder::EncoderInfo*>(self);
  webrtc_c::OptionalSet(info->min_qp, has, value);
}

WEBRTC_EXPORT void webrtc_VideoEncoder_EncoderInfo_get_mapped_resolution(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    int* out_has,
    struct webrtc_VideoEncoder_Resolution* out_value) {
  auto info = reinterpret_cast<webrtc::VideoEncoder::EncoderInfo*>(self);
  auto value = reinterpret_cast<webrtc::VideoEncoder::Resolution*>(out_value);
  webrtc_c::OptionalGet(info->mapped_resolution, out_has, value);
}

WEBRTC_EXPORT void webrtc_VideoEncoder_EncoderInfo_set_mapped_resolution(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    int has,
    const struct webrtc_VideoEncoder_Resolution* value) {
  auto info = reinterpret_cast<webrtc::VideoEncoder::EncoderInfo*>(self);
  auto v = reinterpret_cast<const webrtc::VideoEncoder::Resolution*>(value);
  webrtc_c::OptionalSet(info->mapped_resolution, has, v);
}

WEBRTC_EXPORT struct std_string_unique*
webrtc_VideoEncoder_EncoderInfo_ToString(
    struct webrtc_VideoEncoder_EncoderInfo* self) {
  auto info = reinterpret_cast<webrtc::VideoEncoder::EncoderInfo*>(self);
  auto out = std::make_unique<std::string>(info->ToString());
  return reinterpret_cast<struct std_string_unique*>(out.release());
}

WEBRTC_EXPORT void
webrtc_VideoEncoder_EncoderInfo_GetEncoderBitrateLimitsForResolution(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    int frame_size_pixels,
    int* out_has,
    struct webrtc_VideoEncoder_ResolutionBitrateLimits* out_value) {
  auto info = reinterpret_cast<webrtc::VideoEncoder::EncoderInfo*>(self);
  auto limits = info->GetEncoderBitrateLimitsForResolution(frame_size_pixels);
  auto value = reinterpret_cast<webrtc::VideoEncoder::ResolutionBitrateLimits*>(
      out_value);
  webrtc_c::OptionalGet(limits, out_has, value);
}

WEBRTC_EXPORT struct webrtc_VideoEncoder_EncodedImageCallback_Result_unique*
webrtc_VideoEncoder_EncodedImageCallback_Result_new(int error) {
  auto result = std::make_unique<webrtc::EncodedImageCallback::Result>(
      static_cast<webrtc::EncodedImageCallback::Result::Error>(error));
  return reinterpret_cast<
      struct webrtc_VideoEncoder_EncodedImageCallback_Result_unique*>(
      result.release());
}

WEBRTC_EXPORT struct webrtc_VideoEncoder_EncodedImageCallback_Result_unique*
webrtc_VideoEncoder_EncodedImageCallback_Result_new_with_frame_id(
    int error,
    uint32_t frame_id) {
  auto result = std::make_unique<webrtc::EncodedImageCallback::Result>(
      static_cast<webrtc::EncodedImageCallback::Result::Error>(error),
      frame_id);
  return reinterpret_cast<
      struct webrtc_VideoEncoder_EncodedImageCallback_Result_unique*>(
      result.release());
}

WEBRTC_EXPORT int webrtc_VideoEncoder_EncodedImageCallback_Result_error(
    struct webrtc_VideoEncoder_EncodedImageCallback_Result* self) {
  auto result = reinterpret_cast<webrtc::EncodedImageCallback::Result*>(self);
  return static_cast<int>(result->error);
}

WEBRTC_EXPORT void webrtc_VideoEncoder_EncodedImageCallback_Result_set_error(
    struct webrtc_VideoEncoder_EncodedImageCallback_Result* self,
    int error) {
  auto result = reinterpret_cast<webrtc::EncodedImageCallback::Result*>(self);
  result->error =
      static_cast<webrtc::EncodedImageCallback::Result::Error>(error);
}

WEBRTC_EXPORT uint32_t webrtc_VideoEncoder_EncodedImageCallback_Result_frame_id(
    struct webrtc_VideoEncoder_EncodedImageCallback_Result* self) {
  auto result = reinterpret_cast<webrtc::EncodedImageCallback::Result*>(self);
  return result->frame_id;
}

WEBRTC_EXPORT void webrtc_VideoEncoder_EncodedImageCallback_Result_set_frame_id(
    struct webrtc_VideoEncoder_EncodedImageCallback_Result* self,
    uint32_t frame_id) {
  auto result = reinterpret_cast<webrtc::EncodedImageCallback::Result*>(self);
  result->frame_id = frame_id;
}

WEBRTC_EXPORT int
webrtc_VideoEncoder_EncodedImageCallback_Result_drop_next_frame(
    struct webrtc_VideoEncoder_EncodedImageCallback_Result* self) {
  auto result = reinterpret_cast<webrtc::EncodedImageCallback::Result*>(self);
  return result->drop_next_frame ? 1 : 0;
}

WEBRTC_EXPORT void
webrtc_VideoEncoder_EncodedImageCallback_Result_set_drop_next_frame(
    struct webrtc_VideoEncoder_EncodedImageCallback_Result* self,
    int drop_next_frame) {
  auto result = reinterpret_cast<webrtc::EncodedImageCallback::Result*>(self);
  result->drop_next_frame = drop_next_frame != 0;
}

WEBRTC_EXPORT struct webrtc_VideoEncoder_EncodedImageCallback*
webrtc_VideoEncoder_EncodedImageCallback_new(
    const struct webrtc_VideoEncoder_EncodedImageCallback_cbs* cbs,
    void* user_data) {
  auto callback = new EncodedImageCallbackImpl(cbs, user_data);
  return reinterpret_cast<struct webrtc_VideoEncoder_EncodedImageCallback*>(
      callback);
}

WEBRTC_EXPORT void webrtc_VideoEncoder_EncodedImageCallback_delete(
    struct webrtc_VideoEncoder_EncodedImageCallback* self) {
  auto callback = reinterpret_cast<EncodedImageCallbackImpl*>(self);
  delete callback;
}

WEBRTC_EXPORT struct webrtc_VideoEncoder_EncodedImageCallback_Result_unique*
webrtc_VideoEncoder_EncodedImageCallback_OnEncodedImage(
    struct webrtc_VideoEncoder_EncodedImageCallback* self,
    struct webrtc_EncodedImage* encoded_image,
    struct webrtc_CodecSpecificInfo* codec_specific_info) {
  if (self == nullptr) {
    return nullptr;
  }
  assert(encoded_image != nullptr);
  auto callback = reinterpret_cast<webrtc::EncodedImageCallback*>(self);
  auto image = reinterpret_cast<webrtc::EncodedImage*>(encoded_image);
  auto codec_info =
      reinterpret_cast<webrtc::CodecSpecificInfo*>(codec_specific_info);
  auto result = callback->OnEncodedImage(*image, codec_info);
  auto out = std::make_unique<webrtc::EncodedImageCallback::Result>(result);
  return reinterpret_cast<
      struct webrtc_VideoEncoder_EncodedImageCallback_Result_unique*>(
      out.release());
}

WEBRTC_EXPORT struct webrtc_VideoEncoder_unique* webrtc_VideoEncoder_new(
    const struct webrtc_VideoEncoder_cbs* cbs,
    void* user_data) {
  auto encoder = new VideoEncoderImpl(cbs, user_data);
  return reinterpret_cast<struct webrtc_VideoEncoder_unique*>(encoder);
}

WEBRTC_EXPORT int32_t
webrtc_VideoEncoder_InitEncode(struct webrtc_VideoEncoder* self,
                               struct webrtc_VideoCodec* codec_settings,
                               struct webrtc_VideoEncoder_Settings* settings) {
  if (self == nullptr) {
    return -1;
  }
  auto encoder = reinterpret_cast<webrtc::VideoEncoder*>(self);

  webrtc::VideoCodec codec_storage;
  webrtc::VideoEncoder::Capabilities capabilities(/*loss_notification=*/false);
  webrtc::VideoEncoder::Settings settings_storage(capabilities,
                                                  /*number_of_cores=*/1,
                                                  /*max_payload_size=*/1200);

  auto codec = codec_settings != nullptr
                   ? reinterpret_cast<webrtc::VideoCodec*>(codec_settings)
                   : &codec_storage;
  auto encoder_settings =
      settings != nullptr
          ? reinterpret_cast<webrtc::VideoEncoder::Settings*>(settings)
          : &settings_storage;
  return encoder->InitEncode(codec, *encoder_settings);
}

WEBRTC_EXPORT int32_t
webrtc_VideoEncoder_Encode(struct webrtc_VideoEncoder* self,
                           struct webrtc_VideoFrame* frame,
                           struct webrtc_VideoFrameType_vector* frame_types) {
  if (self == nullptr || frame == nullptr) {
    return -1;
  }
  auto encoder = reinterpret_cast<webrtc::VideoEncoder*>(self);
  auto input_frame = reinterpret_cast<webrtc::VideoFrame*>(frame);
  auto types =
      reinterpret_cast<std::vector<webrtc::VideoFrameType>*>(frame_types);
  return encoder->Encode(*input_frame, types);
}

WEBRTC_EXPORT int32_t webrtc_VideoEncoder_RegisterEncodeCompleteCallback(
    struct webrtc_VideoEncoder* self,
    struct webrtc_VideoEncoder_EncodedImageCallback* callback) {
  if (self == nullptr) {
    return -1;
  }
  auto encoder = reinterpret_cast<webrtc::VideoEncoder*>(self);
  auto encoded_image_callback =
      reinterpret_cast<webrtc::EncodedImageCallback*>(callback);
  return encoder->RegisterEncodeCompleteCallback(encoded_image_callback);
}

WEBRTC_EXPORT int32_t
webrtc_VideoEncoder_Release(struct webrtc_VideoEncoder* self) {
  if (self == nullptr) {
    return -1;
  }
  auto encoder = reinterpret_cast<webrtc::VideoEncoder*>(self);
  return encoder->Release();
}

WEBRTC_EXPORT void webrtc_VideoEncoder_SetRates(
    struct webrtc_VideoEncoder* self,
    struct webrtc_VideoEncoder_RateControlParameters* parameters) {
  if (self == nullptr) {
    return;
  }
  auto encoder = reinterpret_cast<webrtc::VideoEncoder*>(self);

  webrtc::VideoBitrateAllocation target_bitrate;
  target_bitrate.SetBitrate(0, 0, 300000);
  webrtc::VideoBitrateAllocation bitrate;
  bitrate.SetBitrate(0, 0, 250000);
  webrtc::VideoEncoder::RateControlParameters parameters_storage(
      bitrate, /*framerate_fps=*/30.0, webrtc::DataRate::BitsPerSec(350000));
  parameters_storage.target_bitrate = target_bitrate;

  auto rates =
      parameters != nullptr
          ? reinterpret_cast<webrtc::VideoEncoder::RateControlParameters*>(
                parameters)
          : &parameters_storage;
  encoder->SetRates(*rates);
}

WEBRTC_EXPORT struct webrtc_VideoEncoder_EncoderInfo_unique*
webrtc_VideoEncoder_GetEncoderInfo(struct webrtc_VideoEncoder* self) {
  if (self == nullptr) {
    return nullptr;
  }
  auto encoder = reinterpret_cast<webrtc::VideoEncoder*>(self);
  auto info = std::make_unique<webrtc::VideoEncoder::EncoderInfo>(
      encoder->GetEncoderInfo());
  return reinterpret_cast<struct webrtc_VideoEncoder_EncoderInfo_unique*>(
      info.release());
}
}
