#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>

#include <netdb.h>
#include <openssl/err.h>
#include <openssl/ssl.h>
#include <sys/socket.h>

// webrtc_c
#include "webrtc_c.h"

// libyuv
#include <libyuv/convert_from.h>
#include <libyuv/video_common.h>

// POSIX
#ifdef __linux__
#include <bits/pthreadtypes.h>
#endif
#include <pthread.h>
#include <unistd.h>

static void whep_OnSendRequestResponse(char* resp, void* user_data);

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

struct StringBuffer {
  char* data;
  size_t size;
  size_t capacity;
};

static int StringBuffer_Reserve(struct StringBuffer* buffer, size_t capacity) {
  if (capacity <= buffer->capacity) {
    return 1;
  }
  size_t new_capacity = buffer->capacity == 0 ? 1 : buffer->capacity;
  while (new_capacity < capacity) {
    new_capacity *= 2;
  }
  char* new_data = (char*)realloc(buffer->data, new_capacity);
  if (new_data == NULL) {
    return 0;
  }
  buffer->data = new_data;
  buffer->capacity = new_capacity;
  return 1;
}

static int StringBuffer_Append(struct StringBuffer* buffer,
                               const char* data,
                               size_t len) {
  if (!StringBuffer_Reserve(buffer, buffer->size + len + 1)) {
    return 0;
  }
  memcpy(buffer->data + buffer->size, data, len);
  buffer->size += len;
  buffer->data[buffer->size] = '\0';
  return 1;
}

static int StringBuffer_AppendCStr(struct StringBuffer* buffer,
                                   const char* data) {
  return StringBuffer_Append(buffer, data, strlen(data));
}

static int StringBuffer_AppendInt(struct StringBuffer* buffer, int value) {
  char tmp[32];
  int len = snprintf(tmp, sizeof(tmp), "%d", value);
  if (len <= 0) {
    return 0;
  }
  return StringBuffer_Append(buffer, tmp, (size_t)len);
}

static void StringBuffer_Free(struct StringBuffer* buffer) {
  free(buffer->data);
  buffer->data = NULL;
  buffer->size = 0;
  buffer->capacity = 0;
}

struct AnsiRenderer {
  struct webrtc_VideoSinkInterface* sink;
  struct webrtc_VideoSinkInterface_cbs sink_cbs;
  int width;
  int height;
};

static int AnsiRenderer_RgbToAnsi256(uint8_t r, uint8_t g, uint8_t b) {
  // 216色キューブ（6x6x6）を使用
  // RGB値を0-5の範囲に変換
  int r6 = (r * 5) / 255;
  int g6 = (g * 5) / 255;
  int b6 = (b * 5) / 255;

  // ANSI 256色の216色キューブは16から始まる
  return 16 + (r6 * 36) + (g6 * 6) + b6;
}

