use crate::ref_count::{
    AudioDecoderFactoryHandle, AudioDeviceModuleHandle, AudioEncoderFactoryHandle,
    AudioTrackHandle, AudioTrackSourceHandle, MediaStreamTrackHandle,
};
use crate::{Environment, Error, MediaStreamTrack, Result, ScopedRef, ffi};
use std::ffi::c_char;
use std::os::raw::c_void;
use std::ptr::NonNull;
use std::slice;

/// webrtc::AudioDecoderFactory のラッパー。
pub struct AudioDecoderFactory {
    raw_ref: ScopedRef<AudioDecoderFactoryHandle>,
}

impl AudioDecoderFactory {
    pub fn builtin() -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_CreateBuiltinAudioDecoderFactory() })
            .expect("BUG: webrtc_CreateBuiltinAudioDecoderFactory が null を返しました");
        let raw_ref = ScopedRef::<AudioDecoderFactoryHandle>::from_raw(raw);
        Self { raw_ref }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_AudioDecoderFactory {
        self.raw_ref.as_ptr()
    }

    pub(crate) fn as_refcounted_ptr(&self) -> *mut ffi::webrtc_AudioDecoderFactory_refcounted {
        self.raw_ref.as_refcounted_ptr()
    }
}

/// webrtc::AudioEncoderFactory のラッパー。
pub struct AudioEncoderFactory {
    raw_ref: ScopedRef<AudioEncoderFactoryHandle>,
}

impl AudioEncoderFactory {
    pub fn builtin() -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_CreateBuiltinAudioEncoderFactory() })
            .expect("BUG: webrtc_CreateBuiltinAudioEncoderFactory が null を返しました");
        let raw_ref = ScopedRef::<AudioEncoderFactoryHandle>::from_raw(raw);
        Self { raw_ref }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_AudioEncoderFactory {
        self.raw_ref.as_ptr()
    }

    pub(crate) fn as_refcounted_ptr(&self) -> *mut ffi::webrtc_AudioEncoderFactory_refcounted {
        self.raw_ref.as_refcounted_ptr()
    }
}

/// webrtc::AudioDeviceModule のラッパー。
pub struct AudioDeviceModule {
    raw_ref: ScopedRef<AudioDeviceModuleHandle>,
}

impl AudioDeviceModule {
    pub fn new(env: &Environment, audio_type: AudioDeviceModuleAudioLayer) -> Result<Self> {
        let raw = NonNull::new(unsafe {
            ffi::webrtc_CreateAudioDeviceModule(env.as_ptr(), audio_type.to_int())
        })
        .ok_or(Error::NullPointer(
            "webrtc_CreateAudioDeviceModule が null を返しました",
        ))?;
        let raw_ref = ScopedRef::<AudioDeviceModuleHandle>::from_raw(raw);
        Ok(Self { raw_ref })
    }

