#include <chrono>
#include <condition_variable>
#include <cstdint>
#include <functional>
#include <iostream>
#include <map>
#include <memory>
#include <mutex>
#include <optional>
#include <regex>
#include <string>
#include <thread>
#include <utility>
#include <vector>

// WebRTC
#include <api/audio/audio_device.h>
#include <api/audio/builtin_audio_processing_builder.h>
#include <api/audio/create_audio_device_module.h>
#include <api/audio_codecs/builtin_audio_decoder_factory.h>
#include <api/audio_codecs/builtin_audio_encoder_factory.h>
#include <api/create_modular_peer_connection_factory.h>
#include <api/data_channel_interface.h>
#include <api/enable_media.h>
#include <api/environment/environment_factory.h>
#include <api/jsep.h>
#include <api/make_ref_counted.h>
#include <api/media_stream_interface.h>
#include <api/media_types.h>
#include <api/peer_connection_interface.h>
#include <api/ref_count.h>
#include <api/rtc_error.h>
#include <api/rtc_event_log/rtc_event_log_factory.h>
#include <api/rtp_receiver_interface.h>
#include <api/rtp_transceiver_direction.h>
#include <api/rtp_transceiver_interface.h>
#include <api/scoped_refptr.h>
#include <api/set_local_description_observer_interface.h>
#include <api/set_remote_description_observer_interface.h>
#include <api/video/i420_buffer.h>
#include <api/video/video_frame.h>
#include <api/video/video_frame_buffer.h>
#include <api/video/video_sink_interface.h>
#include <api/video/video_source_interface.h>
#include <api/video_codecs/builtin_video_decoder_factory.h>
#include <api/video_codecs/builtin_video_encoder_factory.h>
#include <pc/media_factory.h>
#include <rtc_base/logging.h>
#include <rtc_base/ssl_adapter.h>
#include <rtc_base/ssl_stream_adapter.h>
#include <rtc_base/thread.h>

// libyuv
#include <libyuv/convert_from.h>
#include <libyuv/video_common.h>

// Abseil
#include <absl/memory/memory.h>
#include <absl/strings/ascii.h>
#include <absl/strings/str_split.h>

#ifdef _WIN32
#include <winsock2.h>
#include <ws2tcpip.h>
#else
#include <netdb.h>
#include <sys/socket.h>
#include <unistd.h>
#endif

#ifdef _WIN32
#include <rtc_base/win/scoped_com_initializer.h>
#endif

#include <openssl/err.h>
#include <openssl/ssl.h>

class PeerConnectionFactory : public webrtc::RefCountInterface {
 public:
  static webrtc::scoped_refptr<PeerConnectionFactory> Create() {
    webrtc::InitializeSSL();

    auto c = webrtc::make_ref_counted<PeerConnectionFactory>();

    c->network_thread_ = webrtc::Thread::CreateWithSocketServer();
    c->network_thread_->Start();
    c->worker_thread_ = webrtc::Thread::Create();
    c->worker_thread_->Start();
    c->signaling_thread_ = webrtc::Thread::Create();
    c->signaling_thread_->Start();

    webrtc::PeerConnectionFactoryDependencies dependencies;
    auto env = webrtc::CreateEnvironment();
    dependencies.network_thread = c->network_thread_.get();
    dependencies.worker_thread = c->worker_thread_.get();
    dependencies.signaling_thread = c->signaling_thread_.get();
    dependencies.event_log_factory =
        absl::make_unique<webrtc::RtcEventLogFactory>();

    dependencies.adm = c->worker_thread_->BlockingCall([&] {
      return webrtc::CreateAudioDeviceModule(
          env, webrtc::AudioDeviceModule::kDummyAudio);
    });

    dependencies.audio_encoder_factory =
        webrtc::CreateBuiltinAudioEncoderFactory();
    dependencies.audio_decoder_factory =
        webrtc::CreateBuiltinAudioDecoderFactory();

    dependencies.video_encoder_factory =
        webrtc::CreateBuiltinVideoEncoderFactory();
    dependencies.video_decoder_factory =
        webrtc::CreateBuiltinVideoDecoderFactory();

    dependencies.audio_mixer = nullptr;
    dependencies.audio_processing_builder =
        std::make_unique<webrtc::BuiltinAudioProcessingBuilder>();

    webrtc::EnableMedia(dependencies);

    c->factory_ =
        webrtc::CreateModularPeerConnectionFactory(std::move(dependencies));

    if (c->factory_ == nullptr) {
      RTC_LOG(LS_ERROR) << "Failed to create PeerConnectionFactory";
      return nullptr;
    }

    webrtc::PeerConnectionFactoryInterface::Options factory_options;
    factory_options.disable_encryption = false;
    factory_options.ssl_max_version = webrtc::SSL_PROTOCOL_DTLS_12;
    c->factory_->SetOptions(factory_options);

    return c;
  }

