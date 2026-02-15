#include "peer_connection_interface.h"

#include <stdarg.h>
#include <stddef.h>
#include <memory>
#include <string>
#include <utility>
#include <vector>

// WebRTC
#include <api/audio/audio_device.h>
#include <api/audio/audio_processing.h>
#include <api/audio_codecs/audio_decoder_factory.h>
#include <api/audio_codecs/audio_encoder_factory.h>
#include <api/audio_options.h>
#include <api/create_modular_peer_connection_factory.h>
#include <api/data_channel_interface.h>
#include <api/enable_media.h>
#include <api/jsep.h>
#include <api/make_ref_counted.h>
#include <api/media_stream_interface.h>
#include <api/media_types.h>
#include <api/peer_connection_interface.h>
#include <api/rtc_error.h>
#include <api/rtc_event_log/rtc_event_log_factory_interface.h>
#include <api/rtp_parameters.h>
#include <api/rtp_receiver_interface.h>
#include <api/rtp_transceiver_interface.h>
#include <api/scoped_refptr.h>
#include <api/set_local_description_observer_interface.h>
#include <api/set_remote_description_observer_interface.h>
#include <api/stats/rtc_stats_collector_callback.h>
#include <api/stats/rtc_stats_report.h>
#include <api/video_codecs/video_decoder_factory.h>
#include <api/video_codecs/video_encoder_factory.h>
#include <pc/media_factory.h>
#include <rtc_base/ssl_stream_adapter.h>
#include <rtc_base/thread.h>

#include "../common.impl.h"
#include "../rtc_base/thread.h"
#include "../std.h"
#include "audio/audio_processing.h"
#include "audio_codecs/audio_decoder_factory.h"
#include "audio_codecs/audio_encoder_factory.h"
#include "data_channel_interface.h"
#include "jsep.h"
#include "media_stream_interface.h"
#include "rtc_error.h"
#include "rtc_event_log.h"
#include "rtp_parameters.h"
#include "rtp_receiver_interface.h"
#include "rtp_sender_interface.h"
#include "rtp_transceiver_interface.h"
#include "set_local_description_observer_interface.h"
#include "set_remote_description_observer_interface.h"
#include "stats/rtc_stats_collector_callback.h"
#include "stats/rtc_stats_report.h"
#include "video_codecs/video_decoder_factory.h"
#include "video_codecs/video_encoder_factory.h"

// -------------------------
// webrtc::PeerConnectionObserver
// -------------------------

class PeerConnectionObserverImpl : public webrtc::PeerConnectionObserver {
 public:
  explicit PeerConnectionObserverImpl(
      struct webrtc_PeerConnectionObserver_cbs* observer,
      void* user_data)
      : observer_(observer), user_data_(user_data) {}

  void OnSignalingChange(
      webrtc::PeerConnectionInterface::SignalingState new_state) override {}
  void OnDataChannel(webrtc::scoped_refptr<webrtc::DataChannelInterface>
                         data_channel) override {
    if (observer_->OnDataChannel) {
      webrtc::scoped_refptr<webrtc::DataChannelInterface> data_channel_ref(
          data_channel);
      observer_->OnDataChannel(
          reinterpret_cast<struct webrtc_DataChannelInterface_refcounted*>(
              data_channel_ref.release()),
          user_data_);
    }
  }
  void OnStandardizedIceConnectionChange(
      webrtc::PeerConnectionInterface::IceConnectionState new_state) override {}
  void OnConnectionChange(
      webrtc::PeerConnectionInterface::PeerConnectionState new_state) override {
    if (observer_->OnConnectionChange) {
      observer_->OnConnectionChange(
          static_cast<webrtc_PeerConnectionInterface_PeerConnectionState>(
              new_state),
          user_data_);
    }
  }
  void OnIceGatheringChange(
      webrtc::PeerConnectionInterface::IceGatheringState new_state) override {}
  void OnIceCandidate(const webrtc::IceCandidateInterface* candidate) override {
    if (observer_->OnIceCandidate) {
      observer_->OnIceCandidate(
          reinterpret_cast<const struct webrtc_IceCandidateInterface*>(
              candidate),
          user_data_);
    }
  }
  void OnIceCandidateError(const std::string& address,
                           int port,
                           const std::string& url,
                           int error_code,
                           const std::string& error_text) override {}
  void OnTrack(webrtc::scoped_refptr<webrtc::RtpTransceiverInterface>
                   transceiver) override {
    if (observer_->OnTrack) {
      webrtc::scoped_refptr<webrtc::RtpTransceiverInterface> transceiver_ref(
          transceiver);
      observer_->OnTrack(
          reinterpret_cast<struct webrtc_RtpTransceiverInterface_refcounted*>(
              transceiver_ref.release()),
          user_data_);
    }
  }
  void OnRemoveTrack(
      webrtc::scoped_refptr<webrtc::RtpReceiverInterface> receiver) override {
    if (observer_->OnRemoveTrack) {
      webrtc::scoped_refptr<webrtc::RtpReceiverInterface> receiver_ref(
          receiver);
      observer_->OnRemoveTrack(
          reinterpret_cast<struct webrtc_RtpReceiverInterface_refcounted*>(
              receiver_ref.release()),
          user_data_);
    }
  }

