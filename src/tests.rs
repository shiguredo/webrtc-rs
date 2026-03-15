use super::*;
use std::ptr::NonNull;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::time::Duration;

#[test]
fn create_and_drop_environment() {
    let _env = Environment::new();
}

#[test]
fn cxx_string_round_trip() {
    let mut s = CxxString::from_str("hello");
    assert_eq!(s.len(), 5);
    assert_eq!(
        s.to_string().expect("CxxString の変換に失敗しました"),
        "hello"
    );

    s.append(" world");
    assert_eq!(
        s.to_string().expect("CxxString の変換に失敗しました"),
        "hello world"
    );

    let r = CxxStringRef::from_ptr(NonNull::new(s.as_ptr()).unwrap());
    assert_eq!(r.len(), 11);
    assert_eq!(
        r.to_string().expect("CxxStringRef の変換に失敗しました"),
        "hello world"
    );
}

#[test]
fn time_millis_moves_forward() {
    let start = time_millis();
    std::thread::sleep(Duration::from_millis(10));
    let end = time_millis();
    assert!(
        end >= start,
        "time_millis が単調増加していません: start={start}, end={end}"
    );
}

#[test]
fn random_string_has_requested_length() {
    let s = random_string(8);
    assert_eq!(s.len(), 8);
}

#[test]
fn random_bytes_length_matches() {
    let b = random_bytes(16);
    assert_eq!(b.len(), 16);
}

#[test]
fn timestamp_aligner_translates() {
    let mut aligner = TimestampAligner::new();
    let base = aligner.translate(1_000_000, 2_000_000);
    let later = aligner.translate(2_000_000, 3_000_000);
    assert!(
        later >= base,
        "TimestampAligner の結果が期待と異なります: base={base}, later={later}"
    );
}

#[test]
fn string_vector_push_and_get() {
    let mut vec = StringVector::new(0);
    let hello = CxxString::from_str("hello");
    let world = CxxString::from_str("world");
    vec.push(&hello);
    vec.push(&world);

    assert_eq!(vec.len(), 2);
    assert_eq!(vec.get(0).expect("0 番目の取得に失敗しました"), "hello");
    assert_eq!(vec.get(1).expect("1 番目の取得に失敗しました"), "world");
}

#[test]
fn sdp_type_round_trip() {
    let offer = SdpType::Offer;
    let val = offer.to_int();
    let back = SdpType::from_int(val);
    assert_eq!(back, SdpType::Offer);
}

#[test]
fn media_type_constants() {
    assert_eq!(
        MediaType::from_int(MediaType::Audio.to_int()),
        MediaType::Audio
    );
    assert_eq!(
        MediaType::from_int(MediaType::Video.to_int()),
        MediaType::Video
    );
}

#[test]
fn session_description_to_string() {
    // datachannel 用の最小構成 SDP を使う。
    let sdp = "v=0\r\n\
                   o=- 0 0 IN IP4 127.0.0.1\r\n\
                   s=-\r\n\
                   t=0 0\r\n\
                   a=group:BUNDLE 0\r\n\
                   m=application 9 UDP/DTLS/SCTP webrtc-datachannel\r\n\
                   c=IN IP4 0.0.0.0\r\n\
                   a=mid:0\r\n\
                   a=sctp-port:5000\r\n\
                   a=max-message-size:262144\r\n";
    let desc = SessionDescription::new(SdpType::Offer, sdp)
        .expect("SessionDescription の生成に失敗しました");
    assert_eq!(desc.sdp_type(), SdpType::Offer);
    let out = desc.to_string().expect("SDP の文字列化に失敗しました");
    assert!(
        out.contains("m=application"),
        "SDP に datachannel 用 m=application が含まれていません: {out}"
    );
}

#[test]
fn sdp_video_format_with_parameters() {
    let mut fmt = SdpVideoFormat::new_with_parameters(
        "VP8",
        &std::collections::HashMap::from([
            (String::from("profile-id"), String::from("0")),
            (String::from("level"), String::from("3.1")),
        ]),
        &[ScalabilityMode::L1T1, ScalabilityMode::L1T2],
    );
    let params = fmt.parameters_mut();
    assert_eq!(params.len(), 2);

    let mut found = std::collections::HashMap::new();
    for (k, v) in params.iter() {
        found.insert(k, v);
    }
    assert_eq!(found.get("profile-id").map(String::as_str), Some("0"));
    assert_eq!(found.get("level").map(String::as_str), Some("3.1"));
    assert_eq!(
        fmt.scalability_modes(),
        vec![ScalabilityMode::L1T1, ScalabilityMode::L1T2]
    );

    let other = SdpVideoFormat::new_with_parameters(
        "VP8",
        &std::collections::HashMap::from([
            (String::from("profile-id"), String::from("0")),
            (String::from("level"), String::from("3.1")),
        ]),
        &[ScalabilityMode::L1T1, ScalabilityMode::L1T2],
    );

    assert!(fmt.is_equal(other.as_ref()));

    let mut cloned = fmt.clone();
    assert!(fmt.is_equal(cloned.as_ref()));
    {
        let mut params = cloned.parameters_mut();
        params.set("packetization-mode", "1");
    }
    let mut has_packetization_mode = false;
    for (k, _) in fmt.parameters_mut().iter() {
        if k == "packetization-mode" {
            has_packetization_mode = true;
            break;
        }
    }
    assert!(
        !has_packetization_mode,
        "clone への変更が元の SdpVideoFormat に影響しています"
    );
}

#[test]
fn sdp_video_format_new_has_empty_scalability_modes() {
    let fmt = SdpVideoFormat::new("VP8");
    assert!(fmt.scalability_modes().is_empty());
}

#[test]
fn scalability_mode_round_trip() {
    let mode = ScalabilityMode::L2T2;
    assert_eq!(
        mode.as_str()
            .expect("ScalabilityMode の文字列化に失敗しました"),
        "L2T2"
    );
}

#[test]
fn i420_buffer_and_video_frame() {
    let mut buf = I420Buffer::new(4, 4);
    buf.fill_y(0x10);
    buf.fill_uv(0x80, 0x90);

    let frame = VideoFrame::from_i420(&buf, 12345, 0);
    assert_eq!(frame.width(), 4);
    assert_eq!(frame.height(), 4);
    assert_eq!(frame.timestamp_us(), 12345);

    let copied = frame.buffer();
    assert_eq!(copied.y_data()[0], 0x10);
}

