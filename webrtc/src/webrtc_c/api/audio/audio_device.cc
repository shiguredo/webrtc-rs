#include "audio_device.h"

#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>
#include <optional>

// WebRTC
#include <api/audio/audio_device.h>
#include <api/audio/audio_device_defines.h>
#include <api/audio/create_audio_device_module.h>
#include <api/environment/environment.h>
#include <api/make_ref_counted.h>
#include <api/scoped_refptr.h>

#include "../../common.impl.h"
#include "../environment.h"
#include "audio_device_defines.h"

// -------------------------
// webrtc::AudioDeviceModule
// -------------------------

namespace {

int32_t DefaultActiveAudioLayer(int* audio_layer, void* user_data) {
  return 0;
}

int32_t DefaultRegisterAudioCallback(
    struct webrtc_AudioTransport* audio_transport,
    void* user_data) {
  return 0;
}

int32_t DefaultReturnZeroI32(void* user_data) {
  return 0;
}

int DefaultReturnZeroInt(void* user_data) {
  return 0;
}

int DefaultReturnOneInt(void* user_data) {
  return 1;
}

int16_t DefaultReturnZeroI16(void* user_data) {
  return 0;
}

int32_t DefaultReturnZeroI32WithU16Arg(uint16_t value, void* user_data) {
  return 0;
}

int32_t DefaultReturnZeroI32WithIntArg(int value, void* user_data) {
  return 0;
}

int32_t DefaultReturnZeroI32WithU32Arg(uint32_t value, void* user_data) {
  return 0;
}

int32_t DefaultDeviceName(uint16_t index,
                          char name[128],
                          char guid[128],
                          void* user_data) {
  return 0;
}

int32_t DefaultReturnZeroI32WithIntOut(int* out, void* user_data) {
  return 0;
}

int32_t DefaultReturnZeroI32WithU32Out(uint32_t* out, void* user_data) {
  return 0;
}

int32_t DefaultStereoPlayoutIsAvailable(int* available, void* user_data) {
  if (available != nullptr) {
    *available = 0;
  }
  return 0;
}

int32_t DefaultStereoRecordingIsAvailable(int* available, void* user_data) {
  if (available != nullptr) {
    *available = 0;
  }
  return 0;
}

int32_t DefaultPlayoutDelay(uint16_t* delay_ms, void* user_data) {
  if (delay_ms != nullptr) {
    *delay_ms = 0;
  }
  return 0;
}

int32_t DefaultReturnMinusOneI32WithIntArg(int value, void* user_data) {
  return -1;
}

int32_t DefaultReturnMinusOneI32(void* user_data) {
  return -1;
}

int DefaultGetStats(struct webrtc_AudioDeviceModule_Stats* out_stats,
                    void* user_data) {
  return 0;
}

void DefaultOnDestroy(void* user_data) {}

int DefaultRecordingForInjection(void* user_data) {
  return 1;
}

webrtc_AudioDeviceModule_cbs MakeDefaultAudioDeviceModuleCbs() {
  webrtc_AudioDeviceModule_cbs cbs{};
  cbs.ActiveAudioLayer = DefaultActiveAudioLayer;
  cbs.RegisterAudioCallback = DefaultRegisterAudioCallback;
  cbs.Init = DefaultReturnZeroI32;
  cbs.Terminate = DefaultReturnZeroI32;
  cbs.Initialized = DefaultReturnOneInt;
  cbs.PlayoutDevices = DefaultReturnZeroI16;
  cbs.RecordingDevices = DefaultReturnZeroI16;
  cbs.PlayoutDeviceName = DefaultDeviceName;
  cbs.RecordingDeviceName = DefaultDeviceName;
  cbs.SetPlayoutDevice = DefaultReturnZeroI32WithU16Arg;
  cbs.SetPlayoutDeviceWithWindowsDeviceType = DefaultReturnZeroI32WithIntArg;
  cbs.SetRecordingDevice = DefaultReturnZeroI32WithU16Arg;
  cbs.SetRecordingDeviceWithWindowsDeviceType = DefaultReturnZeroI32WithIntArg;
  cbs.PlayoutIsAvailable = DefaultReturnZeroI32WithIntOut;
  cbs.InitPlayout = DefaultReturnZeroI32;
  cbs.PlayoutIsInitialized = DefaultReturnOneInt;
  cbs.RecordingIsAvailable = DefaultReturnZeroI32WithIntOut;
  cbs.InitRecording = DefaultReturnZeroI32;
  cbs.RecordingIsInitialized = DefaultReturnOneInt;
  cbs.StartPlayout = DefaultReturnZeroI32;
  cbs.StopPlayout = DefaultReturnZeroI32;
  cbs.Playing = DefaultReturnZeroInt;
  cbs.StartRecording = DefaultReturnZeroI32;
  cbs.StopRecording = DefaultReturnZeroI32;
  cbs.Recording = DefaultReturnZeroInt;
  cbs.InitSpeaker = DefaultReturnZeroI32;
  cbs.SpeakerIsInitialized = DefaultReturnOneInt;
  cbs.InitMicrophone = DefaultReturnZeroI32;
  cbs.MicrophoneIsInitialized = DefaultReturnOneInt;
  cbs.SpeakerVolumeIsAvailable = DefaultReturnZeroI32WithIntOut;
  cbs.SetSpeakerVolume = DefaultReturnZeroI32WithU32Arg;
  cbs.SpeakerVolume = DefaultReturnZeroI32WithU32Out;
  cbs.MaxSpeakerVolume = DefaultReturnZeroI32WithU32Out;
  cbs.MinSpeakerVolume = DefaultReturnZeroI32WithU32Out;
  cbs.MicrophoneVolumeIsAvailable = DefaultReturnZeroI32WithIntOut;
  cbs.SetMicrophoneVolume = DefaultReturnZeroI32WithU32Arg;
  cbs.MicrophoneVolume = DefaultReturnZeroI32WithU32Out;
  cbs.MaxMicrophoneVolume = DefaultReturnZeroI32WithU32Out;
  cbs.MinMicrophoneVolume = DefaultReturnZeroI32WithU32Out;
  cbs.SpeakerMuteIsAvailable = DefaultReturnZeroI32WithIntOut;
  cbs.SetSpeakerMute = DefaultReturnZeroI32WithIntArg;
  cbs.SpeakerMute = DefaultReturnZeroI32WithIntOut;
  cbs.MicrophoneMuteIsAvailable = DefaultReturnZeroI32WithIntOut;
  cbs.SetMicrophoneMute = DefaultReturnZeroI32WithIntArg;
  cbs.MicrophoneMute = DefaultReturnZeroI32WithIntOut;
  cbs.StereoPlayoutIsAvailable = DefaultStereoPlayoutIsAvailable;
  cbs.SetStereoPlayout = DefaultReturnZeroI32WithIntArg;
  cbs.StereoPlayout = DefaultReturnZeroI32WithIntOut;
  cbs.StereoRecordingIsAvailable = DefaultStereoRecordingIsAvailable;
  cbs.SetStereoRecording = DefaultReturnZeroI32WithIntArg;
  cbs.StereoRecording = DefaultReturnZeroI32WithIntOut;
  cbs.PlayoutDelay = DefaultPlayoutDelay;
  cbs.BuiltInAECIsAvailable = DefaultReturnZeroInt;
  cbs.BuiltInAGCIsAvailable = DefaultReturnZeroInt;
  cbs.BuiltInNSIsAvailable = DefaultReturnZeroInt;
  cbs.EnableBuiltInAEC = DefaultReturnMinusOneI32WithIntArg;
  cbs.EnableBuiltInAGC = DefaultReturnMinusOneI32WithIntArg;
  cbs.EnableBuiltInNS = DefaultReturnMinusOneI32WithIntArg;
  cbs.GetPlayoutUnderrunCount = DefaultReturnMinusOneI32;
  cbs.GetStats = DefaultGetStats;
  cbs.OnDestroy = DefaultOnDestroy;
  return cbs;
}

void MergeAudioDeviceModuleCbs(webrtc_AudioDeviceModule_cbs* dst,
                               const webrtc_AudioDeviceModule_cbs* src) {
  if (dst == nullptr || src == nullptr) {
    return;
  }
  if (src->ActiveAudioLayer != nullptr) {
    dst->ActiveAudioLayer = src->ActiveAudioLayer;
  }
  if (src->RegisterAudioCallback != nullptr) {
    dst->RegisterAudioCallback = src->RegisterAudioCallback;
  }
  if (src->Init != nullptr) {
    dst->Init = src->Init;
  }
  if (src->Terminate != nullptr) {
    dst->Terminate = src->Terminate;
  }
  if (src->Initialized != nullptr) {
    dst->Initialized = src->Initialized;
  }
  if (src->PlayoutDevices != nullptr) {
    dst->PlayoutDevices = src->PlayoutDevices;
  }
  if (src->RecordingDevices != nullptr) {
    dst->RecordingDevices = src->RecordingDevices;
  }
  if (src->PlayoutDeviceName != nullptr) {
    dst->PlayoutDeviceName = src->PlayoutDeviceName;
  }
  if (src->RecordingDeviceName != nullptr) {
    dst->RecordingDeviceName = src->RecordingDeviceName;
  }
  if (src->SetPlayoutDevice != nullptr) {
    dst->SetPlayoutDevice = src->SetPlayoutDevice;
  }
  if (src->SetPlayoutDeviceWithWindowsDeviceType != nullptr) {
    dst->SetPlayoutDeviceWithWindowsDeviceType =
        src->SetPlayoutDeviceWithWindowsDeviceType;
  }
  if (src->SetRecordingDevice != nullptr) {
    dst->SetRecordingDevice = src->SetRecordingDevice;
  }
  if (src->SetRecordingDeviceWithWindowsDeviceType != nullptr) {
    dst->SetRecordingDeviceWithWindowsDeviceType =
        src->SetRecordingDeviceWithWindowsDeviceType;
  }
  if (src->PlayoutIsAvailable != nullptr) {
    dst->PlayoutIsAvailable = src->PlayoutIsAvailable;
  }
  if (src->InitPlayout != nullptr) {
    dst->InitPlayout = src->InitPlayout;
  }
  if (src->PlayoutIsInitialized != nullptr) {
    dst->PlayoutIsInitialized = src->PlayoutIsInitialized;
  }
  if (src->RecordingIsAvailable != nullptr) {
    dst->RecordingIsAvailable = src->RecordingIsAvailable;
  }
  if (src->InitRecording != nullptr) {
    dst->InitRecording = src->InitRecording;
  }
  if (src->RecordingIsInitialized != nullptr) {
    dst->RecordingIsInitialized = src->RecordingIsInitialized;
  }
  if (src->StartPlayout != nullptr) {
    dst->StartPlayout = src->StartPlayout;
  }
  if (src->StopPlayout != nullptr) {
    dst->StopPlayout = src->StopPlayout;
  }
  if (src->Playing != nullptr) {
    dst->Playing = src->Playing;
  }
  if (src->StartRecording != nullptr) {
    dst->StartRecording = src->StartRecording;
  }
  if (src->StopRecording != nullptr) {
    dst->StopRecording = src->StopRecording;
  }
  if (src->Recording != nullptr) {
    dst->Recording = src->Recording;
  }
  if (src->InitSpeaker != nullptr) {
    dst->InitSpeaker = src->InitSpeaker;
  }
  if (src->SpeakerIsInitialized != nullptr) {
    dst->SpeakerIsInitialized = src->SpeakerIsInitialized;
  }
  if (src->InitMicrophone != nullptr) {
    dst->InitMicrophone = src->InitMicrophone;
  }
  if (src->MicrophoneIsInitialized != nullptr) {
    dst->MicrophoneIsInitialized = src->MicrophoneIsInitialized;
  }
  if (src->SpeakerVolumeIsAvailable != nullptr) {
    dst->SpeakerVolumeIsAvailable = src->SpeakerVolumeIsAvailable;
  }
  if (src->SetSpeakerVolume != nullptr) {
    dst->SetSpeakerVolume = src->SetSpeakerVolume;
  }
  if (src->SpeakerVolume != nullptr) {
    dst->SpeakerVolume = src->SpeakerVolume;
  }
  if (src->MaxSpeakerVolume != nullptr) {
    dst->MaxSpeakerVolume = src->MaxSpeakerVolume;
  }
  if (src->MinSpeakerVolume != nullptr) {
    dst->MinSpeakerVolume = src->MinSpeakerVolume;
  }
  if (src->MicrophoneVolumeIsAvailable != nullptr) {
    dst->MicrophoneVolumeIsAvailable = src->MicrophoneVolumeIsAvailable;
  }
  if (src->SetMicrophoneVolume != nullptr) {
    dst->SetMicrophoneVolume = src->SetMicrophoneVolume;
  }
  if (src->MicrophoneVolume != nullptr) {
    dst->MicrophoneVolume = src->MicrophoneVolume;
  }
  if (src->MaxMicrophoneVolume != nullptr) {
    dst->MaxMicrophoneVolume = src->MaxMicrophoneVolume;
  }
  if (src->MinMicrophoneVolume != nullptr) {
    dst->MinMicrophoneVolume = src->MinMicrophoneVolume;
  }
  if (src->SpeakerMuteIsAvailable != nullptr) {
    dst->SpeakerMuteIsAvailable = src->SpeakerMuteIsAvailable;
  }
  if (src->SetSpeakerMute != nullptr) {
    dst->SetSpeakerMute = src->SetSpeakerMute;
  }
  if (src->SpeakerMute != nullptr) {
    dst->SpeakerMute = src->SpeakerMute;
  }
  if (src->MicrophoneMuteIsAvailable != nullptr) {
    dst->MicrophoneMuteIsAvailable = src->MicrophoneMuteIsAvailable;
  }
  if (src->SetMicrophoneMute != nullptr) {
    dst->SetMicrophoneMute = src->SetMicrophoneMute;
  }
  if (src->MicrophoneMute != nullptr) {
    dst->MicrophoneMute = src->MicrophoneMute;
  }
  if (src->StereoPlayoutIsAvailable != nullptr) {
    dst->StereoPlayoutIsAvailable = src->StereoPlayoutIsAvailable;
  }
  if (src->SetStereoPlayout != nullptr) {
    dst->SetStereoPlayout = src->SetStereoPlayout;
  }
  if (src->StereoPlayout != nullptr) {
    dst->StereoPlayout = src->StereoPlayout;
  }
  if (src->StereoRecordingIsAvailable != nullptr) {
    dst->StereoRecordingIsAvailable = src->StereoRecordingIsAvailable;
  }
  if (src->SetStereoRecording != nullptr) {
    dst->SetStereoRecording = src->SetStereoRecording;
  }
  if (src->StereoRecording != nullptr) {
    dst->StereoRecording = src->StereoRecording;
  }
  if (src->PlayoutDelay != nullptr) {
    dst->PlayoutDelay = src->PlayoutDelay;
  }
  if (src->BuiltInAECIsAvailable != nullptr) {
    dst->BuiltInAECIsAvailable = src->BuiltInAECIsAvailable;
  }
  if (src->BuiltInAGCIsAvailable != nullptr) {
    dst->BuiltInAGCIsAvailable = src->BuiltInAGCIsAvailable;
  }
  if (src->BuiltInNSIsAvailable != nullptr) {
    dst->BuiltInNSIsAvailable = src->BuiltInNSIsAvailable;
  }
  if (src->EnableBuiltInAEC != nullptr) {
    dst->EnableBuiltInAEC = src->EnableBuiltInAEC;
  }
  if (src->EnableBuiltInAGC != nullptr) {
    dst->EnableBuiltInAGC = src->EnableBuiltInAGC;
  }
  if (src->EnableBuiltInNS != nullptr) {
    dst->EnableBuiltInNS = src->EnableBuiltInNS;
  }
  if (src->GetPlayoutUnderrunCount != nullptr) {
    dst->GetPlayoutUnderrunCount = src->GetPlayoutUnderrunCount;
  }
  if (src->GetStats != nullptr) {
    dst->GetStats = src->GetStats;
  }
  if (src->OnDestroy != nullptr) {
    dst->OnDestroy = src->OnDestroy;
  }
}

}  // namespace

