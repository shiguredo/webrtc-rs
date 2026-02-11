#include <math.h>
#include <stdatomic.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>

#include <netdb.h>
#include <sys/socket.h>

#include <openssl/err.h>
#include <openssl/ssl.h>

#ifndef M_PI
#define M_PI 3.14159265358979323846
#endif

// webrtc_c
#include "webrtc_c.h"

// POSIX
#ifdef __linux__
#include <bits/pthreadtypes.h>
#endif
#include <pthread.h>
#include <unistd.h>

// webrtc::RtpCodecCapability を SdpVideoFormat に変換するヘルパー
static struct webrtc_SdpVideoFormat_unique*
whip_SdpVideoFormat_from_RtpCodecCapability(
    struct webrtc_RtpCodecCapability* codec) {
  struct std_string* name = webrtc_RtpCodecCapability_get_name(codec);
  return webrtc_SdpVideoFormat_new(
      std_string_c_str(name), std_string_size(name),
      webrtc_RtpCodecCapability_get_parameters(codec));
}

// webrtc::RtpCodecCapability をフォーマット比較するヘルパー
static int whip_RtpCodecCapability_is_same_format(
    struct webrtc_RtpCodecCapability* lhs,
    struct webrtc_RtpCodecCapability* rhs) {
  struct webrtc_SdpVideoFormat_unique* a =
      whip_SdpVideoFormat_from_RtpCodecCapability(lhs);
  struct webrtc_SdpVideoFormat_unique* b =
      whip_SdpVideoFormat_from_RtpCodecCapability(rhs);
  int result = webrtc_SdpVideoFormat_is_equal(
      webrtc_SdpVideoFormat_unique_get(a), webrtc_SdpVideoFormat_unique_get(b));
  webrtc_SdpVideoFormat_unique_delete(a);
  webrtc_SdpVideoFormat_unique_delete(b);
  return result;
}

static void whip_OnSendRequestResponse(char* resp, void* user_data);

struct PeerConnectionFactory {
  struct webrtc_RefCountInterface_ref* ref;
  struct webrtc_Thread_unique* network_thread;
  struct webrtc_Thread_unique* worker_thread;
  struct webrtc_Thread_unique* signaling_thread;
  struct webrtc_PeerConnectionFactoryInterface_refcounted* factory;
};

void PeerConnectionFactory_delete(void* user_data) {
  struct PeerConnectionFactory* p = (struct PeerConnectionFactory*)user_data;
  if (p->factory != NULL) {
    webrtc_PeerConnectionFactoryInterface_Release(
        webrtc_PeerConnectionFactoryInterface_refcounted_get(p->factory));
  }
  if (p->network_thread != NULL) {
    webrtc_Thread_Stop(webrtc_Thread_unique_get(p->network_thread));
    webrtc_Thread_unique_delete(p->network_thread);
  }
  if (p->worker_thread != NULL) {
    webrtc_Thread_Stop(webrtc_Thread_unique_get(p->worker_thread));
    webrtc_Thread_unique_delete(p->worker_thread);
  }
  if (p->signaling_thread != NULL) {
    webrtc_Thread_Stop(webrtc_Thread_unique_get(p->signaling_thread));
    webrtc_Thread_unique_delete(p->signaling_thread);
  }
  free(p);
}

void* _BlockingCall_create_adm(void* env) {
  return webrtc_CreateAudioDeviceModule((struct webrtc_Environment*)env,
                                        webrtc_AudioDeviceModule_kDummyAudio);
}

struct FakeVideoCapturerConfig {
  int width;
  int height;
  int fps;
  void (*on_tick)(void*);
  void* on_tick_user_data;
};

struct FakeVideoCapturer {
  struct webrtc_RefCountInterface_ref* ref;
  struct FakeVideoCapturerConfig config;
  struct webrtc_TimestampAligner_unique* timestamp_aligner;
  struct webrtc_AdaptedVideoTrackSource_refcounted* source;
  pthread_t capture_thread;
  bool capture_thread_started;
  atomic_bool stop_capture;
  int64_t start_time_ms;
  uint32_t* image;
};

static void* _FakeVideoCapturer_CaptureThread(void* arg);

void FakeVideoCapturer_delete(void* user_data) {
  struct FakeVideoCapturer* p = (struct FakeVideoCapturer*)user_data;
  if (p->capture_thread_started) {
    atomic_store(&p->stop_capture, true);
    pthread_join(p->capture_thread, NULL);
  }
  if (p->source != NULL) {
    webrtc_AdaptedVideoTrackSource_Release(
        webrtc_AdaptedVideoTrackSource_refcounted_get(p->source));
  }
  if (p->timestamp_aligner != NULL) {
    webrtc_TimestampAligner_unique_delete(p->timestamp_aligner);
  }
  if (p->image != NULL) {
    free(p->image);
  }
  free(p);
}

struct FakeVideoCapturer* FakeVideoCapturer_Create(
    struct FakeVideoCapturerConfig* config) {
  struct FakeVideoCapturer* p =
      (struct FakeVideoCapturer*)calloc(1, sizeof(struct FakeVideoCapturer));
  p->ref = webrtc_RefCountInterface_Create(FakeVideoCapturer_delete, p);
  p->config = *config;
  if (p->config.width == 0) {
    p->config.width = 640;
  }
  if (p->config.height == 0) {
    p->config.height = 480;
  }
  if (p->config.fps == 0) {
    p->config.fps = 30;
  }
  p->timestamp_aligner = webrtc_TimestampAligner_new();
  p->source = webrtc_AdaptedVideoTrackSource_Create();
  webrtc_AdaptedVideoTrackSource_AddRef(
      webrtc_AdaptedVideoTrackSource_refcounted_get(p->source));
  atomic_init(&p->stop_capture, false);
  p->start_time_ms = webrtc_TimeMillis();
  p->image =
      (uint32_t*)calloc(p->config.width * p->config.height, sizeof(uint32_t));
  p->capture_thread_started = false;

  pthread_t tid;
  int ret = pthread_create(&tid, NULL, _FakeVideoCapturer_CaptureThread, p);
  if (ret == 0) {
    p->capture_thread = tid;
    p->capture_thread_started = true;
  }
  return p;
}

// 簡易的な URL パーサ
struct URLParts {
  char* scheme;
  char* user_pass;
  char* host;
  char* port;
  char* path_query_fragment;
};

static void URLParts_clear(struct URLParts* parts) {
  free(parts->scheme);
  free(parts->user_pass);
  free(parts->host);
  free(parts->port);
  free(parts->path_query_fragment);
  parts->scheme = NULL;
  parts->user_pass = NULL;
  parts->host = NULL;
  parts->port = NULL;
  parts->path_query_fragment = NULL;
}

static int URLParts_Parse(const char* url, struct URLParts* parts) {
  URLParts_clear(parts);
  if (url == NULL) {
    return 0;
  }
  const char* p = strstr(url, "://");
  if (p == NULL) {
    return 0;
  }
  size_t scheme_len = (size_t)(p - url);
  parts->scheme = strndup(url, scheme_len);

  p += 3;  // skip ://
  const char* slash = strchr(p, '/');
  size_t uphp_len = 0;
  if (slash == NULL) {
    uphp_len = strlen(p);
    parts->path_query_fragment = strdup("");
  } else {
    uphp_len = (size_t)(slash - p);
    parts->path_query_fragment = strdup(slash);
  }
  char* user_pass_host_port = strndup(p, uphp_len);

  char* at = strchr(user_pass_host_port, '@');
  char* host_port = NULL;
  if (at == NULL) {
    parts->user_pass = strdup("");
    host_port = user_pass_host_port;
  } else {
    parts->user_pass =
        strndup(user_pass_host_port, (size_t)(at - user_pass_host_port));
    host_port = strdup(at + 1);
    free(user_pass_host_port);
  }

  char* colon = strchr(host_port, ':');
  if (colon == NULL) {
    parts->host = host_port;
    parts->port = strdup("");
  } else {
    parts->host = strndup(host_port, (size_t)(colon - host_port));
    parts->port = strdup(colon + 1);
    free(host_port);
  }

  return 1;
}

