#include "video_decoder.h"

#include <stddef.h>
#include <stdint.h>
#include <memory>
#include <string>

// WebRTC
#include <api/video/encoded_image.h>
#include <api/video_codecs/video_decoder.h>

#include "../../common.impl.h"
#include "../../std.h"

namespace {

constexpr int32_t kVideoCodecOk = 0;

class VideoDecoderImpl : public webrtc::VideoDecoder {
 public:
  VideoDecoderImpl(const webrtc_VideoDecoder_cbs* cbs, void* user_data)
      : user_data_(user_data) {
    if (cbs != nullptr) {
      cbs_ = *cbs;
    }
  }

  ~VideoDecoderImpl() override {
    if (cbs_.OnDestroy != nullptr) {
      cbs_.OnDestroy(user_data_);
    }
  }

  bool Configure(const webrtc::VideoDecoder::Settings& settings) override {
    if (cbs_.Configure != nullptr) {
      return cbs_.Configure(
                 reinterpret_cast<struct webrtc_VideoDecoder_Settings*>(
                     const_cast<webrtc::VideoDecoder::Settings*>(&settings)),
                 user_data_) != 0;
    }
    return true;
  }

  int32_t Decode(const webrtc::EncodedImage& input_image,
                 int64_t render_time_ms) override {
    if (cbs_.Decode != nullptr) {
      return cbs_.Decode(reinterpret_cast<struct webrtc_EncodedImage*>(
                             const_cast<webrtc::EncodedImage*>(&input_image)),
                         render_time_ms, user_data_);
    }
    return kVideoCodecOk;
  }

  int32_t RegisterDecodeCompleteCallback(
      webrtc::DecodedImageCallback* callback) override {
    if (cbs_.RegisterDecodeCompleteCallback != nullptr) {
      return cbs_.RegisterDecodeCompleteCallback(
          reinterpret_cast<struct webrtc_VideoDecoder_DecodedImageCallback*>(
              callback),
          user_data_);
    }
    return kVideoCodecOk;
  }

  int32_t Release() override {
    if (cbs_.Release != nullptr) {
      return cbs_.Release(user_data_);
    }
    return kVideoCodecOk;
  }

  webrtc::VideoDecoder::DecoderInfo GetDecoderInfo() const override {
    webrtc::VideoDecoder::DecoderInfo info;
    if (cbs_.GetDecoderInfo != nullptr) {
      auto raw_info = cbs_.GetDecoderInfo(user_data_);
      if (raw_info != nullptr) {
        auto raw =
            reinterpret_cast<struct webrtc_VideoDecoder_DecoderInfo_unique*>(
                raw_info);
        auto c_info = reinterpret_cast<webrtc::VideoDecoder::DecoderInfo*>(
            webrtc_VideoDecoder_DecoderInfo_unique_get(raw));
        if (c_info != nullptr) {
          info = *c_info;
        }
        webrtc_VideoDecoder_DecoderInfo_unique_delete(raw);
      }
    }
    return info;
  }

 private:
  webrtc_VideoDecoder_cbs cbs_{};
  void* user_data_ = nullptr;
};

}  // namespace

