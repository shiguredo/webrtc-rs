#pragma once

#include <stddef.h>

#include "../common.h"
#include "../pc/connection_context.h"
#include "../rtc_base/ssl_certificate.h"
#include "../rtc_base/ssl_identity.h"
#include "../rtc_base/thread.h"
#include "../std.h"
#include "data_channel_interface.h"
#include "dtls_transport_interface.h"
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
#include "video_codecs/video_decoder_factory.h"
#include "video_codecs/video_encoder_factory.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::PeerConnectionInterface
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_PeerConnectionInterface);

struct webrtc_PeerConnectionObserver;
struct webrtc_CreateSessionDescriptionObserver;
struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions;
struct webrtc_PeerConnectionInterface_RTCConfiguration;
struct webrtc_NetworkManager;
struct webrtc_PacketSocketFactory;
WEBRTC_EXPORT struct webrtc_PeerConnectionInterface_RTCConfiguration*
webrtc_PeerConnectionInterface_RTCConfiguration_new();
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_RTCConfiguration_delete(
    struct webrtc_PeerConnectionInterface_RTCConfiguration* self);
struct webrtc_PeerConnectionInterface_IceServer;
WEBRTC_EXPORT struct webrtc_PeerConnectionInterface_IceServer*
webrtc_PeerConnectionInterface_IceServer_new();
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_IceServer_delete(
    struct webrtc_PeerConnectionInterface_IceServer* self);
WEBRTC_DECLARE_VECTOR(webrtc_PeerConnectionInterface_IceServer);
WEBRTC_EXPORT struct std_string_vector*
webrtc_PeerConnectionInterface_IceServer_get_urls(
    struct webrtc_PeerConnectionInterface_IceServer* self);
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_IceServer_set_username(
    struct webrtc_PeerConnectionInterface_IceServer* self,
    const char* username,
    size_t username_len);
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_IceServer_set_password(
    struct webrtc_PeerConnectionInterface_IceServer* self,
    const char* password,
    size_t password_len);
typedef int webrtc_PeerConnectionInterface_TlsCertPolicy;
WEBRTC_EXPORT extern const int
    webrtc_PeerConnectionInterface_TlsCertPolicy_kTlsCertPolicySecure;
WEBRTC_EXPORT extern const int
    webrtc_PeerConnectionInterface_TlsCertPolicy_kTlsCertPolicyInsecureNoCheck;
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_IceServer_set_tls_cert_policy(
    struct webrtc_PeerConnectionInterface_IceServer* self,
    webrtc_PeerConnectionInterface_TlsCertPolicy tls_cert_policy);
WEBRTC_EXPORT void
webrtc_PeerConnectionInterface_IceServer_set_tls_client_identity(
    struct webrtc_PeerConnectionInterface_IceServer* self,
    struct webrtc_SSLIdentity_unique* identity);
WEBRTC_EXPORT struct webrtc_PeerConnectionInterface_IceServer_vector*
webrtc_PeerConnectionInterface_RTCConfiguration_get_servers(
    struct webrtc_PeerConnectionInterface_RTCConfiguration* self);
typedef int webrtc_PeerConnectionInterface_IceTransportsType;
WEBRTC_EXPORT extern const int
    webrtc_PeerConnectionInterface_IceTransportsType_kRelay;
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_RTCConfiguration_set_type(
    struct webrtc_PeerConnectionInterface_RTCConfiguration* self,
    webrtc_PeerConnectionInterface_IceTransportsType type);
typedef int webrtc_PeerConnectionInterface_SdpSemantics;
WEBRTC_EXPORT extern const int
    webrtc_PeerConnectionInterface_SdpSemantics_kUnifiedPlan;
WEBRTC_EXPORT void
webrtc_PeerConnectionInterface_RTCConfiguration_set_sdp_semantics(
    struct webrtc_PeerConnectionInterface_RTCConfiguration* self,
    webrtc_PeerConnectionInterface_SdpSemantics sdp_semantics);
WEBRTC_EXPORT void
webrtc_PeerConnectionInterface_RTCConfiguration_set_enable_gcm_crypto_suites(
    struct webrtc_PeerConnectionInterface_RTCConfiguration* self,
    int enable_gcm_crypto_suites);
