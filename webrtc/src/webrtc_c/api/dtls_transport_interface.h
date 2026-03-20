#pragma once

#include "../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::DtlsTransportInterface
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_DtlsTransportInterface);

typedef int webrtc_DtlsTransportState;
WEBRTC_EXPORT extern const int webrtc_DtlsTransportState_kNew;
WEBRTC_EXPORT extern const int webrtc_DtlsTransportState_kConnecting;
WEBRTC_EXPORT extern const int webrtc_DtlsTransportState_kConnected;
WEBRTC_EXPORT extern const int webrtc_DtlsTransportState_kClosed;
WEBRTC_EXPORT extern const int webrtc_DtlsTransportState_kFailed;

webrtc_DtlsTransportState WEBRTC_EXPORT
webrtc_DtlsTransportInterface_state(struct webrtc_DtlsTransportInterface* self);

#if defined(__cplusplus)
}
#endif
