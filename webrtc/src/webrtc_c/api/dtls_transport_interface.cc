#include "dtls_transport_interface.h"

// WebRTC
#include <api/dtls_transport_interface.h>

#include "../common.impl.h"

extern "C" {
WEBRTC_DEFINE_REFCOUNTED(webrtc_DtlsTransportInterface,
                         webrtc::DtlsTransportInterface);

extern const int webrtc_DtlsTransportState_kNew =
    static_cast<int>(webrtc::DtlsTransportState::kNew);
extern const int webrtc_DtlsTransportState_kConnecting =
    static_cast<int>(webrtc::DtlsTransportState::kConnecting);
extern const int webrtc_DtlsTransportState_kConnected =
    static_cast<int>(webrtc::DtlsTransportState::kConnected);
extern const int webrtc_DtlsTransportState_kClosed =
    static_cast<int>(webrtc::DtlsTransportState::kClosed);
extern const int webrtc_DtlsTransportState_kFailed =
    static_cast<int>(webrtc::DtlsTransportState::kFailed);

webrtc_DtlsTransportState webrtc_DtlsTransportInterface_state(
    struct webrtc_DtlsTransportInterface* self) {
  if (self == nullptr) {
    return webrtc_DtlsTransportState_kClosed;
  }
  auto transport = reinterpret_cast<webrtc::DtlsTransportInterface*>(self);
  return static_cast<webrtc_DtlsTransportState>(
      transport->Information().state());
}
}