static const char* URLParts_GetPort(struct URLParts* parts) {
  if (parts->port != NULL && parts->port[0] != '\0') {
    return parts->port;
  }
  if (parts->scheme != NULL && (strcmp(parts->scheme, "wss") == 0 ||
                                strcmp(parts->scheme, "https") == 0)) {
    return "443";
  }
  return "80";
}
struct webrtc_VideoTrackSourceInterface_refcounted* FakeVideoCapturer_GetSource(
    struct FakeVideoCapturer* self) {
  struct webrtc_VideoTrackSourceInterface_refcounted* src =
      webrtc_AdaptedVideoTrackSource_refcounted_cast_to_webrtc_VideoTrackSourceInterface(
          self->source);
  webrtc_VideoTrackSourceInterface_AddRef(
      webrtc_VideoTrackSourceInterface_refcounted_get(src));
  return src;
}

void* _FakeVideoCapturer_CaptureThread(void* arg) {
  struct FakeVideoCapturer* cap = (struct FakeVideoCapturer*)arg;

  while (!atomic_load(&cap->stop_capture)) {
    int64_t now_ms = webrtc_TimeMillis();

    // 画像をクリア
    memset(cap->image, 0,
           sizeof(uint32_t) * cap->config.width * cap->config.height);

    // 円の位置計算
    int64_t elapsed = now_ms - cap->start_time_ms;
    const int radius =
        (cap->config.width < cap->config.height ? cap->config.width
                                                : cap->config.height) /
        4;
    const int center_x = cap->config.width / 2;
    const int center_y = cap->config.height / 2;
    double angle = 2.0 * M_PI * (elapsed % 1000) / 1000.0;
    int circle_x = center_x + (int)(radius * cos(angle));
    int circle_y = center_y + (int)(radius * sin(angle));

    // 円を描画
    const int circle_radius = 100;
    for (int y = -circle_radius; y <= circle_radius; ++y) {
      for (int x = -circle_radius; x <= circle_radius; ++x) {
        if (x * x + y * y <= circle_radius * circle_radius) {
          int draw_x = circle_x + x;
          int draw_y = circle_y + y;
          if (draw_x >= 0 && draw_x < cap->config.width && draw_y >= 0 &&
              draw_y < cap->config.height) {
            uint32_t color = 0xFF000000;
            color |= ((elapsed / 10) % 256) << 16;
            color |= ((elapsed / 5) % 256) << 8;
            color |= (elapsed % 256);
            cap->image[draw_y * cap->config.width + draw_x] = color;
          }
        }
      }
    }

    // on_tick 判定
    if (elapsed % 1000 < (1000 / cap->config.fps)) {
      if (cap->config.on_tick) {
        cap->config.on_tick(cap->config.on_tick_user_data);
      }
    }

    // I420 に変換
    struct webrtc_I420Buffer_refcounted* buffer =
        webrtc_I420Buffer_Create(cap->config.width, cap->config.height);
    int ret = libyuv_ABGRToI420(
        (const uint8_t*)cap->image, cap->config.width * 4,
        webrtc_I420Buffer_MutableDataY(
            webrtc_I420Buffer_refcounted_get(buffer)),
        webrtc_I420Buffer_StrideY(webrtc_I420Buffer_refcounted_get(buffer)),
        webrtc_I420Buffer_MutableDataU(
            webrtc_I420Buffer_refcounted_get(buffer)),
        webrtc_I420Buffer_StrideU(webrtc_I420Buffer_refcounted_get(buffer)),
        webrtc_I420Buffer_MutableDataV(
            webrtc_I420Buffer_refcounted_get(buffer)),
        webrtc_I420Buffer_StrideV(webrtc_I420Buffer_refcounted_get(buffer)),
        cap->config.width, cap->config.height);
    if (ret != 0) {
      // 変換失敗時はフレームを破棄して次へ進む。
      webrtc_I420Buffer_Release(webrtc_I420Buffer_refcounted_get(buffer));
      webrtc_Thread_SleepMs(1);
      continue;
    }

    int64_t timestamp_us = (now_ms - cap->start_time_ms) * 1000;
    struct webrtc_VideoFrame_unique* frame =
        webrtc_VideoFrame_Create(buffer, webrtc_VideoRotation_0, timestamp_us);
    webrtc_I420Buffer_Release(webrtc_I420Buffer_refcounted_get(buffer));

    int adapted_width;
    int adapted_height;
    int crop_width;
    int crop_height;
    int crop_x;
    int crop_y;
    int ok = webrtc_AdaptedVideoTrackSource_AdaptFrame(
        webrtc_AdaptedVideoTrackSource_refcounted_get(cap->source),
        webrtc_VideoFrame_width(webrtc_VideoFrame_unique_get(frame)),
        webrtc_VideoFrame_height(webrtc_VideoFrame_unique_get(frame)),
        timestamp_us, &adapted_width, &adapted_height, &crop_width,
        &crop_height, &crop_x, &crop_y);
    if (!ok) {
      webrtc_VideoFrame_unique_delete(frame);
      webrtc_Thread_SleepMs(1);
      continue;
    }

    if (adapted_width !=
            webrtc_VideoFrame_width(webrtc_VideoFrame_unique_get(frame)) ||
        adapted_height !=
            webrtc_VideoFrame_height(webrtc_VideoFrame_unique_get(frame))) {
      struct webrtc_I420Buffer_refcounted* buf =
          webrtc_VideoFrame_video_frame_buffer(
              webrtc_VideoFrame_unique_get(frame));
      struct webrtc_I420Buffer_refcounted* scaled =
          webrtc_I420Buffer_Create(adapted_width, adapted_height);
      webrtc_I420Buffer_ScaleFrom(webrtc_I420Buffer_refcounted_get(scaled),
                                  webrtc_I420Buffer_refcounted_get(buf));
      webrtc_I420Buffer_Release(webrtc_I420Buffer_refcounted_get(buf));
      webrtc_VideoFrame_unique_delete(frame);
      frame = webrtc_VideoFrame_Create(
          scaled, webrtc_VideoRotation_0,
          webrtc_TimestampAligner_TranslateTimestamp(
              webrtc_TimestampAligner_unique_get(cap->timestamp_aligner),
              timestamp_us, webrtc_TimeMillis() * 1000));
      webrtc_I420Buffer_Release(webrtc_I420Buffer_refcounted_get(scaled));
    } else {
      // 翻訳したタイムスタンプに差し替え
      int64_t translated = webrtc_TimestampAligner_TranslateTimestamp(
          webrtc_TimestampAligner_unique_get(cap->timestamp_aligner),
          timestamp_us, webrtc_TimeMillis() * 1000);
      // VideoFrame は再生成
      struct webrtc_I420Buffer_refcounted* buf =
          webrtc_VideoFrame_video_frame_buffer(
              webrtc_VideoFrame_unique_get(frame));
      webrtc_VideoFrame_unique_delete(frame);
      frame = webrtc_VideoFrame_Create(buf, webrtc_VideoRotation_0, translated);
      webrtc_I420Buffer_Release(webrtc_I420Buffer_refcounted_get(buf));
    }

    webrtc_AdaptedVideoTrackSource_OnFrame(
        webrtc_AdaptedVideoTrackSource_refcounted_get(cap->source),
        webrtc_VideoFrame_unique_get(frame));
    webrtc_VideoFrame_unique_delete(frame);

    int sleep_ms = (1000 / cap->config.fps) - 2;
    if (sleep_ms < 1) {
      sleep_ms = 1;
    }
    webrtc_Thread_SleepMs(sleep_ms);
  }

  return NULL;
}

struct PeerConnectionFactory* PeerConnectionFactory_Create() {
  webrtc_InitializeSSL();

  struct PeerConnectionFactory* p = (struct PeerConnectionFactory*)calloc(
      1, sizeof(struct PeerConnectionFactory));
  p->ref = webrtc_RefCountInterface_Create(PeerConnectionFactory_delete, p);

  p->network_thread = webrtc_Thread_CreateWithSocketServer();
  webrtc_Thread_Start(webrtc_Thread_unique_get(p->network_thread));
  p->worker_thread = webrtc_Thread_Create();
  webrtc_Thread_Start(webrtc_Thread_unique_get(p->worker_thread));
  p->signaling_thread = webrtc_Thread_Create();
  webrtc_Thread_Start(webrtc_Thread_unique_get(p->signaling_thread));

