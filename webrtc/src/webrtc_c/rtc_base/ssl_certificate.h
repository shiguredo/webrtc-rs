#pragma once

#include <stdint.h>

#include "../common.h"
#include "../std.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::SSLCertificate
// -------------------------

struct webrtc_SSLCertificate;
WEBRTC_EXPORT struct std_string_unique* webrtc_SSLCertificate_ToPEMString(
    const struct webrtc_SSLCertificate* self);
WEBRTC_EXPORT struct std_string_unique* webrtc_SSLCertificate_ToDER(
    const struct webrtc_SSLCertificate* self);
WEBRTC_EXPORT int64_t webrtc_SSLCertificate_CertificateExpirationTime(
    const struct webrtc_SSLCertificate* self);

// -------------------------
// webrtc::SSLCertChain
// -------------------------

struct webrtc_SSLCertChain;
WEBRTC_EXPORT int webrtc_SSLCertChain_GetSize(
    const struct webrtc_SSLCertChain* self);
WEBRTC_EXPORT const struct webrtc_SSLCertificate* webrtc_SSLCertChain_Get(
    const struct webrtc_SSLCertChain* self,
    int index);

// -------------------------
// webrtc::SSLCertificateVerifier
// -------------------------

WEBRTC_DECLARE_UNIQUE(webrtc_SSLCertificateVerifier);
struct webrtc_SSLCertificateVerifier_cbs {
  int (*VerifyChain)(const struct webrtc_SSLCertChain* chain, void* user_data);
  void (*OnDestroy)(void* user_data);
};
WEBRTC_EXPORT struct webrtc_SSLCertificateVerifier_unique*
webrtc_SSLCertificateVerifier_new(
    const struct webrtc_SSLCertificateVerifier_cbs* cbs,
    void* user_data);

#if defined(__cplusplus)
}
#endif