 private:
  struct webrtc_PeerConnectionObserver_cbs* observer_;
  void* user_data_;
};

class RTCStatsCollectorCallbackImpl : public webrtc::RTCStatsCollectorCallback {
 public:
  RTCStatsCollectorCallbackImpl(
      struct webrtc_RTCStatsCollectorCallback_cbs* cbs,
      void* user_data)
      : cbs_(cbs), user_data_(user_data) {}

  void OnStatsDelivered(
      const webrtc::scoped_refptr<const webrtc::RTCStatsReport>& report)
      override {
    if (cbs_->OnStatsDelivered == nullptr) {
      return;
    }
    webrtc::scoped_refptr<const webrtc::RTCStatsReport> report_ref(report);
    cbs_->OnStatsDelivered(
        reinterpret_cast<const struct webrtc_RTCStatsReport_refcounted*>(
            report_ref.release()),
        user_data_);
  }

 private:
  struct webrtc_RTCStatsCollectorCallback_cbs* cbs_;
  void* user_data_;
};

struct webrtc_PeerConnectionObserver* webrtc_PeerConnectionObserver_new(
    struct webrtc_PeerConnectionObserver_cbs* observer,
    void* user_data) {
  auto impl = new PeerConnectionObserverImpl(observer, user_data);
  return reinterpret_cast<struct webrtc_PeerConnectionObserver*>(impl);
}
void webrtc_PeerConnectionObserver_delete(
    struct webrtc_PeerConnectionObserver* self) {
  auto impl = reinterpret_cast<PeerConnectionObserverImpl*>(self);
  delete impl;
}

// -------------------------
// webrtc::PeerConnectionInterface
// -------------------------