    /// Rust 側で拡張可能な AudioDeviceModule を生成する。
    pub fn new_with_callbacks(callbacks: AudioDeviceModuleCallbacks) -> Self {
        let mut user_data = Box::new(AudioDeviceModuleUserData {
            callbacks,
            cbs: ffi::webrtc_AudioDeviceModule_cbs {
                ActiveAudioLayer: Some(adm_active_audio_layer),
                RegisterAudioCallback: Some(adm_register_audio_callback),
                Init: Some(adm_init),
                Terminate: Some(adm_terminate),
                Initialized: Some(adm_initialized),
                PlayoutDevices: Some(adm_playout_devices),
                InitRecording: Some(adm_init_recording),
                RecordingDevices: Some(adm_recording_devices),
                PlayoutDeviceName: Some(adm_playout_device_name),
                RecordingDeviceName: Some(adm_recording_device_name),
                SetPlayoutDevice: Some(adm_set_playout_device),
                SetPlayoutDeviceWithWindowsDeviceType: Some(
                    adm_set_playout_device_with_windows_device_type,
                ),
                SetRecordingDevice: Some(adm_set_recording_device),
                SetRecordingDeviceWithWindowsDeviceType: Some(
                    adm_set_recording_device_with_windows_device_type,
                ),
                PlayoutIsAvailable: Some(adm_playout_is_available),
                InitPlayout: Some(adm_init_playout),
                PlayoutIsInitialized: Some(adm_playout_is_initialized),
                RecordingIsAvailable: Some(adm_recording_is_available),
                RecordingIsInitialized: Some(adm_recording_is_initialized),
                StartPlayout: Some(adm_start_playout),
                StopPlayout: Some(adm_stop_playout),
                Playing: Some(adm_playing),
                StartRecording: Some(adm_start_recording),
                StopRecording: Some(adm_stop_recording),
                Recording: Some(adm_recording),
                InitSpeaker: Some(adm_init_speaker),
                SpeakerIsInitialized: Some(adm_speaker_is_initialized),
                InitMicrophone: Some(adm_init_microphone),
                MicrophoneIsInitialized: Some(adm_microphone_is_initialized),
                SpeakerVolumeIsAvailable: Some(adm_speaker_volume_is_available),
                SetSpeakerVolume: Some(adm_set_speaker_volume),
                SpeakerVolume: Some(adm_speaker_volume),
                MaxSpeakerVolume: Some(adm_max_speaker_volume),
                MinSpeakerVolume: Some(adm_min_speaker_volume),
                MicrophoneVolumeIsAvailable: Some(adm_microphone_volume_is_available),
                SetMicrophoneVolume: Some(adm_set_microphone_volume),
                MicrophoneVolume: Some(adm_microphone_volume),
                MaxMicrophoneVolume: Some(adm_max_microphone_volume),
                MinMicrophoneVolume: Some(adm_min_microphone_volume),
                SpeakerMuteIsAvailable: Some(adm_speaker_mute_is_available),
                SetSpeakerMute: Some(adm_set_speaker_mute),
                SpeakerMute: Some(adm_speaker_mute),
                MicrophoneMuteIsAvailable: Some(adm_microphone_mute_is_available),
                SetMicrophoneMute: Some(adm_set_microphone_mute),
                MicrophoneMute: Some(adm_microphone_mute),
                StereoPlayoutIsAvailable: Some(adm_stereo_playout_is_available),
                SetStereoPlayout: Some(adm_set_stereo_playout),
                StereoPlayout: Some(adm_stereo_playout),
                StereoRecordingIsAvailable: Some(adm_stereo_recording_is_available),
                SetStereoRecording: Some(adm_set_stereo_recording),
                StereoRecording: Some(adm_stereo_recording),
                PlayoutDelay: Some(adm_playout_delay),
                BuiltInAECIsAvailable: Some(adm_built_in_aec_is_available),
                BuiltInAGCIsAvailable: Some(adm_built_in_agc_is_available),
                BuiltInNSIsAvailable: Some(adm_built_in_ns_is_available),
                EnableBuiltInAEC: Some(adm_enable_built_in_aec),
                EnableBuiltInAGC: Some(adm_enable_built_in_agc),
                EnableBuiltInNS: Some(adm_enable_built_in_ns),
                GetPlayoutUnderrunCount: Some(adm_get_playout_underrun_count),
                GetStats: Some(adm_get_stats),
                OnDestroy: Some(adm_on_destroy),
            },
        });
        let cbs_ptr = &mut user_data.cbs as *mut ffi::webrtc_AudioDeviceModule_cbs;
        let user_data_ptr = user_data.as_mut() as *mut AudioDeviceModuleUserData as *mut c_void;
        let raw = NonNull::new(unsafe {
            ffi::webrtc_CreateAudioDeviceModuleWithCallback(cbs_ptr, user_data_ptr)
        })
        .expect("BUG: webrtc_CreateAudioDeviceModuleWithCallback が null を返しました");
        let raw_ref = ScopedRef::<AudioDeviceModuleHandle>::from_raw(raw);
        let _ = Box::into_raw(user_data);
        Self { raw_ref }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_AudioDeviceModule {
        self.raw_ref.as_ptr()
    }

    pub fn as_refcounted_ptr(&self) -> *mut ffi::webrtc_AudioDeviceModule_refcounted {
        self.raw_ref.as_refcounted_ptr()
    }

    /// ADM を初期化する。デバイス列挙・選択の前に呼び出す必要がある。
    pub fn init(&mut self) -> Result<()> {
        let ret = unsafe { ffi::webrtc_AudioDeviceModule_Init(self.as_ptr()) };
        if ret != 0 {
            return Err(Error::Message(format!(
                "AudioDeviceModule::Init が失敗しました: {}",
                ret
            )));
        }
        Ok(())
    }

    /// 録音デバイスの数を返す。
    pub fn recording_devices(&self) -> i16 {
        unsafe { ffi::webrtc_AudioDeviceModule_RecordingDevices(self.as_ptr()) }
    }

    /// 録音デバイスの名前と GUID を取得する。
    pub fn recording_device_name(&self, index: u16) -> Result<(String, String)> {
        let mut name = [0 as c_char; 128];
        let mut guid = [0 as c_char; 128];
        let ret = unsafe {
            ffi::webrtc_AudioDeviceModule_RecordingDeviceName(
                self.as_ptr(),
                index,
                name.as_mut_ptr(),
                guid.as_mut_ptr(),
            )
        };
        if ret != 0 {
            return Err(Error::Message(format!(
                "AudioDeviceModule::RecordingDeviceName が失敗しました: {}",
                ret
            )));
        }
        let name_str = unsafe { std::ffi::CStr::from_ptr(name.as_ptr()) }
            .to_str()
            .map_err(Error::Utf8Error)?
            .to_string();
        let guid_str = unsafe { std::ffi::CStr::from_ptr(guid.as_ptr()) }
            .to_str()
            .map_err(Error::Utf8Error)?
            .to_string();
        Ok((name_str, guid_str))
    }

    /// 録音デバイスを選択する。
    pub fn set_recording_device(&mut self, index: u16) -> Result<()> {
        let ret = unsafe { ffi::webrtc_AudioDeviceModule_SetRecordingDevice(self.as_ptr(), index) };
        if ret != 0 {
            return Err(Error::Message(format!(
                "AudioDeviceModule::SetRecordingDevice が失敗しました: {}",
                ret
            )));
        }
        Ok(())
    }
}

impl Clone for AudioDeviceModule {
    fn clone(&self) -> Self {
        Self {
            raw_ref: ScopedRef::clone(&self.raw_ref),
        }
    }
}

// AudioDeviceModule は生成と削除を同じスレッドで実行する必要がある（※）ため Send/Sync にはしない。
// ※同一スレッドの制約があるのは一部のプラットフォーム（iOS, Android）のみ。実装によってはロックも取っていることもある
// unsafe impl Send for AudioDeviceModule {}
// unsafe impl Sync for AudioDeviceModule {}

/// AudioDeviceModule の AudioLayer。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioDeviceModuleAudioLayer {
    PlatformDefault,
    WindowsCoreAudio,
    WindowsCoreAudio2,
    LinuxAlsaAudio,
    LinuxPulseAudio,
    AndroidJavaAudio,
    AndroidOpenSLESAudio,
    AndroidJavaInputAndOpenSLESOutputAudio,
    AndroidAAudioAudio,
    AndroidJavaInputAndAAudioOutputAudio,
    Dummy,
    Unknown(i32),
}

impl AudioDeviceModuleAudioLayer {
    fn to_int(self) -> i32 {
        match self {
            AudioDeviceModuleAudioLayer::PlatformDefault => unsafe {
                ffi::webrtc_AudioDeviceModule_kPlatformDefaultAudio
            },
            AudioDeviceModuleAudioLayer::WindowsCoreAudio => unsafe {
                ffi::webrtc_AudioDeviceModule_kWindowsCoreAudio
            },
            AudioDeviceModuleAudioLayer::WindowsCoreAudio2 => unsafe {
                ffi::webrtc_AudioDeviceModule_kWindowsCoreAudio2
            },
            AudioDeviceModuleAudioLayer::LinuxAlsaAudio => unsafe {
                ffi::webrtc_AudioDeviceModule_kLinuxAlsaAudio
            },
            AudioDeviceModuleAudioLayer::LinuxPulseAudio => unsafe {
                ffi::webrtc_AudioDeviceModule_kLinuxPulseAudio
            },
            AudioDeviceModuleAudioLayer::AndroidJavaAudio => unsafe {
                ffi::webrtc_AudioDeviceModule_kAndroidJavaAudio
            },
            AudioDeviceModuleAudioLayer::AndroidOpenSLESAudio => unsafe {
                ffi::webrtc_AudioDeviceModule_kAndroidOpenSLESAudio
            },
            AudioDeviceModuleAudioLayer::AndroidJavaInputAndOpenSLESOutputAudio => unsafe {
                ffi::webrtc_AudioDeviceModule_kAndroidJavaInputAndOpenSLESOutputAudio
            },
            AudioDeviceModuleAudioLayer::AndroidAAudioAudio => unsafe {
                ffi::webrtc_AudioDeviceModule_kAndroidAAudioAudio
            },
            AudioDeviceModuleAudioLayer::AndroidJavaInputAndAAudioOutputAudio => unsafe {
                ffi::webrtc_AudioDeviceModule_kAndroidJavaInputAndAAudioOutputAudio
            },
            AudioDeviceModuleAudioLayer::Dummy => unsafe {
                ffi::webrtc_AudioDeviceModule_kDummyAudio
            },
            AudioDeviceModuleAudioLayer::Unknown(v) => v,
        }
    }
}

