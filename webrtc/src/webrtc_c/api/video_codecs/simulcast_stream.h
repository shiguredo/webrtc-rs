#pragma once

#include <stdint.h>

#include "../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::VideoCodec::simulcastStream / webrtc::SimulcastStream
// -------------------------

struct webrtc_SimulcastStream;

WEBRTC_EXPORT int webrtc_SimulcastStream_width(
    struct webrtc_SimulcastStream* self);
WEBRTC_EXPORT void webrtc_SimulcastStream_set_width(
    struct webrtc_SimulcastStream* self,
    int width);
WEBRTC_EXPORT int webrtc_SimulcastStream_height(
    struct webrtc_SimulcastStream* self);
WEBRTC_EXPORT void webrtc_SimulcastStream_set_height(
    struct webrtc_SimulcastStream* self,
    int height);
WEBRTC_EXPORT unsigned int webrtc_SimulcastStream_min_bitrate_kbps(
    struct webrtc_SimulcastStream* self);
WEBRTC_EXPORT void webrtc_SimulcastStream_set_min_bitrate_kbps(
    struct webrtc_SimulcastStream* self,
    unsigned int min_bitrate_kbps);
WEBRTC_EXPORT unsigned int webrtc_SimulcastStream_target_bitrate_kbps(
    struct webrtc_SimulcastStream* self);
WEBRTC_EXPORT void webrtc_SimulcastStream_set_target_bitrate_kbps(
    struct webrtc_SimulcastStream* self,
    unsigned int target_bitrate_kbps);
WEBRTC_EXPORT unsigned int webrtc_SimulcastStream_max_bitrate_kbps(
    struct webrtc_SimulcastStream* self);
WEBRTC_EXPORT void webrtc_SimulcastStream_set_max_bitrate_kbps(
    struct webrtc_SimulcastStream* self,
    unsigned int max_bitrate_kbps);

#if defined(__cplusplus)
}
#endif
