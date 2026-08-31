#pragma once

#include <stdint.h>

#include "../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::BitrateAllocationUpdate
// -------------------------

struct webrtc_BitrateAllocationUpdate;
WEBRTC_EXPORT struct webrtc_BitrateAllocationUpdate*
webrtc_BitrateAllocationUpdate_new();
WEBRTC_EXPORT void webrtc_BitrateAllocationUpdate_delete(
    struct webrtc_BitrateAllocationUpdate* self);
WEBRTC_EXPORT struct webrtc_BitrateAllocationUpdate*
webrtc_BitrateAllocationUpdate_copy(
    const struct webrtc_BitrateAllocationUpdate* self);
WEBRTC_EXPORT int64_t webrtc_BitrateAllocationUpdate_get_target_bitrate_bps(
    const struct webrtc_BitrateAllocationUpdate* self);
WEBRTC_EXPORT void webrtc_BitrateAllocationUpdate_set_target_bitrate_bps(
    struct webrtc_BitrateAllocationUpdate* self,
    int64_t value);
WEBRTC_EXPORT double webrtc_BitrateAllocationUpdate_get_packet_loss_ratio(
    const struct webrtc_BitrateAllocationUpdate* self);
WEBRTC_EXPORT void webrtc_BitrateAllocationUpdate_set_packet_loss_ratio(
    struct webrtc_BitrateAllocationUpdate* self,
    double value);
WEBRTC_EXPORT int64_t webrtc_BitrateAllocationUpdate_get_round_trip_time_us(
    const struct webrtc_BitrateAllocationUpdate* self);
WEBRTC_EXPORT void webrtc_BitrateAllocationUpdate_set_round_trip_time_us(
    struct webrtc_BitrateAllocationUpdate* self,
    int64_t value);
WEBRTC_EXPORT double webrtc_BitrateAllocationUpdate_get_cwnd_reduce_ratio(
    const struct webrtc_BitrateAllocationUpdate* self);
WEBRTC_EXPORT void webrtc_BitrateAllocationUpdate_set_cwnd_reduce_ratio(
    struct webrtc_BitrateAllocationUpdate* self,
    double value);
WEBRTC_EXPORT int64_t webrtc_BitrateAllocationUpdate_get_packet_overhead_bytes(
    const struct webrtc_BitrateAllocationUpdate* self);
WEBRTC_EXPORT void webrtc_BitrateAllocationUpdate_set_packet_overhead_bytes(
    struct webrtc_BitrateAllocationUpdate* self,
    int64_t value);

#if defined(__cplusplus)
}
#endif
