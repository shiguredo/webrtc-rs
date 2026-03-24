#include "ref_count.h"

#include <stdarg.h>
#include <stddef.h>

// WebRTC
#include <api/make_ref_counted.h>
#include <api/ref_count.h>
#include <api/scoped_refptr.h>

#include "../common.h"

// -------------------------
// webrtc::RefCountInterface
// -------------------------

struct webrtc_RefCountInterface_ref : webrtc::RefCountInterface {
  webrtc_RefCountInterface_ref(void (*dtor)(void*), void* user_data)
      : dtor_(dtor), user_data_(user_data) {}
  ~webrtc_RefCountInterface_ref() { dtor_(user_data_); }

 private:
  void (*dtor_)(void*);
  void* user_data_;
};

extern "C" {

WEBRTC_EXPORT void webrtc_RefCountInterface_AddRef(
    struct webrtc_RefCountInterface_ref* ref) {
  ref->AddRef();
}
WEBRTC_EXPORT void webrtc_RefCountInterface_Release(
    struct webrtc_RefCountInterface_ref* ref) {
  ref->Release();
}
WEBRTC_EXPORT struct webrtc_RefCountInterface_ref*
webrtc_RefCountInterface_Create(void (*dtor)(void*), void* user_data) {
  return webrtc::make_ref_counted<webrtc_RefCountInterface_ref>(dtor, user_data)
      .release();
}
}
