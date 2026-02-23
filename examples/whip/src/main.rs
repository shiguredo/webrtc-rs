use rustls::pki_types::ServerName;
use rustls::{ClientConfig, ClientConnection, StreamOwned};
use rustls_platform_verifier::ConfigVerifierExt;
use shiguredo_http11::{Request, Response, ResponseDecoder, uri::Uri};
use shiguredo_webrtc::{
    AdaptFrameResult, AdaptedVideoTrackSource, AudioDecoderFactory, AudioDeviceModule,
    AudioDeviceModuleAudioLayer, AudioEncoderFactory, AudioProcessingBuilder, CxxString,
    Environment, IceServer, IceTransportsType, MediaType, PeerConnection,
    PeerConnectionDependencies, PeerConnectionFactory, PeerConnectionFactoryDependencies,
    PeerConnectionObserver, PeerConnectionObserverBuilder, PeerConnectionOfferAnswerOptions,
    PeerConnectionRtcConfiguration, PeerConnectionState, RtcEventLogFactory, RtpCodec,
    RtpCodecCapabilityVector, RtpEncodingParameters, RtpEncodingParametersVector,
    RtpTransceiverDirection, RtpTransceiverInit, SdpType, SessionDescription,
    SetLocalDescriptionObserver, SetRemoteDescriptionObserver, Thread, VideoDecoderFactory,
    VideoEncoderFactory, VideoTrackSource, log,
};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::{Duration, Instant};

/// u32 のスライスを読み取り専用の u8 スライスとして扱う。
fn u32_slice_as_u8_slice(data: &[u32]) -> &[u8] {
    let len = std::mem::size_of_val(data);
    let ptr = data.as_ptr() as *const u8;
    // 安全性: u32 の連続領域を読み取り専用の u8 スライスとして扱う。
    unsafe { std::slice::from_raw_parts(ptr, len) }
}

/// PeerConnectionFactory と関連リソースをまとめて管理する。
pub struct FactoryHolder {
    factory: PeerConnectionFactory,
    _network: Thread,
    _worker: Thread,
    _signaling: Thread,
}

impl FactoryHolder {
    pub fn new() -> Option<Arc<Self>> {
        let env = Environment::new();
        let mut network = Thread::new_with_socket_server();
        let mut worker = Thread::new();
        let mut signaling = Thread::new();
        network.start();
        worker.start();
        signaling.start();

        let mut deps = PeerConnectionFactoryDependencies::new();
        deps.set_network_thread(&network);
        deps.set_worker_thread(&worker);
        deps.set_signaling_thread(&signaling);
        let event_log = RtcEventLogFactory::new();
        deps.set_event_log_factory(event_log);
        let adm = AudioDeviceModule::new(&env, AudioDeviceModuleAudioLayer::Dummy).ok()?;
        deps.set_audio_device_module(&adm);
        let audio_enc = AudioEncoderFactory::builtin();
        let audio_dec = AudioDecoderFactory::builtin();
        deps.set_audio_encoder_factory(&audio_enc);
        deps.set_audio_decoder_factory(&audio_dec);
        let video_enc = VideoEncoderFactory::builtin();
        let video_dec = VideoDecoderFactory::builtin();
        deps.set_video_encoder_factory(video_enc);
        deps.set_video_decoder_factory(video_dec);
        let apb = AudioProcessingBuilder::new_builtin();
        deps.set_audio_processing_builder(apb);
        deps.enable_media();

        let factory = PeerConnectionFactory::create_modular(&mut deps).ok()?;
        #[allow(clippy::arc_with_non_send_sync)]
        Some(Arc::new(Self {
            factory,
            _network: network,
            _worker: worker,
            _signaling: signaling,
        }))
    }

    pub fn factory(&self) -> &PeerConnectionFactory {
        &self.factory
    }
}

