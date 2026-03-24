#pragma once

#include <stddef.h>
#include <stdint.h>

#include "../common.h"
#include "../std.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::DataChannelInterface
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_DataChannelInterface);

// DataState 定数
typedef int webrtc_DataChannelInterface_DataState;
WEBRTC_EXPORT extern const int
    webrtc_DataChannelInterface_DataState_kConnecting;
WEBRTC_EXPORT extern const int webrtc_DataChannelInterface_DataState_kOpen;
WEBRTC_EXPORT extern const int webrtc_DataChannelInterface_DataState_kClosing;
WEBRTC_EXPORT extern const int webrtc_DataChannelInterface_DataState_kClosed;

// DataChannel メソッド
WEBRTC_EXPORT struct std_string_unique* webrtc_DataChannelInterface_label(
    struct webrtc_DataChannelInterface* self);
WEBRTC_EXPORT webrtc_DataChannelInterface_DataState
webrtc_DataChannelInterface_state(struct webrtc_DataChannelInterface* self);
WEBRTC_EXPORT int webrtc_DataChannelInterface_Send(
    struct webrtc_DataChannelInterface* self,
    const uint8_t* data,
    size_t len,
    int is_binary);
WEBRTC_EXPORT void webrtc_DataChannelInterface_Close(
    struct webrtc_DataChannelInterface* self);

// -------------------------
// webrtc::DataChannelObserver
// -------------------------

struct webrtc_DataChannelObserver;

struct webrtc_DataChannelObserver_cbs {
  void (*OnStateChange)(void* user_data);
  void (*OnMessage)(const uint8_t* data,
                    size_t len,
                    int is_binary,
                    void* user_data);
  void (*OnDestroy)(void* user_data);
};

WEBRTC_EXPORT struct webrtc_DataChannelObserver* webrtc_DataChannelObserver_new(
    const struct webrtc_DataChannelObserver_cbs* cbs,
    void* user_data);
WEBRTC_EXPORT void webrtc_DataChannelObserver_delete(
    struct webrtc_DataChannelObserver* self);

WEBRTC_EXPORT void webrtc_DataChannelInterface_RegisterObserver(
    struct webrtc_DataChannelInterface* self,
    struct webrtc_DataChannelObserver* observer);
WEBRTC_EXPORT void webrtc_DataChannelInterface_UnregisterObserver(
    struct webrtc_DataChannelInterface* self);

// -------------------------
// webrtc::DataChannelInit
// -------------------------

struct webrtc_DataChannelInit;
WEBRTC_EXPORT struct webrtc_DataChannelInit* webrtc_DataChannelInit_new();
WEBRTC_EXPORT void webrtc_DataChannelInit_delete(
    struct webrtc_DataChannelInit* self);
WEBRTC_EXPORT void webrtc_DataChannelInit_set_ordered(
    struct webrtc_DataChannelInit* self,
    int ordered);
WEBRTC_EXPORT void webrtc_DataChannelInit_set_protocol(
    struct webrtc_DataChannelInit* self,
    const char* protocol,
    size_t protocol_len);

#if defined(__cplusplus)
}
#endif
