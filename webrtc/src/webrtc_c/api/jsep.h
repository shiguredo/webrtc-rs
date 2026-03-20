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
int WEBRTC_EXPORT webrtc_SdpTypeFromString(const char* type, size_t type_len);
const char* WEBRTC_EXPORT webrtc_SdpTypeToString(int type);

// -------------------------
// webrtc::SessionDescriptionInterface
// -------------------------

WEBRTC_DECLARE_UNIQUE(webrtc_SessionDescriptionInterface);
struct webrtc_SessionDescriptionInterface_unique* WEBRTC_EXPORT
webrtc_CreateSessionDescription(int sdp_type, const char* sdp, size_t sdp_len);
int WEBRTC_EXPORT webrtc_SessionDescriptionInterface_GetType(
    struct webrtc_SessionDescriptionInterface* self);
int WEBRTC_EXPORT webrtc_SessionDescriptionInterface_ToString(
    struct webrtc_SessionDescriptionInterface* self,
    struct std_string_unique** out_sdp);

// -------------------------
// webrtc::IceCandidate
// -------------------------

struct webrtc_IceCandidate;
WEBRTC_DECLARE_UNIQUE(webrtc_SdpParseError);
void WEBRTC_EXPORT webrtc_SdpParseError_line(struct webrtc_SdpParseError* self,
                                             const char** out_line,
                                             size_t* out_len);
void WEBRTC_EXPORT
webrtc_SdpParseError_description(struct webrtc_SdpParseError* self,
                                 const char** out_description,
                                 size_t* out_len);
struct webrtc_IceCandidate* WEBRTC_EXPORT
webrtc_CreateIceCandidate(const char* sdp_mid,
                          size_t sdp_mid_len,
                          int sdp_mline_index,
                          const char* sdp,
                          size_t sdp_len,
                          struct webrtc_SdpParseError_unique** out_error);
void WEBRTC_EXPORT webrtc_IceCandidate_delete(struct webrtc_IceCandidate* self);
void WEBRTC_EXPORT
webrtc_IceCandidate_sdp_mid(const struct webrtc_IceCandidate* self,
                            struct std_string_unique** out);
int WEBRTC_EXPORT
webrtc_IceCandidate_sdp_mline_index(const struct webrtc_IceCandidate* self);
int WEBRTC_EXPORT
webrtc_IceCandidate_ToString(const struct webrtc_IceCandidate* self,
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

struct webrtc_CreateSessionDescriptionObserver* WEBRTC_EXPORT
webrtc_CreateSessionDescriptionObserver_make_ref_counted(
    const struct webrtc_CreateSessionDescriptionObserver_cbs* cbs,
    void* user_data);

#if defined(__cplusplus)
}
#endif
