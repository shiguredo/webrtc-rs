#include <math.h>
#include <algorithm>
#include <atomic>
#include <chrono>
#include <cmath>
#include <condition_variable>
#include <cstdint>
#include <cstdlib>
#include <cstring>
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
#include <api/rtp_parameters.h>
#include <api/rtp_receiver_interface.h>
#include <api/rtp_transceiver_direction.h>
#include <api/rtp_transceiver_interface.h>
#include <api/scoped_refptr.h>
#include <api/set_local_description_observer_interface.h>
#include <api/set_remote_description_observer_interface.h>
#include <api/video/i420_buffer.h>
#include <api/video/video_frame.h>
#include <api/video/video_frame_buffer.h>
#include <api/video/video_rotation.h>
#include <api/video_codecs/builtin_video_decoder_factory.h>
#include <api/video_codecs/builtin_video_encoder_factory.h>
#include <api/video_codecs/sdp_video_format.h>
#include <media/base/adapted_video_track_source.h>
#include <media/base/codec.h>
#include <media/base/codec_comparators.h>
#include <pc/media_factory.h>
#include <pc/session_description.h>
#include <rtc_base/crypto_random.h>
#include <rtc_base/logging.h>
#include <rtc_base/ssl_adapter.h>
#include <rtc_base/ssl_stream_adapter.h>
#include <rtc_base/thread.h>
#include <rtc_base/time_utils.h>
#include <rtc_base/timestamp_aligner.h>

// libyuv
#include <libyuv/convert.h>

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

struct FakeVideoCapturerConfig {
  int width = 640;
  int height = 480;
  int fps = 30;
  // 円が一周した時に呼ばれるコールバック
  std::function<void()> on_tick;
};

class FakeVideoCapturer : public webrtc::AdaptedVideoTrackSource {
 public:
  FakeVideoCapturer(FakeVideoCapturerConfig config)
      : config_(std::move(config)) {
    StartCapture();
  }

  ~FakeVideoCapturer() { StopCapture(); }

  static webrtc::scoped_refptr<FakeVideoCapturer> Create(
      FakeVideoCapturerConfig config) {
    return webrtc::make_ref_counted<FakeVideoCapturer>(std::move(config));
  }

  bool is_screencast() const override { return false; }
  std::optional<bool> needs_denoising() const override { return false; }
  webrtc::MediaSourceInterface::SourceState state() const override {
    return webrtc::MediaSourceInterface::kLive;
  }
  bool remote() const override { return false; }

  void StartCapture() {
    if (capture_thread_) {
      return;
    }

    stop_capture_ = false;
    frame_counter_ = 0;
    start_time_ = std::chrono::high_resolution_clock::now();

    capture_thread_ =
        std::make_unique<std::thread>([this] { CaptureThread(); });
  }

  void StopCapture() {
    if (!capture_thread_) {
      return;
    }

    stop_capture_ = true;
    if (capture_thread_->joinable()) {
      capture_thread_->join();
    }
    capture_thread_.reset();
  }

 private:
  void CaptureThread() {
    image_.reset(new uint32_t[config_.width * config_.height]());
    frame_counter_ = 0;

    while (!stop_capture_) {
      auto now = std::chrono::high_resolution_clock::now();

      // 画像を更新
      UpdateImage(now);

      webrtc::scoped_refptr<webrtc::I420Buffer> buffer =
          webrtc::I420Buffer::Create(config_.width, config_.height);

      libyuv::ABGRToI420((const uint8_t*)image_.get(), config_.width * 4,
                         buffer->MutableDataY(), buffer->StrideY(),
                         buffer->MutableDataU(), buffer->StrideU(),
                         buffer->MutableDataV(), buffer->StrideV(),
                         config_.width, config_.height);

      // タイムスタンプを計算
      int64_t timestamp_us =
          std::chrono::duration_cast<std::chrono::microseconds>(now -
                                                                start_time_)
              .count();

      // フレームを送信
      bool captured =
          OnCapturedFrame(webrtc::VideoFrame::Builder()
                              .set_video_frame_buffer(buffer)
                              .set_rotation(webrtc::kVideoRotation_0)
                              .set_timestamp_us(timestamp_us)
                              .build());

      if (captured) {
        // スリープ時間を std::chrono::milliseconds(1000 / config_.fps) にすると
        // 起きるための時間があるからフレームレートが保たれないことがあるので、少し短い時間にする
        std::this_thread::sleep_for(
            std::chrono::milliseconds(1000 / config_.fps - 2));
        frame_counter_ += 1;
      } else {
        std::this_thread::sleep_for(std::chrono::milliseconds(1));
      }
    }
  }

