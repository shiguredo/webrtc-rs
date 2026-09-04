#pragma once

#include "../common.h"
#include "../std.h"
#include "frame_transformer_interface.h"
#include "media_stream_interface.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::RtpReceiverInterface
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_RtpReceiverInterface);

WEBRTC_EXPORT struct webrtc_MediaStreamTrackInterface_refcounted*
webrtc_RtpReceiverInterface_track(struct webrtc_RtpReceiverInterface* self);
// 受信器に関連付けられた Stream ID 群を新規確保したベクタで返す。
// 呼び出し側が所有権を持ち、std_string_vector_delete で破棄する。
WEBRTC_EXPORT struct std_string_vector* webrtc_RtpReceiverInterface_stream_ids(
    const struct webrtc_RtpReceiverInterface* self);
WEBRTC_EXPORT void webrtc_RtpReceiverInterface_SetFrameTransformer(
    struct webrtc_RtpReceiverInterface* self,
    struct webrtc_FrameTransformerInterface_refcounted* frame_transformer);

#if defined(__cplusplus)
}
#endif