/// webrtc::AudioTransport の参照ラッパー。
#[derive(Debug, Clone, Copy)]
pub struct AudioTransportRef {
    raw: NonNull<ffi::webrtc_AudioTransport>,
}

impl AudioTransportRef {
    fn from_raw(raw: *mut ffi::webrtc_AudioTransport) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_AudioTransport {
        self.raw.as_ptr()
    }

    /// # Safety
    /// `audio_samples` は `n_samples` と `n_bytes_per_sample` に応じた長さの
    /// 有効なメモリを指している必要がある。
    /// `new_mic_level` は書き込み可能なポインタである必要がある。
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn recorded_data_is_available(
        &self,
        audio_samples: *const u8,
        n_samples: usize,
        n_bytes_per_sample: usize,
        n_channels: usize,
        samples_per_sec: u32,
        total_delay_ms: u32,
        clock_drift: i32,
        current_mic_level: u32,
        key_pressed: bool,
        new_mic_level: &mut u32,
        estimated_capture_time_ns: Option<i64>,
    ) -> i32 {
        let estimated_capture_time_ns_value;
        let estimated_capture_time_ns_ptr = match estimated_capture_time_ns {
            Some(value) => {
                estimated_capture_time_ns_value = value;
                &estimated_capture_time_ns_value as *const i64
            }
            None => std::ptr::null(),
        };
        unsafe {
            ffi::webrtc_AudioTransport_RecordedDataIsAvailable(
                self.raw.as_ptr(),
                audio_samples as *const c_void,
                n_samples,
                n_bytes_per_sample,
                n_channels,
                samples_per_sec,
                total_delay_ms,
                clock_drift,
                current_mic_level,
                if key_pressed { 1 } else { 0 },
                new_mic_level as *mut u32,
                estimated_capture_time_ns_ptr,
            )
        }
    }

    /// # Safety
    /// `audio_samples` は `n_samples` と `n_bytes_per_sample` に応じた長さの
    /// 書き込み可能なメモリを指している必要がある。
    /// `n_samples_out` は書き込み可能なポインタである必要がある。
    /// `elapsed_time_ms` と `ntp_time_ms` は null または書き込み可能なポインタである必要がある。
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn need_more_play_data(
        &self,
        n_samples: usize,
        n_bytes_per_sample: usize,
        n_channels: usize,
        samples_per_sec: u32,
        audio_samples: *mut u8,
        n_samples_out: &mut usize,
        elapsed_time_ms: *mut i64,
        ntp_time_ms: *mut i64,
    ) -> i32 {
        unsafe {
            ffi::webrtc_AudioTransport_NeedMorePlayData(
                self.raw.as_ptr(),
                n_samples,
                n_bytes_per_sample,
                n_channels,
                samples_per_sec,
                audio_samples as *mut c_void,
                n_samples_out as *mut usize,
                elapsed_time_ms,
                ntp_time_ms,
            )
        }
    }

    /// # Safety
    /// `audio_data` は `number_of_frames` と `number_of_channels` と `bits_per_sample` に
    /// 応じた長さの書き込み可能なメモリを指している必要がある。
    /// `elapsed_time_ms` と `ntp_time_ms` は null または書き込み可能なポインタである必要がある。
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn pull_render_data(
        &self,
        bits_per_sample: i32,
        sample_rate: i32,
        number_of_channels: usize,
        number_of_frames: usize,
        audio_data: *mut u8,
        elapsed_time_ms: *mut i64,
        ntp_time_ms: *mut i64,
    ) {
        unsafe {
            ffi::webrtc_AudioTransport_PullRenderData(
                self.raw.as_ptr(),
                bits_per_sample,
                sample_rate,
                number_of_channels,
                number_of_frames,
                audio_data as *mut c_void,
                elapsed_time_ms,
                ntp_time_ms,
            )
        }
    }
}

unsafe impl Send for AudioTransportRef {}

/// Rust 側でカスタム実装を持てる webrtc::AudioTransport の所有型。
pub struct AudioTransport {
    raw: NonNull<ffi::webrtc_AudioTransport>,
    _cbs: Box<ffi::webrtc_AudioTransport_cbs>,
    _user_data: Box<AudioTransportCallbackState>,
}

impl AudioTransport {
    pub fn new(callbacks: AudioTransportCallbacks) -> Self {
        let mut state = Box::new(AudioTransportCallbackState { callbacks });
        let user_data = state.as_mut() as *mut AudioTransportCallbackState as *mut c_void;
        let mut cbs = Box::new(ffi::webrtc_AudioTransport_cbs {
            RecordedDataIsAvailable: Some(audio_transport_recorded_data_is_available),
            NeedMorePlayData: Some(audio_transport_need_more_play_data),
            PullRenderData: Some(audio_transport_pull_render_data),
        });
        let cbs_ptr = cbs.as_mut() as *mut ffi::webrtc_AudioTransport_cbs;
        let raw = NonNull::new(unsafe { ffi::webrtc_AudioTransport_new(cbs_ptr, user_data) })
            .expect("BUG: webrtc_AudioTransport_new が null を返しました");
        Self {
            raw,
            _cbs: cbs,
            _user_data: state,
        }
    }

    pub fn as_ref(&self) -> AudioTransportRef {
        AudioTransportRef { raw: self.raw }
    }

    /// # Safety
    /// `AudioTransportRef::recorded_data_is_available` と同じ前提条件を満たす必要がある。
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn recorded_data_is_available(
        &self,
        audio_samples: *const u8,
        n_samples: usize,
        n_bytes_per_sample: usize,
        n_channels: usize,
        samples_per_sec: u32,
        total_delay_ms: u32,
        clock_drift: i32,
        current_mic_level: u32,
        key_pressed: bool,
        new_mic_level: &mut u32,
        estimated_capture_time_ns: Option<i64>,
    ) -> i32 {
        unsafe {
            self.as_ref().recorded_data_is_available(
                audio_samples,
                n_samples,
                n_bytes_per_sample,
                n_channels,
                samples_per_sec,
                total_delay_ms,
                clock_drift,
                current_mic_level,
                key_pressed,
                new_mic_level,
                estimated_capture_time_ns,
            )
        }
    }