class AudioDeviceModuleImpl : public webrtc::AudioDeviceModule {
 public:
  AudioDeviceModuleImpl(struct webrtc_AudioDeviceModule_cbs* cbs,
                        void* user_data)
      : user_data_(user_data) {
    InitCallbacks(cbs);
  }

  ~AudioDeviceModuleImpl() override { cbs_->OnDestroy(user_data_); }

  int32_t ActiveAudioLayer(AudioLayer* audio_layer) const override {
    int layer = static_cast<int>(webrtc::AudioDeviceModule::kDummyAudio);
    int32_t ret = cbs_->ActiveAudioLayer(&layer, user_data_);
    if (audio_layer != nullptr) {
      *audio_layer = static_cast<AudioLayer>(layer);
    }
    return ret;
  }

  int32_t RegisterAudioCallback(webrtc::AudioTransport* transport) override {
    return cbs_->RegisterAudioCallback(
        reinterpret_cast<struct webrtc_AudioTransport*>(transport), user_data_);
  }

  int32_t Init() override { return cbs_->Init(user_data_); }

  int32_t Terminate() override { return cbs_->Terminate(user_data_); }

  bool Initialized() const override {
    return cbs_->Initialized(user_data_) != 0;
  }

  int16_t PlayoutDevices() override { return cbs_->PlayoutDevices(user_data_); }

