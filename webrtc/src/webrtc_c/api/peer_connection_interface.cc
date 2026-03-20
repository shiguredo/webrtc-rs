#include "peer_connection_interface.h"

#include <stdarg.h>
#include <stddef.h>
#include <exception>
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
#include <api/environment/environment_factory.h>
#include <api/jsep.h>
#include <api/make_ref_counted.h>
#include <api/media_stream_interface.h>
#include <api/media_types.h>
#include <api/packet_socket_factory.h>
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
#include <p2p/client/basic_port_allocator.h>
#include <pc/connection_context.h>
#include <pc/media_factory.h>
#include <pc/peer_connection_factory.h>
#include <pc/peer_connection_factory_proxy.h>
#include <rtc_base/checks.h>
#include <rtc_base/crypt_string_revive.h>
#include <rtc_base/network.h>
#include <rtc_base/proxy_info_revive.h>
#include <rtc_base/socket_address.h>
#include <rtc_base/ssl_identity.h>
#include <rtc_base/ssl_stream_adapter.h>
#include <rtc_base/thread.h>

#include "../common.h"
#include "../common.impl.h"
#include "../pc/connection_context.h"
#include "../rtc_base/ssl_certificate.h"
#include "../rtc_base/ssl_identity.h"
#include "../rtc_base/thread.h"
#include "../std.h"
#include "audio/audio_processing.h"
#include "audio_codecs/audio_decoder_factory.h"
#include "audio_codecs/audio_encoder_factory.h"
#include "data_channel_interface.h"
#include "dtls_transport_interface.h"
#include "jsep.h"
#include "media_stream_interface.h"
#include "rtc_base/ssl_certificate.h"
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
      const struct webrtc_PeerConnectionObserver_cbs* observer,
      void* user_data)
      : user_data_(user_data) {
    if (observer != nullptr) {
      observer_ = *observer;
    }
  }

  ~PeerConnectionObserverImpl() override {
    if (observer_.OnDestroy != nullptr) {
      observer_.OnDestroy(user_data_);
    }
  }

  void OnSignalingChange(
      webrtc::PeerConnectionInterface::SignalingState new_state) override {}
  void OnDataChannel(webrtc::scoped_refptr<webrtc::DataChannelInterface>
                         data_channel) override {
    if (observer_.OnDataChannel != nullptr) {
      webrtc::scoped_refptr<webrtc::DataChannelInterface> data_channel_ref(
          data_channel);
      observer_.OnDataChannel(
          reinterpret_cast<struct webrtc_DataChannelInterface_refcounted*>(
              data_channel_ref.release()),
          user_data_);
    }
  }
  void OnStandardizedIceConnectionChange(
      webrtc::PeerConnectionInterface::IceConnectionState new_state) override {
    if (observer_.OnStandardizedIceConnectionChange != nullptr) {
      observer_.OnStandardizedIceConnectionChange(
          static_cast<webrtc_PeerConnectionInterface_IceConnectionState>(
              new_state),
          user_data_);
    }
  }
  void OnConnectionChange(
      webrtc::PeerConnectionInterface::PeerConnectionState new_state) override {
    if (observer_.OnConnectionChange != nullptr) {
      observer_.OnConnectionChange(
          static_cast<webrtc_PeerConnectionInterface_PeerConnectionState>(
              new_state),
          user_data_);
    }
  }
  void OnIceGatheringChange(
      webrtc::PeerConnectionInterface::IceGatheringState new_state) override {
    if (observer_.OnIceGatheringChange != nullptr) {
      observer_.OnIceGatheringChange(
          static_cast<webrtc_PeerConnectionInterface_IceGatheringState>(
              new_state),
          user_data_);
    }
  }
  void OnIceCandidate(const webrtc::IceCandidate* candidate) override {
    if (observer_.OnIceCandidate != nullptr) {
      observer_.OnIceCandidate(
          reinterpret_cast<const struct webrtc_IceCandidate*>(candidate),
          user_data_);
    }
  }
  void OnIceCandidateError(const std::string& address,
                           int port,
                           const std::string& url,
                           int error_code,
                           const std::string& error_text) override {
    if (observer_.OnIceCandidateError != nullptr) {
      observer_.OnIceCandidateError(
          address.c_str(), address.size(), port, url.c_str(), url.size(),
          error_code, error_text.c_str(), error_text.size(), user_data_);
    }
  }
  void OnTrack(webrtc::scoped_refptr<webrtc::RtpTransceiverInterface>
                   transceiver) override {
    if (observer_.OnTrack != nullptr) {
      webrtc::scoped_refptr<webrtc::RtpTransceiverInterface> transceiver_ref(
          transceiver);
      observer_.OnTrack(
          reinterpret_cast<struct webrtc_RtpTransceiverInterface_refcounted*>(
              transceiver_ref.release()),
          user_data_);
    }
  }
  void OnRemoveTrack(
      webrtc::scoped_refptr<webrtc::RtpReceiverInterface> receiver) override {
    if (observer_.OnRemoveTrack != nullptr) {
      webrtc::scoped_refptr<webrtc::RtpReceiverInterface> receiver_ref(
          receiver);
      observer_.OnRemoveTrack(
          reinterpret_cast<struct webrtc_RtpReceiverInterface_refcounted*>(
              receiver_ref.release()),
          user_data_);
    }
  }

 private:
  webrtc_PeerConnectionObserver_cbs observer_{};
  void* user_data_;
};

