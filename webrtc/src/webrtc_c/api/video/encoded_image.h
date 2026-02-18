#pragma once

#include <stddef.h>
#include <stdint.h>

#include "../../common.h"
#include "encoded_image_buffer.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::EncodedImage
// -------------------------

WEBRTC_DECLARE_UNIQUE(webrtc_EncodedImage);

struct webrtc_EncodedImage;
struct webrtc_EncodedImageBuffer;

struct webrtc_EncodedImage_unique* webrtc_EncodedImage_new();
void webrtc_EncodedImage_set_encoded_data(
    struct webrtc_EncodedImage* self,
    struct webrtc_EncodedImageBuffer* encoded_data);
void webrtc_EncodedImage_set_rtp_timestamp(struct webrtc_EncodedImage* self,
                                           uint32_t rtp_timestamp);
void webrtc_EncodedImage_set_encoded_width(struct webrtc_EncodedImage* self,
                                           uint32_t encoded_width);
void webrtc_EncodedImage_set_encoded_height(struct webrtc_EncodedImage* self,
                                            uint32_t encoded_height);
void webrtc_EncodedImage_set_frame_type(struct webrtc_EncodedImage* self,
                                        int frame_type);
void webrtc_EncodedImage_set_qp(struct webrtc_EncodedImage* self, int qp);

struct webrtc_EncodedImageBuffer_refcounted* webrtc_EncodedImage_encoded_data(
    struct webrtc_EncodedImage* self);
uint32_t webrtc_EncodedImage_rtp_timestamp(struct webrtc_EncodedImage* self);
uint32_t webrtc_EncodedImage_encoded_width(struct webrtc_EncodedImage* self);
uint32_t webrtc_EncodedImage_encoded_height(struct webrtc_EncodedImage* self);
int webrtc_EncodedImage_frame_type(struct webrtc_EncodedImage* self);
int webrtc_EncodedImage_qp(struct webrtc_EncodedImage* self);

#if defined(__cplusplus)
}
#endif