  int16_t RecordingDevices() override {
    return cbs_->RecordingDevices(user_data_);
  }

  int32_t PlayoutDeviceName(uint16_t index,
                            char name[128],
                            char guid[128]) override {
    return cbs_->PlayoutDeviceName(index, name, guid, user_data_);
  }

  int32_t RecordingDeviceName(uint16_t index,
                              char name[128],
                              char guid[128]) override {
    return cbs_->RecordingDeviceName(index, name, guid, user_data_);
  }

  int32_t SetPlayoutDevice(uint16_t index) override {
    return cbs_->SetPlayoutDevice(index, user_data_);
  }

  int32_t SetPlayoutDevice(WindowsDeviceType device) override {
    return cbs_->SetPlayoutDeviceWithWindowsDeviceType(static_cast<int>(device),
                                                       user_data_);
  }

  int32_t SetRecordingDevice(uint16_t index) override {
    return cbs_->SetRecordingDevice(index, user_data_);
  }

  int32_t SetRecordingDevice(WindowsDeviceType device) override {
    return cbs_->SetRecordingDeviceWithWindowsDeviceType(
        static_cast<int>(device), user_data_);
  }

  int32_t PlayoutIsAvailable(bool* available) override {
    int available_value = 0;
    int32_t ret = cbs_->PlayoutIsAvailable(&available_value, user_data_);
    if (available != nullptr) {
      *available = available_value != 0;
    }
    return ret;
  }

