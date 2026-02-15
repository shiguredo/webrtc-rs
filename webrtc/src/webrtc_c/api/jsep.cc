#include "jsep.h"

#include <cstddef>
#include <memory>
#include <string>
#include <utility>

// WebRTC
#include <api/jsep.h>
#include <api/make_ref_counted.h>
#include <api/rtc_error.h>
#include <api/scoped_refptr.h>

#include "../common.impl.h"
#include "../std.h"
#include "rtc_error.h"

// -------------------------
// webrtc::SdpType
// -------------------------

extern "C" {
const int webrtc_SdpType_kOffer = static_cast<int>(webrtc::SdpType::kOffer);
const int webrtc_SdpType_kPrAnswer =
    static_cast<int>(webrtc::SdpType::kPrAnswer);
const int webrtc_SdpType_kAnswer = static_cast<int>(webrtc::SdpType::kAnswer);
const int webrtc_SdpType_kRollback =
    static_cast<int>(webrtc::SdpType::kRollback);

int webrtc_SdpTypeFromString(const char* type, size_t type_len) {
  std::string type_str =
      type != nullptr ? std::string(type, type_len) : std::string();
  auto sdp_type = webrtc::SdpTypeFromString(type_str);
  return static_cast<int>(sdp_type.value_or(webrtc::SdpType::kOffer));
}
const char* webrtc_SdpTypeToString(int type) {
  return webrtc::SdpTypeToString(static_cast<webrtc::SdpType>(type));
}
}

// -------------------------
// webrtc::SessionDescriptionInterface
// -------------------------

extern "C" {
WEBRTC_DEFINE_UNIQUE(webrtc_SessionDescriptionInterface,
                     webrtc::SessionDescriptionInterface);

struct webrtc_SessionDescriptionInterface_unique*
webrtc_CreateSessionDescription(int sdp_type, const char* sdp, size_t sdp_len) {
  auto type = static_cast<webrtc::SdpType>(sdp_type);
  auto desc = webrtc::CreateSessionDescription(type, std::string(sdp, sdp_len));
  if (!desc) {
    return nullptr;
  }
  return reinterpret_cast<struct webrtc_SessionDescriptionInterface_unique*>(
      desc.release());
}
int webrtc_SessionDescriptionInterface_GetType(
    struct webrtc_SessionDescriptionInterface* self) {
  auto desc = reinterpret_cast<webrtc::SessionDescriptionInterface*>(self);
  return static_cast<int>(desc->GetType());
}
int webrtc_SessionDescriptionInterface_ToString(
    struct webrtc_SessionDescriptionInterface* self,
    struct std_string_unique** out_sdp) {
  auto desc = reinterpret_cast<webrtc::SessionDescriptionInterface*>(self);
  std::string sdp;
  if (!desc->ToString(&sdp)) {
    *out_sdp = nullptr;
    return 0;
  }
  auto out = std::make_unique<std::string>(sdp);
  *out_sdp = reinterpret_cast<struct std_string_unique*>(out.release());
  return 1;
}
}

extern "C" {
struct webrtc_IceCandidateInterface* webrtc_CreateIceCandidate(
    const char* sdp_mid,
    size_t sdp_mid_len,
    int sdp_mline_index,
    const char* sdp,
    size_t sdp_len) {
  webrtc::SdpParseError error;
  auto* ice_candidate = webrtc::CreateIceCandidate(
      std::string(sdp_mid, sdp_mid_len), sdp_mline_index,
      std::string(sdp, sdp_len), &error);
  if (!ice_candidate) {
    return nullptr;
  }
  return reinterpret_cast<struct webrtc_IceCandidateInterface*>(
      ice_candidate);
}
void webrtc_IceCandidateInterface_delete(
    struct webrtc_IceCandidateInterface* self) {
  auto candidate = reinterpret_cast<webrtc::IceCandidateInterface*>(self);
  delete candidate;
}
void webrtc_IceCandidateInterface_sdp_mid(
    const struct webrtc_IceCandidateInterface* self,
    struct std_string_unique** out) {
  auto candidate = reinterpret_cast<const webrtc::IceCandidateInterface*>(self);
  auto mid = std::make_unique<std::string>(candidate->sdp_mid());
  *out = reinterpret_cast<struct std_string_unique*>(mid.release());
}
int webrtc_IceCandidateInterface_sdp_mline_index(
    const struct webrtc_IceCandidateInterface* self) {
  auto candidate = reinterpret_cast<const webrtc::IceCandidateInterface*>(self);
  return candidate->sdp_mline_index();
}
int webrtc_IceCandidateInterface_ToString(
    const struct webrtc_IceCandidateInterface* self,
    struct std_string_unique** out_sdp) {
  auto candidate = reinterpret_cast<const webrtc::IceCandidateInterface*>(self);
  std::string sdp;
  if (!candidate->ToString(&sdp)) {
    *out_sdp = nullptr;
    return 0;
  }
  auto out = std::make_unique<std::string>(sdp);
  *out_sdp = reinterpret_cast<struct std_string_unique*>(out.release());
  return 1;
}
}

// -------------------------
// webrtc::CreateSessionDescriptionObserver
// -------------------------

class CreateSessionDescriptionObserverImpl
    : public webrtc::CreateSessionDescriptionObserver {
 public:
  CreateSessionDescriptionObserverImpl(
      struct webrtc_CreateSessionDescriptionObserver_cbs* cbs,
      void* user_data)
      : cbs_(cbs), user_data_(user_data) {}

  void OnSuccess(webrtc::SessionDescriptionInterface* desc) override {
    std::unique_ptr<webrtc::SessionDescriptionInterface> ptr(desc);
    cbs_->OnSuccess(
        reinterpret_cast<struct webrtc_SessionDescriptionInterface_unique*>(
            ptr.release()),
        user_data_);
  }
  void OnFailure(webrtc::RTCError error) override {
    auto rtc_error = std::make_unique<webrtc::RTCError>(std::move(error));
    cbs_->OnFailure(
        reinterpret_cast<struct webrtc_RTCError_unique*>(rtc_error.release()),
        user_data_);
  }

 private:
  struct webrtc_CreateSessionDescriptionObserver_cbs* cbs_;
  void* user_data_;
};

extern "C" {
WEBRTC_DEFINE_REFCOUNTED(webrtc_CreateSessionDescriptionObserver,
                         webrtc::CreateSessionDescriptionObserver);

struct webrtc_CreateSessionDescriptionObserver*
webrtc_CreateSessionDescriptionObserver_make_ref_counted(
    struct webrtc_CreateSessionDescriptionObserver_cbs* cbs,
    void* user_data) {
  auto impl = webrtc::make_ref_counted<CreateSessionDescriptionObserverImpl>(
      cbs, user_data);
  return reinterpret_cast<struct webrtc_CreateSessionDescriptionObserver*>(
      static_cast<webrtc::CreateSessionDescriptionObserver*>(impl.release()));
}
}
