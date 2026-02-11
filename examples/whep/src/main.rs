use rustls::pki_types::ServerName;
use rustls::{ClientConfig, ClientConnection, StreamOwned};
use rustls_platform_verifier::ConfigVerifierExt;
use shiguredo_http11::{Request, Response, ResponseDecoder, uri::Uri};
use shiguredo_webrtc::{
    AudioDecoderFactory, AudioDeviceModule, AudioDeviceModuleAudioLayer, AudioEncoderFactory,
    AudioProcessingBuilder, Environment, I420Buffer, IceServer, IceTransportsType, MediaType,
    PeerConnection, PeerConnectionDependencies, PeerConnectionFactory,
    PeerConnectionFactoryDependencies, PeerConnectionObserver, PeerConnectionObserverBuilder,
    PeerConnectionOfferAnswerOptions, PeerConnectionRtcConfiguration, PeerConnectionState,
    RtcEventLogFactory, RtpTransceiverDirection, RtpTransceiverInit, SdpType, SessionDescription,
    SetLocalDescriptionObserver, SetRemoteDescriptionObserver, Thread, VideoDecoderFactory,
    VideoEncoderFactory, VideoFrameRef, VideoSink, VideoSinkBuilder, VideoSinkWants, VideoTrack,
};
use std::fmt::Write as FmtWrite;
use std::io::{self, Read, Write as IoWrite};
use std::net::TcpStream;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::{Duration, Instant};

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

/// ANSI 描画用の簡易レンダラー。
struct AnsiRenderer {
    sink: VideoSink,
}

impl AnsiRenderer {
    fn new() -> Self {
        let width = 80;
        let height = 45;
        let sink = VideoSinkBuilder::new(move |frame| {
            render_frame(frame, width, height);
        })
        .build();
        Self { sink }
    }

    fn sink(&self) -> &VideoSink {
        &self.sink
    }
}

fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> i32 {
    let r6 = (r as i32 * 5) / 255;
    let g6 = (g as i32 * 5) / 255;
    let b6 = (b as i32 * 5) / 255;
    16 + (r6 * 36) + (g6 * 6) + b6
}

fn render_frame(frame: VideoFrameRef, width: i32, height: i32) {
    let src = frame.buffer();
    let mut scaled = I420Buffer::new(width, height);
    scaled.scale_from(&src);

    let image = match shiguredo_webrtc::i420_to_argb(&scaled) {
        Some(image) => image,
        None => return,
    };
    let width_u = width.max(0) as usize;
    let height_u = height.max(0) as usize;
    let capacity = width_u.saturating_mul(height_u).saturating_mul(20);
    let mut output = String::with_capacity(capacity);
    output.push_str("\x1b[H");

    // 2x1 ピクセルを 1 文字で表現する。
    for y in (0..height_u).step_by(2) {
        output.push_str("\x1b[2K");
        for x in 0..width_u {
            let upper_offset = (y * width_u + x) * 4;
            let upper_r = image[upper_offset + 2];
            let upper_g = image[upper_offset + 1];
            let upper_b = image[upper_offset];

            let (lower_r, lower_g, lower_b) = if y + 1 < height_u {
                let lower_offset = ((y + 1) * width_u + x) * 4;
                let lower_r = image[lower_offset + 2];
                let lower_g = image[lower_offset + 1];
                let lower_b = image[lower_offset];
                (lower_r, lower_g, lower_b)
            } else {
                (upper_r, upper_g, upper_b)
            };
            let upper_color = rgb_to_ansi256(upper_r, upper_g, upper_b);
            let lower_color = rgb_to_ansi256(lower_r, lower_g, lower_b);
            let _ = write!(
                output,
                "\x1b[38;5;{}m\x1b[48;5;{}m▀",
                upper_color, lower_color
            );
        }
        output.push_str("\x1b[0m\n");
    }

    let mut stdout = io::stdout();
    let _ = stdout.write_all(output.as_bytes());
    let _ = stdout.flush();
}

/// WHEP 用の設定。
#[derive(Clone)]
pub struct SignalingWhepConfig {
    pub pc_factory: Arc<FactoryHolder>,
    pub signaling_url: String,
    pub channel_id: String,
}

