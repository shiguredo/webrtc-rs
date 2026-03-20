#include "connection_context.h"

// WebRTC
#include <pc/connection_context.h>

#include "../common.h"
#include "../common.impl.h"

extern "C" {
WEBRTC_DEFINE_REFCOUNTED(webrtc_ConnectionContext, webrtc::ConnectionContext);

struct webrtc_NetworkManager* WEBRTC_EXPORT
webrtc_ConnectionContext_default_network_manager(
    struct webrtc_ConnectionContext* self) {
  auto* context = reinterpret_cast<webrtc::ConnectionContext*>(self);
  auto* network_manager = context->signaling_thread()->BlockingCall(
      [context]() { return context->default_network_manager(); });
  return reinterpret_cast<struct webrtc_NetworkManager*>(network_manager);
}

struct webrtc_PacketSocketFactory* WEBRTC_EXPORT
webrtc_ConnectionContext_default_socket_factory(
    struct webrtc_ConnectionContext* self) {
  auto* context = reinterpret_cast<webrtc::ConnectionContext*>(self);
  auto* socket_factory = context->signaling_thread()->BlockingCall(
      [context]() { return context->default_socket_factory(); });
  return reinterpret_cast<struct webrtc_PacketSocketFactory*>(socket_factory);
}
}
