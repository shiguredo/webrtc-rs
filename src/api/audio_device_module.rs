use crate::ref_count::AudioDeviceModuleHandle;
use crate::{Environment, Error, Result, ScopedRef, ffi};
use std::ffi::c_char;
use std::os::raw::c_void;
use std::ptr::NonNull;
use std::slice;

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
    pub fn new_with_handler(handler: Box<dyn AudioDeviceModuleHandler>) -> Self {
        let mut cbs = ffi::webrtc_AudioDeviceModule_cbs {
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
            GetPlayoutAudioParameters: Some(adm_get_playout_audio_parameters),
            GetRecordAudioParameters: Some(adm_get_record_audio_parameters),
            GetStats: Some(adm_get_stats),
            OnDestroy: Some(adm_on_destroy),
        };
        let mut state = Box::new(AudioDeviceModuleHandlerState { handler });
        let user_data_ptr = state.as_mut() as *mut AudioDeviceModuleHandlerState as *mut c_void;
        let raw = NonNull::new(unsafe {
            ffi::webrtc_CreateAudioDeviceModuleWithCallback(&mut cbs, user_data_ptr)
        })
        .expect("BUG: webrtc_CreateAudioDeviceModuleWithCallback が null を返しました");
        let raw_ref = ScopedRef::<AudioDeviceModuleHandle>::from_raw(raw);
        let _ = Box::into_raw(state);
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
}

