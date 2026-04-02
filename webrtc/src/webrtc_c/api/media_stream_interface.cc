#include "media_stream_interface.h"

#include <stddef.h>
#include <stdint.h>
#include <memory>
#include <string>
#include <vector>  // IWYU pragma: keep

// WebRTC
#include <api/media_stream_interface.h>
#include <api/scoped_refptr.h>
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
// webrtc::AudioTrackSinkInterface
// -------------------------

class AudioTrackSinkInterfaceImpl : public webrtc::AudioTrackSinkInterface {
 public:
  AudioTrackSinkInterfaceImpl(
      const struct webrtc_AudioTrackSinkInterface_cbs* cbs,
      void* user_data)
      : user_data_(user_data) {
    if (cbs != nullptr) {
      cbs_ = *cbs;
    }
  }

  ~AudioTrackSinkInterfaceImpl() override {
    if (cbs_.OnDestroy != nullptr) {
      cbs_.OnDestroy(user_data_);
    }
  }

  void OnData(const void* audio_data,
              int bits_per_sample,
              int sample_rate,
              size_t number_of_channels,
              size_t number_of_frames) override {
    if (cbs_.OnData != nullptr) {
      cbs_.OnData(audio_data, bits_per_sample, sample_rate, number_of_channels,
                  number_of_frames, user_data_);
    }
  }

 private:
  webrtc_AudioTrackSinkInterface_cbs cbs_{};
  void* user_data_;
};