    /// # Safety
    /// `AudioTransportRef::need_more_play_data` と同じ前提条件を満たす必要がある。
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn need_more_play_data(
        &self,
        n_samples: usize,
        n_bytes_per_sample: usize,
        n_channels: usize,
        samples_per_sec: u32,
        audio_samples: *mut u8,
        n_samples_out: &mut usize,
        elapsed_time_ms: *mut i64,
        ntp_time_ms: *mut i64,
    ) -> i32 {
        unsafe {
            self.as_ref().need_more_play_data(
                n_samples,
                n_bytes_per_sample,
                n_channels,
                samples_per_sec,
                audio_samples,
                n_samples_out,
                elapsed_time_ms,
                ntp_time_ms,
            )
        }
    }

    /// # Safety
    /// `AudioTransportRef::pull_render_data` と同じ前提条件を満たす必要がある。
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn pull_render_data(
        &self,
        bits_per_sample: i32,
        sample_rate: i32,
        number_of_channels: usize,
        number_of_frames: usize,
        audio_data: *mut u8,
        elapsed_time_ms: *mut i64,
        ntp_time_ms: *mut i64,
    ) {
        unsafe {
            self.as_ref().pull_render_data(
                bits_per_sample,
                sample_rate,
                number_of_channels,
                number_of_frames,
                audio_data,
                elapsed_time_ms,
                ntp_time_ms,
            )
        }
    }
}

impl Drop for AudioTransport {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_AudioTransport_delete(self.raw.as_ptr()) };
    }
}

type RecordedDataIsAvailableCallback = Box<
    dyn FnMut(
            *const u8,
            usize,
            usize,
            usize,
            u32,
            u32,
            i32,
            u32,
            bool,
            &mut u32,
            Option<i64>,
        ) -> i32
        + Send
        + 'static,
>;
type NeedMorePlayDataCallback = Box<
    dyn FnMut(usize, usize, usize, u32, *mut u8, &mut usize, *mut i64, *mut i64) -> i32
        + Send
        + 'static,
>;
type PullRenderDataCallback =
    Box<dyn FnMut(i32, i32, usize, usize, *mut u8, *mut i64, *mut i64) + Send + 'static>;

#[derive(Default)]
pub struct AudioTransportCallbacks {
    pub recorded_data_is_available: Option<RecordedDataIsAvailableCallback>,
    pub need_more_play_data: Option<NeedMorePlayDataCallback>,
    pub pull_render_data: Option<PullRenderDataCallback>,
}

struct AudioTransportCallbackState {
    callbacks: AudioTransportCallbacks,
}

unsafe extern "C" fn audio_transport_recorded_data_is_available(
    audio_samples: *const c_void,
    n_samples: usize,
    n_bytes_per_sample: usize,
    n_channels: usize,
    samples_per_sec: u32,
    total_delay_ms: u32,
    clock_drift: i32,
    current_mic_level: u32,
    key_pressed: i32,
    new_mic_level: *mut u32,
    estimated_capture_time_ns: *const i64,
    user_data: *mut c_void,
) -> i32 {
    let state = unsafe { &mut *(user_data as *mut AudioTransportCallbackState) };
    let cb = match state.callbacks.recorded_data_is_available.as_mut() {
        Some(cb) => cb,
        None => return 0,
    };
    if new_mic_level.is_null() {
        return -1;
    }
    let mut new_level = unsafe { *new_mic_level };
    let estimated_capture_time_ns_value = if estimated_capture_time_ns.is_null() {
        None
    } else {
        Some(unsafe { *estimated_capture_time_ns })
    };
    let ret = cb(
        audio_samples as *const u8,
        n_samples,
        n_bytes_per_sample,
        n_channels,
        samples_per_sec,
        total_delay_ms,
        clock_drift,
        current_mic_level,
        key_pressed != 0,
        &mut new_level,
        estimated_capture_time_ns_value,
    );
    unsafe {
        *new_mic_level = new_level;
    }
    ret
}

unsafe extern "C" fn audio_transport_need_more_play_data(
    n_samples: usize,
    n_bytes_per_sample: usize,
    n_channels: usize,
    samples_per_sec: u32,
    audio_samples: *mut c_void,
    n_samples_out: *mut usize,
    elapsed_time_ms: *mut i64,
    ntp_time_ms: *mut i64,
    user_data: *mut c_void,
) -> i32 {
    let state = unsafe { &mut *(user_data as *mut AudioTransportCallbackState) };
    let cb = match state.callbacks.need_more_play_data.as_mut() {
        Some(cb) => cb,
        None => return 0,
    };
    if n_samples_out.is_null() {
        return -1;
    }
    let mut n_samples_out_value = unsafe { *n_samples_out };
    let ret = cb(
        n_samples,
        n_bytes_per_sample,
        n_channels,
        samples_per_sec,
        audio_samples as *mut u8,
        &mut n_samples_out_value,
        elapsed_time_ms,
        ntp_time_ms,
    );
    unsafe {
        *n_samples_out = n_samples_out_value;
    }
    ret
}

unsafe extern "C" fn audio_transport_pull_render_data(
    bits_per_sample: i32,
    sample_rate: i32,
    number_of_channels: usize,
    number_of_frames: usize,
    audio_data: *mut c_void,
    elapsed_time_ms: *mut i64,
    ntp_time_ms: *mut i64,
    user_data: *mut c_void,
) {
    let state = unsafe { &mut *(user_data as *mut AudioTransportCallbackState) };
    let cb = match state.callbacks.pull_render_data.as_mut() {
        Some(cb) => cb,
        None => return,
    };
    cb(
        bits_per_sample,
        sample_rate,
        number_of_channels,
        number_of_frames,
        audio_data as *mut u8,
        elapsed_time_ms,
        ntp_time_ms,
    );
}

type AudioLayerCallback = Box<dyn Fn(&mut i32) -> i32 + Send + Sync + 'static>;
type AudioTransportRegisterCallback =
    Box<dyn Fn(Option<AudioTransportRef>) -> i32 + Send + Sync + 'static>;
type NoArgI32Callback = Box<dyn Fn() -> i32 + Send + Sync + 'static>;
type NoArgBoolCallback = Box<dyn Fn() -> bool + Send + Sync + 'static>;
type NoArgI16Callback = Box<dyn Fn() -> i16 + Send + Sync + 'static>;
type BoolOutCallback = Box<dyn Fn(&mut bool) -> i32 + Send + Sync + 'static>;
type U32OutCallback = Box<dyn Fn(&mut u32) -> i32 + Send + Sync + 'static>;
type U16OutCallback = Box<dyn Fn(&mut u16) -> i32 + Send + Sync + 'static>;
type U16ArgI32Callback = Box<dyn Fn(u16) -> i32 + Send + Sync + 'static>;
type I32ArgI32Callback = Box<dyn Fn(i32) -> i32 + Send + Sync + 'static>;
type BoolArgI32Callback = Box<dyn Fn(bool) -> i32 + Send + Sync + 'static>;
type U32ArgI32Callback = Box<dyn Fn(u32) -> i32 + Send + Sync + 'static>;
type StringPairCallback = Box<dyn Fn(u16) -> Option<(String, String)> + Send + Sync + 'static>;
type StatsCallback = Box<dyn Fn() -> Option<AudioDeviceModuleStats> + Send + Sync + 'static>;

