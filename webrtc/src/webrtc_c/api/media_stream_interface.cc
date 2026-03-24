#include "media_stream_interface.h"

#include <memory>
#include <string>

// WebRTC
#include <api/media_stream_interface.h>
#include <api/video/video_frame.h>
#include <api/video/video_sink_interface.h>
#include <api/video/video_source_interface.h>

#include "../common.h"
#include "../common.impl.h"
#include "../std.h"
#include "video/video_sink_interface.h"
#include "video/video_source_interface.h"

// -------------------------
// webrtc::VideoTrackSourceInterface
// -------------------------

extern "C" {
WEBRTC_DEFINE_REFCOUNTED(webrtc_VideoTrackSourceInterface,
                         webrtc::VideoTrackSourceInterface);
}

// -------------------------
// webrtc::MediaStreamTrackInterface
// -------------------------

extern "C" {
WEBRTC_DEFINE_REFCOUNTED(webrtc_MediaStreamTrackInterface,
                         webrtc::MediaStreamTrackInterface);
WEBRTC_DEFINE_CAST_REFCOUNTED(webrtc_MediaStreamTrackInterface,
                              webrtc_VideoTrackInterface,
                              webrtc::MediaStreamTrackInterface,
                              webrtc::VideoTrackInterface);
WEBRTC_DEFINE_CAST_REFCOUNTED(webrtc_VideoTrackInterface,
                              webrtc_MediaStreamTrackInterface,
                              webrtc::VideoTrackInterface,
                              webrtc::MediaStreamTrackInterface);

WEBRTC_EXPORT struct std_string_unique* webrtc_MediaStreamTrackInterface_kind(
    struct webrtc_MediaStreamTrackInterface* self) {
  auto track = reinterpret_cast<webrtc::MediaStreamTrackInterface*>(self);
  auto kind = std::make_unique<std::string>(track->kind());
  return reinterpret_cast<struct std_string_unique*>(kind.release());
}

WEBRTC_EXPORT struct std_string_unique* webrtc_MediaStreamTrackInterface_id(
    struct webrtc_MediaStreamTrackInterface* self) {
  auto track = reinterpret_cast<webrtc::MediaStreamTrackInterface*>(self);
  auto id = std::make_unique<std::string>(track->id());
  return reinterpret_cast<struct std_string_unique*>(id.release());
}

WEBRTC_EXPORT int8_t webrtc_MediaStreamTrackInterface_enabled(
    struct webrtc_MediaStreamTrackInterface* self) {
  auto track = reinterpret_cast<webrtc::MediaStreamTrackInterface*>(self);
  return track->enabled() ? 1 : 0;
}

WEBRTC_EXPORT int8_t webrtc_MediaStreamTrackInterface_set_enabled(
    struct webrtc_MediaStreamTrackInterface* self,
    int8_t enable) {
  auto track = reinterpret_cast<webrtc::MediaStreamTrackInterface*>(self);
  return track->set_enabled(enable != 0) ? 1 : 0;
}
}

// -------------------------
// webrtc::VideoTrackInterface
// -------------------------

extern "C" {
WEBRTC_DEFINE_REFCOUNTED(webrtc_VideoTrackInterface,
                         webrtc::VideoTrackInterface);

WEBRTC_EXPORT void webrtc_VideoTrackInterface_AddOrUpdateSink(
    struct webrtc_VideoTrackInterface* self,
    struct webrtc_VideoSinkInterface* sink,
    struct webrtc_VideoSinkWants* wants) {
  auto track = reinterpret_cast<webrtc::VideoTrackInterface*>(self);
  auto sink_impl =
      reinterpret_cast<webrtc::VideoSinkInterface<webrtc::VideoFrame>*>(sink);
  auto wants_impl = reinterpret_cast<webrtc::VideoSinkWants*>(wants);
  track->AddOrUpdateSink(sink_impl, *wants_impl);
}

WEBRTC_EXPORT void webrtc_VideoTrackInterface_RemoveSink(
    struct webrtc_VideoTrackInterface* self,
    struct webrtc_VideoSinkInterface* sink) {
  auto track = reinterpret_cast<webrtc::VideoTrackInterface*>(self);
  auto sink_impl =
      reinterpret_cast<webrtc::VideoSinkInterface<webrtc::VideoFrame>*>(sink);
  track->RemoveSink(sink_impl);
}
}

// -------------------------
// webrtc::AudioSourceInterface
// -------------------------

extern "C" {
WEBRTC_DEFINE_REFCOUNTED(webrtc_AudioSourceInterface,
                         webrtc::AudioSourceInterface);
}

// -------------------------
// webrtc::AudioTrackInterface
// -------------------------

extern "C" {
WEBRTC_DEFINE_REFCOUNTED(webrtc_AudioTrackInterface,
                         webrtc::AudioTrackInterface);

WEBRTC_EXPORT void webrtc_AudioTrackInterface_AddSink(
    struct webrtc_AudioTrackInterface* self,
    struct webrtc_AudioTrackSinkInterface* sink) {
  auto track = reinterpret_cast<webrtc::AudioTrackInterface*>(self);
  auto sink_impl = reinterpret_cast<webrtc::AudioTrackSinkInterface*>(sink);
  track->AddSink(sink_impl);
}

WEBRTC_EXPORT void webrtc_AudioTrackInterface_RemoveSink(
    struct webrtc_AudioTrackInterface* self,
    struct webrtc_AudioTrackSinkInterface* sink) {
  auto track = reinterpret_cast<webrtc::AudioTrackInterface*>(self);
  auto sink_impl = reinterpret_cast<webrtc::AudioTrackSinkInterface*>(sink);
  track->RemoveSink(sink_impl);
}
WEBRTC_DEFINE_CAST_REFCOUNTED(webrtc_AudioTrackInterface,
                              webrtc_MediaStreamTrackInterface,
                              webrtc::AudioTrackInterface,
                              webrtc::MediaStreamTrackInterface);
WEBRTC_DEFINE_CAST_REFCOUNTED(webrtc_MediaStreamTrackInterface,
                              webrtc_AudioTrackInterface,
                              webrtc::MediaStreamTrackInterface,
                              webrtc::AudioTrackInterface);
}
