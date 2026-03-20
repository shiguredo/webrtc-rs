#pragma once

#include <stddef.h>

#include "../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::SSLIdentity
// -------------------------

WEBRTC_DECLARE_UNIQUE(webrtc_SSLIdentity);

// PEM 形式の秘密鍵と証明書から SSLIdentity を生成する
WEBRTC_EXPORT struct webrtc_SSLIdentity_unique*
webrtc_SSLIdentity_CreateFromPEMStrings(const char* private_key,
                                        size_t private_key_len,
                                        const char* certificate,
                                        size_t certificate_len);

// PEM 形式の秘密鍵と証明書チェーンから SSLIdentity を生成する
WEBRTC_EXPORT struct webrtc_SSLIdentity_unique*
webrtc_SSLIdentity_CreateFromPEMChainStrings(const char* private_key,
                                             size_t private_key_len,
                                             const char* certificate_chain,
                                             size_t certificate_chain_len);

#if defined(__cplusplus)
}
#endif
