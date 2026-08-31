#include "audio_decoder_factory.h"

#include <stdarg.h>
#include <stddef.h>
#include <cassert>
#include <memory>
#include <vector>

// WebRTC
#include <api/audio_codecs/audio_decoder_factory.h>
#include <api/audio_codecs/builtin_audio_decoder_factory.h>
#include <api/environment/environment.h>
#include <api/make_ref_counted.h>
#include <api/scoped_refptr.h>

#include "../../common.h"
#include "../../common.impl.h"
#include "../environment.h"
#include "audio_decoder.h"
#include "audio_format.h"

namespace {

class AudioDecoderFactoryImpl : public webrtc::AudioDecoderFactory {
 public:
  AudioDecoderFactoryImpl(const webrtc_AudioDecoderFactory_cbs* cbs,
                          void* user_data)
      : user_data_(user_data) {
    assert(cbs != nullptr);
    assert(cbs->GetSupportedDecoders != nullptr);
    assert(cbs->IsSupportedDecoder != nullptr);
    assert(cbs->Create != nullptr);
    assert(cbs->OnDestroy != nullptr);
    cbs_ = *cbs;
  }

  ~AudioDecoderFactoryImpl() override { cbs_.OnDestroy(user_data_); }

  std::vector<webrtc::AudioCodecSpec> GetSupportedDecoders() override {
    auto raw = cbs_.GetSupportedDecoders(user_data_);
    assert(raw != nullptr);
    auto vec = reinterpret_cast<std::vector<webrtc::AudioCodecSpec>*>(raw);
    auto copied = *vec;
    webrtc_AudioCodecSpec_vector_delete(raw);
    return copied;
  }

  bool IsSupportedDecoder(const webrtc::SdpAudioFormat& format) override {
    return cbs_.IsSupportedDecoder(
               reinterpret_cast<const struct webrtc_SdpAudioFormat*>(&format),
               user_data_) != 0;
  }

  std::unique_ptr<webrtc::AudioDecoder> Create(
      const webrtc::Environment& env,
      const webrtc::SdpAudioFormat& format) override {
    auto raw = cbs_.Create(
        reinterpret_cast<const struct webrtc_Environment*>(&env),
        reinterpret_cast<const struct webrtc_SdpAudioFormat*>(&format),
        user_data_);
    if (raw == nullptr) {
      return nullptr;
    }
    auto decoder = reinterpret_cast<webrtc::AudioDecoder*>(
        webrtc_AudioDecoder_unique_get(raw));
    return std::unique_ptr<webrtc::AudioDecoder>(decoder);
  }

 private:
  webrtc_AudioDecoderFactory_cbs cbs_{};
  void* user_data_ = nullptr;
};

}  // namespace

extern "C" {

// -------------------------
// webrtc::AudioDecoderFactory
// -------------------------

WEBRTC_DEFINE_REFCOUNTED(webrtc_AudioDecoderFactory,
                         webrtc::AudioDecoderFactory);

WEBRTC_EXPORT struct webrtc_AudioDecoderFactory_refcounted*
webrtc_AudioDecoderFactory_make_ref_counted(
    const struct webrtc_AudioDecoderFactory_cbs* cbs,
    void* user_data) {
  auto impl = webrtc::make_ref_counted<AudioDecoderFactoryImpl>(cbs, user_data);
  return reinterpret_cast<struct webrtc_AudioDecoderFactory_refcounted*>(
      impl.release());
}

WEBRTC_EXPORT struct webrtc_AudioCodecSpec_vector*
webrtc_AudioDecoderFactory_GetSupportedDecoders(
    struct webrtc_AudioDecoderFactory* self) {
  assert(self != nullptr);
  auto factory = reinterpret_cast<webrtc::AudioDecoderFactory*>(self);
  auto specs = factory->GetSupportedDecoders();
  auto vec = new std::vector<webrtc::AudioCodecSpec>(specs);
  return reinterpret_cast<struct webrtc_AudioCodecSpec_vector*>(vec);
}

WEBRTC_EXPORT int webrtc_AudioDecoderFactory_IsSupportedDecoder(
    struct webrtc_AudioDecoderFactory* self,
    const struct webrtc_SdpAudioFormat* format) {
  assert(self != nullptr);
  assert(format != nullptr);
  auto factory = reinterpret_cast<webrtc::AudioDecoderFactory*>(self);
  auto cpp_format = reinterpret_cast<const webrtc::SdpAudioFormat*>(format);
  return factory->IsSupportedDecoder(*cpp_format) ? 1 : 0;
}

WEBRTC_EXPORT struct webrtc_AudioDecoder_unique*
webrtc_AudioDecoderFactory_MakeAudioDecoder(
    struct webrtc_AudioDecoderFactory* self,
    const struct webrtc_Environment* env,
    const struct webrtc_SdpAudioFormat* format) {
  assert(self != nullptr);
  assert(env != nullptr);
  assert(format != nullptr);
  auto factory = reinterpret_cast<webrtc::AudioDecoderFactory*>(self);
  auto cpp_env = reinterpret_cast<const webrtc::Environment*>(env);
  auto cpp_format = reinterpret_cast<const webrtc::SdpAudioFormat*>(format);
  auto decoder = factory->Create(*cpp_env, *cpp_format);
  return reinterpret_cast<struct webrtc_AudioDecoder_unique*>(
      decoder.release());
}

WEBRTC_EXPORT struct webrtc_AudioDecoderFactory_refcounted*
webrtc_CreateBuiltinAudioDecoderFactory() {
  auto factory = webrtc::CreateBuiltinAudioDecoderFactory();
  return reinterpret_cast<struct webrtc_AudioDecoderFactory_refcounted*>(
      factory.release());
}
}
