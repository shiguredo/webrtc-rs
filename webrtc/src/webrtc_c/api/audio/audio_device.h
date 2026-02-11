#pragma once

#include <stdint.h>

#include "../../common.h"
#include "../environment.h"
#include "audio_device_defines.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// webrtc::AudioDeviceModule
// -------------------------

extern const int webrtc_AudioDeviceModule_kPlatformDefaultAudio;
extern const int webrtc_AudioDeviceModule_kWindowsCoreAudio;
extern const int webrtc_AudioDeviceModule_kWindowsCoreAudio2;
extern const int webrtc_AudioDeviceModule_kLinuxAlsaAudio;
extern const int webrtc_AudioDeviceModule_kLinuxPulseAudio;
extern const int webrtc_AudioDeviceModule_kAndroidJavaAudio;
extern const int webrtc_AudioDeviceModule_kAndroidOpenSLESAudio;
extern const int
    webrtc_AudioDeviceModule_kAndroidJavaInputAndOpenSLESOutputAudio;
extern const int webrtc_AudioDeviceModule_kAndroidAAudioAudio;
extern const int webrtc_AudioDeviceModule_kAndroidJavaInputAndAAudioOutputAudio;
extern const int webrtc_AudioDeviceModule_kDummyAudio;

extern const int webrtc_AudioDeviceModule_kDefaultCommunicationDevice;
extern const int webrtc_AudioDeviceModule_kDefaultDevice;

WEBRTC_DECLARE_REFCOUNTED(webrtc_AudioDeviceModule);
struct webrtc_AudioDeviceModule_Stats;

struct webrtc_AudioDeviceModule_refcounted* webrtc_CreateAudioDeviceModule(
    struct webrtc_Environment* env,
    int audio_type);

int32_t webrtc_AudioDeviceModule_ActiveAudioLayer(
    struct webrtc_AudioDeviceModule* self,
    int* audio_layer);

int32_t webrtc_AudioDeviceModule_RegisterAudioCallback(
    struct webrtc_AudioDeviceModule* self,
    struct webrtc_AudioTransport* audio_transport);

int32_t webrtc_AudioDeviceModule_Init(struct webrtc_AudioDeviceModule* self);

int32_t webrtc_AudioDeviceModule_Terminate(
    struct webrtc_AudioDeviceModule* self);

int webrtc_AudioDeviceModule_Initialized(struct webrtc_AudioDeviceModule* self);

int16_t webrtc_AudioDeviceModule_PlayoutDevices(
    struct webrtc_AudioDeviceModule* self);

int16_t webrtc_AudioDeviceModule_RecordingDevices(
    struct webrtc_AudioDeviceModule* self);

int32_t webrtc_AudioDeviceModule_PlayoutDeviceName(
    struct webrtc_AudioDeviceModule* self,
    uint16_t index,
    char name[128],
    char guid[128]);

int32_t webrtc_AudioDeviceModule_RecordingDeviceName(
    struct webrtc_AudioDeviceModule* self,
    uint16_t index,
    char name[128],
    char guid[128]);

int32_t webrtc_AudioDeviceModule_SetPlayoutDevice(
    struct webrtc_AudioDeviceModule* self,
    uint16_t index);

int32_t webrtc_AudioDeviceModule_SetPlayoutDeviceWithWindowsDeviceType(
    struct webrtc_AudioDeviceModule* self,
    int device);

int32_t webrtc_AudioDeviceModule_SetRecordingDevice(
    struct webrtc_AudioDeviceModule* self,
    uint16_t index);

int32_t webrtc_AudioDeviceModule_SetRecordingDeviceWithWindowsDeviceType(
    struct webrtc_AudioDeviceModule* self,
    int device);

int32_t webrtc_AudioDeviceModule_PlayoutIsAvailable(
    struct webrtc_AudioDeviceModule* self,
    int* available);

int32_t webrtc_AudioDeviceModule_InitPlayout(
    struct webrtc_AudioDeviceModule* self);