extern "C" {
WEBRTC_EXPORT struct webrtc_AudioTrackSinkInterface*
webrtc_AudioTrackSinkInterface_new(
    const struct webrtc_AudioTrackSinkInterface_cbs* cbs,
    void* user_data) {
  auto sink = new AudioTrackSinkInterfaceImpl(cbs, user_data);
  return reinterpret_cast<struct webrtc_AudioTrackSinkInterface*>(sink);
}

WEBRTC_EXPORT void webrtc_AudioTrackSinkInterface_delete(
    struct webrtc_AudioTrackSinkInterface* self) {
  auto sink = reinterpret_cast<AudioTrackSinkInterfaceImpl*>(self);
  delete sink;
}
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

// -------------------------
// std::vector<scoped_refptr<webrtc::AudioTrackInterface>>
// -------------------------

WEBRTC_DEFINE_REFCOUNTED_VECTOR(webrtc_AudioTrackInterface,
                                webrtc::AudioTrackInterface);

// -------------------------
// std::vector<scoped_refptr<webrtc::VideoTrackInterface>>
// -------------------------

WEBRTC_DEFINE_REFCOUNTED_VECTOR(webrtc_VideoTrackInterface,
                                webrtc::VideoTrackInterface);

// -------------------------
// webrtc::MediaStreamInterface
// -------------------------

WEBRTC_DEFINE_REFCOUNTED(webrtc_MediaStreamInterface,
                         webrtc::MediaStreamInterface);
WEBRTC_EXPORT struct std_string_unique* webrtc_MediaStreamInterface_id(
    struct webrtc_MediaStreamInterface* self) {
  auto stream = reinterpret_cast<webrtc::MediaStreamInterface*>(self);
  auto id = std::make_unique<std::string>(stream->id());
  return reinterpret_cast<struct std_string_unique*>(id.release());
}

WEBRTC_EXPORT struct webrtc_AudioTrackInterface_refcounted_vector*
webrtc_MediaStreamInterface_GetAudioTracks(
    struct webrtc_MediaStreamInterface* self) {
  auto stream = reinterpret_cast<webrtc::MediaStreamInterface*>(self);
  auto tracks = new webrtc::AudioTrackVector(stream->GetAudioTracks());
  return reinterpret_cast<struct webrtc_AudioTrackInterface_refcounted_vector*>(
      tracks);
}

WEBRTC_EXPORT struct webrtc_VideoTrackInterface_refcounted_vector*
webrtc_MediaStreamInterface_GetVideoTracks(
    struct webrtc_MediaStreamInterface* self) {
  auto stream = reinterpret_cast<webrtc::MediaStreamInterface*>(self);
  auto tracks = new webrtc::VideoTrackVector(stream->GetVideoTracks());
  return reinterpret_cast<struct webrtc_VideoTrackInterface_refcounted_vector*>(
      tracks);
}

WEBRTC_EXPORT struct webrtc_AudioTrackInterface_refcounted*
webrtc_MediaStreamInterface_FindAudioTrack(
    struct webrtc_MediaStreamInterface* self,
    const char* track_id,
    size_t track_id_len) {
  auto stream = reinterpret_cast<webrtc::MediaStreamInterface*>(self);
  auto track = stream->FindAudioTrack(std::string(track_id, track_id_len));
  return reinterpret_cast<struct webrtc_AudioTrackInterface_refcounted*>(
      track.release());
}

WEBRTC_EXPORT struct webrtc_VideoTrackInterface_refcounted*
webrtc_MediaStreamInterface_FindVideoTrack(
    struct webrtc_MediaStreamInterface* self,
    const char* track_id,
    size_t track_id_len) {
  auto stream = reinterpret_cast<webrtc::MediaStreamInterface*>(self);
  auto track = stream->FindVideoTrack(std::string(track_id, track_id_len));
  return reinterpret_cast<struct webrtc_VideoTrackInterface_refcounted*>(
      track.release());
}

WEBRTC_EXPORT int8_t webrtc_MediaStreamInterface_AddTrackWithAudioTrack(
    struct webrtc_MediaStreamInterface* self,
    struct webrtc_AudioTrackInterface_refcounted* track) {
  auto stream = reinterpret_cast<webrtc::MediaStreamInterface*>(self);
  auto raw_track = webrtc_AudioTrackInterface_refcounted_get(track);
  auto audio_track = reinterpret_cast<webrtc::AudioTrackInterface*>(raw_track);
  webrtc::scoped_refptr<webrtc::AudioTrackInterface> track_ref(audio_track);
  return stream->AddTrack(track_ref) ? 1 : 0;
}

WEBRTC_EXPORT int8_t webrtc_MediaStreamInterface_AddTrackWithVideoTrack(
    struct webrtc_MediaStreamInterface* self,
    struct webrtc_VideoTrackInterface_refcounted* track) {
  auto stream = reinterpret_cast<webrtc::MediaStreamInterface*>(self);
  auto raw_track = webrtc_VideoTrackInterface_refcounted_get(track);
  auto video_track = reinterpret_cast<webrtc::VideoTrackInterface*>(raw_track);
  webrtc::scoped_refptr<webrtc::VideoTrackInterface> track_ref(video_track);
  return stream->AddTrack(track_ref) ? 1 : 0;
}

WEBRTC_EXPORT int8_t webrtc_MediaStreamInterface_RemoveTrackWithAudioTrack(
    struct webrtc_MediaStreamInterface* self,
    struct webrtc_AudioTrackInterface_refcounted* track) {
  auto stream = reinterpret_cast<webrtc::MediaStreamInterface*>(self);
  auto raw_track = webrtc_AudioTrackInterface_refcounted_get(track);
  auto audio_track = reinterpret_cast<webrtc::AudioTrackInterface*>(raw_track);
  webrtc::scoped_refptr<webrtc::AudioTrackInterface> track_ref(audio_track);
  return stream->RemoveTrack(track_ref) ? 1 : 0;
}

WEBRTC_EXPORT int8_t webrtc_MediaStreamInterface_RemoveTrackWithVideoTrack(
    struct webrtc_MediaStreamInterface* self,
    struct webrtc_VideoTrackInterface_refcounted* track) {
  auto stream = reinterpret_cast<webrtc::MediaStreamInterface*>(self);
  auto raw_track = webrtc_VideoTrackInterface_refcounted_get(track);
  auto video_track = reinterpret_cast<webrtc::VideoTrackInterface*>(raw_track);
  webrtc::scoped_refptr<webrtc::VideoTrackInterface> track_ref(video_track);
  return stream->RemoveTrack(track_ref) ? 1 : 0;
}
}