extern "C" {
WEBRTC_DEFINE_REFCOUNTED(webrtc_PeerConnectionInterface,
                         webrtc::PeerConnectionInterface);

struct webrtc_PeerConnectionInterface_RTCConfiguration*
webrtc_PeerConnectionInterface_RTCConfiguration_new() {
  auto config = new webrtc::PeerConnectionInterface::RTCConfiguration();
  return reinterpret_cast<
      struct webrtc_PeerConnectionInterface_RTCConfiguration*>(config);
}
void webrtc_PeerConnectionInterface_RTCConfiguration_delete(
    struct webrtc_PeerConnectionInterface_RTCConfiguration* self) {
  auto config =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCConfiguration*>(
          self);
  delete config;
}
struct webrtc_PeerConnectionInterface_IceServer*
webrtc_PeerConnectionInterface_IceServer_new() {
  auto server = new webrtc::PeerConnectionInterface::IceServer();
  return reinterpret_cast<struct webrtc_PeerConnectionInterface_IceServer*>(
      server);
}
void webrtc_PeerConnectionInterface_IceServer_delete(
    struct webrtc_PeerConnectionInterface_IceServer* self) {
  auto server =
      reinterpret_cast<webrtc::PeerConnectionInterface::IceServer*>(self);
  delete server;
}
WEBRTC_DEFINE_VECTOR(webrtc_PeerConnectionInterface_IceServer,
                     webrtc::PeerConnectionInterface::IceServer);
struct std_string_vector* webrtc_PeerConnectionInterface_IceServer_get_urls(
    struct webrtc_PeerConnectionInterface_IceServer* self) {
  auto server =
      reinterpret_cast<webrtc::PeerConnectionInterface::IceServer*>(self);
  return reinterpret_cast<struct std_string_vector*>(&server->urls);
}
void webrtc_PeerConnectionInterface_IceServer_set_username(
    struct webrtc_PeerConnectionInterface_IceServer* self,
    const char* username,
    size_t username_len) {
  auto server =
      reinterpret_cast<webrtc::PeerConnectionInterface::IceServer*>(self);
  server->username =
      username != nullptr ? std::string(username, username_len) : std::string();
}
void webrtc_PeerConnectionInterface_IceServer_set_password(
    struct webrtc_PeerConnectionInterface_IceServer* self,
    const char* password,
    size_t password_len) {
  auto server =
      reinterpret_cast<webrtc::PeerConnectionInterface::IceServer*>(self);
  server->password =
      password != nullptr ? std::string(password, password_len) : std::string();
}
struct webrtc_PeerConnectionInterface_IceServer_vector*
webrtc_PeerConnectionInterface_RTCConfiguration_get_servers(
    struct webrtc_PeerConnectionInterface_RTCConfiguration* self) {
  auto config =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCConfiguration*>(
          self);
  return reinterpret_cast<
      struct webrtc_PeerConnectionInterface_IceServer_vector*>(
      &config->servers);
}
extern const int webrtc_PeerConnectionInterface_IceTransportsType_kRelay =
    static_cast<int>(
        webrtc::PeerConnectionInterface::IceTransportsType::kRelay);
void webrtc_PeerConnectionInterface_RTCConfiguration_set_type(
    struct webrtc_PeerConnectionInterface_RTCConfiguration* self,
    webrtc_PeerConnectionInterface_IceTransportsType type) {
  auto config =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCConfiguration*>(
          self);
  config->type =
      static_cast<webrtc::PeerConnectionInterface::IceTransportsType>(type);
}
struct webrtc_PeerConnectionDependencies* webrtc_PeerConnectionDependencies_new(
    struct webrtc_PeerConnectionObserver* observer) {
  auto pc_observer = reinterpret_cast<PeerConnectionObserverImpl*>(observer);
  auto deps = new webrtc::PeerConnectionDependencies(pc_observer);
  return reinterpret_cast<webrtc_PeerConnectionDependencies*>(deps);
}
void webrtc_PeerConnectionDependencies_delete(
    struct webrtc_PeerConnectionDependencies* self) {
  auto deps = reinterpret_cast<webrtc::PeerConnectionDependencies*>(self);
  delete deps;
}

void webrtc_PeerConnectionInterface_CreateDataChannelOrError(
    struct webrtc_PeerConnectionInterface* self,
    const char* label,
    size_t label_len,
    struct webrtc_DataChannelInit* init,
    struct webrtc_DataChannelInterface_refcounted** out_data_channel,
    struct webrtc_RTCError_unique** out_rtc_error) {
  auto pc = reinterpret_cast<webrtc::PeerConnectionInterface*>(self);
  auto label_str = std::string(label, label_len);
  auto dc_init = reinterpret_cast<webrtc::DataChannelInit*>(init);
  auto r = pc->CreateDataChannelOrError(label_str, dc_init);
  if (r.ok()) {
    webrtc::scoped_refptr<webrtc::DataChannelInterface> dc_ref(r.MoveValue());
    *out_data_channel =
        reinterpret_cast<struct webrtc_DataChannelInterface_refcounted*>(
            dc_ref.release());
    *out_rtc_error = nullptr;
  } else {
    *out_data_channel = nullptr;
    auto rtc_error = new webrtc::RTCError(r.error());
    *out_rtc_error =
        reinterpret_cast<struct webrtc_RTCError_unique*>(rtc_error);
  }
}

void webrtc_PeerConnectionInterface_AddTransceiver(
    struct webrtc_PeerConnectionInterface* self,
    int media_type,
    struct webrtc_RtpTransceiverInit* init,
    struct webrtc_RtpTransceiverInterface_refcounted** out_transceiver,
    struct webrtc_RTCError_unique** out_rtc_error) {
  auto pc = reinterpret_cast<webrtc::PeerConnectionInterface*>(self);
  auto r =
      pc->AddTransceiver(static_cast<webrtc::MediaType>(media_type),
                         *reinterpret_cast<webrtc::RtpTransceiverInit*>(init));
  if (r.ok()) {
    *out_transceiver =
        reinterpret_cast<struct webrtc_RtpTransceiverInterface_refcounted*>(
            r.MoveValue().release());
    *out_rtc_error = nullptr;
  } else {
    *out_transceiver = nullptr;
    auto rtc_error = new webrtc::RTCError(r.error());
    *out_rtc_error =
        reinterpret_cast<struct webrtc_RTCError_unique*>(rtc_error);
  }
}
void webrtc_PeerConnectionInterface_AddTransceiverWithTrack(
    struct webrtc_PeerConnectionInterface* self,
    struct webrtc_VideoTrackInterface_refcounted* track,
    struct webrtc_RtpTransceiverInit* init,
    struct webrtc_RtpTransceiverInterface_refcounted** out_transceiver,
    struct webrtc_RTCError_unique** out_rtc_error) {
  auto pc = reinterpret_cast<webrtc::PeerConnectionInterface*>(self);
  auto media_track = reinterpret_cast<webrtc::VideoTrackInterface*>(
      webrtc_VideoTrackInterface_refcounted_get(track));
  webrtc::scoped_refptr<webrtc::MediaStreamTrackInterface> track_ref(
      media_track);
  auto r = pc->AddTransceiver(
      track_ref, *reinterpret_cast<webrtc::RtpTransceiverInit*>(init));
  if (r.ok()) {
    *out_transceiver =
        reinterpret_cast<struct webrtc_RtpTransceiverInterface_refcounted*>(
            r.MoveValue().release());
    *out_rtc_error = nullptr;
  } else {
    *out_transceiver = nullptr;
    auto rtc_error = new webrtc::RTCError(r.error());
    *out_rtc_error =
        reinterpret_cast<struct webrtc_RTCError_unique*>(rtc_error);
  }
}
void webrtc_PeerConnectionInterface_AddTrack(
    struct webrtc_PeerConnectionInterface* self,
    struct webrtc_MediaStreamTrackInterface_refcounted* track,
    struct std_string_vector* stream_ids,
    struct webrtc_RtpSenderInterface_refcounted** out_sender,
    struct webrtc_RTCError_unique** out_rtc_error) {
  auto pc = reinterpret_cast<webrtc::PeerConnectionInterface*>(self);
  auto raw_track = webrtc_MediaStreamTrackInterface_refcounted_get(track);
  auto media_track =
      reinterpret_cast<webrtc::MediaStreamTrackInterface*>(raw_track);
  webrtc::scoped_refptr<webrtc::MediaStreamTrackInterface> track_ref(
      media_track);
  auto ids = reinterpret_cast<std::vector<std::string>*>(stream_ids);
  auto r = pc->AddTrack(track_ref, *ids);
  if (r.ok()) {
    *out_sender =
        reinterpret_cast<struct webrtc_RtpSenderInterface_refcounted*>(
            r.MoveValue().release());
    *out_rtc_error = nullptr;
  } else {
    *out_sender = nullptr;
    auto rtc_error = new webrtc::RTCError(r.error());
    *out_rtc_error =
        reinterpret_cast<struct webrtc_RTCError_unique*>(rtc_error);
  }
}
void webrtc_PeerConnectionInterface_CreateOffer(
    struct webrtc_PeerConnectionInterface* self,
    struct webrtc_CreateSessionDescriptionObserver* observer,
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* options) {
  auto pc = reinterpret_cast<webrtc::PeerConnectionInterface*>(self);
  auto obs =
      reinterpret_cast<webrtc::CreateSessionDescriptionObserver*>(observer);
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          options);
  if (opts == nullptr) {
    webrtc::PeerConnectionInterface::RTCOfferAnswerOptions empty;
    pc->CreateOffer(obs, empty);
    return;
  }
  pc->CreateOffer(obs, *opts);
}
void webrtc_PeerConnectionInterface_CreateAnswer(
    struct webrtc_PeerConnectionInterface* self,
    struct webrtc_CreateSessionDescriptionObserver* observer,
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* options) {
  auto pc = reinterpret_cast<webrtc::PeerConnectionInterface*>(self);
  auto obs =
      reinterpret_cast<webrtc::CreateSessionDescriptionObserver*>(observer);
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          options);
  if (opts == nullptr) {
    webrtc::PeerConnectionInterface::RTCOfferAnswerOptions empty;
    pc->CreateAnswer(obs, empty);
    return;
  }
  pc->CreateAnswer(obs, *opts);
}
void webrtc_PeerConnectionInterface_SetLocalDescription(
    struct webrtc_PeerConnectionInterface* self,
    struct webrtc_SessionDescriptionInterface_unique* desc,
    struct webrtc_SetLocalDescriptionObserverInterface_refcounted* observer) {
  auto pc = reinterpret_cast<webrtc::PeerConnectionInterface*>(self);
  std::unique_ptr<webrtc::SessionDescriptionInterface> cpp_desc;
  if (desc != nullptr) {
    auto raw_desc = reinterpret_cast<webrtc::SessionDescriptionInterface*>(
        webrtc_SessionDescriptionInterface_unique_get(desc));
    cpp_desc.reset(raw_desc);
  }
  webrtc::scoped_refptr<webrtc::SetLocalDescriptionObserverInterface> obs_ref;
  if (observer != nullptr) {
    auto obs = reinterpret_cast<webrtc::SetLocalDescriptionObserverInterface*>(
        webrtc_SetLocalDescriptionObserverInterface_refcounted_get(observer));
    obs_ref = obs;
  }
  pc->SetLocalDescription(std::move(cpp_desc), obs_ref);
}
void webrtc_PeerConnectionInterface_SetRemoteDescription(
    struct webrtc_PeerConnectionInterface* self,
    struct webrtc_SessionDescriptionInterface_unique* desc,
    struct webrtc_SetRemoteDescriptionObserverInterface_refcounted* observer) {
  auto pc = reinterpret_cast<webrtc::PeerConnectionInterface*>(self);
  std::unique_ptr<webrtc::SessionDescriptionInterface> cpp_desc;
  if (desc != nullptr) {
    auto raw_desc = reinterpret_cast<webrtc::SessionDescriptionInterface*>(
        webrtc_SessionDescriptionInterface_unique_get(desc));
    cpp_desc.reset(raw_desc);
  }
  webrtc::scoped_refptr<webrtc::SetRemoteDescriptionObserverInterface> obs_ref;
  if (observer != nullptr) {
    auto obs = reinterpret_cast<webrtc::SetRemoteDescriptionObserverInterface*>(
        webrtc_SetRemoteDescriptionObserverInterface_refcounted_get(observer));
    obs_ref = obs;
  }
  pc->SetRemoteDescription(std::move(cpp_desc), obs_ref);
}
int webrtc_PeerConnectionInterface_AddIceCandidate(
    struct webrtc_PeerConnectionInterface* self,
    const struct webrtc_IceCandidateInterface* candidate) {
  auto pc = reinterpret_cast<webrtc::PeerConnectionInterface*>(self);
  auto ice =
      reinterpret_cast<const webrtc::IceCandidateInterface*>(candidate);
  return pc->AddIceCandidate(ice) ? 1 : 0;
}
void webrtc_PeerConnectionInterface_SetConfiguration(
    struct webrtc_PeerConnectionInterface* self,
    struct webrtc_PeerConnectionInterface_RTCConfiguration* config,
    struct webrtc_RTCError_unique** out_rtc_error) {
  auto pc = reinterpret_cast<webrtc::PeerConnectionInterface*>(self);
  auto cfg =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCConfiguration*>(
          config);
  auto result = pc->SetConfiguration(*cfg);
  if (result.ok()) {
    *out_rtc_error = nullptr;
  } else {
    auto rtc_error = new webrtc::RTCError(result);
    *out_rtc_error =
        reinterpret_cast<struct webrtc_RTCError_unique*>(rtc_error);
  }
}