int webrtc_AudioDeviceModule_PlayoutIsInitialized(
    struct webrtc_AudioDeviceModule* self);

int32_t webrtc_AudioDeviceModule_RecordingIsAvailable(
    struct webrtc_AudioDeviceModule* self,
    int* available);

int32_t webrtc_AudioDeviceModule_InitRecording(
    struct webrtc_AudioDeviceModule* self);

int webrtc_AudioDeviceModule_RecordingIsInitialized(
    struct webrtc_AudioDeviceModule* self);

int32_t webrtc_AudioDeviceModule_StartPlayout(
    struct webrtc_AudioDeviceModule* self);

int32_t webrtc_AudioDeviceModule_StopPlayout(
    struct webrtc_AudioDeviceModule* self);

int webrtc_AudioDeviceModule_Playing(struct webrtc_AudioDeviceModule* self);

int32_t webrtc_AudioDeviceModule_StartRecording(
    struct webrtc_AudioDeviceModule* self);

int32_t webrtc_AudioDeviceModule_StopRecording(
    struct webrtc_AudioDeviceModule* self);

int webrtc_AudioDeviceModule_Recording(struct webrtc_AudioDeviceModule* self);

int32_t webrtc_AudioDeviceModule_InitSpeaker(
    struct webrtc_AudioDeviceModule* self);

int webrtc_AudioDeviceModule_SpeakerIsInitialized(
    struct webrtc_AudioDeviceModule* self);

int32_t webrtc_AudioDeviceModule_InitMicrophone(
    struct webrtc_AudioDeviceModule* self);

int webrtc_AudioDeviceModule_MicrophoneIsInitialized(
    struct webrtc_AudioDeviceModule* self);

int32_t webrtc_AudioDeviceModule_SpeakerVolumeIsAvailable(
    struct webrtc_AudioDeviceModule* self,
    int* available);

int32_t webrtc_AudioDeviceModule_SetSpeakerVolume(
    struct webrtc_AudioDeviceModule* self,
    uint32_t volume);

int32_t webrtc_AudioDeviceModule_SpeakerVolume(
    struct webrtc_AudioDeviceModule* self,
    uint32_t* volume);

int32_t webrtc_AudioDeviceModule_MaxSpeakerVolume(
    struct webrtc_AudioDeviceModule* self,
    uint32_t* max_volume);

int32_t webrtc_AudioDeviceModule_MinSpeakerVolume(
    struct webrtc_AudioDeviceModule* self,
    uint32_t* min_volume);

int32_t webrtc_AudioDeviceModule_MicrophoneVolumeIsAvailable(
    struct webrtc_AudioDeviceModule* self,
    int* available);

int32_t webrtc_AudioDeviceModule_SetMicrophoneVolume(
    struct webrtc_AudioDeviceModule* self,
    uint32_t volume);

int32_t webrtc_AudioDeviceModule_MicrophoneVolume(
    struct webrtc_AudioDeviceModule* self,
    uint32_t* volume);

int32_t webrtc_AudioDeviceModule_MaxMicrophoneVolume(
    struct webrtc_AudioDeviceModule* self,
    uint32_t* max_volume);

int32_t webrtc_AudioDeviceModule_MinMicrophoneVolume(
    struct webrtc_AudioDeviceModule* self,
    uint32_t* min_volume);

int32_t webrtc_AudioDeviceModule_SpeakerMuteIsAvailable(
    struct webrtc_AudioDeviceModule* self,
    int* available);

int32_t webrtc_AudioDeviceModule_SetSpeakerMute(
    struct webrtc_AudioDeviceModule* self,
    int enable);

int32_t webrtc_AudioDeviceModule_SpeakerMute(
    struct webrtc_AudioDeviceModule* self,
    int* enabled);

int32_t webrtc_AudioDeviceModule_MicrophoneMuteIsAvailable(
    struct webrtc_AudioDeviceModule* self,
    int* available);