struct webrtc_PeerConnectionDependencies;
WEBRTC_EXPORT struct webrtc_PeerConnectionDependencies*
webrtc_PeerConnectionDependencies_new(
    struct webrtc_PeerConnectionObserver* observer);
WEBRTC_EXPORT void webrtc_PeerConnectionDependencies_delete(
    struct webrtc_PeerConnectionDependencies* self);
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
    size_t proxy_agent_len);
WEBRTC_EXPORT void webrtc_PeerConnectionDependencies_set_tls_cert_verifier(
    struct webrtc_PeerConnectionDependencies* self,
    struct webrtc_SSLCertificateVerifier_unique* tls_cert_verifier);
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_CreateDataChannelOrError(
    struct webrtc_PeerConnectionInterface* self,
    const char* label,
    size_t label_len,
    struct webrtc_DataChannelInit* init,
    struct webrtc_DataChannelInterface_refcounted** out_data_channel,
    struct webrtc_RTCError_unique** out_rtc_error);
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_AddTransceiver(
    struct webrtc_PeerConnectionInterface* self,
    int media_type,
    struct webrtc_RtpTransceiverInit* init,
    struct webrtc_RtpTransceiverInterface_refcounted** out_transceiver,
    struct webrtc_RTCError_unique** out_rtc_error);
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_AddTransceiverWithTrack(
    struct webrtc_PeerConnectionInterface* self,
    struct webrtc_VideoTrackInterface_refcounted* track,
    struct webrtc_RtpTransceiverInit* init,
    struct webrtc_RtpTransceiverInterface_refcounted** out_transceiver,
    struct webrtc_RTCError_unique** out_rtc_error);
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_AddTrack(
    struct webrtc_PeerConnectionInterface* self,
    struct webrtc_MediaStreamTrackInterface_refcounted* track,
    struct std_string_vector* stream_ids,
    struct webrtc_RtpSenderInterface_refcounted** out_sender,
    struct webrtc_RTCError_unique** out_rtc_error);
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_CreateOffer(
    struct webrtc_PeerConnectionInterface* self,
    struct webrtc_CreateSessionDescriptionObserver* observer,
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* options);
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_CreateAnswer(
    struct webrtc_PeerConnectionInterface* self,
    struct webrtc_CreateSessionDescriptionObserver* observer,
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* options);
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_SetLocalDescription(
    struct webrtc_PeerConnectionInterface* self,
    struct webrtc_SessionDescriptionInterface_unique* desc,
    struct webrtc_SetLocalDescriptionObserverInterface_refcounted* observer);
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_SetRemoteDescription(
    struct webrtc_PeerConnectionInterface* self,
    struct webrtc_SessionDescriptionInterface_unique* desc,
    struct webrtc_SetRemoteDescriptionObserverInterface_refcounted* observer);
WEBRTC_EXPORT int webrtc_PeerConnectionInterface_AddIceCandidate(
    struct webrtc_PeerConnectionInterface* self,
    const struct webrtc_IceCandidate* candidate);
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_SetConfiguration(
    struct webrtc_PeerConnectionInterface* self,
    struct webrtc_PeerConnectionInterface_RTCConfiguration* config,
    struct webrtc_RTCError_unique** out_rtc_error);
WEBRTC_EXPORT struct webrtc_DtlsTransportInterface_refcounted*
webrtc_PeerConnectionInterface_LookupDtlsTransportByMid(
    struct webrtc_PeerConnectionInterface* self,
    const char* mid,
    size_t mid_len);

WEBRTC_EXPORT void webrtc_PeerConnectionInterface_GetStats(
    struct webrtc_PeerConnectionInterface* self,
    struct webrtc_RTCStatsCollectorCallback_cbs* cbs,
    void* user_data);

typedef int webrtc_PeerConnectionInterface_PeerConnectionState;
WEBRTC_EXPORT extern const int
    webrtc_PeerConnectionInterface_PeerConnectionState_kNew;
WEBRTC_EXPORT extern const int
    webrtc_PeerConnectionInterface_PeerConnectionState_kConnecting;
WEBRTC_EXPORT extern const int
    webrtc_PeerConnectionInterface_PeerConnectionState_kConnected;
WEBRTC_EXPORT extern const int
    webrtc_PeerConnectionInterface_PeerConnectionState_kDisconnected;
WEBRTC_EXPORT extern const int
    webrtc_PeerConnectionInterface_PeerConnectionState_kFailed;
