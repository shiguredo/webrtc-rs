#include "audio_encoder_factory.h"

#include <stdarg.h>
#include <stddef.h>
#include <cassert>
#include <cstdint>
#include <memory>
#include <optional>
#include <vector>

// WebRTC
#include <api/audio_codecs/audio_codec_pair_id.h>
#include <api/audio_codecs/audio_encoder_factory.h>
#include <api/audio_codecs/builtin_audio_encoder_factory.h>
#include <api/environment/environment.h>
#include <api/make_ref_counted.h>
#include <api/scoped_refptr.h>

#include "../../common.h"
#include "../../common.impl.h"
#include "../../std.impl.h"
#include "../environment.h"
#include "audio_encoder.h"
#include "audio_format.h"

namespace {

class AudioEncoderFactoryImpl : public webrtc::AudioEncoderFactory {
 public:
  AudioEncoderFactoryImpl(const webrtc_AudioEncoderFactory_cbs* cbs,
                          void* user_data)
      : user_data_(user_data) {
    assert(cbs != nullptr);
    assert(cbs->GetSupportedEncoders != nullptr);
    assert(cbs->QueryAudioEncoder != nullptr);
    assert(cbs->Create != nullptr);
    assert(cbs->OnDestroy != nullptr);
    cbs_ = *cbs;
  }

  ~AudioEncoderFactoryImpl() override { cbs_.OnDestroy(user_data_); }

  std::vector<webrtc::AudioCodecSpec> GetSupportedEncoders() override {
    auto raw = cbs_.GetSupportedEncoders(user_data_);
    assert(raw != nullptr);
    auto vec = reinterpret_cast<std::vector<webrtc::AudioCodecSpec>*>(raw);
    auto copied = *vec;
    webrtc_AudioCodecSpec_vector_delete(raw);
    return copied;
  }

  std::optional<webrtc::AudioCodecInfo> QueryAudioEncoder(
      const webrtc::SdpAudioFormat& format) override {
    auto raw = cbs_.QueryAudioEncoder(
        reinterpret_cast<const struct webrtc_SdpAudioFormat*>(&format),
        user_data_);
    if (raw == nullptr) {
      return std::nullopt;
    }
    auto cpp = reinterpret_cast<webrtc::AudioCodecInfo*>(raw);
    webrtc::AudioCodecInfo result = *cpp;
    webrtc_AudioCodecInfo_delete(raw);
    return result;
  }

  std::unique_ptr<webrtc::AudioEncoder> Create(
      const webrtc::Environment& env,
      const webrtc::SdpAudioFormat& format,
      Options options) override {
    auto raw = cbs_.Create(
        reinterpret_cast<const struct webrtc_Environment*>(&env),
        reinterpret_cast<const struct webrtc_SdpAudioFormat*>(&format),
        reinterpret_cast<struct webrtc_AudioEncoderFactory_Options*>(&options),
        user_data_);
    if (raw == nullptr) {
      return nullptr;
    }
    auto encoder = reinterpret_cast<webrtc::AudioEncoder*>(
        webrtc_AudioEncoder_unique_get(raw));
    return std::unique_ptr<webrtc::AudioEncoder>(encoder);
  }

 private:
  webrtc_AudioEncoderFactory_cbs cbs_{};
  void* user_data_ = nullptr;
};

}  // namespace