#[test]
fn i420_buffer_mutable_planes_and_video_frame_rtp_timestamp() {
    let mut buf = I420Buffer::new(4, 4);
    buf.y_data_mut().fill(0x11);
    buf.u_data_mut().fill(0x22);
    buf.v_data_mut().fill(0x33);
    assert!(buf.y_data().iter().all(|&v| v == 0x11));
    assert!(buf.u_data().iter().all(|&v| v == 0x22));
    assert!(buf.v_data().iter().all(|&v| v == 0x33));

    let frame = VideoFrame::from_i420(&buf, 12345, 67890);
    assert_eq!(frame.timestamp_us(), 12345);
    assert_eq!(frame.rtp_timestamp(), 67890);
    assert_eq!(frame.as_ref().rtp_timestamp(), 67890);
}

#[test]
fn abgr_to_i420_conversion() {
    // 2x2 ピクセル、ABGR = 0xff804020 (B=0x20, G=0x40, R=0x80, A=0xff)
    let pixel = [0x20u8, 0x40, 0x80, 0xff];
    let mut src = Vec::new();
    for _ in 0..4 {
        src.extend_from_slice(&pixel);
    }
    let buf = abgr_to_i420(&src, 2, 2).expect("abgr_to_i420 の変換に失敗しました");
    // 単色なので Y/U/V は全て同一値になるはず。
    assert!(buf.y_data().iter().all(|&v| v == buf.y_data()[0]));
    assert!(buf.u_data().iter().all(|&v| v == buf.u_data()[0]));
    assert!(buf.v_data().iter().all(|&v| v == buf.v_data()[0]));
}

#[test]
fn convert_from_i420_argb_conversion() {
    let mut src = I420Buffer::new(2, 2);
    src.fill_y(0x30);
    src.fill_uv(0x80, 0x80);

    let dst = convert_from_i420(&src, LibyuvFourcc::Argb)
        .expect("convert_from_i420(Argb) の変換に失敗しました");
    assert_eq!(dst.len(), 2 * 2 * 4);
}

#[test]
fn i420_to_nv12_round_trip() {
    let width = 4;
    let height = 4;
    let mut src = I420Buffer::new(width, height);
    for (i, p) in src.y_data_mut().iter_mut().enumerate() {
        *p = (i as u8).wrapping_mul(3);
    }
    for (i, p) in src.u_data_mut().iter_mut().enumerate() {
        *p = 0x40u8.wrapping_add(i as u8);
    }
    for (i, p) in src.v_data_mut().iter_mut().enumerate() {
        *p = 0x80u8.wrapping_add(i as u8);
    }

    let nv12 = i420_to_nv12(&src).expect("i420_to_nv12 の変換に失敗しました");
    let y_size = (width * height) as usize;
    let (y, uv) = nv12.split_at(y_size);
    let restored = nv12_to_i420(y, width, uv, width, width, height)
        .expect("nv12_to_i420 の逆変換に失敗しました");

    assert_eq!(src.y_data(), restored.y_data());
    assert_eq!(src.u_data(), restored.u_data());
    assert_eq!(src.v_data(), restored.v_data());
}

#[test]
fn logging_functions_are_callable() {
    // severity は 0 にしておく。実際のログ内容は検証しない。
    log::log_to_debug(log::Severity::Info);
    log::enable_timestamps();
    log::enable_threads();
    log::print(log::Severity::Info, "webrtc-c", 0, "log test");
}

#[test]
fn thread_blocking_call_runs() {
    let mut thread = Thread::new();
    thread.start();
    let result = thread.blocking_call(|| 42);
    assert_eq!(result, 42);

    // () 戻り値のパスも通す
    thread.blocking_call(|| {});
    thread.stop();
}

#[test]
fn thread_sleep_ms_runs() {
    thread_sleep_ms(1);
}

#[test]
fn builtin_audio_factories_create() {
    let dec = AudioDecoderFactory::builtin();
    assert!(!dec.as_ptr().is_null());
    let enc = AudioEncoderFactory::builtin();
    assert!(!enc.as_ptr().is_null());

    // AudioProcessingBuilder も生成確認する。
    let apb = AudioProcessingBuilder::new_builtin();
    assert!(!apb.as_ptr().is_null());

    // PeerConnectionFactoryDependencies を組み立てて EnableMedia まで呼ぶ。
    let mut deps = PeerConnectionFactoryDependencies::new();
    let mut network = Thread::new();
    let mut worker = Thread::new();
    let mut signaling = Thread::new();
    network.start();
    worker.start();
    signaling.start();
    deps.set_network_thread(&network);
    deps.set_worker_thread(&worker);
    deps.set_signaling_thread(&signaling);
    deps.set_audio_encoder_factory(&enc);
    deps.set_audio_decoder_factory(&dec);
    deps.set_audio_processing_builder(apb);
    // Dummy ADM を設定してメディア初期化を通す。
    let env = Environment::new();
    let adm = AudioDeviceModule::new(&env, AudioDeviceModuleAudioLayer::Dummy)
        .expect("AudioDeviceModule の生成に失敗しました");
    deps.set_audio_device_module(&adm);
    deps.enable_media();
    assert!(!deps.as_ptr().is_null());
    drop(deps);
    network.stop();
    worker.stop();
    signaling.stop();
}

#[test]
fn audio_device_module_recording_device_name_roundtrip() {
    struct TestAudioDeviceModuleHandler {
        name: String,
        guid: String,
    }

    impl AudioDeviceModuleHandler for TestAudioDeviceModuleHandler {
        fn init(&self) -> i32 {
            0
        }

        fn recording_devices(&self) -> i16 {
            1
        }

        fn recording_device_name(&self, index: u16) -> Option<(String, String)> {
            if index == 0 {
                Some((self.name.clone(), self.guid.clone()))
            } else {
                None
            }
        }
    }

    fn make_ascii_string(len: usize) -> String {
        (0..len).map(|i| (b'a' + (i % 26) as u8) as char).collect()
    }

    let lengths = [0usize, 1, 2, 3, 7, 31, 63, 64];
    for &len in &lengths {
        let name = make_ascii_string(len);
        let guid = make_ascii_string(64usize.saturating_sub(len));
        let expected_name = name.clone();
        let expected_guid = guid.clone();
        let mut adm = AudioDeviceModule::new_with_handler(Box::new(TestAudioDeviceModuleHandler {
            name,
            guid,
        }));
        adm.init().expect("AudioDeviceModule::init が失敗しました");
        assert_eq!(adm.recording_devices(), 1);
        let (got_name, got_guid) = adm
            .recording_device_name(0)
            .expect("recording_device_name が失敗しました");
        assert_eq!(got_name, expected_name);
        assert_eq!(got_guid, expected_guid);
    }
}

