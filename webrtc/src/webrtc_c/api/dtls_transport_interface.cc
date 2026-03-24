#include "dtls_transport_interface.h"

// WebRTC
#include <api/dtls_transport_interface.h>

#include "../common.h"
#include "../common.impl.h"
#include "api/rtc_error.h"

extern "C" {
WEBRTC_DEFINE_REFCOUNTED(webrtc_DtlsTransportInterface,
                         webrtc::DtlsTransportInterface);

WEBRTC_EXPORT extern const int webrtc_DtlsTransportState_kNew =
    static_cast<int>(webrtc::DtlsTransportState::kNew);
WEBRTC_EXPORT extern const int webrtc_DtlsTransportState_kConnecting =
    static_cast<int>(webrtc::DtlsTransportState::kConnecting);
WEBRTC_EXPORT extern const int webrtc_DtlsTransportState_kConnected =
    static_cast<int>(webrtc::DtlsTransportState::kConnected);
WEBRTC_EXPORT extern const int webrtc_DtlsTransportState_kClosed =
    static_cast<int>(webrtc::DtlsTransportState::kClosed);
WEBRTC_EXPORT extern const int webrtc_DtlsTransportState_kFailed =
    static_cast<int>(webrtc::DtlsTransportState::kFailed);

WEBRTC_EXPORT webrtc_DtlsTransportState webrtc_DtlsTransportInterface_state(
    struct webrtc_DtlsTransportInterface* self) {
  if (self == nullptr) {
    return webrtc_DtlsTransportState_kClosed;
  }
  auto transport = reinterpret_cast<webrtc::DtlsTransportInterface*>(self);
  return static_cast<webrtc_DtlsTransportState>(
      transport->Information().state());
}
}

// -------------------------
// webrtc::DtlsTransportObserverInterface
// -------------------------

class DtlsTransportObserverImpl
    : public webrtc::DtlsTransportObserverInterface {
 public:
  DtlsTransportObserverImpl(const struct webrtc_DtlsTransportObserver_cbs* cbs,
                            void* user_data)
      : user_data_(user_data) {
    if (cbs != nullptr) {
      cbs_ = *cbs;
    }
  }

  ~DtlsTransportObserverImpl() override {
    if (cbs_.OnDestroy != nullptr) {
      cbs_.OnDestroy(user_data_);
    }
  }

  void OnStateChange(webrtc::DtlsTransportInformation info) override {
    if (cbs_.OnStateChange != nullptr) {
      cbs_.OnStateChange(static_cast<webrtc_DtlsTransportState>(info.state()),
                         user_data_);
    }
  }

  void OnError(webrtc::RTCError error) override {
    if (cbs_.OnError != nullptr) {
      cbs_.OnError(user_data_);
    }
  }

 private:
  webrtc_DtlsTransportObserver_cbs cbs_{};
  void* user_data_;
};

extern "C" {
WEBRTC_EXPORT struct webrtc_DtlsTransportObserver*
webrtc_DtlsTransportObserver_new(
    const struct webrtc_DtlsTransportObserver_cbs* cbs,
    void* user_data) {
  auto impl = new DtlsTransportObserverImpl(cbs, user_data);
  return reinterpret_cast<struct webrtc_DtlsTransportObserver*>(impl);
}

WEBRTC_EXPORT void webrtc_DtlsTransportObserver_delete(
    struct webrtc_DtlsTransportObserver* self) {
  auto impl = reinterpret_cast<DtlsTransportObserverImpl*>(self);
  delete impl;
}

WEBRTC_EXPORT void webrtc_DtlsTransportInterface_RegisterObserver(
    struct webrtc_DtlsTransportInterface* self,
    struct webrtc_DtlsTransportObserver* observer) {
  auto transport = reinterpret_cast<webrtc::DtlsTransportInterface*>(self);
  auto obs = reinterpret_cast<DtlsTransportObserverImpl*>(observer);
  transport->RegisterObserver(obs);
}

WEBRTC_EXPORT void webrtc_DtlsTransportInterface_UnregisterObserver(
    struct webrtc_DtlsTransportInterface* self) {
  auto transport = reinterpret_cast<webrtc::DtlsTransportInterface*>(self);
  transport->UnregisterObserver();
}
}