  int32_t InitPlayout() override { return cbs_->InitPlayout(user_data_); }

  bool PlayoutIsInitialized() const override {
    return cbs_->PlayoutIsInitialized(user_data_) != 0;
  }

  int32_t RecordingIsAvailable(bool* available) override {
    int available_value = 0;
    int32_t ret = cbs_->RecordingIsAvailable(&available_value, user_data_);
    if (available != nullptr) {
      *available = available_value != 0;
    }
    return ret;
  }

  int32_t InitRecording() override { return cbs_->InitRecording(user_data_); }

  bool RecordingIsInitialized() const override {
    return cbs_->RecordingIsInitialized(user_data_) != 0;
  }

  int32_t StartPlayout() override { return cbs_->StartPlayout(user_data_); }

  int32_t StopPlayout() override { return cbs_->StopPlayout(user_data_); }

  bool Playing() const override { return cbs_->Playing(user_data_) != 0; }

  int32_t StartRecording() override { return cbs_->StartRecording(user_data_); }

  int32_t StopRecording() override { return cbs_->StopRecording(user_data_); }

  bool Recording() const override { return cbs_->Recording(user_data_) != 0; }

  int32_t InitSpeaker() override { return cbs_->InitSpeaker(user_data_); }

  bool SpeakerIsInitialized() const override {
    return cbs_->SpeakerIsInitialized(user_data_) != 0;
  }

  int32_t InitMicrophone() override { return cbs_->InitMicrophone(user_data_); }

  bool MicrophoneIsInitialized() const override {
    return cbs_->MicrophoneIsInitialized(user_data_) != 0;
  }

  int32_t SpeakerVolumeIsAvailable(bool* available) override {
    int available_value = 0;
    int32_t ret = cbs_->SpeakerVolumeIsAvailable(&available_value, user_data_);
    if (available != nullptr) {
      *available = available_value != 0;
    }
    return ret;
  }

  int32_t SetSpeakerVolume(uint32_t volume) override {
    return cbs_->SetSpeakerVolume(volume, user_data_);
  }

  int32_t SpeakerVolume(uint32_t* volume) const override {
    uint32_t value = 0;
    int32_t ret = cbs_->SpeakerVolume(&value, user_data_);
    if (volume != nullptr) {
      *volume = value;
    }
    return ret;
  }

  int32_t MaxSpeakerVolume(uint32_t* max_volume) const override {
    uint32_t value = 0;
    int32_t ret = cbs_->MaxSpeakerVolume(&value, user_data_);
    if (max_volume != nullptr) {
      *max_volume = value;
    }
    return ret;
  }

  int32_t MinSpeakerVolume(uint32_t* min_volume) const override {
    uint32_t value = 0;
    int32_t ret = cbs_->MinSpeakerVolume(&value, user_data_);
    if (min_volume != nullptr) {
      *min_volume = value;
    }
    return ret;
  }

  int32_t MicrophoneVolumeIsAvailable(bool* available) override {
    int available_value = 0;
    int32_t ret =
        cbs_->MicrophoneVolumeIsAvailable(&available_value, user_data_);
    if (available != nullptr) {
      *available = available_value != 0;
    }
    return ret;
  }

  int32_t SetMicrophoneVolume(uint32_t volume) override {
    return cbs_->SetMicrophoneVolume(volume, user_data_);
  }

  int32_t MicrophoneVolume(uint32_t* volume) const override {
    uint32_t value = 0;
    int32_t ret = cbs_->MicrophoneVolume(&value, user_data_);
    if (volume != nullptr) {
      *volume = value;
    }
    return ret;
  }

