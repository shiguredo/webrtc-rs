#include "environment.h"

#include <stdarg.h>
#include <stddef.h>

// WebRTC
#include <api/environment/environment.h>
#include <api/environment/environment_factory.h>

#include "../common.h"

// -------------------------
// webrtc::Environment
// -------------------------

extern "C" {
struct webrtc_Environment* WEBRTC_EXPORT webrtc_CreateEnvironment() {
  auto env = new webrtc::Environment(webrtc::CreateEnvironment());
  return reinterpret_cast<struct webrtc_Environment*>(env);
}
void WEBRTC_EXPORT webrtc_Environment_delete(struct webrtc_Environment* self) {
  auto env = reinterpret_cast<webrtc::Environment*>(self);
  delete env;
}
}