  struct webrtc_PeerConnectionFactoryDependencies* dependencies =
      webrtc_PeerConnectionFactoryDependencies_new();
  struct webrtc_Environment* env = webrtc_CreateEnvironment();
  webrtc_PeerConnectionFactoryDependencies_set_network_thread(
      dependencies, webrtc_Thread_unique_get(p->network_thread));
  webrtc_PeerConnectionFactoryDependencies_set_worker_thread(
      dependencies, webrtc_Thread_unique_get(p->worker_thread));
  webrtc_PeerConnectionFactoryDependencies_set_signaling_thread(
      dependencies, webrtc_Thread_unique_get(p->signaling_thread));
  webrtc_PeerConnectionFactoryDependencies_set_event_log_factory(
      dependencies, webrtc_RtcEventLogFactory_Create());
  struct webrtc_AudioDeviceModule_refcounted* adm =
      (struct webrtc_AudioDeviceModule_refcounted*)webrtc_Thread_BlockingCall_r(
          webrtc_Thread_unique_get(p->worker_thread), _BlockingCall_create_adm,
          env);
  webrtc_PeerConnectionFactoryDependencies_set_adm(dependencies, adm);
  webrtc_AudioDeviceModule_Release(
      webrtc_AudioDeviceModule_refcounted_get(adm));

  struct webrtc_AudioEncoderFactory_refcounted* audio_encoder_factory =
      webrtc_CreateBuiltinAudioEncoderFactory();
  webrtc_PeerConnectionFactoryDependencies_set_audio_encoder_factory(
      dependencies, audio_encoder_factory);
  webrtc_AudioEncoderFactory_Release(
      webrtc_AudioEncoderFactory_refcounted_get(audio_encoder_factory));

  struct webrtc_AudioDecoderFactory_refcounted* audio_decoder_factory =
      webrtc_CreateBuiltinAudioDecoderFactory();
  webrtc_PeerConnectionFactoryDependencies_set_audio_decoder_factory(
      dependencies, audio_decoder_factory);
  webrtc_AudioDecoderFactory_Release(
      webrtc_AudioDecoderFactory_refcounted_get(audio_decoder_factory));

  struct webrtc_VideoEncoderFactory_unique* video_encoder_factory =
      webrtc_CreateBuiltinVideoEncoderFactory();
  webrtc_PeerConnectionFactoryDependencies_set_video_encoder_factory(
      dependencies, video_encoder_factory);
  struct webrtc_VideoDecoderFactory_unique* video_decoder_factory =
      webrtc_CreateBuiltinVideoDecoderFactory();
  webrtc_PeerConnectionFactoryDependencies_set_video_decoder_factory(
      dependencies, video_decoder_factory);

  webrtc_PeerConnectionFactoryDependencies_set_audio_processing_builder(
      dependencies, webrtc_BuiltinAudioProcessingBuilder_Create());

  webrtc_EnableMedia(dependencies);

  p->factory = webrtc_CreateModularPeerConnectionFactory(dependencies);

  webrtc_PeerConnectionFactoryDependencies_delete(dependencies);
  webrtc_Environment_delete(env);

  if (p->factory == NULL) {
    webrtc_RefCountInterface_Release(p->ref);
    return NULL;
  }

  struct webrtc_PeerConnectionFactoryInterface_Options* factory_options =
      webrtc_PeerConnectionFactoryInterface_Options_new();
  webrtc_PeerConnectionFactoryInterface_Options_set_disable_encryption(
      factory_options, 0);
  webrtc_PeerConnectionFactoryInterface_Options_set_ssl_max_version(
      factory_options, webrtc_SSL_PROTOCOL_DTLS_12);
  webrtc_PeerConnectionFactoryInterface_SetOptions(
      webrtc_PeerConnectionFactoryInterface_refcounted_get(p->factory),
      factory_options);
  webrtc_PeerConnectionFactoryInterface_Options_delete(factory_options);

  return p;
}

struct SignalingWhipConfig {
  struct webrtc_PeerConnectionFactoryInterface_refcounted* pc_factory;
  struct webrtc_VideoTrackSourceInterface_refcounted* video_source;
  struct webrtc_RtpEncodingParameters_vector* send_encodings;
  char* signaling_url;
  char* channel_id;
};
struct SignalingWhipConfig* SignalingWhipConfig_create() {
  return (struct SignalingWhipConfig*)calloc(
      1, sizeof(struct SignalingWhipConfig));
}
struct SignalingWhipConfig* SignalingWhipConfig_copy(
    struct SignalingWhipConfig* src) {
  struct SignalingWhipConfig* dst = (struct SignalingWhipConfig*)calloc(
      1, sizeof(struct SignalingWhipConfig));
  if (src->pc_factory != NULL) {
    dst->pc_factory = src->pc_factory;
    webrtc_PeerConnectionFactoryInterface_AddRef(
        webrtc_PeerConnectionFactoryInterface_refcounted_get(dst->pc_factory));
  }
  if (src->video_source != NULL) {
    dst->video_source = src->video_source;
    webrtc_VideoTrackSourceInterface_AddRef(
        webrtc_VideoTrackSourceInterface_refcounted_get(dst->video_source));
  }
  if (src->send_encodings != NULL) {
    dst->send_encodings =
        webrtc_RtpEncodingParameters_vector_clone(src->send_encodings);
  }
  if (src->signaling_url != NULL) {
    dst->signaling_url = strdup(src->signaling_url);
  }
  if (src->channel_id != NULL) {
    dst->channel_id = strdup(src->channel_id);
  }
  return dst;
}
void SignalingWhipConfig_delete(struct SignalingWhipConfig* config) {
  if (config->pc_factory != NULL) {
    webrtc_PeerConnectionFactoryInterface_Release(
        webrtc_PeerConnectionFactoryInterface_refcounted_get(
            config->pc_factory));
  }
  if (config->video_source != NULL) {
    webrtc_VideoTrackSourceInterface_Release(
        webrtc_VideoTrackSourceInterface_refcounted_get(config->video_source));
  }
  if (config->send_encodings != NULL) {
    webrtc_RtpEncodingParameters_vector_delete(config->send_encodings);
  }
  if (config->signaling_url != NULL) {
    free(config->signaling_url);
  }
  if (config->channel_id != NULL) {
    free(config->channel_id);
  }
  free(config);
}
void SignalingWhipConfig_set_pc_factory(
    struct SignalingWhipConfig* config,
    struct webrtc_PeerConnectionFactoryInterface_refcounted* pc_factory) {
  if (config->pc_factory != NULL) {
    webrtc_PeerConnectionFactoryInterface_Release(
        webrtc_PeerConnectionFactoryInterface_refcounted_get(
            config->pc_factory));
  }
  config->pc_factory = pc_factory;
  if (pc_factory != NULL) {
    webrtc_PeerConnectionFactoryInterface_AddRef(
        webrtc_PeerConnectionFactoryInterface_refcounted_get(pc_factory));
  }
}
void SignalingWhipConfig_set_video_source(
    struct SignalingWhipConfig* config,
    struct webrtc_VideoTrackSourceInterface_refcounted* video_source) {
  if (config->video_source != NULL) {
    webrtc_VideoTrackSourceInterface_Release(
        webrtc_VideoTrackSourceInterface_refcounted_get(config->video_source));
  }
  config->video_source = video_source;
  if (config->video_source != NULL) {
    webrtc_VideoTrackSourceInterface_AddRef(
        webrtc_VideoTrackSourceInterface_refcounted_get(config->video_source));
  }
}
void SignalingWhipConfig_set_send_encodings(
    struct SignalingWhipConfig* config,
    struct webrtc_RtpEncodingParameters_vector* send_encodings) {
  if (config->send_encodings != NULL) {
    webrtc_RtpEncodingParameters_vector_delete(config->send_encodings);
    config->send_encodings = NULL;
  }
  if (send_encodings != NULL) {
    config->send_encodings =
        webrtc_RtpEncodingParameters_vector_clone(send_encodings);
  }
}
void SignalingWhipConfig_set_signaling_url(struct SignalingWhipConfig* config,
                                           const char* signaling_url) {
  if (config->signaling_url != NULL) {
    free(config->signaling_url);
  }
  config->signaling_url = signaling_url ? strdup(signaling_url) : NULL;
}
void SignalingWhipConfig_set_channel_id(struct SignalingWhipConfig* config,
                                        const char* channel_id) {
  if (config->channel_id != NULL) {
    free(config->channel_id);
  }
  config->channel_id = channel_id ? strdup(channel_id) : NULL;
}