void webrtc_PeerConnectionInterface_GetStats(
    struct webrtc_PeerConnectionInterface* self,
    struct webrtc_RTCStatsCollectorCallback_cbs* cbs,
    void* user_data) {
  if (self == nullptr || cbs == nullptr) {
    return;
  }
  auto pc = reinterpret_cast<webrtc::PeerConnectionInterface*>(self);
  auto callback =
      webrtc::make_ref_counted<RTCStatsCollectorCallbackImpl>(cbs, user_data);
  pc->GetStats(callback.get());
}

struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions*
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_new() {
  auto opts = new webrtc::PeerConnectionInterface::RTCOfferAnswerOptions();
  return reinterpret_cast<
      struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions*>(opts);
}
void webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_delete(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  delete opts;
}
int webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_offer_to_receive_video(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  return opts->offer_to_receive_video;
}
void webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_offer_to_receive_video(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self,
    int offer_to_receive_video) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  opts->offer_to_receive_video = offer_to_receive_video;
}
int webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_offer_to_receive_audio(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  return opts->offer_to_receive_audio;
}
void webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_offer_to_receive_audio(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self,
    int offer_to_receive_audio) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  opts->offer_to_receive_audio = offer_to_receive_audio;
}
int webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_voice_activity_detection(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  return opts->voice_activity_detection ? 1 : 0;
}
void webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_voice_activity_detection(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self,
    int voice_activity_detection) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  opts->voice_activity_detection = voice_activity_detection != 0;
}
int webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_ice_restart(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  return opts->ice_restart ? 1 : 0;
}
void webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_ice_restart(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self,
    int ice_restart) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  opts->ice_restart = ice_restart != 0;
}
int webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_use_rtp_mux(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  return opts->use_rtp_mux ? 1 : 0;
}
void webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_use_rtp_mux(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self,
    int use_rtp_mux) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  opts->use_rtp_mux = use_rtp_mux != 0;
}
int webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_raw_packetization_for_video(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  return opts->raw_packetization_for_video ? 1 : 0;
}
void webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_raw_packetization_for_video(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self,
    int raw_packetization_for_video) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  opts->raw_packetization_for_video = raw_packetization_for_video != 0;
}
int webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_num_simulcast_layers(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  return opts->num_simulcast_layers;
}
void webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_num_simulcast_layers(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self,
    int num_simulcast_layers) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  opts->num_simulcast_layers = num_simulcast_layers;
}
int webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_use_obsolete_sctp_sdp(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  return opts->use_obsolete_sctp_sdp ? 1 : 0;
}
void webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_use_obsolete_sctp_sdp(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self,
    int use_obsolete_sctp_sdp) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  opts->use_obsolete_sctp_sdp = use_obsolete_sctp_sdp != 0;
}

