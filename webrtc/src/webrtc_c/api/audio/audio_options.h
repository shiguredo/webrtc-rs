#pragma once

#include "../../common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::AudioOptions
// -------------------------

struct webrtc_AudioOptions;

// webrtc::AudioOptions のインスタンスを生成する。
// 生成したインスタンスは webrtc_AudioOptions_delete() で解放すること。
WEBRTC_EXPORT struct webrtc_AudioOptions* webrtc_AudioOptions_new();
// webrtc_AudioOptions_new() で生成したインスタンスを解放する。
WEBRTC_EXPORT void webrtc_AudioOptions_delete(struct webrtc_AudioOptions* self);

// エコーキャンセルの有効/無効 (std::optional<bool>)。
WEBRTC_EXPORT void webrtc_AudioOptions_get_echo_cancellation(
    struct webrtc_AudioOptions* self,
    int* out_has,
    int* out_value);
WEBRTC_EXPORT void webrtc_AudioOptions_set_echo_cancellation(
    struct webrtc_AudioOptions* self,
    int has,
    const int* value);

// 自動ゲインコントロールの有効/無効 (std::optional<bool>)。
WEBRTC_EXPORT void webrtc_AudioOptions_get_auto_gain_control(
    struct webrtc_AudioOptions* self,
    int* out_has,
    int* out_value);
WEBRTC_EXPORT void webrtc_AudioOptions_set_auto_gain_control(
    struct webrtc_AudioOptions* self,
    int has,
    const int* value);

// ノイズサプレッションの有効/無効 (std::optional<bool>)。
WEBRTC_EXPORT void webrtc_AudioOptions_get_noise_suppression(
    struct webrtc_AudioOptions* self,
    int* out_has,
    int* out_value);
WEBRTC_EXPORT void webrtc_AudioOptions_set_noise_suppression(
    struct webrtc_AudioOptions* self,
    int has,
    const int* value);

// ハイパスフィルタの有効/無効 (std::optional<bool>)。
WEBRTC_EXPORT void webrtc_AudioOptions_get_highpass_filter(
    struct webrtc_AudioOptions* self,
    int* out_has,
    int* out_value);
WEBRTC_EXPORT void webrtc_AudioOptions_set_highpass_filter(
    struct webrtc_AudioOptions* self,
    int has,
    const int* value);

// 左右チャンネルの入れ替えの有効/無効 (std::optional<bool>)。
WEBRTC_EXPORT void webrtc_AudioOptions_get_stereo_swapping(
    struct webrtc_AudioOptions* self,
    int* out_has,
    int* out_value);
WEBRTC_EXPORT void webrtc_AudioOptions_set_stereo_swapping(
    struct webrtc_AudioOptions* self,
    int has,
    const int* value);

// 受信側 jitter buffer (NetEq) の最大パケット数 (std::optional<int>)。
WEBRTC_EXPORT void webrtc_AudioOptions_get_audio_jitter_buffer_max_packets(
    struct webrtc_AudioOptions* self,
    int* out_has,
    int* out_value);
WEBRTC_EXPORT void webrtc_AudioOptions_set_audio_jitter_buffer_max_packets(
    struct webrtc_AudioOptions* self,
    int has,
    const int* value);

// 受信側 jitter buffer (NetEq) の fast accelerate モードの有効/無効 (std::optional<bool>)。
WEBRTC_EXPORT void webrtc_AudioOptions_get_audio_jitter_buffer_fast_accelerate(
    struct webrtc_AudioOptions* self,
    int* out_has,
    int* out_value);
WEBRTC_EXPORT void webrtc_AudioOptions_set_audio_jitter_buffer_fast_accelerate(
    struct webrtc_AudioOptions* self,
    int has,
    const int* value);

// 受信側 jitter buffer (NetEq) の最小ターゲット遅延 (ミリ秒) (std::optional<int>)。
WEBRTC_EXPORT void webrtc_AudioOptions_get_audio_jitter_buffer_min_delay_ms(
    struct webrtc_AudioOptions* self,
    int* out_has,
    int* out_value);
WEBRTC_EXPORT void webrtc_AudioOptions_set_audio_jitter_buffer_min_delay_ms(
    struct webrtc_AudioOptions* self,
    int has,
    const int* value);

#if defined(__cplusplus)
}
#endif