int32_t webrtc_AudioDeviceModule_SetMicrophoneMute(
    struct webrtc_AudioDeviceModule* self,
    int enable);

int32_t webrtc_AudioDeviceModule_MicrophoneMute(
    struct webrtc_AudioDeviceModule* self,
    int* enabled);

int32_t webrtc_AudioDeviceModule_StereoPlayoutIsAvailable(
    struct webrtc_AudioDeviceModule* self,
    int* available);

int32_t webrtc_AudioDeviceModule_SetStereoPlayout(
    struct webrtc_AudioDeviceModule* self,
    int enable);

int32_t webrtc_AudioDeviceModule_StereoPlayout(
    struct webrtc_AudioDeviceModule* self,
    int* enabled);

int32_t webrtc_AudioDeviceModule_StereoRecordingIsAvailable(
    struct webrtc_AudioDeviceModule* self,
    int* available);

int32_t webrtc_AudioDeviceModule_SetStereoRecording(
    struct webrtc_AudioDeviceModule* self,
    int enable);

int32_t webrtc_AudioDeviceModule_StereoRecording(
    struct webrtc_AudioDeviceModule* self,
    int* enabled);

int32_t webrtc_AudioDeviceModule_PlayoutDelay(
    struct webrtc_AudioDeviceModule* self,
    uint16_t* delay_ms);

int webrtc_AudioDeviceModule_BuiltInAECIsAvailable(
    struct webrtc_AudioDeviceModule* self);

int webrtc_AudioDeviceModule_BuiltInAGCIsAvailable(
    struct webrtc_AudioDeviceModule* self);

int webrtc_AudioDeviceModule_BuiltInNSIsAvailable(
    struct webrtc_AudioDeviceModule* self);

int32_t webrtc_AudioDeviceModule_EnableBuiltInAEC(
    struct webrtc_AudioDeviceModule* self,
    int enable);

int32_t webrtc_AudioDeviceModule_EnableBuiltInAGC(
    struct webrtc_AudioDeviceModule* self,
    int enable);

int32_t webrtc_AudioDeviceModule_EnableBuiltInNS(
    struct webrtc_AudioDeviceModule* self,
    int enable);

int32_t webrtc_AudioDeviceModule_GetPlayoutUnderrunCount(
    struct webrtc_AudioDeviceModule* self);

int webrtc_AudioDeviceModule_GetStats(
    struct webrtc_AudioDeviceModule* self,
    struct webrtc_AudioDeviceModule_Stats* out_stats);

// -------------------------
// AudioDeviceModule (Callbacks)
// -------------------------

struct webrtc_AudioDeviceModule_Stats {
  double synthesized_samples_duration_s;
  uint64_t synthesized_samples_events;
  double total_samples_duration_s;
  double total_playout_delay_s;
  uint64_t total_samples_count;
};

struct webrtc_AudioDeviceModule_cbs {
  int32_t (*ActiveAudioLayer)(int* audio_layer, void* user_data);
  int32_t (*RegisterAudioCallback)(
      struct webrtc_AudioTransport* audio_transport,
      void* user_data);
  int32_t (*Init)(void* user_data);
  int32_t (*Terminate)(void* user_data);
  int (*Initialized)(void* user_data);

  int16_t (*PlayoutDevices)(void* user_data);
  int16_t (*RecordingDevices)(void* user_data);
  int32_t (*PlayoutDeviceName)(uint16_t index,
                               char name[128],
                               char guid[128],
                               void* user_data);
  int32_t (*RecordingDeviceName)(uint16_t index,
                                 char name[128],
                                 char guid[128],
                                 void* user_data);

  int32_t (*SetPlayoutDevice)(uint16_t index, void* user_data);
  int32_t (*SetPlayoutDeviceWithWindowsDeviceType)(int device, void* user_data);
  int32_t (*SetRecordingDevice)(uint16_t index, void* user_data);
  int32_t (*SetRecordingDeviceWithWindowsDeviceType)(int device,
                                                     void* user_data);