/// on_tick は 1 秒ごとに、1 秒の先頭で生成されるフレームだけで呼ばれる（先頭の幅は 1000 / fps ミリ秒）。
#[derive(Clone)]
pub struct FakeVideoCapturerConfig {
    pub width: i32,
    pub height: i32,
    pub fps: i32,
    pub on_tick: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl Default for FakeVideoCapturerConfig {
    fn default() -> Self {
        Self {
            width: 640,
            height: 480,
            fps: 30,
            on_tick: None,
        }
    }
}

pub struct FakeVideoCapturer {
    source: AdaptedVideoTrackSource,
    timestamp_aligner: Option<shiguredo_webrtc::TimestampAligner>,
    image: Vec<u32>,
    width: i32,
    height: i32,
    fps: i32,
    start_time_ms: i64,
    on_tick: Option<Arc<dyn Fn() + Send + Sync>>,
    video_source: VideoTrackSource,
    stop: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl FakeVideoCapturer {
    pub fn new(config: FakeVideoCapturerConfig) -> Option<Self> {
        let width = if config.width > 0 { config.width } else { 640 };
        let height = if config.height > 0 {
            config.height
        } else {
            480
        };
        let fps = if config.fps > 0 { config.fps } else { 30 };
        let source = AdaptedVideoTrackSource::new();
        let timestamp_aligner = shiguredo_webrtc::TimestampAligner::new();
        let video_source = source.cast_to_video_track_source();
        Some(Self {
            image: vec![0u32; (width * height) as usize],
            width,
            height,
            fps,
            start_time_ms: shiguredo_webrtc::time_millis(),
            on_tick: config.on_tick,
            video_source,
            source,
            timestamp_aligner: Some(timestamp_aligner),
            stop: Arc::new(AtomicBool::new(false)),
            handle: None,
        })
    }

    pub fn video_source(&self) -> VideoTrackSource {
        self.video_source.clone()
    }

    /// キャプチャスレッドを開始する。
    pub fn start(&mut self) -> bool {
        if self.handle.is_some() {
            return true;
        }
        let mut source = self.source.clone();
        let mut timestamp_aligner = match self.timestamp_aligner.take() {
            Some(t) => t,
            None => return true,
        };
        let mut image = std::mem::take(&mut self.image);
        let width = self.width;
        let height = self.height;
        let fps = self.fps.max(1);
        let start_time_ms = self.start_time_ms;
        let on_tick = self.on_tick.clone();
        let stop = self.stop.clone();
        let handle = thread::Builder::new()
            .name("fake-video-capturer".to_string())
            .spawn(move || {
                while !stop.load(Ordering::Acquire) {
                    tick_once(
                        &mut source,
                        &mut timestamp_aligner,
                        &mut image,
                        width,
                        height,
                        start_time_ms,
                        fps,
                        on_tick.as_ref(),
                    );
                    let sleep_ms = (1000 / fps).saturating_sub(2).max(1);
                    shiguredo_webrtc::thread_sleep_ms(sleep_ms);
                }
            });
        match handle {
            Ok(h) => {
                self.handle = Some(h);
                true
            }
            Err(_) => false,
        }
    }

    /// スレッド停止を指示する。
    pub fn stop(&mut self) {
        self.stop.store(true, Ordering::Release);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for FakeVideoCapturer {
    fn drop(&mut self) {
        self.stop();
    }
}

#[allow(clippy::too_many_arguments)]
fn tick_once(
    source: &mut AdaptedVideoTrackSource,
    timestamp_aligner: &mut shiguredo_webrtc::TimestampAligner,
    image: &mut [u32],
    width: i32,
    height: i32,
    start_time_ms: i64,
    fps: i32,
    on_tick: Option<&Arc<dyn Fn() + Send + Sync>>,
) {
    let elapsed_ms = shiguredo_webrtc::time_millis() - start_time_ms;
    let radius = (width.min(height)) / 4;
    let center_x = width / 2;
    let center_y = height / 2;
    let angle = 2.0 * std::f64::consts::PI * (elapsed_ms % 1000) as f64 / 1000.0;
    let circle_x = center_x + (radius as f64 * angle.cos()) as i32;
    let circle_y = center_y + (radius as f64 * angle.sin()) as i32;
    let circle_radius = 100;

    image.fill(0);
    for y in -circle_radius..=circle_radius {
        for x in -circle_radius..=circle_radius {
            if x * x + y * y <= circle_radius * circle_radius {
                let draw_x = circle_x + x;
                let draw_y = circle_y + y;
                if draw_x >= 0 && draw_x < width && draw_y >= 0 && draw_y < height {
                    let mut color = 0xFF00_0000u32;
                    color |= (((elapsed_ms / 10) % 256) as u32) << 16;
                    color |= (((elapsed_ms / 5) % 256) as u32) << 8;
                    color |= (elapsed_ms % 256) as u32;
                    image[(draw_y * width + draw_x) as usize] = color;
                }
            }
        }
    }

    if let Some(cb) = on_tick {
        let tick_span_ms = (1000 / fps.max(1)) as i64;
        if elapsed_ms % 1000 < tick_span_ms {
            cb();
        }
    }

    if let Some(buffer) =
        shiguredo_webrtc::abgr_to_i420(u32_slice_as_u8_slice(image), width, height)
    {
        let timestamp_us = elapsed_ms * 1000;
        let frame = shiguredo_webrtc::VideoFrame::from_i420(&buffer, timestamp_us, 0);
        let AdaptFrameResult { applied, size } = source.adapt_frame(width, height, timestamp_us);
        let frame = if applied
            && (size.adapted_width != frame.width() || size.adapted_height != frame.height())
        {
            let mut scaled =
                shiguredo_webrtc::I420Buffer::new(size.adapted_width, size.adapted_height);
            scaled.scale_from(&buffer);
            shiguredo_webrtc::VideoFrame::from_i420(
                &scaled,
                timestamp_aligner.translate(timestamp_us, shiguredo_webrtc::time_millis() * 1000),
                0,
            )
        } else {
            shiguredo_webrtc::VideoFrame::from_i420(
                &buffer,
                timestamp_aligner.translate(timestamp_us, shiguredo_webrtc::time_millis() * 1000),
                0,
            )
        };
        source.on_frame(&frame);
    }
}

/// WHIP 用の設定。
#[derive(Clone)]
pub struct SignalingWhipConfig {
    pub pc_factory: Arc<FactoryHolder>,
    pub video_source: Option<VideoTrackSource>,
    pub send_encodings: Option<RtpEncodingParametersVector>,
    pub signaling_url: String,
    pub channel_id: String,
}

impl SignalingWhipConfig {
    pub fn new(pc_factory: Arc<FactoryHolder>) -> Self {
        Self {
            pc_factory,
            video_source: None,
            send_encodings: None,
            signaling_url: String::new(),
            channel_id: String::new(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum WhipState {
    Init,
    Connecting,
    Connected,
    Closed,
}

/// シンプルな WHIP クライアント。
pub struct SignalingWhip {
    config: SignalingWhipConfig,
    state: Arc<(Mutex<WhipState>, Condvar)>,
    pc_observer: Option<PeerConnectionObserver>,
    pc: Option<PeerConnection>,
}

impl SignalingWhip {
    pub fn new(config: SignalingWhipConfig) -> Self {
        Self {
            config,
            state: Arc::new((Mutex::new(WhipState::Init), Condvar::new())),
            pc_observer: None,
            pc: None,
        }
    }

    fn set_state(&self, state: WhipState) {
        let (lock, cvar) = &*self.state;
        let mut guard = lock.lock().unwrap();
        *guard = state;
        cvar.notify_all();
    }

    fn wait_for_state(&self, target: WhipState, timeout: Duration) -> bool {
        let (lock, cvar) = &*self.state;
        let mut guard = lock.lock().unwrap();
        let deadline = Instant::now() + timeout;
        while *guard != target {
            let now = Instant::now();
            if now >= deadline {
                return false;
            }
            let remaining = deadline - now;
            let (g, res) = cvar.wait_timeout(guard, remaining).unwrap();
            guard = g;
            if res.timed_out() {
                return false;
            }
        }
        true
    }

    pub fn connect(&mut self) -> Result<(), String> {
        self.set_state(WhipState::Connecting);
        let pc_factory = self.config.pc_factory.factory();
        let observer_state = self.state.clone();
        let observer = PeerConnectionObserverBuilder::new()
            .on_connection_change(move |state| {
                let (lock, cvar) = &*observer_state;
                let mut guard = lock.lock().unwrap();
                if matches!(state, PeerConnectionState::Connected) {
                    *guard = WhipState::Connected;
                } else if matches!(
                    state,
                    PeerConnectionState::Failed | PeerConnectionState::Closed
                ) {
                    *guard = WhipState::Closed;
                }
                cvar.notify_all();
            })
            .build();
        // Keep observer alive for the lifetime of the PeerConnection.
        let mut deps = PeerConnectionDependencies::new(&observer);
        // Store observer so it lives as long as SignalingWhip.
        self.pc_observer = Some(observer);
        let mut config = PeerConnectionRtcConfiguration::new();
        let pc = PeerConnection::create(pc_factory, &mut config, &mut deps)
            .map_err(|e| format!("pc create failed: {e}"))?;
        self.pc = Some(pc);

        self.add_audio_transceiver()?;
        self.add_video_transceiver()?;
        self.create_offer_and_exchange()?;
        Ok(())
    }

    fn add_audio_transceiver(&self) -> Result<(), String> {
        let pc = self.pc.as_ref().ok_or("pc not available")?;
        let mut init = RtpTransceiverInit::new();
        init.set_direction(RtpTransceiverDirection::SendOnly);
        let mut transceiver = pc
            .add_transceiver(MediaType::Audio, &mut init)
            .map_err(|e| format!("add audio transceiver failed: {e}"))?;

        let caps = self
            .config
            .pc_factory
            .factory()
            .get_rtp_sender_capabilities(MediaType::Audio);
        let mut codecs = RtpCodecCapabilityVector::new(0);
        let src = caps.codecs();
        for i in 0..src.len() {
            if let Some(cap) = src.get(i) {
                let name = cap
                    .name()
                    .map_err(|e| format!("codec 名の取得に失敗しました : {e}"))?;
                if name.eq_ignore_ascii_case("opus") {
                    codecs.push(&cap);
                    break;
                }
            }
        }
        transceiver
            .set_codec_preferences(&codecs)
            .map_err(|e| format!("set audio codec failed: {e}"))?;
        Ok(())
    }

    fn add_video_transceiver(&self) -> Result<(), String> {
        let pc = self.pc.as_ref().ok_or("pc not available")?;
        let mut init = RtpTransceiverInit::new();
        init.set_direction(RtpTransceiverDirection::SendOnly);
        if let Some(encodings) = &self.config.send_encodings {
            init.set_send_encodings(encodings);
        }
        let mut stream_ids = init.stream_ids();
        let stream_id = shiguredo_webrtc::random_string(16);
        stream_ids.push(&CxxString::from_str(&stream_id));
        let source = match &self.config.video_source {
            Some(s) => s.clone(),
            None => return Ok(()),
        };
        let track_id = shiguredo_webrtc::random_string(16);
        let track = self
            .config
            .pc_factory
            .factory()
            .create_video_track(&source, &track_id)
            .map_err(|_| "create video track failed".to_string())?;
        let mut transceiver = pc
            .add_transceiver_with_track(&track, &mut init)
            .map_err(|e| format!("add video transceiver failed: {e}"))?;
        let caps = self
            .config
            .pc_factory
            .factory()
            .get_rtp_sender_capabilities(MediaType::Video);
        let mut codecs = RtpCodecCapabilityVector::new(0);
        let src = caps.codecs();
        for i in 0..src.len() {
            if let Some(cap) = src.get(i) {
                let name = cap
                    .name()
                    .map_err(|e| format!("codec 名の取得に失敗しました : {e}"))?
                    .to_ascii_lowercase();
                if name == "rtx" {
                    codecs.push(&cap);
                    continue;
                }
                if let Some(encs) = &self.config.send_encodings {
                    let mut matched = false;
                    for idx in 0..encs.len() {
                        if let Some(enc) = encs.get(idx) {
                            let Some(enc_codec) = enc.codec() else {
                                continue;
                            };
                            let enc_name = enc_codec
                                .name()
                                .map_err(|e| format!("codec 名の取得に失敗しました : {e}"))?
                                .to_ascii_lowercase();
                            if enc_name == name {
                                matched = true;
                                break;
                            }
                        }
                    }
                    if matched {
                        codecs.push(&cap);
                    }
                }
            }
        }
        transceiver
            .set_codec_preferences(&codecs)
            .map_err(|e| format!("set video codec failed: {e}"))?;
        Ok(())
    }

    fn create_offer_and_exchange(&mut self) -> Result<(), String> {
        let pc = self.pc.as_mut().ok_or("pc not available")?;
        let mut opts = PeerConnectionOfferAnswerOptions::new();
        opts.set_offer_to_receive_audio(0);
        opts.set_offer_to_receive_video(0);

        let (offer_tx, offer_rx) = std::sync::mpsc::channel::<Result<String, String>>();
        let offer_tx_err = offer_tx.clone();
        let mut offer_obs = shiguredo_webrtc::CreateSessionDescriptionObserver::new(
            move |desc| {
                let sdp = desc
                    .to_string()
                    .map_err(|e| format!("offer to_string failed: {e}"));
                let _ = offer_tx.send(sdp);
            },
            move |err| {
                let msg = err.message().unwrap_or_else(|_| "unknown".to_string());
                let _ = offer_tx_err.send(Err(msg));
            },
        );
        pc.create_offer(&mut offer_obs, &mut opts);
        let offer_sdp = offer_rx
            .recv_timeout(Duration::from_secs(5))
            .map_err(|_| "offer timeout".to_string())?
            .map_err(|e| format!("offer failed: {e}"))?;
        let whip_url = build_whip_url(&self.config.signaling_url, &self.config.channel_id)
            .map_err(|e| format!("invalid signaling url: {e}"))?;
        let body =
            send_offer(&whip_url, &offer_sdp).map_err(|e| format!("send offer failed: {e}"))?;

        let mut config = PeerConnectionRtcConfiguration::new();
        let mut server = IceServer::new();
        for url in body.ice_urls {
            server.add_url(&url);
        }
        if let Some(user) = body.username {
            server.set_username(&user);
        }
        if let Some(pass) = body.credential {
            server.set_password(&pass);
        }
        config.servers().push(&server);
        config.set_type(IceTransportsType::Relay);
        pc.set_configuration(&mut config)
            .map_err(|e| format!("set config failed: {e}"))?;

        let offer_desc = SessionDescription::new(SdpType::Offer, &offer_sdp)
            .map_err(|e| format!("offer create failed: {e}"))?;
        let (loc_tx, loc_rx) = std::sync::mpsc::channel::<Option<String>>();
        let loc_obs = SetLocalDescriptionObserver::new(move |err| {
            let msg = if err.ok() {
                None
            } else {
                Some(err.message().unwrap_or_else(|_| "unknown".to_string()))
            };
            let _ = loc_tx.send(msg);
        });
        pc.set_local_description(offer_desc, &loc_obs);
        let loc_res = loc_rx
            .recv_timeout(Duration::from_secs(5))
            .map_err(|_| "set local timeout".to_string())?;
        if let Some(err) = loc_res {
            return Err(format!("set local error: {err:?}"));
        }

        let answer = SessionDescription::new(SdpType::Answer, &body.sdp)
            .map_err(|e| format!("answer create failed: {e}"))?;
        let (rem_tx, rem_rx) = std::sync::mpsc::channel::<Option<String>>();
        let rem_obs = SetRemoteDescriptionObserver::new(move |err| {
            let msg = if err.ok() {
                None
            } else {
                Some(err.message().unwrap_or_else(|_| "unknown".to_string()))
            };
            let _ = rem_tx.send(msg);
        });
        pc.set_remote_description(answer, &rem_obs);
        let rem_res = rem_rx
            .recv_timeout(Duration::from_secs(5))
            .map_err(|_| "set remote timeout".to_string())?;
        if let Some(err) = rem_res {
            return Err(format!("set remote error: {err:?}"));
        }

        Ok(())
    }

    pub fn wait_for_connect(&self, timeout: Duration) -> bool {
        self.wait_for_state(WhipState::Connected, timeout)
    }

    pub fn disconnect(&mut self) {
        if let Some(pc) = self.pc.take() {
            drop(pc);
        }
        self.pc_observer = None;
        self.set_state(WhipState::Closed);
    }
}

struct LinkHeaderBody {
    ice_urls: Vec<String>,
    username: Option<String>,
    credential: Option<String>,
    sdp: String,
}

struct RequestTarget {
    scheme: String,
    host: String,
    port: u16,
    path: String,
    host_header: String,
}

fn build_whip_url(base: &str, channel_id: &str) -> Result<String, String> {
    let uri = Uri::parse(base).map_err(|e| e.to_string())?;
    let scheme = uri.scheme().ok_or("スキームがありません")?;
    let host = uri.host().ok_or("ホストがありません")?;
    let port = uri.port();
    let mut path = uri.path().to_string();
    if !channel_id.is_empty() {
        if !path.ends_with('/') {
            path.push('/');
        }
        path.push_str(channel_id);
    }
    let query = "video_bit_rate=6000";
    let url = if let Some(port) = port {
        format!("{scheme}://{host}:{port}{path}?{query}")
    } else {
        format!("{scheme}://{host}{path}?{query}")
    };
    Ok(url)
}

fn build_request_target(url: &str) -> Result<RequestTarget, String> {
    let uri = Uri::parse(url).map_err(|e| e.to_string())?;
    let scheme = uri.scheme().ok_or("スキームがありません")?.to_string();
    if scheme != "http" && scheme != "https" {
        return Err("サポート外の URL スキームです".to_string());
    }
    let host = uri.host().ok_or("URL にホスト名がありません")?.to_string();
    let default_port = if scheme == "https" { 443 } else { 80 };
    let port = uri.port().unwrap_or(default_port);
    let mut path = uri.path().to_string();
    if let Some(query) = uri.query() {
        path.push('?');
        path.push_str(query);
    }
    let host_header = if (scheme == "http" && port != 80) || (scheme == "https" && port != 443) {
        format!("{host}:{port}")
    } else {
        host.clone()
    };
    Ok(RequestTarget {
        scheme,
        host,
        port,
        path,
        host_header,
    })
}

fn send_offer(url: &str, sdp: &str) -> Result<LinkHeaderBody, String> {
    let target = build_request_target(url)?;
    let request = Request::new("POST", &target.path)
        .header("Host", &target.host_header)
        .header("Content-Type", "application/sdp")
        .header("Connection", "close")
        .header("User-Agent", "Whip-Client")
        .body(sdp.as_bytes().to_vec());
    let response = send_http_request(&target, &request)?;
    let link_header = response
        .get_header("Link")
        .ok_or_else(|| "Link ヘッダーがありません".to_string())?
        .to_string();
    let body = String::from_utf8(response.body).map_err(|e| e.to_string())?;
    let parsed = parse_link_header(&link_header);
    Ok(LinkHeaderBody {
        ice_urls: parsed.urls,
        username: parsed.username,
        credential: parsed.credential,
        sdp: body,
    })
}

fn send_http_request(target: &RequestTarget, request: &Request) -> Result<Response, String> {
    let request_bytes = request.encode();
    if target.scheme == "https" {
        https_request(&target.host, target.port, &request_bytes)
    } else {
        http_request(&target.host, target.port, &request_bytes)
    }
}

fn http_request(host: &str, port: u16, request_bytes: &[u8]) -> Result<Response, String> {
    let mut stream = connect_tcp(host, port)?;
    send_request_stream(&mut stream, request_bytes)
}

fn https_request(host: &str, port: u16, request_bytes: &[u8]) -> Result<Response, String> {
    let config = ClientConfig::with_platform_verifier().map_err(|e| e.to_string())?;
    let server_name = ServerName::try_from(host.to_string()).map_err(|e| e.to_string())?;
    let conn = ClientConnection::new(Arc::new(config), server_name).map_err(|e| e.to_string())?;
    let stream = connect_tcp(host, port)?;
    let mut tls = StreamOwned::new(conn, stream);
    send_request_stream(&mut tls, request_bytes)
}

fn connect_tcp(host: &str, port: u16) -> Result<TcpStream, String> {
    let stream = TcpStream::connect((host, port)).map_err(|e| e.to_string())?;
    stream
        .set_read_timeout(Some(Duration::from_secs(10)))
        .map_err(|e| e.to_string())?;
    stream
        .set_write_timeout(Some(Duration::from_secs(10)))
        .map_err(|e| e.to_string())?;
    Ok(stream)
}

fn send_request_stream<T: Read + Write>(
    stream: &mut T,
    request_bytes: &[u8],
) -> Result<Response, String> {
    stream.write_all(request_bytes).map_err(|e| e.to_string())?;
    read_response(stream)
}

fn read_response<T: Read>(stream: &mut T) -> Result<Response, String> {
    let mut decoder = ResponseDecoder::new();
    let mut buf = [0u8; 4096];
    loop {
        let n = stream.read(&mut buf).map_err(|e| e.to_string())?;
        if n == 0 {
            return Err("レスポンス受信前に接続が閉じられました".to_string());
        }
        decoder.feed(&buf[..n]).map_err(|e| e.to_string())?;
        if let Some(response) = decoder.decode().map_err(|e| e.to_string())? {
            return Ok(response);
        }
    }
}

struct ParsedLink {
    urls: Vec<String>,
    username: Option<String>,
    credential: Option<String>,
}

fn parse_link_header(header: &str) -> ParsedLink {
    let mut urls = Vec::new();
    let mut username = None;
    let mut credential = None;
    for part in header.split(',') {
        let part = part.trim();
        if let Some(start) = part.find('<')
            && let Some(end) = part[start + 1..].find('>')
        {
            let url = &part[start + 1..start + 1 + end];
            urls.push(url.to_string());
        }
        if let Some(pos) = part.to_lowercase().find("username=\"") {
            let rest = &part[pos + "username=\"".len()..];
            if let Some(end) = rest.find('"') {
                username = Some(rest[..end].to_string());
            }
        }
        if let Some(pos) = part.to_lowercase().find("credential=\"") {
            let rest = &part[pos + "credential=\"".len()..];
            if let Some(end) = rest.find('"') {
                credential = Some(rest[..end].to_string());
            }
        }
    }
    ParsedLink {
        urls,
        username,
        credential,
    }
}

struct Args {
    url: String,
    channel_id: String,
}

fn parse_args() -> noargs::Result<Args> {
    let mut args = noargs::raw_args();
    args.metadata_mut().app_name = "whip";
    args.metadata_mut().app_description = "WHIP client example";

    noargs::HELP_FLAG.take_help(&mut args);

    let url: String = noargs::opt("url")
        .short('u')
        .ty("URL")
        .doc("WHIP signaling URL")
        .env("WHIP_URL")
        .take(&mut args)
        .then(|o| o.value().parse())?;

    let channel_id: String = noargs::opt("channel-id")
        .short('c')
        .doc("Channel ID")
        .env("WHIP_CHANNEL_ID")
        .take(&mut args)
        .then(|o| o.value().parse())?;

    if let Some(help) = args.finish()? {
        print!("{help}");
        std::process::exit(0);
    }

    Ok(Args { url, channel_id })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = match parse_args() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("{err:?}");
            std::process::exit(1);
        }
    };

    log::log_to_debug(log::Severity::Info);
    log::enable_timestamps();
    log::enable_threads();

    let factory = FactoryHolder::new().ok_or("factory create failed")?;

    let mut capturer = FakeVideoCapturer::new(FakeVideoCapturerConfig {
        width: 1920,
        height: 1080,
        fps: 30,
        on_tick: None,
    })
    .ok_or("capturer create failed")?;
    if !capturer.start() {
        return Err("capturer start failed".into());
    }

    let mut send_encodings = RtpEncodingParametersVector::new(0);
    let mut av1 = RtpCodec::new();
    av1.set_kind(MediaType::Video);
    av1.set_name("AV1");
    av1.set_clock_rate(Some(90_000));
    let mut params = av1.parameters();
    params.set("level-idx", "5");
    params.set("profile", "0");
    params.set("tier", "0");
    for (rid, scale) in [("r0", 4.0), ("r1", 2.0), ("r2", 1.0)] {
        let mut enc = RtpEncodingParameters::new();
        enc.set_rid(rid);
        enc.set_scale_resolution_down_by(Some(scale));
        enc.set_codec(Some(&av1));
        send_encodings.push(&enc);
    }

    let mut whip_cfg = SignalingWhipConfig::new(factory.clone());
    whip_cfg.signaling_url = args.url;
    whip_cfg.channel_id = args.channel_id;
    whip_cfg.video_source = Some(capturer.video_source());
    whip_cfg.send_encodings = Some(send_encodings.clone());

    let mut whip = SignalingWhip::new(whip_cfg);
    whip.connect()?;
    whip.wait_for_connect(Duration::from_secs(5));

    thread::sleep(Duration::from_secs(10));

    whip.disconnect();
    capturer.stop();
    Ok(())
}
