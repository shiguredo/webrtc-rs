#pragma once

#include <stddef.h>
#include <stdint.h>

#include "../common.h"
#include "../std.h"
#include "video/video_sink_interface.h"
#include "video/video_source_interface.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::VideoTrackSourceInterface
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_VideoTrackSourceInterface);

// -------------------------
// webrtc::MediaStreamTrackInterface
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_MediaStreamTrackInterface);
WEBRTC_EXPORT struct std_string_unique* webrtc_MediaStreamTrackInterface_kind(
    struct webrtc_MediaStreamTrackInterface* self);
WEBRTC_EXPORT struct std_string_unique* webrtc_MediaStreamTrackInterface_id(
    struct webrtc_MediaStreamTrackInterface* self);
WEBRTC_EXPORT int8_t webrtc_MediaStreamTrackInterface_enabled(
    struct webrtc_MediaStreamTrackInterface* self);
WEBRTC_EXPORT int8_t webrtc_MediaStreamTrackInterface_set_enabled(
    struct webrtc_MediaStreamTrackInterface* self,
    int8_t enable);
WEBRTC_DECLARE_CAST_REFCOUNTED(webrtc_MediaStreamTrackInterface,
                               webrtc_VideoTrackInterface);
WEBRTC_DECLARE_CAST_REFCOUNTED(webrtc_VideoTrackInterface,
                               webrtc_MediaStreamTrackInterface);

// -------------------------
// webrtc::VideoTrackInterface
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_VideoTrackInterface);
WEBRTC_EXPORT void webrtc_VideoTrackInterface_AddOrUpdateSink(
    struct webrtc_VideoTrackInterface* self,
    struct webrtc_VideoSinkInterface* sink,
    struct webrtc_VideoSinkWants* wants);
WEBRTC_EXPORT void webrtc_VideoTrackInterface_RemoveSink(
    struct webrtc_VideoTrackInterface* self,
    struct webrtc_VideoSinkInterface* sink);

// -------------------------
// webrtc::AudioSourceInterface
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_AudioSourceInterface);

// -------------------------
// webrtc::AudioTrackInterface
// -------------------------

struct webrtc_AudioTrackSinkInterface;

struct webrtc_AudioTrackSinkInterface_cbs {
  void (*OnData)(const void* audio_data,
                 int bits_per_sample,
                 int sample_rate,
                 size_t number_of_channels,
                 size_t number_of_frames,
                 void* user_data);
  void (*OnDestroy)(void* user_data);
};

WEBRTC_EXPORT struct webrtc_AudioTrackSinkInterface*
webrtc_AudioTrackSinkInterface_new(
    const struct webrtc_AudioTrackSinkInterface_cbs* cbs,
    void* user_data);
WEBRTC_EXPORT void webrtc_AudioTrackSinkInterface_delete(
    struct webrtc_AudioTrackSinkInterface* self);

WEBRTC_DECLARE_REFCOUNTED(webrtc_AudioTrackInterface);
WEBRTC_EXPORT void webrtc_AudioTrackInterface_AddSink(
    struct webrtc_AudioTrackInterface* self,
    struct webrtc_AudioTrackSinkInterface* sink);
WEBRTC_EXPORT void webrtc_AudioTrackInterface_RemoveSink(
    struct webrtc_AudioTrackInterface* self,
    struct webrtc_AudioTrackSinkInterface* sink);
WEBRTC_DECLARE_CAST_REFCOUNTED(webrtc_AudioTrackInterface,
                               webrtc_MediaStreamTrackInterface);
WEBRTC_DECLARE_CAST_REFCOUNTED(webrtc_MediaStreamTrackInterface,
                               webrtc_AudioTrackInterface);

#if defined(__cplusplus)
}
#endif