struct SignalingWhip {
  struct webrtc_RefCountInterface_ref* ref;
  struct webrtc_PeerConnectionObserver_cbs observer_cbs;
  struct webrtc_PeerConnectionObserver* observer;
  struct SignalingWhipConfig* config;
  struct webrtc_PeerConnectionInterface_refcounted* pc;
  struct webrtc_VideoTrackSourceInterface_refcounted* video_source;
  pthread_mutex_t mutex;
  pthread_cond_t cond;
  struct webrtc_SetLocalDescriptionObserverInterface_cbs loc_cbs;
  struct webrtc_SetRemoteDescriptionObserverInterface_cbs rem_cbs;
  struct webrtc_CreateSessionDescriptionObserver_cbs offer_cbs;
  enum {
    SIGNALLING_WHIP_STATE_INIT = 0,
    SIGNALLING_WHIP_STATE_CONNECTING,
    SIGNALLING_WHIP_STATE_CONNECTED,
    SIGNALLING_WHIP_STATE_CLOSED,
  } state;
};

struct whip_send_request_ctx {
  struct SignalingWhip* self;
  struct webrtc_SessionDescriptionInterface_unique* desc;
};

void SignalingWhip_delete(void* user_data) {
  struct SignalingWhip* p = (struct SignalingWhip*)user_data;
  SignalingWhipConfig_delete(p->config);
  if (p->pc != NULL) {
    webrtc_PeerConnectionInterface_Release(
        webrtc_PeerConnectionInterface_refcounted_get(p->pc));
  }
  if (p->observer != NULL) {
    webrtc_PeerConnectionObserver_delete(p->observer);
  }
  pthread_mutex_destroy(&p->mutex);
  pthread_cond_destroy(&p->cond);
  free(p);
}

void SignalingWhip_OnConnectionChange(
    webrtc_PeerConnectionInterface_PeerConnectionState new_state,
    void* user_data) {
  struct SignalingWhip* self = (struct SignalingWhip*)user_data;
  RTC_LOG_INFO("SignalingWhip_OnConnectionChange: new_state=%d", new_state);
  pthread_mutex_lock(&self->mutex);
  if (new_state ==
      webrtc_PeerConnectionInterface_PeerConnectionState_kConnected) {
    self->state = SIGNALLING_WHIP_STATE_CONNECTED;
  } else if (new_state ==
                 webrtc_PeerConnectionInterface_PeerConnectionState_kFailed ||
             new_state ==
                 webrtc_PeerConnectionInterface_PeerConnectionState_kClosed) {
    self->state = SIGNALLING_WHIP_STATE_CLOSED;
  }
  pthread_cond_broadcast(&self->cond);
  pthread_mutex_unlock(&self->mutex);
}

static void SignalingWhip_SetState(struct SignalingWhip* self, int state) {
  pthread_mutex_lock(&self->mutex);
  self->state = state;
  pthread_cond_broadcast(&self->cond);
  pthread_mutex_unlock(&self->mutex);
}

struct whip_set_local_ctx {
  struct SignalingWhip* self;
  char* answer_body;
};

// forward declarations
static void whip_SendRequest(const char* host,
                             const char* port,
                             const char* req,
                             void (*on_response)(char* resp, void* user_data),
                             void* user_data);

static char* whip_find_header_value(const char* headers, const char* key) {
  size_t key_len = strlen(key);
  const char* p = headers;
  while (p && *p) {
    const char* line_end = strstr(p, "\r\n");
    if (line_end == NULL) {
      line_end = p + strlen(p);
    }
    const char* colon = strchr(p, ':');
    if (colon != NULL && colon < line_end) {
      if ((size_t)(colon - p) == key_len && strncasecmp(p, key, key_len) == 0) {
        const char* value_start = colon + 1;
        while (value_start < line_end &&
               (*value_start == ' ' || *value_start == '\t')) {
          value_start++;
        }
        size_t value_len = (size_t)(line_end - value_start);
        char* value = (char*)malloc(value_len + 1);
        if (value != NULL) {
          memcpy(value, value_start, value_len);
          value[value_len] = '\0';
        }
        return value;
      }
    }
    if (*line_end == '\0') {
      break;
    }
    p = line_end + 2;
  }
  return NULL;
}

static void whip_OnSetRemoteDescriptionComplete(
    struct webrtc_RTCError_unique* error,
    void* user_data) {
  struct SignalingWhip* self = (struct SignalingWhip*)user_data;
  if (error != NULL && !webrtc_RTCError_ok(webrtc_RTCError_unique_get(error))) {
    const char* message = NULL;
    size_t message_len = 0;
    webrtc_RTCError_message(webrtc_RTCError_unique_get(error), &message,
                            &message_len);
    RTC_LOG_ERROR("Failed to SetRemoteDescription: %.*s", (int)message_len,
                  message ? message : "");
    SignalingWhip_SetState(self, SIGNALLING_WHIP_STATE_CLOSED);
    webrtc_RTCError_unique_delete(error);
    return;
  }
  if (error != NULL) {
    webrtc_RTCError_unique_delete(error);
  }
  SignalingWhip_SetState(self, SIGNALLING_WHIP_STATE_CONNECTED);
}

static void whip_OnSetLocalDescriptionComplete(
    struct webrtc_RTCError_unique* error,
    void* user_data) {
  struct whip_set_local_ctx* ctx = (struct whip_set_local_ctx*)user_data;
  struct SignalingWhip* self = ctx->self;
  if (error != NULL && !webrtc_RTCError_ok(webrtc_RTCError_unique_get(error))) {
    const char* message = NULL;
    size_t message_len = 0;
    webrtc_RTCError_message(webrtc_RTCError_unique_get(error), &message,
                            &message_len);
    RTC_LOG_ERROR("Failed to SetLocalDescription: %.*s", (int)message_len,
                  message ? message : "");
    SignalingWhip_SetState(self, SIGNALLING_WHIP_STATE_CLOSED);
    webrtc_RTCError_unique_delete(error);
    free(ctx->answer_body);
    free(ctx);
    return;
  }
  if (error != NULL) {
    webrtc_RTCError_unique_delete(error);
  }

  struct webrtc_SessionDescriptionInterface_unique* answer =
      webrtc_CreateSessionDescription(webrtc_SdpType_kAnswer, ctx->answer_body,
                                      strlen(ctx->answer_body));
  if (answer == NULL) {
    RTC_LOG_ERROR("Failed to create answer description");
    SignalingWhip_SetState(self, SIGNALLING_WHIP_STATE_CLOSED);
    free(ctx->answer_body);
    free(ctx);
    return;
  }

  struct webrtc_SetRemoteDescriptionObserverInterface_refcounted* rem_obs =
      webrtc_SetRemoteDescriptionObserverInterface_make_ref_counted(
          &self->rem_cbs, self);
  webrtc_PeerConnectionInterface_SetRemoteDescription(
      webrtc_PeerConnectionInterface_refcounted_get(self->pc), answer, rem_obs);
  webrtc_SetRemoteDescriptionObserverInterface_Release(
      webrtc_SetRemoteDescriptionObserverInterface_refcounted_get(rem_obs));

  free(ctx->answer_body);
  free(ctx);
}

static void whip_OnCreateOfferFailure(struct webrtc_RTCError_unique* error,
                                      void* user_data) {
  struct SignalingWhip* self = (struct SignalingWhip*)user_data;
  const char* msg = "unknown";
  size_t msg_len = 0;
  if (error) {
    webrtc_RTCError_message(webrtc_RTCError_unique_get(error), &msg, &msg_len);
  }
  if (msg_len > 0) {
    RTC_LOG_ERROR("Failed to CreateOffer: %.*s", (int)msg_len, msg ? msg : "");
  } else {
    RTC_LOG_ERROR("Failed to CreateOffer: %s", msg ? msg : "unknown");
  }
  SignalingWhip_SetState(self, SIGNALLING_WHIP_STATE_CLOSED);
  if (error != NULL) {
    webrtc_RTCError_unique_delete(error);
  }
}

