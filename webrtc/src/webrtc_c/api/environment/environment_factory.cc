#include "environment_factory.h"

#include <memory>

// WebRTC
#include <api/environment/environment.h>
#include <api/environment/environment_factory.h>
#include <api/field_trials.h>

#include "../../common.h"
#include "../field_trials.h"

// -------------------------
// webrtc::EnvironmentFactory
// -------------------------

extern "C" {
WEBRTC_EXPORT struct webrtc_EnvironmentFactory*
webrtc_EnvironmentFactory_new() {
  auto factory = new webrtc::EnvironmentFactory();
  return reinterpret_cast<struct webrtc_EnvironmentFactory*>(factory);
}
WEBRTC_EXPORT void webrtc_EnvironmentFactory_delete(
    struct webrtc_EnvironmentFactory* self) {
  auto factory = reinterpret_cast<webrtc::EnvironmentFactory*>(self);
  delete factory;
}
WEBRTC_EXPORT void webrtc_EnvironmentFactory_Set_field_trials(
    struct webrtc_EnvironmentFactory* self,
    struct webrtc_FieldTrials_unique* utility) {
  auto factory = reinterpret_cast<webrtc::EnvironmentFactory*>(self);
  // Set() は unique_ptr<const FieldTrialsView> を受け取るため、具象型の
  // unique_ptr<FieldTrials> を構築して所有権ごとムーブする。
  auto field_trials = std::unique_ptr<webrtc::FieldTrials>(
      reinterpret_cast<webrtc::FieldTrials*>(
          webrtc_FieldTrials_unique_get(utility)));
  factory->Set(std::move(field_trials));
}
WEBRTC_EXPORT struct webrtc_Environment* webrtc_EnvironmentFactory_Create(
    const struct webrtc_EnvironmentFactory* self) {
  auto factory = reinterpret_cast<const webrtc::EnvironmentFactory*>(self);
  auto env = new webrtc::Environment(factory->Create());
  return reinterpret_cast<struct webrtc_Environment*>(env);
}
}
