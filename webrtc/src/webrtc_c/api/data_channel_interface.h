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
extern const int webrtc_DataChannelInterface_DataState_kConnecting;
extern const int webrtc_DataChannelInterface_DataState_kOpen;
extern const int webrtc_DataChannelInterface_DataState_kClosing;
extern const int webrtc_DataChannelInterface_DataState_kClosed;

// DataChannel メソッド
struct std_string_unique* webrtc_DataChannelInterface_label(
    struct webrtc_DataChannelInterface* self);
webrtc_DataChannelInterface_DataState webrtc_DataChannelInterface_state(
    struct webrtc_DataChannelInterface* self);
int webrtc_DataChannelInterface_Send(struct webrtc_DataChannelInterface* self,
                                     const uint8_t* data,
                                     size_t len,
                                     int is_binary);
void webrtc_DataChannelInterface_Close(
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
};

struct webrtc_DataChannelObserver* webrtc_DataChannelObserver_new(
    struct webrtc_DataChannelObserver_cbs* cbs,
    void* user_data);
void webrtc_DataChannelObserver_delete(struct webrtc_DataChannelObserver* self);

void webrtc_DataChannelInterface_RegisterObserver(
    struct webrtc_DataChannelInterface* self,
    struct webrtc_DataChannelObserver* observer);
void webrtc_DataChannelInterface_UnregisterObserver(
    struct webrtc_DataChannelInterface* self);

// -------------------------
// webrtc::DataChannelInit
// -------------------------

struct webrtc_DataChannelInit;
struct webrtc_DataChannelInit* webrtc_DataChannelInit_new();
void webrtc_DataChannelInit_delete(struct webrtc_DataChannelInit* self);
void webrtc_DataChannelInit_set_ordered(struct webrtc_DataChannelInit* self,
                                        int ordered);
void webrtc_DataChannelInit_set_protocol(struct webrtc_DataChannelInit* self,
                                         const char* protocol,
                                         size_t protocol_len);

#if defined(__cplusplus)
}
#endif