  int32_t MaxMicrophoneVolume(uint32_t* max_volume) const override {
    uint32_t value = 0;
    int32_t ret = cbs_->MaxMicrophoneVolume(&value, user_data_);
    if (max_volume != nullptr) {
      *max_volume = value;
    }
    return ret;
  }

  int32_t MinMicrophoneVolume(uint32_t* min_volume) const override {
    uint32_t value = 0;
    int32_t ret = cbs_->MinMicrophoneVolume(&value, user_data_);
    if (min_volume != nullptr) {
      *min_volume = value;
    }
    return ret;
  }

  int32_t SpeakerMuteIsAvailable(bool* available) override {
    int available_value = 0;
    int32_t ret = cbs_->SpeakerMuteIsAvailable(&available_value, user_data_);
    if (available != nullptr) {
      *available = available_value != 0;
    }
    return ret;
  }

  int32_t SetSpeakerMute(bool enable) override {
    return cbs_->SetSpeakerMute(enable ? 1 : 0, user_data_);
  }

  int32_t SpeakerMute(bool* enabled) const override {
    int enabled_value = 0;
    int32_t ret = cbs_->SpeakerMute(&enabled_value, user_data_);
    if (enabled != nullptr) {
      *enabled = enabled_value != 0;
    }
    return ret;
  }

  int32_t MicrophoneMuteIsAvailable(bool* available) override {
    int available_value = 0;
    int32_t ret = cbs_->MicrophoneMuteIsAvailable(&available_value, user_data_);
    if (available != nullptr) {
      *available = available_value != 0;
    }
    return ret;
  }

  int32_t SetMicrophoneMute(bool enable) override {
    return cbs_->SetMicrophoneMute(enable ? 1 : 0, user_data_);
  }

  int32_t MicrophoneMute(bool* enabled) const override {
    int enabled_value = 0;
    int32_t ret = cbs_->MicrophoneMute(&enabled_value, user_data_);
    if (enabled != nullptr) {
      *enabled = enabled_value != 0;
    }
    return ret;
  }

  int32_t StereoPlayoutIsAvailable(bool* available) const override {
    int available_value = 0;
    int32_t ret = cbs_->StereoPlayoutIsAvailable(&available_value, user_data_);
    if (available != nullptr) {
      *available = available_value != 0;
    }
    return ret;
  }

  int32_t SetStereoPlayout(bool enable) override {
    return cbs_->SetStereoPlayout(enable ? 1 : 0, user_data_);
  }

  int32_t StereoPlayout(bool* enabled) const override {
    int enabled_value = 0;
    int32_t ret = cbs_->StereoPlayout(&enabled_value, user_data_);
    if (enabled != nullptr) {
      *enabled = enabled_value != 0;
    }
    return ret;
  }

  int32_t StereoRecordingIsAvailable(bool* available) const override {
    int available_value = 0;
    int32_t ret =
        cbs_->StereoRecordingIsAvailable(&available_value, user_data_);
    if (available != nullptr) {
      *available = available_value != 0;
    }
    return ret;
  }

  int32_t SetStereoRecording(bool enable) override {
    return cbs_->SetStereoRecording(enable ? 1 : 0, user_data_);
  }

  int32_t StereoRecording(bool* enabled) const override {
    int enabled_value = 0;
    int32_t ret = cbs_->StereoRecording(&enabled_value, user_data_);
    if (enabled != nullptr) {
      *enabled = enabled_value != 0;
    }
    return ret;
  }

  int32_t PlayoutDelay(uint16_t* delay_ms) const override {
    return cbs_->PlayoutDelay(delay_ms, user_data_);
  }

  bool BuiltInAECIsAvailable() const override {
    return cbs_->BuiltInAECIsAvailable(user_data_) != 0;
  }

  bool BuiltInAGCIsAvailable() const override {
    return cbs_->BuiltInAGCIsAvailable(user_data_) != 0;
  }

  bool BuiltInNSIsAvailable() const override {
    return cbs_->BuiltInNSIsAvailable(user_data_) != 0;
  }

  int32_t EnableBuiltInAEC(bool enable) override {
    return cbs_->EnableBuiltInAEC(enable ? 1 : 0, user_data_);
  }

  int32_t EnableBuiltInAGC(bool enable) override {
    return cbs_->EnableBuiltInAGC(enable ? 1 : 0, user_data_);
  }

  int32_t EnableBuiltInNS(bool enable) override {
    return cbs_->EnableBuiltInNS(enable ? 1 : 0, user_data_);
  }

  int32_t GetPlayoutUnderrunCount() const override {
    return cbs_->GetPlayoutUnderrunCount(user_data_);
  }

  std::optional<Stats> GetStats() const override {
    webrtc_AudioDeviceModule_Stats stats;
    if (cbs_->GetStats(&stats, user_data_) == 0) {
      return std::nullopt;
    }
    Stats out;
    out.synthesized_samples_duration_s = stats.synthesized_samples_duration_s;
    out.synthesized_samples_events = stats.synthesized_samples_events;
    out.total_samples_duration_s = stats.total_samples_duration_s;
    out.total_playout_delay_s = stats.total_playout_delay_s;
    out.total_samples_count = stats.total_samples_count;
    return out;
  }

 private:
  void InitCallbacks(struct webrtc_AudioDeviceModule_cbs* cbs) {
    cbs_storage_ = MakeDefaultAudioDeviceModuleCbs();
    MergeAudioDeviceModuleCbs(&cbs_storage_, cbs);
    cbs_ = &cbs_storage_;
  }

  struct webrtc_AudioDeviceModule_cbs* cbs_ = nullptr;
  struct webrtc_AudioDeviceModule_cbs cbs_storage_{};
  void* user_data_ = nullptr;
};

