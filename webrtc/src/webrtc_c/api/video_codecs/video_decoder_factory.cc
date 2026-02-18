#include "video_decoder_factory.h"

#include <stdarg.h>
#include <stddef.h>
#include <memory>
#include <vector>

// WebRTC
#include <api/environment/environment.h>
#include <api/video_codecs/builtin_video_decoder_factory.h>
#include <api/video_codecs/video_decoder_factory.h>

#include "../../common.impl.h"
#include "/home/melpon/dev/sora-rust-sdk-private/crates/webrtc-rs/webrtc/src/webrtc_c/api/environment.h"
#include "api/video_codecs/sdp_video_format.h"
#include "api/video_codecs/video_decoder.h"
#include "sdp_video_format.h"
#include "video_decoder.h"

namespace {

class RustVideoDecoderFactory : public webrtc::VideoDecoderFactory {
 public:
  RustVideoDecoderFactory(const webrtc_VideoDecoderFactory_cbs* cbs,
                          void* user_data)
      : user_data_(user_data) {
    if (cbs != nullptr) {
      cbs_ = *cbs;
    }
  }

  ~RustVideoDecoderFactory() override {
    if (cbs_.OnDestroy != nullptr) {
      cbs_.OnDestroy(user_data_);
    }
  }

  std::vector<webrtc::SdpVideoFormat> GetSupportedFormats() const override {
    if (cbs_.GetSupportedFormats == nullptr) {
      return {};
    }
    auto raw_formats = cbs_.GetSupportedFormats(user_data_);
    if (raw_formats == nullptr) {
      return {};
    }
    auto formats =
        reinterpret_cast<std::vector<webrtc::SdpVideoFormat>*>(raw_formats);
    auto copied = *formats;
    webrtc_SdpVideoFormat_vector_delete(raw_formats);
    return copied;
  }

  std::unique_ptr<webrtc::VideoDecoder> Create(
      const webrtc::Environment& env,
      const webrtc::SdpVideoFormat& format) override {
    if (cbs_.Create == nullptr) {
      return nullptr;
    }
    auto raw_decoder =
        cbs_.Create(reinterpret_cast<struct webrtc_Environment*>(
                        const_cast<webrtc::Environment*>(&env)),
                    reinterpret_cast<struct webrtc_SdpVideoFormat*>(
                        const_cast<webrtc::SdpVideoFormat*>(&format)),
                    user_data_);
    if (raw_decoder == nullptr) {
      return nullptr;
    }
    auto decoder =
        reinterpret_cast<webrtc::VideoDecoder*>(webrtc_VideoDecoder_unique_get(
            reinterpret_cast<struct webrtc_VideoDecoder_unique*>(raw_decoder)));
    return std::unique_ptr<webrtc::VideoDecoder>(decoder);
  }

 private:
  webrtc_VideoDecoderFactory_cbs cbs_{};
  void* user_data_ = nullptr;
};

}  // namespace

// -------------------------
// webrtc::VideoDecoderFactory
// -------------------------

extern "C" {
WEBRTC_DEFINE_UNIQUE(webrtc_VideoDecoderFactory, webrtc::VideoDecoderFactory);
struct webrtc_VideoDecoderFactory_unique* webrtc_VideoDecoderFactory_new(
    const struct webrtc_VideoDecoderFactory_cbs* cbs,
    void* user_data) {
  auto factory = new RustVideoDecoderFactory(cbs, user_data);
  return reinterpret_cast<struct webrtc_VideoDecoderFactory_unique*>(factory);
}

struct webrtc_VideoDecoder_unique* webrtc_VideoDecoderFactory_Create(
    struct webrtc_VideoDecoderFactory* self,
    struct webrtc_Environment* env,
    struct webrtc_SdpVideoFormat* format) {
  if (self == nullptr || env == nullptr || format == nullptr) {
    return nullptr;
  }
  auto factory = reinterpret_cast<webrtc::VideoDecoderFactory*>(self);
  auto cpp_env = reinterpret_cast<webrtc::Environment*>(env);
  auto cpp_format = reinterpret_cast<webrtc::SdpVideoFormat*>(format);
  auto decoder = factory->Create(*cpp_env, *cpp_format);
  return reinterpret_cast<struct webrtc_VideoDecoder_unique*>(
      decoder.release());
}

struct webrtc_VideoDecoderFactory_unique*
webrtc_CreateBuiltinVideoDecoderFactory() {
  auto factory = webrtc::CreateBuiltinVideoDecoderFactory();
  return reinterpret_cast<struct webrtc_VideoDecoderFactory_unique*>(
      factory.release());
}
}
