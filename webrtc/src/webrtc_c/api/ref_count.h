#pragma once

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::RefCountInterface
// -------------------------

struct webrtc_RefCountInterface_ref;
void webrtc_RefCountInterface_AddRef(struct webrtc_RefCountInterface_ref* ref);
void webrtc_RefCountInterface_Release(struct webrtc_RefCountInterface_ref* ref);
struct webrtc_RefCountInterface_ref* webrtc_RefCountInterface_Create(
    void (*dtor)(void*),
    void* user_data);

#if defined(__cplusplus)
}
#endif