static void whip_OnCreateOfferSuccess(
    struct webrtc_SessionDescriptionInterface_unique* desc,
    void* user_data) {
  struct SignalingWhip* self = (struct SignalingWhip*)user_data;
  struct std_string_unique* offer_sdp = NULL;
  if (!webrtc_SessionDescriptionInterface_ToString(
          webrtc_SessionDescriptionInterface_unique_get(desc), &offer_sdp)) {
    RTC_LOG_ERROR("Failed to get SDP");
    SignalingWhip_SetState(self, SIGNALLING_WHIP_STATE_CLOSED);
    webrtc_SessionDescriptionInterface_unique_delete(desc);
    return;
  }
  const char* offer_sdp_str =
      std_string_c_str(std_string_unique_get(offer_sdp));

  struct URLParts parts = {0};
  if (!URLParts_Parse(self->config->signaling_url, &parts)) {
    RTC_LOG_ERROR("Failed to parse url: %s", self->config->signaling_url);
    std_string_unique_delete(offer_sdp);
    SignalingWhip_SetState(self, SIGNALLING_WHIP_STATE_CLOSED);
    webrtc_SessionDescriptionInterface_unique_delete(desc);
    return;
  }

  size_t target_len = strlen(parts.path_query_fragment) + 1 +
                      strlen(self->config->channel_id) +
                      strlen("?video_bit_rate=6000") + 1;
  char* target = (char*)malloc(target_len);
  if (target == NULL) {
    URLParts_clear(&parts);
    std_string_unique_delete(offer_sdp);
    SignalingWhip_SetState(self, SIGNALLING_WHIP_STATE_CLOSED);
    webrtc_SessionDescriptionInterface_unique_delete(desc);
    return;
  }
  snprintf(target, target_len, "%s/%s?video_bit_rate=6000",
           parts.path_query_fragment, self->config->channel_id);

  char content_length[32];
  snprintf(content_length, sizeof(content_length), "%zu",
           strlen(offer_sdp_str));

  size_t req_len =
      strlen("POST ") + strlen(target) + strlen(" HTTP/1.1\r\nHost: ") +
      strlen(parts.host) + 1 + strlen(URLParts_GetPort(&parts)) +
      strlen("\r\n") +
      strlen("Content-Type: application/sdp\r\nContent-Length: ") +
      strlen(content_length) +
      strlen("\r\nUser-Agent: Whip-Client\r\nConnection: close\r\n\r\n") +
      strlen(offer_sdp_str) + 1;
  char* req = (char*)malloc(req_len);
  if (req == NULL) {
    free(target);
    URLParts_clear(&parts);
    std_string_unique_delete(offer_sdp);
    SignalingWhip_SetState(self, SIGNALLING_WHIP_STATE_CLOSED);
    webrtc_SessionDescriptionInterface_unique_delete(desc);
    return;
  }
  snprintf(req, req_len,
           "POST %s HTTP/1.1\r\nHost: %s:%s\r\n"
           "Content-Type: application/sdp\r\nContent-Length: %s\r\n"
           "User-Agent: Whip-Client\r\nConnection: close\r\n\r\n%s",
           target, parts.host, URLParts_GetPort(&parts), content_length,
           offer_sdp_str);

  struct whip_send_request_ctx* send_ctx =
      (struct whip_send_request_ctx*)calloc(
          1, sizeof(struct whip_send_request_ctx));
  if (send_ctx == NULL) {
    free(req);
    free(target);
    URLParts_clear(&parts);
    std_string_unique_delete(offer_sdp);
    SignalingWhip_SetState(self, SIGNALLING_WHIP_STATE_CLOSED);
    webrtc_SessionDescriptionInterface_unique_delete(desc);
    return;
  }
  send_ctx->self = self;
  send_ctx->desc = desc;

  whip_SendRequest(parts.host, URLParts_GetPort(&parts), req,
                   whip_OnSendRequestResponse, send_ctx);

  free(req);
  free(target);
  URLParts_clear(&parts);
  std_string_unique_delete(offer_sdp);
}

static void whip_SendRequest(const char* host,
                             const char* port,
                             const char* req,
                             void (*on_response)(char* resp, void* user_data),
                             void* user_data) {
  char* response = NULL;

  struct addrinfo hints;
  memset(&hints, 0, sizeof(hints));
  hints.ai_family = AF_UNSPEC;
  hints.ai_socktype = SOCK_STREAM;
  hints.ai_protocol = IPPROTO_TCP;

  struct addrinfo* result = NULL;
  int gai_err = getaddrinfo(host, port, &hints, &result);
  if (gai_err != 0) {
    RTC_LOG_ERROR("getaddrinfo failed: %s", gai_strerror(gai_err));
    on_response(NULL, user_data);
    return;
  }

  int sock = -1;
  for (struct addrinfo* rp = result; rp != NULL; rp = rp->ai_next) {
    sock = (int)socket(rp->ai_family, rp->ai_socktype, rp->ai_protocol);
    if (sock < 0) {
      continue;
    }
    if (connect(sock, rp->ai_addr, (socklen_t)rp->ai_addrlen) == 0) {
      break;
    }
    close(sock);
    sock = -1;
  }

  freeaddrinfo(result);

  if (sock < 0) {
    RTC_LOG_ERROR("Failed to connect to %s:%s", host, port);
    on_response(NULL, user_data);
    return;
  }

  SSL_CTX* ctx = SSL_CTX_new(TLS_client_method());
  if (ctx == NULL) {
    RTC_LOG_ERROR("SSL_CTX_new failed");
    close(sock);
    on_response(NULL, user_data);
    return;
  }
  SSL_CTX_set_min_proto_version(ctx, TLS1_2_VERSION);
  SSL_CTX_set_max_proto_version(ctx, TLS1_3_VERSION);
  SSL_CTX_set_options(ctx, SSL_OP_ALL | SSL_OP_NO_SSLv2 | SSL_OP_NO_SSLv3 |
                               SSL_OP_NO_TLSv1 | SSL_OP_NO_TLSv1_1 |
                               SSL_OP_SINGLE_DH_USE);

  SSL* ssl = SSL_new(ctx);
  if (ssl == NULL) {
    RTC_LOG_ERROR("SSL_new failed");
    SSL_CTX_free(ctx);
    close(sock);
    on_response(NULL, user_data);
    return;
  }

  if (!SSL_set_tlsext_host_name(ssl, host)) {
    RTC_LOG_ERROR("Failed to set SNI");
    SSL_free(ssl);
    SSL_CTX_free(ctx);
    close(sock);
    on_response(NULL, user_data);
    return;
  }

  SSL_set_fd(ssl, sock);
  if (SSL_connect(ssl) != 1) {
    RTC_LOG_ERROR("SSL_connect failed");
    SSL_free(ssl);
    SSL_CTX_free(ctx);
    close(sock);
    on_response(NULL, user_data);
    return;
  }

  if (SSL_write(ssl, req, (int)strlen(req)) <= 0) {
    RTC_LOG_ERROR("SSL_write failed");
    SSL_free(ssl);
    SSL_CTX_free(ctx);
    close(sock);
    on_response(NULL, user_data);
    return;
  }

  size_t resp_cap = 4096;
  size_t resp_len = 0;
  char* resp = (char*)malloc(resp_cap);
  if (resp == NULL) {
    SSL_free(ssl);
    SSL_CTX_free(ctx);
    close(sock);
    on_response(NULL, user_data);
    return;
  }

  char buf[4096];
  for (;;) {
    int n = SSL_read(ssl, buf, sizeof(buf));
    if (n <= 0) {
      break;
    }
    if (resp_len + (size_t)n + 1 > resp_cap) {
      resp_cap = (resp_len + (size_t)n + 1) * 2;
      char* new_resp = (char*)realloc(resp, resp_cap);
      if (new_resp == NULL) {
        free(resp);
        SSL_free(ssl);
        SSL_CTX_free(ctx);
        close(sock);
        on_response(NULL, user_data);
        return;
      }
      resp = new_resp;
    }
    memcpy(resp + resp_len, buf, (size_t)n);
    resp_len += (size_t)n;
  }
  resp[resp_len] = '\0';

  SSL_free(ssl);
  SSL_CTX_free(ctx);
  close(sock);

  response = resp;
  on_response(response, user_data);
}

