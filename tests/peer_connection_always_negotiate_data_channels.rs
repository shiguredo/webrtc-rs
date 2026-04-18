use std::sync::mpsc;
use std::time::Duration;

use shiguredo_webrtc::{
    AudioDecoderFactory, AudioDeviceModule, AudioDeviceModuleAudioLayer, AudioEncoderFactory,
    AudioProcessingBuilder, CreateSessionDescriptionObserver,
    CreateSessionDescriptionObserverHandler, Environment, PeerConnection,
    PeerConnectionDependencies, PeerConnectionFactory, PeerConnectionFactoryDependencies,
    PeerConnectionObserver, PeerConnectionObserverHandler, PeerConnectionOfferAnswerOptions,
    PeerConnectionRtcConfiguration, RtcError, SessionDescription, Thread,
};

struct NoopObserverHandler;
impl PeerConnectionObserverHandler for NoopObserverHandler {}

struct OfferHandler {
    tx: mpsc::Sender<Result<String, String>>,
}

impl CreateSessionDescriptionObserverHandler for OfferHandler {
    fn on_success(&mut self, desc: SessionDescription) {
        let sdp = desc
            .to_string()
            .map_err(|e| format!("SessionDescription::to_string failed: {e}"));
        let _ = self.tx.send(sdp);
    }

    fn on_failure(&mut self, err: RtcError) {
        let msg = err.message().unwrap_or_else(|_| "unknown".to_string());
        let _ = self.tx.send(Err(msg));
    }
}

/// 指定 configuration で PeerConnection を作り createOffer を同期完了して SDP を返す。
fn create_offer_sdp(
    factory: &PeerConnectionFactory,
    config: &mut PeerConnectionRtcConfiguration,
) -> String {
    let observer = PeerConnectionObserver::new_with_handler(Box::new(NoopObserverHandler));
    let mut pc_deps = PeerConnectionDependencies::new(&observer);
    let pc = PeerConnection::create(factory, config, &mut pc_deps)
        .expect("PeerConnection の生成に失敗しました");

    let mut opts = PeerConnectionOfferAnswerOptions::new();
    let (tx, rx) = mpsc::channel::<Result<String, String>>();
    let mut obs = CreateSessionDescriptionObserver::new_with_handler(Box::new(OfferHandler { tx }));
    pc.create_offer(&mut obs, &mut opts);
    let sdp = rx
        .recv_timeout(Duration::from_secs(5))
        .expect("createOffer がタイムアウトしました")
        .expect("createOffer が失敗しました");

    drop(obs);
    drop(pc);
    drop(pc_deps);
    sdp
}

#[test]
fn always_negotiate_data_channels_adds_data_section() {
    let dec = AudioDecoderFactory::builtin();
    let enc = AudioEncoderFactory::builtin();
    let apb = AudioProcessingBuilder::new_builtin();
    let mut deps_factory = PeerConnectionFactoryDependencies::new();
    let mut network = Thread::new();
    let mut worker = Thread::new();
    let mut signaling = Thread::new();
    network.start();
    worker.start();
    signaling.start();
    deps_factory.set_network_thread(&network);
    deps_factory.set_worker_thread(&worker);
    deps_factory.set_signaling_thread(&signaling);
    deps_factory.set_audio_encoder_factory(&enc);
    deps_factory.set_audio_decoder_factory(&dec);
    deps_factory.set_audio_processing_builder(apb);
    let env = Environment::new();
    let adm = AudioDeviceModule::new(&env, AudioDeviceModuleAudioLayer::Dummy)
        .expect("AudioDeviceModule の生成に失敗しました");
    deps_factory.set_audio_device_module(&adm);
    deps_factory.enable_media();
    let factory = PeerConnectionFactory::create_modular(&mut deps_factory)
        .expect("PeerConnectionFactory の生成に失敗しました");

    // always_negotiate_data_channels=true かつ DataChannel 未生成でも m=application が含まれる。
    let mut pc_config_on = PeerConnectionRtcConfiguration::new();
    pc_config_on.set_always_negotiate_data_channels(true);
    let sdp_on = create_offer_sdp(&factory, &mut pc_config_on);
    assert!(
        sdp_on.contains("m=application"),
        "always_negotiate_data_channels=true で SDP に m=application が含まれません: {sdp_on}"
    );

    // 対照実験: デフォルト (false) で DataChannel 未生成なら m=application は含まれない。
    let mut pc_config_off = PeerConnectionRtcConfiguration::new();
    let sdp_off = create_offer_sdp(&factory, &mut pc_config_off);
    assert!(
        !sdp_off.contains("m=application"),
        "always_negotiate_data_channels=false で SDP に m=application が含まれました: {sdp_off}"
    );

    drop(factory);
    drop(deps_factory);
    drop(adm);
    drop(env);
    network.stop();
    worker.stop();
    signaling.stop();
}