#[derive(Debug, Clone, Copy, Default)]
pub struct AudioDeviceModuleStats {
    pub synthesized_samples_duration_s: f64,
    pub synthesized_samples_events: u64,
    pub total_samples_duration_s: f64,
    pub total_playout_delay_s: f64,
    pub total_samples_count: u64,
}

/// Rust 側で拡張可能な AudioDeviceModule のコールバック群。
#[derive(Default)]
pub struct AudioDeviceModuleCallbacks {
    pub active_audio_layer: Option<AudioLayerCallback>,
    pub register_audio_callback: Option<AudioTransportRegisterCallback>,
    pub init: Option<NoArgI32Callback>,
    pub terminate: Option<NoArgI32Callback>,
    pub initialized: Option<NoArgBoolCallback>,
    pub playout_devices: Option<NoArgI16Callback>,
    pub recording_devices: Option<NoArgI16Callback>,
    pub playout_device_name: Option<StringPairCallback>,
    pub recording_device_name: Option<StringPairCallback>,
    pub set_playout_device: Option<U16ArgI32Callback>,
    pub set_playout_device_with_windows_device_type: Option<I32ArgI32Callback>,
    pub set_recording_device: Option<U16ArgI32Callback>,
    pub set_recording_device_with_windows_device_type: Option<I32ArgI32Callback>,
    pub playout_is_available: Option<BoolOutCallback>,
    pub init_playout: Option<NoArgI32Callback>,
    pub playout_is_initialized: Option<NoArgBoolCallback>,
    pub recording_is_available: Option<BoolOutCallback>,
    pub init_recording: Option<NoArgI32Callback>,
    pub recording_is_initialized: Option<NoArgBoolCallback>,
    pub start_playout: Option<NoArgI32Callback>,
    pub stop_playout: Option<NoArgI32Callback>,
    pub playing: Option<NoArgBoolCallback>,
    pub start_recording: Option<NoArgI32Callback>,
    pub stop_recording: Option<NoArgI32Callback>,
    pub recording: Option<NoArgBoolCallback>,
    pub init_speaker: Option<NoArgI32Callback>,
    pub speaker_is_initialized: Option<NoArgBoolCallback>,
    pub init_microphone: Option<NoArgI32Callback>,
    pub microphone_is_initialized: Option<NoArgBoolCallback>,
    pub speaker_volume_is_available: Option<BoolOutCallback>,
    pub set_speaker_volume: Option<U32ArgI32Callback>,
    pub speaker_volume: Option<U32OutCallback>,
    pub max_speaker_volume: Option<U32OutCallback>,
    pub min_speaker_volume: Option<U32OutCallback>,
    pub microphone_volume_is_available: Option<BoolOutCallback>,
    pub set_microphone_volume: Option<U32ArgI32Callback>,
    pub microphone_volume: Option<U32OutCallback>,
    pub max_microphone_volume: Option<U32OutCallback>,
    pub min_microphone_volume: Option<U32OutCallback>,
    pub speaker_mute_is_available: Option<BoolOutCallback>,
    pub set_speaker_mute: Option<BoolArgI32Callback>,
    pub speaker_mute: Option<BoolOutCallback>,
    pub microphone_mute_is_available: Option<BoolOutCallback>,
    pub set_microphone_mute: Option<BoolArgI32Callback>,
    pub microphone_mute: Option<BoolOutCallback>,
    pub stereo_playout_is_available: Option<BoolOutCallback>,
    pub set_stereo_playout: Option<BoolArgI32Callback>,
    pub stereo_playout: Option<BoolOutCallback>,
    pub stereo_recording_is_available: Option<BoolOutCallback>,
    pub set_stereo_recording: Option<BoolArgI32Callback>,
    pub stereo_recording: Option<BoolOutCallback>,
    pub playout_delay: Option<U16OutCallback>,
    pub built_in_aec_is_available: Option<NoArgBoolCallback>,
    pub built_in_agc_is_available: Option<NoArgBoolCallback>,
    pub built_in_ns_is_available: Option<NoArgBoolCallback>,
    pub enable_built_in_aec: Option<BoolArgI32Callback>,
    pub enable_built_in_agc: Option<BoolArgI32Callback>,
    pub enable_built_in_ns: Option<BoolArgI32Callback>,
    pub get_playout_underrun_count: Option<NoArgI32Callback>,
    pub get_stats: Option<StatsCallback>,
}

struct AudioDeviceModuleUserData {
    callbacks: AudioDeviceModuleCallbacks,
    cbs: ffi::webrtc_AudioDeviceModule_cbs,
}

fn bool_to_i32(value: bool) -> i32 {
    if value { 1 } else { 0 }
}

fn bool_from_i32(value: i32) -> bool {
    value != 0
}

fn write_c_string(dest: &mut [c_char], value: &str) {
    dest.fill(0);
    if dest.is_empty() {
        return;
    }
    let bytes = value.as_bytes();
    let len = bytes.len().min(dest.len() - 1);
    for (out, src) in dest[..len].iter_mut().zip(bytes.iter()) {
        *out = *src as c_char;
    }
}

unsafe fn adm_user_data(user_data: *mut c_void) -> &'static AudioDeviceModuleUserData {
    unsafe { &*(user_data as *const AudioDeviceModuleUserData) }
}

fn write_i32(out: *mut i32, value: i32) {
    if out.is_null() {
        return;
    }
    unsafe {
        *out = value;
    }
}

fn write_u32(out: *mut u32, value: u32) {
    if out.is_null() {
        return;
    }
    unsafe {
        *out = value;
    }
}

fn write_u16(out: *mut u16, value: u16) {
    if out.is_null() {
        return;
    }
    unsafe {
        *out = value;
    }
}

unsafe extern "C" fn adm_active_audio_layer(audio_layer: *mut i32, user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    let cb = match data.callbacks.active_audio_layer.as_ref() {
        Some(cb) => cb,
        None => {
            write_i32(audio_layer, 0);
            return 0;
        }
    };
    let mut value = if audio_layer.is_null() {
        0
    } else {
        unsafe { *audio_layer }
    };
    let ret = cb(&mut value);
    write_i32(audio_layer, value);
    ret
}