static void whip_OnSendRequestResponse(char* resp, void* user_data) {
  struct whip_send_request_ctx* ctx = (struct whip_send_request_ctx*)user_data;
  struct SignalingWhip* self = ctx->self;
  struct webrtc_SessionDescriptionInterface_unique* desc = ctx->desc;

  if (resp == NULL) {
    RTC_LOG_ERROR("Failed to send request");
    SignalingWhip_SetState(self, SIGNALLING_WHIP_STATE_CLOSED);
    webrtc_SessionDescriptionInterface_unique_delete(desc);
    free(ctx);
    return;
  }

  char* header_end = strstr(resp, "\r\n\r\n");
  if (header_end == NULL) {
    RTC_LOG_ERROR("Invalid response");
    free(resp);
    SignalingWhip_SetState(self, SIGNALLING_WHIP_STATE_CLOSED);
    webrtc_SessionDescriptionInterface_unique_delete(desc);
    free(ctx);
    return;
  }

  size_t header_len = (size_t)(header_end - resp);
  char* headers = (char*)malloc(header_len + 1);
  if (headers == NULL) {
    free(resp);
    SignalingWhip_SetState(self, SIGNALLING_WHIP_STATE_CLOSED);
    webrtc_SessionDescriptionInterface_unique_delete(desc);
    free(ctx);
    return;
  }
  memcpy(headers, resp, header_len);
  headers[header_len] = '\0';
  char* body = strdup(header_end + 4);

  char* link_header = whip_find_header_value(headers, "link");
  free(headers);
  if (link_header == NULL) {
    RTC_LOG_ERROR("No Link header");
    free(resp);
    free(body);
    SignalingWhip_SetState(self, SIGNALLING_WHIP_STATE_CLOSED);
    webrtc_SessionDescriptionInterface_unique_delete(desc);
    free(ctx);
    return;
  }

  struct webrtc_PeerConnectionInterface_IceServer* server =
      webrtc_PeerConnectionInterface_IceServer_new();
  struct std_string_vector* urls =
      webrtc_PeerConnectionInterface_IceServer_get_urls(server);

  char* saveptr = NULL;
  char* token = strtok_r(link_header, ",", &saveptr);
  while (token != NULL) {
    char* url_start = strchr(token, '<');
    char* url_end = strchr(token, '>');
    if (url_start != NULL && url_end != NULL && url_end > url_start + 1) {
      char* url_cstr =
          strndup(url_start + 1, (size_t)(url_end - url_start - 1));
      if (url_cstr != NULL) {
        struct std_string_unique* url = std_string_new_from_cstr(url_cstr);
        if (url != NULL) {
          std_string_vector_push_back(urls, std_string_unique_get(url));
          std_string_unique_delete(url);
        }
        free(url_cstr);
      }
    }
    char* username_pos = strstr(token, "username=\"");
    if (username_pos != NULL) {
      username_pos += strlen("username=\"");
      char* username_end = strchr(username_pos, '"');
      if (username_end != NULL) {
        char* username =
            strndup(username_pos, (size_t)(username_end - username_pos));
        webrtc_PeerConnectionInterface_IceServer_set_username(server, username,
                                                              strlen(username));
        free(username);
      }
    }
    char* credential_pos = strstr(token, "credential=\"");
    if (credential_pos != NULL) {
      credential_pos += strlen("credential=\"");
      char* credential_end = strchr(credential_pos, '"');
      if (credential_end != NULL) {
        char* credential =
            strndup(credential_pos, (size_t)(credential_end - credential_pos));
        webrtc_PeerConnectionInterface_IceServer_set_password(
            server, credential, strlen(credential));
        free(credential);
      }
    }

    token = strtok_r(NULL, ",", &saveptr);
  }

  struct webrtc_PeerConnectionInterface_RTCConfiguration* rtc_config =
      webrtc_PeerConnectionInterface_RTCConfiguration_new();
  struct webrtc_PeerConnectionInterface_IceServer_vector* servers =
      webrtc_PeerConnectionInterface_RTCConfiguration_get_servers(rtc_config);
  webrtc_PeerConnectionInterface_IceServer_vector_push_back(servers, server);
  webrtc_PeerConnectionInterface_IceServer_delete(server);
  webrtc_PeerConnectionInterface_RTCConfiguration_set_type(
      rtc_config, webrtc_PeerConnectionInterface_IceTransportsType_kRelay);
  struct webrtc_RTCError_unique* set_config_error = NULL;
  webrtc_PeerConnectionInterface_SetConfiguration(
      webrtc_PeerConnectionInterface_refcounted_get(self->pc), rtc_config,
      &set_config_error);
  webrtc_PeerConnectionInterface_RTCConfiguration_delete(rtc_config);
  free(link_header);

  if (set_config_error != NULL) {
    const char* message = NULL;
    size_t message_len = 0;
    webrtc_RTCError_message(webrtc_RTCError_unique_get(set_config_error),
                            &message, &message_len);
    RTC_LOG_ERROR("Failed to SetConfiguration: %.*s", (int)message_len,
                  message ? message : "");
    webrtc_RTCError_unique_delete(set_config_error);
    free(resp);
    free(body);
    SignalingWhip_SetState(self, SIGNALLING_WHIP_STATE_CLOSED);
    webrtc_SessionDescriptionInterface_unique_delete(desc);
    free(ctx);
    return;
  }

  struct whip_set_local_ctx* loc_ctx =
      (struct whip_set_local_ctx*)calloc(1, sizeof(struct whip_set_local_ctx));
  if (loc_ctx == NULL) {
    free(resp);
    free(body);
    SignalingWhip_SetState(self, SIGNALLING_WHIP_STATE_CLOSED);
    webrtc_SessionDescriptionInterface_unique_delete(desc);
    free(ctx);
    return;
  }
  loc_ctx->self = self;
  loc_ctx->answer_body = body;

  struct webrtc_SetLocalDescriptionObserverInterface_refcounted* loc_obs =
      webrtc_SetLocalDescriptionObserverInterface_make_ref_counted(
          &self->loc_cbs, loc_ctx);

  webrtc_PeerConnectionInterface_SetLocalDescription(
      webrtc_PeerConnectionInterface_refcounted_get(self->pc), desc, loc_obs);
  webrtc_SetLocalDescriptionObserverInterface_Release(
      webrtc_SetLocalDescriptionObserverInterface_refcounted_get(loc_obs));
  free(resp);
  free(ctx);
}

struct SignalingWhip* SignalingWhip_Create(struct SignalingWhipConfig* config) {
  struct SignalingWhip* p =
      (struct SignalingWhip*)calloc(1, sizeof(struct SignalingWhip));
  p->ref = webrtc_RefCountInterface_Create(SignalingWhip_delete, p);
  p->observer_cbs.OnConnectionChange = SignalingWhip_OnConnectionChange;
  p->loc_cbs.OnSetLocalDescriptionComplete = whip_OnSetLocalDescriptionComplete;
  p->rem_cbs.OnSetRemoteDescriptionComplete =
      whip_OnSetRemoteDescriptionComplete;
  p->offer_cbs.OnSuccess = whip_OnCreateOfferSuccess;
  p->offer_cbs.OnFailure = whip_OnCreateOfferFailure;
  p->config = SignalingWhipConfig_copy(config);
  pthread_mutex_init(&p->mutex, NULL);
  pthread_cond_init(&p->cond, NULL);
  p->state = SIGNALLING_WHIP_STATE_INIT;

  return p;
}