#[test]
fn adapted_video_track_source() {
    let mut src = AdaptedVideoTrackSource::new();
    let adapted = src.adapt_frame(640, 480, 1_000_000);
    // applied が false の場合でもサイズ情報が得られることを確認する。
    assert!(adapted.size.adapted_width >= 0);
    assert!(adapted.size.adapted_height >= 0);

    let buf = I420Buffer::new(2, 2);
    let frame = VideoFrame::from_i420(&buf, 2_000_000, 0);
    src.on_frame(&frame);
}

#[test]
fn peer_connection_factory_and_capabilities() {
    let dec = AudioDecoderFactory::builtin();
    let enc = AudioEncoderFactory::builtin();
    let apb = AudioProcessingBuilder::new_builtin();

    // PeerConnectionFactoryDependencies を組み立てる。スレッドのライフサイクルはここで管理する。
    let mut deps = PeerConnectionFactoryDependencies::new();
    let mut network = Thread::new();
    let mut worker = Thread::new();
    let mut signaling = Thread::new();
    network.start();
    worker.start();
    signaling.start();
    deps.set_network_thread(&network);
    deps.set_worker_thread(&worker);
    deps.set_signaling_thread(&signaling);
    deps.set_audio_encoder_factory(&enc);
    deps.set_audio_decoder_factory(&dec);
    deps.set_audio_processing_builder(apb);
    let event_log = RtcEventLogFactory::new();
    deps.set_event_log_factory(event_log);
    let env = Environment::new();
    let adm = AudioDeviceModule::new(&env, AudioDeviceModuleAudioLayer::Dummy)
        .expect("AudioDeviceModule の生成に失敗しました");
    deps.set_audio_device_module(&adm);
    deps.enable_media();

    // Factory を生成し、オプションと RTP 能力を取得する。
    let (mut factory, context) = PeerConnectionFactory::create_modular_with_context(&mut deps)
        .expect("PeerConnectionFactory と ConnectionContext の生成に失敗しました");
    let mut opts = PeerConnectionFactoryOptions::new();
    opts.set_disable_encryption(false);
    let dtls12 = unsafe { ffi::webrtc_SSL_PROTOCOL_DTLS_12 };
    opts.set_ssl_max_version(dtls12);
    factory.set_options(&opts);

    let network_manager = context.default_network_manager();
    let socket_factory = context.default_socket_factory();
    assert!(!network_manager.as_ptr().is_null());
    assert!(!socket_factory.as_ptr().is_null());

    let caps = factory.get_rtp_sender_capabilities(MediaType::Audio);
    assert!(caps.codec_len() >= 0);
    let codecs = caps.codecs();
    assert_eq!(codecs.len() as i32, caps.codec_len());
    if !codecs.is_empty() {
        let first = codecs.get(0).expect("先頭 codec の取得に失敗しました");
        assert!(first.name().is_ok());
    }

    drop(caps);
    drop(context);
    drop(factory);
    drop(deps);
    network.stop();
    worker.stop();
    signaling.stop();
}

#[test]
fn rtc_configuration_and_ice_server() {
    let mut config = PeerConnectionRtcConfiguration::new();
    config.set_type(IceTransportsType::Relay);
    let mut server = IceServer::new();
    server.set_username("user");
    server.set_password("pass");
    server.set_tls_cert_policy(TlsCertPolicy::InsecureNoCheck);
    server.add_url("stun:192.0.2.1:3478");

    {
        let mut servers = config.servers();
        let len_before = servers.len();
        servers.push(&server);
        assert_eq!(servers.len(), len_before + 1);
    }

    // 所有ベクタでも同じ挙動になることを確認しておく。
    let mut owned = IceServerVector::new(0);
    let len_before = owned.len();
    owned.push(&server);
    assert_eq!(owned.len(), len_before + 1);
}

#[test]
fn tls_cert_policy_round_trip() {
    assert_eq!(
        TlsCertPolicy::from_int(TlsCertPolicy::Secure.to_int()),
        TlsCertPolicy::Secure
    );
    assert_eq!(
        TlsCertPolicy::from_int(TlsCertPolicy::InsecureNoCheck.to_int()),
        TlsCertPolicy::InsecureNoCheck
    );
    assert_eq!(
        TlsCertPolicy::from_int(123456),
        TlsCertPolicy::Unknown(123456)
    );
}

#[test]
fn create_modular_with_context_returns_default_network_objects() {
    let dec = AudioDecoderFactory::builtin();
    let enc = AudioEncoderFactory::builtin();
    let apb = AudioProcessingBuilder::new_builtin();

    let mut deps = PeerConnectionFactoryDependencies::new();
    let mut network = Thread::new();
    let mut worker = Thread::new();
    let mut signaling = Thread::new();
    network.start();
    worker.start();
    signaling.start();
    deps.set_network_thread(&network);
    deps.set_worker_thread(&worker);
    deps.set_signaling_thread(&signaling);
    deps.set_audio_encoder_factory(&enc);
    deps.set_audio_decoder_factory(&dec);
    deps.set_audio_processing_builder(apb);
    let env = Environment::new();
    let adm = AudioDeviceModule::new(&env, AudioDeviceModuleAudioLayer::Dummy)
        .expect("AudioDeviceModule の生成に失敗しました");
    deps.set_audio_device_module(&adm);
    deps.enable_media();

    let (factory, context) = PeerConnectionFactory::create_modular_with_context(&mut deps)
        .expect("PeerConnectionFactory と ConnectionContext の生成に失敗しました");
    let network_manager = context.default_network_manager();
    let socket_factory = context.default_socket_factory();
    assert!(!network_manager.as_ptr().is_null());
    assert!(!socket_factory.as_ptr().is_null());
    assert!(!factory.as_ptr().is_null());

    drop(context);
    drop(factory);
    drop(deps);
    network.stop();
    worker.stop();
    signaling.stop();
}

