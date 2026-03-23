#pragma once

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

WEBRTC_DECLARE_REFCOUNTED(webrtc_AudioTrackInterface);
WEBRTC_DECLARE_CAST_REFCOUNTED(webrtc_AudioTrackInterface,
                               webrtc_MediaStreamTrackInterface);
WEBRTC_DECLARE_CAST_REFCOUNTED(webrtc_MediaStreamTrackInterface,
                               webrtc_AudioTrackInterface);

#if defined(__cplusplus)
}
#endif