static void AnsiRenderer_OnFrame(const struct webrtc_VideoFrame* frame,
                                 void* user_data) {
  struct AnsiRenderer* renderer = (struct AnsiRenderer*)user_data;
  if (renderer == NULL || frame == NULL) {
    return;
  }

  // width, height に合わせてリサイズ
  struct webrtc_I420Buffer_refcounted* buf =
      webrtc_I420Buffer_Create(renderer->width, renderer->height);
  struct webrtc_I420Buffer_refcounted* src =
      webrtc_VideoFrame_video_frame_buffer(frame);
  webrtc_I420Buffer_ScaleFrom(webrtc_I420Buffer_refcounted_get(buf),
                              webrtc_I420Buffer_refcounted_get(src));
  webrtc_I420Buffer_Release(webrtc_I420Buffer_refcounted_get(src));

  // ARGB に変換
  size_t image_size = (size_t)renderer->width * (size_t)renderer->height * 4;
  uint8_t* image = (uint8_t*)malloc(image_size);
  if (image == NULL) {
    webrtc_I420Buffer_Release(webrtc_I420Buffer_refcounted_get(buf));
    return;
  }
  ConvertFromI420(
      webrtc_I420Buffer_MutableDataY(webrtc_I420Buffer_refcounted_get(buf)),
      webrtc_I420Buffer_StrideY(webrtc_I420Buffer_refcounted_get(buf)),
      webrtc_I420Buffer_MutableDataU(webrtc_I420Buffer_refcounted_get(buf)),
      webrtc_I420Buffer_StrideU(webrtc_I420Buffer_refcounted_get(buf)),
      webrtc_I420Buffer_MutableDataV(webrtc_I420Buffer_refcounted_get(buf)),
      webrtc_I420Buffer_StrideV(webrtc_I420Buffer_refcounted_get(buf)), image,
      renderer->width * 4, renderer->width, renderer->height, FOURCC_ARGB);
  webrtc_I420Buffer_Release(webrtc_I420Buffer_refcounted_get(buf));

  struct StringBuffer output = {0};
  if (!StringBuffer_Reserve(
          &output, (size_t)renderer->width * (size_t)renderer->height * 20)) {
    free(image);
    return;
  }
  StringBuffer_AppendCStr(&output, "\033[H");

  // 2x1ピクセルを1文字で表現（上半分と下半分の色を使用）
  for (int y = 0; y < renderer->height; y += 2) {
    StringBuffer_AppendCStr(&output, "\033[2K");  // 行をクリア

    for (int x = 0; x < renderer->width; x++) {
      // 上のピクセル（y）
      int upper_offset = (y * renderer->width + x) * 4;
      uint8_t upper_r = image[upper_offset + 2];
      uint8_t upper_g = image[upper_offset + 1];
      uint8_t upper_b = image[upper_offset + 0];

      // 下のピクセル（y+1）
      uint8_t lower_r = upper_r;
      uint8_t lower_g = upper_g;
      uint8_t lower_b = upper_b;
      if (y + 1 < renderer->height) {
        int lower_offset = ((y + 1) * renderer->width + x) * 4;
        lower_r = image[lower_offset + 2];
        lower_g = image[lower_offset + 1];
        lower_b = image[lower_offset + 0];
      }

      // 上半分の色を前景色、下半分の色を背景色として設定
      int upper_color = AnsiRenderer_RgbToAnsi256(upper_r, upper_g, upper_b);
      int lower_color = AnsiRenderer_RgbToAnsi256(lower_r, lower_g, lower_b);

      // 上半分ブロック文字（▀）を使用
      StringBuffer_AppendCStr(&output, "\033[38;5;");
      StringBuffer_AppendInt(&output, upper_color);
      StringBuffer_AppendCStr(&output, "m\033[48;5;");
      StringBuffer_AppendInt(&output, lower_color);
      StringBuffer_AppendCStr(&output, "m▀");
    }

    StringBuffer_AppendCStr(&output, "\033[0m\n");  // 色をリセットして改行
  }

  // 一括出力
  fwrite(output.data, 1, output.size, stdout);
  fflush(stdout);
  StringBuffer_Free(&output);
  free(image);
}

static struct AnsiRenderer* AnsiRenderer_Create() {
  struct AnsiRenderer* renderer =
      (struct AnsiRenderer*)calloc(1, sizeof(struct AnsiRenderer));
  if (renderer == NULL) {
    return NULL;
  }
  renderer->width = 80;
  renderer->height = 45;
  renderer->sink_cbs.OnFrame = AnsiRenderer_OnFrame;
  renderer->sink = webrtc_VideoSinkInterface_new(&renderer->sink_cbs, renderer);
  if (renderer->sink == NULL) {
    free(renderer);
    return NULL;
  }
  return renderer;
}

