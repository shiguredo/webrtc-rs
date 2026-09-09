#pragma once

#include <stddef.h>
#include <stdint.h>

#include "../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::Buffer
// -------------------------

WEBRTC_EXPORT struct webrtc_Buffer* webrtc_Buffer_new();
WEBRTC_EXPORT void webrtc_Buffer_delete(struct webrtc_Buffer* self);
WEBRTC_EXPORT void webrtc_Buffer_Clear(struct webrtc_Buffer* self);
WEBRTC_EXPORT void webrtc_Buffer_AppendData(struct webrtc_Buffer* self,
                                            const uint8_t* data,
                                            size_t len);
WEBRTC_EXPORT size_t webrtc_Buffer_size(const struct webrtc_Buffer* self);
WEBRTC_EXPORT const uint8_t* webrtc_Buffer_data(
    const struct webrtc_Buffer* self);

// -------------------------
// webrtc::BufferT<int16_t>
// -------------------------

struct webrtc_BufferS16;
WEBRTC_EXPORT void webrtc_BufferS16_Clear(struct webrtc_BufferS16* self);
WEBRTC_EXPORT void webrtc_BufferS16_AppendData(struct webrtc_BufferS16* self,
                                               const int16_t* data,
                                               size_t len);
WEBRTC_EXPORT size_t webrtc_BufferS16_size(const struct webrtc_BufferS16* self);
WEBRTC_EXPORT const int16_t* webrtc_BufferS16_data(
    const struct webrtc_BufferS16* self);

#if defined(__cplusplus)
}
#endif
