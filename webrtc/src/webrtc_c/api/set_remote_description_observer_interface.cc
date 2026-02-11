#include "set_remote_description_observer_interface.h"

#include <stdarg.h>
#include <stddef.h>
#include <cstring>
#include <memory>
#include <utility>

// WebRTC
#include <api/make_ref_counted.h>
#include <api/rtc_error.h>
#include <api/scoped_refptr.h>
#include <api/set_remote_description_observer_interface.h>

#include "../common.impl.h"
#include "rtc_error.h"

// -------------------------
// webrtc::SetRemoteDescriptionObserverInterface
// -------------------------

class SetRemoteDescriptionObserverInterfaceImpl
    : public webrtc::SetRemoteDescriptionObserverInterface {
 public:
  SetRemoteDescriptionObserverInterfaceImpl(
      struct webrtc_SetRemoteDescriptionObserverInterface_cbs* cbs,
      void* user_data)
      : cbs_(cbs), user_data_(user_data) {}

  void OnSetRemoteDescriptionComplete(webrtc::RTCError error) override {
    auto rtc_error = std::make_unique<webrtc::RTCError>(std::move(error));
    cbs_->OnSetRemoteDescriptionComplete(
        reinterpret_cast<struct webrtc_RTCError_unique*>(rtc_error.release()),
        user_data_);
  }

 private:
  struct webrtc_SetRemoteDescriptionObserverInterface_cbs* cbs_;
  void* user_data_;
};

extern "C" {
WEBRTC_DEFINE_REFCOUNTED(webrtc_SetRemoteDescriptionObserverInterface,
                         webrtc::SetRemoteDescriptionObserverInterface);

struct webrtc_SetRemoteDescriptionObserverInterface_refcounted*
webrtc_SetRemoteDescriptionObserverInterface_make_ref_counted(
    struct webrtc_SetRemoteDescriptionObserverInterface_cbs* cbs,
    void* user_data) {
  auto impl =
      webrtc::make_ref_counted<SetRemoteDescriptionObserverInterfaceImpl>(
          cbs, user_data);
  return reinterpret_cast<
      struct webrtc_SetRemoteDescriptionObserverInterface_refcounted*>(
      impl.release());
}
}