extern "C" {

extern const int webrtc_AudioDeviceModule_kPlatformDefaultAudio =
    webrtc::AudioDeviceModule::kPlatformDefaultAudio;
extern const int webrtc_AudioDeviceModule_kWindowsCoreAudio =
    webrtc::AudioDeviceModule::kWindowsCoreAudio;
extern const int webrtc_AudioDeviceModule_kWindowsCoreAudio2 =
    webrtc::AudioDeviceModule::kWindowsCoreAudio2;
extern const int webrtc_AudioDeviceModule_kLinuxAlsaAudio =
    webrtc::AudioDeviceModule::kLinuxAlsaAudio;
extern const int webrtc_AudioDeviceModule_kLinuxPulseAudio =
    webrtc::AudioDeviceModule::kLinuxPulseAudio;
extern const int webrtc_AudioDeviceModule_kAndroidJavaAudio =
    webrtc::AudioDeviceModule::kAndroidJavaAudio;
extern const int webrtc_AudioDeviceModule_kAndroidOpenSLESAudio =
    webrtc::AudioDeviceModule::kAndroidOpenSLESAudio;
extern const int
    webrtc_AudioDeviceModule_kAndroidJavaInputAndOpenSLESOutputAudio =
        webrtc::AudioDeviceModule::kAndroidJavaInputAndOpenSLESOutputAudio;
extern const int webrtc_AudioDeviceModule_kAndroidAAudioAudio =
    webrtc::AudioDeviceModule::kAndroidAAudioAudio;
extern const int
    webrtc_AudioDeviceModule_kAndroidJavaInputAndAAudioOutputAudio =
        webrtc::AudioDeviceModule::kAndroidJavaInputAndAAudioOutputAudio;
extern const int webrtc_AudioDeviceModule_kDummyAudio =
    webrtc::AudioDeviceModule::kDummyAudio;

extern const int webrtc_AudioDeviceModule_kDefaultCommunicationDevice =
    webrtc::AudioDeviceModule::kDefaultCommunicationDevice;
extern const int webrtc_AudioDeviceModule_kDefaultDevice =
    webrtc::AudioDeviceModule::kDefaultDevice;

WEBRTC_DEFINE_REFCOUNTED(webrtc_AudioDeviceModule, webrtc::AudioDeviceModule);
struct webrtc_AudioDeviceModule_refcounted* webrtc_CreateAudioDeviceModule(
    struct webrtc_Environment* env,
    int audio_type) {
  auto environment = reinterpret_cast<webrtc::Environment*>(env);
  auto adm = webrtc::CreateAudioDeviceModule(
      *environment,
      static_cast<webrtc::AudioDeviceModule::AudioLayer>(audio_type));
  return reinterpret_cast<struct webrtc_AudioDeviceModule_refcounted*>(
      adm.release());
}

int32_t webrtc_AudioDeviceModule_ActiveAudioLayer(
    struct webrtc_AudioDeviceModule* self,
    int* audio_layer) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  webrtc::AudioDeviceModule::AudioLayer layer =
      webrtc::AudioDeviceModule::kPlatformDefaultAudio;
  int32_t ret = adm->ActiveAudioLayer(&layer);
  if (audio_layer != nullptr) {
    *audio_layer = static_cast<int>(layer);
  }
  return ret;
}

int32_t webrtc_AudioDeviceModule_RegisterAudioCallback(
    struct webrtc_AudioDeviceModule* self,
    struct webrtc_AudioTransport* audio_transport) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  auto transport = reinterpret_cast<webrtc::AudioTransport*>(audio_transport);
  return adm->RegisterAudioCallback(transport);
}

int32_t webrtc_AudioDeviceModule_Init(struct webrtc_AudioDeviceModule* self) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->Init();
}

int32_t webrtc_AudioDeviceModule_Terminate(
    struct webrtc_AudioDeviceModule* self) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->Terminate();
}

int webrtc_AudioDeviceModule_Initialized(
    struct webrtc_AudioDeviceModule* self) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->Initialized() ? 1 : 0;
}

int16_t webrtc_AudioDeviceModule_PlayoutDevices(
    struct webrtc_AudioDeviceModule* self) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->PlayoutDevices();
}

int16_t webrtc_AudioDeviceModule_RecordingDevices(
    struct webrtc_AudioDeviceModule* self) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->RecordingDevices();
}

int32_t webrtc_AudioDeviceModule_PlayoutDeviceName(
    struct webrtc_AudioDeviceModule* self,
    uint16_t index,
    char name[128],
    char guid[128]) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->PlayoutDeviceName(index, name, guid);
}

int32_t webrtc_AudioDeviceModule_RecordingDeviceName(
    struct webrtc_AudioDeviceModule* self,
    uint16_t index,
    char name[128],
    char guid[128]) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->RecordingDeviceName(index, name, guid);
}

int32_t webrtc_AudioDeviceModule_SetPlayoutDevice(
    struct webrtc_AudioDeviceModule* self,
    uint16_t index) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->SetPlayoutDevice(index);
}

int32_t webrtc_AudioDeviceModule_SetPlayoutDeviceWithWindowsDeviceType(
    struct webrtc_AudioDeviceModule* self,
    int device) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->SetPlayoutDevice(
      static_cast<webrtc::AudioDeviceModule::WindowsDeviceType>(device));
}

int32_t webrtc_AudioDeviceModule_SetRecordingDevice(
    struct webrtc_AudioDeviceModule* self,
    uint16_t index) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->SetRecordingDevice(index);
}

int32_t webrtc_AudioDeviceModule_SetRecordingDeviceWithWindowsDeviceType(
    struct webrtc_AudioDeviceModule* self,
    int device) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->SetRecordingDevice(
      static_cast<webrtc::AudioDeviceModule::WindowsDeviceType>(device));
}

