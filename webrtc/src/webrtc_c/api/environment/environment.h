#pragma once

#include "../../common.h"
#include "../field_trials_view.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::Environment
// -------------------------

struct webrtc_Environment;
WEBRTC_EXPORT struct webrtc_Environment* webrtc_CreateEnvironment();
// webrtc::Environment のコピーを生成する。C++ のコピーコンストラクタに対応する。
WEBRTC_EXPORT struct webrtc_Environment* webrtc_Environment_copy(
    const struct webrtc_Environment* self);
WEBRTC_EXPORT void webrtc_Environment_delete(struct webrtc_Environment* self);
// webrtc::Environment::field_trials() に対応する。
// 戻り値は Environment が保持する FieldTrialsView への借用ポインタであり、呼び出し側は解放しない。
WEBRTC_EXPORT const struct webrtc_FieldTrialsView*
webrtc_Environment_field_trials(const struct webrtc_Environment* self);

#if defined(__cplusplus)
}
#endif
