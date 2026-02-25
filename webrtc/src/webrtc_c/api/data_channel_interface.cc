#include "data_channel_interface.h"

#include <stddef.h>
#include <stdint.h>
#include <string.h>
#include <memory>
#include <string>

// WebRTC
#include <api/data_channel_interface.h>
#include <rtc_base/copy_on_write_buffer.h>

#include "../common.impl.h"
#include "../std.h"

// -------------------------
// webrtc::DataChannelObserver
// -------------------------

class DataChannelObserverImpl : public webrtc::DataChannelObserver {
 public:
  DataChannelObserverImpl(const struct webrtc_DataChannelObserver_cbs* cbs,
                          void* user_data)
      : user_data_(user_data) {
    if (cbs != nullptr) {
      cbs_ = *cbs;
    }
  }

  ~DataChannelObserverImpl() override {
    if (cbs_.OnDestroy != nullptr) {
      cbs_.OnDestroy(user_data_);
    }
  }

  void OnStateChange() override {
    if (cbs_.OnStateChange != nullptr) {
      cbs_.OnStateChange(user_data_);
    }
  }

  void OnMessage(const webrtc::DataBuffer& buffer) override {
    if (cbs_.OnMessage != nullptr) {
      cbs_.OnMessage(buffer.data.data<uint8_t>(), buffer.data.size(),
                     buffer.binary ? 1 : 0, user_data_);
    }
  }

 private:
  webrtc_DataChannelObserver_cbs cbs_{};
  void* user_data_;
};

struct webrtc_DataChannelObserver* webrtc_DataChannelObserver_new(
    const struct webrtc_DataChannelObserver_cbs* cbs,
    void* user_data) {
  auto impl = new DataChannelObserverImpl(cbs, user_data);
  return reinterpret_cast<struct webrtc_DataChannelObserver*>(impl);
}

void webrtc_DataChannelObserver_delete(
    struct webrtc_DataChannelObserver* self) {
  auto impl = reinterpret_cast<DataChannelObserverImpl*>(self);
  delete impl;
}

// -------------------------
// webrtc::DataChannelInterface
// -------------------------

extern "C" {
WEBRTC_DEFINE_REFCOUNTED(webrtc_DataChannelInterface,
                         webrtc::DataChannelInterface);

extern const int webrtc_DataChannelInterface_DataState_kConnecting =
    static_cast<int>(webrtc::DataChannelInterface::DataState::kConnecting);
extern const int webrtc_DataChannelInterface_DataState_kOpen =
    static_cast<int>(webrtc::DataChannelInterface::DataState::kOpen);
extern const int webrtc_DataChannelInterface_DataState_kClosing =
    static_cast<int>(webrtc::DataChannelInterface::DataState::kClosing);
extern const int webrtc_DataChannelInterface_DataState_kClosed =
    static_cast<int>(webrtc::DataChannelInterface::DataState::kClosed);

struct std_string_unique* webrtc_DataChannelInterface_label(
    struct webrtc_DataChannelInterface* self) {
  auto dc = reinterpret_cast<webrtc::DataChannelInterface*>(self);
  auto label = std::make_unique<std::string>(dc->label());
  return reinterpret_cast<struct std_string_unique*>(label.release());
}

webrtc_DataChannelInterface_DataState webrtc_DataChannelInterface_state(
    struct webrtc_DataChannelInterface* self) {
  auto dc = reinterpret_cast<webrtc::DataChannelInterface*>(self);
  return static_cast<webrtc_DataChannelInterface_DataState>(dc->state());
}

int webrtc_DataChannelInterface_Send(struct webrtc_DataChannelInterface* self,
                                     const uint8_t* data,
                                     size_t len,
                                     int is_binary) {
  auto dc = reinterpret_cast<webrtc::DataChannelInterface*>(self);
  webrtc::CopyOnWriteBuffer buffer(data, len);
  webrtc::DataBuffer data_buffer(buffer, is_binary != 0);
  return dc->Send(data_buffer) ? 1 : 0;
}

void webrtc_DataChannelInterface_Close(
    struct webrtc_DataChannelInterface* self) {
  auto dc = reinterpret_cast<webrtc::DataChannelInterface*>(self);
  dc->Close();
}

void webrtc_DataChannelInterface_RegisterObserver(
    struct webrtc_DataChannelInterface* self,
    struct webrtc_DataChannelObserver* observer) {
  auto dc = reinterpret_cast<webrtc::DataChannelInterface*>(self);
  auto obs = reinterpret_cast<DataChannelObserverImpl*>(observer);
  dc->RegisterObserver(obs);
}

void webrtc_DataChannelInterface_UnregisterObserver(
    struct webrtc_DataChannelInterface* self) {
  auto dc = reinterpret_cast<webrtc::DataChannelInterface*>(self);
  dc->UnregisterObserver();
}

// -------------------------
// webrtc::DataChannelInit
// -------------------------

struct webrtc_DataChannelInit* webrtc_DataChannelInit_new() {
  auto init = new webrtc::DataChannelInit();
  return reinterpret_cast<struct webrtc_DataChannelInit*>(init);
}

void webrtc_DataChannelInit_delete(struct webrtc_DataChannelInit* self) {
  auto init = reinterpret_cast<webrtc::DataChannelInit*>(self);
  delete init;
}

void webrtc_DataChannelInit_set_ordered(struct webrtc_DataChannelInit* self,
                                        int ordered) {
  auto init = reinterpret_cast<webrtc::DataChannelInit*>(self);
  init->ordered = ordered != 0;
}

void webrtc_DataChannelInit_set_protocol(struct webrtc_DataChannelInit* self,
                                         const char* protocol,
                                         size_t protocol_len) {
  auto init = reinterpret_cast<webrtc::DataChannelInit*>(self);
  init->protocol = std::string(protocol, protocol_len);
}
}