void SignalingWhip_Connect(struct SignalingWhip* self) {
  struct webrtc_PeerConnectionInterface_RTCConfiguration* rtc_config =
      webrtc_PeerConnectionInterface_RTCConfiguration_new();
  self->observer = webrtc_PeerConnectionObserver_new(&self->observer_cbs, self);
  struct webrtc_PeerConnectionDependencies* pc_dependencies =
      webrtc_PeerConnectionDependencies_new(self->observer);
  struct webrtc_PeerConnectionInterface_refcounted* pc;
  struct webrtc_RTCError_unique* rtc_error;
  webrtc_PeerConnectionFactoryInterface_CreatePeerConnectionOrError(
      webrtc_PeerConnectionFactoryInterface_refcounted_get(
          self->config->pc_factory),
      rtc_config, pc_dependencies, &pc, &rtc_error);

  webrtc_PeerConnectionInterface_RTCConfiguration_delete(rtc_config);
  webrtc_PeerConnectionDependencies_delete(pc_dependencies);
  if (rtc_error != NULL) {
    RTC_LOG_ERROR("Failed to create PeerConnection: error=%p", rtc_error);
    webrtc_RTCError_unique_delete(rtc_error);
    return;
  }

  self->pc = pc;
  pc = NULL;

  {
    struct webrtc_RtpTransceiverInit* init = webrtc_RtpTransceiverInit_new();
    webrtc_RtpTransceiverInit_set_direction(
        init, webrtc_RtpTransceiverDirection_kSendOnly);
    struct webrtc_RtpTransceiverInterface_refcounted* transceiver = NULL;
    rtc_error = NULL;
    webrtc_PeerConnectionInterface_AddTransceiver(
        webrtc_PeerConnectionInterface_refcounted_get(self->pc),
        webrtc_MediaType_AUDIO, init, &transceiver, &rtc_error);
    webrtc_RtpTransceiverInit_delete(init);
    if (rtc_error != NULL) {
      RTC_LOG_ERROR("Failed to AddTransceiver(audio): error=%p", rtc_error);
      webrtc_RTCError_unique_delete(rtc_error);
      return;
    }

    struct webrtc_RtpCapabilities* caps =
        webrtc_PeerConnectionFactoryInterface_GetRtpSenderCapabilities(
            webrtc_PeerConnectionFactoryInterface_refcounted_get(
                self->config->pc_factory),
            webrtc_MediaType_AUDIO);
    struct webrtc_RtpCodecCapability_vector* src_codecs =
        webrtc_RtpCapabilities_get_codecs(caps);
    int codecs_size = webrtc_RtpCodecCapability_vector_size(src_codecs);
    struct webrtc_RtpCodecCapability_vector* codecs =
        webrtc_RtpCodecCapability_vector_new(0);
    for (int i = 0; i < codecs_size; ++i) {
      struct std_string* name = webrtc_RtpCodecCapability_get_name(
          webrtc_RtpCodecCapability_vector_get(src_codecs, i));
      const char* c_name = std_string_c_str(name);
      if (strcmp(c_name, "OPUS") == 0) {
        webrtc_RtpCodecCapability_vector_push_back(
            codecs, webrtc_RtpCodecCapability_vector_get(src_codecs, i));
        break;
      }
    }
    webrtc_RtpCapabilities_delete(caps);

    rtc_error = webrtc_RtpTransceiverInterface_SetCodecPreferences(
        webrtc_RtpTransceiverInterface_refcounted_get(transceiver), codecs);
    webrtc_RtpCodecCapability_vector_delete(codecs);
    webrtc_RtpTransceiverInterface_Release(
        webrtc_RtpTransceiverInterface_refcounted_get(transceiver));
    if (rtc_error != NULL) {
      RTC_LOG_ERROR("Failed to SetCodecPreferences(audio): error=%p",
                    rtc_error);
      webrtc_RTCError_unique_delete(rtc_error);
    }
  }

  {
    struct webrtc_RtpTransceiverInit* video_init =
        webrtc_RtpTransceiverInit_new();
    if (self->config->video_source != NULL) {
      struct std_string_unique* video_track_id = webrtc_CreateRandomString(16);
      struct webrtc_VideoTrackInterface_refcounted* video_track = NULL;
      struct std_string* video_track_id_raw =
          std_string_unique_get(video_track_id);
      webrtc_PeerConnectionFactoryInterface_CreateVideoTrack(
          webrtc_PeerConnectionFactoryInterface_refcounted_get(
              self->config->pc_factory),
          self->config->video_source, std_string_c_str(video_track_id_raw),
          std_string_size(video_track_id_raw), &video_track);
      std_string_unique_delete(video_track_id);
      if (video_track == NULL) {
        RTC_LOG_ERROR("Failed to create VideoTrack");
        webrtc_RtpTransceiverInit_delete(video_init);
        return;
      }

      struct std_string_unique* stream_id = webrtc_CreateRandomString(16);
      webrtc_RtpTransceiverInit_set_direction(
          video_init, webrtc_RtpTransceiverDirection_kSendOnly);
      struct std_string_vector* stream_ids =
          webrtc_RtpTransceiverInit_get_stream_ids(video_init);
      std_string_vector_push_back(stream_ids, std_string_unique_get(stream_id));
      std_string_unique_delete(stream_id);
      if (self->config->send_encodings != NULL) {
        webrtc_RtpTransceiverInit_set_send_encodings(
            video_init, self->config->send_encodings);
      }

      struct webrtc_RtpTransceiverInterface_refcounted* transceiver = NULL;
      rtc_error = NULL;
      webrtc_PeerConnectionInterface_AddTransceiverWithTrack(
          webrtc_PeerConnectionInterface_refcounted_get(self->pc), video_track,
          video_init, &transceiver, &rtc_error);
      webrtc_VideoTrackInterface_Release(
          webrtc_VideoTrackInterface_refcounted_get(video_track));
      video_track = NULL;
      if (rtc_error != NULL) {
        RTC_LOG_ERROR("Failed to AddTransceiver(video): error=%p", rtc_error);
        webrtc_RtpTransceiverInit_delete(video_init);
        webrtc_RTCError_unique_delete(rtc_error);
        return;
      }

      struct webrtc_RtpCapabilities* caps =
          webrtc_PeerConnectionFactoryInterface_GetRtpSenderCapabilities(
              webrtc_PeerConnectionFactoryInterface_refcounted_get(
                  self->config->pc_factory),
              webrtc_MediaType_VIDEO);
      struct webrtc_RtpCodecCapability_vector* src_codecs =
          webrtc_RtpCapabilities_get_codecs(caps);
      int src_codecs_size = webrtc_RtpCodecCapability_vector_size(src_codecs);
      for (int i = 0; i < src_codecs_size; ++i) {
        struct webrtc_RtpCodecCapability* codec =
            webrtc_RtpCodecCapability_vector_get(src_codecs, i);
        struct std_string* codec_name =
            webrtc_RtpCodecCapability_get_name(codec);
        RTC_LOG_WARNING("codec: %s", std_string_c_str(codec_name));
        struct std_map_string_string* params =
            webrtc_RtpCodecCapability_get_parameters(codec);
        struct std_map_string_string_iter* params_iter =
            std_map_string_string_iter_new(params);
        struct std_string_unique* key = NULL;
        struct std_string_unique* value = NULL;
        while (std_map_string_string_iter_next(params_iter, &key, &value)) {
          RTC_LOG_WARNING("  %s: %s",
                          std_string_c_str(std_string_unique_get(key)),
                          std_string_c_str(std_string_unique_get(value)));
          std_string_unique_delete(key);
          std_string_unique_delete(value);
          key = NULL;
          value = NULL;
        }
        std_map_string_string_iter_delete(params_iter);
      }
      struct webrtc_RtpCodecCapability_vector* codecs =
          webrtc_RtpCodecCapability_vector_new(0);
      if (self->config->send_encodings != NULL) {
        int enc_size = webrtc_RtpEncodingParameters_vector_size(
            self->config->send_encodings);
        for (int i = 0; i < enc_size; ++i) {
          struct webrtc_RtpEncodingParameters* enc =
              webrtc_RtpEncodingParameters_vector_get(
                  self->config->send_encodings, i);
          struct webrtc_RtpCodecCapability* encoding_codec =
              webrtc_RtpEncodingParameters_get_codec(enc);
          const char* encoding_codec_name = "none";
          if (encoding_codec != NULL) {
            encoding_codec_name = std_string_c_str(
                webrtc_RtpCodecCapability_get_name(encoding_codec));
          }
          RTC_LOG_WARNING("send_encoding: %s", encoding_codec_name);
          if (encoding_codec == NULL) {
            continue;
          }
          for (int j = 0; j < src_codecs_size; ++j) {
            struct webrtc_RtpCodecCapability* cap_codec =
                webrtc_RtpCodecCapability_vector_get(src_codecs, j);
            if (!whip_RtpCodecCapability_is_same_format(cap_codec,
                                                        encoding_codec)) {
              continue;
            }
            RTC_LOG_WARNING("match codec: %s",
                            std_string_c_str(
                                webrtc_RtpCodecCapability_get_name(cap_codec)));
            bool exists = false;
            int codecs_size = webrtc_RtpCodecCapability_vector_size(codecs);
            for (int k = 0; k < codecs_size; ++k) {
              struct webrtc_RtpCodecCapability* existing =
                  webrtc_RtpCodecCapability_vector_get(codecs, k);
              if (whip_RtpCodecCapability_is_same_format(existing, cap_codec)) {
                exists = true;
                break;
              }
            }
            if (!exists) {
              RTC_LOG_WARNING(
                  "add codec: %s",
                  std_string_c_str(
                      webrtc_RtpCodecCapability_get_name(cap_codec)));
              webrtc_RtpCodecCapability_vector_push_back(codecs, cap_codec);
            }
            break;
          }
        }
      }
      for (int i = 0; i < src_codecs_size; ++i) {
        struct webrtc_RtpCodecCapability* codec =
            webrtc_RtpCodecCapability_vector_get(src_codecs, i);
        struct std_string* name = webrtc_RtpCodecCapability_get_name(codec);
        if (strcmp(std_string_c_str(name), "rtx") == 0) {
          webrtc_RtpCodecCapability_vector_push_back(codecs, codec);
          break;
        }
      }

      rtc_error = webrtc_RtpTransceiverInterface_SetCodecPreferences(
          webrtc_RtpTransceiverInterface_refcounted_get(transceiver), codecs);
      webrtc_RtpCodecCapability_vector_delete(codecs);
      webrtc_RtpCapabilities_delete(caps);
      webrtc_RtpTransceiverInterface_Release(
          webrtc_RtpTransceiverInterface_refcounted_get(transceiver));
      if (rtc_error != NULL) {
        RTC_LOG_ERROR("Failed to SetCodecPreferences(video): error=%p",
                      rtc_error);
        webrtc_RTCError_unique_delete(rtc_error);
        return;
      }
    }
    webrtc_RtpTransceiverInit_delete(video_init);
  }

  SignalingWhip_SetState(self, SIGNALLING_WHIP_STATE_CONNECTING);

  struct webrtc_CreateSessionDescriptionObserver* offer_observer =
      webrtc_CreateSessionDescriptionObserver_make_ref_counted(&self->offer_cbs,
                                                               self);
  webrtc_PeerConnectionInterface_CreateOffer(
      webrtc_PeerConnectionInterface_refcounted_get(self->pc), offer_observer,
      NULL);
  webrtc_CreateSessionDescriptionObserver_Release(offer_observer);
}