impl AudioTransport {
    pub fn new_with_handler(handler: Box<dyn AudioTransportHandler>) -> Self {
        let state = Box::new(AudioTransportHandlerState { handler });
        let user_data = Box::into_raw(state) as *mut c_void;
        let cbs = ffi::webrtc_AudioTransport_cbs {
            RecordedDataIsAvailable: Some(audio_transport_recorded_data_is_available),
            NeedMorePlayData: Some(audio_transport_need_more_play_data),
            PullRenderData: Some(audio_transport_pull_render_data),
            OnDestroy: Some(audio_transport_on_destroy),
        };
        let raw = match NonNull::new(unsafe { ffi::webrtc_AudioTransport_new(&cbs, user_data) }) {
            Some(raw) => raw,
            None => {
                let _ = unsafe { Box::from_raw(user_data as *mut AudioTransportHandlerState) };
                panic!("BUG: webrtc_AudioTransport_new が null を返しました");
            }
        };
        Self { raw }
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

pub trait AudioTransportHandler: Send {
    #[allow(clippy::too_many_arguments)]
    #[expect(unused_variables)]
    fn recorded_data_is_available(
        &mut self,
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
        0
    }

    #[allow(clippy::too_many_arguments)]
    #[expect(unused_variables)]
    fn need_more_play_data(
        &mut self,
        n_samples: usize,
        n_bytes_per_sample: usize,
        n_channels: usize,
        samples_per_sec: u32,
        audio_samples: *mut u8,
        n_samples_out: &mut usize,
        elapsed_time_ms: *mut i64,
        ntp_time_ms: *mut i64,
    ) -> i32 {
        0
    }

    #[allow(clippy::too_many_arguments)]
    #[expect(unused_variables)]
    fn pull_render_data(
        &mut self,
        bits_per_sample: i32,
        sample_rate: i32,
        number_of_channels: usize,
        number_of_frames: usize,
        audio_data: *mut u8,
        elapsed_time_ms: *mut i64,
        ntp_time_ms: *mut i64,
    ) {
    }
}

struct AudioTransportHandlerState {
    handler: Box<dyn AudioTransportHandler>,
}

unsafe extern "C" fn audio_transport_on_destroy(user_data: *mut c_void) {
    assert!(
        !user_data.is_null(),
        "audio_transport_on_destroy: user_data is null"
    );
    let _ = unsafe { Box::from_raw(user_data as *mut AudioTransportHandlerState) };
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
    let state = unsafe { &mut *(user_data as *mut AudioTransportHandlerState) };
    if new_mic_level.is_null() {
        return -1;
    }
    let mut new_level = unsafe { *new_mic_level };
    let estimated_capture_time_ns_value = if estimated_capture_time_ns.is_null() {
        None
    } else {
        Some(unsafe { *estimated_capture_time_ns })
    };
    let ret = state.handler.recorded_data_is_available(
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
    let state = unsafe { &mut *(user_data as *mut AudioTransportHandlerState) };
    if n_samples_out.is_null() {
        return -1;
    }
    let mut n_samples_out_value = unsafe { *n_samples_out };
    let ret = state.handler.need_more_play_data(
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
    let state = unsafe { &mut *(user_data as *mut AudioTransportHandlerState) };
    state.handler.pull_render_data(
        bits_per_sample,
        sample_rate,
        number_of_channels,
        number_of_frames,
        audio_data as *mut u8,
        elapsed_time_ms,
        ntp_time_ms,
    );
}

#[derive(Debug)]
pub struct AudioParameters {
    raw: NonNull<ffi::webrtc_AudioParameters_unique>,
}

impl AudioParameters {
    pub fn new(sample_rate: i32, channels: usize, frames_per_buffer: usize) -> Self {
        let raw = NonNull::new(unsafe {
            ffi::webrtc_AudioParameters_new(sample_rate, channels, frames_per_buffer)
        })
        .expect("AudioParameters::new: webrtc_AudioParameters_new が null を返しました");
        Self { raw }
    }

    pub fn sample_rate(&self) -> i32 {
        unsafe { ffi::webrtc_AudioParameters_get_sample_rate(self.as_ptr()) }
    }

    pub fn channels(&self) -> usize {
        unsafe { ffi::webrtc_AudioParameters_get_channels(self.as_ptr()) }
    }

    pub fn frames_per_buffer(&self) -> usize {
        unsafe { ffi::webrtc_AudioParameters_get_frames_per_buffer(self.as_ptr()) }
    }

    fn as_ptr(&self) -> *mut ffi::webrtc_AudioParameters {
        unsafe { ffi::webrtc_AudioParameters_unique_get(self.raw.as_ptr()) }
    }

    fn into_raw(self) -> *mut ffi::webrtc_AudioParameters_unique {
        let raw = self.raw.as_ptr();
        std::mem::forget(self);
        raw
    }
}

impl Drop for AudioParameters {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_AudioParameters_unique_delete(self.raw.as_ptr()) };
    }
}

#[derive(Debug)]
pub struct AudioDeviceModuleStats {
    raw: NonNull<ffi::webrtc_AudioDeviceModule_Stats_unique>,
}

impl AudioDeviceModuleStats {
    pub fn new(
        synthesized_samples_duration_s: f64,
        synthesized_samples_events: u64,
        total_samples_duration_s: f64,
        total_playout_delay_s: f64,
        total_samples_count: u64,
    ) -> Self {
        let raw = NonNull::new(unsafe {
            ffi::webrtc_AudioDeviceModule_Stats_new(
                synthesized_samples_duration_s,
                synthesized_samples_events,
                total_samples_duration_s,
                total_playout_delay_s,
                total_samples_count,
            )
        })
        .expect(
            "AudioDeviceModuleStats::new: webrtc_AudioDeviceModule_Stats_new が null を返しました",
        );
        Self { raw }
    }

    pub fn synthesized_samples_duration_s(&self) -> f64 {
        unsafe {
            ffi::webrtc_AudioDeviceModule_Stats_get_synthesized_samples_duration_s(self.as_ptr())
        }
    }

    pub fn synthesized_samples_events(&self) -> u64 {
        unsafe { ffi::webrtc_AudioDeviceModule_Stats_get_synthesized_samples_events(self.as_ptr()) }
    }

    pub fn total_samples_duration_s(&self) -> f64 {
        unsafe { ffi::webrtc_AudioDeviceModule_Stats_get_total_samples_duration_s(self.as_ptr()) }
    }

    pub fn total_playout_delay_s(&self) -> f64 {
        unsafe { ffi::webrtc_AudioDeviceModule_Stats_get_total_playout_delay_s(self.as_ptr()) }
    }

    pub fn total_samples_count(&self) -> u64 {
        unsafe { ffi::webrtc_AudioDeviceModule_Stats_get_total_samples_count(self.as_ptr()) }
    }

    fn as_ptr(&self) -> *mut ffi::webrtc_AudioDeviceModule_Stats {
        unsafe { ffi::webrtc_AudioDeviceModule_Stats_unique_get(self.raw.as_ptr()) }
    }

    fn into_raw(self) -> *mut ffi::webrtc_AudioDeviceModule_Stats_unique {
        let raw = self.raw.as_ptr();
        std::mem::forget(self);
        raw
    }
}

impl Drop for AudioDeviceModuleStats {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_AudioDeviceModule_Stats_unique_delete(self.raw.as_ptr()) };
    }
}