unsafe extern "C" fn adm_register_audio_callback(
    audio_transport: *mut ffi::webrtc_AudioTransport,
    user_data: *mut c_void,
) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    let cb = match data.callbacks.register_audio_callback.as_ref() {
        Some(cb) => cb,
        None => return 0,
    };
    let transport = AudioTransportRef::from_raw(audio_transport);
    cb(transport)
}

unsafe extern "C" fn adm_init(user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks.init.as_ref().map_or(0, |cb| cb())
}

unsafe extern "C" fn adm_terminate(user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks.terminate.as_ref().map_or(0, |cb| cb())
}

unsafe extern "C" fn adm_initialized(user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks
        .initialized
        .as_ref()
        .map_or(0, |cb| bool_to_i32(cb()))
}

unsafe extern "C" fn adm_playout_devices(user_data: *mut c_void) -> i16 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks.playout_devices.as_ref().map_or(0, |cb| cb())
}

unsafe extern "C" fn adm_recording_devices(user_data: *mut c_void) -> i16 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks
        .recording_devices
        .as_ref()
        .map_or(0, |cb| cb())
}

fn handle_device_name(
    cb: &StringPairCallback,
    index: u16,
    name: *mut c_char,
    guid: *mut c_char,
) -> i32 {
    let (name_value, guid_value) = match cb(index) {
        Some(value) => value,
        None => return -1,
    };
    if name.is_null() || guid.is_null() {
        return -1;
    }
    let name_buf: &mut [c_char] = unsafe { slice::from_raw_parts_mut(name, 128) };
    let guid_buf: &mut [c_char] = unsafe { slice::from_raw_parts_mut(guid, 128) };
    write_c_string(name_buf, &name_value);
    write_c_string(guid_buf, &guid_value);
    0
}

unsafe extern "C" fn adm_playout_device_name(
    index: u16,
    name: *mut c_char,
    guid: *mut c_char,
    user_data: *mut c_void,
) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    let cb = match data.callbacks.playout_device_name.as_ref() {
        Some(cb) => cb,
        None => return 0,
    };
    handle_device_name(cb, index, name, guid)
}

unsafe extern "C" fn adm_recording_device_name(
    index: u16,
    name: *mut c_char,
    guid: *mut c_char,
    user_data: *mut c_void,
) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    let cb = match data.callbacks.recording_device_name.as_ref() {
        Some(cb) => cb,
        None => return 0,
    };
    handle_device_name(cb, index, name, guid)
}

unsafe extern "C" fn adm_set_playout_device(index: u16, user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks
        .set_playout_device
        .as_ref()
        .map_or(0, |cb| cb(index))
}

unsafe extern "C" fn adm_set_playout_device_with_windows_device_type(
    device: i32,
    user_data: *mut c_void,
) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks
        .set_playout_device_with_windows_device_type
        .as_ref()
        .map_or(0, |cb| cb(device))
}

unsafe extern "C" fn adm_set_recording_device(index: u16, user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks
        .set_recording_device
        .as_ref()
        .map_or(0, |cb| cb(index))
}

unsafe extern "C" fn adm_set_recording_device_with_windows_device_type(
    device: i32,
    user_data: *mut c_void,
) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks
        .set_recording_device_with_windows_device_type
        .as_ref()
        .map_or(0, |cb| cb(device))
}

unsafe extern "C" fn adm_playout_is_available(available: *mut i32, user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    let cb = match data.callbacks.playout_is_available.as_ref() {
        Some(cb) => cb,
        None => {
            write_i32(available, 0);
            return 0;
        }
    };
    let mut value = false;
    let ret = cb(&mut value);
    write_i32(available, bool_to_i32(value));
    ret
}

unsafe extern "C" fn adm_init_playout(user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks.init_playout.as_ref().map_or(0, |cb| cb())
}

unsafe extern "C" fn adm_playout_is_initialized(user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks
        .playout_is_initialized
        .as_ref()
        .map_or(1, |cb| bool_to_i32(cb()))
}

unsafe extern "C" fn adm_recording_is_available(
    available: *mut i32,
    user_data: *mut c_void,
) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    let cb = match data.callbacks.recording_is_available.as_ref() {
        Some(cb) => cb,
        None => {
            write_i32(available, 0);
            return 0;
        }
    };
    let mut value = false;
    let ret = cb(&mut value);
    write_i32(available, bool_to_i32(value));
    ret
}

unsafe extern "C" fn adm_init_recording(user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks.init_recording.as_ref().map_or(0, |cb| cb())
}

unsafe extern "C" fn adm_recording_is_initialized(user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks
        .recording_is_initialized
        .as_ref()
        .map_or(1, |cb| bool_to_i32(cb()))
}

unsafe extern "C" fn adm_start_playout(user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks.start_playout.as_ref().map_or(0, |cb| cb())
}

unsafe extern "C" fn adm_stop_playout(user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks.stop_playout.as_ref().map_or(0, |cb| cb())
}

unsafe extern "C" fn adm_playing(user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks
        .playing
        .as_ref()
        .map_or(0, |cb| bool_to_i32(cb()))
}

unsafe extern "C" fn adm_start_recording(user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks.start_recording.as_ref().map_or(0, |cb| cb())
}

unsafe extern "C" fn adm_stop_recording(user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks.stop_recording.as_ref().map_or(0, |cb| cb())
}

unsafe extern "C" fn adm_recording(user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks
        .recording
        .as_ref()
        .map_or(0, |cb| bool_to_i32(cb()))
}

unsafe extern "C" fn adm_init_speaker(user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks.init_speaker.as_ref().map_or(0, |cb| cb())
}

unsafe extern "C" fn adm_speaker_is_initialized(user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks
        .speaker_is_initialized
        .as_ref()
        .map_or(1, |cb| bool_to_i32(cb()))
}

unsafe extern "C" fn adm_init_microphone(user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks.init_microphone.as_ref().map_or(0, |cb| cb())
}

unsafe extern "C" fn adm_microphone_is_initialized(user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks
        .microphone_is_initialized
        .as_ref()
        .map_or(1, |cb| bool_to_i32(cb()))
}

unsafe extern "C" fn adm_speaker_volume_is_available(
    available: *mut i32,
    user_data: *mut c_void,
) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    let cb = match data.callbacks.speaker_volume_is_available.as_ref() {
        Some(cb) => cb,
        None => {
            write_i32(available, 0);
            return 0;
        }
    };
    let mut value = false;
    let ret = cb(&mut value);
    write_i32(available, bool_to_i32(value));
    ret
}

