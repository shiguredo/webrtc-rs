#pragma once

#include "../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::RefCountInterface
// -------------------------

struct webrtc_RefCountInterface_ref;
void WEBRTC_EXPORT
webrtc_RefCountInterface_AddRef(struct webrtc_RefCountInterface_ref* ref);
void WEBRTC_EXPORT
webrtc_RefCountInterface_Release(struct webrtc_RefCountInterface_ref* ref);
struct webrtc_RefCountInterface_ref* WEBRTC_EXPORT
webrtc_RefCountInterface_Create(void (*dtor)(void*), void* user_data);

#if defined(__cplusplus)
}
#endif
