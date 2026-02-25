#include "video_encoder.h"

#include <stddef.h>
#include <stdint.h>
#include <cassert>
#include <memory>
#include <string>
#include <vector>

// WebRTC
#include <api/units/data_rate.h>
#include <api/video/encoded_image.h>
#include <api/video/video_bitrate_allocation.h>
#include <api/video/video_frame.h>
#include <api/video/video_frame_type.h>
#include <api/video_codecs/video_codec.h>
#include <api/video_codecs/video_encoder.h>
#include <modules/video_coding/include/video_codec_interface.h>
#include <modules/video_coding/include/video_error_codes.h>

#include "../../common.impl.h"
#include "../../std.h"

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

struct webrtc_VideoEncoder_EncoderInfo_unique*
webrtc_VideoEncoder_EncoderInfo_new() {
  auto info = std::make_unique<webrtc::VideoEncoder::EncoderInfo>();
  return reinterpret_cast<struct webrtc_VideoEncoder_EncoderInfo_unique*>(
      info.release());
}

struct std_string_unique*
webrtc_VideoEncoder_EncoderInfo_get_implementation_name(
    struct webrtc_VideoEncoder_EncoderInfo* self) {
  auto info = reinterpret_cast<webrtc::VideoEncoder::EncoderInfo*>(self);
  auto out = std::make_unique<std::string>(info->implementation_name);
  return reinterpret_cast<struct std_string_unique*>(out.release());
}

void webrtc_VideoEncoder_EncoderInfo_set_implementation_name(
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

int webrtc_VideoEncoder_EncoderInfo_get_is_hardware_accelerated(
    struct webrtc_VideoEncoder_EncoderInfo* self) {
  auto info = reinterpret_cast<webrtc::VideoEncoder::EncoderInfo*>(self);
  return info->is_hardware_accelerated ? 1 : 0;
}

void webrtc_VideoEncoder_EncoderInfo_set_is_hardware_accelerated(
    struct webrtc_VideoEncoder_EncoderInfo* self,
    int value) {
  auto info = reinterpret_cast<webrtc::VideoEncoder::EncoderInfo*>(self);
  info->is_hardware_accelerated = value != 0;
}

struct webrtc_VideoEncoder_EncodedImageCallback_Result_unique*
webrtc_VideoEncoder_EncodedImageCallback_Result_new(int error) {
  auto result = std::make_unique<webrtc::EncodedImageCallback::Result>(
      static_cast<webrtc::EncodedImageCallback::Result::Error>(error));
  return reinterpret_cast<
      struct webrtc_VideoEncoder_EncodedImageCallback_Result_unique*>(
      result.release());
}

struct webrtc_VideoEncoder_EncodedImageCallback_Result_unique*
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

int webrtc_VideoEncoder_EncodedImageCallback_Result_error(
    struct webrtc_VideoEncoder_EncodedImageCallback_Result* self) {
  auto result = reinterpret_cast<webrtc::EncodedImageCallback::Result*>(self);
  return static_cast<int>(result->error);
}

void webrtc_VideoEncoder_EncodedImageCallback_Result_set_error(
    struct webrtc_VideoEncoder_EncodedImageCallback_Result* self,
    int error) {
  auto result = reinterpret_cast<webrtc::EncodedImageCallback::Result*>(self);
  result->error =
      static_cast<webrtc::EncodedImageCallback::Result::Error>(error);
}

uint32_t webrtc_VideoEncoder_EncodedImageCallback_Result_frame_id(
    struct webrtc_VideoEncoder_EncodedImageCallback_Result* self) {
  auto result = reinterpret_cast<webrtc::EncodedImageCallback::Result*>(self);
  return result->frame_id;
}

void webrtc_VideoEncoder_EncodedImageCallback_Result_set_frame_id(
    struct webrtc_VideoEncoder_EncodedImageCallback_Result* self,
    uint32_t frame_id) {
  auto result = reinterpret_cast<webrtc::EncodedImageCallback::Result*>(self);
  result->frame_id = frame_id;
}

int webrtc_VideoEncoder_EncodedImageCallback_Result_drop_next_frame(
    struct webrtc_VideoEncoder_EncodedImageCallback_Result* self) {
  auto result = reinterpret_cast<webrtc::EncodedImageCallback::Result*>(self);
  return result->drop_next_frame ? 1 : 0;
}

void webrtc_VideoEncoder_EncodedImageCallback_Result_set_drop_next_frame(
    struct webrtc_VideoEncoder_EncodedImageCallback_Result* self,
    int drop_next_frame) {
  auto result = reinterpret_cast<webrtc::EncodedImageCallback::Result*>(self);
  result->drop_next_frame = drop_next_frame != 0;
}

struct webrtc_VideoEncoder_EncodedImageCallback*
webrtc_VideoEncoder_EncodedImageCallback_new(
    const struct webrtc_VideoEncoder_EncodedImageCallback_cbs* cbs,
    void* user_data) {
  auto callback = new EncodedImageCallbackImpl(cbs, user_data);
  return reinterpret_cast<struct webrtc_VideoEncoder_EncodedImageCallback*>(
      callback);
}

void webrtc_VideoEncoder_EncodedImageCallback_delete(
    struct webrtc_VideoEncoder_EncodedImageCallback* self) {
  auto callback = reinterpret_cast<EncodedImageCallbackImpl*>(self);
  delete callback;
}

struct webrtc_VideoEncoder_EncodedImageCallback_Result_unique*
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

struct webrtc_VideoEncoder_unique* webrtc_VideoEncoder_new(
    const struct webrtc_VideoEncoder_cbs* cbs,
    void* user_data) {
  auto encoder = new VideoEncoderImpl(cbs, user_data);
  return reinterpret_cast<struct webrtc_VideoEncoder_unique*>(encoder);
}

int32_t webrtc_VideoEncoder_InitEncode(
    struct webrtc_VideoEncoder* self,
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

int32_t webrtc_VideoEncoder_Encode(
    struct webrtc_VideoEncoder* self,
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

int32_t webrtc_VideoEncoder_RegisterEncodeCompleteCallback(
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

void webrtc_VideoEncoder_SetRates(
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

struct webrtc_VideoEncoder_EncoderInfo_unique*
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