WEBRTC_EXPORT extern const int
    webrtc_PeerConnectionInterface_PeerConnectionState_kClosed;
typedef int webrtc_PeerConnectionInterface_IceConnectionState;
WEBRTC_EXPORT extern const int
    webrtc_PeerConnectionInterface_IceConnectionState_kIceConnectionNew;
WEBRTC_EXPORT extern const int
    webrtc_PeerConnectionInterface_IceConnectionState_kIceConnectionChecking;
WEBRTC_EXPORT extern const int
    webrtc_PeerConnectionInterface_IceConnectionState_kIceConnectionConnected;
WEBRTC_EXPORT extern const int
    webrtc_PeerConnectionInterface_IceConnectionState_kIceConnectionCompleted;
WEBRTC_EXPORT extern const int
    webrtc_PeerConnectionInterface_IceConnectionState_kIceConnectionFailed;
WEBRTC_EXPORT extern const int
    webrtc_PeerConnectionInterface_IceConnectionState_kIceConnectionDisconnected;
WEBRTC_EXPORT extern const int
    webrtc_PeerConnectionInterface_IceConnectionState_kIceConnectionClosed;
WEBRTC_EXPORT extern const int
    webrtc_PeerConnectionInterface_IceConnectionState_kIceConnectionMax;
typedef int webrtc_PeerConnectionInterface_IceGatheringState;
WEBRTC_EXPORT extern const int
    webrtc_PeerConnectionInterface_IceGatheringState_kIceGatheringNew;
WEBRTC_EXPORT extern const int
    webrtc_PeerConnectionInterface_IceGatheringState_kIceGatheringGathering;
WEBRTC_EXPORT extern const int
    webrtc_PeerConnectionInterface_IceGatheringState_kIceGatheringComplete;

WEBRTC_EXPORT struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions*
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_new();
WEBRTC_EXPORT void webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_delete(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self);
WEBRTC_EXPORT int
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_offer_to_receive_video(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self);
WEBRTC_EXPORT void
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_offer_to_receive_video(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self,
    int offer_to_receive_video);
WEBRTC_EXPORT int
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_offer_to_receive_audio(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self);
WEBRTC_EXPORT void
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_offer_to_receive_audio(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self,
    int offer_to_receive_audio);
WEBRTC_EXPORT int
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_voice_activity_detection(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self);
WEBRTC_EXPORT void
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_voice_activity_detection(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self,
    int voice_activity_detection);
WEBRTC_EXPORT int
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_ice_restart(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self);
WEBRTC_EXPORT void
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_ice_restart(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self,
    int ice_restart);
WEBRTC_EXPORT int
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_use_rtp_mux(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self);
WEBRTC_EXPORT void
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_use_rtp_mux(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self,
    int use_rtp_mux);
WEBRTC_EXPORT int
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_raw_packetization_for_video(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self);
WEBRTC_EXPORT void
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_raw_packetization_for_video(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self,
    int raw_packetization_for_video);
WEBRTC_EXPORT int
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_num_simulcast_layers(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self);
WEBRTC_EXPORT void
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_num_simulcast_layers(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self,
    int num_simulcast_layers);
WEBRTC_EXPORT int
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_get_use_obsolete_sctp_sdp(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self);
WEBRTC_EXPORT void
webrtc_PeerConnectionInterface_RTCOfferAnswerOptions_set_use_obsolete_sctp_sdp(
    struct webrtc_PeerConnectionInterface_RTCOfferAnswerOptions* self,
    int use_obsolete_sctp_sdp);

// -------------------------
// webrtc::PeerConnectionObserver
// -------------------------

