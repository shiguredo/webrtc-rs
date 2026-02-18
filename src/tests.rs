use super::*;
use std::ptr::NonNull;
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
    let mut fmt = SdpVideoFormat::new("VP8");
    {
        let mut params = fmt.parameters_mut();
        params.set("profile-id", "0");
        params.set("level", "3.1");
        assert_eq!(params.len(), 2);

        let mut found = std::collections::HashMap::new();
        for (k, v) in params.iter() {
            found.insert(k, v);
        }
        assert_eq!(found.get("profile-id").map(String::as_str), Some("0"));
        assert_eq!(found.get("level").map(String::as_str), Some("3.1"));
    }

    let mut other = SdpVideoFormat::new("VP8");
    {
        let mut params = other.parameters_mut();
        params.set("profile-id", "0");
        params.set("level", "3.1");
    }

    assert!(fmt.is_equal(&other));
}

#[test]
fn i420_buffer_and_video_frame() {
    let mut buf = I420Buffer::new(4, 4);
    buf.fill_y(0x10);
    buf.fill_uv(0x80, 0x90);

    let frame = VideoFrame::from_i420(&buf, 12345);
    assert_eq!(frame.width(), 4);
    assert_eq!(frame.height(), 4);
    assert_eq!(frame.timestamp_us(), 12345);

    let copied = frame.buffer();
    assert_eq!(copied.y_data()[0], 0x10);
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
    fn make_ascii_string(len: usize) -> String {
        (0..len).map(|i| (b'a' + (i % 26) as u8) as char).collect()
    }

    let lengths = [0usize, 1, 2, 3, 7, 31, 63, 64];
    for &len in &lengths {
        let name = make_ascii_string(len);
        let guid = make_ascii_string(64usize.saturating_sub(len));
        let name_value = name.clone();
        let guid_value = guid.clone();
        let expected_name = name.clone();
        let expected_guid = guid.clone();
        let callbacks = AudioDeviceModuleCallbacks {
            init: Some(Box::new(|| 0)),
            recording_devices: Some(Box::new(|| 1)),
            recording_device_name: Some(Box::new(move |_| {
                Some((name_value.clone(), guid_value.clone()))
            })),
            ..Default::default()
        };
        let adm = AudioDeviceModule::new_with_callbacks(callbacks);
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
    let frame = VideoFrame::from_i420(&buf, 2_000_000);
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
    let factory = PeerConnectionFactory::create_modular(&mut deps)
        .expect("PeerConnectionFactory の生成に失敗しました");
    let mut opts = PeerConnectionFactoryOptions::new();
    opts.set_disable_encryption(false);
    let dtls12 = unsafe { ffi::webrtc_SSL_PROTOCOL_DTLS_12 };
    opts.set_ssl_max_version(dtls12);
    factory.set_options(&opts);

    let caps = factory.get_rtp_sender_capabilities(MediaType::Audio);
    assert!(caps.codec_len() >= 0);
    let codecs = caps.codecs();
    assert_eq!(codecs.len() as i32, caps.codec_len());
    if !codecs.is_empty() {
        let first = codecs.get(0).expect("先頭 codec の取得に失敗しました");
        assert!(first.name().is_ok());
    }

    drop(caps);
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
    vec.push(&cap);
    assert_eq!(vec.len(), len_before + 1);
    vec.resize(2);
    let mut cap2 = RtpCodecCapability::new();
    cap2.set_kind(MediaType::Audio);
    cap2.set_name("PCMU");
    cap2.set_clock_rate(Some(8_000));
    assert!(vec.set(1, &cap2));
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
    let observer = PeerConnectionObserverBuilder::new().build();
    let mut pc_deps = PeerConnectionDependencies::new(&observer);
    let pc = PeerConnection::create(&factory, &mut pc_config, &mut pc_deps)
        .expect("PeerConnection の生成に失敗しました");

    let stream_track = track.cast_to_media_stream_track();
    let mut stream_ids = StringVector::new(0);
    stream_ids.push(&CxxString::from_str("stream-0"));
    let sender = pc
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
    let observer = PeerConnectionObserverBuilder::new().build();
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
    let frame = VideoFrame::from_i420(&buf, 1_000_000);
    source.on_frame(&frame);

    // PeerConnection を作成し、トラック付きで transceiver を追加する。
    let mut pc_config = PeerConnectionRtcConfiguration::new();
    let observer = PeerConnectionObserverBuilder::new().build();
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
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    let counter = Arc::new(AtomicUsize::new(0));
    let counter_cloned = Arc::clone(&counter);
    let mut observer = PeerConnectionObserverBuilder::new()
        .on_connection_change(move |state| {
            if matches!(state, PeerConnectionState::Connected) {
                counter_cloned.fetch_add(1, Ordering::SeqCst);
            }
        })
        .build();
    observer.invoke_connection_change_for_test(PeerConnectionState::Connected);
    assert_eq!(counter.load(Ordering::SeqCst), 1);

    let observer2 = PeerConnectionObserverBuilder::new().build();
    let mut deps = PeerConnectionDependencies::new(&observer2);
    assert!(!deps.as_ptr().is_null());
    drop(deps);
}

#[test]
fn create_and_set_local_description_observers() {
    let _create_obs = CreateSessionDescriptionObserver::new(|_| {}, |_| {});
    let _set_local = SetLocalDescriptionObserver::new(|_| {});
    let _set_remote = SetRemoteDescriptionObserver::new(|_| {});
}

#[test]
fn custom_video_encoder_factory_create_and_encode_calls_callbacks() {
    let mut created = false;

    let factory = VideoEncoderFactory::new_with_callbacks(VideoEncoderFactoryCallbacks {
        create: {
            Some(Box::new(move |env, format| {
                assert!(!env.as_ptr().is_null());
                assert_eq!(
                    format.name().expect("SdpVideoFormatRef::name に失敗しました"),
                    "VP8"
                );
                if created {
                    return None;
                }
                created = true;
                let mut encode_count = 0;
                Some(VideoEncoder::new_with_callbacks(VideoEncoderCallbacks {
                    encode: Some(Box::new(move |_, frame_types| {
                        let frame_types = frame_types.expect("frame_types が None です");
                        assert_eq!(frame_types.len(), 2);
                        assert_eq!(frame_types.get(0), Some(VideoFrameType::Key));
                        assert_eq!(frame_types.get(1), Some(VideoFrameType::Delta));
                        encode_count += 1;
                        encode_count
                    })),
                    ..Default::default()
                }))
            }))
        },
        ..Default::default()
    });

    let env = Environment::new();
    let format = SdpVideoFormat::new("VP8");
    let encoder = factory
        .create(&env, &format)
        .expect("custom encoder の作成に失敗しました");

    let buffer = I420Buffer::new(2, 2);
    let frame = VideoFrame::from_i420(&buffer, 123);
    let mut frame_types = VideoFrameTypeVector::new(0);
    frame_types.push(VideoFrameType::Key);
    frame_types.push(VideoFrameType::Delta);

    assert_eq!(encoder.encode_with_frame_types(&frame, Some(frame_types.as_ref())), 1);
    assert_eq!(encoder.encode_with_frame_types(&frame, Some(frame_types.as_ref())), 2);
    assert!(
        factory.create(&env, &format).is_none(),
        "2 回目の create は None を返す想定です"
    );
}

#[test]
fn custom_video_encoder_register_and_encode_calls_encoded_image_callback() {
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

    let mut state = Box::new(State::default());
    let state_ptr = StatePtr((&mut *state) as *mut State);
    let state_ptr_for_register = state_ptr;
    let state_ptr_for_encode = state_ptr;
    let state_ptr_for_callback = state_ptr;

    let encoder = VideoEncoder::new_with_callbacks(VideoEncoderCallbacks {
        register_encode_complete_callback: Some(Box::new(move |callback| {
            let callback = callback.expect("register 側 callback が None です");
            let state = unsafe { state_ptr_for_register.get_mut() };
            state.register_called = true;
            state.order.push("register");
            state.callback_ptr = Some(unsafe { VideoEncoderEncodedImageCallbackPtr::from_ref(callback) });
            0
        })),
        encode: Some(Box::new(move |_, _| {
            {
                let state = unsafe { state_ptr_for_encode.get_mut() };
                state.encode_called = true;
                state.order.push("encode");
            }

            let callback_ptr = {
                let state = unsafe { state_ptr_for_encode.get_mut() };
                state.callback_ptr.expect("encode 側 callback_ptr が未設定です")
            };
            let image = EncodedImage::new();
            let result = unsafe { callback_ptr.on_encoded_image(image.as_ref(), None) };
            assert_eq!(result.error(), VideoEncoderEncodedImageCallbackResultError::Ok);
            77
        })),
        ..Default::default()
    });

    let encoded_image_callback = VideoEncoderEncodedImageCallback::new_with_callbacks(
        VideoEncoderEncodedImageCallbackCallbacks {
            on_encoded_image: Some(Box::new(move |image, codec_specific_info| {
                let state = unsafe { state_ptr_for_callback.get_mut() };
                state.on_encoded_image_called = true;
                state.order.push("on_encoded_image");
                assert!(image.encoded_data().is_none());
                assert!(
                    codec_specific_info.is_none(),
                    "codec_specific_info は None の想定です"
                );
                VideoEncoderEncodedImageCallbackResult::new(
                    VideoEncoderEncodedImageCallbackResultError::Ok,
                )
            })),
        },
    );

    assert_eq!(
        encoder.register_encode_complete_callback(Some(encoded_image_callback.as_ref())),
        0
    );

    let buffer = I420Buffer::new(2, 2);
    let frame = VideoFrame::from_i420(&buffer, 123);
    assert_eq!(encoder.encode(&frame), 77);

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

    let mut state = Box::new(State::default());
    let state_ptr = StatePtr((&mut *state) as *mut State);
    let state_ptr_for_register = state_ptr;
    let state_ptr_for_encode = state_ptr;
    let state_ptr_for_callback = state_ptr;

    let encoder = VideoEncoder::new_with_callbacks(VideoEncoderCallbacks {
        register_encode_complete_callback: Some(Box::new(move |callback| {
            let callback = callback.expect("register 側 callback が None です");
            let state = unsafe { state_ptr_for_register.get_mut() };
            state.register_called = true;
            state.order.push("register");
            state.callback_ptr = Some(unsafe { VideoEncoderEncodedImageCallbackPtr::from_ref(callback) });
            0
        })),
        encode: Some(Box::new(move |_, _| {
            {
                let state = unsafe { state_ptr_for_encode.get_mut() };
                state.encode_called = true;
                state.order.push("encode");
            }

            let callback_ptr = {
                let state = unsafe { state_ptr_for_encode.get_mut() };
                state.callback_ptr.expect("encode 側 callback_ptr が未設定です")
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
                callback_ptr.on_encoded_image(
                    image.as_ref(),
                    Some(codec_specific_info.as_ref()),
                )
            };
            assert_eq!(result.error(), VideoEncoderEncodedImageCallbackResultError::Ok);
            assert_eq!(result.frame_id(), 9999);
            assert!(!result.drop_next_frame());
            88
        })),
        ..Default::default()
    });

    let encoded_image_callback = VideoEncoderEncodedImageCallback::new_with_callbacks(
        VideoEncoderEncodedImageCallbackCallbacks {
            on_encoded_image: Some(Box::new(move |image, codec_specific_info| {
                let state = unsafe { state_ptr_for_callback.get_mut() };
                state.on_encoded_image_called = true;
                state.order.push("on_encoded_image");

                let encoded_data = image
                    .encoded_data()
                    .expect("encoded_data が None です");
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
            })),
        },
    );

    assert_eq!(
        encoder.register_encode_complete_callback(Some(encoded_image_callback.as_ref())),
        0
    );

    let buffer = I420Buffer::new(2, 2);
    let frame = VideoFrame::from_i420(&buffer, 123);
    assert_eq!(encoder.encode(&frame), 88);

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

#[test]
fn custom_video_decoder_factory_create_and_decode_calls_callbacks() {
    let mut created = false;

    let factory = VideoDecoderFactory::new_with_callbacks(VideoDecoderFactoryCallbacks {
        create: {
            Some(Box::new(move |env, _| {
                assert!(!env.as_ptr().is_null());
                if created {
                    return None;
                }
                created = true;
                let mut decode_count = 0;
                Some(VideoDecoder::new_with_callbacks(VideoDecoderCallbacks {
                    decode: Some(Box::new(move |input, render_time_ms| {
                        assert!(input.encoded_data().is_none());
                        assert_eq!(render_time_ms, 456);
                        decode_count += 1;
                        decode_count
                    })),
                    ..Default::default()
                }))
            }))
        },
        ..Default::default()
    });

    let env = Environment::new();
    let format = SdpVideoFormat::new("VP8");
    let decoder = factory
        .create(&env, &format)
        .expect("custom decoder の作成に失敗しました");

    assert_eq!(decoder.decode(None, 456), 1);
    assert_eq!(decoder.decode(None, 456), 2);
    assert!(
        factory.create(&env, &format).is_none(),
        "2 回目の create は None を返す想定です"
    );
}

#[test]
fn custom_video_encoder_init_encode_and_set_rates_callbacks_getters() {
    struct BoolPtr(*mut bool);
    unsafe impl Send for BoolPtr {}
    impl BoolPtr {
        fn set_true(&self) {
            unsafe {
                *self.0 = true;
            }
        }
    }

    let mut set_rates_called = false;
    let set_rates_called_ptr = BoolPtr(&mut set_rates_called as *mut bool);
    let encoder = VideoEncoder::new_with_callbacks(VideoEncoderCallbacks {
        init_encode: Some(Box::new(move |codec, settings| {
            assert_eq!(codec.codec_type(), VideoCodecType::Generic);
            assert_eq!(codec.width(), 0);
            assert_eq!(codec.height(), 0);
            assert_eq!(settings.number_of_cores(), 1);
            assert_eq!(settings.max_payload_size(), 1200);
            assert!(!settings.loss_notification());
            assert_eq!(settings.encoder_thread_limit(), None);
            123
        })),
        set_rates: Some(Box::new(move |parameters| {
            assert_eq!(parameters.framerate_fps(), 30.0);
            assert_eq!(parameters.target_bitrate_sum_bps(), 300_000);
            assert_eq!(parameters.bitrate_sum_bps(), 250_000);
            assert_eq!(parameters.bandwidth_allocation_bps(), 350_000);
            set_rates_called_ptr.set_true();
        })),
        ..Default::default()
    });

    assert_eq!(encoder.init_encode(), 123);
    encoder.set_rates();
    assert!(set_rates_called, "set_rates callback が呼ばれませんでした");
}

#[test]
fn custom_video_decoder_configure_callback_getters() {
    let decoder = VideoDecoder::new_with_callbacks(VideoDecoderCallbacks {
        configure: Some(Box::new(move |settings| {
            assert_eq!(settings.number_of_cores(), 1);
            assert_eq!(settings.codec_type(), VideoCodecType::Generic);
            assert_eq!(settings.buffer_pool_size(), None);
            assert_eq!(settings.max_render_resolution_width(), 0);
            assert_eq!(settings.max_render_resolution_height(), 0);
            false
        })),
        ..Default::default()
    });

    assert!(!decoder.configure());
}

#[test]
fn custom_video_decoder_get_decoder_info_name_experiment() {
    let expected = "decoder-info-name-".repeat(128);
    let decoder = VideoDecoder::new_with_callbacks(VideoDecoderCallbacks {
        get_decoder_info: Some(Box::new({
            let expected = expected.clone();
            move || {
                let mut info = VideoDecoderDecoderInfo::new();
                info.set_implementation_name(&expected);
                info.set_is_hardware_accelerated(false);
                info
            }
        })),
        ..Default::default()
    });

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