#[test]
fn rtp_codec_capability_vector() {
    let mut cap = RtpCodecCapability::new();
    cap.set_kind(MediaType::Audio);
    cap.set_name("opus");
    cap.set_clock_rate(Some(48_000));
    {
        let mut params = cap.parameters();
        params.set("stereo", "1");
        assert!(params.iter().any(|(k, v)| k == "stereo" && v == "1"));
    }

    let mut vec = RtpCodecCapabilityVector::new(0);
    let len_before = vec.len();
    vec.push(&cap.as_ref());
    assert_eq!(vec.len(), len_before + 1);
    vec.resize(2);
    let mut cap2 = RtpCodecCapability::new();
    cap2.set_kind(MediaType::Audio);
    cap2.set_name("PCMU");
    cap2.set_clock_rate(Some(8_000));
    assert!(vec.set(1, &cap2.as_ref()));
    assert_eq!(vec.len(), 2);
    let first = vec.get(0).expect("先頭 codec の取得に失敗しました");
    let second = vec.get(1).expect("2 番目 codec の取得に失敗しました");
    assert_eq!(
        first.name().expect("1 番目 codec 名の取得に失敗しました"),
        "opus"
    );
    assert_eq!(
        second.name().expect("2 番目 codec 名の取得に失敗しました"),
        "PCMU"
    );
}

#[test]
fn rtp_encoding_parameters_and_transceiver_init() {
    let mut codec = RtpCodec::new();
    codec.set_kind(MediaType::Audio);
    codec.set_name("opus");
    codec.set_clock_rate(Some(48_000));
    codec.set_num_channels(Some(2));

    let mut enc = RtpEncodingParameters::new();
    enc.set_rid("f");
    enc.set_ssrc(Some(1234));
    enc.set_max_bitrate_bps(Some(1_500_000));
    enc.set_min_bitrate_bps(Some(100_000));
    enc.set_max_framerate(Some(30.0));
    enc.set_scale_resolution_down_by(Some(2.0));
    let mut resolution = Resolution::new();
    resolution.set_width(960);
    resolution.set_height(540);
    enc.set_scale_resolution_down_to(Some(&resolution));
    enc.set_active(false);
    enc.set_adaptive_ptime(true);
    enc.set_scalability_mode(Some("L1T3"));
    enc.set_codec(Some(&codec));
    assert_eq!(enc.rid().expect("rid の取得に失敗しました"), "f");
    assert_eq!(enc.ssrc(), Some(1234));
    assert_eq!(enc.max_bitrate_bps(), Some(1_500_000));
    assert_eq!(enc.min_bitrate_bps(), Some(100_000));
    assert_eq!(enc.max_framerate(), Some(30.0));
    assert_eq!(enc.scale_resolution_down_by(), Some(2.0));
    let got_resolution = enc
        .scale_resolution_down_to()
        .expect("scale_resolution_down_to の取得に失敗しました");
    assert_eq!(got_resolution.width(), 960);
    assert_eq!(got_resolution.height(), 540);
    assert!(!enc.active());
    assert!(enc.adaptive_ptime());
    assert_eq!(
        enc.scalability_mode()
            .expect("scalability_mode が未設定でした")
            .expect("scalability_mode の取得に失敗しました"),
        "L1T3".to_string()
    );
    let enc_codec = enc.codec().expect("codec の取得に失敗しました");
    assert_eq!(
        enc_codec.name().expect("codec 名の取得に失敗しました"),
        "opus"
    );
    assert_eq!(enc_codec.clock_rate(), Some(48_000));
    assert_eq!(enc_codec.num_channels(), Some(2));
    enc.set_scalability_mode(None);
    assert!(enc.scalability_mode().is_none());
    enc.set_codec(None);
    assert!(enc.codec().is_none());

    let mut vec = RtpEncodingParametersVector::new(0);
    vec.push(&enc);
    assert_eq!(vec.len(), 1);
    vec.resize(2);
    let mut enc2 = RtpEncodingParameters::new();
    enc2.set_rid("h");
    assert!(vec.set(1, &enc2));
    assert_eq!(vec.len(), 2);
    let cloned = vec.clone_self();
    assert_eq!(cloned.len(), vec.len());

    let mut init = RtpTransceiverInit::new();
    init.set_direction(RtpTransceiverDirection::SendOnly);
    init.set_send_encodings(&vec);
    let mut stream_ids = init.stream_ids();
    stream_ids.push(&CxxString::from_str("stream-1"));
    assert_eq!(stream_ids.len(), 1);

    let mut offer = PeerConnectionOfferAnswerOptions::new();
    offer.set_offer_to_receive_audio(1);
    offer.set_offer_to_receive_video(1);
    offer.set_voice_activity_detection(true);
    offer.set_ice_restart(false);
    offer.set_use_rtp_mux(true);
    offer.set_raw_packetization_for_video(false);
    offer.set_num_simulcast_layers(0);
    offer.set_use_obsolete_sctp_sdp(false);
    assert_eq!(offer.offer_to_receive_audio(), 1);
    assert_eq!(offer.offer_to_receive_video(), 1);
    assert!(offer.voice_activity_detection());
    assert!(offer.use_rtp_mux());
}

#[test]
fn rtp_parameters_round_trip() {
    let mut params = RtpParameters::new();
    params.set_transaction_id("tx-1");
    params.set_mid("video-0");
    assert_eq!(
        params
            .transaction_id()
            .expect("transaction_id の取得に失敗しました"),
        "tx-1"
    );
    assert_eq!(params.mid().expect("mid の取得に失敗しました"), "video-0");

    let mut enc = RtpEncodingParameters::new();
    enc.set_rid("r0");
    enc.set_max_bitrate_bps(Some(500_000));
    let mut encodings = RtpEncodingParametersVector::new(0);
    encodings.push(&enc);
    params.set_encodings(&encodings);

    let got = params.encodings();
    assert_eq!(got.len(), 1);
    let first = got.get(0).expect("encodings の取得に失敗しました");
    assert_eq!(first.rid().expect("rid の取得に失敗しました"), "r0");

    params.set_degradation_preference(Some(DegradationPreference::Balanced));
    assert_eq!(
        params.degradation_preference(),
        Some(DegradationPreference::Balanced)
    );
    params.set_degradation_preference(None);
    assert_eq!(params.degradation_preference(), None);
}