struct webrtc_PeerConnectionObserver_cbs {
  // void OnSignalingChange(
  //     webrtc::PeerConnectionInterface::SignalingState new_state) override {
  //   RTC_LOG(LS_INFO) << "OnSignalingChange: new_state="
  //                    << webrtc::PeerConnectionInterface::AsString(new_state);
  // }
  // void OnDataChannel(webrtc::scoped_refptr<webrtc::DataChannelInterface> data_channel) override {}
  void (*OnStandardizedIceConnectionChange)(
      webrtc_PeerConnectionInterface_IceConnectionState new_state,
      void* user_data);
  void (*OnConnectionChange)(
      webrtc_PeerConnectionInterface_PeerConnectionState new_state,
      void* user_data);
  void (*OnIceCandidate)(const struct webrtc_IceCandidate* candidate,
                         void* user_data);
  void (*OnIceCandidateError)(const char* address,
                              size_t address_len,
                              int port,
                              const char* url,
                              size_t url_len,
                              int error_code,
                              const char* error_text,
                              size_t error_text_len,
                              void* user_data);
  void (*OnTrack)(struct webrtc_RtpTransceiverInterface_refcounted* transceiver,
                  void* user_data);
  void (*OnRemoveTrack)(struct webrtc_RtpReceiverInterface_refcounted* receiver,
                        void* user_data);
  void (*OnDataChannel)(
      struct webrtc_DataChannelInterface_refcounted* data_channel,
      void* user_data);
  void (*OnDestroy)(void* user_data);
  void (*OnIceGatheringChange)(
      webrtc_PeerConnectionInterface_IceGatheringState new_state,
      void* user_data);
  // void OnIceCandidate(const webrtc::IceCandidate* candidate) override {}
  // void OnIceCandidateError(const std::string& address, int port, const std::string& url, int error_code, const std::string& error_text) override {}
  // void OnTrack(webrtc::scoped_refptr<webrtc::RtpTransceiverInterface> transceiver) override {}
  // void OnRemoveTrack(webrtc::scoped_refptr<webrtc::RtpReceiverInterface> receiver) override {}
};
WEBRTC_EXPORT struct webrtc_PeerConnectionObserver*
webrtc_PeerConnectionObserver_new(
    const struct webrtc_PeerConnectionObserver_cbs* observer,
    void* user_data);
WEBRTC_EXPORT void webrtc_PeerConnectionObserver_delete(
    struct webrtc_PeerConnectionObserver* self);

// -------------------------
// webrtc::PeerConnectionFactoryDependencies
// -------------------------

struct webrtc_PeerConnectionFactoryDependencies;
WEBRTC_EXPORT struct webrtc_PeerConnectionFactoryDependencies*
webrtc_PeerConnectionFactoryDependencies_new();
WEBRTC_EXPORT void webrtc_PeerConnectionFactoryDependencies_delete(
    struct webrtc_PeerConnectionFactoryDependencies* self);
WEBRTC_EXPORT void webrtc_PeerConnectionFactoryDependencies_set_network_thread(
    struct webrtc_PeerConnectionFactoryDependencies* self,
    struct webrtc_Thread* network_thread);
WEBRTC_EXPORT void webrtc_PeerConnectionFactoryDependencies_set_worker_thread(
    struct webrtc_PeerConnectionFactoryDependencies* self,
    struct webrtc_Thread* worker_thread);
WEBRTC_EXPORT void
webrtc_PeerConnectionFactoryDependencies_set_signaling_thread(
    struct webrtc_PeerConnectionFactoryDependencies* self,
    struct webrtc_Thread* signaling_thread);
WEBRTC_EXPORT void webrtc_PeerConnectionFactoryDependencies_set_adm(
    struct webrtc_PeerConnectionFactoryDependencies* self,
    struct webrtc_AudioDeviceModule_refcounted* adm);
WEBRTC_EXPORT void
webrtc_PeerConnectionFactoryDependencies_set_event_log_factory(
    struct webrtc_PeerConnectionFactoryDependencies* self,
    struct webrtc_RtcEventLogFactory_unique* event_log_factory);
WEBRTC_EXPORT void
webrtc_PeerConnectionFactoryDependencies_set_audio_encoder_factory(
    struct webrtc_PeerConnectionFactoryDependencies* self,
    struct webrtc_AudioEncoderFactory_refcounted* audio_encoder_factory);
WEBRTC_EXPORT void
webrtc_PeerConnectionFactoryDependencies_set_audio_decoder_factory(
    struct webrtc_PeerConnectionFactoryDependencies* self,
    struct webrtc_AudioDecoderFactory_refcounted* audio_decoder_factory);
WEBRTC_EXPORT void
webrtc_PeerConnectionFactoryDependencies_set_audio_processing_builder(
    struct webrtc_PeerConnectionFactoryDependencies* self,
    struct webrtc_AudioProcessingBuilderInterface_unique*
        audio_processing_builder);