static void AnsiRenderer_Delete(struct AnsiRenderer* renderer) {
  if (renderer == NULL) {
    return;
  }
  if (renderer->sink != NULL) {
    webrtc_VideoSinkInterface_delete(renderer->sink);
  }
  free(renderer);
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

struct SignalingWhepConfig {
  struct webrtc_PeerConnectionFactoryInterface_refcounted* pc_factory;
  char* signaling_url;
  char* channel_id;
};
struct SignalingWhepConfig* SignalingWhepConfig_create() {
  return (struct SignalingWhepConfig*)calloc(
      1, sizeof(struct SignalingWhepConfig));
}
struct SignalingWhepConfig* SignalingWhepConfig_copy(
    struct SignalingWhepConfig* src) {
  struct SignalingWhepConfig* dst = (struct SignalingWhepConfig*)calloc(
      1, sizeof(struct SignalingWhepConfig));
  if (src->pc_factory != NULL) {
    dst->pc_factory = src->pc_factory;
    webrtc_PeerConnectionFactoryInterface_AddRef(
        webrtc_PeerConnectionFactoryInterface_refcounted_get(dst->pc_factory));
  }
  if (src->signaling_url != NULL) {
    dst->signaling_url = strdup(src->signaling_url);
  }
  if (src->channel_id != NULL) {
    dst->channel_id = strdup(src->channel_id);
  }
  return dst;
}
void SignalingWhepConfig_delete(struct SignalingWhepConfig* config) {
  if (config->pc_factory != NULL) {
    webrtc_PeerConnectionFactoryInterface_Release(
        webrtc_PeerConnectionFactoryInterface_refcounted_get(
            config->pc_factory));
  }
  if (config->signaling_url != NULL) {
    free(config->signaling_url);
  }
  if (config->channel_id != NULL) {
    free(config->channel_id);
  }
  free(config);
}
void SignalingWhepConfig_set_pc_factory(
    struct SignalingWhepConfig* config,
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
void SignalingWhepConfig_set_signaling_url(struct SignalingWhepConfig* config,
                                           const char* signaling_url) {
  if (config->signaling_url != NULL) {
    free(config->signaling_url);
  }
  config->signaling_url = signaling_url ? strdup(signaling_url) : NULL;
}
void SignalingWhepConfig_set_channel_id(struct SignalingWhepConfig* config,
                                        const char* channel_id) {
  if (config->channel_id != NULL) {
    free(config->channel_id);
  }
  config->channel_id = channel_id ? strdup(channel_id) : NULL;
}

struct SignalingWhep {
  struct webrtc_RefCountInterface_ref* ref;
  struct webrtc_PeerConnectionObserver_cbs observer_cbs;
  struct webrtc_PeerConnectionObserver* observer;
  struct SignalingWhepConfig* config;
  struct webrtc_PeerConnectionInterface_refcounted* pc;
  struct AnsiRenderer* renderer;
  struct webrtc_VideoTrackInterface_refcounted* video_track;
  pthread_mutex_t mutex;
  pthread_cond_t cond;
  struct webrtc_SetLocalDescriptionObserverInterface_cbs loc_cbs;
  struct webrtc_SetRemoteDescriptionObserverInterface_cbs rem_cbs;
  struct webrtc_CreateSessionDescriptionObserver_cbs offer_cbs;
  enum {
    SIGNALLING_WHEP_STATE_INIT = 0,
    SIGNALLING_WHEP_STATE_CONNECTING,
    SIGNALLING_WHEP_STATE_CONNECTED,
    SIGNALLING_WHEP_STATE_CLOSED,
  } state;
};

struct whep_send_request_ctx {
  struct SignalingWhep* self;
  struct webrtc_SessionDescriptionInterface_unique* desc;
};

static void SignalingWhep_DetachVideoSink(struct SignalingWhep* self);
static void SignalingWhep_OnTrack(
    struct webrtc_RtpTransceiverInterface_refcounted* transceiver,
    void* user_data);
static void SignalingWhep_OnRemoveTrack(
    struct webrtc_RtpReceiverInterface_refcounted* receiver,
    void* user_data);

void SignalingWhep_delete(void* user_data) {
  struct SignalingWhep* p = (struct SignalingWhep*)user_data;
  SignalingWhepConfig_delete(p->config);
  if (p->pc != NULL) {
    webrtc_PeerConnectionInterface_Release(
        webrtc_PeerConnectionInterface_refcounted_get(p->pc));
  }
  SignalingWhep_DetachVideoSink(p);
  if (p->renderer != NULL) {
    AnsiRenderer_Delete(p->renderer);
  }
  if (p->observer != NULL) {
    webrtc_PeerConnectionObserver_delete(p->observer);
  }
  pthread_mutex_destroy(&p->mutex);
  pthread_cond_destroy(&p->cond);
  free(p);
}

void SignalingWhep_OnConnectionChange(
    webrtc_PeerConnectionInterface_PeerConnectionState new_state,
    void* user_data) {
  struct SignalingWhep* self = (struct SignalingWhep*)user_data;
  RTC_LOG_INFO("SignalingWhep_OnConnectionChange: new_state=%d", new_state);
  pthread_mutex_lock(&self->mutex);
  if (new_state ==
      webrtc_PeerConnectionInterface_PeerConnectionState_kConnected) {
    self->state = SIGNALLING_WHEP_STATE_CONNECTED;
  } else if (new_state ==
                 webrtc_PeerConnectionInterface_PeerConnectionState_kFailed ||
             new_state ==
                 webrtc_PeerConnectionInterface_PeerConnectionState_kClosed) {
    self->state = SIGNALLING_WHEP_STATE_CLOSED;
  }
  pthread_cond_broadcast(&self->cond);
  pthread_mutex_unlock(&self->mutex);
}

static void SignalingWhep_SetState(struct SignalingWhep* self, int state) {
  pthread_mutex_lock(&self->mutex);
  self->state = state;
  pthread_cond_broadcast(&self->cond);
  pthread_mutex_unlock(&self->mutex);
}

static void SignalingWhep_DetachVideoSink(struct SignalingWhep* self) {
  if (self->video_track == NULL) {
    return;
  }
  if (self->renderer != NULL && self->renderer->sink != NULL) {
    webrtc_VideoTrackInterface_RemoveSink(
        webrtc_VideoTrackInterface_refcounted_get(self->video_track),
        self->renderer->sink);
  }
  webrtc_VideoTrackInterface_Release(
      webrtc_VideoTrackInterface_refcounted_get(self->video_track));
  self->video_track = NULL;
}

static void SignalingWhep_OnTrack(
    struct webrtc_RtpTransceiverInterface_refcounted* transceiver,
    void* user_data) {
  struct SignalingWhep* self = (struct SignalingWhep*)user_data;
  if (self == NULL || transceiver == NULL) {
    return;
  }

  struct webrtc_RtpReceiverInterface_refcounted* receiver =
      webrtc_RtpTransceiverInterface_receiver(
          webrtc_RtpTransceiverInterface_refcounted_get(transceiver));
  webrtc_RtpTransceiverInterface_Release(
      webrtc_RtpTransceiverInterface_refcounted_get(transceiver));
  if (receiver == NULL) {
    return;
  }
  struct webrtc_MediaStreamTrackInterface_refcounted* track =
      webrtc_RtpReceiverInterface_track(
          webrtc_RtpReceiverInterface_refcounted_get(receiver));
  webrtc_RtpReceiverInterface_Release(
      webrtc_RtpReceiverInterface_refcounted_get(receiver));
  if (track == NULL) {
    return;
  }

  struct std_string_unique* kind = webrtc_MediaStreamTrackInterface_kind(
      webrtc_MediaStreamTrackInterface_refcounted_get(track));
  const char* kind_str = NULL;
  if (kind != NULL) {
    kind_str = std_string_c_str(std_string_unique_get(kind));
  }
  int is_video = kind_str != NULL && strcmp(kind_str, "video") == 0;
  std_string_unique_delete(kind);
  if (!is_video) {
    webrtc_MediaStreamTrackInterface_Release(
        webrtc_MediaStreamTrackInterface_refcounted_get(track));
    return;
  }

  struct webrtc_VideoTrackInterface_refcounted* video_track =
      webrtc_MediaStreamTrackInterface_refcounted_cast_to_webrtc_VideoTrackInterface(
          track);
  webrtc_MediaStreamTrackInterface_Release(
      webrtc_MediaStreamTrackInterface_refcounted_get(track));
  if (video_track == NULL) {
    return;
  }

  if (self->renderer == NULL || self->renderer->sink == NULL) {
    webrtc_VideoTrackInterface_Release(
        webrtc_VideoTrackInterface_refcounted_get(video_track));
    return;
  }

  if (self->video_track != NULL &&
      webrtc_VideoTrackInterface_refcounted_get(self->video_track) ==
          webrtc_VideoTrackInterface_refcounted_get(video_track)) {
    webrtc_VideoTrackInterface_Release(
        webrtc_VideoTrackInterface_refcounted_get(video_track));
    return;
  }

  SignalingWhep_DetachVideoSink(self);
  struct webrtc_VideoSinkWants* wants = webrtc_VideoSinkWants_new();
  if (wants == NULL) {
    webrtc_VideoTrackInterface_Release(
        webrtc_VideoTrackInterface_refcounted_get(video_track));
    return;
  }
  webrtc_VideoTrackInterface_AddOrUpdateSink(
      webrtc_VideoTrackInterface_refcounted_get(video_track),
      self->renderer->sink, wants);
  webrtc_VideoSinkWants_delete(wants);
  self->video_track = video_track;
}

static void SignalingWhep_OnRemoveTrack(
    struct webrtc_RtpReceiverInterface_refcounted* receiver,
    void* user_data) {
  struct SignalingWhep* self = (struct SignalingWhep*)user_data;
  if (self == NULL || receiver == NULL) {
    return;
  }
  struct webrtc_MediaStreamTrackInterface_refcounted* track =
      webrtc_RtpReceiverInterface_track(
          webrtc_RtpReceiverInterface_refcounted_get(receiver));
  webrtc_RtpReceiverInterface_Release(
      webrtc_RtpReceiverInterface_refcounted_get(receiver));
  if (track == NULL) {
    return;
  }
  struct std_string_unique* kind = webrtc_MediaStreamTrackInterface_kind(
      webrtc_MediaStreamTrackInterface_refcounted_get(track));
  const char* kind_str = NULL;
  if (kind != NULL) {
    kind_str = std_string_c_str(std_string_unique_get(kind));
  }
  int is_video = kind_str != NULL && strcmp(kind_str, "video") == 0;
  std_string_unique_delete(kind);
  if (!is_video) {
    webrtc_MediaStreamTrackInterface_Release(
        webrtc_MediaStreamTrackInterface_refcounted_get(track));
    return;
  }
  struct webrtc_VideoTrackInterface_refcounted* video_track =
      webrtc_MediaStreamTrackInterface_refcounted_cast_to_webrtc_VideoTrackInterface(
          track);
  webrtc_MediaStreamTrackInterface_Release(
      webrtc_MediaStreamTrackInterface_refcounted_get(track));
  if (video_track == NULL) {
    return;
  }
  if (self->video_track != NULL &&
      webrtc_VideoTrackInterface_refcounted_get(self->video_track) ==
          webrtc_VideoTrackInterface_refcounted_get(video_track)) {
    SignalingWhep_DetachVideoSink(self);
  }
  webrtc_VideoTrackInterface_Release(
      webrtc_VideoTrackInterface_refcounted_get(video_track));
}

struct whep_set_local_ctx {
  struct SignalingWhep* self;
  char* answer_body;
};

// forward declarations
static void whep_SendRequest(const char* host,
                             const char* port,
                             const char* req,
                             void (*on_response)(char* resp, void* user_data),
                             void* user_data);

static char* whep_find_header_value(const char* headers, const char* key) {
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

static void whep_OnSetRemoteDescriptionComplete(
    struct webrtc_RTCError_unique* error,
    void* user_data) {
  struct SignalingWhep* self = (struct SignalingWhep*)user_data;
  if (error != NULL && !webrtc_RTCError_ok(webrtc_RTCError_unique_get(error))) {
    const char* message = NULL;
    size_t message_len = 0;
    webrtc_RTCError_message(webrtc_RTCError_unique_get(error), &message,
                            &message_len);
    RTC_LOG_ERROR("Failed to SetRemoteDescription: %.*s", (int)message_len,
                  message ? message : "");
    SignalingWhep_SetState(self, SIGNALLING_WHEP_STATE_CLOSED);
    webrtc_RTCError_unique_delete(error);
    return;
  }
  if (error != NULL) {
    webrtc_RTCError_unique_delete(error);
  }
  SignalingWhep_SetState(self, SIGNALLING_WHEP_STATE_CONNECTED);
}

static void whep_OnSetLocalDescriptionComplete(
    struct webrtc_RTCError_unique* error,
    void* user_data) {
  struct whep_set_local_ctx* ctx = (struct whep_set_local_ctx*)user_data;
  struct SignalingWhep* self = ctx->self;
  if (error != NULL && !webrtc_RTCError_ok(webrtc_RTCError_unique_get(error))) {
    const char* message = NULL;
    size_t message_len = 0;
    webrtc_RTCError_message(webrtc_RTCError_unique_get(error), &message,
                            &message_len);
    RTC_LOG_ERROR("Failed to SetLocalDescription: %.*s", (int)message_len,
                  message ? message : "");
    SignalingWhep_SetState(self, SIGNALLING_WHEP_STATE_CLOSED);
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
    SignalingWhep_SetState(self, SIGNALLING_WHEP_STATE_CLOSED);
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

static void whep_OnCreateOfferFailure(struct webrtc_RTCError_unique* error,
                                      void* user_data) {
  struct SignalingWhep* self = (struct SignalingWhep*)user_data;
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
  SignalingWhep_SetState(self, SIGNALLING_WHEP_STATE_CLOSED);
  if (error != NULL) {
    webrtc_RTCError_unique_delete(error);
  }
}

static void whep_OnCreateOfferSuccess(
    struct webrtc_SessionDescriptionInterface_unique* desc,
    void* user_data) {
  struct SignalingWhep* self = (struct SignalingWhep*)user_data;
  struct std_string_unique* offer_sdp = NULL;
  if (!webrtc_SessionDescriptionInterface_ToString(
          webrtc_SessionDescriptionInterface_unique_get(desc), &offer_sdp)) {
    RTC_LOG_ERROR("Failed to get SDP");
    SignalingWhep_SetState(self, SIGNALLING_WHEP_STATE_CLOSED);
    webrtc_SessionDescriptionInterface_unique_delete(desc);
    return;
  }
  const char* offer_sdp_str =
      std_string_c_str(std_string_unique_get(offer_sdp));

  struct URLParts parts = {0};
  if (!URLParts_Parse(self->config->signaling_url, &parts)) {
    RTC_LOG_ERROR("Failed to parse url: %s", self->config->signaling_url);
    std_string_unique_delete(offer_sdp);
    SignalingWhep_SetState(self, SIGNALLING_WHEP_STATE_CLOSED);
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
    SignalingWhep_SetState(self, SIGNALLING_WHEP_STATE_CLOSED);
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
      strlen("\r\nUser-Agent: Whep-Client\r\nConnection: close\r\n\r\n") +
      strlen(offer_sdp_str) + 1;
  char* req = (char*)malloc(req_len);
  if (req == NULL) {
    free(target);
    URLParts_clear(&parts);
    std_string_unique_delete(offer_sdp);
    SignalingWhep_SetState(self, SIGNALLING_WHEP_STATE_CLOSED);
    webrtc_SessionDescriptionInterface_unique_delete(desc);
    return;
  }
  snprintf(req, req_len,
           "POST %s HTTP/1.1\r\nHost: %s:%s\r\n"
           "Content-Type: application/sdp\r\nContent-Length: %s\r\n"
           "User-Agent: Whep-Client\r\nConnection: close\r\n\r\n%s",
           target, parts.host, URLParts_GetPort(&parts), content_length,
           offer_sdp_str);

  struct whep_send_request_ctx* send_ctx =
      (struct whep_send_request_ctx*)calloc(
          1, sizeof(struct whep_send_request_ctx));
  if (send_ctx == NULL) {
    free(req);
    free(target);
    URLParts_clear(&parts);
    std_string_unique_delete(offer_sdp);
    SignalingWhep_SetState(self, SIGNALLING_WHEP_STATE_CLOSED);
    webrtc_SessionDescriptionInterface_unique_delete(desc);
    return;
  }
  send_ctx->self = self;
  send_ctx->desc = desc;

  whep_SendRequest(parts.host, URLParts_GetPort(&parts), req,
                   whep_OnSendRequestResponse, send_ctx);

  free(req);
  free(target);
  URLParts_clear(&parts);
  std_string_unique_delete(offer_sdp);
}

static void whep_SendRequest(const char* host,
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

static void whep_OnSendRequestResponse(char* resp, void* user_data) {
  struct whep_send_request_ctx* ctx = (struct whep_send_request_ctx*)user_data;
  struct SignalingWhep* self = ctx->self;
  struct webrtc_SessionDescriptionInterface_unique* desc = ctx->desc;

  if (resp == NULL) {
    RTC_LOG_ERROR("Failed to send request");
    SignalingWhep_SetState(self, SIGNALLING_WHEP_STATE_CLOSED);
    webrtc_SessionDescriptionInterface_unique_delete(desc);
    free(ctx);
    return;
  }
  RTC_LOG_INFO("Received response: %s", resp);

  char* header_end = strstr(resp, "\r\n\r\n");
  if (header_end == NULL) {
    RTC_LOG_ERROR("Invalid response");
    free(resp);
    SignalingWhep_SetState(self, SIGNALLING_WHEP_STATE_CLOSED);
    webrtc_SessionDescriptionInterface_unique_delete(desc);
    free(ctx);
    return;
  }

  size_t header_len = (size_t)(header_end - resp);
  char* headers = (char*)malloc(header_len + 1);
  if (headers == NULL) {
    free(resp);
    SignalingWhep_SetState(self, SIGNALLING_WHEP_STATE_CLOSED);
    webrtc_SessionDescriptionInterface_unique_delete(desc);
    free(ctx);
    return;
  }
  memcpy(headers, resp, header_len);
  headers[header_len] = '\0';
  char* body = strdup(header_end + 4);

  char* link_header = whep_find_header_value(headers, "link");
  free(headers);
  if (link_header == NULL) {
    RTC_LOG_ERROR("No Link header");
    free(resp);
    free(body);
    SignalingWhep_SetState(self, SIGNALLING_WHEP_STATE_CLOSED);
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
    SignalingWhep_SetState(self, SIGNALLING_WHEP_STATE_CLOSED);
    webrtc_SessionDescriptionInterface_unique_delete(desc);
    free(ctx);
    return;
  }

  struct whep_set_local_ctx* loc_ctx =
      (struct whep_set_local_ctx*)calloc(1, sizeof(struct whep_set_local_ctx));
  if (loc_ctx == NULL) {
    free(resp);
    free(body);
    SignalingWhep_SetState(self, SIGNALLING_WHEP_STATE_CLOSED);
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

struct SignalingWhep* SignalingWhep_Create(struct SignalingWhepConfig* config) {
  struct SignalingWhep* p =
      (struct SignalingWhep*)calloc(1, sizeof(struct SignalingWhep));
  p->ref = webrtc_RefCountInterface_Create(SignalingWhep_delete, p);
  p->observer_cbs.OnConnectionChange = SignalingWhep_OnConnectionChange;
  p->observer_cbs.OnTrack = SignalingWhep_OnTrack;
  p->observer_cbs.OnRemoveTrack = SignalingWhep_OnRemoveTrack;
  p->loc_cbs.OnSetLocalDescriptionComplete = whep_OnSetLocalDescriptionComplete;
  p->rem_cbs.OnSetRemoteDescriptionComplete =
      whep_OnSetRemoteDescriptionComplete;
  p->offer_cbs.OnSuccess = whep_OnCreateOfferSuccess;
  p->offer_cbs.OnFailure = whep_OnCreateOfferFailure;
  p->config = SignalingWhepConfig_copy(config);
  p->renderer = AnsiRenderer_Create();
  pthread_mutex_init(&p->mutex, NULL);
  pthread_cond_init(&p->cond, NULL);
  p->state = SIGNALLING_WHEP_STATE_INIT;

  return p;
}

void SignalingWhep_Connect(struct SignalingWhep* self) {
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
        init, webrtc_RtpTransceiverDirection_kRecvOnly);
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
    webrtc_RtpTransceiverInterface_Release(
        webrtc_RtpTransceiverInterface_refcounted_get(transceiver));
  }

  {
    struct webrtc_RtpTransceiverInit* init = webrtc_RtpTransceiverInit_new();
    webrtc_RtpTransceiverInit_set_direction(
        init, webrtc_RtpTransceiverDirection_kRecvOnly);
    struct webrtc_RtpTransceiverInterface_refcounted* transceiver = NULL;
    rtc_error = NULL;
    webrtc_PeerConnectionInterface_AddTransceiver(
        webrtc_PeerConnectionInterface_refcounted_get(self->pc),
        webrtc_MediaType_VIDEO, init, &transceiver, &rtc_error);
    webrtc_RtpTransceiverInit_delete(init);
    if (rtc_error != NULL) {
      RTC_LOG_ERROR("Failed to AddTransceiver(video): error=%p", rtc_error);
      webrtc_RTCError_unique_delete(rtc_error);
      return;
    }
    webrtc_RtpTransceiverInterface_Release(
        webrtc_RtpTransceiverInterface_refcounted_get(transceiver));
  }

  SignalingWhep_SetState(self, SIGNALLING_WHEP_STATE_CONNECTING);

  struct webrtc_CreateSessionDescriptionObserver* offer_observer =
      webrtc_CreateSessionDescriptionObserver_make_ref_counted(&self->offer_cbs,
                                                               self);
  webrtc_PeerConnectionInterface_CreateOffer(
      webrtc_PeerConnectionInterface_refcounted_get(self->pc), offer_observer,
      NULL);
  webrtc_CreateSessionDescriptionObserver_Release(offer_observer);
}

int SignalingWhep_WaitForConnect(struct SignalingWhep* self) {
  RTC_LOG_INFO("SignalingWhep_WaitForConnect");
  pthread_mutex_lock(&self->mutex);
  while (self->state == SIGNALLING_WHEP_STATE_CONNECTING) {
    pthread_cond_wait(&self->cond, &self->mutex);
  }
  int connected = (self->state == SIGNALLING_WHEP_STATE_CONNECTED);
  pthread_mutex_unlock(&self->mutex);
  return connected;
}

void SignalingWhep_Disconnect(struct SignalingWhep* self) {
  RTC_LOG_INFO("SignalingWhep_Disconnect");
  SignalingWhep_DetachVideoSink(self);
  if (self->pc != NULL) {
    webrtc_PeerConnectionInterface_Release(
        webrtc_PeerConnectionInterface_refcounted_get(self->pc));
    self->pc = NULL;
  }
  SignalingWhep_SetState(self, SIGNALLING_WHEP_STATE_CLOSED);
}

int main() {
  //webrtc_LogMessage_LogToDebug(webrtc_LogSeverity_LS_INFO);
  //webrtc_LogMessage_LogTimestamps();
  //webrtc_LogMessage_LogThreads();

  struct PeerConnectionFactory* factory = PeerConnectionFactory_Create();
  if (factory == NULL) {
    return 1;
  }

  struct SignalingWhepConfig* config = SignalingWhepConfig_create();
  SignalingWhepConfig_set_signaling_url(config,
                                        "http://192.0.2.1/whep");
  SignalingWhepConfig_set_channel_id(config, "sora");
  SignalingWhepConfig_set_pc_factory(config, factory->factory);
  struct SignalingWhep* whep = SignalingWhep_Create(config);
  SignalingWhepConfig_delete(config);
  SignalingWhep_Connect(whep);
  SignalingWhep_WaitForConnect(whep);

  usleep(30000000);

  SignalingWhep_Disconnect(whep);
  webrtc_RefCountInterface_Release(whep->ref);
  webrtc_RefCountInterface_Release(factory->ref);

  return 0;
}