extern const int webrtc_PeerConnectionInterface_PeerConnectionState_kNew =
    (int)webrtc::PeerConnectionInterface::PeerConnectionState::kNew;
extern const int
    webrtc_PeerConnectionInterface_PeerConnectionState_kConnecting =
        (int)webrtc::PeerConnectionInterface::PeerConnectionState::kConnecting;
extern const int webrtc_PeerConnectionInterface_PeerConnectionState_kConnected =
    (int)webrtc::PeerConnectionInterface::PeerConnectionState::kConnected;
extern const int
    webrtc_PeerConnectionInterface_PeerConnectionState_kDisconnected = (int)
        webrtc::PeerConnectionInterface::PeerConnectionState::kDisconnected;
extern const int webrtc_PeerConnectionInterface_PeerConnectionState_kFailed =
    (int)webrtc::PeerConnectionInterface::PeerConnectionState::kFailed;
extern const int webrtc_PeerConnectionInterface_PeerConnectionState_kClosed =
    (int)webrtc::PeerConnectionInterface::PeerConnectionState::kClosed;
}

// -------------------------
// webrtc::PeerConnectionFactoryDependencies
// -------------------------

extern "C" {
struct webrtc_PeerConnectionFactoryDependencies*
webrtc_PeerConnectionFactoryDependencies_new() {
  auto deps = new webrtc::PeerConnectionFactoryDependencies();
  return reinterpret_cast<struct webrtc_PeerConnectionFactoryDependencies*>(
      deps);
}
void webrtc_PeerConnectionFactoryDependencies_delete(
    struct webrtc_PeerConnectionFactoryDependencies* self) {
  auto deps =
      reinterpret_cast<webrtc::PeerConnectionFactoryDependencies*>(self);
  delete deps;
}
void webrtc_PeerConnectionFactoryDependencies_set_network_thread(
    struct webrtc_PeerConnectionFactoryDependencies* self,
    struct webrtc_Thread* network_thread) {
  auto deps =
      reinterpret_cast<webrtc::PeerConnectionFactoryDependencies*>(self);
  auto thread = reinterpret_cast<webrtc::Thread*>(network_thread);
  deps->network_thread = thread;
}
void webrtc_PeerConnectionFactoryDependencies_set_worker_thread(
    struct webrtc_PeerConnectionFactoryDependencies* self,
    struct webrtc_Thread* worker_thread) {
  auto deps =
      reinterpret_cast<webrtc::PeerConnectionFactoryDependencies*>(self);
  auto thread = reinterpret_cast<webrtc::Thread*>(worker_thread);
  deps->worker_thread = thread;
}
void webrtc_PeerConnectionFactoryDependencies_set_signaling_thread(
    struct webrtc_PeerConnectionFactoryDependencies* self,
    struct webrtc_Thread* signaling_thread) {
  auto deps =
      reinterpret_cast<webrtc::PeerConnectionFactoryDependencies*>(self);
  auto thread = reinterpret_cast<webrtc::Thread*>(signaling_thread);
  deps->signaling_thread = thread;
}
void webrtc_PeerConnectionFactoryDependencies_set_adm(
    struct webrtc_PeerConnectionFactoryDependencies* self,
    struct webrtc_AudioDeviceModule_refcounted* adm) {
  auto deps =
      reinterpret_cast<webrtc::PeerConnectionFactoryDependencies*>(self);
  auto audio_device_module = reinterpret_cast<webrtc::AudioDeviceModule*>(adm);
  deps->adm = audio_device_module;
}
void webrtc_PeerConnectionFactoryDependencies_set_event_log_factory(
    struct webrtc_PeerConnectionFactoryDependencies* self,
    struct webrtc_RtcEventLogFactory_unique* event_log_factory) {
  auto deps =
      reinterpret_cast<webrtc::PeerConnectionFactoryDependencies*>(self);
  auto factory = reinterpret_cast<webrtc::RtcEventLogFactoryInterface*>(
      webrtc_RtcEventLogFactory_unique_get(event_log_factory));
  deps->event_log_factory =
      std::move(std::unique_ptr<webrtc::RtcEventLogFactoryInterface>(factory));
}
void webrtc_PeerConnectionFactoryDependencies_set_audio_encoder_factory(
    struct webrtc_PeerConnectionFactoryDependencies* self,
    struct webrtc_AudioEncoderFactory_refcounted* audio_encoder_factory) {
  auto deps =
      reinterpret_cast<webrtc::PeerConnectionFactoryDependencies*>(self);
  auto factory =
      reinterpret_cast<webrtc::AudioEncoderFactory*>(audio_encoder_factory);
  deps->audio_encoder_factory = factory;
}
void webrtc_PeerConnectionFactoryDependencies_set_audio_decoder_factory(
    struct webrtc_PeerConnectionFactoryDependencies* self,
    struct webrtc_AudioDecoderFactory_refcounted* audio_decoder_factory) {
  auto deps =
      reinterpret_cast<webrtc::PeerConnectionFactoryDependencies*>(self);
  auto factory =
      reinterpret_cast<webrtc::AudioDecoderFactory*>(audio_decoder_factory);
  deps->audio_decoder_factory = factory;
}
void webrtc_PeerConnectionFactoryDependencies_set_audio_processing_builder(
    struct webrtc_PeerConnectionFactoryDependencies* self,
    struct webrtc_AudioProcessingBuilderInterface_unique*
        audio_processing_builder) {
  auto deps =
      reinterpret_cast<webrtc::PeerConnectionFactoryDependencies*>(self);
  auto builder = reinterpret_cast<webrtc::AudioProcessingBuilderInterface*>(
      webrtc_AudioProcessingBuilderInterface_unique_get(
          audio_processing_builder));
  deps->audio_processing_builder = std::move(
      std::unique_ptr<webrtc::AudioProcessingBuilderInterface>(builder));
}
void webrtc_PeerConnectionFactoryDependencies_set_video_encoder_factory(
    struct webrtc_PeerConnectionFactoryDependencies* self,
    struct webrtc_VideoEncoderFactory_unique* video_encoder_factory) {
  auto deps =
      reinterpret_cast<webrtc::PeerConnectionFactoryDependencies*>(self);
  auto factory = reinterpret_cast<webrtc::VideoEncoderFactory*>(
      webrtc_VideoEncoderFactory_unique_get(video_encoder_factory));
  deps->video_encoder_factory =
      std::move(std::unique_ptr<webrtc::VideoEncoderFactory>(factory));
}
void webrtc_PeerConnectionFactoryDependencies_set_video_decoder_factory(
    struct webrtc_PeerConnectionFactoryDependencies* self,
    struct webrtc_VideoDecoderFactory_unique* video_decoder_factory) {
  auto deps =
      reinterpret_cast<webrtc::PeerConnectionFactoryDependencies*>(self);
  auto factory = reinterpret_cast<webrtc::VideoDecoderFactory*>(
      webrtc_VideoDecoderFactory_unique_get(video_decoder_factory));
  deps->video_decoder_factory =
      std::move(std::unique_ptr<webrtc::VideoDecoderFactory>(factory));
}

void webrtc_EnableMedia(
    struct webrtc_PeerConnectionFactoryDependencies* dependencies) {
  auto deps = reinterpret_cast<webrtc::PeerConnectionFactoryDependencies*>(
      dependencies);
  webrtc::EnableMedia(*deps);
}
}

