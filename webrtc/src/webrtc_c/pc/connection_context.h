#pragma once

#include "../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::ConnectionContext
// -------------------------

struct webrtc_NetworkManager;
struct webrtc_PacketSocketFactory;

WEBRTC_DECLARE_REFCOUNTED(webrtc_ConnectionContext);

WEBRTC_EXPORT struct webrtc_NetworkManager*
webrtc_ConnectionContext_default_network_manager(
    struct webrtc_ConnectionContext* self);
WEBRTC_EXPORT struct webrtc_PacketSocketFactory*
webrtc_ConnectionContext_default_socket_factory(
    struct webrtc_ConnectionContext* self);

#if defined(__cplusplus)
}
#endif