  ~PeerConnectionFactory() {
    factory_ = nullptr;
    network_thread_->Stop();
    worker_thread_->Stop();
    signaling_thread_->Stop();

    // webrtc::CleanupSSL();
  }

  webrtc::Thread* network_thread() const { return network_thread_.get(); }
  webrtc::Thread* worker_thread() const { return worker_thread_.get(); }
  webrtc::Thread* signaling_thread() const { return signaling_thread_.get(); }
  webrtc::scoped_refptr<webrtc::PeerConnectionFactoryInterface>
  peer_connection_factory() const {
    return factory_;
  }

 private:
  std::unique_ptr<webrtc::Thread> network_thread_;
  std::unique_ptr<webrtc::Thread> worker_thread_;
  std::unique_ptr<webrtc::Thread> signaling_thread_;
  webrtc::scoped_refptr<webrtc::PeerConnectionFactoryInterface> factory_;
};

// CreateSessionDescriptionObserver のコールバックを関数オブジェクトで扱えるようにするためのクラス
class CreateSessionDescriptionThunk
    : public webrtc::CreateSessionDescriptionObserver {
 public:
  static webrtc::scoped_refptr<CreateSessionDescriptionThunk> Create(
      std::function<void(webrtc::SessionDescriptionInterface*)> on_success,
      std::function<void(webrtc::RTCError)> on_failure) {
    return webrtc::make_ref_counted<CreateSessionDescriptionThunk>(
        std::move(on_success), std::move(on_failure));
  }

 protected:
  CreateSessionDescriptionThunk(
      std::function<void(webrtc::SessionDescriptionInterface*)> on_success,
      std::function<void(webrtc::RTCError)> on_failure)
      : on_success_(std::move(on_success)),
        on_failure_(std::move(on_failure)) {}
  void OnSuccess(webrtc::SessionDescriptionInterface* desc) override {
    auto f = std::move(on_success_);
    on_success_ = nullptr;
    if (f) {
      f(desc);
    }
  }
  void OnFailure(webrtc::RTCError error) override {
    RTC_LOG(LS_ERROR) << "Failed to create session description : "
                      << webrtc::ToString(error.type()) << ": "
                      << error.message();
    auto f = std::move(on_failure_);
    on_failure_ = nullptr;
    if (f) {
      f(error);
    }
  }

 private:
  std::function<void(webrtc::SessionDescriptionInterface*)> on_success_;
  std::function<void(webrtc::RTCError)> on_failure_;
};

// SetLocalDescriptionObserverInterface のコールバックを関数オブジェクトで扱えるようにするためのクラス
class SetLocalDescriptionThunk
    : public webrtc::SetLocalDescriptionObserverInterface {
 public:
  static webrtc::scoped_refptr<SetLocalDescriptionThunk> Create(
      std::function<void(webrtc::RTCError)> on_complete) {
    return webrtc::make_ref_counted<SetLocalDescriptionThunk>(
        std::move(on_complete));
  }

 protected:
  SetLocalDescriptionThunk(std::function<void(webrtc::RTCError)> on_complete)
      : on_complete_(std::move(on_complete)) {}
  void OnSetLocalDescriptionComplete(webrtc::RTCError error) override {
    auto f = std::move(on_complete_);
    on_complete_ = nullptr;
    if (f) {
      f(error);
    }
  }

 private:
  std::function<void(webrtc::RTCError)> on_complete_;
};

