#include "set_local_description_observer_interface.h"

#include <stdarg.h>
#include <stddef.h>
#include <cstring>
#include <memory>
#include <utility>

// WebRTC
#include <api/make_ref_counted.h>
#include <api/rtc_error.h>
#include <api/scoped_refptr.h>
#include <api/set_local_description_observer_interface.h>

#include "../common.impl.h"
#include "rtc_error.h"

// -------------------------
// webrtc::SetLocalDescriptionObserverInterface
// -------------------------

class SetLocalDescriptionObserverInterfaceImpl
    : public webrtc::SetLocalDescriptionObserverInterface {
 public:
  SetLocalDescriptionObserverInterfaceImpl(
      const struct webrtc_SetLocalDescriptionObserverInterface_cbs* cbs,
      void* user_data)
      : user_data_(user_data) {
    if (cbs != nullptr) {
      cbs_ = *cbs;
    }
  }

  ~SetLocalDescriptionObserverInterfaceImpl() override {
    if (cbs_.OnDestroy != nullptr) {
      cbs_.OnDestroy(user_data_);
    }
  }

  void OnSetLocalDescriptionComplete(webrtc::RTCError error) override {
    if (cbs_.OnSetLocalDescriptionComplete == nullptr) {
      return;
    }
    auto rtc_error = std::make_unique<webrtc::RTCError>(std::move(error));
    cbs_.OnSetLocalDescriptionComplete(
        reinterpret_cast<struct webrtc_RTCError_unique*>(rtc_error.release()),
        user_data_);
  }

 private:
  webrtc_SetLocalDescriptionObserverInterface_cbs cbs_{};
  void* user_data_;
};

extern "C" {
WEBRTC_DEFINE_REFCOUNTED(webrtc_SetLocalDescriptionObserverInterface,
                         webrtc::SetLocalDescriptionObserverInterface);

struct webrtc_SetLocalDescriptionObserverInterface_refcounted*
webrtc_SetLocalDescriptionObserverInterface_make_ref_counted(
    const struct webrtc_SetLocalDescriptionObserverInterface_cbs* cbs,
    void* user_data) {
  auto impl =
      webrtc::make_ref_counted<SetLocalDescriptionObserverInterfaceImpl>(
          cbs, user_data);
  return reinterpret_cast<
      struct webrtc_SetLocalDescriptionObserverInterface_refcounted*>(
      impl.release());
}
}
