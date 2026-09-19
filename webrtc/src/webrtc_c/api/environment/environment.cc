#include "environment.h"

#include <stdarg.h>
#include <stddef.h>

// WebRTC
#include <api/environment/environment.h>
#include <api/environment/environment_factory.h>

#include "../../common.h"

// -------------------------
// webrtc::Environment
// -------------------------

extern "C" {
WEBRTC_EXPORT struct webrtc_Environment* webrtc_CreateEnvironment() {
  auto env = new webrtc::Environment(webrtc::CreateEnvironment());
  return reinterpret_cast<struct webrtc_Environment*>(env);
}
WEBRTC_EXPORT struct webrtc_Environment* webrtc_Environment_copy(
    const struct webrtc_Environment* self) {
  auto env = reinterpret_cast<const webrtc::Environment*>(self);
  return reinterpret_cast<struct webrtc_Environment*>(
      new webrtc::Environment(*env));
}
WEBRTC_EXPORT void webrtc_Environment_delete(struct webrtc_Environment* self) {
  auto env = reinterpret_cast<webrtc::Environment*>(self);
  delete env;
}
WEBRTC_EXPORT const struct webrtc_FieldTrialsView*
webrtc_Environment_field_trials(const struct webrtc_Environment* self) {
  auto env = reinterpret_cast<const webrtc::Environment*>(self);
  // field_trials() が返す参照は Environment が保持しているため、そのまま借用ポインタを返す。
  return reinterpret_cast<const struct webrtc_FieldTrialsView*>(
      &env->field_trials());
}
}