// SetRemoteDescriptionObserverInterface のコールバックを関数オブジェクトで扱えるようにするためのクラス
class SetRemoteDescriptionThunk
    : public webrtc::SetRemoteDescriptionObserverInterface {
 public:
  static webrtc::scoped_refptr<SetRemoteDescriptionThunk> Create(
      std::function<void(webrtc::RTCError)> on_complete) {
    return webrtc::make_ref_counted<SetRemoteDescriptionThunk>(
        std::move(on_complete));
  }

 protected:
  SetRemoteDescriptionThunk(std::function<void(webrtc::RTCError)> on_complete)
      : on_complete_(std::move(on_complete)) {}
  void OnSetRemoteDescriptionComplete(webrtc::RTCError error) override {
    auto f = std::move(on_complete_);
    on_complete_ = nullptr;
    if (f) {
      f(error);
    }
  }

 private:
  std::function<void(webrtc::RTCError)> on_complete_;
};

// 適当な URL パーサ
struct URLParts {
  std::string scheme;
  std::string user_pass;
  std::string host;
  std::string port;
  std::string path_query_fragment;

  // 適当 URL パース
  // scheme://[user_pass@]host[:port][/path_query_fragment]
  static bool Parse(std::string url, URLParts& parts) {
    auto n = url.find("://");
    if (n == std::string::npos) {
      return false;
    }
    parts.scheme = url.substr(0, n);

    n += 3;
    auto m = url.find('/', n);
    std::string user_pass_host_port;
    if (m == std::string::npos) {
      user_pass_host_port = url.substr(n);
      parts.path_query_fragment = "";
    } else {
      user_pass_host_port = url.substr(n, m - n);
      parts.path_query_fragment = url.substr(m);
    }

    n = 0;
    m = user_pass_host_port.find('@');
    std::string host_port;
    if (m == std::string::npos) {
      parts.user_pass = "";
      host_port = std::move(user_pass_host_port);
    } else {
      parts.user_pass = user_pass_host_port.substr(n, m - n);
      host_port = user_pass_host_port.substr(m + 1);
    }

    n = 0;
    m = host_port.find(':');
    if (m == std::string::npos) {
      parts.host = std::move(host_port);
      parts.port = "";
    } else {
      parts.host = host_port.substr(n, m - n);
      parts.port = host_port.substr(m + 1);
    }

    return true;
  }

  // port を返すが、特に指定されていなかった場合、
  // scheme が https/wss の場合は 443、それ以外の場合は 80 を返す
  std::string GetPort() const {
    if (!port.empty()) {
      return port;
    }
    if (scheme == "wss" || scheme == "https") {
      return "443";
    } else {
      return "80";
    }
  }
};

class AnsiRenderer : public webrtc::VideoSinkInterface<webrtc::VideoFrame> {
 public:
  explicit AnsiRenderer() {}

  void OnFrame(const webrtc::VideoFrame& frame) override {
    // width_, height_ に合わせてリサイズ
    webrtc::scoped_refptr<webrtc::I420Buffer> buf =
        webrtc::I420Buffer::Create(width_, height_);
    buf->ScaleFrom(*frame.video_frame_buffer()->ToI420());

    // ARGB に変換
    std::unique_ptr<uint8_t[]> image(new uint8_t[width_ * height_ * 4]);
    libyuv::ConvertFromI420(buf->DataY(), buf->StrideY(), buf->DataU(),
                            buf->StrideU(), buf->DataV(), buf->StrideV(),
                            image.get(), width_ * 4, buf->width(),
                            buf->height(), libyuv::FOURCC_ARGB);

    std::string output;
    output.reserve(width_ * height_ * 20);  // 大体のサイズを予約
    output += "\033[H";

    // 2x1ピクセルを1文字で表現（上半分と下半分の色を使用）
    for (int y = 0; y < height_; y += 2) {
      output += "\033[2K";  // 行をクリア

      for (int x = 0; x < width_; x++) {
        // 上のピクセル（y）
        int upper_offset = (y * width_ + x) * 4;
        uint8_t upper_r = image[upper_offset + 2];
        uint8_t upper_g = image[upper_offset + 1];
        uint8_t upper_b = image[upper_offset + 0];

        // 下のピクセル（y+1）
        uint8_t lower_r = upper_r, lower_g = upper_g, lower_b = upper_b;
        if (y + 1 < height_) {
          int lower_offset = ((y + 1) * width_ + x) * 4;
          lower_r = image[lower_offset + 2];
          lower_g = image[lower_offset + 1];
          lower_b = image[lower_offset + 0];
        }

        // 上半分の色を前景色、下半分の色を背景色として設定
        int upper_color = RgbToAnsi256(upper_r, upper_g, upper_b);
        int lower_color = RgbToAnsi256(lower_r, lower_g, lower_b);

        // 上半分ブロック文字（▀）を使用
        output += "\033[38;5;";
        output += std::to_string(upper_color);
        output += "m\033[48;5;";
        output += std::to_string(lower_color);
        output += "m▀";
      }

      output += "\033[0m\n";  // 色をリセットして改行
    }

    // 一括出力
    std::cout << output << std::flush;
  }