class RTCStatsCollectorCallbackImpl : public webrtc::RTCStatsCollectorCallback {
 public:
  RTCStatsCollectorCallbackImpl(
      struct webrtc_RTCStatsCollectorCallback_cbs* cbs,
      void* user_data)
      : cbs_(*cbs), user_data_(user_data) {}

  void OnStatsDelivered(
      const webrtc::scoped_refptr<const webrtc::RTCStatsReport>& report)
      override {
    if (cbs_.OnStatsDelivered == nullptr) {
      return;
    }
    webrtc::scoped_refptr<const webrtc::RTCStatsReport> report_ref(report);
    cbs_.OnStatsDelivered(
        reinterpret_cast<const struct webrtc_RTCStatsReport_refcounted*>(
            report_ref.release()),
        user_data_);
  }

 private:
  struct webrtc_RTCStatsCollectorCallback_cbs cbs_{};
  void* user_data_;
};

class RawCryptString : public webrtc::revive::CryptStringImpl {
 public:
  explicit RawCryptString(const std::string& str) : str_(str) {}

  size_t GetLength() const override { return str_.size(); }

  void CopyTo(char* dest, bool nullterminate) const override {
    for (size_t i = 0; i < str_.size(); ++i) {
      *dest++ = str_[i];
    }
    if (nullterminate) {
      *dest = '\0';
    }
  }

  std::string UrlEncode() const override { throw std::exception(); }

  CryptStringImpl* Copy() const override { return new RawCryptString(str_); }

  void CopyRawTo(std::vector<unsigned char>* dest) const override {
    dest->assign(str_.begin(), str_.end());
  }

 private:
  std::string str_;
};