WEBRTC_EXPORT void
webrtc_PeerConnectionFactoryDependencies_set_video_encoder_factory(
    struct webrtc_PeerConnectionFactoryDependencies* self,
    struct webrtc_VideoEncoderFactory_unique* video_encoder_factory);
WEBRTC_EXPORT void
webrtc_PeerConnectionFactoryDependencies_set_video_decoder_factory(
    struct webrtc_PeerConnectionFactoryDependencies* self,
    struct webrtc_VideoDecoderFactory_unique* video_decoder_factory);

WEBRTC_EXPORT void webrtc_EnableMedia(
    struct webrtc_PeerConnectionFactoryDependencies* dependencies);

// -------------------------
// webrtc::PeerConnectionFactoryInterface
// -------------------------

WEBRTC_DECLARE_REFCOUNTED(webrtc_PeerConnectionFactoryInterface);
WEBRTC_EXPORT struct webrtc_PeerConnectionFactoryInterface_refcounted*
webrtc_CreateModularPeerConnectionFactory(
    struct webrtc_PeerConnectionFactoryDependencies* dependencies);
WEBRTC_EXPORT struct webrtc_PeerConnectionFactoryInterface_refcounted*
webrtc_CreateModularPeerConnectionFactoryWithContext(
    struct webrtc_PeerConnectionFactoryDependencies* dependencies,
    struct webrtc_ConnectionContext_refcounted** out_context);

WEBRTC_EXPORT void
webrtc_PeerConnectionFactoryInterface_CreatePeerConnectionOrError(
    struct webrtc_PeerConnectionFactoryInterface* self,
    struct webrtc_PeerConnectionInterface_RTCConfiguration* rtc_config,
    struct webrtc_PeerConnectionDependencies* dependencies,
    struct webrtc_PeerConnectionInterface_refcounted** out_pc,
    struct webrtc_RTCError_unique** out_rtc_error);
WEBRTC_EXPORT void webrtc_PeerConnectionFactoryInterface_CreateVideoTrack(
    struct webrtc_PeerConnectionFactoryInterface* self,
    struct webrtc_VideoTrackSourceInterface_refcounted* source,
    const char* track_id,
    size_t track_id_len,
    struct webrtc_VideoTrackInterface_refcounted** out_track);

WEBRTC_EXPORT void webrtc_PeerConnectionFactoryInterface_CreateAudioSource(
    struct webrtc_PeerConnectionFactoryInterface* self,
    struct webrtc_AudioSourceInterface_refcounted** out_source);

WEBRTC_EXPORT void webrtc_PeerConnectionFactoryInterface_CreateAudioTrack(
    struct webrtc_PeerConnectionFactoryInterface* self,
    struct webrtc_AudioSourceInterface_refcounted* source,
    const char* track_id,
    size_t track_id_len,
    struct webrtc_AudioTrackInterface_refcounted** out_track);
WEBRTC_EXPORT struct webrtc_RtpCapabilities*
webrtc_PeerConnectionFactoryInterface_GetRtpSenderCapabilities(
    struct webrtc_PeerConnectionFactoryInterface* self,
    int media_type);

struct webrtc_PeerConnectionFactoryInterface_Options;
WEBRTC_EXPORT struct webrtc_PeerConnectionFactoryInterface_Options*
webrtc_PeerConnectionFactoryInterface_Options_new();
WEBRTC_EXPORT void webrtc_PeerConnectionFactoryInterface_Options_delete(
    struct webrtc_PeerConnectionFactoryInterface_Options* self);
WEBRTC_EXPORT void
webrtc_PeerConnectionFactoryInterface_Options_set_disable_encryption(
    struct webrtc_PeerConnectionFactoryInterface_Options* self,
    int disable_encryption);
WEBRTC_EXPORT void
webrtc_PeerConnectionFactoryInterface_Options_set_ssl_max_version(
    struct webrtc_PeerConnectionFactoryInterface_Options* self,
    int ssl_max_version);
WEBRTC_EXPORT void webrtc_PeerConnectionFactoryInterface_SetOptions(
    struct webrtc_PeerConnectionFactoryInterface* self,
    struct webrtc_PeerConnectionFactoryInterface_Options* options);
WEBRTC_EXPORT extern const int webrtc_SSL_PROTOCOL_DTLS_12;

#if defined(__cplusplus)
}
#endif