unsafe extern "C" fn adm_set_speaker_volume(volume: u32, user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks
        .set_speaker_volume
        .as_ref()
        .map_or(0, |cb| cb(volume))
}

unsafe extern "C" fn adm_speaker_volume(volume: *mut u32, user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    let cb = match data.callbacks.speaker_volume.as_ref() {
        Some(cb) => cb,
        None => {
            write_u32(volume, 0);
            return 0;
        }
    };
    let mut value = 0;
    let ret = cb(&mut value);
    write_u32(volume, value);
    ret
}

unsafe extern "C" fn adm_max_speaker_volume(volume: *mut u32, user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    let cb = match data.callbacks.max_speaker_volume.as_ref() {
        Some(cb) => cb,
        None => {
            write_u32(volume, 0);
            return 0;
        }
    };
    let mut value = 0;
    let ret = cb(&mut value);
    write_u32(volume, value);
    ret
}

unsafe extern "C" fn adm_min_speaker_volume(volume: *mut u32, user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    let cb = match data.callbacks.min_speaker_volume.as_ref() {
        Some(cb) => cb,
        None => {
            write_u32(volume, 0);
            return 0;
        }
    };
    let mut value = 0;
    let ret = cb(&mut value);
    write_u32(volume, value);
    ret
}

unsafe extern "C" fn adm_microphone_volume_is_available(
    available: *mut i32,
    user_data: *mut c_void,
) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    let cb = match data.callbacks.microphone_volume_is_available.as_ref() {
        Some(cb) => cb,
        None => {
            write_i32(available, 0);
            return 0;
        }
    };
    let mut value = false;
    let ret = cb(&mut value);
    write_i32(available, bool_to_i32(value));
    ret
}

unsafe extern "C" fn adm_set_microphone_volume(volume: u32, user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks
        .set_microphone_volume
        .as_ref()
        .map_or(0, |cb| cb(volume))
}

unsafe extern "C" fn adm_microphone_volume(volume: *mut u32, user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    let cb = match data.callbacks.microphone_volume.as_ref() {
        Some(cb) => cb,
        None => {
            write_u32(volume, 0);
            return 0;
        }
    };
    let mut value = 0;
    let ret = cb(&mut value);
    write_u32(volume, value);
    ret
}

unsafe extern "C" fn adm_max_microphone_volume(volume: *mut u32, user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    let cb = match data.callbacks.max_microphone_volume.as_ref() {
        Some(cb) => cb,
        None => {
            write_u32(volume, 0);
            return 0;
        }
    };
    let mut value = 0;
    let ret = cb(&mut value);
    write_u32(volume, value);
    ret
}

unsafe extern "C" fn adm_min_microphone_volume(volume: *mut u32, user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    let cb = match data.callbacks.min_microphone_volume.as_ref() {
        Some(cb) => cb,
        None => {
            write_u32(volume, 0);
            return 0;
        }
    };
    let mut value = 0;
    let ret = cb(&mut value);
    write_u32(volume, value);
    ret
}

unsafe extern "C" fn adm_speaker_mute_is_available(
    available: *mut i32,
    user_data: *mut c_void,
) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    let cb = match data.callbacks.speaker_mute_is_available.as_ref() {
        Some(cb) => cb,
        None => {
            write_i32(available, 0);
            return 0;
        }
    };
    let mut value = false;
    let ret = cb(&mut value);
    write_i32(available, bool_to_i32(value));
    ret
}

unsafe extern "C" fn adm_set_speaker_mute(enable: i32, user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks
        .set_speaker_mute
        .as_ref()
        .map_or(0, |cb| cb(bool_from_i32(enable)))
}

unsafe extern "C" fn adm_speaker_mute(enabled: *mut i32, user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    let cb = match data.callbacks.speaker_mute.as_ref() {
        Some(cb) => cb,
        None => {
            write_i32(enabled, 0);
            return 0;
        }
    };
    let mut value = false;
    let ret = cb(&mut value);
    write_i32(enabled, bool_to_i32(value));
    ret
}

unsafe extern "C" fn adm_microphone_mute_is_available(
    available: *mut i32,
    user_data: *mut c_void,
) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    let cb = match data.callbacks.microphone_mute_is_available.as_ref() {
        Some(cb) => cb,
        None => {
            write_i32(available, 0);
            return 0;
        }
    };
    let mut value = false;
    let ret = cb(&mut value);
    write_i32(available, bool_to_i32(value));
    ret
}

unsafe extern "C" fn adm_set_microphone_mute(enable: i32, user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks
        .set_microphone_mute
        .as_ref()
        .map_or(0, |cb| cb(bool_from_i32(enable)))
}

unsafe extern "C" fn adm_microphone_mute(enabled: *mut i32, user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    let cb = match data.callbacks.microphone_mute.as_ref() {
        Some(cb) => cb,
        None => {
            write_i32(enabled, 0);
            return 0;
        }
    };
    let mut value = false;
    let ret = cb(&mut value);
    write_i32(enabled, bool_to_i32(value));
    ret
}

unsafe extern "C" fn adm_stereo_playout_is_available(
    available: *mut i32,
    user_data: *mut c_void,
) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    let cb = match data.callbacks.stereo_playout_is_available.as_ref() {
        Some(cb) => cb,
        None => {
            write_i32(available, 0);
            return 0;
        }
    };
    let mut value = false;
    let ret = cb(&mut value);
    write_i32(available, bool_to_i32(value));
    ret
}

unsafe extern "C" fn adm_set_stereo_playout(enable: i32, user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks
        .set_stereo_playout
        .as_ref()
        .map_or(0, |cb| cb(bool_from_i32(enable)))
}

unsafe extern "C" fn adm_stereo_playout(enabled: *mut i32, user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    let cb = match data.callbacks.stereo_playout.as_ref() {
        Some(cb) => cb,
        None => {
            write_i32(enabled, 0);
            return 0;
        }
    };
    let mut value = false;
    let ret = cb(&mut value);
    write_i32(enabled, bool_to_i32(value));
    ret
}

unsafe extern "C" fn adm_stereo_recording_is_available(
    available: *mut i32,
    user_data: *mut c_void,
) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    let cb = match data.callbacks.stereo_recording_is_available.as_ref() {
        Some(cb) => cb,
        None => {
            write_i32(available, 0);
            return 0;
        }
    };
    let mut value = false;
    let ret = cb(&mut value);
    write_i32(available, bool_to_i32(value));
    ret
}

unsafe extern "C" fn adm_set_stereo_recording(enable: i32, user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks
        .set_stereo_recording
        .as_ref()
        .map_or(0, |cb| cb(bool_from_i32(enable)))
}

