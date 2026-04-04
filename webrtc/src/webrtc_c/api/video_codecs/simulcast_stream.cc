#include "simulcast_stream.h"

// WebRTC
#include <api/video_codecs/video_codec.h>

extern "C" {
WEBRTC_EXPORT int webrtc_SimulcastStream_width(
    struct webrtc_SimulcastStream* self) {
  auto stream = reinterpret_cast<webrtc::SimulcastStream*>(self);
  return static_cast<int>(stream->width);
}

WEBRTC_EXPORT void webrtc_SimulcastStream_set_width(
    struct webrtc_SimulcastStream* self,
    int width) {
  auto stream = reinterpret_cast<webrtc::SimulcastStream*>(self);
  stream->width = static_cast<decltype(stream->width)>(width);
}

WEBRTC_EXPORT int webrtc_SimulcastStream_height(
    struct webrtc_SimulcastStream* self) {
  auto stream = reinterpret_cast<webrtc::SimulcastStream*>(self);
  return static_cast<int>(stream->height);
}

WEBRTC_EXPORT void webrtc_SimulcastStream_set_height(
    struct webrtc_SimulcastStream* self,
    int height) {
  auto stream = reinterpret_cast<webrtc::SimulcastStream*>(self);
  stream->height = static_cast<decltype(stream->height)>(height);
}

WEBRTC_EXPORT unsigned int webrtc_SimulcastStream_min_bitrate_kbps(
    struct webrtc_SimulcastStream* self) {
  auto stream = reinterpret_cast<webrtc::SimulcastStream*>(self);
  return stream->minBitrate;
}

WEBRTC_EXPORT void webrtc_SimulcastStream_set_min_bitrate_kbps(
    struct webrtc_SimulcastStream* self,
    unsigned int min_bitrate_kbps) {
  auto stream = reinterpret_cast<webrtc::SimulcastStream*>(self);
  stream->minBitrate =
      static_cast<decltype(stream->minBitrate)>(min_bitrate_kbps);
}

WEBRTC_EXPORT unsigned int webrtc_SimulcastStream_target_bitrate_kbps(
    struct webrtc_SimulcastStream* self) {
  auto stream = reinterpret_cast<webrtc::SimulcastStream*>(self);
  return stream->targetBitrate;
}

WEBRTC_EXPORT void webrtc_SimulcastStream_set_target_bitrate_kbps(
    struct webrtc_SimulcastStream* self,
    unsigned int target_bitrate_kbps) {
  auto stream = reinterpret_cast<webrtc::SimulcastStream*>(self);
  stream->targetBitrate =
      static_cast<decltype(stream->targetBitrate)>(target_bitrate_kbps);
}

WEBRTC_EXPORT unsigned int webrtc_SimulcastStream_max_bitrate_kbps(
    struct webrtc_SimulcastStream* self) {
  auto stream = reinterpret_cast<webrtc::SimulcastStream*>(self);
  return stream->maxBitrate;
}

WEBRTC_EXPORT void webrtc_SimulcastStream_set_max_bitrate_kbps(
    struct webrtc_SimulcastStream* self,
    unsigned int max_bitrate_kbps) {
  auto stream = reinterpret_cast<webrtc::SimulcastStream*>(self);
  stream->maxBitrate =
      static_cast<decltype(stream->maxBitrate)>(max_bitrate_kbps);
}
}