#[test]
fn rtp_sender_get_set_parameters() {
    let dec_audio = AudioDecoderFactory::builtin();
    let enc_audio = AudioEncoderFactory::builtin();
    let enc_video = VideoEncoderFactory::builtin();
    let dec_video = VideoDecoderFactory::builtin();
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
    deps_factory.set_audio_encoder_factory(&enc_audio);
    deps_factory.set_audio_decoder_factory(&dec_audio);
    deps_factory.set_video_encoder_factory(enc_video);
    deps_factory.set_video_decoder_factory(dec_video);
    deps_factory.set_audio_processing_builder(apb);
    let env = Environment::new();
    let adm = AudioDeviceModule::new(&env, AudioDeviceModuleAudioLayer::Dummy)
        .expect("AudioDeviceModule の生成に失敗しました");
    deps_factory.set_audio_device_module(&adm);
    deps_factory.enable_media();
    let factory = PeerConnectionFactory::create_modular(&mut deps_factory)
        .expect("PeerConnectionFactory の生成に失敗しました");

    let source = AdaptedVideoTrackSource::new();
    let vts = source.cast_to_video_track_source();
    let track = factory
        .create_video_track(&vts, "video-track-1")
        .expect("VideoTrack の生成に失敗しました");

    let mut pc_config = PeerConnectionRtcConfiguration::new();
    let observer = PeerConnectionObserver::new_with_handler(Box::new(()));
    let mut pc_deps = PeerConnectionDependencies::new(&observer);
    let pc = PeerConnection::create(&factory, &mut pc_config, &mut pc_deps)
        .expect("PeerConnection の生成に失敗しました");

    let stream_track = track.cast_to_media_stream_track();
    let mut stream_ids = StringVector::new(0);
    stream_ids.push(&CxxString::from_str("stream-0"));
    let mut sender = pc
        .add_track(&stream_track, &stream_ids)
        .expect("AddTrack が失敗しました");

    let params = sender.get_parameters();
    sender
        .set_parameters(&params)
        .expect("set_parameters が失敗しました");

    drop(sender);
    drop(stream_track);
    drop(pc);
    drop(track);
    drop(vts);
    drop(source);
    drop(pc_deps);
    drop(factory);
    drop(deps_factory);
    drop(adm);
    drop(env);
    network.stop();
    worker.stop();
    signaling.stop();
}

#[test]
fn peer_connection_create_and_transceiver() {
    // Factory を組み立てる。
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

    // PC 用の構成と observer/dependencies を準備する。
    let mut pc_config = PeerConnectionRtcConfiguration::new();
    let observer = PeerConnectionObserver::new_with_handler(Box::new(()));
    let mut pc_deps = PeerConnectionDependencies::new(&observer);

    // PeerConnection を生成し、取得できることを確認する。
    let pc = PeerConnection::create(&factory, &mut pc_config, &mut pc_deps)
        .expect("PeerConnection の生成に失敗しました");
    assert!(!pc.as_ptr().is_null());

    drop(pc);
    drop(pc_deps);
    drop(factory);
    drop(deps_factory);
    network.stop();
    worker.stop();
    signaling.stop();
}

#[test]
fn peer_connection_create_with_proxy_allocator() {
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
    let (factory, context) = PeerConnectionFactory::create_modular_with_context(&mut deps_factory)
        .expect("PeerConnectionFactory と ConnectionContext の生成に失敗しました");

    let network_manager = context.default_network_manager();
    let socket_factory = context.default_socket_factory();
    assert!(!network_manager.as_ptr().is_null());
    assert!(!socket_factory.as_ptr().is_null());

    let mut pc_config = PeerConnectionRtcConfiguration::new();
    let observer = PeerConnectionObserver::new_with_handler(Box::new(()));
    let mut pc_deps = PeerConnectionDependencies::new(&observer);
    pc_deps.set_proxy(
        network_manager,
        socket_factory,
        "127.0.0.1",
        8080,
        "user",
        "pass",
        "shiguredo_webrtc test",
    );
    let pc = PeerConnection::create(&factory, &mut pc_config, &mut pc_deps)
        .expect("Proxy 設定付き PeerConnection の生成に失敗しました");
    assert!(!pc.as_ptr().is_null());

    drop(pc);
    drop(pc_deps);
    drop(context);
    drop(factory);
    drop(deps_factory);
    network.stop();
    worker.stop();
    signaling.stop();
}

#[test]
fn video_track_and_transceiver_with_track() {
    let dec_audio = AudioDecoderFactory::builtin();
    let enc_audio = AudioEncoderFactory::builtin();
    let enc_video = VideoEncoderFactory::builtin();
    let dec_video = VideoDecoderFactory::builtin();
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
    deps_factory.set_audio_encoder_factory(&enc_audio);
    deps_factory.set_audio_decoder_factory(&dec_audio);
    deps_factory.set_video_encoder_factory(enc_video);
    deps_factory.set_video_decoder_factory(dec_video);
    deps_factory.set_audio_processing_builder(apb);
    let env = Environment::new();
    let adm = AudioDeviceModule::new(&env, AudioDeviceModuleAudioLayer::Dummy)
        .expect("AudioDeviceModule の生成に失敗しました");
    deps_factory.set_audio_device_module(&adm);
    deps_factory.enable_media();
    let factory = PeerConnectionFactory::create_modular(&mut deps_factory)
        .expect("PeerConnectionFactory の生成に失敗しました");

    // VideoTrack を生成する。
    let mut source = AdaptedVideoTrackSource::new();
    let vts = source.cast_to_video_track_source();
    let track = factory
        .create_video_track(&vts, "video-track-0")
        .expect("VideoTrack の生成に失敗しました");
    // ついでにフレーム投入 API も呼んでおく。
    let buf = I420Buffer::new(2, 2);
    let frame = VideoFrame::from_i420(&buf, 1_000_000, 0);
    source.on_frame(&frame);

    // PeerConnection を作成し、トラック付きで transceiver を追加する。
    let mut pc_config = PeerConnectionRtcConfiguration::new();
    let observer = PeerConnectionObserver::new_with_handler(Box::new(()));
    let mut pc_deps = PeerConnectionDependencies::new(&observer);
    let pc = PeerConnection::create(&factory, &mut pc_config, &mut pc_deps)
        .expect("PeerConnection の生成に失敗しました");

    let mut init = RtpTransceiverInit::new();
    init.set_direction(RtpTransceiverDirection::SendOnly);
    pc.add_transceiver_with_track(&track, &mut init)
        .expect("AddTransceiverWithTrack が失敗しました");

    // webrtc オブジェクトを先に解放してからスレッドを停止する。
    drop(pc);
    drop(track);
    drop(vts);
    drop(source);
    drop(pc_deps);
    drop(factory);
    drop(deps_factory);
    drop(adm);
    drop(env);
    network.stop();
    worker.stop();
    signaling.stop();
}

#[test]
fn peer_connection_observer_and_dependencies() {
    let observer = PeerConnectionObserver::new_with_handler(Box::new(()));
    let deps = PeerConnectionDependencies::new(&observer);
    assert!(!deps.as_ptr().is_null());
    drop(deps);
}