extern "C" {

// -------------------------
// webrtc::AudioEncoderFactory::Options
// -------------------------

WEBRTC_EXPORT struct webrtc_AudioEncoderFactory_Options*
webrtc_AudioEncoderFactory_Options_new() {
  auto options = new webrtc::AudioEncoderFactory::Options();
  return reinterpret_cast<struct webrtc_AudioEncoderFactory_Options*>(options);
}

WEBRTC_EXPORT void webrtc_AudioEncoderFactory_Options_delete(
    struct webrtc_AudioEncoderFactory_Options* self) {
  auto options = reinterpret_cast<webrtc::AudioEncoderFactory::Options*>(self);
  delete options;
}

WEBRTC_EXPORT int webrtc_AudioEncoderFactory_Options_get_payload_type(
    const struct webrtc_AudioEncoderFactory_Options* self) {
  auto options =
      reinterpret_cast<const webrtc::AudioEncoderFactory::Options*>(self);
  return options->payload_type;
}

WEBRTC_EXPORT void webrtc_AudioEncoderFactory_Options_set_payload_type(
    struct webrtc_AudioEncoderFactory_Options* self,
    int value) {
  auto options = reinterpret_cast<webrtc::AudioEncoderFactory::Options*>(self);
  options->payload_type = value;
}

WEBRTC_EXPORT struct webrtc_AudioCodecPairId*
webrtc_AudioEncoderFactory_Options_get_codec_pair_id(
    const struct webrtc_AudioEncoderFactory_Options* self) {
  auto options =
      reinterpret_cast<const webrtc::AudioEncoderFactory::Options*>(self);
  if (!options->codec_pair_id) {
    return nullptr;
  }
  // AudioCodecPairId はデフォルト構築不可のため、保持している値の所有コピーを返す。
  auto copied = new webrtc::AudioCodecPairId(*options->codec_pair_id);
  return reinterpret_cast<struct webrtc_AudioCodecPairId*>(copied);
}

WEBRTC_EXPORT void webrtc_AudioEncoderFactory_Options_set_codec_pair_id(
    struct webrtc_AudioEncoderFactory_Options* self,
    int has,
    const struct webrtc_AudioCodecPairId* value) {
  auto options = reinterpret_cast<webrtc::AudioEncoderFactory::Options*>(self);
  webrtc_c::OptionalSet(
      options->codec_pair_id, has,
      reinterpret_cast<const webrtc::AudioCodecPairId*>(value));
}

// -------------------------
// webrtc::AudioEncoderFactory
// -------------------------

WEBRTC_DEFINE_REFCOUNTED(webrtc_AudioEncoderFactory,
                         webrtc::AudioEncoderFactory);

WEBRTC_EXPORT struct webrtc_AudioEncoderFactory_refcounted*
webrtc_AudioEncoderFactory_make_ref_counted(
    const struct webrtc_AudioEncoderFactory_cbs* cbs,
    void* user_data) {
  auto impl = webrtc::make_ref_counted<AudioEncoderFactoryImpl>(cbs, user_data);
  return reinterpret_cast<struct webrtc_AudioEncoderFactory_refcounted*>(
      impl.release());
}

WEBRTC_EXPORT struct webrtc_AudioCodecSpec_vector*
webrtc_AudioEncoderFactory_GetSupportedEncoders(
    struct webrtc_AudioEncoderFactory* self) {
  assert(self != nullptr);
  auto factory = reinterpret_cast<webrtc::AudioEncoderFactory*>(self);
  auto specs = factory->GetSupportedEncoders();
  auto vec = new std::vector<webrtc::AudioCodecSpec>(specs);
  return reinterpret_cast<struct webrtc_AudioCodecSpec_vector*>(vec);
}

WEBRTC_EXPORT struct webrtc_AudioCodecInfo*
webrtc_AudioEncoderFactory_QueryAudioEncoder(
    struct webrtc_AudioEncoderFactory* self,
    const struct webrtc_SdpAudioFormat* format) {
  assert(self != nullptr);
  assert(format != nullptr);
  auto factory = reinterpret_cast<webrtc::AudioEncoderFactory*>(self);
  auto cpp_format = reinterpret_cast<const webrtc::SdpAudioFormat*>(format);
  auto info = factory->QueryAudioEncoder(*cpp_format);
  if (!info.has_value()) {
    return nullptr;
  }
  return reinterpret_cast<struct webrtc_AudioCodecInfo*>(
      new webrtc::AudioCodecInfo(*info));
}

WEBRTC_EXPORT struct webrtc_AudioEncoder_unique*
webrtc_AudioEncoderFactory_MakeAudioEncoder(
    struct webrtc_AudioEncoderFactory* self,
    const struct webrtc_Environment* env,
    const struct webrtc_SdpAudioFormat* format,
    struct webrtc_AudioEncoderFactory_Options* options) {
  assert(self != nullptr);
  assert(env != nullptr);
  assert(format != nullptr);
  auto factory = reinterpret_cast<webrtc::AudioEncoderFactory*>(self);
  auto cpp_env = reinterpret_cast<const webrtc::Environment*>(env);
  auto cpp_format = reinterpret_cast<const webrtc::SdpAudioFormat*>(format);
  auto cpp_options =
      reinterpret_cast<webrtc::AudioEncoderFactory::Options*>(options);
  auto encoder = factory->Create(*cpp_env, *cpp_format, *cpp_options);
  return reinterpret_cast<struct webrtc_AudioEncoder_unique*>(
      encoder.release());
}

WEBRTC_EXPORT struct webrtc_AudioEncoderFactory_refcounted*
webrtc_CreateBuiltinAudioEncoderFactory() {
  auto factory = webrtc::CreateBuiltinAudioEncoderFactory();
  return reinterpret_cast<struct webrtc_AudioEncoderFactory_refcounted*>(
      factory.release());
}
}