 private:
  int RgbToAnsi256(uint8_t r, uint8_t g, uint8_t b) {
    // 216色キューブ（6x6x6）を使用
    // RGB値を0-5の範囲に変換
    int r6 = (r * 5) / 255;
    int g6 = (g * 5) / 255;
    int b6 = (b * 5) / 255;

    // ANSI 256色の216色キューブは16から始まる
    return 16 + (r6 * 36) + (g6 * 6) + b6;
  }

 private:
  int width_ = 80;
  int height_ = 45;
};

struct SignalingWhepConfig {
  webrtc::scoped_refptr<webrtc::PeerConnectionFactoryInterface> pc_factory;
  webrtc::PeerConnectionObserver* pc_observer;

  std::string signaling_url;
  std::string channel_id;
};

class SignalingWhep : public webrtc::RefCountInterface,
                      public webrtc::PeerConnectionObserver {
 public:
  SignalingWhep(const SignalingWhepConfig& config) : config_(config) {
    video_sink_ = std::make_unique<AnsiRenderer>();
  }

  ~SignalingWhep() { RTC_LOG(LS_INFO) << "SignalingWhep::~SignalingWhep"; }

  static webrtc::scoped_refptr<SignalingWhep> Create(
      const SignalingWhepConfig& config) {
    return webrtc::make_ref_counted<SignalingWhep>(config);
  }

  void Connect() {
    RTC_LOG(LS_INFO) << "SignalingWhep::Connect";
    webrtc::PeerConnectionInterface::RTCConfiguration rtc_config;
    webrtc::PeerConnectionDependencies pc_dependencies(this);
    auto result = config_.pc_factory->CreatePeerConnectionOrError(
        rtc_config, std::move(pc_dependencies));
    if (!result.ok()) {
      RTC_LOG(LS_ERROR) << "Failed to create PeerConnection: "
                        << result.error().message();
      return;
    }
    auto pc = result.value();
    {
      webrtc::RtpTransceiverInit init;
      init.direction = webrtc::RtpTransceiverDirection::kRecvOnly;
      auto transceiver = pc->AddTransceiver(webrtc::MediaType::AUDIO, init);
      if (!transceiver.ok()) {
        RTC_LOG(LS_ERROR) << "Failed to AddTransceiver(audio): error="
                          << transceiver.error().message();
        return;
      }
    }
    {
      webrtc::RtpTransceiverInit init;
      init.direction = webrtc::RtpTransceiverDirection::kRecvOnly;
      auto transceiver = pc->AddTransceiver(webrtc::MediaType::VIDEO, init);
      if (!transceiver.ok()) {
        RTC_LOG(LS_ERROR) << "Failed to AddTransceiver(video): error="
                          << transceiver.error().message();
        return;
      }
    }

    pc_ = pc;
    SetState(State::kConnecting);

    pc->CreateOffer(
        CreateSessionDescriptionThunk::Create(
            [this](webrtc::SessionDescriptionInterface* description) {
              bool succeeded = false;
              ScopeExit on_exit([this, &succeeded]() {
                if (!succeeded) {
                  SetState(State::kClosed);
                }
              });

              auto offer = std::unique_ptr<webrtc::SessionDescriptionInterface>(
                  description);

              std::string offer_sdp;
              if (!offer->ToString(&offer_sdp)) {
                RTC_LOG(LS_ERROR) << "Failed to get SDP";
                SetState(State::kClosed);
                return;
              }
              RTC_LOG(LS_INFO) << "Offer SDP: " << offer_sdp;

              URLParts parts;
              if (!URLParts::Parse(config_.signaling_url, parts)) {
                RTC_LOG(LS_ERROR)
                    << "Failed to parse url: " << config_.signaling_url;
                SetState(State::kClosed);
                return;
              }

              std::string target = parts.path_query_fragment + "/" +
                                   config_.channel_id + "?video_bit_rate=6000";
              std::string req = "POST " + target + " HTTP/1.1\r\n";
              // self->req_.set(boost::beast::http::field::authorization, "Bearer " + self->config_.secret_key);
              req += "Host: " + parts.host + ":" + parts.GetPort() + "\r\n";
              req += "Content-Type: application/sdp\r\n";
              req += "Content-Length: " + std::to_string(offer_sdp.size()) +
                     "\r\n";
              req += "User-Agent: Whep-Client\r\n";
              req += "Connection: close\r\n";
              req += "\r\n";
              req += offer_sdp;
              RTC_LOG(LS_INFO) << "Send request to: " << target;
              SendRequest(
                  parts.host, parts.GetPort(), req,
                  [this, offer_sdp](std::optional<std::string> resp) {
                    bool succeeded = false;
                    ScopeExit on_exit([this, &succeeded]() {
                      if (!succeeded) {
                        SetState(State::kClosed);
                      }
                    });

                    if (!resp) {
                      return;
                    }
                    RTC_LOG(LS_INFO) << "Received response: " << *resp;

                    // ヘッダーとボディに分割
                    std::map<std::string, std::string> headers;
                    std::string body;
                    auto n = resp->find("\r\n\r\n");
                    if (n == std::string::npos) {
                      RTC_LOG(LS_ERROR) << "Invalid response";
                      return;
                    }
                    auto header_str = resp->substr(0, n);
                    body = resp->substr(n + 4);
                    std::vector<std::string> lines =
                        absl::StrSplit(header_str, "\r\n");
                    for (const auto& line : lines) {
                      std::smatch m;
                      auto r =
                          std::regex_match(line.begin(), line.end(), m,
                                           std::regex(R"(([^:]+):[ \t]*(.+))"));
                      if (r) {
                        headers[absl::AsciiStrToLower(m[1].str())] = m[2].str();
                      }
                    }

                    // link ヘッダーはこんな感じの文字列になってる（見やすさのために改行を入れているが実際は含まない）
                    //
                    // <turn:turn.example.com:3478?transport=udp>; rel="ice-server"; username="user"; credential="credential"; credential-type="password",
                    // <turn:turn.example.com:3478?transport=tcp>; rel="ice-server"; username="user"; credential="credential"; credential-type="password"
                    auto link = headers["link"];
                    if (link.empty()) {
                      RTC_LOG(LS_ERROR) << "No Link header";
                      return;
                    }
                    std::vector<std::string> strs = absl::StrSplit(link, ",");

                    webrtc::PeerConnectionInterface::IceServer server;
                    for (const auto& str : strs) {
                      std::smatch m;
                      if (!std::regex_search(str.begin(), str.end(), m,
                                             std::regex(R"(<([^>]+)>)"))) {
                        RTC_LOG(LS_ERROR)
                            << "Failed to match <...>: str=" << str;
                        return;
                      }
                      server.urls.push_back(m[1].str());
                      if (!std::regex_search(
                              str.begin(), str.end(), m,
                              std::regex(R"|(username="([^"]+)")|"))) {
                        RTC_LOG(LS_ERROR)
                            << "Failed to match username=\"...\": str=" << str;
                        return;
                      }
                      server.username = m[1].str();
                      if (!std::regex_search(
                              str.begin(), str.end(), m,
                              std::regex(R"|(credential="([^"]+)")|"))) {
                        RTC_LOG(LS_ERROR)
                            << "Failed to match credential=\"...\": str="
                            << str;
                        return;
                      }
                      server.password = m[1].str();
                      RTC_LOG(LS_INFO) << "Server: url=" << server.urls.back()
                                       << ", username=" << server.username
                                       << ", password=" << server.password;
                    }
                    webrtc::PeerConnectionInterface::RTCConfiguration config;
                    config.servers.push_back(server);
                    config.type = webrtc::PeerConnectionInterface::
                        IceTransportsType::kRelay;
                    pc_->SetConfiguration(config);

                    auto offer = webrtc::CreateSessionDescription(
                        webrtc::SdpType::kOffer, offer_sdp);
                    pc_->SetLocalDescription(
                        std::move(offer),
                        SetLocalDescriptionThunk::Create([this,
                                                          body](webrtc::RTCError
                                                                    error) {
                          if (!error.ok()) {
                            RTC_LOG(LS_ERROR)
                                << "Failed to SetLocalDescription";
                            SetState(State::kClosed);
                            return;
                          }
                          auto answer = webrtc::CreateSessionDescription(
                              webrtc::SdpType::kAnswer, body);
                          pc_->SetRemoteDescription(
                              std::move(answer),
                              SetRemoteDescriptionThunk::Create(
                                  [this](webrtc::RTCError error) {
                                    if (!error.ok()) {
                                      RTC_LOG(LS_ERROR)
                                          << "Failed to SetRemoteDescription";
                                      SetState(State::kClosed);
                                      return;
                                    }
                                    RTC_LOG(LS_INFO) << "Succeeded to "
                                                        "SetRemoteDescription";
                                  }));
                        }));
                    succeeded = true;
                  });
              succeeded = true;
            },
            [this](webrtc::RTCError error) {
              RTC_LOG(LS_ERROR)
                  << "Failed to CreateOffer: error=" << error.message();
              SetState(State::kClosed);
            })
            .get(),
        webrtc::PeerConnectionInterface::RTCOfferAnswerOptions());
  }

  bool WaitForConnect() {
    RTC_LOG(LS_INFO) << "SignalingWhep::WaitForConnected";
    std::unique_lock<std::mutex> lock(mutex_);
    cv_.wait(lock, [this]() { return state_ != State::kConnecting; });
    return state_ == State::kConnected;
  }

  void Disconnect() {
    RTC_LOG(LS_INFO) << "SignalingWhep::Disconnect";
    DetachVideoSink();
    pc_ = nullptr;
    SetState(State::kClosed);
  }

 private:
  enum State {
    kInit,
    kConnecting,
    kConnected,
    kClosed,
  };

  void SetState(State state) {
    std::unique_lock<std::mutex> lock(mutex_);
    state_ = state;
    cv_.notify_all();
  }

  void DetachVideoSink() {
    if (!video_track_ || !video_sink_) {
      return;
    }
    video_track_->RemoveSink(video_sink_.get());
    video_track_ = nullptr;
  }

#ifdef _WIN32
  using SocketType = SOCKET;
  static constexpr SocketType kInvalidSocket = INVALID_SOCKET;
#else
  using SocketType = int;
  static constexpr SocketType kInvalidSocket = -1;
#endif

  static bool IsInvalidSocket(SocketType sock) {
#ifdef _WIN32
    return sock == INVALID_SOCKET;
#else
    return sock < 0;
#endif
  }

  static void CloseSocket(SocketType sock) {
    if (!IsInvalidSocket(sock)) {
#ifdef _WIN32
      closesocket(sock);
#else
      close(sock);
#endif
    }
  }

  struct ScopeExit {
    std::function<void()> f;
    ScopeExit(std::function<void()> f) : f(std::move(f)) {}
    ~ScopeExit() { f(); }
  };

  // 全て同期的に処理するけど、いつでも非同期に直せるようなインターフェースにしておく
  static void SendRequest(
      const std::string& host,
      const std::string& port,
      const std::string& req,
      std::function<void(std::optional<std::string>)> on_response) {
    RTC_LOG(LS_INFO) << "SignalingWhep::SendRequest";

    std::optional<std::string> response_body;
    ScopeExit on_response_guard(
        [&on_response, &response_body]() { on_response(response_body); });

    addrinfo hints{};
    hints.ai_family = AF_UNSPEC;
    hints.ai_socktype = SOCK_STREAM;
    hints.ai_protocol = IPPROTO_TCP;

    addrinfo* result = nullptr;
    int gai_err = getaddrinfo(host.c_str(), port.c_str(), &hints, &result);
    if (gai_err != 0) {
      std::cerr << "getaddrinfo failed: " << gai_strerror(gai_err) << std::endl;
#ifdef _WIN32
      WSACleanup();
#endif
      on_response(std::nullopt);
      return;
    }
    ScopeExit freeaddrinfo_guard{[result]() { freeaddrinfo(result); }};

    SocketType sock = kInvalidSocket;
    for (addrinfo* rp = result; rp != nullptr; rp = rp->ai_next) {
      sock = static_cast<SocketType>(
          socket(rp->ai_family, rp->ai_socktype, rp->ai_protocol));
      if (IsInvalidSocket(sock)) {
        continue;
      }
      if (connect(sock, rp->ai_addr, static_cast<int>(rp->ai_addrlen)) == 0) {
        break;
      }
      CloseSocket(sock);
      sock = kInvalidSocket;
    }

    if (IsInvalidSocket(sock)) {
      RTC_LOG(LS_ERROR) << "Failed to connect to " << host << ":" << port;
      on_response(std::nullopt);
      return;
    }
    ScopeExit close_socket_guard{[sock]() { CloseSocket(sock); }};

    SSL_CTX* ctx = SSL_CTX_new(TLS_client_method());
    if (!ctx) {
      RTC_LOG(LS_ERROR) << "SSL_CTX_new failed";
      return;
    }
    ScopeExit ssl_ctx_free_guard{[ctx]() { SSL_CTX_free(ctx); }};
    SSL_CTX_set_min_proto_version(ctx, TLS1_2_VERSION);
    SSL_CTX_set_max_proto_version(ctx, TLS1_3_VERSION);
    SSL_CTX_set_options(ctx, SSL_OP_ALL | SSL_OP_NO_SSLv2 | SSL_OP_NO_SSLv3 |
                                 SSL_OP_NO_TLSv1 | SSL_OP_NO_TLSv1_1 |
                                 SSL_OP_SINGLE_DH_USE);

    SSL* ssl = SSL_new(ctx);
    if (!ssl) {
      RTC_LOG(LS_ERROR) << "SSL_new failed";
      return;
    }
    ScopeExit ssl_free_guard{[ssl]() { SSL_free(ssl); }};

    if (!SSL_set_tlsext_host_name(ssl, host.c_str())) {
      RTC_LOG(LS_ERROR) << "Failed to set SNI: ec=" << ERR_get_error();
      return;
    }

    SSL_set_fd(ssl, static_cast<int>(sock));
    if (SSL_connect(ssl) != 1) {
      RTC_LOG(LS_ERROR) << "SSL_connect failed: ec=" << ERR_get_error();
      return;
    }

    if (SSL_write(ssl, req.c_str(), req.size()) <= 0) {
      RTC_LOG(LS_ERROR) << "SSL_write failed: ec=" << ERR_get_error();
      return;
    }

    std::string resp;
    resp.reserve(4096);
    char buf[4096];
    for (;;) {
      int n = SSL_read(ssl, buf, sizeof(buf));
      if (n <= 0) {
        break;
      }
      resp.append(buf, n);
    }
    response_body = resp;
  }

  // webrtc::PeerConnectionObserver の実装
 private:
  void OnSignalingChange(
      webrtc::PeerConnectionInterface::SignalingState new_state) override {
    RTC_LOG(LS_INFO) << "OnSignalingChange: new_state="
                     << webrtc::PeerConnectionInterface::AsString(new_state);
  }
  void OnDataChannel(webrtc::scoped_refptr<webrtc::DataChannelInterface>
                         data_channel) override {}
  void OnStandardizedIceConnectionChange(
      webrtc::PeerConnectionInterface::IceConnectionState new_state) override {}
  void OnConnectionChange(
      webrtc::PeerConnectionInterface::PeerConnectionState new_state) override {
    RTC_LOG(LS_INFO) << "OnConnectionChange: new_state="
                     << webrtc::PeerConnectionInterface::AsString(new_state);
    if (new_state ==
        webrtc::PeerConnectionInterface::PeerConnectionState::kConnected) {
      SetState(State::kConnected);
    } else if (new_state == webrtc::PeerConnectionInterface::
                                PeerConnectionState::kFailed ||
               new_state == webrtc::PeerConnectionInterface::
                                PeerConnectionState::kClosed) {
      SetState(State::kClosed);
    }
  }
  void OnIceGatheringChange(
      webrtc::PeerConnectionInterface::IceGatheringState new_state) override {}
  void OnIceCandidate(const webrtc::IceCandidate* candidate) override {
  }
  void OnIceCandidateError(const std::string& address,
                           int port,
                           const std::string& url,
                           int error_code,
                           const std::string& error_text) override {}
  void OnTrack(webrtc::scoped_refptr<webrtc::RtpTransceiverInterface>
                   transceiver) override {
    auto receiver = transceiver->receiver();
    if (!receiver) {
      return;
    }
    auto track = receiver->track();
    if (!track) {
      return;
    }
    if (track->kind() != webrtc::MediaStreamTrackInterface::kVideoKind) {
      return;
    }
    auto* video_track = static_cast<webrtc::VideoTrackInterface*>(track.get());
    if (video_track_ && video_track_.get() == video_track) {
      return;
    }
    DetachVideoSink();
    video_track_ = video_track;
    webrtc::VideoSinkWants wants;
    video_track_->AddOrUpdateSink(video_sink_.get(), wants);
  }
  void OnRemoveTrack(
      webrtc::scoped_refptr<webrtc::RtpReceiverInterface> receiver) override {
    auto track = receiver->track();
    if (!track) {
      return;
    }
    if (track->kind() != webrtc::MediaStreamTrackInterface::kVideoKind) {
      return;
    }
    DetachVideoSink();
  }

 private:
  SignalingWhepConfig config_;

  webrtc::scoped_refptr<webrtc::PeerConnectionInterface> pc_;
  std::unique_ptr<AnsiRenderer> video_sink_;
  webrtc::scoped_refptr<webrtc::VideoTrackInterface> video_track_;
  std::mutex video_mutex_;
  std::optional<webrtc::VideoFrame> last_video_frame_;
  bool logged_first_frame_ = false;

  std::mutex mutex_;
  std::condition_variable cv_;
  State state_ = State::kInit;
};

class WhepClient : public webrtc::RefCountInterface {
 public:
  WhepClient() {}
  ~WhepClient() { RTC_LOG(LS_INFO) << "WhepClient dtor"; }
  static webrtc::scoped_refptr<WhepClient> Create() {
    return webrtc::make_ref_counted<WhepClient>();
  }

  void Run() {
    context_ = PeerConnectionFactory::Create();

    SignalingWhepConfig config;
    config.pc_factory = context_->peer_connection_factory();
    config.signaling_url = "http://192.0.2.1/whep";
    config.channel_id = "sora";

    conn_ = SignalingWhep::Create(config);

    conn_->Connect();
    conn_->WaitForConnect();
    std::this_thread::sleep_for(std::chrono::seconds(30));
    conn_->Disconnect();
  }

 private:
  webrtc::scoped_refptr<PeerConnectionFactory> context_;
  webrtc::scoped_refptr<SignalingWhep> conn_;
};

int main() {
#ifdef _WIN32
  webrtc::ScopedCOMInitializer com_initializer(
      webrtc::ScopedCOMInitializer::kMTA);
  if (!com_initializer.Succeeded()) {
    std::cerr << "CoInitializeEx failed" << std::endl;
    return 1;
  }
#endif
#ifdef _WIN32
  WSADATA wsa_data;
  if (WSAStartup(MAKEWORD(2, 2), &wsa_data) != 0) {
    std::cerr << "WSAStartup failed" << std::endl;
    return 1;
  }
  struct WSADeleter {
    ~WSADeleter() { WSACleanup(); }
  } wsa_deleter;
#endif

  //webrtc::LogMessage::LogToDebug(webrtc::LS_INFO);
  //webrtc::LogMessage::LogTimestamps();
  //webrtc::LogMessage::LogThreads();

  auto client = WhepClient::Create();
  client->Run();

  return 0;
}