#[test]
fn peer_connection_dependencies_set_tls_cert_verifier() {
    struct TestVerifier {
        dropped: Arc<AtomicBool>,
    }

    impl SSLCertificateVerifierHandler for TestVerifier {
        fn verify_chain(&mut self, _chain: SSLCertChainRef<'_>) -> bool {
            true
        }
    }

    impl Drop for TestVerifier {
        fn drop(&mut self) {
            self.dropped.store(true, Ordering::SeqCst);
        }
    }

    let dropped = Arc::new(AtomicBool::new(false));
    let observer = PeerConnectionObserver::new_with_handler(Box::new(()));
    let mut deps = PeerConnectionDependencies::new(&observer);
    let verifier = SSLCertificateVerifier::new_with_handler(Box::new(TestVerifier {
        dropped: dropped.clone(),
    }));
    deps.set_tls_cert_verifier(verifier);

    drop(deps);
    assert!(
        dropped.load(Ordering::SeqCst),
        "SSLCertificateVerifierHandler が解放されていません"
    );
}

#[test]
fn create_and_set_local_description_observers() {
    let _create_obs = CreateSessionDescriptionObserver::new_with_handler(Box::new(()));
    let _set_local = SetLocalDescriptionObserver::new_with_handler(Box::new(()));
    let _set_remote = SetRemoteDescriptionObserver::new_with_handler(Box::new(()));
}

// VideoEncoderFactory でカスタムエンコーダーを登録して encode を呼び、
// encode callback が呼ばれることを確認する。
#[test]
fn custom_video_encoder_factory_create_and_encode_calls_callbacks() {
    struct TestVideoEncoderHandler {
        encode_count: i32,
    }
    impl VideoEncoderHandler for TestVideoEncoderHandler {
        fn encode(
            &mut self,
            _frame: VideoFrameRef<'_>,
            frame_types: Option<VideoFrameTypeVectorRef<'_>>,
        ) -> VideoCodecStatus {
            let frame_types = frame_types.expect("frame_types が None です");
            assert_eq!(frame_types.len(), 2);
            assert_eq!(frame_types.get(0), Some(VideoFrameType::Key));
            assert_eq!(frame_types.get(1), Some(VideoFrameType::Delta));
            self.encode_count += 1;
            VideoCodecStatus::Unknown(self.encode_count)
        }
    }

    struct TestVideoEncoderFactoryHandler {
        created: bool,
    }
    impl VideoEncoderFactoryHandler for TestVideoEncoderFactoryHandler {
        fn create(
            &mut self,
            env: EnvironmentRef<'_>,
            format: SdpVideoFormatRef<'_>,
        ) -> Option<Box<dyn VideoEncoderHandler>> {
            assert!(!env.as_ptr().is_null());
            assert_eq!(
                format
                    .name()
                    .expect("SdpVideoFormatRef::name に失敗しました"),
                "VP8"
            );
            if self.created {
                return None;
            }
            self.created = true;
            Some(Box::new(TestVideoEncoderHandler { encode_count: 0 }))
        }
    }

    let factory = VideoEncoderFactory::new_with_handler(Box::new(TestVideoEncoderFactoryHandler {
        created: false,
    }));
    let env = Environment::new();
    let format = SdpVideoFormat::new("VP8");
    let mut encoder = factory
        .create(env.as_ref(), format.as_ref())
        .expect("custom encoder の作成に失敗しました");

    let buffer = I420Buffer::new(2, 2);
    let frame = VideoFrame::from_i420(&buffer, 123, 0);
    let mut frame_types = VideoFrameTypeVector::new(0);
    frame_types.push(VideoFrameType::Key);
    frame_types.push(VideoFrameType::Delta);

    assert_eq!(
        encoder.encode(frame.as_ref(), Some(frame_types.as_ref())),
        VideoCodecStatus::NoOutput
    );
    assert_eq!(
        encoder.encode(frame.as_ref(), Some(frame_types.as_ref())),
        VideoCodecStatus::Unknown(2)
    );
    assert!(
        factory.create(env.as_ref(), format.as_ref()).is_none(),
        "2 回目の create は None を返す想定です"
    );
}

#[test]
fn video_encoder_factory_get_supported_formats_returns_owned_formats() {
    struct TestVideoEncoderFactoryHandler;
    impl VideoEncoderFactoryHandler for TestVideoEncoderFactoryHandler {
        fn get_supported_formats(&mut self) -> Vec<SdpVideoFormat> {
            let mut h264 = SdpVideoFormat::new("H264");
            h264.parameters_mut().set("profile-level-id", "42e01f");
            let mut vp8 = SdpVideoFormat::new("VP8");
            vp8.parameters_mut().set("x-google-start-bitrate", "300");
            vec![h264, vp8]
        }
    }

    let factory = VideoEncoderFactory::new_with_handler(Box::new(TestVideoEncoderFactoryHandler));
    let mut formats = factory.get_supported_formats();
    assert_eq!(formats.len(), 2);
    assert_eq!(
        formats[0].name().expect("name の取得に失敗しました"),
        "H264"
    );
    assert_eq!(formats[1].name().expect("name の取得に失敗しました"), "VP8");

    let params: std::collections::HashMap<String, String> = formats
        .get_mut(0)
        .expect("先頭フォーマットが存在しません")
        .parameters_mut()
        .iter()
        .collect();
    assert_eq!(
        params.get("profile-level-id").map(String::as_str),
        Some("42e01f")
    );
}

#[test]
fn video_decoder_factory_get_supported_formats_returns_owned_formats() {
    struct TestVideoDecoderFactoryHandler;
    impl VideoDecoderFactoryHandler for TestVideoDecoderFactoryHandler {
        fn get_supported_formats(&mut self) -> Vec<SdpVideoFormat> {
            let mut h264 = SdpVideoFormat::new("H264");
            h264.parameters_mut().set("packetization-mode", "1");
            vec![h264]
        }
    }

    let factory = VideoDecoderFactory::new_with_handler(Box::new(TestVideoDecoderFactoryHandler));
    let mut formats = factory.get_supported_formats();
    assert_eq!(formats.len(), 1);
    assert_eq!(
        formats[0].name().expect("name の取得に失敗しました"),
        "H264"
    );
    let params: std::collections::HashMap<String, String> = formats
        .get_mut(0)
        .expect("先頭フォーマットが存在しません")
        .parameters_mut()
        .iter()
        .collect();
    assert_eq!(
        params.get("packetization-mode").map(String::as_str),
        Some("1")
    );
}