pub trait AudioDeviceModuleHandler: Send + Sync {
    fn active_audio_layer(&self, audio_layer: &mut i32) -> i32 {
        *audio_layer = 0;
        0
    }
    #[expect(unused_variables)]
    fn register_audio_callback(&self, audio_transport: Option<AudioTransportRef>) -> i32 {
        0
    }
    fn init(&self) -> i32 {
        0
    }
    fn terminate(&self) -> i32 {
        0
    }
    fn initialized(&self) -> bool {
        false
    }
    fn playout_devices(&self) -> i16 {
        0
    }
    fn recording_devices(&self) -> i16 {
        0
    }
    #[expect(unused_variables)]
    fn playout_device_name(&self, index: u16) -> Option<(String, String)> {
        Some((String::new(), String::new()))
    }
    #[expect(unused_variables)]
    fn recording_device_name(&self, index: u16) -> Option<(String, String)> {
        Some((String::new(), String::new()))
    }
    #[expect(unused_variables)]
    fn set_playout_device(&self, index: u16) -> i32 {
        0
    }
    #[expect(unused_variables)]
    fn set_playout_device_with_windows_device_type(&self, device: i32) -> i32 {
        0
    }
    #[expect(unused_variables)]
    fn set_recording_device(&self, index: u16) -> i32 {
        0
    }
    #[expect(unused_variables)]
    fn set_recording_device_with_windows_device_type(&self, device: i32) -> i32 {
        0
    }
    fn playout_is_available(&self, available: &mut bool) -> i32 {
        *available = false;
        0
    }
    fn init_playout(&self) -> i32 {
        0
    }
    fn playout_is_initialized(&self) -> bool {
        true
    }
    fn recording_is_available(&self, available: &mut bool) -> i32 {
        *available = false;
        0
    }
    fn init_recording(&self) -> i32 {
        0
    }
    fn recording_is_initialized(&self) -> bool {
        true
    }
    fn start_playout(&self) -> i32 {
        0
    }
    fn stop_playout(&self) -> i32 {
        0
    }
    fn playing(&self) -> bool {
        false
    }
    fn start_recording(&self) -> i32 {
        0
    }
    fn stop_recording(&self) -> i32 {
        0
    }
    fn recording(&self) -> bool {
        false
    }
    fn init_speaker(&self) -> i32 {
        0
    }
    fn speaker_is_initialized(&self) -> bool {
        true
    }
    fn init_microphone(&self) -> i32 {
        0
    }
    fn microphone_is_initialized(&self) -> bool {
        true
    }
    fn speaker_volume_is_available(&self, available: &mut bool) -> i32 {
        *available = false;
        0
    }
    #[expect(unused_variables)]
    fn set_speaker_volume(&self, volume: u32) -> i32 {
        0
    }
    fn speaker_volume(&self, volume: &mut u32) -> i32 {
        *volume = 0;
        0
    }
    fn max_speaker_volume(&self, volume: &mut u32) -> i32 {
        *volume = 0;
        0
    }
    fn min_speaker_volume(&self, volume: &mut u32) -> i32 {
        *volume = 0;
        0
    }
    fn microphone_volume_is_available(&self, available: &mut bool) -> i32 {
        *available = false;
        0
    }
    #[expect(unused_variables)]
    fn set_microphone_volume(&self, volume: u32) -> i32 {
        0
    }
    fn microphone_volume(&self, volume: &mut u32) -> i32 {
        *volume = 0;
        0
    }
    fn max_microphone_volume(&self, volume: &mut u32) -> i32 {
        *volume = 0;
        0
    }
    fn min_microphone_volume(&self, volume: &mut u32) -> i32 {
        *volume = 0;
        0
    }
    fn speaker_mute_is_available(&self, available: &mut bool) -> i32 {
        *available = false;
        0
    }
    #[expect(unused_variables)]
    fn set_speaker_mute(&self, enable: bool) -> i32 {
        0
    }
    fn speaker_mute(&self, enabled: &mut bool) -> i32 {
        *enabled = false;
        0
    }
    fn microphone_mute_is_available(&self, available: &mut bool) -> i32 {
        *available = false;
        0
    }
    #[expect(unused_variables)]
    fn set_microphone_mute(&self, enable: bool) -> i32 {
        0
    }
    fn microphone_mute(&self, enabled: &mut bool) -> i32 {
        *enabled = false;
        0
    }
    fn stereo_playout_is_available(&self, available: &mut bool) -> i32 {
        *available = false;
        0
    }
    #[expect(unused_variables)]
    fn set_stereo_playout(&self, enable: bool) -> i32 {
        0
    }
    fn stereo_playout(&self, enabled: &mut bool) -> i32 {
        *enabled = false;
        0
    }
    fn stereo_recording_is_available(&self, available: &mut bool) -> i32 {
        *available = false;
        0
    }
    #[expect(unused_variables)]
    fn set_stereo_recording(&self, enable: bool) -> i32 {
        0
    }
    fn stereo_recording(&self, enabled: &mut bool) -> i32 {
        *enabled = false;
        0
    }
    fn playout_delay(&self, delay_ms: &mut u16) -> i32 {
        *delay_ms = 0;
        0
    }
    fn built_in_aec_is_available(&self) -> bool {
        false
    }
    fn built_in_agc_is_available(&self) -> bool {
        false
    }
    fn built_in_ns_is_available(&self) -> bool {
        false
    }
    #[expect(unused_variables)]
    fn enable_built_in_aec(&self, enable: bool) -> i32 {
        -1
    }
    #[expect(unused_variables)]
    fn enable_built_in_agc(&self, enable: bool) -> i32 {
        -1
    }
    #[expect(unused_variables)]
    fn enable_built_in_ns(&self, enable: bool) -> i32 {
        -1
    }
    fn get_playout_underrun_count(&self) -> i32 {
        -1
    }
    fn get_playout_audio_parameters(&self, params: &mut Option<AudioParameters>) -> i32 {
        *params = None;
        -1
    }
    fn get_record_audio_parameters(&self, params: &mut Option<AudioParameters>) -> i32 {
        *params = None;
        -1
    }
    fn get_stats(&self) -> Option<AudioDeviceModuleStats> {
        None
    }
}

