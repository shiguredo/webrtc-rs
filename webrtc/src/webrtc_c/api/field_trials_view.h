#pragma once

#include <stddef.h>

#include "../common.h"
#include "../std.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::FieldTrialsView
// -------------------------

struct webrtc_FieldTrialsView;
// webrtc::FieldTrialsView::IsEnabled に対応する。
WEBRTC_EXPORT int webrtc_FieldTrialsView_IsEnabled(
    const struct webrtc_FieldTrialsView* self,
    const char* key,
    size_t key_len);
// webrtc::FieldTrialsView::IsDisabled に対応する。
WEBRTC_EXPORT int webrtc_FieldTrialsView_IsDisabled(
    const struct webrtc_FieldTrialsView* self,
    const char* key,
    size_t key_len);
// webrtc::FieldTrialsView::Lookup に対応する。
// 設定された値のヒープ確保したコピーを返し、呼び出し側が std_string_unique_delete で
// 破棄する。設定されていないキーでは空文字列を返す。
WEBRTC_EXPORT struct std_string_unique* webrtc_FieldTrialsView_Lookup(
    const struct webrtc_FieldTrialsView* self,
    const char* key,
    size_t key_len);

#if defined(__cplusplus)
}
#endif