#[test]
fn video_encoder_factory_create_calls_create_callback() {
    struct TestVideoEncoderFactoryHandler {
        called: std::sync::Arc<std::sync::atomic::AtomicBool>,
    }
    impl VideoEncoderFactoryHandler for TestVideoEncoderFactoryHandler {
        fn create(
            &mut self,
            env: EnvironmentRef<'_>,
            format: SdpVideoFormatRef<'_>,
        ) -> Option<Box<dyn VideoEncoderHandler>> {
            self.called.store(true, std::sync::atomic::Ordering::SeqCst);
            assert!(!env.as_ptr().is_null());
            assert_eq!(
                format
                    .name()
                    .expect("SdpVideoFormatRef::name に失敗しました"),
                "H264"
            );
            Some(Box::new(()))
        }
    }

    let called = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let factory = VideoEncoderFactory::new_with_handler(Box::new(TestVideoEncoderFactoryHandler {
        called: called.clone(),
    }));
    let env = Environment::new();
    let format = SdpVideoFormat::new("H264");
    let encoder = factory.create(env.as_ref(), format.as_ref());
    assert!(encoder.is_some(), "create が None を返しました");
    assert!(
        called.load(std::sync::atomic::Ordering::SeqCst),
        "create callback が呼ばれていません"
    );
}

#[test]
fn video_decoder_factory_create_calls_create_callback() {
    struct TestVideoDecoderFactoryHandler {
        called: std::sync::Arc<std::sync::atomic::AtomicBool>,
    }
    impl VideoDecoderFactoryHandler for TestVideoDecoderFactoryHandler {
        fn create(
            &mut self,
            env: EnvironmentRef<'_>,
            format: SdpVideoFormatRef<'_>,
        ) -> Option<Box<dyn VideoDecoderHandler>> {
            self.called.store(true, std::sync::atomic::Ordering::SeqCst);
            assert!(!env.as_ptr().is_null());
            assert_eq!(
                format
                    .name()
                    .expect("SdpVideoFormatRef::name に失敗しました"),
                "H264"
            );
            Some(Box::new(()))
        }
    }

    let called = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let factory = VideoDecoderFactory::new_with_handler(Box::new(TestVideoDecoderFactoryHandler {
        called: called.clone(),
    }));
    let env = Environment::new();
    let format = SdpVideoFormat::new("H264");
    let decoder = factory.create(env.as_ref(), format.as_ref());
    assert!(decoder.is_some(), "create が None を返しました");
    assert!(
        called.load(std::sync::atomic::Ordering::SeqCst),
        "create callback が呼ばれていません"
    );
}

// VideoEncoder で encode を呼び、encoded_image と codec_specific_info を受け取れることを確認する。
#[test]
fn custom_video_encoder_register_and_encode_calls_encoded_image_and_codec_specific_info() {
    #[derive(Default)]
    struct State {
        callback_ptr: Option<VideoEncoderEncodedImageCallbackPtr>,
        register_called: bool,
        encode_called: bool,
        on_encoded_image_called: bool,
        order: Vec<&'static str>,
    }

    #[derive(Clone, Copy)]
    struct StatePtr(*mut State);
    unsafe impl Send for StatePtr {}
    impl StatePtr {
        unsafe fn get_mut<'a>(&self) -> &'a mut State {
            unsafe { &mut *self.0 }
        }
    }

    struct TestVideoEncoderHandler {
        state_ptr: StatePtr,
    }
    impl VideoEncoderHandler for TestVideoEncoderHandler {
        fn register_encode_complete_callback(
            &mut self,
            callback: Option<VideoEncoderEncodedImageCallbackRef<'_>>,
        ) -> VideoCodecStatus {
            let callback = callback.expect("register 側 callback が None です");
            let state = unsafe { self.state_ptr.get_mut() };
            state.register_called = true;
            state.order.push("register");
            state.callback_ptr =
                Some(unsafe { VideoEncoderEncodedImageCallbackPtr::from_ref(callback) });
            VideoCodecStatus::Ok
        }

        fn encode(
            &mut self,
            _frame: VideoFrameRef<'_>,
            _frame_types: Option<VideoFrameTypeVectorRef<'_>>,
        ) -> VideoCodecStatus {
            {
                let state = unsafe { self.state_ptr.get_mut() };
                state.encode_called = true;
                state.order.push("encode");
            }

            let callback_ptr = {
                let state = unsafe { self.state_ptr.get_mut() };
                state
                    .callback_ptr
                    .expect("encode 側 callback_ptr が未設定です")
            };

            let buffer = EncodedImageBuffer::from_bytes(&[1, 2, 3, 4]);
            let mut image = EncodedImage::new();
            image.set_encoded_data(&buffer);
            image.set_rtp_timestamp(12345);
            image.set_encoded_width(640);
            image.set_encoded_height(360);
            image.set_frame_type(VideoFrameType::Key);
            image.set_qp(31);

            let mut codec_specific_info = CodecSpecificInfo::new();
            codec_specific_info.set_codec_type(VideoCodecType::H264);
            codec_specific_info.set_end_of_picture(true);
            codec_specific_info.set_h264_packetization_mode(H264PacketizationMode::SingleNalUnit);
            codec_specific_info.set_h264_temporal_idx(2);
            codec_specific_info.set_h264_base_layer_sync(true);
            codec_specific_info.set_h264_idr_frame(true);

            let result = unsafe {
                callback_ptr.on_encoded_image(image.as_ref(), Some(codec_specific_info.as_ref()))
            };
            assert_eq!(
                result.error(),
                VideoEncoderEncodedImageCallbackResultError::Ok
            );
            assert_eq!(result.frame_id(), 9999);
            assert!(!result.drop_next_frame());
            VideoCodecStatus::Unknown(88)
        }
    }

    struct TestEncodedImageCallbackHandler {
        state_ptr: StatePtr,
    }
    impl VideoEncoderEncodedImageCallbackHandler for TestEncodedImageCallbackHandler {
        fn on_encoded_image(
            &mut self,
            image: EncodedImageRef<'_>,
            codec_specific_info: Option<CodecSpecificInfoRef<'_>>,
        ) -> VideoEncoderEncodedImageCallbackResult {
            let state = unsafe { self.state_ptr.get_mut() };
            state.on_encoded_image_called = true;
            state.order.push("on_encoded_image");

            let encoded_data = image.encoded_data().expect("encoded_data が None です");
            assert_eq!(encoded_data.data(), [1, 2, 3, 4]);
            assert_eq!(encoded_data.data().len(), 4);
            assert_eq!(image.rtp_timestamp(), 12345);
            assert_eq!(image.encoded_width(), 640);
            assert_eq!(image.encoded_height(), 360);
            assert_eq!(image.frame_type(), VideoFrameType::Key);
            assert_eq!(image.qp(), 31);

            let codec_specific_info =
                codec_specific_info.expect("codec_specific_info が None です");
            assert_eq!(codec_specific_info.codec_type(), VideoCodecType::H264);
            assert!(codec_specific_info.end_of_picture());
            assert_eq!(
                codec_specific_info.h264_packetization_mode(),
                H264PacketizationMode::SingleNalUnit
            );
            assert_eq!(codec_specific_info.h264_temporal_idx(), 2);
            assert!(codec_specific_info.h264_base_layer_sync());
            assert!(codec_specific_info.h264_idr_frame());
            VideoEncoderEncodedImageCallbackResult::new_with_frame_id(
                VideoEncoderEncodedImageCallbackResultError::Ok,
                9999,
            )
        }
    }

    let mut state = Box::new(State::default());
    let state_ptr = StatePtr((&mut *state) as *mut State);
    let mut encoder =
        VideoEncoder::new_with_handler(Box::new(TestVideoEncoderHandler { state_ptr }));
    let encoded_image_callback = VideoEncoderEncodedImageCallback::new_with_handler(Box::new(
        TestEncodedImageCallbackHandler { state_ptr },
    ));

    assert_eq!(
        encoder.register_encode_complete_callback(Some(encoded_image_callback.as_ref())),
        VideoCodecStatus::Ok
    );

    let buffer = I420Buffer::new(2, 2);
    let frame = VideoFrame::from_i420(&buffer, 123, 0);
    assert_eq!(
        encoder.encode(frame.as_ref(), None),
        VideoCodecStatus::Unknown(88)
    );

    assert!(state.register_called, "register が呼ばれていません");
    assert!(state.encode_called, "encode が呼ばれていません");
    assert!(
        state.on_encoded_image_called,
        "on_encoded_image が呼ばれていません"
    );
    assert_eq!(
        state.order,
        vec!["register", "encode", "on_encoded_image"],
        "呼び出し順が不正です"
    );
}

