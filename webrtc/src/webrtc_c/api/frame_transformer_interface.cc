#include "frame_transformer_interface.h"

#include <assert.h>
#include <memory>
#include <optional>
#include <span>
#include <string>
#include <utility>
#include <variant>

#include <api/frame_transformer_interface.h>
#include <api/make_ref_counted.h>
#include <api/scoped_refptr.h>
#include <api/units/timestamp.h>
#include <api/video/video_frame_metadata.h>

#include "../common.h"
#include "../common.impl.h"
#include "../std.impl.h"
#include "video/video_frame_metadata.h"

class FrameTransformerInterfaceImpl : public webrtc::FrameTransformerInterface {
 public:
  FrameTransformerInterfaceImpl(
      const struct webrtc_FrameTransformerInterface_cbs* cbs,
      void* user_data)
      : user_data_(user_data) {
    assert(cbs != nullptr);
    assert(cbs->Transform != nullptr);
    assert(cbs->RegisterTransformedFrameCallback != nullptr);
    assert(cbs->RegisterTransformedFrameSinkCallback != nullptr);
    assert(cbs->UnregisterTransformedFrameCallback != nullptr);
    assert(cbs->UnregisterTransformedFrameSinkCallback != nullptr);
    assert(cbs->OnDestroy != nullptr);
    cbs_ = *cbs;
  }

  ~FrameTransformerInterfaceImpl() override { cbs_.OnDestroy(user_data_); }

  void Transform(
      std::unique_ptr<webrtc::TransformableFrameInterface> frame) override {
    // フレームの所有権をコールバックへ移し、配送と破棄を Rust 側で判断する。
    auto* raw = frame.release();
    cbs_.Transform(
        reinterpret_cast<struct webrtc_TransformableFrameInterface_unique*>(
            raw),
        user_data_);
  }

  void RegisterTransformedFrameCallback(
      webrtc::scoped_refptr<webrtc::TransformedFrameCallback> callback)
      override {
    cbs_.RegisterTransformedFrameCallback(
        reinterpret_cast<struct webrtc_TransformedFrameCallback_refcounted*>(
            callback.release()),
        user_data_);
  }

  void RegisterTransformedFrameSinkCallback(
      webrtc::scoped_refptr<webrtc::TransformedFrameCallback> callback,
      uint32_t ssrc) override {
    cbs_.RegisterTransformedFrameSinkCallback(
        reinterpret_cast<struct webrtc_TransformedFrameCallback_refcounted*>(
            callback.release()),
        ssrc, user_data_);
  }

  void UnregisterTransformedFrameCallback() override {
    cbs_.UnregisterTransformedFrameCallback(user_data_);
  }

  void UnregisterTransformedFrameSinkCallback(uint32_t ssrc) override {
    cbs_.UnregisterTransformedFrameSinkCallback(ssrc, user_data_);
  }

 private:
  struct webrtc_FrameTransformerInterface_cbs cbs_{};
  void* user_data_;
};

