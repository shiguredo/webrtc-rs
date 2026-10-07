#include "field_trials_view.h"

#include <stddef.h>
#include <memory>
#include <string>

// WebRTC
#include <absl/strings/string_view.h>
#include <api/field_trials_view.h>

#include "../common.h"

// -------------------------
// webrtc::FieldTrialsView
// -------------------------

extern "C" {
WEBRTC_EXPORT int webrtc_FieldTrialsView_IsEnabled(
    const struct webrtc_FieldTrialsView* self,
    const char* key,
    size_t key_len) {
  auto view = reinterpret_cast<const webrtc::FieldTrialsView*>(self);
  return view->IsEnabled(absl::string_view(key, key_len)) ? 1 : 0;
}
WEBRTC_EXPORT int webrtc_FieldTrialsView_IsDisabled(
    const struct webrtc_FieldTrialsView* self,
    const char* key,
    size_t key_len) {
  auto view = reinterpret_cast<const webrtc::FieldTrialsView*>(self);
  return view->IsDisabled(absl::string_view(key, key_len)) ? 1 : 0;
}
WEBRTC_EXPORT struct std_string_unique* webrtc_FieldTrialsView_Lookup(
    const struct webrtc_FieldTrialsView* self,
    const char* key,
    size_t key_len) {
  auto view = reinterpret_cast<const webrtc::FieldTrialsView*>(self);
  auto value = std::make_unique<std::string>(
      view->Lookup(absl::string_view(key, key_len)));
  return reinterpret_cast<struct std_string_unique*>(value.release());
}
}