struct AudioDeviceModuleHandlerState {
    handler: Box<dyn AudioDeviceModuleHandler>,
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

unsafe fn adm_state(user_data: *mut c_void) -> &'static AudioDeviceModuleHandlerState {
    unsafe { &*(user_data as *const AudioDeviceModuleHandlerState) }
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
    let state = unsafe { adm_state(user_data) };
    let mut value = if audio_layer.is_null() {
        0
    } else {
        unsafe { *audio_layer }
    };
    let ret = state.handler.active_audio_layer(&mut value);
    write_i32(audio_layer, value);
    ret
}

unsafe extern "C" fn adm_register_audio_callback(
    audio_transport: *mut ffi::webrtc_AudioTransport,
    user_data: *mut c_void,
) -> i32 {
    let state = unsafe { adm_state(user_data) };
    let transport = AudioTransportRef::from_raw(audio_transport);
    state.handler.register_audio_callback(transport)
}

unsafe extern "C" fn adm_init(user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    state.handler.init()
}

unsafe extern "C" fn adm_terminate(user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    state.handler.terminate()
}

unsafe extern "C" fn adm_initialized(user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    bool_to_i32(state.handler.initialized())
}

unsafe extern "C" fn adm_playout_devices(user_data: *mut c_void) -> i16 {
    let state = unsafe { adm_state(user_data) };
    state.handler.playout_devices()
}