int32_t webrtc_AudioDeviceModule_PlayoutIsAvailable(
    struct webrtc_AudioDeviceModule* self,
    int* available) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  bool value = false;
  int32_t ret = adm->PlayoutIsAvailable(&value);
  if (available != nullptr) {
    *available = value ? 1 : 0;
  }
  return ret;
}

int32_t webrtc_AudioDeviceModule_InitPlayout(
    struct webrtc_AudioDeviceModule* self) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->InitPlayout();
}

int webrtc_AudioDeviceModule_PlayoutIsInitialized(
    struct webrtc_AudioDeviceModule* self) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->PlayoutIsInitialized() ? 1 : 0;
}

int32_t webrtc_AudioDeviceModule_RecordingIsAvailable(
    struct webrtc_AudioDeviceModule* self,
    int* available) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  bool value = false;
  int32_t ret = adm->RecordingIsAvailable(&value);
  if (available != nullptr) {
    *available = value ? 1 : 0;
  }
  return ret;
}

int32_t webrtc_AudioDeviceModule_InitRecording(
    struct webrtc_AudioDeviceModule* self) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->InitRecording();
}

int webrtc_AudioDeviceModule_RecordingIsInitialized(
    struct webrtc_AudioDeviceModule* self) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->RecordingIsInitialized() ? 1 : 0;
}

int32_t webrtc_AudioDeviceModule_StartPlayout(
    struct webrtc_AudioDeviceModule* self) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->StartPlayout();
}

int32_t webrtc_AudioDeviceModule_StopPlayout(
    struct webrtc_AudioDeviceModule* self) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->StopPlayout();
}

int webrtc_AudioDeviceModule_Playing(struct webrtc_AudioDeviceModule* self) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->Playing() ? 1 : 0;
}

int32_t webrtc_AudioDeviceModule_StartRecording(
    struct webrtc_AudioDeviceModule* self) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->StartRecording();
}

int32_t webrtc_AudioDeviceModule_StopRecording(
    struct webrtc_AudioDeviceModule* self) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->StopRecording();
}

int webrtc_AudioDeviceModule_Recording(struct webrtc_AudioDeviceModule* self) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->Recording() ? 1 : 0;
}

int32_t webrtc_AudioDeviceModule_InitSpeaker(
    struct webrtc_AudioDeviceModule* self) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->InitSpeaker();
}

int webrtc_AudioDeviceModule_SpeakerIsInitialized(
    struct webrtc_AudioDeviceModule* self) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->SpeakerIsInitialized() ? 1 : 0;
}

int32_t webrtc_AudioDeviceModule_InitMicrophone(
    struct webrtc_AudioDeviceModule* self) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->InitMicrophone();
}

int webrtc_AudioDeviceModule_MicrophoneIsInitialized(
    struct webrtc_AudioDeviceModule* self) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->MicrophoneIsInitialized() ? 1 : 0;
}

int32_t webrtc_AudioDeviceModule_SpeakerVolumeIsAvailable(
    struct webrtc_AudioDeviceModule* self,
    int* available) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  bool value = false;
  int32_t ret = adm->SpeakerVolumeIsAvailable(&value);
  if (available != nullptr) {
    *available = value ? 1 : 0;
  }
  return ret;
}

int32_t webrtc_AudioDeviceModule_SetSpeakerVolume(
    struct webrtc_AudioDeviceModule* self,
    uint32_t volume) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->SetSpeakerVolume(volume);
}

int32_t webrtc_AudioDeviceModule_SpeakerVolume(
    struct webrtc_AudioDeviceModule* self,
    uint32_t* volume) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->SpeakerVolume(volume);
}

int32_t webrtc_AudioDeviceModule_MaxSpeakerVolume(
    struct webrtc_AudioDeviceModule* self,
    uint32_t* max_volume) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->MaxSpeakerVolume(max_volume);
}

int32_t webrtc_AudioDeviceModule_MinSpeakerVolume(
    struct webrtc_AudioDeviceModule* self,
    uint32_t* min_volume) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->MinSpeakerVolume(min_volume);
}

int32_t webrtc_AudioDeviceModule_MicrophoneVolumeIsAvailable(
    struct webrtc_AudioDeviceModule* self,
    int* available) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  bool value = false;
  int32_t ret = adm->MicrophoneVolumeIsAvailable(&value);
  if (available != nullptr) {
    *available = value ? 1 : 0;
  }
  return ret;
}

int32_t webrtc_AudioDeviceModule_SetMicrophoneVolume(
    struct webrtc_AudioDeviceModule* self,
    uint32_t volume) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->SetMicrophoneVolume(volume);
}

int32_t webrtc_AudioDeviceModule_MicrophoneVolume(
    struct webrtc_AudioDeviceModule* self,
    uint32_t* volume) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->MicrophoneVolume(volume);
}

int32_t webrtc_AudioDeviceModule_MaxMicrophoneVolume(
    struct webrtc_AudioDeviceModule* self,
    uint32_t* max_volume) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->MaxMicrophoneVolume(max_volume);
}

int32_t webrtc_AudioDeviceModule_MinMicrophoneVolume(
    struct webrtc_AudioDeviceModule* self,
    uint32_t* min_volume) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->MinMicrophoneVolume(min_volume);
}

int32_t webrtc_AudioDeviceModule_SpeakerMuteIsAvailable(
    struct webrtc_AudioDeviceModule* self,
    int* available) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  bool value = false;
  int32_t ret = adm->SpeakerMuteIsAvailable(&value);
  if (available != nullptr) {
    *available = value ? 1 : 0;
  }
  return ret;
}

int32_t webrtc_AudioDeviceModule_SetSpeakerMute(
    struct webrtc_AudioDeviceModule* self,
    int enable) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->SetSpeakerMute(enable != 0);
}