WEBRTC_EXPORT struct webrtc_PeerConnectionObserver*
webrtc_PeerConnectionObserver_new(
    const struct webrtc_PeerConnectionObserver_cbs* observer,
    void* user_data) {
  auto impl = new PeerConnectionObserverImpl(observer, user_data);
  return reinterpret_cast<struct webrtc_PeerConnectionObserver*>(impl);
}
WEBRTC_EXPORT void webrtc_PeerConnectionObserver_delete(
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

WEBRTC_EXPORT struct webrtc_PeerConnectionInterface_RTCConfiguration*
webrtc_PeerConnectionInterface_RTCConfiguration_new() {
  auto config = new webrtc::PeerConnectionInterface::RTCConfiguration();
  return reinterpret_cast<
      struct webrtc_PeerConnectionInterface_RTCConfiguration*>(config);
}
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_RTCConfiguration_delete(
    struct webrtc_PeerConnectionInterface_RTCConfiguration* self) {
  auto config =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCConfiguration*>(
          self);
  delete config;
}
WEBRTC_EXPORT struct webrtc_PeerConnectionInterface_IceServer*
webrtc_PeerConnectionInterface_IceServer_new() {
  auto server = new webrtc::PeerConnectionInterface::IceServer();
  return reinterpret_cast<struct webrtc_PeerConnectionInterface_IceServer*>(
      server);
}
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_IceServer_delete(
    struct webrtc_PeerConnectionInterface_IceServer* self) {
  auto server =
      reinterpret_cast<webrtc::PeerConnectionInterface::IceServer*>(self);
  delete server;
}
WEBRTC_DEFINE_VECTOR(webrtc_PeerConnectionInterface_IceServer,
                     webrtc::PeerConnectionInterface::IceServer);
WEBRTC_EXPORT struct std_string_vector*
webrtc_PeerConnectionInterface_IceServer_get_urls(
    struct webrtc_PeerConnectionInterface_IceServer* self) {
  auto server =
      reinterpret_cast<webrtc::PeerConnectionInterface::IceServer*>(self);
  return reinterpret_cast<struct std_string_vector*>(&server->urls);
}
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_IceServer_set_username(
    struct webrtc_PeerConnectionInterface_IceServer* self,
    const char* username,
    size_t username_len) {
  auto server =
      reinterpret_cast<webrtc::PeerConnectionInterface::IceServer*>(self);
  server->username =
      username != nullptr ? std::string(username, username_len) : std::string();
}
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_IceServer_set_password(
    struct webrtc_PeerConnectionInterface_IceServer* self,
    const char* password,
    size_t password_len) {
  auto server =
      reinterpret_cast<webrtc::PeerConnectionInterface::IceServer*>(self);
  server->password =
      password != nullptr ? std::string(password, password_len) : std::string();
}
extern const int
    webrtc_PeerConnectionInterface_TlsCertPolicy_kTlsCertPolicySecure =
        static_cast<int>(webrtc::PeerConnectionInterface::TlsCertPolicy::
                             kTlsCertPolicySecure);
extern const int
    webrtc_PeerConnectionInterface_TlsCertPolicy_kTlsCertPolicyInsecureNoCheck =
        static_cast<int>(webrtc::PeerConnectionInterface::TlsCertPolicy::
                             kTlsCertPolicyInsecureNoCheck);
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_IceServer_set_tls_cert_policy(
    struct webrtc_PeerConnectionInterface_IceServer* self,
    webrtc_PeerConnectionInterface_TlsCertPolicy tls_cert_policy) {
  auto server =
      reinterpret_cast<webrtc::PeerConnectionInterface::IceServer*>(self);
  server->tls_cert_policy =
      static_cast<webrtc::PeerConnectionInterface::TlsCertPolicy>(
          tls_cert_policy);
}
WEBRTC_EXPORT void
webrtc_PeerConnectionInterface_IceServer_set_tls_client_identity(
    struct webrtc_PeerConnectionInterface_IceServer* self,
    struct webrtc_SSLIdentity_unique* identity) {
  auto server =
      reinterpret_cast<webrtc::PeerConnectionInterface::IceServer*>(self);
  if (identity != nullptr) {
    auto ssl_identity = std::unique_ptr<webrtc::SSLIdentity>(
        reinterpret_cast<webrtc::SSLIdentity*>(identity));
    server->tls_client_identity = std::move(ssl_identity);
  } else {
    server->tls_client_identity.reset();
  }
}
WEBRTC_EXPORT struct webrtc_PeerConnectionInterface_IceServer_vector*
webrtc_PeerConnectionInterface_RTCConfiguration_get_servers(
    struct webrtc_PeerConnectionInterface_RTCConfiguration* self) {
  auto config =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCConfiguration*>(
          self);
  return reinterpret_cast<
      struct webrtc_PeerConnectionInterface_IceServer_vector*>(
      &config->servers);
}
WEBRTC_EXPORT extern const int
    webrtc_PeerConnectionInterface_IceTransportsType_kRelay = static_cast<int>(
        webrtc::PeerConnectionInterface::IceTransportsType::kRelay);
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_RTCConfiguration_set_type(
    struct webrtc_PeerConnectionInterface_RTCConfiguration* self,
    webrtc_PeerConnectionInterface_IceTransportsType type) {
  auto config =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCConfiguration*>(
          self);
  config->type =
      static_cast<webrtc::PeerConnectionInterface::IceTransportsType>(type);
}
WEBRTC_EXPORT extern const int
    webrtc_PeerConnectionInterface_SdpSemantics_kUnifiedPlan =
        static_cast<int>(webrtc::SdpSemantics::kUnifiedPlan);
WEBRTC_EXPORT void
webrtc_PeerConnectionInterface_RTCConfiguration_set_sdp_semantics(
    struct webrtc_PeerConnectionInterface_RTCConfiguration* self,
    webrtc_PeerConnectionInterface_SdpSemantics sdp_semantics) {
  auto config =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCConfiguration*>(
          self);
  config->sdp_semantics = static_cast<webrtc::SdpSemantics>(sdp_semantics);
}
WEBRTC_EXPORT void
webrtc_PeerConnectionInterface_RTCConfiguration_set_enable_gcm_crypto_suites(
    struct webrtc_PeerConnectionInterface_RTCConfiguration* self,
    int enable_gcm_crypto_suites) {
  auto config =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCConfiguration*>(
          self);
  config->crypto_options.srtp.enable_gcm_crypto_suites =
      enable_gcm_crypto_suites != 0;
}
WEBRTC_EXPORT struct webrtc_PeerConnectionDependencies*
webrtc_PeerConnectionDependencies_new(
    struct webrtc_PeerConnectionObserver* observer) {
  auto pc_observer = reinterpret_cast<PeerConnectionObserverImpl*>(observer);
  auto deps = new webrtc::PeerConnectionDependencies(pc_observer);
  return reinterpret_cast<webrtc_PeerConnectionDependencies*>(deps);
}
WEBRTC_EXPORT void webrtc_PeerConnectionDependencies_delete(
    struct webrtc_PeerConnectionDependencies* self) {
  auto deps = reinterpret_cast<webrtc::PeerConnectionDependencies*>(self);
  delete deps;
}
// この関数は C ラッパーの方針である「libwebrtc の薄いラッパーに徹する」からは逸脱して複合的な処理を行っている。
// これは、この機能が元々の libwebrtc から削除されたのを shiguredo-webrtc-build で無理やり復活させた機能であるため、
// 薄いラッパーに徹することによるメリットが少ないと判断したためである。
// この部分を薄いラッパーに徹するのであれば、復活させた webrtc::revive::ProxyInfo などを Rust 側に公開することになるが、
// おそらく他の場所で利用することは無いため、この関数内で完結させる方針にした。
WEBRTC_EXPORT void webrtc_PeerConnectionDependencies_set_proxy(
    struct webrtc_PeerConnectionDependencies* self,
    struct webrtc_NetworkManager* network_manager,
    struct webrtc_PacketSocketFactory* socket_factory,
    const char* proxy_host,
    size_t proxy_host_len,
    int proxy_port,
    const char* proxy_username,
    size_t proxy_username_len,
    const char* proxy_password,
    size_t proxy_password_len,
    const char* proxy_agent,
    size_t proxy_agent_len) {
  auto deps = reinterpret_cast<webrtc::PeerConnectionDependencies*>(self);
  auto* nm = reinterpret_cast<webrtc::NetworkManager*>(network_manager);
  auto* sf = reinterpret_cast<webrtc::PacketSocketFactory*>(socket_factory);
  RTC_CHECK(nm != nullptr);
  RTC_CHECK(sf != nullptr);

  deps->allocator = std::make_unique<webrtc::BasicPortAllocator>(
      webrtc::CreateEnvironment(), nm, sf);

  webrtc::revive::ProxyInfo pi;
  pi.type = webrtc::revive::PROXY_HTTPS;
  const std::string host = proxy_host != nullptr
                               ? std::string(proxy_host, proxy_host_len)
                               : std::string();
  pi.address = webrtc::SocketAddress(host, proxy_port);

  if (proxy_username != nullptr && proxy_username_len != 0) {
    pi.username = std::string(proxy_username, proxy_username_len);
  }
  if (proxy_password != nullptr && proxy_password_len != 0) {
    pi.password = webrtc::revive::CryptString(
        RawCryptString(std::string(proxy_password, proxy_password_len)));
  }

  const std::string agent = proxy_agent != nullptr
                                ? std::string(proxy_agent, proxy_agent_len)
                                : std::string();
  deps->allocator->set_proxy(agent, pi);
}

WEBRTC_EXPORT void webrtc_PeerConnectionDependencies_set_tls_cert_verifier(
    struct webrtc_PeerConnectionDependencies* self,
    struct webrtc_SSLCertificateVerifier_unique* tls_cert_verifier) {
  auto deps = reinterpret_cast<webrtc::PeerConnectionDependencies*>(self);
  if (tls_cert_verifier == nullptr) {
    deps->tls_cert_verifier = nullptr;
    return;
  }
  auto verifier = reinterpret_cast<webrtc::SSLCertificateVerifier*>(
      webrtc_SSLCertificateVerifier_unique_get(tls_cert_verifier));
  deps->tls_cert_verifier =
      std::move(std::unique_ptr<webrtc::SSLCertificateVerifier>(verifier));
}

WEBRTC_EXPORT void webrtc_PeerConnectionInterface_CreateDataChannelOrError(
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

WEBRTC_EXPORT void webrtc_PeerConnectionInterface_AddTransceiver(
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
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_AddTransceiverWithTrack(
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
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_AddTrack(
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
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_CreateOffer(
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
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_CreateAnswer(
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
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_SetLocalDescription(
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
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_SetRemoteDescription(
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
WEBRTC_EXPORT int webrtc_PeerConnectionInterface_AddIceCandidate(
    struct webrtc_PeerConnectionInterface* self,
    const struct webrtc_IceCandidate* candidate) {
  auto pc = reinterpret_cast<webrtc::PeerConnectionInterface*>(self);
  auto ice = reinterpret_cast<const webrtc::IceCandidate*>(candidate);
  return pc->AddIceCandidate(ice) ? 1 : 0;
}
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_SetConfiguration(
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

WEBRTC_EXPORT struct webrtc_DtlsTransportInterface_refcounted*
webrtc_PeerConnectionInterface_LookupDtlsTransportByMid(
    struct webrtc_PeerConnectionInterface* self,
    const char* mid,
    size_t mid_len) {
  if (self == nullptr || mid == nullptr) {
    return nullptr;
  }
  auto pc = reinterpret_cast<webrtc::PeerConnectionInterface*>(self);
  auto transport = pc->LookupDtlsTransportByMid(std::string(mid, mid_len));
  if (transport == nullptr) {
    return nullptr;
  }
  return reinterpret_cast<struct webrtc_DtlsTransportInterface_refcounted*>(
      transport.release());
}

WEBRTC_EXPORT void webrtc_PeerConnectionInterface_GetStats(
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

WEBRTC_EXPORT struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions*
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_new() {
  auto opts = new webrtc::PeerConnectionInterface::RTCOfferAnswerOptions();
  return reinterpret_cast<
      struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions*>(opts);
}
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_delete(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  delete opts;
}
WEBRTC_EXPORT int
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_offer_to_receive_video(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  return opts->offer_to_receive_video;
}
WEBRTC_EXPORT void
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_offer_to_receive_video(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self,
    int offer_to_receive_video) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  opts->offer_to_receive_video = offer_to_receive_video;
}
WEBRTC_EXPORT int
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_offer_to_receive_audio(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  return opts->offer_to_receive_audio;
}
WEBRTC_EXPORT void
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_offer_to_receive_audio(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self,
    int offer_to_receive_audio) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  opts->offer_to_receive_audio = offer_to_receive_audio;
}
WEBRTC_EXPORT int
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_voice_activity_detection(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  return opts->voice_activity_detection ? 1 : 0;
}
WEBRTC_EXPORT void
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_voice_activity_detection(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self,
    int voice_activity_detection) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  opts->voice_activity_detection = voice_activity_detection != 0;
}
WEBRTC_EXPORT int
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_ice_restart(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  return opts->ice_restart ? 1 : 0;
}
WEBRTC_EXPORT void
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_ice_restart(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self,
    int ice_restart) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  opts->ice_restart = ice_restart != 0;
}
WEBRTC_EXPORT int
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_use_rtp_mux(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  return opts->use_rtp_mux ? 1 : 0;
}
WEBRTC_EXPORT void
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_use_rtp_mux(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self,
    int use_rtp_mux) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  opts->use_rtp_mux = use_rtp_mux != 0;
}
WEBRTC_EXPORT int
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_raw_packetization_for_video(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  return opts->raw_packetization_for_video ? 1 : 0;
}
WEBRTC_EXPORT void
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_raw_packetization_for_video(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self,
    int raw_packetization_for_video) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  opts->raw_packetization_for_video = raw_packetization_for_video != 0;
}
WEBRTC_EXPORT int
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_num_simulcast_layers(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  return opts->num_simulcast_layers;
}
WEBRTC_EXPORT void
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_num_simulcast_layers(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self,
    int num_simulcast_layers) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  opts->num_simulcast_layers = num_simulcast_layers;
}
WEBRTC_EXPORT int
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_use_obsolete_sctp_sdp(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  return opts->use_obsolete_sctp_sdp ? 1 : 0;
}
WEBRTC_EXPORT void
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_use_obsolete_sctp_sdp(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self,
    int use_obsolete_sctp_sdp) {
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionInterface::RTCOfferAnswerOptions*>(
          self);
  opts->use_obsolete_sctp_sdp = use_obsolete_sctp_sdp != 0;
}

WEBRTC_EXPORT extern const int
    webrtc_PeerConnectionInterface_PeerConnectionState_kNew =
        (int)webrtc::PeerConnectionInterface::PeerConnectionState::kNew;
extern const int
    webrtc_PeerConnectionInterface_PeerConnectionState_kConnecting =
        (int)webrtc::PeerConnectionInterface::PeerConnectionState::kConnecting;
WEBRTC_EXPORT extern const int
    webrtc_PeerConnectionInterface_PeerConnectionState_kConnected =
        (int)webrtc::PeerConnectionInterface::PeerConnectionState::kConnected;
extern const int
    webrtc_PeerConnectionInterface_PeerConnectionState_kDisconnected = (int)
        webrtc::PeerConnectionInterface::PeerConnectionState::kDisconnected;
WEBRTC_EXPORT extern const int
    webrtc_PeerConnectionInterface_PeerConnectionState_kFailed =
        (int)webrtc::PeerConnectionInterface::PeerConnectionState::kFailed;
WEBRTC_EXPORT extern const int
    webrtc_PeerConnectionInterface_PeerConnectionState_kClosed =
        (int)webrtc::PeerConnectionInterface::PeerConnectionState::kClosed;
extern const int
    webrtc_PeerConnectionInterface_IceConnectionState_kIceConnectionNew = (int)
        webrtc::PeerConnectionInterface::IceConnectionState::kIceConnectionNew;
extern const int
    webrtc_PeerConnectionInterface_IceConnectionState_kIceConnectionChecking =
        (int)webrtc::PeerConnectionInterface::IceConnectionState::
            kIceConnectionChecking;
extern const int
    webrtc_PeerConnectionInterface_IceConnectionState_kIceConnectionConnected =
        (int)webrtc::PeerConnectionInterface::IceConnectionState::
            kIceConnectionConnected;
extern const int
    webrtc_PeerConnectionInterface_IceConnectionState_kIceConnectionCompleted =
        (int)webrtc::PeerConnectionInterface::IceConnectionState::
            kIceConnectionCompleted;
extern const int
    webrtc_PeerConnectionInterface_IceConnectionState_kIceConnectionFailed =
        (int)webrtc::PeerConnectionInterface::IceConnectionState::
            kIceConnectionFailed;
extern const int
    webrtc_PeerConnectionInterface_IceConnectionState_kIceConnectionDisconnected =
        (int)webrtc::PeerConnectionInterface::IceConnectionState::
            kIceConnectionDisconnected;
extern const int
    webrtc_PeerConnectionInterface_IceConnectionState_kIceConnectionClosed =
        (int)webrtc::PeerConnectionInterface::IceConnectionState::
            kIceConnectionClosed;
extern const int
    webrtc_PeerConnectionInterface_IceConnectionState_kIceConnectionMax = (int)
        webrtc::PeerConnectionInterface::IceConnectionState::kIceConnectionMax;
extern const int
    webrtc_PeerConnectionInterface_IceGatheringState_kIceGatheringNew = (int)
        webrtc::PeerConnectionInterface::IceGatheringState::kIceGatheringNew;
extern const int
    webrtc_PeerConnectionInterface_IceGatheringState_kIceGatheringGathering =
        (int)webrtc::PeerConnectionInterface::IceGatheringState::
            kIceGatheringGathering;
extern const int
    webrtc_PeerConnectionInterface_IceGatheringState_kIceGatheringComplete =
        (int)webrtc::PeerConnectionInterface::IceGatheringState::
            kIceGatheringComplete;
}

// -------------------------
// webrtc::PeerConnectionFactoryDependencies
// -------------------------

extern "C" {
WEBRTC_EXPORT struct webrtc_PeerConnectionFactoryDependencies*
webrtc_PeerConnectionFactoryDependencies_new() {
  auto deps = new webrtc::PeerConnectionFactoryDependencies();
  return reinterpret_cast<struct webrtc_PeerConnectionFactoryDependencies*>(
      deps);
}
WEBRTC_EXPORT void webrtc_PeerConnectionFactoryDependencies_delete(
    struct webrtc_PeerConnectionFactoryDependencies* self) {
  auto deps =
      reinterpret_cast<webrtc::PeerConnectionFactoryDependencies*>(self);
  delete deps;
}
WEBRTC_EXPORT void webrtc_PeerConnectionFactoryDependencies_set_network_thread(
    struct webrtc_PeerConnectionFactoryDependencies* self,
    struct webrtc_Thread* network_thread) {
  auto deps =
      reinterpret_cast<webrtc::PeerConnectionFactoryDependencies*>(self);
  auto thread = reinterpret_cast<webrtc::Thread*>(network_thread);
  deps->network_thread = thread;
}
WEBRTC_EXPORT void webrtc_PeerConnectionFactoryDependencies_set_worker_thread(
    struct webrtc_PeerConnectionFactoryDependencies* self,
    struct webrtc_Thread* worker_thread) {
  auto deps =
      reinterpret_cast<webrtc::PeerConnectionFactoryDependencies*>(self);
  auto thread = reinterpret_cast<webrtc::Thread*>(worker_thread);
  deps->worker_thread = thread;
}
WEBRTC_EXPORT void
webrtc_PeerConnectionFactoryDependencies_set_signaling_thread(
    struct webrtc_PeerConnectionFactoryDependencies* self,
    struct webrtc_Thread* signaling_thread) {
  auto deps =
      reinterpret_cast<webrtc::PeerConnectionFactoryDependencies*>(self);
  auto thread = reinterpret_cast<webrtc::Thread*>(signaling_thread);
  deps->signaling_thread = thread;
}
WEBRTC_EXPORT void webrtc_PeerConnectionFactoryDependencies_set_adm(
    struct webrtc_PeerConnectionFactoryDependencies* self,
    struct webrtc_AudioDeviceModule_refcounted* adm) {
  auto deps =
      reinterpret_cast<webrtc::PeerConnectionFactoryDependencies*>(self);
  auto audio_device_module = reinterpret_cast<webrtc::AudioDeviceModule*>(adm);
  deps->adm = audio_device_module;
}
WEBRTC_EXPORT void
webrtc_PeerConnectionFactoryDependencies_set_event_log_factory(
    struct webrtc_PeerConnectionFactoryDependencies* self,
    struct webrtc_RtcEventLogFactory_unique* event_log_factory) {
  auto deps =
      reinterpret_cast<webrtc::PeerConnectionFactoryDependencies*>(self);
  auto factory = reinterpret_cast<webrtc::RtcEventLogFactoryInterface*>(
      webrtc_RtcEventLogFactory_unique_get(event_log_factory));
  deps->event_log_factory =
      std::move(std::unique_ptr<webrtc::RtcEventLogFactoryInterface>(factory));
}
WEBRTC_EXPORT void
webrtc_PeerConnectionFactoryDependencies_set_audio_encoder_factory(
    struct webrtc_PeerConnectionFactoryDependencies* self,
    struct webrtc_AudioEncoderFactory_refcounted* audio_encoder_factory) {
  auto deps =
      reinterpret_cast<webrtc::PeerConnectionFactoryDependencies*>(self);
  auto factory =
      reinterpret_cast<webrtc::AudioEncoderFactory*>(audio_encoder_factory);
  deps->audio_encoder_factory = factory;
}
WEBRTC_EXPORT void
webrtc_PeerConnectionFactoryDependencies_set_audio_decoder_factory(
    struct webrtc_PeerConnectionFactoryDependencies* self,
    struct webrtc_AudioDecoderFactory_refcounted* audio_decoder_factory) {
  auto deps =
      reinterpret_cast<webrtc::PeerConnectionFactoryDependencies*>(self);
  auto factory =
      reinterpret_cast<webrtc::AudioDecoderFactory*>(audio_decoder_factory);
  deps->audio_decoder_factory = factory;
}
WEBRTC_EXPORT void
webrtc_PeerConnectionFactoryDependencies_set_audio_processing_builder(
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
WEBRTC_EXPORT void
webrtc_PeerConnectionFactoryDependencies_set_video_encoder_factory(
    struct webrtc_PeerConnectionFactoryDependencies* self,
    struct webrtc_VideoEncoderFactory_unique* video_encoder_factory) {
  auto deps =
      reinterpret_cast<webrtc::PeerConnectionFactoryDependencies*>(self);
  auto factory = reinterpret_cast<webrtc::VideoEncoderFactory*>(
      webrtc_VideoEncoderFactory_unique_get(video_encoder_factory));
  deps->video_encoder_factory =
      std::move(std::unique_ptr<webrtc::VideoEncoderFactory>(factory));
}
WEBRTC_EXPORT void
webrtc_PeerConnectionFactoryDependencies_set_video_decoder_factory(
    struct webrtc_PeerConnectionFactoryDependencies* self,
    struct webrtc_VideoDecoderFactory_unique* video_decoder_factory) {
  auto deps =
      reinterpret_cast<webrtc::PeerConnectionFactoryDependencies*>(self);
  auto factory = reinterpret_cast<webrtc::VideoDecoderFactory*>(
      webrtc_VideoDecoderFactory_unique_get(video_decoder_factory));
  deps->video_decoder_factory =
      std::move(std::unique_ptr<webrtc::VideoDecoderFactory>(factory));
}

WEBRTC_EXPORT void webrtc_EnableMedia(
    struct webrtc_PeerConnectionFactoryDependencies* dependencies) {
  auto deps = reinterpret_cast<webrtc::PeerConnectionFactoryDependencies*>(
      dependencies);
  webrtc::EnableMedia(*deps);
}
}

// -------------------------
// webrtc::PeerConnectionFactoryInterface
// -------------------------

namespace {

class PeerConnectionFactoryWithContext : public webrtc::PeerConnectionFactory {
 public:
  explicit PeerConnectionFactoryWithContext(
      webrtc::PeerConnectionFactoryDependencies dependencies)
      : PeerConnectionFactoryWithContext(
            webrtc::ConnectionContext::Create(webrtc::CreateEnvironment(),
                                              &dependencies),
            &dependencies) {}

  PeerConnectionFactoryWithContext(
      webrtc::scoped_refptr<webrtc::ConnectionContext> context,
      webrtc::PeerConnectionFactoryDependencies* dependencies)
      : conn_context_(context),
        webrtc::PeerConnectionFactory(webrtc::CreateEnvironment(),
                                      context,
                                      dependencies) {}

  static webrtc::scoped_refptr<PeerConnectionFactoryWithContext> Create(
      webrtc::PeerConnectionFactoryDependencies dependencies) {
    return webrtc::make_ref_counted<PeerConnectionFactoryWithContext>(
        std::move(dependencies));
  }

  webrtc::scoped_refptr<webrtc::ConnectionContext> GetContext() const {
    return conn_context_;
  }

 private:
  webrtc::scoped_refptr<webrtc::ConnectionContext> conn_context_;
};

std::pair<webrtc::scoped_refptr<webrtc::PeerConnectionFactoryInterface>,
          webrtc::scoped_refptr<webrtc::ConnectionContext>>
CreateModularPeerConnectionFactoryWithContext(
    webrtc::PeerConnectionFactoryDependencies dependencies) {
  using result_type =
      std::pair<webrtc::scoped_refptr<webrtc::PeerConnectionFactoryInterface>,
                webrtc::scoped_refptr<webrtc::ConnectionContext>>;
  return dependencies.signaling_thread->BlockingCall([&dependencies]() {
    auto factory =
        PeerConnectionFactoryWithContext::Create(std::move(dependencies));
    if (factory == nullptr) {
      return result_type(nullptr, nullptr);
    }
    auto context = factory->GetContext();
    auto proxy = webrtc::PeerConnectionFactoryProxy::Create(
        factory->signaling_thread(), factory->worker_thread(), factory);
    return result_type(proxy, context);
  });
}

}  // namespace

extern "C" {
WEBRTC_DEFINE_REFCOUNTED(webrtc_PeerConnectionFactoryInterface,
                         webrtc::PeerConnectionFactoryInterface);

WEBRTC_EXPORT struct webrtc_PeerConnectionFactoryInterface_refcounted*
webrtc_CreateModularPeerConnectionFactory(
    struct webrtc_PeerConnectionFactoryDependencies* dependencies) {
  auto deps = reinterpret_cast<webrtc::PeerConnectionFactoryDependencies*>(
      dependencies);
  auto factory = webrtc::CreateModularPeerConnectionFactory(std::move(*deps));
  if (factory == nullptr) {
    return nullptr;
  }
  return reinterpret_cast<
      struct webrtc_PeerConnectionFactoryInterface_refcounted*>(
      factory.release());
}

WEBRTC_EXPORT struct webrtc_PeerConnectionFactoryInterface_refcounted*
webrtc_CreateModularPeerConnectionFactoryWithContext(
    struct webrtc_PeerConnectionFactoryDependencies* dependencies,
    struct webrtc_ConnectionContext_refcounted** out_context) {
  if (out_context == nullptr) {
    return nullptr;
  }
  *out_context = nullptr;
  auto deps = reinterpret_cast<webrtc::PeerConnectionFactoryDependencies*>(
      dependencies);
  auto p = CreateModularPeerConnectionFactoryWithContext(std::move(*deps));
  auto factory = p.first;
  auto context = p.second;
  if (factory == nullptr || context == nullptr) {
    return nullptr;
  }
  *out_context = reinterpret_cast<struct webrtc_ConnectionContext_refcounted*>(
      context.release());
  return reinterpret_cast<
      struct webrtc_PeerConnectionFactoryInterface_refcounted*>(
      factory.release());
}

WEBRTC_EXPORT void
webrtc_PeerConnectionFactoryInterface_CreatePeerConnectionOrError(
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
WEBRTC_EXPORT void webrtc_PeerConnectionFactoryInterface_CreateVideoTrack(
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
WEBRTC_EXPORT struct webrtc_RtpCapabilities*
webrtc_PeerConnectionFactoryInterface_GetRtpSenderCapabilities(
    struct webrtc_PeerConnectionFactoryInterface* self,
    int media_type) {
  auto factory =
      reinterpret_cast<webrtc::PeerConnectionFactoryInterface*>(self);
  auto caps = new webrtc::RtpCapabilities(factory->GetRtpSenderCapabilities(
      static_cast<webrtc::MediaType>(media_type)));
  return reinterpret_cast<struct webrtc_RtpCapabilities*>(caps);
}

WEBRTC_EXPORT struct webrtc_PeerConnectionFactoryInterface_Options*
webrtc_PeerConnectionFactoryInterface_Options_new() {
  auto options = new webrtc::PeerConnectionFactoryInterface::Options();
  return reinterpret_cast<
      struct webrtc_PeerConnectionFactoryInterface_Options*>(options);
}
WEBRTC_EXPORT void webrtc_PeerConnectionFactoryInterface_Options_delete(
    struct webrtc_PeerConnectionFactoryInterface_Options* self) {
  auto options =
      reinterpret_cast<webrtc::PeerConnectionFactoryInterface::Options*>(self);
  delete options;
}
WEBRTC_EXPORT void
webrtc_PeerConnectionFactoryInterface_Options_set_disable_encryption(
    struct webrtc_PeerConnectionFactoryInterface_Options* self,
    int disable_encryption) {
  auto options =
      reinterpret_cast<webrtc::PeerConnectionFactoryInterface::Options*>(self);
  options->disable_encryption = disable_encryption != 0;
}
WEBRTC_EXPORT void
webrtc_PeerConnectionFactoryInterface_Options_set_ssl_max_version(
    struct webrtc_PeerConnectionFactoryInterface_Options* self,
    int ssl_max_version) {
  auto options =
      reinterpret_cast<webrtc::PeerConnectionFactoryInterface::Options*>(self);
  options->ssl_max_version =
      static_cast<webrtc::SSLProtocolVersion>(ssl_max_version);
}
WEBRTC_EXPORT void webrtc_PeerConnectionFactoryInterface_SetOptions(
    struct webrtc_PeerConnectionFactoryInterface* self,
    struct webrtc_PeerConnectionFactoryInterface_Options* options) {
  auto factory =
      reinterpret_cast<webrtc::PeerConnectionFactoryInterface*>(self);
  auto opts =
      reinterpret_cast<webrtc::PeerConnectionFactoryInterface::Options*>(
          options);
  factory->SetOptions(*opts);
}

WEBRTC_EXPORT extern const int webrtc_SSL_PROTOCOL_DTLS_12 =
    webrtc::SSL_PROTOCOL_DTLS_12;

WEBRTC_EXPORT void webrtc_PeerConnectionFactoryInterface_CreateAudioSource(
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

WEBRTC_EXPORT void webrtc_PeerConnectionFactoryInterface_CreateAudioTrack(
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