// -------------------------
// webrtc::PeerConnectionFactoryInterface
// -------------------------

extern "C" {
WEBRTC_DEFINE_REFCOUNTED(webrtc_PeerConnectionFactoryInterface,
                         webrtc::PeerConnectionFactoryInterface);

struct webrtc_PeerConnectionFactoryInterface_refcounted*
webrtc_CreateModularPeerConnectionFactory(
    struct webrtc_PeerConnectionFactoryDependencies* dependencies) {
  auto deps = reinterpret_cast<webrtc::PeerConnectionFactoryDependencies*>(
      dependencies);
  auto factory = webrtc::CreateModularPeerConnectionFactory(std::move(*deps));
  return reinterpret_cast<
      struct webrtc_PeerConnectionFactoryInterface_refcounted*>(
      factory.release());
}

void webrtc_PeerConnectionFactoryInterface_CreatePeerConnectionOrError(
    struct webrtc_PeerConnectionFactoryInterface* self,
    struct webrtc_PeerConnectionInterface_RTCConfiguration* rtc_config,
    struct webrtc_PeerConnectionDependencies* dependencies,
    struct webrtc_PeerConnectionInterface_refcounted** out_pc,
    struct webrtc_RTCError_unique** out_rtc_error) {
  auto factory =
      reinterpret_cast<webrtc::PeerConnectionFactoryInterface*>(self);
  auto config =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCConfiguration*>(
          rtc_config);
  auto deps =
      reinterpret_cast<webrtc::PeerConnectionDependencies*>(dependencies);
  auto result = factory->CreatePeerConnectionOrError(*config, std::move(*deps));
  if (result.ok()) {
    *out_pc =
        reinterpret_cast<struct webrtc_PeerConnectionInterface_refcounted*>(
            result.MoveValue().release());
    *out_rtc_error = nullptr;
  } else {
    *out_pc = nullptr;
    auto rtc_error = new webrtc::RTCError(result.error());
    *out_rtc_error =
        reinterpret_cast<struct webrtc_RTCError_unique*>(rtc_error);
  }
}
void webrtc_PeerConnectionFactoryInterface_CreateVideoTrack(
    struct webrtc_PeerConnectionFactoryInterface* self,
    struct webrtc_VideoTrackSourceInterface_refcounted* source,
    const char* track_id,
    size_t track_id_len,
    struct webrtc_VideoTrackInterface_refcounted** out_track) {
  auto factory =
      reinterpret_cast<webrtc::PeerConnectionFactoryInterface*>(self);
  auto track_source = reinterpret_cast<webrtc::VideoTrackSourceInterface*>(
      webrtc_VideoTrackSourceInterface_refcounted_get(source));
  webrtc::scoped_refptr<webrtc::VideoTrackSourceInterface> track_source_ref(
      track_source);
  auto track_id_str = std::string(track_id, track_id_len);
  auto track = factory->CreateVideoTrack(track_source_ref, track_id_str);
  if (track) {
    *out_track =
        reinterpret_cast<struct webrtc_VideoTrackInterface_refcounted*>(
            track.release());
  } else {
    *out_track = nullptr;
  }
}
struct webrtc_RtpCapabilities*
webrtc_PeerConnectionFactoryInterface_GetRtpSenderCapabilities(
    struct webrtc_PeerConnectionFactoryInterface* self,
    int media_type) {
  auto factory =
      reinterpret_cast<webrtc::PeerConnectionFactoryInterface*>(self);
  auto caps = new webrtc::RtpCapabilities(factory->GetRtpSenderCapabilities(
      static_cast<webrtc::MediaType>(media_type)));
  return reinterpret_cast<struct webrtc_RtpCapabilities*>(caps);
}

struct webrtc_PeerConnectionFactoryInterface_Options*
webrtc_PeerConnectionFactoryInterface_Options_new() {
  auto options = new webrtc::PeerConnectionFactoryInterface::Options();
  return reinterpret_cast<
      struct webrtc_PeerConnectionFactoryInterface_Options*>(options);
}
void webrtc_PeerConnectionFactoryInterface_Options_delete(
    struct webrtc_PeerConnectionFactoryInterface_Options* self) {
  auto options =
      reinterpret_cast<webrtc::PeerConnectionFactoryInterface::Options*>(self);
  delete options;
}
void webrtc_PeerConnectionFactoryInterface_Options_set_disable_encryption(
    struct webrtc_PeerConnectionFactoryInterface_Options* self,
    int disable_encryption) {
  auto options =
      reinterpret_cast<webrtc::PeerConnectionFactoryInterface::Options*>(self);
  options->disable_encryption = disable_encryption != 0;
}
void webrtc_PeerConnectionFactoryInterface_Options_set_ssl_max_version(
    struct webrtc_PeerConnectionFactoryInterface_Options* self,
    int ssl_max_version) {
  auto options =
      reinterpret_cast<webrtc::PeerConnectionFactoryInterface::Options*>(self);
  options->ssl_max_version =
      static_cast<webrtc::SSLProtocolVersion>(ssl_max_version);
}
void webrtc_PeerConnectionFactoryInterface_SetOptions(
    struct webrtc_PeerConnectionFactoryInterface* self,
    struct webrtc_PeerConnectionFactoryInterface_Options* options) {
  auto factory =
      reinterpret_cast<webrtc::PeerConnectionFactoryInterface*>(self);
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionFactoryInterface::Options*>(
          options);
  factory->SetOptions(*opts);
}

extern const int webrtc_SSL_PROTOCOL_DTLS_12 = webrtc::SSL_PROTOCOL_DTLS_12;

void webrtc_PeerConnectionFactoryInterface_CreateAudioSource(
    struct webrtc_PeerConnectionFactoryInterface* self,
    struct webrtc_AudioSourceInterface_refcounted** out_source) {
  auto factory =
      reinterpret_cast<webrtc::PeerConnectionFactoryInterface*>(self);
  webrtc::AudioOptions options;
  auto source = factory->CreateAudioSource(options);
  if (source) {
    *out_source =
        reinterpret_cast<struct webrtc_AudioSourceInterface_refcounted*>(
            source.release());
  } else {
    *out_source = nullptr;
  }
}

void webrtc_PeerConnectionFactoryInterface_CreateAudioTrack(
    struct webrtc_PeerConnectionFactoryInterface* self,
    struct webrtc_AudioSourceInterface_refcounted* source,
    const char* track_id,
    size_t track_id_len,
    struct webrtc_AudioTrackInterface_refcounted** out_track) {
  auto factory =
      reinterpret_cast<webrtc::PeerConnectionFactoryInterface*>(self);
  auto audio_source = reinterpret_cast<webrtc::AudioSourceInterface*>(
      webrtc_AudioSourceInterface_refcounted_get(source));
  webrtc::scoped_refptr<webrtc::AudioSourceInterface> source_ref(audio_source);
  auto track_id_str = std::string(track_id, track_id_len);
  auto track = factory->CreateAudioTrack(track_id_str, source_ref.get());
  if (track) {
    *out_track =
        reinterpret_cast<struct webrtc_AudioTrackInterface_refcounted*>(
            track.release());
  } else {
    *out_track = nullptr;
  }
}
}