int32_t webrtc_AudioDeviceModule_SpeakerMute(
    struct webrtc_AudioDeviceModule* self,
    int* enabled) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  bool value = false;
  int32_t ret = adm->SpeakerMute(&value);
  if (enabled != nullptr) {
    *enabled = value ? 1 : 0;
  }
  return ret;
}

int32_t webrtc_AudioDeviceModule_MicrophoneMuteIsAvailable(
    struct webrtc_AudioDeviceModule* self,
    int* available) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  bool value = false;
  int32_t ret = adm->MicrophoneMuteIsAvailable(&value);
  if (available != nullptr) {
    *available = value ? 1 : 0;
  }
  return ret;
}

int32_t webrtc_AudioDeviceModule_SetMicrophoneMute(
    struct webrtc_AudioDeviceModule* self,
    int enable) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->SetMicrophoneMute(enable != 0);
}

int32_t webrtc_AudioDeviceModule_MicrophoneMute(
    struct webrtc_AudioDeviceModule* self,
    int* enabled) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  bool value = false;
  int32_t ret = adm->MicrophoneMute(&value);
  if (enabled != nullptr) {
    *enabled = value ? 1 : 0;
  }
  return ret;
}

int32_t webrtc_AudioDeviceModule_StereoPlayoutIsAvailable(
    struct webrtc_AudioDeviceModule* self,
    int* available) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  bool value = false;
  int32_t ret = adm->StereoPlayoutIsAvailable(&value);
  if (available != nullptr) {
    *available = value ? 1 : 0;
  }
  return ret;
}

int32_t webrtc_AudioDeviceModule_SetStereoPlayout(
    struct webrtc_AudioDeviceModule* self,
    int enable) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->SetStereoPlayout(enable != 0);
}

int32_t webrtc_AudioDeviceModule_StereoPlayout(
    struct webrtc_AudioDeviceModule* self,
    int* enabled) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  bool value = false;
  int32_t ret = adm->StereoPlayout(&value);
  if (enabled != nullptr) {
    *enabled = value ? 1 : 0;
  }
  return ret;
}

int32_t webrtc_AudioDeviceModule_StereoRecordingIsAvailable(
    struct webrtc_AudioDeviceModule* self,
    int* available) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  bool value = false;
  int32_t ret = adm->StereoRecordingIsAvailable(&value);
  if (available != nullptr) {
    *available = value ? 1 : 0;
  }
  return ret;
}

int32_t webrtc_AudioDeviceModule_SetStereoRecording(
    struct webrtc_AudioDeviceModule* self,
    int enable) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->SetStereoRecording(enable != 0);
}

int32_t webrtc_AudioDeviceModule_StereoRecording(
    struct webrtc_AudioDeviceModule* self,
    int* enabled) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  bool value = false;
  int32_t ret = adm->StereoRecording(&value);
  if (enabled != nullptr) {
    *enabled = value ? 1 : 0;
  }
  return ret;
}

int32_t webrtc_AudioDeviceModule_PlayoutDelay(
    struct webrtc_AudioDeviceModule* self,
    uint16_t* delay_ms) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->PlayoutDelay(delay_ms);
}

int webrtc_AudioDeviceModule_BuiltInAECIsAvailable(
    struct webrtc_AudioDeviceModule* self) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->BuiltInAECIsAvailable() ? 1 : 0;
}

int webrtc_AudioDeviceModule_BuiltInAGCIsAvailable(
    struct webrtc_AudioDeviceModule* self) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->BuiltInAGCIsAvailable() ? 1 : 0;
}

int webrtc_AudioDeviceModule_BuiltInNSIsAvailable(
    struct webrtc_AudioDeviceModule* self) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->BuiltInNSIsAvailable() ? 1 : 0;
}

int32_t webrtc_AudioDeviceModule_EnableBuiltInAEC(
    struct webrtc_AudioDeviceModule* self,
    int enable) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->EnableBuiltInAEC(enable != 0);
}

int32_t webrtc_AudioDeviceModule_EnableBuiltInAGC(
    struct webrtc_AudioDeviceModule* self,
    int enable) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->EnableBuiltInAGC(enable != 0);
}

int32_t webrtc_AudioDeviceModule_EnableBuiltInNS(
    struct webrtc_AudioDeviceModule* self,
    int enable) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->EnableBuiltInNS(enable != 0);
}

int32_t webrtc_AudioDeviceModule_GetPlayoutUnderrunCount(
    struct webrtc_AudioDeviceModule* self) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  return adm->GetPlayoutUnderrunCount();
}

int webrtc_AudioDeviceModule_GetStats(
    struct webrtc_AudioDeviceModule* self,
    struct webrtc_AudioDeviceModule_Stats* out_stats) {
  auto adm = reinterpret_cast<webrtc::AudioDeviceModule*>(self);
  auto stats = adm->GetStats();
  if (!stats.has_value()) {
    return 0;
  }
  if (out_stats == nullptr) {
    return 0;
  }
  out_stats->synthesized_samples_duration_s =
      stats->synthesized_samples_duration_s;
  out_stats->synthesized_samples_events = stats->synthesized_samples_events;
  out_stats->total_samples_duration_s = stats->total_samples_duration_s;
  out_stats->total_playout_delay_s = stats->total_playout_delay_s;
  out_stats->total_samples_count = stats->total_samples_count;
  return 1;
}

struct webrtc_AudioDeviceModule_refcounted*
webrtc_CreateAudioDeviceModuleWithCallback(
    struct webrtc_AudioDeviceModule_cbs* cbs,
    void* user_data) {
  auto adm = webrtc::make_ref_counted<AudioDeviceModuleImpl>(cbs, user_data);
  return reinterpret_cast<struct webrtc_AudioDeviceModule_refcounted*>(
      adm.release());
}
}
