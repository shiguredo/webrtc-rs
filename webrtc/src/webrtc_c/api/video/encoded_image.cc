#include "encoded_image.h"

#include <stddef.h>
#include <stdint.h>
#include <memory>

// WebRTC
#include <api/scoped_refptr.h>
#include <api/video/encoded_image.h>

#include "../../common.impl.h"

extern "C" {
WEBRTC_DEFINE_UNIQUE(webrtc_EncodedImage, webrtc::EncodedImage);

struct webrtc_EncodedImage_unique* webrtc_EncodedImage_new() {
  auto image = std::make_unique<webrtc::EncodedImage>();
  return reinterpret_cast<struct webrtc_EncodedImage_unique*>(image.release());
}

void webrtc_EncodedImage_set_encoded_data(
    struct webrtc_EncodedImage* self,
    struct webrtc_EncodedImageBuffer* encoded_data) {
  auto image = reinterpret_cast<webrtc::EncodedImage*>(self);
  if (encoded_data == nullptr) {
    image->ClearEncodedData();
    return;
  }
  auto buffer =
      reinterpret_cast<webrtc::EncodedImageBufferInterface*>(encoded_data);
  webrtc::scoped_refptr<webrtc::EncodedImageBufferInterface> ref(buffer);
  image->SetEncodedData(ref);
}

void webrtc_EncodedImage_set_rtp_timestamp(struct webrtc_EncodedImage* self,
                                           uint32_t rtp_timestamp) {
  auto image = reinterpret_cast<webrtc::EncodedImage*>(self);
  image->SetRtpTimestamp(rtp_timestamp);
}

void webrtc_EncodedImage_set_encoded_width(struct webrtc_EncodedImage* self,
                                           uint32_t encoded_width) {
  auto image = reinterpret_cast<webrtc::EncodedImage*>(self);
  image->_encodedWidth = encoded_width;
}

void webrtc_EncodedImage_set_encoded_height(struct webrtc_EncodedImage* self,
                                            uint32_t encoded_height) {
  auto image = reinterpret_cast<webrtc::EncodedImage*>(self);
  image->_encodedHeight = encoded_height;
}

void webrtc_EncodedImage_set_frame_type(struct webrtc_EncodedImage* self,
                                        int frame_type) {
  auto image = reinterpret_cast<webrtc::EncodedImage*>(self);
  image->SetFrameType(static_cast<webrtc::VideoFrameType>(frame_type));
}

void webrtc_EncodedImage_set_qp(struct webrtc_EncodedImage* self, int qp) {
  auto image = reinterpret_cast<webrtc::EncodedImage*>(self);
  image->qp_ = qp;
}

struct webrtc_EncodedImageBuffer_refcounted* webrtc_EncodedImage_encoded_data(
    struct webrtc_EncodedImage* self) {
  auto image = reinterpret_cast<webrtc::EncodedImage*>(self);
  auto encoded_data = image->GetEncodedData();
  if (encoded_data == nullptr) {
    return nullptr;
  }
  return reinterpret_cast<struct webrtc_EncodedImageBuffer_refcounted*>(
      encoded_data.release());
}

uint32_t webrtc_EncodedImage_rtp_timestamp(struct webrtc_EncodedImage* self) {
  auto image = reinterpret_cast<webrtc::EncodedImage*>(self);
  return image->RtpTimestamp();
}

uint32_t webrtc_EncodedImage_encoded_width(struct webrtc_EncodedImage* self) {
  auto image = reinterpret_cast<webrtc::EncodedImage*>(self);
  return image->_encodedWidth;
}

uint32_t webrtc_EncodedImage_encoded_height(struct webrtc_EncodedImage* self) {
  auto image = reinterpret_cast<webrtc::EncodedImage*>(self);
  return image->_encodedHeight;
}

int webrtc_EncodedImage_frame_type(struct webrtc_EncodedImage* self) {
  auto image = reinterpret_cast<webrtc::EncodedImage*>(self);
  return static_cast<int>(image->FrameType());
}

int webrtc_EncodedImage_qp(struct webrtc_EncodedImage* self) {
  auto image = reinterpret_cast<webrtc::EncodedImage*>(self);
  return image->qp_;
}
}
