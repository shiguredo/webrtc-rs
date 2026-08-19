#pragma once

#include <stddef.h>
#include <stdint.h>

#include "../common.h"
#include "../std.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::FrameTransformerInterface
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_FrameTransformerInterface);

struct webrtc_TransformableFrameInterface;

// 全コールバックは必須（null 非許容）。
// 呼び出し側は全関数ポインタを非 null で設定しなければならない。
// 各コールバック名は元の C++ のメソッド名と一致させる。
struct webrtc_FrameTransformerInterface_cbs {
  // 変換処理を実行する。フレームの所有権は呼び出し側へ移るため、
  // 配送する場合は webrtc_TransformedFrameCallback_OnTransformedFrame を、
  // 破棄する場合は webrtc_TransformableFrameInterface_unique_delete を呼ぶこと。
  void (*Transform)(struct webrtc_TransformableFrameInterface_unique* frame,
                    void* user_data);
  void (*RegisterTransformedFrameCallback)(
      struct webrtc_TransformedFrameCallback_refcounted* callback,
      void* user_data);
  void (*RegisterTransformedFrameSinkCallback)(
      struct webrtc_TransformedFrameCallback_refcounted* callback,
      uint32_t ssrc,
      void* user_data);
  void (*UnregisterTransformedFrameCallback)(void* user_data);
  void (*UnregisterTransformedFrameSinkCallback)(uint32_t ssrc,
                                                 void* user_data);
  void (*OnDestroy)(void* user_data);
};

WEBRTC_EXPORT struct webrtc_FrameTransformerInterface_refcounted*
webrtc_FrameTransformerInterface_new(
    const struct webrtc_FrameTransformerInterface_cbs* cbs,
    void* user_data);

// -------------------------
// webrtc::RtpTimestampInfo
// (std::variant<RtpTimestampWithOffset, RtpTimestampWithoutOffset>)
// -------------------------

// ヒープ確保したコピーを返し、webrtc_RtpTimestampInfo_unique_delete で破棄する。
WEBRTC_DECLARE_VARIANT(webrtc_RtpTimestampInfo);

// 各 alternative の値。アクティブでない場合は null。戻り値は借用 (delete しない)。
WEBRTC_EXPORT struct webrtc_RtpTimestampWithOffset*
webrtc_RtpTimestampInfo_get_RtpTimestampWithOffset(
    struct webrtc_RtpTimestampInfo* self);
WEBRTC_EXPORT struct webrtc_RtpTimestampWithoutOffset*
webrtc_RtpTimestampInfo_get_RtpTimestampWithoutOffset(
    struct webrtc_RtpTimestampInfo* self);
// alternative の C 型。フィールドへは get_field 規約でアクセスする。
struct webrtc_RtpTimestampWithOffset;
WEBRTC_EXPORT uint32_t webrtc_RtpTimestampWithOffset_get_value(
    struct webrtc_RtpTimestampWithOffset* self);
struct webrtc_RtpTimestampWithoutOffset;
WEBRTC_EXPORT uint32_t webrtc_RtpTimestampWithoutOffset_get_value(
    struct webrtc_RtpTimestampWithoutOffset* self);

// -------------------------
// webrtc::TransformableFrameInterface
// -------------------------

WEBRTC_DECLARE_UNIQUE(webrtc_TransformableFrameInterface);

WEBRTC_EXPORT void webrtc_TransformableFrameInterface_GetData(
    struct webrtc_TransformableFrameInterface* self,
    const uint8_t** out_data,
    size_t* out_len);
WEBRTC_EXPORT void webrtc_TransformableFrameInterface_SetData(
    struct webrtc_TransformableFrameInterface* self,
    const uint8_t* data,
    size_t size);
WEBRTC_EXPORT uint8_t webrtc_TransformableFrameInterface_GetPayloadType(
    struct webrtc_TransformableFrameInterface* self);
WEBRTC_EXPORT int webrtc_TransformableFrameInterface_CanSetPayloadType(
    struct webrtc_TransformableFrameInterface* self);
