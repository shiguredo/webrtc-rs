#include "video_encoder_factory.h"

#include <stdarg.h>
#include <stddef.h>
#include <memory>
#include <vector>

// WebRTC
#include <api/environment/environment.h>
#include <api/video_codecs/builtin_video_encoder_factory.h>
#include <api/video_codecs/video_encoder_factory.h>

#include "../../common.impl.h"
#include "/home/melpon/dev/sora-rust-sdk-private/crates/webrtc-rs/webrtc/src/webrtc_c/api/environment.h"
#include "api/video_codecs/sdp_video_format.h"
#include "api/video_codecs/video_encoder.h"
#include "sdp_video_format.h"
#include "video_encoder.h"

namespace {

class RustVideoEncoderFactory : public webrtc::VideoEncoderFactory {
 public:
  RustVideoEncoderFactory(const webrtc_VideoEncoderFactory_cbs* cbs,
                          void* user_data)
      : user_data_(user_data) {
    if (cbs != nullptr) {
      cbs_ = *cbs;
    }
  }

  ~RustVideoEncoderFactory() override {
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

  std::unique_ptr<webrtc::VideoEncoder> Create(
      const webrtc::Environment& env,
      const webrtc::SdpVideoFormat& format) override {
    if (cbs_.Create == nullptr) {
      return nullptr;
    }
    auto raw_encoder =
        cbs_.Create(reinterpret_cast<struct webrtc_Environment*>(
                        const_cast<webrtc::Environment*>(&env)),
                    reinterpret_cast<struct webrtc_SdpVideoFormat*>(
                        const_cast<webrtc::SdpVideoFormat*>(&format)),
                    user_data_);
    if (raw_encoder == nullptr) {
      return nullptr;
    }
    auto encoder =
        reinterpret_cast<webrtc::VideoEncoder*>(webrtc_VideoEncoder_unique_get(
            reinterpret_cast<struct webrtc_VideoEncoder_unique*>(raw_encoder)));
    return std::unique_ptr<webrtc::VideoEncoder>(encoder);
  }

 private:
  webrtc_VideoEncoderFactory_cbs cbs_{};
  void* user_data_ = nullptr;
};

}  // namespace

// -------------------------
// webrtc::VideoEncoderFactory
// -------------------------

extern "C" {
WEBRTC_DEFINE_UNIQUE(webrtc_VideoEncoderFactory, webrtc::VideoEncoderFactory);
struct webrtc_VideoEncoderFactory_unique* webrtc_VideoEncoderFactory_new(
    const struct webrtc_VideoEncoderFactory_cbs* cbs,
    void* user_data) {
  auto factory = new RustVideoEncoderFactory(cbs, user_data);
  return reinterpret_cast<struct webrtc_VideoEncoderFactory_unique*>(factory);
}

struct webrtc_VideoEncoder_unique* webrtc_VideoEncoderFactory_Create(
    struct webrtc_VideoEncoderFactory* self,
    struct webrtc_Environment* env,
    struct webrtc_SdpVideoFormat* format) {
  if (self == nullptr || env == nullptr || format == nullptr) {
    return nullptr;
  }
  auto factory = reinterpret_cast<webrtc::VideoEncoderFactory*>(self);
  auto cpp_env = reinterpret_cast<webrtc::Environment*>(env);
  auto cpp_format = reinterpret_cast<webrtc::SdpVideoFormat*>(format);
  auto encoder = factory->Create(*cpp_env, *cpp_format);
  return reinterpret_cast<struct webrtc_VideoEncoder_unique*>(
      encoder.release());
}

struct webrtc_VideoEncoderFactory_unique*
webrtc_CreateBuiltinVideoEncoderFactory() {
  auto factory = webrtc::CreateBuiltinVideoEncoderFactory();
  return reinterpret_cast<struct webrtc_VideoEncoderFactory_unique*>(
      factory.release());
}
}