int SignalingWhip_WaitForConnect(struct SignalingWhip* self) {
  RTC_LOG_INFO("SignalingWhip_WaitForConnect");
  pthread_mutex_lock(&self->mutex);
  while (self->state == SIGNALLING_WHIP_STATE_CONNECTING) {
    pthread_cond_wait(&self->cond, &self->mutex);
  }
  int connected = (self->state == SIGNALLING_WHIP_STATE_CONNECTED);
  pthread_mutex_unlock(&self->mutex);
  return connected;
}

void SignalingWhip_Disconnect(struct SignalingWhip* self) {
  RTC_LOG_INFO("SignalingWhip_Disconnect");
  if (self->pc != NULL) {
    webrtc_PeerConnectionInterface_Release(
        webrtc_PeerConnectionInterface_refcounted_get(self->pc));
    self->pc = NULL;
  }
  SignalingWhip_SetState(self, SIGNALLING_WHIP_STATE_CLOSED);
}

int main() {
  webrtc_LogMessage_LogToDebug(webrtc_LogSeverity_LS_INFO);
  webrtc_LogMessage_LogTimestamps();
  webrtc_LogMessage_LogThreads();

  struct PeerConnectionFactory* factory = PeerConnectionFactory_Create();
  if (factory == NULL) {
    return 1;
  }

  struct FakeVideoCapturerConfig fake_config = {0};
  fake_config.width = 1920;
  fake_config.height = 1080;
  fake_config.fps = 30;
  struct FakeVideoCapturer* capturer = FakeVideoCapturer_Create(&fake_config);

  struct webrtc_VideoTrackSourceInterface_refcounted* video_source =
      FakeVideoCapturer_GetSource(capturer);

  struct webrtc_RtpEncodingParameters_vector* send_encodings =
      webrtc_RtpEncodingParameters_vector_new(3);
  struct webrtc_RtpEncodingParameters* enc0 =
      webrtc_RtpEncodingParameters_vector_get(send_encodings, 0);
  struct webrtc_RtpEncodingParameters* enc1 =
      webrtc_RtpEncodingParameters_vector_get(send_encodings, 1);
  struct webrtc_RtpEncodingParameters* enc2 =
      webrtc_RtpEncodingParameters_vector_get(send_encodings, 2);
  webrtc_RtpEncodingParameters_set_rid(enc0, "r0", strlen("r0"));
  webrtc_RtpEncodingParameters_set_scale_resolution_down_by(enc0, 4.0);
  webrtc_RtpEncodingParameters_set_rid(enc1, "r1", strlen("r1"));
  webrtc_RtpEncodingParameters_set_scale_resolution_down_by(enc1, 2.0);
  webrtc_RtpEncodingParameters_set_rid(enc2, "r2", strlen("r2"));
  webrtc_RtpEncodingParameters_set_scale_resolution_down_by(enc2, 1.0);
  struct webrtc_RtpCodecCapability* av1_codec = webrtc_RtpCodecCapability_new();
  webrtc_RtpCodecCapability_set_kind(av1_codec, webrtc_MediaType_VIDEO);
  webrtc_RtpCodecCapability_set_name(av1_codec, "AV1", strlen("AV1"));
  webrtc_RtpCodecCapability_set_clock_rate(av1_codec, 90000);
  struct std_map_string_string* av1_params =
      webrtc_RtpCodecCapability_get_parameters(av1_codec);
  std_map_string_string_set(av1_params, "level-idx", 9, "5", 1);
  std_map_string_string_set(av1_params, "profile", 7, "0", 1);
  std_map_string_string_set(av1_params, "tier", 4, "0", 1);
  webrtc_RtpEncodingParameters_set_codec(enc0, av1_codec);
  webrtc_RtpEncodingParameters_set_codec(enc1, av1_codec);
  webrtc_RtpEncodingParameters_set_codec(enc2, av1_codec);

  struct SignalingWhipConfig* config = SignalingWhipConfig_create();
  SignalingWhipConfig_set_signaling_url(config,
                                        "http://192.0.2.1/whip");
  SignalingWhipConfig_set_channel_id(config, "sora");
  SignalingWhipConfig_set_pc_factory(config, factory->factory);
  if (video_source != NULL) {
    SignalingWhipConfig_set_video_source(config, video_source);
  }
  SignalingWhipConfig_set_send_encodings(config, send_encodings);
  struct SignalingWhip* whip = SignalingWhip_Create(config);
  SignalingWhipConfig_delete(config);
  if (video_source != NULL) {
    webrtc_VideoTrackSourceInterface_Release(
        webrtc_VideoTrackSourceInterface_refcounted_get(video_source));
  }
  webrtc_RtpEncodingParameters_vector_delete(send_encodings);
  webrtc_RtpCodecCapability_delete(av1_codec);
  SignalingWhip_Connect(whip);
  SignalingWhip_WaitForConnect(whip);

  usleep(10000000);  // 10 second

  SignalingWhip_Disconnect(whip);
  webrtc_RefCountInterface_Release(whip->ref);
  webrtc_RefCountInterface_Release(factory->ref);

  webrtc_RefCountInterface_Release(capturer->ref);

  return 0;
}