  void UpdateImage(std::chrono::high_resolution_clock::time_point now) {
    // 経過時間を計算
    auto elapsed =
        std::chrono::duration_cast<std::chrono::milliseconds>(now - start_time_)
            .count();

    // 円の位置を計算
    const int radius = std::min(config_.width, config_.height) / 4;
    const int center_x = config_.width / 2;
    const int center_y = config_.height / 2;
    const double angle = 2.0 * M_PI * (elapsed % (1000)) / 1000.0;
    const int circle_x = center_x + static_cast<int>(radius * std::cos(angle));
    const int circle_y = center_y + static_cast<int>(radius * std::sin(angle));

    // 画像をクリア
    std::memset(image_.get(), 0,
                sizeof(uint32_t) * config_.width * config_.height);

    // 円を描画
    const int circle_radius = 100;
    for (int y = -circle_radius; y <= circle_radius; ++y) {
      for (int x = -circle_radius; x <= circle_radius; ++x) {
        if (x * x + y * y <= circle_radius * circle_radius) {
          int draw_x = circle_x + x;
          int draw_y = circle_y + y;
          if (draw_x >= 0 && draw_x < config_.width && draw_y >= 0 &&
              draw_y < config_.height) {
            // 経過時間で色を変える
            uint32_t color = 0xFF000000;
            color |= ((elapsed / 10) % 256) << 16;  // 赤成分を時間で変化させる
            color |= ((elapsed / 5) % 256) << 8;    // 緑成分を時間で変化させる
            color |= (elapsed % 256);               // 青成分を時間で変化させる
            image_[draw_y * config_.width + draw_x] = color;
          }
        }
      }
    }

    // 一周したら on_tick コールバックを呼ぶ
    if (elapsed % 1000 < (1000 / config_.fps)) {
      if (config_.on_tick) {
        config_.on_tick();
      }
    }
  }

  bool OnCapturedFrame(const webrtc::VideoFrame& frame) {
    const int64_t timestamp_us = frame.timestamp_us();
    const int64_t translated_timestamp_us =
        timestamp_aligner_.TranslateTimestamp(timestamp_us,
                                              webrtc::TimeMicros());

    int adapted_width;
    int adapted_height;
    int crop_width;
    int crop_height;
    int crop_x;
    int crop_y;
    if (!AdaptFrame(frame.width(), frame.height(), timestamp_us, &adapted_width,
                    &adapted_height, &crop_width, &crop_height, &crop_x,
                    &crop_y)) {
      return false;
    }

    if (frame.video_frame_buffer()->type() ==
        webrtc::VideoFrameBuffer::Type::kNative) {
      OnFrame(frame);
      return true;
    }

    webrtc::scoped_refptr<webrtc::VideoFrameBuffer> buffer =
        frame.video_frame_buffer();

    if (adapted_width != frame.width() || adapted_height != frame.height()) {
      // Video adapter has requested a down-scale. Allocate a new buffer and
      // return scaled version.
      webrtc::scoped_refptr<webrtc::I420Buffer> i420_buffer =
          webrtc::I420Buffer::Create(adapted_width, adapted_height);
      i420_buffer->ScaleFrom(*buffer->ToI420());
      buffer = i420_buffer;
    }

    OnFrame(webrtc::VideoFrame::Builder()
                .set_video_frame_buffer(buffer)
                .set_rotation(frame.rotation())
                .set_timestamp_us(translated_timestamp_us)
                .build());

    return true;
  }

 private:
  FakeVideoCapturerConfig config_;
  webrtc::TimestampAligner timestamp_aligner_;
  std::unique_ptr<std::thread> capture_thread_;
  std::atomic<bool> stop_capture_{false};
  std::chrono::high_resolution_clock::time_point start_time_;

  std::unique_ptr<uint32_t[]> image_;
  uint32_t frame_counter_ = 0;
};

struct SignalingWhipConfig {
  webrtc::scoped_refptr<webrtc::PeerConnectionFactoryInterface> pc_factory;
  webrtc::PeerConnectionObserver* pc_observer;

  std::string signaling_url;
  std::string channel_id;
  std::optional<std::vector<webrtc::RtpEncodingParameters>> send_encodings;
  webrtc::scoped_refptr<webrtc::VideoTrackSourceInterface> video_source;
};