  int32_t (*PlayoutIsAvailable)(int* available, void* user_data);
  int32_t (*InitPlayout)(void* user_data);
  int (*PlayoutIsInitialized)(void* user_data);
  int32_t (*RecordingIsAvailable)(int* available, void* user_data);
  int32_t (*InitRecording)(void* user_data);
  int (*RecordingIsInitialized)(void* user_data);

  int32_t (*StartPlayout)(void* user_data);
  int32_t (*StopPlayout)(void* user_data);
  int (*Playing)(void* user_data);
  int32_t (*StartRecording)(void* user_data);
  int32_t (*StopRecording)(void* user_data);
  int (*Recording)(void* user_data);

  int32_t (*InitSpeaker)(void* user_data);
  int (*SpeakerIsInitialized)(void* user_data);
  int32_t (*InitMicrophone)(void* user_data);
  int (*MicrophoneIsInitialized)(void* user_data);

  int32_t (*SpeakerVolumeIsAvailable)(int* available, void* user_data);
  int32_t (*SetSpeakerVolume)(uint32_t volume, void* user_data);
  int32_t (*SpeakerVolume)(uint32_t* volume, void* user_data);
  int32_t (*MaxSpeakerVolume)(uint32_t* max_volume, void* user_data);
  int32_t (*MinSpeakerVolume)(uint32_t* min_volume, void* user_data);

  int32_t (*MicrophoneVolumeIsAvailable)(int* available, void* user_data);
  int32_t (*SetMicrophoneVolume)(uint32_t volume, void* user_data);
  int32_t (*MicrophoneVolume)(uint32_t* volume, void* user_data);
  int32_t (*MaxMicrophoneVolume)(uint32_t* max_volume, void* user_data);
  int32_t (*MinMicrophoneVolume)(uint32_t* min_volume, void* user_data);

  int32_t (*SpeakerMuteIsAvailable)(int* available, void* user_data);
  int32_t (*SetSpeakerMute)(int enable, void* user_data);
  int32_t (*SpeakerMute)(int* enabled, void* user_data);

  int32_t (*MicrophoneMuteIsAvailable)(int* available, void* user_data);
  int32_t (*SetMicrophoneMute)(int enable, void* user_data);
  int32_t (*MicrophoneMute)(int* enabled, void* user_data);

  int32_t (*StereoPlayoutIsAvailable)(int* available, void* user_data);
  int32_t (*SetStereoPlayout)(int enable, void* user_data);
  int32_t (*StereoPlayout)(int* enabled, void* user_data);

  int32_t (*StereoRecordingIsAvailable)(int* available, void* user_data);
  int32_t (*SetStereoRecording)(int enable, void* user_data);
  int32_t (*StereoRecording)(int* enabled, void* user_data);

  int32_t (*PlayoutDelay)(uint16_t* delay_ms, void* user_data);

  int (*BuiltInAECIsAvailable)(void* user_data);
  int (*BuiltInAGCIsAvailable)(void* user_data);
  int (*BuiltInNSIsAvailable)(void* user_data);

  int32_t (*EnableBuiltInAEC)(int enable, void* user_data);
  int32_t (*EnableBuiltInAGC)(int enable, void* user_data);
  int32_t (*EnableBuiltInNS)(int enable, void* user_data);

  int32_t (*GetPlayoutUnderrunCount)(void* user_data);
  int (*GetStats)(struct webrtc_AudioDeviceModule_Stats* out_stats,
                  void* user_data);

  void (*OnDestroy)(void* user_data);
};

struct webrtc_AudioDeviceModule_refcounted*
webrtc_CreateAudioDeviceModuleWithCallback(
    struct webrtc_AudioDeviceModule_cbs* cbs,
    void* user_data);

#if defined(__cplusplus)
}
#endif
