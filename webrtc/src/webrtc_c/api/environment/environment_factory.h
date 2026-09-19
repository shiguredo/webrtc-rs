#pragma once

#include "../../common.h"
#include "../field_trials.h"
#include "environment.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::EnvironmentFactory
// -------------------------

struct webrtc_EnvironmentFactory;
WEBRTC_EXPORT struct webrtc_EnvironmentFactory* webrtc_EnvironmentFactory_new();
WEBRTC_EXPORT void webrtc_EnvironmentFactory_delete(
    struct webrtc_EnvironmentFactory* self);
// webrtc::EnvironmentFactory::Set(std::unique_ptr<const webrtc::FieldTrialsView>) に対応する。
// field_trials の所有権は EnvironmentFactory に移る。
WEBRTC_EXPORT void webrtc_EnvironmentFactory_Set_field_trials(
    struct webrtc_EnvironmentFactory* self,
    struct webrtc_FieldTrials_unique* utility);
// webrtc::EnvironmentFactory::Create() に対応する。
WEBRTC_EXPORT struct webrtc_Environment* webrtc_EnvironmentFactory_Create(
    const struct webrtc_EnvironmentFactory* self);

#if defined(__cplusplus)
}
#endif
