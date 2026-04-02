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
WEBRTC_DECLARE_CAST_REFCOUNTED(webrtc_MediaStreamTrackInterface,
                               webrtc_VideoTrackInterface);
WEBRTC_DECLARE_CAST_REFCOUNTED(webrtc_MediaStreamTrackInterface,
                               webrtc_AudioTrackInterface);
WEBRTC_EXPORT struct std_string_unique* webrtc_MediaStreamTrackInterface_kind(
    struct webrtc_MediaStreamTrackInterface* self);
WEBRTC_EXPORT struct std_string_unique* webrtc_MediaStreamTrackInterface_id(
    struct webrtc_MediaStreamTrackInterface* self);
WEBRTC_EXPORT int8_t webrtc_MediaStreamTrackInterface_enabled(
    struct webrtc_MediaStreamTrackInterface* self);
WEBRTC_EXPORT int8_t webrtc_MediaStreamTrackInterface_set_enabled(
    struct webrtc_MediaStreamTrackInterface* self,
    int8_t enable);

// -------------------------
// webrtc::VideoTrackInterface
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_VideoTrackInterface);
WEBRTC_DECLARE_CAST_REFCOUNTED(webrtc_VideoTrackInterface,
                               webrtc_MediaStreamTrackInterface);
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
// webrtc::AudioTrackSinkInterface
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

// -------------------------
// webrtc::AudioTrackInterface
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_AudioTrackInterface);
WEBRTC_DECLARE_CAST_REFCOUNTED(webrtc_AudioTrackInterface,
                               webrtc_MediaStreamTrackInterface);
WEBRTC_EXPORT void webrtc_AudioTrackInterface_AddSink(
    struct webrtc_AudioTrackInterface* self,
    struct webrtc_AudioTrackSinkInterface* sink);
WEBRTC_EXPORT void webrtc_AudioTrackInterface_RemoveSink(
    struct webrtc_AudioTrackInterface* self,
    struct webrtc_AudioTrackSinkInterface* sink);

// -------------------------
// std::vector<scoped_refptr<webrtc::AudioTrackInterface>>
// -------------------------

WEBRTC_DECLARE_REFCOUNTED_VECTOR(webrtc_AudioTrackInterface);

// -------------------------
// std::vector<scoped_refptr<webrtc::VideoTrackInterface>>
// -------------------------

WEBRTC_DECLARE_REFCOUNTED_VECTOR(webrtc_VideoTrackInterface);

// -------------------------
// webrtc::MediaStreamInterface
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_MediaStreamInterface);
WEBRTC_EXPORT struct std_string_unique* webrtc_MediaStreamInterface_id(
    struct webrtc_MediaStreamInterface* self);
WEBRTC_EXPORT struct webrtc_AudioTrackInterface_refcounted_vector*
webrtc_MediaStreamInterface_GetAudioTracks(
    struct webrtc_MediaStreamInterface* self);
WEBRTC_EXPORT struct webrtc_VideoTrackInterface_refcounted_vector*
webrtc_MediaStreamInterface_GetVideoTracks(
    struct webrtc_MediaStreamInterface* self);
WEBRTC_EXPORT struct webrtc_AudioTrackInterface_refcounted*
webrtc_MediaStreamInterface_FindAudioTrack(
    struct webrtc_MediaStreamInterface* self,
    const char* track_id,
    size_t track_id_len);
WEBRTC_EXPORT struct webrtc_VideoTrackInterface_refcounted*
webrtc_MediaStreamInterface_FindVideoTrack(
    struct webrtc_MediaStreamInterface* self,
    const char* track_id,
    size_t track_id_len);
WEBRTC_EXPORT int8_t webrtc_MediaStreamInterface_AddTrackWithAudioTrack(
    struct webrtc_MediaStreamInterface* self,
    struct webrtc_AudioTrackInterface_refcounted* track);
WEBRTC_EXPORT int8_t webrtc_MediaStreamInterface_AddTrackWithVideoTrack(
    struct webrtc_MediaStreamInterface* self,
    struct webrtc_VideoTrackInterface_refcounted* track);
WEBRTC_EXPORT int8_t webrtc_MediaStreamInterface_RemoveTrackWithAudioTrack(
    struct webrtc_MediaStreamInterface* self,
    struct webrtc_AudioTrackInterface_refcounted* track);
WEBRTC_EXPORT int8_t webrtc_MediaStreamInterface_RemoveTrackWithVideoTrack(
    struct webrtc_MediaStreamInterface* self,
    struct webrtc_VideoTrackInterface_refcounted* track);

#if defined(__cplusplus)
}
#endif
