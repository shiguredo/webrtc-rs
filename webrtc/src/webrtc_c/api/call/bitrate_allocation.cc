#include "bitrate_allocation.h"

#include <limits>

// WebRTC
#include <api/call/bitrate_allocation.h>
#include <api/units/data_rate.h>
#include <api/units/data_size.h>
#include <api/units/time_delta.h>

namespace {

// webrtc::TimeDelta::PlusInfinity() は std::numeric_limits<int64_t>::max() で
// 表現される（unit_base.h の PlusInfinityVal）。有限値はこの値に絶対到達しない
// ため、番兵として扱う。
constexpr int64_t kPlusInfinityUs = std::numeric_limits<int64_t>::max();

}  // namespace

extern "C" {

WEBRTC_EXPORT struct webrtc_BitrateAllocationUpdate*
webrtc_BitrateAllocationUpdate_new() {
  auto update = new webrtc::BitrateAllocationUpdate();
  return reinterpret_cast<struct webrtc_BitrateAllocationUpdate*>(update);
}

WEBRTC_EXPORT void webrtc_BitrateAllocationUpdate_delete(
    struct webrtc_BitrateAllocationUpdate* self) {
  auto update = reinterpret_cast<webrtc::BitrateAllocationUpdate*>(self);
  delete update;
}

WEBRTC_EXPORT struct webrtc_BitrateAllocationUpdate*
webrtc_BitrateAllocationUpdate_copy(
    const struct webrtc_BitrateAllocationUpdate* self) {
  auto update = reinterpret_cast<const webrtc::BitrateAllocationUpdate*>(self);
  auto copied = new webrtc::BitrateAllocationUpdate(*update);
  return reinterpret_cast<struct webrtc_BitrateAllocationUpdate*>(copied);
}

WEBRTC_EXPORT int64_t webrtc_BitrateAllocationUpdate_get_target_bitrate_bps(
    const struct webrtc_BitrateAllocationUpdate* self) {
  auto update = reinterpret_cast<const webrtc::BitrateAllocationUpdate*>(self);
  return update->target_bitrate.bps();
}

WEBRTC_EXPORT void webrtc_BitrateAllocationUpdate_set_target_bitrate_bps(
    struct webrtc_BitrateAllocationUpdate* self,
    int64_t value) {
  auto update = reinterpret_cast<webrtc::BitrateAllocationUpdate*>(self);
  update->target_bitrate = webrtc::DataRate::BitsPerSec(value);
}

WEBRTC_EXPORT double webrtc_BitrateAllocationUpdate_get_packet_loss_ratio(
    const struct webrtc_BitrateAllocationUpdate* self) {
  auto update = reinterpret_cast<const webrtc::BitrateAllocationUpdate*>(self);
  return update->packet_loss_ratio;
}

WEBRTC_EXPORT void webrtc_BitrateAllocationUpdate_set_packet_loss_ratio(
    struct webrtc_BitrateAllocationUpdate* self,
    double value) {
  auto update = reinterpret_cast<webrtc::BitrateAllocationUpdate*>(self);
  update->packet_loss_ratio = value;
}

WEBRTC_EXPORT int64_t webrtc_BitrateAllocationUpdate_get_round_trip_time_us(
    const struct webrtc_BitrateAllocationUpdate* self) {
  auto update = reinterpret_cast<const webrtc::BitrateAllocationUpdate*>(self);
  return update->round_trip_time.IsPlusInfinity()
             ? kPlusInfinityUs
             : update->round_trip_time.us();
}

WEBRTC_EXPORT void webrtc_BitrateAllocationUpdate_set_round_trip_time_us(
    struct webrtc_BitrateAllocationUpdate* self,
    int64_t value) {
  auto update = reinterpret_cast<webrtc::BitrateAllocationUpdate*>(self);
  update->round_trip_time = value == kPlusInfinityUs
                                ? webrtc::TimeDelta::PlusInfinity()
                                : webrtc::TimeDelta::Micros(value);
}

WEBRTC_EXPORT double webrtc_BitrateAllocationUpdate_get_cwnd_reduce_ratio(
    const struct webrtc_BitrateAllocationUpdate* self) {
  auto update = reinterpret_cast<const webrtc::BitrateAllocationUpdate*>(self);
  return update->cwnd_reduce_ratio;
}

WEBRTC_EXPORT void webrtc_BitrateAllocationUpdate_set_cwnd_reduce_ratio(
    struct webrtc_BitrateAllocationUpdate* self,
    double value) {
  auto update = reinterpret_cast<webrtc::BitrateAllocationUpdate*>(self);
  update->cwnd_reduce_ratio = value;
}

WEBRTC_EXPORT int64_t webrtc_BitrateAllocationUpdate_get_packet_overhead_bytes(
    const struct webrtc_BitrateAllocationUpdate* self) {
  auto update = reinterpret_cast<const webrtc::BitrateAllocationUpdate*>(self);
  return update->packet_overhead.bytes();
}

WEBRTC_EXPORT void webrtc_BitrateAllocationUpdate_set_packet_overhead_bytes(
    struct webrtc_BitrateAllocationUpdate* self,
    int64_t value) {
  auto update = reinterpret_cast<webrtc::BitrateAllocationUpdate*>(self);
  update->packet_overhead = webrtc::DataSize::Bytes(value);
}

}  // extern "C"