WEBRTC_EXPORT void webrtc_TransformableFrameInterface_SetPayloadType(
    struct webrtc_TransformableFrameInterface* self,
    uint8_t payload_type);
WEBRTC_EXPORT uint32_t webrtc_TransformableFrameInterface_GetSsrc(
    struct webrtc_TransformableFrameInterface* self);
// std::variant<RtpTimestampWithOffset, RtpTimestampWithoutOffset> のコピーを返す。
// 破棄は webrtc_RtpTimestampInfo_unique_delete で行う。
WEBRTC_EXPORT struct webrtc_RtpTimestampInfo_unique*
webrtc_TransformableFrameInterface_GetRtpTimestampInfo(
    struct webrtc_TransformableFrameInterface* self);
WEBRTC_EXPORT void webrtc_TransformableFrameInterface_SetRTPTimestamp(
    struct webrtc_TransformableFrameInterface* self,
    uint32_t rtp_timestamp_with_offset);
// Direction を返す。0=kUnknown, 1=kReceiver, 2=kSender。
WEBRTC_EXPORT int webrtc_TransformableFrameInterface_GetDirection(
    struct webrtc_TransformableFrameInterface* self);
WEBRTC_EXPORT struct std_string_unique*
webrtc_TransformableFrameInterface_GetMimeType(
    struct webrtc_TransformableFrameInterface* self);
// 各 optional は int* has に 0 か 1 を設定し、値はマイクロ秒で返す。
WEBRTC_EXPORT void webrtc_TransformableFrameInterface_ReceiveTime(
    struct webrtc_TransformableFrameInterface* self,
    int* has,
    int64_t* timestamp_us);
WEBRTC_EXPORT void webrtc_TransformableFrameInterface_GetPresentationTimestamp(
    struct webrtc_TransformableFrameInterface* self,
    int* has,
    int64_t* timestamp_us);
WEBRTC_EXPORT void webrtc_TransformableFrameInterface_CaptureTime(
    struct webrtc_TransformableFrameInterface* self,
    int* has,
    int64_t* timestamp_us);
WEBRTC_EXPORT int webrtc_TransformableFrameInterface_CanSetCaptureTime(
    struct webrtc_TransformableFrameInterface* self);
WEBRTC_EXPORT void webrtc_TransformableFrameInterface_SetCaptureTime(
    struct webrtc_TransformableFrameInterface* self,
    int has,
    int64_t timestamp_us);
WEBRTC_EXPORT void webrtc_TransformableFrameInterface_SenderCaptureTimeOffset(
    struct webrtc_TransformableFrameInterface* self,
    int* has,
    int64_t* delta_us);

// -------------------------
// webrtc::TransformableVideoFrameInterface
// -------------------------

struct webrtc_TransformableVideoFrameInterface;

WEBRTC_EXPORT int webrtc_TransformableVideoFrameInterface_IsKeyFrame(
    struct webrtc_TransformableVideoFrameInterface* self);
// std::optional<std::string>。値がある場合はヒープ確保したコピーを
// out_value に返し、呼び出し側が std_string_unique_delete で破棄する。
WEBRTC_EXPORT void webrtc_TransformableVideoFrameInterface_Rid(
    struct webrtc_TransformableVideoFrameInterface* self,
    int* out_has,
    struct std_string_unique** out_value);
WEBRTC_EXPORT struct webrtc_VideoFrameMetadata*
webrtc_TransformableVideoFrameInterface_Metadata(
    struct webrtc_TransformableVideoFrameInterface* self);
WEBRTC_EXPORT void webrtc_TransformableVideoFrameInterface_SetMetadata(
    struct webrtc_TransformableVideoFrameInterface* self,
    const struct webrtc_VideoFrameMetadata* metadata);

// -------------------------
// webrtc::TransformedFrameCallback
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_TransformedFrameCallback);

// frame の所有権を引き継いで OnTransformedFrame を呼ぶ。
WEBRTC_EXPORT void webrtc_TransformedFrameCallback_OnTransformedFrame(
    struct webrtc_TransformedFrameCallback* self,
    struct webrtc_TransformableFrameInterface_unique* frame);

#if defined(__cplusplus)
}
#endif