class SignalingWhip : public webrtc::RefCountInterface,
                      public webrtc::PeerConnectionObserver {
 public:
  SignalingWhip(const SignalingWhipConfig& config) : config_(config) {}

  ~SignalingWhip() { RTC_LOG(LS_INFO) << "SignalingWhip::~SignalingWhip"; }

  static webrtc::scoped_refptr<SignalingWhip> Create(
      const SignalingWhipConfig& config) {
    return webrtc::make_ref_counted<SignalingWhip>(config);
  }

  void Connect() {
    RTC_LOG(LS_INFO) << "SignalingWhip::Connect";

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
      init.direction = webrtc::RtpTransceiverDirection::kSendOnly;
      auto transceiver = pc->AddTransceiver(webrtc::MediaType::AUDIO, init);
      if (!transceiver.ok()) {
        RTC_LOG(LS_ERROR) << "Failed to AddTransceiver(audio): error="
                          << transceiver.error().message();
        return;
      }

      auto cap = config_.pc_factory->GetRtpSenderCapabilities(
          webrtc::MediaType::AUDIO);
      std::vector<webrtc::RtpCodecCapability> codecs;
      for (const webrtc::RtpCodecCapability& codec : cap.codecs) {
        if (codec.name == "OPUS") {
          codecs.push_back(codec);
          break;
        }
      }
      transceiver.value()->SetCodecPreferences(codecs);
    }
    webrtc::RtpTransceiverInit video_init;
    if (config_.video_source != nullptr) {
      std::string video_track_id = webrtc::CreateRandomString(16);
      auto video_track = config_.pc_factory->CreateVideoTrack(
          config_.video_source, video_track_id);
      auto& init = video_init;
      init.direction = webrtc::RtpTransceiverDirection::kSendOnly;
      init.stream_ids = {webrtc::CreateRandomString(16)};
      if (config_.send_encodings) {
        init.send_encodings = *config_.send_encodings;
      }
      auto transceiver = pc->AddTransceiver(video_track, init);
      if (!transceiver.ok()) {
        RTC_LOG(LS_ERROR) << "Failed to AddTransceiver(video): error="
                          << transceiver.error().message();
        return;
      }

      auto cap = config_.pc_factory->GetRtpSenderCapabilities(
          webrtc::MediaType::VIDEO);
      for (const webrtc::RtpCodecCapability& codec : cap.codecs) {
        RTC_LOG(LS_WARNING) << "codec: " << codec.name;
        for (const auto& param : codec.parameters) {
          RTC_LOG(LS_WARNING) << "  " << param.first << ": " << param.second;
        }
      }
      std::vector<webrtc::RtpCodecCapability> codecs;
      for (const auto& send_encoding : init.send_encodings) {
        RTC_LOG(LS_WARNING)
            << "send_encoding: "
            << (send_encoding.codec ? send_encoding.codec->name : "none");
        for (const webrtc::RtpCodecCapability& codec : cap.codecs) {
          auto codec_format =
              webrtc::SdpVideoFormat(codec.name, codec.parameters);
          if (send_encoding.codec) {
            auto encoding_format = webrtc::SdpVideoFormat(
                send_encoding.codec->name, send_encoding.codec->parameters);
            if (codec_format == encoding_format) {
              RTC_LOG(LS_WARNING) << "match codec: " << codec.name;
              auto it = std::find_if(
                  codecs.begin(), codecs.end(),
                  [&codec_format](const webrtc::RtpCodecCapability& c) {
                    auto format = webrtc::SdpVideoFormat(c.name, c.parameters);
                    return codec_format == format;
                  });
              if (it == codecs.end()) {
                RTC_LOG(LS_WARNING) << "add codec: " << codec.name;
                codecs.push_back(codec);
              }
              break;
            }
          }
        }
      }
      //for (const webrtc::RtpCodecCapability& codec : cap.codecs) {
      //  if (codec.name == "H264") {
      //    codecs.push_back(codec);
      //    break;
      //  }
      //}
      //for (const webrtc::RtpCodecCapability& codec : cap.codecs) {
      //  if (codec.name == "H265") {
      //    codecs.push_back(codec);
      //    break;
      //  }
      //}
      //for (const webrtc::RtpCodecCapability& codec : cap.codecs) {
      //  if (codec.name == "VP9") {
      //    codecs.push_back(codec);
      //    break;
      //  }
      //}
      //for (const webrtc::RtpCodecCapability& codec : cap.codecs) {
      //  if (codec.name == "AV1") {
      //    codecs.push_back(codec);
      //    break;
      //  }
      //}
      for (const webrtc::RtpCodecCapability& codec : cap.codecs) {
        if (codec.name == "rtx") {
          codecs.push_back(codec);
          break;
        }
      }
      transceiver.value()->SetCodecPreferences(codecs);
    }

    pc_ = pc;
    SetState(State::kConnecting);

    pc->CreateOffer(
        CreateSessionDescriptionThunk::Create(
            [this,
             video_init](webrtc::SessionDescriptionInterface* description) {
              bool succeeded = false;
              ScopeExit on_exit([this, &succeeded]() {
                if (!succeeded) {
                  SetState(State::kClosed);
                }
              });

              auto offer = std::unique_ptr<webrtc::SessionDescriptionInterface>(
                  description);

              // 各 RtpEncodingParameters の利用するコーデックを関連付ける
              std::map<std::string, webrtc::Codec> rid_codec_map;
              auto& content = offer->description()->contents()[1];
              auto media_desc = content.media_description();
              for (auto& send_encoding : video_init.send_encodings) {
                RTC_LOG(LS_WARNING)
                    << "send_encoding: " << send_encoding.codec->name;
                for (auto& codec : media_desc->codecs()) {
                  RTC_LOG(LS_WARNING) << "codec: " << codec.name;
                  if (send_encoding.codec &&
                      webrtc::IsSameRtpCodec(codec, *send_encoding.codec)) {
                    RTC_LOG(LS_WARNING) << "rid=" << send_encoding.rid
                                        << " codec=" << codec.name
                                        << " payload_type=" << codec.id;
                    rid_codec_map[send_encoding.rid] = codec;
                  }
                }
              }
              auto& track = media_desc->mutable_streams()[0];
              auto rids = track.rids();
              for (auto& rid : rids) {
                //if (rid.rid == "r0" || rid.rid == "r1") {
                //  continue;
                //}
                auto it = rid_codec_map.find(rid.rid);
                if (it == rid_codec_map.end()) {
                  continue;
                }
                rid.codecs.clear();
                rid.codecs.push_back(it->second);
              }
              track.set_rids(rids);

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
              req += "User-Agent: Whip-Client\r\n";
              req += "Connection: close\r\n";
              req += "\r\n";
              req += offer_sdp;
              RTC_LOG(LS_INFO) << "Send request to: " << target;
              SendRequest(
                  parts.host, parts.GetPort(), req,
                  [this, offer_sdp,
                   video_init](std::optional<std::string> resp) {
                    bool succeeded = false;
                    ScopeExit on_exit([this, &succeeded]() {
                      if (!succeeded) {
                        SetState(State::kClosed);
                      }
                    });

                    if (!resp) {
                      return;
                    }

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
                        SetLocalDescriptionThunk::Create([this, video_init,
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
                                  [this, video_init](webrtc::RTCError error) {
                                    if (!error.ok()) {
                                      RTC_LOG(LS_ERROR)
                                          << "Failed to SetRemoteDescription";
                                      SetState(State::kClosed);
                                      return;
                                    }
                                    RTC_LOG(LS_INFO) << "Succeeded to "
                                                        "SetRemoteDescription";
                                    auto p =
                                        pc_->GetSenders()[1]->GetParameters();
                                    for (int i = 0; i < p.encodings.size();
                                         i++) {
                                      p.encodings[i].codec =
                                          video_init.send_encodings[i].codec;
                                      p.encodings[i].scalability_mode =
                                          video_init.send_encodings[i]
                                              .scalability_mode;
                                    }
                                    pc_->GetSenders()[1]->SetParameters(p);
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
    RTC_LOG(LS_INFO) << "SignalingWhip::WaitForConnected";
    std::unique_lock<std::mutex> lock(mutex_);
    cv_.wait(lock, [this]() { return state_ != State::kConnecting; });
    return state_ == State::kConnected;
  }

  void Disconnect() {
    RTC_LOG(LS_INFO) << "SignalingWhip::Disconnect";
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
    RTC_LOG(LS_INFO) << "SignalingWhip::SendRequest";

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
  void OnIceCandidate(const webrtc::IceCandidateInterface* candidate) override {
  }
  void OnIceCandidateError(const std::string& address,
                           int port,
                           const std::string& url,
                           int error_code,
                           const std::string& error_text) override {}
  void OnTrack(webrtc::scoped_refptr<webrtc::RtpTransceiverInterface>
                   transceiver) override {}
  void OnRemoveTrack(
      webrtc::scoped_refptr<webrtc::RtpReceiverInterface> receiver) override {}

 private:
  SignalingWhipConfig config_;

  webrtc::scoped_refptr<webrtc::PeerConnectionInterface> pc_;

  std::mutex mutex_;
  std::condition_variable cv_;
  State state_ = State::kInit;
};

class WhipClient : public webrtc::RefCountInterface {
 public:
  WhipClient() {}
  ~WhipClient() { RTC_LOG(LS_INFO) << "WhipClient dtor"; }
  static webrtc::scoped_refptr<WhipClient> Create() {
    return webrtc::make_ref_counted<WhipClient>();
  }

  void Run() {
    context_ = PeerConnectionFactory::Create();

    FakeVideoCapturerConfig fake_config;
    fake_config.width = 1920;
    fake_config.height = 1080;
    fake_config.fps = 30;
    auto video_source = FakeVideoCapturer::Create(fake_config);

    SignalingWhipConfig config;
    config.pc_factory = context_->peer_connection_factory();
    config.signaling_url = "http://192.0.2.1/whip";
    config.channel_id = "sora";
    config.video_source = video_source;

    auto& send_encodings = config.send_encodings.emplace();
    webrtc::RtpCodecCapability vp9_codec;
    webrtc::RtpCodecCapability av1_codec;
    webrtc::RtpCodecCapability h264_codec;
    webrtc::RtpCodecCapability h265_codec;
    vp9_codec.kind = webrtc::MediaType::VIDEO;
    vp9_codec.name = "VP9";
    vp9_codec.parameters["profile-id"] = "0";
    vp9_codec.clock_rate = 90000;
    av1_codec.kind = webrtc::MediaType::VIDEO;
    av1_codec.name = "AV1";
    av1_codec.clock_rate = 90000;
    av1_codec.parameters["level-idx"] = "5";
    av1_codec.parameters["profile"] = "0";
    av1_codec.parameters["tier"] = "0";
    h264_codec.kind = webrtc::MediaType::VIDEO;
    h264_codec.name = "H264";
    h264_codec.clock_rate = 90000;
    h264_codec.parameters["profile-level-id"] = "42001f";
    h264_codec.parameters["level-asymmetry-allowed"] = "1";
    h264_codec.parameters["packetization-mode"] = "1";
    h265_codec.kind = webrtc::MediaType::VIDEO;
    h265_codec.name = "H265";
    h265_codec.clock_rate = 90000;
    send_encodings.resize(3);
    send_encodings[0].rid = "r0";
    send_encodings[0].scale_resolution_down_by = 4.0;
    send_encodings[1].rid = "r1";
    send_encodings[1].scale_resolution_down_by = 2.0;
    send_encodings[2].rid = "r2";
    send_encodings[2].scale_resolution_down_by = 1.0;

    // send_encodings[0].codec = av1_codec;
    // send_encodings[0].scalability_mode = "L1T2";
    // send_encodings[0].codec = h264_codec;
    send_encodings[0].codec = av1_codec;

    // send_encodings[1].codec = av1_codec;
    // send_encodings[1].scalability_mode = "L1T2";
    // send_encodings[1].codec = h264_codec;
    send_encodings[1].codec = av1_codec;

    // send_encodings[2].codec = av1_codec;
    // send_encodings[2].scalability_mode = "L1T2";
    // send_encodings[2].codec = h264_codec;
    // send_encodings[2].codec = h265_codec;
    send_encodings[2].codec = av1_codec;

    conn_ = SignalingWhip::Create(config);

    conn_->Connect();
    conn_->WaitForConnect();
    std::this_thread::sleep_for(std::chrono::seconds(30));
    conn_->Disconnect();
  }

 private:
  webrtc::scoped_refptr<PeerConnectionFactory> context_;
  webrtc::scoped_refptr<SignalingWhip> conn_;
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

  webrtc::LogMessage::LogToDebug(webrtc::LS_INFO);
  webrtc::LogMessage::LogTimestamps();
  webrtc::LogMessage::LogThreads();

  auto client = WhipClient::Create();
  client->Run();

  return 0;
}
