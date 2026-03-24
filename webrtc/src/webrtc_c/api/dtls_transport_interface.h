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

WEBRTC_EXPORT webrtc_DtlsTransportState
webrtc_DtlsTransportInterface_state(struct webrtc_DtlsTransportInterface* self);

// -------------------------
// webrtc::DtlsTransportObserverInterface
// -------------------------

struct webrtc_DtlsTransportObserver;
struct webrtc_DtlsTransportObserver_cbs {
  void (*OnStateChange)(webrtc_DtlsTransportState new_state, void* user_data);
  void (*OnError)(void* user_data);
  void (*OnDestroy)(void* user_data);
};
WEBRTC_EXPORT struct webrtc_DtlsTransportObserver*
webrtc_DtlsTransportObserver_new(
    const struct webrtc_DtlsTransportObserver_cbs* cbs,
    void* user_data);
WEBRTC_EXPORT void webrtc_DtlsTransportObserver_delete(
    struct webrtc_DtlsTransportObserver* self);

WEBRTC_EXPORT void webrtc_DtlsTransportInterface_RegisterObserver(
    struct webrtc_DtlsTransportInterface* self,
    struct webrtc_DtlsTransportObserver* observer);
WEBRTC_EXPORT void webrtc_DtlsTransportInterface_UnregisterObserver(
    struct webrtc_DtlsTransportInterface* self);

#if defined(__cplusplus)
}
#endif