unsafe extern "C" fn adm_stereo_recording(enabled: *mut i32, user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    let cb = match data.callbacks.stereo_recording.as_ref() {
        Some(cb) => cb,
        None => {
            write_i32(enabled, 0);
            return 0;
        }
    };
    let mut value = false;
    let ret = cb(&mut value);
    write_i32(enabled, bool_to_i32(value));
    ret
}

unsafe extern "C" fn adm_playout_delay(delay_ms: *mut u16, user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    let cb = match data.callbacks.playout_delay.as_ref() {
        Some(cb) => cb,
        None => {
            write_u16(delay_ms, 0);
            return 0;
        }
    };
    let mut value = 0u16;
    let ret = cb(&mut value);
    write_u16(delay_ms, value);
    ret
}

unsafe extern "C" fn adm_built_in_aec_is_available(user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks
        .built_in_aec_is_available
        .as_ref()
        .map_or(0, |cb| bool_to_i32(cb()))
}

unsafe extern "C" fn adm_built_in_agc_is_available(user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks
        .built_in_agc_is_available
        .as_ref()
        .map_or(0, |cb| bool_to_i32(cb()))
}

unsafe extern "C" fn adm_built_in_ns_is_available(user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks
        .built_in_ns_is_available
        .as_ref()
        .map_or(0, |cb| bool_to_i32(cb()))
}

unsafe extern "C" fn adm_enable_built_in_aec(enable: i32, user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks
        .enable_built_in_aec
        .as_ref()
        .map_or(-1, |cb| cb(bool_from_i32(enable)))
}

unsafe extern "C" fn adm_enable_built_in_agc(enable: i32, user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks
        .enable_built_in_agc
        .as_ref()
        .map_or(-1, |cb| cb(bool_from_i32(enable)))
}

unsafe extern "C" fn adm_enable_built_in_ns(enable: i32, user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks
        .enable_built_in_ns
        .as_ref()
        .map_or(-1, |cb| cb(bool_from_i32(enable)))
}

unsafe extern "C" fn adm_get_playout_underrun_count(user_data: *mut c_void) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    data.callbacks
        .get_playout_underrun_count
        .as_ref()
        .map_or(-1, |cb| cb())
}

unsafe extern "C" fn adm_get_stats(
    out_stats: *mut ffi::webrtc_AudioDeviceModule_Stats,
    user_data: *mut c_void,
) -> i32 {
    let data = unsafe { adm_user_data(user_data) };
    let cb = match data.callbacks.get_stats.as_ref() {
        Some(cb) => cb,
        None => return 0,
    };
    let stats = match cb() {
        Some(stats) => stats,
        None => return 0,
    };
    if out_stats.is_null() {
        return 0;
    }
    unsafe {
        *out_stats = ffi::webrtc_AudioDeviceModule_Stats {
            synthesized_samples_duration_s: stats.synthesized_samples_duration_s,
            synthesized_samples_events: stats.synthesized_samples_events,
            total_samples_duration_s: stats.total_samples_duration_s,
            total_playout_delay_s: stats.total_playout_delay_s,
            total_samples_count: stats.total_samples_count,
        };
    }
    1
}

unsafe extern "C" fn adm_on_destroy(user_data: *mut c_void) {
    if user_data.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(user_data as *mut AudioDeviceModuleUserData);
    }
}

/// webrtc::AudioSourceInterface のラッパー。
pub struct AudioTrackSource {
    raw_ref: ScopedRef<AudioTrackSourceHandle>,
}

impl AudioTrackSource {
    pub(crate) fn from_scoped_ref(raw_ref: ScopedRef<AudioTrackSourceHandle>) -> Self {
        Self { raw_ref }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_AudioSourceInterface {
        self.raw_ref.as_ptr()
    }

    pub fn as_refcounted_ptr(&self) -> *mut ffi::webrtc_AudioSourceInterface_refcounted {
        self.raw_ref.as_refcounted_ptr()
    }
}

unsafe impl Send for AudioTrackSource {}

/// webrtc::AudioTrackInterface のラッパー。
pub struct AudioTrack {
    raw_ref: ScopedRef<AudioTrackHandle>,
}

impl AudioTrack {
    pub(crate) fn from_scoped_ref(raw_ref: ScopedRef<AudioTrackHandle>) -> Self {
        Self { raw_ref }
    }

    /// AudioTrack を MediaStreamTrack にキャストする。
    pub fn cast_to_media_stream_track(&self) -> MediaStreamTrack {
        let raw = unsafe {
            ffi::webrtc_AudioTrackInterface_refcounted_cast_to_webrtc_MediaStreamTrackInterface(
                self.raw_ref.as_refcounted_ptr(),
            )
        };
        let raw = NonNull::new(raw)
            .expect("BUG: AudioTrackInterface から MediaStreamTrackInterface へのキャストが null を返しました");
        MediaStreamTrack::from_scoped_ref(ScopedRef::<MediaStreamTrackHandle>::from_raw(raw))
    }
}

unsafe impl Send for AudioTrack {}
// AudioTracklInterface の実体はシーケンシャルにする Proxy 経由で
// アクセスするためスレッドセーフに使用できる。
// ref: https://source.chromium.org/chromium/chromium/src/+/main:third_party/webrtc/pc/media_stream_track_proxy.h;l=26-40;drc=ef55be496e45889ace33ace4b05094ca19cb499b
unsafe impl Sync for AudioTrack {}

/// webrtc::AudioProcessingBuilderInterface のラッパー。
pub struct AudioProcessingBuilder {
    raw_unique: NonNull<ffi::webrtc_AudioProcessingBuilderInterface_unique>,
}

impl AudioProcessingBuilder {
    /// BuiltinAudioProcessingBuilder を生成する。
    pub fn new_builtin() -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_BuiltinAudioProcessingBuilder_Create() })
            .expect("webrtc_BuiltinAudioProcessingBuilder_Create が null を返しました");
        Self { raw_unique: raw }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_AudioProcessingBuilderInterface {
        unsafe { ffi::webrtc_AudioProcessingBuilderInterface_unique_get(self.raw_unique.as_ptr()) }
    }

    /// 所有権を C++ 側に移譲する。
    pub fn into_raw(self) -> *mut ffi::webrtc_AudioProcessingBuilderInterface_unique {
        std::mem::ManuallyDrop::new(self).raw_unique.as_ptr()
    }
}

impl Drop for AudioProcessingBuilder {
    fn drop(&mut self) {
        unsafe {
            ffi::webrtc_AudioProcessingBuilderInterface_unique_delete(self.raw_unique.as_ptr())
        };
    }
}
