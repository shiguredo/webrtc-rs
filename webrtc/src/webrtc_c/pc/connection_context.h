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

struct webrtc_NetworkManager* WEBRTC_EXPORT
webrtc_ConnectionContext_default_network_manager(
    struct webrtc_ConnectionContext* self);
struct webrtc_PacketSocketFactory* WEBRTC_EXPORT
webrtc_ConnectionContext_default_socket_factory(
    struct webrtc_ConnectionContext* self);

#if defined(__cplusplus)
}
#endif