impl SignalingWhepConfig {
    pub fn new(pc_factory: Arc<FactoryHolder>) -> Self {
        Self {
            pc_factory,
            signaling_url: String::new(),
            channel_id: String::new(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum WhepState {
    Init,
    Connecting,
    Connected,
    Closed,
}

struct VideoState {
    renderer: AnsiRenderer,
    video_track: Option<VideoTrack>,
}

impl VideoState {
    fn detach_sink(&mut self) {
        if let Some(track) = self.video_track.take() {
            track.remove_sink(self.renderer.sink());
        }
    }
}

/// シンプルな WHEP クライアント。
pub struct SignalingWhep {
    config: SignalingWhepConfig,
    state: Arc<(Mutex<WhepState>, Condvar)>,
    pc_observer: Option<PeerConnectionObserver>,
    pc: Option<PeerConnection>,
    video_state: Arc<Mutex<VideoState>>,
}

impl SignalingWhep {
    pub fn new(config: SignalingWhepConfig) -> Self {
        let video_state = VideoState {
            renderer: AnsiRenderer::new(),
            video_track: None,
        };
        Self {
            config,
            state: Arc::new((Mutex::new(WhepState::Init), Condvar::new())),
            pc_observer: None,
            pc: None,
            video_state: Arc::new(Mutex::new(video_state)),
        }
    }

    fn set_state(&self, state: WhepState) {
        let (lock, cvar) = &*self.state;
        let mut guard = lock.lock().unwrap();
        *guard = state;
        cvar.notify_all();
    }

    fn wait_for_state(&self, target: WhepState, timeout: Duration) -> bool {
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
        self.set_state(WhepState::Connecting);
        let pc_factory = self.config.pc_factory.factory();
        let observer_state = self.state.clone();
        let video_state_track = self.video_state.clone();
        let video_state_remove = self.video_state.clone();
        let observer = PeerConnectionObserverBuilder::new()
            .on_connection_change(move |state| {
                let (lock, cvar) = &*observer_state;
                let mut guard = lock.lock().unwrap();
                if matches!(state, PeerConnectionState::Connected) {
                    *guard = WhepState::Connected;
                } else if matches!(
                    state,
                    PeerConnectionState::Failed | PeerConnectionState::Closed
                ) {
                    *guard = WhepState::Closed;
                }
                cvar.notify_all();
            })
            .on_track(move |transceiver| {
                let receiver = transceiver.receiver();
                let track = receiver.track();
                let kind = match track.kind() {
                    Ok(kind) => kind,
                    Err(_) => return,
                };
                if kind != "video" {
                    return;
                }
                let video_track = track.cast_to_video_track();
                let mut state = video_state_track.lock().unwrap();
                if let Some(current) = state.video_track.as_ref()
                    && current.as_ptr() == video_track.as_ptr()
                {
                    return;
                }
                if let Some(track) = state.video_track.take() {
                    track.remove_sink(state.renderer.sink());
                }
                let wants = VideoSinkWants::new();
                video_track.add_or_update_sink(state.renderer.sink(), &wants);
                state.video_track = Some(video_track);
            })
            .on_remove_track(move |receiver| {
                let track = receiver.track();
                let kind = match track.kind() {
                    Ok(kind) => kind,
                    Err(_) => return,
                };
                if kind != "video" {
                    return;
                }
                let video_track = track.cast_to_video_track();
                let mut state = video_state_remove.lock().unwrap();
                if let Some(current) = state.video_track.as_ref()
                    && current.as_ptr() == video_track.as_ptr()
                {
                    state.detach_sink();
                }
            })
            .build();
        // Keep observer alive for the lifetime of the PeerConnection.
        let mut deps = PeerConnectionDependencies::new(&observer);
        // Store observer so it lives as long as SignalingWhep.
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
        init.set_direction(RtpTransceiverDirection::RecvOnly);
        pc.add_transceiver(MediaType::Audio, &mut init)
            .map_err(|e| format!("add audio transceiver failed: {e}"))?;
        Ok(())
    }

    fn add_video_transceiver(&self) -> Result<(), String> {
        let pc = self.pc.as_ref().ok_or("pc not available")?;
        let mut init = RtpTransceiverInit::new();
        init.set_direction(RtpTransceiverDirection::RecvOnly);
        pc.add_transceiver(MediaType::Video, &mut init)
            .map_err(|e| format!("add video transceiver failed: {e}"))?;
        Ok(())
    }

    fn create_offer_and_exchange(&mut self) -> Result<(), String> {
        let pc = self.pc.as_ref().ok_or("pc not available")?;
        let mut opts = PeerConnectionOfferAnswerOptions::new();

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
        let whep_url = build_whep_url(&self.config.signaling_url, &self.config.channel_id)
            .map_err(|e| format!("invalid signaling url: {e}"))?;
        let body =
            send_offer(&whep_url, &offer_sdp).map_err(|e| format!("send offer failed: {e}"))?;

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
        self.wait_for_state(WhepState::Connected, timeout)
    }

    pub fn disconnect(&mut self) {
        self.detach_video_sink();
        if let Some(pc) = self.pc.take() {
            drop(pc);
        }
        self.pc_observer = None;
        self.set_state(WhepState::Closed);
    }

    fn detach_video_sink(&self) {
        let mut state = self.video_state.lock().unwrap();
        state.detach_sink();
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

fn build_whep_url(base: &str, channel_id: &str) -> Result<String, String> {
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
        .header("User-Agent", "Whep-Client")
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

fn send_request_stream<T: Read + IoWrite>(
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
    args.metadata_mut().app_name = "whep";
    args.metadata_mut().app_description = "WHEP client example";

    noargs::HELP_FLAG.take_help(&mut args);

    let url: String = noargs::opt("url")
        .short('u')
        .ty("URL")
        .doc("WHEP signaling URL")
        .env("WHEP_URL")
        .take(&mut args)
        .then(|o| o.value().parse())?;

    let channel_id: String = noargs::opt("channel-id")
        .short('c')
        .doc("Channel ID")
        .env("WHEP_CHANNEL_ID")
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

    //log::log_to_debug(log::Severity::Info);
    //log::enable_timestamps();
    //log::enable_threads();

    let factory = FactoryHolder::new().ok_or("factory create failed")?;

    let mut whep_cfg = SignalingWhepConfig::new(factory.clone());
    whep_cfg.signaling_url = args.url;
    whep_cfg.channel_id = args.channel_id;

    let mut whep = SignalingWhep::new(whep_cfg);
    whep.connect()?;
    whep.wait_for_connect(Duration::from_secs(5));

    thread::sleep(Duration::from_secs(30));

    whep.disconnect();
    Ok(())
}