// VideoDecoderFactory の create callback と、VideoDecoder の decode callback が呼ばれることを確認する。
#[test]
fn custom_video_decoder_factory_create_and_decode_calls_callbacks() {
    struct TestVideoDecoderHandler {
        decode_count: i32,
    }
    impl VideoDecoderHandler for TestVideoDecoderHandler {
        fn decode(
            &mut self,
            input_image: EncodedImageRef<'_>,
            render_time_ms: i64,
        ) -> VideoCodecStatus {
            assert!(input_image.encoded_data().is_none());
            assert_eq!(render_time_ms, 456);
            self.decode_count += 1;
            VideoCodecStatus::Unknown(self.decode_count)
        }
    }

    struct TestVideoDecoderFactoryHandler {
        created: bool,
    }
    impl VideoDecoderFactoryHandler for TestVideoDecoderFactoryHandler {
        fn create(
            &mut self,
            env: EnvironmentRef<'_>,
            _format: SdpVideoFormatRef<'_>,
        ) -> Option<Box<dyn VideoDecoderHandler>> {
            assert!(!env.as_ptr().is_null());
            if self.created {
                return None;
            }
            self.created = true;
            Some(Box::new(TestVideoDecoderHandler { decode_count: 0 }))
        }
    }

    let factory = VideoDecoderFactory::new_with_handler(Box::new(TestVideoDecoderFactoryHandler {
        created: false,
    }));
    let env = Environment::new();
    let format = SdpVideoFormat::new("VP8");
    let mut decoder = factory
        .create(env.as_ref(), format.as_ref())
        .expect("custom decoder の作成に失敗しました");
    let image = EncodedImage::new();

    assert_eq!(
        decoder.decode(image.as_ref(), 456),
        VideoCodecStatus::NoOutput
    );
    assert_eq!(
        decoder.decode(image.as_ref(), 456),
        VideoCodecStatus::Unknown(2)
    );
    assert!(
        factory.create(env.as_ref(), format.as_ref()).is_none(),
        "2 回目の create は None を返す想定です"
    );
}

#[test]
fn video_decoder_handler_register_decode_complete_callback_accepts_none_and_some() {
    struct TestVideoDecoderHandler {
        called_with_none: bool,
        called_with_some: bool,
    }
    impl VideoDecoderHandler for TestVideoDecoderHandler {
        fn register_decode_complete_callback(
            &mut self,
            callback: Option<VideoDecoderDecodedImageCallbackPtr>,
        ) -> VideoCodecStatus {
            if callback.is_some() {
                self.called_with_some = true;
            } else {
                self.called_with_none = true;
            }
            VideoCodecStatus::Ok
        }
    }

    let mut handler = TestVideoDecoderHandler {
        called_with_none: false,
        called_with_some: false,
    };
    assert_eq!(
        handler.register_decode_complete_callback(None),
        VideoCodecStatus::Ok
    );
    let dummy_callback = unsafe {
        // このテストでは callback を呼び出さず Option::Some 経路だけを確認する。
        VideoDecoderDecodedImageCallbackPtr::from_raw(NonNull::dangling())
    };
    assert_eq!(
        handler.register_decode_complete_callback(Some(dummy_callback)),
        VideoCodecStatus::Ok
    );
    assert!(handler.called_with_none);
    assert!(handler.called_with_some);
}

// implementation_name() が解放済みの値を返していることがあったので、その回帰テストを行う
#[test]
fn custom_video_decoder_get_decoder_info_name_experiment() {
    struct TestVideoDecoderHandler {
        expected: String,
    }
    impl VideoDecoderHandler for TestVideoDecoderHandler {
        fn get_decoder_info(&mut self) -> VideoDecoderDecoderInfo {
            let mut info = VideoDecoderDecoderInfo::new();
            info.set_implementation_name(&self.expected);
            info.set_is_hardware_accelerated(false);
            info
        }
    }

    let expected = "decoder-info-name-".repeat(128);
    let decoder = VideoDecoder::new_with_handler(Box::new(TestVideoDecoderHandler {
        expected: expected.clone(),
    }));

    for _ in 0..100 {
        let info = decoder.get_decoder_info();
        assert_eq!(
            info.implementation_name()
                .expect("implementation_name の取得に失敗しました"),
            expected,
            "GetDecoderInfo の implementation_name が不一致になりました"
        );
        assert!(!info.is_hardware_accelerated());
    }
}