unsafe extern "C" fn adm_recording_devices(user_data: *mut c_void) -> i16 {
    let state = unsafe { adm_state(user_data) };
    state.handler.recording_devices()
}

fn handle_device_name(
    device_info: Option<(String, String)>,
    name: *mut c_char,
    guid: *mut c_char,
) -> i32 {
    let (name_value, guid_value) = match device_info {
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
    let state = unsafe { adm_state(user_data) };
    handle_device_name(state.handler.playout_device_name(index), name, guid)
}

unsafe extern "C" fn adm_recording_device_name(
    index: u16,
    name: *mut c_char,
    guid: *mut c_char,
    user_data: *mut c_void,
) -> i32 {
    let state = unsafe { adm_state(user_data) };
    handle_device_name(state.handler.recording_device_name(index), name, guid)
}

unsafe extern "C" fn adm_set_playout_device(index: u16, user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    state.handler.set_playout_device(index)
}

unsafe extern "C" fn adm_set_playout_device_with_windows_device_type(
    device: i32,
    user_data: *mut c_void,
) -> i32 {
    let state = unsafe { adm_state(user_data) };
    state
        .handler
        .set_playout_device_with_windows_device_type(device)
}

unsafe extern "C" fn adm_set_recording_device(index: u16, user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    state.handler.set_recording_device(index)
}

unsafe extern "C" fn adm_set_recording_device_with_windows_device_type(
    device: i32,
    user_data: *mut c_void,
) -> i32 {
    let state = unsafe { adm_state(user_data) };
    state
        .handler
        .set_recording_device_with_windows_device_type(device)
}

unsafe extern "C" fn adm_playout_is_available(available: *mut i32, user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    let mut value = false;
    let ret = state.handler.playout_is_available(&mut value);
    write_i32(available, bool_to_i32(value));
    ret
}

unsafe extern "C" fn adm_init_playout(user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    state.handler.init_playout()
}

unsafe extern "C" fn adm_playout_is_initialized(user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    bool_to_i32(state.handler.playout_is_initialized())
}

unsafe extern "C" fn adm_recording_is_available(
    available: *mut i32,
    user_data: *mut c_void,
) -> i32 {
    let state = unsafe { adm_state(user_data) };
    let mut value = false;
    let ret = state.handler.recording_is_available(&mut value);
    write_i32(available, bool_to_i32(value));
    ret
}

unsafe extern "C" fn adm_init_recording(user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    state.handler.init_recording()
}

unsafe extern "C" fn adm_recording_is_initialized(user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    bool_to_i32(state.handler.recording_is_initialized())
}

unsafe extern "C" fn adm_start_playout(user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    state.handler.start_playout()
}

unsafe extern "C" fn adm_stop_playout(user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    state.handler.stop_playout()
}

unsafe extern "C" fn adm_playing(user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    bool_to_i32(state.handler.playing())
}

unsafe extern "C" fn adm_start_recording(user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    state.handler.start_recording()
}

unsafe extern "C" fn adm_stop_recording(user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    state.handler.stop_recording()
}

unsafe extern "C" fn adm_recording(user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    bool_to_i32(state.handler.recording())
}

unsafe extern "C" fn adm_init_speaker(user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    state.handler.init_speaker()
}

unsafe extern "C" fn adm_speaker_is_initialized(user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    bool_to_i32(state.handler.speaker_is_initialized())
}

unsafe extern "C" fn adm_init_microphone(user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    state.handler.init_microphone()
}

unsafe extern "C" fn adm_microphone_is_initialized(user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    bool_to_i32(state.handler.microphone_is_initialized())
}

unsafe extern "C" fn adm_speaker_volume_is_available(
    available: *mut i32,
    user_data: *mut c_void,
) -> i32 {
    let state = unsafe { adm_state(user_data) };
    let mut value = false;
    let ret = state.handler.speaker_volume_is_available(&mut value);
    write_i32(available, bool_to_i32(value));
    ret
}

unsafe extern "C" fn adm_set_speaker_volume(volume: u32, user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    state.handler.set_speaker_volume(volume)
}

unsafe extern "C" fn adm_speaker_volume(volume: *mut u32, user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    let mut value = 0;
    let ret = state.handler.speaker_volume(&mut value);
    write_u32(volume, value);
    ret
}

unsafe extern "C" fn adm_max_speaker_volume(volume: *mut u32, user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    let mut value = 0;
    let ret = state.handler.max_speaker_volume(&mut value);
    write_u32(volume, value);
    ret
}

unsafe extern "C" fn adm_min_speaker_volume(volume: *mut u32, user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    let mut value = 0;
    let ret = state.handler.min_speaker_volume(&mut value);
    write_u32(volume, value);
    ret
}

unsafe extern "C" fn adm_microphone_volume_is_available(
    available: *mut i32,
    user_data: *mut c_void,
) -> i32 {
    let state = unsafe { adm_state(user_data) };
    let mut value = false;
    let ret = state.handler.microphone_volume_is_available(&mut value);
    write_i32(available, bool_to_i32(value));
    ret
}

unsafe extern "C" fn adm_set_microphone_volume(volume: u32, user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    state.handler.set_microphone_volume(volume)
}

unsafe extern "C" fn adm_microphone_volume(volume: *mut u32, user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    let mut value = 0;
    let ret = state.handler.microphone_volume(&mut value);
    write_u32(volume, value);
    ret
}

unsafe extern "C" fn adm_max_microphone_volume(volume: *mut u32, user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    let mut value = 0;
    let ret = state.handler.max_microphone_volume(&mut value);
    write_u32(volume, value);
    ret
}

unsafe extern "C" fn adm_min_microphone_volume(volume: *mut u32, user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    let mut value = 0;
    let ret = state.handler.min_microphone_volume(&mut value);
    write_u32(volume, value);
    ret
}

unsafe extern "C" fn adm_speaker_mute_is_available(
    available: *mut i32,
    user_data: *mut c_void,
) -> i32 {
    let state = unsafe { adm_state(user_data) };
    let mut value = false;
    let ret = state.handler.speaker_mute_is_available(&mut value);
    write_i32(available, bool_to_i32(value));
    ret
}

unsafe extern "C" fn adm_set_speaker_mute(enable: i32, user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    state.handler.set_speaker_mute(bool_from_i32(enable))
}

unsafe extern "C" fn adm_speaker_mute(enabled: *mut i32, user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    let mut value = false;
    let ret = state.handler.speaker_mute(&mut value);
    write_i32(enabled, bool_to_i32(value));
    ret
}

unsafe extern "C" fn adm_microphone_mute_is_available(
    available: *mut i32,
    user_data: *mut c_void,
) -> i32 {
    let state = unsafe { adm_state(user_data) };
    let mut value = false;
    let ret = state.handler.microphone_mute_is_available(&mut value);
    write_i32(available, bool_to_i32(value));
    ret
}

unsafe extern "C" fn adm_set_microphone_mute(enable: i32, user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    state.handler.set_microphone_mute(bool_from_i32(enable))
}

unsafe extern "C" fn adm_microphone_mute(enabled: *mut i32, user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    let mut value = false;
    let ret = state.handler.microphone_mute(&mut value);
    write_i32(enabled, bool_to_i32(value));
    ret
}

unsafe extern "C" fn adm_stereo_playout_is_available(
    available: *mut i32,
    user_data: *mut c_void,
) -> i32 {
    let state = unsafe { adm_state(user_data) };
    let mut value = false;
    let ret = state.handler.stereo_playout_is_available(&mut value);
    write_i32(available, bool_to_i32(value));
    ret
}

unsafe extern "C" fn adm_set_stereo_playout(enable: i32, user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    state.handler.set_stereo_playout(bool_from_i32(enable))
}

unsafe extern "C" fn adm_stereo_playout(enabled: *mut i32, user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    let mut value = false;
    let ret = state.handler.stereo_playout(&mut value);
    write_i32(enabled, bool_to_i32(value));
    ret
}

unsafe extern "C" fn adm_stereo_recording_is_available(
    available: *mut i32,
    user_data: *mut c_void,
) -> i32 {
    let state = unsafe { adm_state(user_data) };
    let mut value = false;
    let ret = state.handler.stereo_recording_is_available(&mut value);
    write_i32(available, bool_to_i32(value));
    ret
}

unsafe extern "C" fn adm_set_stereo_recording(enable: i32, user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    state.handler.set_stereo_recording(bool_from_i32(enable))
}

unsafe extern "C" fn adm_stereo_recording(enabled: *mut i32, user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    let mut value = false;
    let ret = state.handler.stereo_recording(&mut value);
    write_i32(enabled, bool_to_i32(value));
    ret
}

unsafe extern "C" fn adm_playout_delay(delay_ms: *mut u16, user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    let mut value = 0u16;
    let ret = state.handler.playout_delay(&mut value);
    write_u16(delay_ms, value);
    ret
}

unsafe extern "C" fn adm_built_in_aec_is_available(user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    bool_to_i32(state.handler.built_in_aec_is_available())
}

unsafe extern "C" fn adm_built_in_agc_is_available(user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    bool_to_i32(state.handler.built_in_agc_is_available())
}

unsafe extern "C" fn adm_built_in_ns_is_available(user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    bool_to_i32(state.handler.built_in_ns_is_available())
}

unsafe extern "C" fn adm_enable_built_in_aec(enable: i32, user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    state.handler.enable_built_in_aec(bool_from_i32(enable))
}

unsafe extern "C" fn adm_enable_built_in_agc(enable: i32, user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    state.handler.enable_built_in_agc(bool_from_i32(enable))
}

unsafe extern "C" fn adm_enable_built_in_ns(enable: i32, user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    state.handler.enable_built_in_ns(bool_from_i32(enable))
}

unsafe extern "C" fn adm_get_playout_underrun_count(user_data: *mut c_void) -> i32 {
    let state = unsafe { adm_state(user_data) };
    state.handler.get_playout_underrun_count()
}

unsafe extern "C" fn adm_get_playout_audio_parameters(
    out_params: *mut *mut ffi::webrtc_AudioParameters_unique,
    user_data: *mut c_void,
) -> i32 {
    let state = unsafe { adm_state(user_data) };
    let mut params = None;
    let ret = state.handler.get_playout_audio_parameters(&mut params);
    if ret != 0 {
        return ret;
    }
    let params = match params {
        Some(params) => params,
        None => return -1,
    };
    assert!(
        !out_params.is_null(),
        "adm_get_playout_audio_parameters: out_params is null"
    );
    unsafe {
        *out_params = params.into_raw();
    }
    0
}

unsafe extern "C" fn adm_get_record_audio_parameters(
    out_params: *mut *mut ffi::webrtc_AudioParameters_unique,
    user_data: *mut c_void,
) -> i32 {
    let state = unsafe { adm_state(user_data) };
    let mut params = None;
    let ret = state.handler.get_record_audio_parameters(&mut params);
    if ret != 0 {
        return ret;
    }
    let params = match params {
        Some(params) => params,
        None => return -1,
    };
    assert!(
        !out_params.is_null(),
        "adm_get_record_audio_parameters: out_params is null"
    );
    unsafe {
        *out_params = params.into_raw();
    }
    0
}

unsafe extern "C" fn adm_get_stats(
    out_stats: *mut *mut ffi::webrtc_AudioDeviceModule_Stats_unique,
    user_data: *mut c_void,
) -> i32 {
    let state = unsafe { adm_state(user_data) };
    let stats = match state.handler.get_stats() {
        Some(stats) => stats,
        None => return 0,
    };
    assert!(!out_stats.is_null(), "adm_get_stats: out_stats is null");
    unsafe {
        *out_stats = stats.into_raw();
    }
    1
}

unsafe extern "C" fn adm_on_destroy(user_data: *mut c_void) {
    assert!(!user_data.is_null(), "adm_on_destroy: user_data is null");
    unsafe {
        let _ = Box::from_raw(user_data as *mut AudioDeviceModuleHandlerState);
    }
}
