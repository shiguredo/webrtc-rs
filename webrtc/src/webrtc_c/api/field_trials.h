#pragma once

#include <stddef.h>

#include "../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::FieldTrials
// -------------------------

WEBRTC_DECLARE_UNIQUE(webrtc_FieldTrials);
// webrtc::FieldTrials::Create() に対応する。
// フィールドトライアル文字列が不正な場合は nullptr を返す。
WEBRTC_EXPORT struct webrtc_FieldTrials_unique* webrtc_FieldTrials_Create(
    const char* s,
    size_t s_len);

#if defined(__cplusplus)
}
#endif
