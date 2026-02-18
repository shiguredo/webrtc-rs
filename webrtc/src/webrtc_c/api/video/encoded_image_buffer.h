#pragma once

#include <stddef.h>
#include <stdint.h>

#include "../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::EncodedImageBuffer
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_EncodedImageBuffer);

struct webrtc_EncodedImageBuffer_refcounted* webrtc_EncodedImageBuffer_Create();
struct webrtc_EncodedImageBuffer_refcounted*
webrtc_EncodedImageBuffer_Create_from_data(const uint8_t* data, size_t size);
size_t webrtc_EncodedImageBuffer_size(struct webrtc_EncodedImageBuffer* self);
const uint8_t* webrtc_EncodedImageBuffer_data(struct webrtc_EncodedImageBuffer* self);

#if defined(__cplusplus)
}
#endif
