#include "video_source_interface.h"

#include <api/video/video_source_interface.h>

extern "C" {
struct webrtc_VideoSinkWants* webrtc_VideoSinkWants_new() {
  auto wants = new webrtc::VideoSinkWants();
  return reinterpret_cast<struct webrtc_VideoSinkWants*>(wants);
}

void webrtc_VideoSinkWants_delete(struct webrtc_VideoSinkWants* self) {
  auto wants = reinterpret_cast<webrtc::VideoSinkWants*>(self);
  delete wants;
}
}
