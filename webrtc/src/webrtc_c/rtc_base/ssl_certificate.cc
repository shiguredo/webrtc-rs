#include "ssl_certificate.h"

#include <stddef.h>
#include <memory>
#include <string>
#include <utility>
#include <vector>

// WebRTC
#include <rtc_base/ssl_certificate.h>

#include "../common.impl.h"

namespace {

class SSLCertificateVerifierImpl : public webrtc::SSLCertificateVerifier {
 public:
  SSLCertificateVerifierImpl(
      const struct webrtc_SSLCertificateVerifier_cbs* cbs,
      void* user_data)
      : user_data_(user_data) {
    if (cbs != nullptr) {
      cbs_ = *cbs;
    }
  }

  ~SSLCertificateVerifierImpl() override {
    if (cbs_.OnDestroy != nullptr) {
      cbs_.OnDestroy(user_data_);
    }
  }

  bool VerifyChain(const webrtc::SSLCertChain& chain) override {
    if (cbs_.VerifyChain == nullptr) {
      return false;
    }
    auto* c_chain = reinterpret_cast<const struct webrtc_SSLCertChain*>(&chain);
    return cbs_.VerifyChain(c_chain, user_data_) != 0;
  }

 private:
  webrtc_SSLCertificateVerifier_cbs cbs_{};
  void* user_data_ = nullptr;
};

}  // namespace

extern "C" {
WEBRTC_DEFINE_UNIQUE(webrtc_SSLCertificateVerifier,
                     webrtc::SSLCertificateVerifier);

struct std_string_unique* webrtc_SSLCertificate_ToPEMString(
    const struct webrtc_SSLCertificate* self) {
  auto cert = reinterpret_cast<const webrtc::SSLCertificate*>(self);
  auto pem = std::make_unique<std::string>(cert->ToPEMString());
  return reinterpret_cast<struct std_string_unique*>(pem.release());
}

struct std_string_unique* webrtc_SSLCertificate_ToDER(
    const struct webrtc_SSLCertificate* self) {
  auto cert = reinterpret_cast<const webrtc::SSLCertificate*>(self);
  webrtc::Buffer der_buffer;
  cert->ToDER(&der_buffer);
  auto der = std::make_unique<std::string>(
      reinterpret_cast<const char*>(der_buffer.data()), der_buffer.size());
  return reinterpret_cast<struct std_string_unique*>(der.release());
}

int64_t webrtc_SSLCertificate_CertificateExpirationTime(
    const struct webrtc_SSLCertificate* self) {
  auto cert = reinterpret_cast<const webrtc::SSLCertificate*>(self);
  return cert->CertificateExpirationTime();
}

int webrtc_SSLCertChain_GetSize(const struct webrtc_SSLCertChain* self) {
  auto chain = reinterpret_cast<const webrtc::SSLCertChain*>(self);
  return static_cast<int>(chain->GetSize());
}

const struct webrtc_SSLCertificate* webrtc_SSLCertChain_Get(
    const struct webrtc_SSLCertChain* self,
    int index) {
  auto chain = reinterpret_cast<const webrtc::SSLCertChain*>(self);
  if (index < 0 || static_cast<size_t>(index) >= chain->GetSize()) {
    return nullptr;
  }
  auto& cert = chain->Get(static_cast<size_t>(index));
  return reinterpret_cast<const struct webrtc_SSLCertificate*>(&cert);
}

struct webrtc_SSLCertificateVerifier_unique* webrtc_SSLCertificateVerifier_new(
    const struct webrtc_SSLCertificateVerifier_cbs* cbs,
    void* user_data) {
  auto verifier = std::make_unique<SSLCertificateVerifierImpl>(cbs, user_data);
  return reinterpret_cast<struct webrtc_SSLCertificateVerifier_unique*>(
      verifier.release());
}
}
