#pragma once

#include <stddef.h>

#include "../common.h"
#include "../std.h"
#include "rtc_error.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::SdpType
// -------------------------

typedef int webrtc_SdpType;
extern const int webrtc_SdpType_kOffer;
extern const int webrtc_SdpType_kPrAnswer;
extern const int webrtc_SdpType_kAnswer;
extern const int webrtc_SdpType_kRollback;
int webrtc_SdpTypeFromString(const char* type, size_t type_len);
const char* webrtc_SdpTypeToString(int type);

// -------------------------
// webrtc::SessionDescriptionInterface
// -------------------------

WEBRTC_DECLARE_UNIQUE(webrtc_SessionDescriptionInterface);
struct webrtc_SessionDescriptionInterface_unique*
webrtc_CreateSessionDescription(int sdp_type, const char* sdp, size_t sdp_len);
int webrtc_SessionDescriptionInterface_GetType(
    struct webrtc_SessionDescriptionInterface* self);
int webrtc_SessionDescriptionInterface_ToString(
    struct webrtc_SessionDescriptionInterface* self,
    struct std_string_unique** out_sdp);

// -------------------------
// webrtc::IceCandidateInterface
// -------------------------

struct webrtc_IceCandidateInterface;
int webrtc_IceCandidateInterface_ToString(
    const struct webrtc_IceCandidateInterface* self,
    struct std_string_unique** out_sdp);

// -------------------------
// webrtc::CreateSessionDescriptionObserver
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_CreateSessionDescriptionObserver);

struct webrtc_CreateSessionDescriptionObserver_cbs {
  void (*OnSuccess)(struct webrtc_SessionDescriptionInterface_unique* desc,
                    void* user_data);
  void (*OnFailure)(struct webrtc_RTCError_unique* error, void* user_data);
};

struct webrtc_CreateSessionDescriptionObserver*
webrtc_CreateSessionDescriptionObserver_make_ref_counted(
    struct webrtc_CreateSessionDescriptionObserver_cbs* cbs,
    void* user_data);

#if defined(__cplusplus)
}
#endif