extern "C" {
WEBRTC_DEFINE_UNIQUE(webrtc_VideoDecoder, webrtc::VideoDecoder);
WEBRTC_DEFINE_UNIQUE(webrtc_VideoDecoder_DecoderInfo,
                     webrtc::VideoDecoder::DecoderInfo);

struct webrtc_VideoDecoder_DecoderInfo_unique*
webrtc_VideoDecoder_DecoderInfo_new() {
  auto info = std::make_unique<webrtc::VideoDecoder::DecoderInfo>();
  return reinterpret_cast<struct webrtc_VideoDecoder_DecoderInfo_unique*>(
      info.release());
}

struct std_string_unique*
webrtc_VideoDecoder_DecoderInfo_get_implementation_name(
    struct webrtc_VideoDecoder_DecoderInfo* self) {
  auto info = reinterpret_cast<webrtc::VideoDecoder::DecoderInfo*>(self);
  auto out = std::make_unique<std::string>(info->implementation_name);
  return reinterpret_cast<struct std_string_unique*>(out.release());
}

void webrtc_VideoDecoder_DecoderInfo_set_implementation_name(
    struct webrtc_VideoDecoder_DecoderInfo* self,
    struct std_string_unique* name) {
  auto info = reinterpret_cast<webrtc::VideoDecoder::DecoderInfo*>(self);
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

int webrtc_VideoDecoder_DecoderInfo_get_is_hardware_accelerated(
    struct webrtc_VideoDecoder_DecoderInfo* self) {
  auto info = reinterpret_cast<webrtc::VideoDecoder::DecoderInfo*>(self);
  return info->is_hardware_accelerated ? 1 : 0;
}

void webrtc_VideoDecoder_DecoderInfo_set_is_hardware_accelerated(
    struct webrtc_VideoDecoder_DecoderInfo* self,
    int value) {
  auto info = reinterpret_cast<webrtc::VideoDecoder::DecoderInfo*>(self);
  info->is_hardware_accelerated = value != 0;
}

int webrtc_VideoDecoder_Settings_number_of_cores(
    struct webrtc_VideoDecoder_Settings* self) {
  auto settings = reinterpret_cast<webrtc::VideoDecoder::Settings*>(self);
  return settings->number_of_cores();
}

int webrtc_VideoDecoder_Settings_codec_type(
    struct webrtc_VideoDecoder_Settings* self) {
  auto settings = reinterpret_cast<webrtc::VideoDecoder::Settings*>(self);
  return static_cast<int>(settings->codec_type());
}

int webrtc_VideoDecoder_Settings_has_buffer_pool_size(
    struct webrtc_VideoDecoder_Settings* self) {
  auto settings = reinterpret_cast<webrtc::VideoDecoder::Settings*>(self);
  return settings->buffer_pool_size().has_value() ? 1 : 0;
}

int webrtc_VideoDecoder_Settings_buffer_pool_size(
    struct webrtc_VideoDecoder_Settings* self) {
  auto settings = reinterpret_cast<webrtc::VideoDecoder::Settings*>(self);
  return settings->buffer_pool_size().value_or(0);
}

int webrtc_VideoDecoder_Settings_max_render_resolution_width(
    struct webrtc_VideoDecoder_Settings* self) {
  auto settings = reinterpret_cast<webrtc::VideoDecoder::Settings*>(self);
  return settings->max_render_resolution().Width();
}

int webrtc_VideoDecoder_Settings_max_render_resolution_height(
    struct webrtc_VideoDecoder_Settings* self) {
  auto settings = reinterpret_cast<webrtc::VideoDecoder::Settings*>(self);
  return settings->max_render_resolution().Height();
}

struct webrtc_VideoDecoder_unique* webrtc_VideoDecoder_new(
    const struct webrtc_VideoDecoder_cbs* cbs,
    void* user_data) {
  auto decoder = new VideoDecoderImpl(cbs, user_data);
  return reinterpret_cast<struct webrtc_VideoDecoder_unique*>(decoder);
}

int webrtc_VideoDecoder_Configure(
    struct webrtc_VideoDecoder* self,
    struct webrtc_VideoDecoder_Settings* settings) {
  if (self == nullptr) {
    return 0;
  }
  auto decoder = reinterpret_cast<webrtc::VideoDecoder*>(self);
  webrtc::VideoDecoder::Settings settings_storage;
  auto decoder_settings =
      settings != nullptr
          ? reinterpret_cast<webrtc::VideoDecoder::Settings*>(settings)
          : &settings_storage;
  return decoder->Configure(*decoder_settings) ? 1 : 0;
}

int32_t webrtc_VideoDecoder_Decode(struct webrtc_VideoDecoder* self,
                                   struct webrtc_EncodedImage* input_image,
                                   int64_t render_time_ms) {
  if (self == nullptr) {
    return -1;
  }
  auto decoder = reinterpret_cast<webrtc::VideoDecoder*>(self);
  if (input_image != nullptr) {
    auto cpp_input_image = reinterpret_cast<webrtc::EncodedImage*>(input_image);
    return decoder->Decode(*cpp_input_image, render_time_ms);
  }
  webrtc::EncodedImage default_input_image;
  return decoder->Decode(default_input_image, render_time_ms);
}

struct webrtc_VideoDecoder_DecoderInfo_unique*
webrtc_VideoDecoder_GetDecoderInfo(struct webrtc_VideoDecoder* self) {
  if (self == nullptr) {
    return nullptr;
  }
  auto decoder = reinterpret_cast<webrtc::VideoDecoder*>(self);
  auto info = std::make_unique<webrtc::VideoDecoder::DecoderInfo>(
      decoder->GetDecoderInfo());
  return reinterpret_cast<struct webrtc_VideoDecoder_DecoderInfo_unique*>(
      info.release());
}
}