extern "C" {
WEBRTC_DEFINE_REFCOUNTED(webrtc_FrameTransformerInterface,
                         webrtc::FrameTransformerInterface);
WEBRTC_DEFINE_UNIQUE(webrtc_TransformableFrameInterface,
                     webrtc::TransformableFrameInterface);
WEBRTC_DEFINE_REFCOUNTED(webrtc_TransformedFrameCallback,
                         webrtc::TransformedFrameCallback);
WEBRTC_DEFINE_VARIANT(webrtc_RtpTimestampInfo, webrtc::RtpTimestampInfo);

// -------------------------
// webrtc::RtpTimestampInfo
// -------------------------

WEBRTC_EXPORT struct webrtc_RtpTimestampWithOffset*
webrtc_RtpTimestampInfo_get_RtpTimestampWithOffset(
    struct webrtc_RtpTimestampInfo* self) {
  auto variant = reinterpret_cast<webrtc::RtpTimestampInfo*>(self);
  auto* value = std::get_if<webrtc::RtpTimestampWithOffset>(variant);
  return value == nullptr
             ? nullptr
             : reinterpret_cast<struct webrtc_RtpTimestampWithOffset*>(value);
}

WEBRTC_EXPORT struct webrtc_RtpTimestampWithoutOffset*
webrtc_RtpTimestampInfo_get_RtpTimestampWithoutOffset(
    struct webrtc_RtpTimestampInfo* self) {
  auto variant = reinterpret_cast<webrtc::RtpTimestampInfo*>(self);
  auto* value = std::get_if<webrtc::RtpTimestampWithoutOffset>(variant);
  return value == nullptr
             ? nullptr
             : reinterpret_cast<struct webrtc_RtpTimestampWithoutOffset*>(
                   value);
}

WEBRTC_EXPORT uint32_t webrtc_RtpTimestampWithOffset_get_value(
    struct webrtc_RtpTimestampWithOffset* self) {
  auto value = reinterpret_cast<webrtc::RtpTimestampWithOffset*>(self);
  return value->value;
}

WEBRTC_EXPORT uint32_t webrtc_RtpTimestampWithoutOffset_get_value(
    struct webrtc_RtpTimestampWithoutOffset* self) {
  auto value = reinterpret_cast<webrtc::RtpTimestampWithoutOffset*>(self);
  return value->value;
}

WEBRTC_EXPORT struct webrtc_FrameTransformerInterface_refcounted*
webrtc_FrameTransformerInterface_new(
    const struct webrtc_FrameTransformerInterface_cbs* cbs,
    void* user_data) {
  auto impl =
      webrtc::make_ref_counted<FrameTransformerInterfaceImpl>(cbs, user_data);
  return reinterpret_cast<struct webrtc_FrameTransformerInterface_refcounted*>(
      impl.release());
}

WEBRTC_EXPORT void webrtc_TransformedFrameCallback_OnTransformedFrame(
    struct webrtc_TransformedFrameCallback* self,
    struct webrtc_TransformableFrameInterface_unique* frame) {
  auto callback = reinterpret_cast<webrtc::TransformedFrameCallback*>(self);
  auto f = std::unique_ptr<webrtc::TransformableFrameInterface>(
      reinterpret_cast<webrtc::TransformableFrameInterface*>(
          webrtc_TransformableFrameInterface_unique_get(frame)));
  callback->OnTransformedFrame(std::move(f));
}

WEBRTC_EXPORT void webrtc_TransformableFrameInterface_GetData(
    struct webrtc_TransformableFrameInterface* self,
    const uint8_t** out_data,
    size_t* out_len) {
  auto frame = reinterpret_cast<webrtc::TransformableFrameInterface*>(self);
  assert(out_data != nullptr);
  assert(out_len != nullptr);
  auto data = frame->GetData();
  *out_data = data.data();
  *out_len = data.size();
}

WEBRTC_EXPORT void webrtc_TransformableFrameInterface_SetData(
    struct webrtc_TransformableFrameInterface* self,
    const uint8_t* data,
    size_t size) {
  auto frame = reinterpret_cast<webrtc::TransformableFrameInterface*>(self);
  frame->SetData(std::span<const uint8_t>(data, size));
}

WEBRTC_EXPORT uint8_t webrtc_TransformableFrameInterface_GetPayloadType(
    struct webrtc_TransformableFrameInterface* self) {
  auto frame = reinterpret_cast<webrtc::TransformableFrameInterface*>(self);
  return frame->GetPayloadType();
}

WEBRTC_EXPORT int webrtc_TransformableFrameInterface_CanSetPayloadType(
    struct webrtc_TransformableFrameInterface* self) {
  auto frame = reinterpret_cast<webrtc::TransformableFrameInterface*>(self);
  return frame->CanSetPayloadType() ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_TransformableFrameInterface_SetPayloadType(
    struct webrtc_TransformableFrameInterface* self,
    uint8_t payload_type) {
  auto frame = reinterpret_cast<webrtc::TransformableFrameInterface*>(self);
  frame->SetPayloadType(payload_type);
}

WEBRTC_EXPORT uint32_t webrtc_TransformableFrameInterface_GetSsrc(
    struct webrtc_TransformableFrameInterface* self) {
  auto frame = reinterpret_cast<webrtc::TransformableFrameInterface*>(self);
  return frame->GetSsrc();
}

WEBRTC_EXPORT struct webrtc_RtpTimestampInfo_unique*
webrtc_TransformableFrameInterface_GetRtpTimestampInfo(
    struct webrtc_TransformableFrameInterface* self) {
  auto frame = reinterpret_cast<webrtc::TransformableFrameInterface*>(self);
  auto variant =
      std::make_unique<webrtc::RtpTimestampInfo>(frame->GetRtpTimestampInfo());
  return reinterpret_cast<struct webrtc_RtpTimestampInfo_unique*>(
      variant.release());
}

WEBRTC_EXPORT void webrtc_TransformableFrameInterface_SetRTPTimestamp(
    struct webrtc_TransformableFrameInterface* self,
    uint32_t rtp_timestamp_with_offset) {
  auto frame = reinterpret_cast<webrtc::TransformableFrameInterface*>(self);
  frame->SetRTPTimestamp(rtp_timestamp_with_offset);
}

WEBRTC_EXPORT int webrtc_TransformableFrameInterface_GetDirection(
    struct webrtc_TransformableFrameInterface* self) {
  auto frame = reinterpret_cast<webrtc::TransformableFrameInterface*>(self);
  switch (frame->GetDirection()) {
    case webrtc::TransformableFrameInterface::Direction::kReceiver:
      return 1;
    case webrtc::TransformableFrameInterface::Direction::kSender:
      return 2;
    default:
      return 0;
  }
}

WEBRTC_EXPORT struct std_string_unique*
webrtc_TransformableFrameInterface_GetMimeType(
    struct webrtc_TransformableFrameInterface* self) {
  auto frame = reinterpret_cast<webrtc::TransformableFrameInterface*>(self);
  auto mime = std::make_unique<std::string>(frame->GetMimeType());
  return reinterpret_cast<struct std_string_unique*>(mime.release());
}

WEBRTC_EXPORT void webrtc_TransformableFrameInterface_ReceiveTime(
    struct webrtc_TransformableFrameInterface* self,
    int* has,
    int64_t* timestamp_us) {
  auto frame = reinterpret_cast<webrtc::TransformableFrameInterface*>(self);
  auto value = frame->ReceiveTime();
  webrtc_c::OptionalGetAs(value, has, timestamp_us,
                          [&]() { return value->us(); });
}

WEBRTC_EXPORT void webrtc_TransformableFrameInterface_GetPresentationTimestamp(
    struct webrtc_TransformableFrameInterface* self,
    int* has,
    int64_t* timestamp_us) {
  auto frame = reinterpret_cast<webrtc::TransformableFrameInterface*>(self);
  auto value = frame->GetPresentationTimestamp();
  webrtc_c::OptionalGetAs(value, has, timestamp_us,
                          [&]() { return value->us(); });
}

WEBRTC_EXPORT void webrtc_TransformableFrameInterface_CaptureTime(
    struct webrtc_TransformableFrameInterface* self,
    int* has,
    int64_t* timestamp_us) {
  auto frame = reinterpret_cast<webrtc::TransformableFrameInterface*>(self);
  auto value = frame->CaptureTime();
  webrtc_c::OptionalGetAs(value, has, timestamp_us,
                          [&]() { return value->us(); });
}

WEBRTC_EXPORT int webrtc_TransformableFrameInterface_CanSetCaptureTime(
    struct webrtc_TransformableFrameInterface* self) {
  auto frame = reinterpret_cast<webrtc::TransformableFrameInterface*>(self);
  return frame->CanSetCaptureTime() ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_TransformableFrameInterface_SetCaptureTime(
    struct webrtc_TransformableFrameInterface* self,
    int has,
    int64_t timestamp_us) {
  auto frame = reinterpret_cast<webrtc::TransformableFrameInterface*>(self);
  std::optional<webrtc::Timestamp> capture_time;
  webrtc_c::OptionalSetAs(capture_time, has, &timestamp_us, [&]() {
    return webrtc::Timestamp::Micros(timestamp_us);
  });
  frame->SetCaptureTime(capture_time);
}

WEBRTC_EXPORT void webrtc_TransformableFrameInterface_SenderCaptureTimeOffset(
    struct webrtc_TransformableFrameInterface* self,
    int* has,
    int64_t* delta_us) {
  auto frame = reinterpret_cast<webrtc::TransformableFrameInterface*>(self);
  auto value = frame->SenderCaptureTimeOffset();
  webrtc_c::OptionalGetAs(value, has, delta_us, [&]() { return value->us(); });
}

WEBRTC_EXPORT int webrtc_TransformableVideoFrameInterface_IsKeyFrame(
    struct webrtc_TransformableVideoFrameInterface* self) {
  auto frame =
      reinterpret_cast<webrtc::TransformableVideoFrameInterface*>(self);
  return frame->IsKeyFrame() ? 1 : 0;
}

WEBRTC_EXPORT void webrtc_TransformableVideoFrameInterface_Rid(
    struct webrtc_TransformableVideoFrameInterface* self,
    int* out_has,
    struct std_string_unique** out_value) {
  assert(out_has != nullptr);
  assert(out_value != nullptr);
  auto frame =
      reinterpret_cast<webrtc::TransformableVideoFrameInterface*>(self);
  auto rid = frame->Rid();
  webrtc_c::OptionalGetAs(rid, out_has, out_value, [&]() {
    auto s = std::make_unique<std::string>(std::move(*rid));
    return reinterpret_cast<struct std_string_unique*>(s.release());
  });
}

WEBRTC_EXPORT struct webrtc_VideoFrameMetadata*
webrtc_TransformableVideoFrameInterface_Metadata(
    struct webrtc_TransformableVideoFrameInterface* self) {
  auto frame =
      reinterpret_cast<webrtc::TransformableVideoFrameInterface*>(self);
  auto metadata =
      std::make_unique<webrtc::VideoFrameMetadata>(frame->Metadata());
  return reinterpret_cast<struct webrtc_VideoFrameMetadata*>(
      metadata.release());
}

WEBRTC_EXPORT void webrtc_TransformableVideoFrameInterface_SetMetadata(
    struct webrtc_TransformableVideoFrameInterface* self,
    const struct webrtc_VideoFrameMetadata* metadata) {
  assert(metadata != nullptr);
  auto frame =
      reinterpret_cast<webrtc::TransformableVideoFrameInterface*>(self);
  auto m = reinterpret_cast<const webrtc::VideoFrameMetadata*>(metadata);
  frame->SetMetadata(*m);
}
}
