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
extern const int webrtc_DtlsTransportState_kNew;
extern const int webrtc_DtlsTransportState_kConnecting;
extern const int webrtc_DtlsTransportState_kConnected;
extern const int webrtc_DtlsTransportState_kClosed;
extern const int webrtc_DtlsTransportState_kFailed;

webrtc_DtlsTransportState webrtc_DtlsTransportInterface_state(
    struct webrtc_DtlsTransportInterface* self);

#if defined(__cplusplus)
}
#endif
