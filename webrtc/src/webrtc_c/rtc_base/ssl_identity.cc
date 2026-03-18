#include "ssl_identity.h"

#include <stddef.h>
#include <memory>
#include <string>

// WebRTC
#include <rtc_base/ssl_identity.h>

#include "../common.impl.h"

extern "C" {
WEBRTC_DEFINE_UNIQUE(webrtc_SSLIdentity, webrtc::SSLIdentity);

struct webrtc_SSLIdentity_unique* webrtc_SSLIdentity_CreateFromPEMStrings(
    const char* private_key,
    size_t private_key_len,
    const char* certificate,
    size_t certificate_len) {
  std::string pk(private_key, private_key_len);
  std::string cert(certificate, certificate_len);
  auto identity = webrtc::SSLIdentity::CreateFromPEMStrings(pk, cert);
  if (!identity) {
    return nullptr;
  }
  return reinterpret_cast<struct webrtc_SSLIdentity_unique*>(
      identity.release());
}

struct webrtc_SSLIdentity_unique* webrtc_SSLIdentity_CreateFromPEMChainStrings(
    const char* private_key,
    size_t private_key_len,
    const char* certificate_chain,
    size_t certificate_chain_len) {
  std::string pk(private_key, private_key_len);
  std::string chain(certificate_chain, certificate_chain_len);
  auto identity = webrtc::SSLIdentity::CreateFromPEMChainStrings(pk, chain);
  if (!identity) {
    return nullptr;
  }
  return reinterpret_cast<struct webrtc_SSLIdentity_unique*>(
      identity.release());
}
}
