#include "field_trials.h"

#include <stddef.h>
#include <memory>

// WebRTC
#include <absl/strings/string_view.h>
#include <api/field_trials.h>

#include "../common.h"
#include "../common.impl.h"

// -------------------------
// webrtc::FieldTrials
// -------------------------

extern "C" {
WEBRTC_DEFINE_UNIQUE(webrtc_FieldTrials, webrtc::FieldTrials);
WEBRTC_EXPORT struct webrtc_FieldTrials_unique* webrtc_FieldTrials_Create(
    const char* s,
    size_t s_len) {
  std::unique_ptr<webrtc::FieldTrials> field_trials =
      webrtc::FieldTrials::Create(absl::string_view(s, s_len));
  return reinterpret_cast<struct webrtc_FieldTrials_unique*>(
      field_trials.release());
}
}
