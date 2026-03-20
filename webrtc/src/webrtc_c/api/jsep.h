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
WEBRTC_EXPORT extern const int webrtc_SdpType_kOffer;
WEBRTC_EXPORT extern const int webrtc_SdpType_kPrAnswer;
WEBRTC_EXPORT extern const int webrtc_SdpType_kAnswer;
WEBRTC_EXPORT extern const int webrtc_SdpType_kRollback;
WEBRTC_EXPORT int webrtc_SdpTypeFromString(const char* type, size_t type_len);
WEBRTC_EXPORT const char* webrtc_SdpTypeToString(int type);

// -------------------------
// webrtc::SessionDescriptionInterface
// -------------------------

WEBRTC_DECLARE_UNIQUE(webrtc_SessionDescriptionInterface);
WEBRTC_EXPORT struct webrtc_SessionDescriptionInterface_unique*
webrtc_CreateSessionDescription(int sdp_type, const char* sdp, size_t sdp_len);
WEBRTC_EXPORT int webrtc_SessionDescriptionInterface_GetType(
    struct webrtc_SessionDescriptionInterface* self);
WEBRTC_EXPORT int webrtc_SessionDescriptionInterface_ToString(
    struct webrtc_SessionDescriptionInterface* self,
    struct std_string_unique** out_sdp);

// -------------------------
// webrtc::IceCandidate
// -------------------------

struct webrtc_IceCandidate;
WEBRTC_DECLARE_UNIQUE(webrtc_SdpParseError);
WEBRTC_EXPORT void webrtc_SdpParseError_line(struct webrtc_SdpParseError* self,
                                             const char** out_line,
                                             size_t* out_len);
WEBRTC_EXPORT void webrtc_SdpParseError_description(
    struct webrtc_SdpParseError* self,
    const char** out_description,
    size_t* out_len);
WEBRTC_EXPORT struct webrtc_IceCandidate* webrtc_CreateIceCandidate(
    const char* sdp_mid,
    size_t sdp_mid_len,
    int sdp_mline_index,
    const char* sdp,
    size_t sdp_len,
    struct webrtc_SdpParseError_unique** out_error);
WEBRTC_EXPORT void webrtc_IceCandidate_delete(struct webrtc_IceCandidate* self);
WEBRTC_EXPORT void webrtc_IceCandidate_sdp_mid(
    const struct webrtc_IceCandidate* self,
    struct std_string_unique** out);
WEBRTC_EXPORT int webrtc_IceCandidate_sdp_mline_index(
    const struct webrtc_IceCandidate* self);
WEBRTC_EXPORT int webrtc_IceCandidate_ToString(
    const struct webrtc_IceCandidate* self,
    struct std_string_unique** out_sdp);

// -------------------------
// webrtc::CreateSessionDescriptionObserver
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_CreateSessionDescriptionObserver);

struct webrtc_CreateSessionDescriptionObserver_cbs {
  void (*OnSuccess)(struct webrtc_SessionDescriptionInterface_unique* desc,
                    void* user_data);
  void (*OnFailure)(struct webrtc_RTCError_unique* error, void* user_data);
  void (*OnDestroy)(void* user_data);
};

WEBRTC_EXPORT struct webrtc_CreateSessionDescriptionObserver*
webrtc_CreateSessionDescriptionObserver_make_ref_counted(
    const struct webrtc_CreateSessionDescriptionObserver_cbs* cbs,
    void* user_data);

#if defined(__cplusplus)
}
#endif
