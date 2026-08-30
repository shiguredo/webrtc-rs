use crate::helper::handler::{HandlerState, create_with_handler, destroy_handler};
use crate::helper::non_null::expect_non_null;
use crate::helper::optional::{
    get_optional, get_optional_bool, get_optional2, set_optional, set_optional_bool, set_optional2,
};
use crate::helper::ref_count::{
    AudioDecoderFactoryHandle, AudioEncoderFactoryHandle, AudioTrackHandle, AudioTrackSourceHandle,
    MediaStreamTrackHandle,
};
use crate::rtc_base::{Buffer, BufferRef, BufferS16Ref};
use crate::{
    CxxString, CxxStringRef, EnvironmentRef, Error, MapStringString, MediaStreamTrack,
    RawBufferWriter, Result, ScopedRef, ffi,
};
use std::marker::PhantomData;
use std::os::raw::c_void;
use std::ptr::NonNull;
use std::slice;

/// webrtc::AudioOptions のラッパー。
///
/// 音声トラックのノイズ系処理 (エコーキャンセラ、自動ゲインコントロール、
/// ノイズサプレッサ、ハイパスフィルタ) の有効/無効を設定する。
/// None になっている設定は WebRtcVoiceEngine のデフォルトが適用される。
pub struct AudioOptions {
    raw: NonNull<ffi::webrtc_AudioOptions>,
}

unsafe impl Send for AudioOptions {}

impl AudioOptions {
    /// 何も設定されていない AudioOptions を生成する。
    pub fn new() -> Self {
        let raw = expect_non_null(
            unsafe { ffi::webrtc_AudioOptions_new() },
            "webrtc_AudioOptions_new",
        );
        Self { raw }
    }

    /// エコーキャンセルの有効/無効を取得する。
    pub fn echo_cancellation(&self) -> Option<bool> {
        get_optional_bool(|has, value| unsafe {
            ffi::webrtc_AudioOptions_get_echo_cancellation(self.raw.as_ptr(), has, value)
        })
    }

    /// エコーキャンセルの有効/無効を設定する。
    pub fn set_echo_cancellation(&mut self, value: Option<bool>) {
        set_optional_bool(value, |has, value_ptr| unsafe {
            ffi::webrtc_AudioOptions_set_echo_cancellation(self.raw.as_ptr(), has, value_ptr)
        });
    }

    /// 自動ゲインコントロールの有効/無効を取得する。
    pub fn auto_gain_control(&self) -> Option<bool> {
        get_optional_bool(|has, value| unsafe {
            ffi::webrtc_AudioOptions_get_auto_gain_control(self.raw.as_ptr(), has, value)
        })
    }

    /// 自動ゲインコントロールの有効/無効を設定する。
    pub fn set_auto_gain_control(&mut self, value: Option<bool>) {
        set_optional_bool(value, |has, value_ptr| unsafe {
            ffi::webrtc_AudioOptions_set_auto_gain_control(self.raw.as_ptr(), has, value_ptr)
        });
    }

    /// ノイズサプレッションの有効/無効を取得する。
    pub fn noise_suppression(&self) -> Option<bool> {
        get_optional_bool(|has, value| unsafe {
            ffi::webrtc_AudioOptions_get_noise_suppression(self.raw.as_ptr(), has, value)
        })
    }

    /// ノイズサプレッションの有効/無効を設定する。
    pub fn set_noise_suppression(&mut self, value: Option<bool>) {
        set_optional_bool(value, |has, value_ptr| unsafe {
            ffi::webrtc_AudioOptions_set_noise_suppression(self.raw.as_ptr(), has, value_ptr)
        });
    }

    /// ハイパスフィルタの有効/無効を取得する。
    pub fn highpass_filter(&self) -> Option<bool> {
        get_optional_bool(|has, value| unsafe {
            ffi::webrtc_AudioOptions_get_highpass_filter(self.raw.as_ptr(), has, value)
        })
    }

    /// ハイパスフィルタの有効/無効を設定する。
    pub fn set_highpass_filter(&mut self, value: Option<bool>) {
        set_optional_bool(value, |has, value_ptr| unsafe {
            ffi::webrtc_AudioOptions_set_highpass_filter(self.raw.as_ptr(), has, value_ptr)
        });
    }

    /// 左右チャンネルの入れ替えの有効/無効を取得する。
    pub fn stereo_swapping(&self) -> Option<bool> {
        get_optional_bool(|has, value| unsafe {
            ffi::webrtc_AudioOptions_get_stereo_swapping(self.raw.as_ptr(), has, value)
        })
    }

    /// 左右チャンネルの入れ替えの有効/無効を設定する。
    pub fn set_stereo_swapping(&mut self, value: Option<bool>) {
        set_optional_bool(value, |has, value_ptr| unsafe {
            ffi::webrtc_AudioOptions_set_stereo_swapping(self.raw.as_ptr(), has, value_ptr)
        });
    }

    /// 受信側 jitter buffer (NetEq) の最大パケット数を取得する。
    pub fn audio_jitter_buffer_max_packets(&self) -> Option<i32> {
        get_optional(|has, value| unsafe {
            ffi::webrtc_AudioOptions_get_audio_jitter_buffer_max_packets(
                self.raw.as_ptr(),
                has,
                value,
            )
        })
    }

    /// 受信側 jitter buffer (NetEq) の最大パケット数を設定する。
    pub fn set_audio_jitter_buffer_max_packets(&mut self, value: Option<i32>) {
        set_optional(value, |has, value_ptr| unsafe {
            ffi::webrtc_AudioOptions_set_audio_jitter_buffer_max_packets(
                self.raw.as_ptr(),
                has,
                value_ptr,
            )
        });
    }

    /// 受信側 jitter buffer (NetEq) の fast accelerate モードの有効/無効を取得する。
    pub fn audio_jitter_buffer_fast_accelerate(&self) -> Option<bool> {
        get_optional_bool(|has, value| unsafe {
            ffi::webrtc_AudioOptions_get_audio_jitter_buffer_fast_accelerate(
                self.raw.as_ptr(),
                has,
                value,
            )
        })
    }

    /// 受信側 jitter buffer (NetEq) の fast accelerate モードの有効/無効を設定する。
    pub fn set_audio_jitter_buffer_fast_accelerate(&mut self, value: Option<bool>) {
        set_optional_bool(value, |has, value_ptr| unsafe {
            ffi::webrtc_AudioOptions_set_audio_jitter_buffer_fast_accelerate(
                self.raw.as_ptr(),
                has,
                value_ptr,
            )
        });
    }

    /// 受信側 jitter buffer (NetEq) の最小ターゲット遅延 (ミリ秒) を取得する。
    pub fn audio_jitter_buffer_min_delay_ms(&self) -> Option<i32> {
        get_optional(|has, value| unsafe {
            ffi::webrtc_AudioOptions_get_audio_jitter_buffer_min_delay_ms(
                self.raw.as_ptr(),
                has,
                value,
            )
        })
    }

    /// 受信側 jitter buffer (NetEq) の最小ターゲット遅延 (ミリ秒) を設定する。
    pub fn set_audio_jitter_buffer_min_delay_ms(&mut self, value: Option<i32>) {
        set_optional(value, |has, value_ptr| unsafe {
            ffi::webrtc_AudioOptions_set_audio_jitter_buffer_min_delay_ms(
                self.raw.as_ptr(),
                has,
                value_ptr,
            )
        });
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_AudioOptions {
        self.raw.as_ptr()
    }
}

impl Default for AudioOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for AudioOptions {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_AudioOptions_delete(self.raw.as_ptr()) };
    }
}

/// webrtc::AudioDecoderFactory のラッパー。
pub struct AudioDecoderFactory {
    raw_ref: ScopedRef<AudioDecoderFactoryHandle>,
}

unsafe impl Send for AudioDecoderFactory {}

impl AudioDecoderFactory {
    pub fn builtin() -> Self {
        let raw = expect_non_null(
            unsafe { ffi::webrtc_CreateBuiltinAudioDecoderFactory() },
            "webrtc_CreateBuiltinAudioDecoderFactory",
        );
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

unsafe impl Send for AudioEncoderFactory {}

impl AudioEncoderFactory {
    pub fn builtin() -> Self {
        let raw = expect_non_null(
            unsafe { ffi::webrtc_CreateBuiltinAudioEncoderFactory() },
            "webrtc_CreateBuiltinAudioEncoderFactory",
        );
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

/// webrtc::AudioSourceInterface のラッパー。
pub struct AudioTrackSource {
    raw_ref: ScopedRef<AudioTrackSourceHandle>,
}

unsafe impl Send for AudioTrackSource {}

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

/// webrtc::AudioTrackInterface のラッパー。
pub struct AudioTrack {
    raw_ref: ScopedRef<AudioTrackHandle>,
}

unsafe impl Send for AudioTrack {}

// AudioTrackInterface の実体はシーケンシャルにする Proxy 経由で
// アクセスするためスレッドセーフに使用できる。
// ref: https://source.chromium.org/chromium/chromium/src/+/main:third_party/webrtc/pc/media_stream_track_proxy.h;l=26-40;drc=ef55be496e45889ace33ace4b05094ca19cb499b
unsafe impl Sync for AudioTrack {}

impl AudioTrack {
    pub(crate) fn from_scoped_ref(raw_ref: ScopedRef<AudioTrackHandle>) -> Self {
        Self { raw_ref }
    }

    pub fn as_refcounted_ptr(&self) -> *mut ffi::webrtc_AudioTrackInterface_refcounted {
        self.raw_ref.as_refcounted_ptr()
    }

    /// AudioTrack を MediaStreamTrack にキャストする。
    pub fn cast_to_media_stream_track(&self) -> MediaStreamTrack {
        let raw = unsafe {
            ffi::webrtc_AudioTrackInterface_refcounted_cast_to_webrtc_MediaStreamTrackInterface(
                self.raw_ref.as_refcounted_ptr(),
            )
        };
        let raw = expect_non_null(
            raw,
            "webrtc_AudioTrackInterface_refcounted_cast_to_webrtc_MediaStreamTrackInterface",
        );
        MediaStreamTrack::from_scoped_ref(ScopedRef::<MediaStreamTrackHandle>::from_raw(raw))
    }

    /// AudioTrack に AudioTrackSink を登録する。
    ///
    /// この AudioTrack に登録した `sink` は、`remove_sink` で登録を解除するまで
    /// drop してはならない。
    pub fn add_sink(&self, sink: &AudioTrackSink) {
        unsafe {
            ffi::webrtc_AudioTrackInterface_AddSink(self.raw_ref.as_ptr(), sink.as_ptr());
        }
    }

    /// AudioTrack から AudioTrackSink を解除する。
    pub fn remove_sink(&self, sink: &AudioTrackSink) {
        unsafe {
            ffi::webrtc_AudioTrackInterface_RemoveSink(self.raw_ref.as_ptr(), sink.as_ptr());
        }
    }
}

/// 音声データを受信するためのコールバックハンドラ。
pub trait AudioTrackSinkHandler: Send {
    /// 音声データを受信した際に呼ばれる。
    fn on_data(
        &mut self,
        audio_data: &[u8],
        bits_per_sample: i32,
        sample_rate: i32,
        number_of_channels: usize,
        number_of_frames: usize,
    );
}

type AudioTrackSinkHandlerState = HandlerState<dyn AudioTrackSinkHandler>;

unsafe extern "C" fn audio_track_sink_on_data(
    audio_data: *const c_void,
    bits_per_sample: i32,
    sample_rate: i32,
    number_of_channels: usize,
    number_of_frames: usize,
    user_data: *mut c_void,
) {
    let state = unsafe { &mut *(user_data as *mut AudioTrackSinkHandlerState) };
    let byte_len = number_of_frames * number_of_channels * (bits_per_sample as usize) / 8;
    let data = if audio_data.is_null() || byte_len == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(audio_data as *const u8, byte_len) }
    };
    state.handler.on_data(
        data,
        bits_per_sample,
        sample_rate,
        number_of_channels,
        number_of_frames,
    );
}

unsafe extern "C" fn audio_track_sink_on_destroy(user_data: *mut c_void) {
    unsafe {
        destroy_handler::<AudioTrackSinkHandlerState>("audio_track_sink_on_destroy", user_data)
    };
}

/// webrtc::AudioTrackSinkInterface のラッパー。
pub struct AudioTrackSink {
    raw: NonNull<ffi::webrtc_AudioTrackSinkInterface>,
}

unsafe impl Send for AudioTrackSink {}

impl AudioTrackSink {
    pub fn new_with_handler(handler: Box<dyn AudioTrackSinkHandler>) -> Self {
        let user_data = Box::into_raw(Box::new(HandlerState::new(handler))) as *mut c_void;
        let cbs = ffi::webrtc_AudioTrackSinkInterface_cbs {
            OnData: Some(audio_track_sink_on_data),
            OnDestroy: Some(audio_track_sink_on_destroy),
        };
        let raw = unsafe {
            create_with_handler::<AudioTrackSinkHandlerState, _>(
                "webrtc_AudioTrackSinkInterface_new",
                user_data,
                |user_data| ffi::webrtc_AudioTrackSinkInterface_new(&cbs, user_data),
            )
        };
        Self { raw }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_AudioTrackSinkInterface {
        self.raw.as_ptr()
    }
}

impl Drop for AudioTrackSink {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_AudioTrackSinkInterface_delete(self.raw.as_ptr()) };
    }
}

/// webrtc::AudioProcessingBuilderInterface のラッパー。
pub struct AudioProcessingBuilder {
    raw_unique: NonNull<ffi::webrtc_AudioProcessingBuilderInterface_unique>,
}

unsafe impl Send for AudioProcessingBuilder {}

impl AudioProcessingBuilder {
    /// BuiltinAudioProcessingBuilder を生成する。
    pub fn new_builtin() -> Self {
        let raw = expect_non_null(
            unsafe { ffi::webrtc_BuiltinAudioProcessingBuilder_Create() },
            "webrtc_BuiltinAudioProcessingBuilder_Create",
        );
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

/// 音声コーデックの種類を表す列挙型。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioCodecType {
    /// 未指定のコーデック。
    Other,
    /// Opus コーデック。
    Opus,
    /// iSAC コーデック。
    Isac,
    /// G.722 コーデック。
    G722,
    /// PCMA (G.711 A-law) コーデック。
    PcmA,
    /// PCMU (G.711 mu-law) コーデック。
    PcmU,
    /// 未知のコーデック。
    Unknown(i32),
}

impl AudioCodecType {
    /// SDP コーデック名を返す。未知または未指定の場合は `None` を返す。
    pub fn as_str(self) -> Option<&'static str> {
        match self {
            Self::Other => None,
            Self::Opus => Some("opus"),
            Self::Isac => Some("ISAC"),
            Self::G722 => Some("G722"),
            Self::PcmA => Some("PCMA"),
            Self::PcmU => Some("PCMU"),
            Self::Unknown(_) => None,
        }
    }

    /// 生のコーデックタイプ値 (`webrtc::AudioEncoder::CodecType`) から構築する。
    ///
    /// 既知の値は対応するバリアントへ、それ以外の値は `Unknown` へマップする。
    pub fn from_raw(value: i32) -> Self {
        unsafe {
            if value == ffi::webrtc_AudioEncoder_CodecType_Other {
                Self::Other
            } else if value == ffi::webrtc_AudioEncoder_CodecType_Opus {
                Self::Opus
            } else if value == ffi::webrtc_AudioEncoder_CodecType_Isac {
                Self::Isac
            } else if value == ffi::webrtc_AudioEncoder_CodecType_G722 {
                Self::G722
            } else if value == ffi::webrtc_AudioEncoder_CodecType_PcmA {
                Self::PcmA
            } else if value == ffi::webrtc_AudioEncoder_CodecType_PcmU {
                Self::PcmU
            } else {
                Self::Unknown(value)
            }
        }
    }

    /// 生のコーデックタイプ値 (`webrtc::AudioEncoder::CodecType`) を返す。
    pub fn to_raw(self) -> i32 {
        unsafe {
            match self {
                Self::Other => ffi::webrtc_AudioEncoder_CodecType_Other,
                Self::Opus => ffi::webrtc_AudioEncoder_CodecType_Opus,
                Self::Isac => ffi::webrtc_AudioEncoder_CodecType_Isac,
                Self::G722 => ffi::webrtc_AudioEncoder_CodecType_G722,
                Self::PcmA => ffi::webrtc_AudioEncoder_CodecType_PcmA,
                Self::PcmU => ffi::webrtc_AudioEncoder_CodecType_PcmU,
                Self::Unknown(value) => value,
            }
        }
    }
}

impl TryFrom<&str> for AudioCodecType {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "opus" => Ok(Self::Opus),
            "ISAC" => Ok(Self::Isac),
            "G722" => Ok(Self::G722),
            "PCMA" => Ok(Self::PcmA),
            "PCMU" => Ok(Self::PcmU),
            _ => Err(Error::InvalidAudioCodecType(value.to_string())),
        }
    }
}

impl std::str::FromStr for AudioCodecType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::try_from(s)
    }
}

/// webrtc::SdpAudioFormat のラッパー。
pub struct SdpAudioFormat {
    raw_unique: NonNull<ffi::webrtc_SdpAudioFormat_unique>,
}

unsafe impl Send for SdpAudioFormat {}

impl SdpAudioFormat {
    /// 新しい SDP 音声フォーマットを生成する。
    pub fn new(name: &str, clockrate_hz: i32, num_channels: usize) -> Self {
        let raw = unsafe {
            ffi::webrtc_SdpAudioFormat_new(
                name.as_ptr() as *const _,
                name.len(),
                clockrate_hz,
                num_channels,
            )
        };
        Self {
            raw_unique: expect_non_null(raw, "webrtc_SdpAudioFormat_new"),
        }
    }

    /// パラメータ付きで新しい SDP 音声フォーマットを生成する。
    pub fn new_with_parameters(
        name: &str,
        clockrate_hz: i32,
        num_channels: usize,
        parameters: &MapStringString<'_>,
    ) -> Self {
        let raw = unsafe {
            ffi::webrtc_SdpAudioFormat_new_with_parameters(
                name.as_ptr() as *const _,
                name.len(),
                clockrate_hz,
                num_channels,
                parameters.raw(),
            )
        };
        Self {
            raw_unique: expect_non_null(raw, "webrtc_SdpAudioFormat_new_with_parameters"),
        }
    }

    /// SDP コーデック名を返す。
    pub fn name(&self) -> Result<String> {
        self.as_ref().name()
    }

    /// SDP コーデック名を設定する。
    pub fn set_name(&mut self, name: &str) {
        let name = CxxString::from_str(name);
        unsafe {
            ffi::webrtc_SdpAudioFormat_set_name(self.raw().as_ptr(), name.as_ptr());
        }
    }

    /// クロックレート (Hz) を返す。
    pub fn clockrate_hz(&self) -> i32 {
        self.as_ref().clockrate_hz()
    }

    /// クロックレート (Hz) を設定する。
    pub fn set_clockrate_hz(&mut self, value: i32) {
        unsafe { ffi::webrtc_SdpAudioFormat_set_clockrate_hz(self.raw().as_ptr(), value) }
    }

    /// チャンネル数を返す。
    pub fn num_channels(&self) -> usize {
        self.as_ref().num_channels()
    }

    /// チャンネル数を設定する。
    pub fn set_num_channels(&mut self, value: usize) {
        unsafe { ffi::webrtc_SdpAudioFormat_set_num_channels(self.raw().as_ptr(), value) }
    }

    /// コーデックパラメータへの可変参照を返す。
    pub fn parameters_mut(&mut self) -> MapStringString<'_> {
        let ptr = unsafe { ffi::webrtc_SdpAudioFormat_get_parameters(self.raw().as_ptr()) };
        MapStringString::from_raw(expect_non_null(ptr, "webrtc_SdpAudioFormat_get_parameters"))
    }

    /// コーデックパラメータを設定する。
    pub fn set_parameters(&mut self, parameters: &MapStringString<'_>) {
        unsafe { ffi::webrtc_SdpAudioFormat_set_parameters(self.raw().as_ptr(), parameters.raw()) }
    }

    /// 等価かどうかを返す。
    pub fn is_equal(&self, other: SdpAudioFormatRef<'_>) -> bool {
        unsafe { ffi::webrtc_SdpAudioFormat_is_equal(self.raw().as_ptr(), other.raw.as_ptr()) != 0 }
    }

    /// コーデックがマッチするかどうかを返す。
    pub fn matches(&self, other: SdpAudioFormatRef<'_>) -> bool {
        unsafe { ffi::webrtc_SdpAudioFormat_Matches(self.raw().as_ptr(), other.raw.as_ptr()) != 0 }
    }

    pub fn as_ref(&self) -> SdpAudioFormatRef<'_> {
        // Safety: self.raw() は SdpAudioFormat の生存中は常に有効です。
        unsafe { SdpAudioFormatRef::from_raw(self.raw()) }
    }

    pub(crate) fn raw(&self) -> NonNull<ffi::webrtc_SdpAudioFormat> {
        let raw = unsafe { ffi::webrtc_SdpAudioFormat_unique_get(self.raw_unique.as_ptr()) };
        expect_non_null(raw, "webrtc_SdpAudioFormat_unique_get")
    }
}

impl Clone for SdpAudioFormat {
    fn clone(&self) -> Self {
        let raw = unsafe { ffi::webrtc_SdpAudioFormat_copy(self.raw().as_ptr()) };
        Self {
            raw_unique: expect_non_null(raw, "webrtc_SdpAudioFormat_copy"),
        }
    }
}

impl Drop for SdpAudioFormat {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_SdpAudioFormat_unique_delete(self.raw_unique.as_ptr()) };
    }
}

/// webrtc::SdpAudioFormat への借用ラッパー。
pub struct SdpAudioFormatRef<'a> {
    raw: NonNull<ffi::webrtc_SdpAudioFormat>,
    _marker: PhantomData<&'a ffi::webrtc_SdpAudioFormat>,
}

unsafe impl<'a> Send for SdpAudioFormatRef<'a> {}

impl<'a> SdpAudioFormatRef<'a> {
    /// # Safety
    /// `raw` は有効な `webrtc_SdpAudioFormat` を指している必要があります。
    pub unsafe fn from_raw(raw: NonNull<ffi::webrtc_SdpAudioFormat>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    /// SDP コーデック名を返す。
    pub fn name(&self) -> Result<String> {
        let ptr = unsafe { ffi::webrtc_SdpAudioFormat_get_name(self.raw.as_ptr()) };
        CxxStringRef::from_ptr(expect_non_null(ptr, "webrtc_SdpAudioFormat_get_name")).to_string()
    }

    /// クロックレート (Hz) を返す。
    pub fn clockrate_hz(&self) -> i32 {
        unsafe { ffi::webrtc_SdpAudioFormat_get_clockrate_hz(self.raw.as_ptr()) }
    }

    /// チャンネル数を返す。
    pub fn num_channels(&self) -> usize {
        unsafe { ffi::webrtc_SdpAudioFormat_get_num_channels(self.raw.as_ptr()) }
    }

    /// コーデックパラメータへの可変参照を返す。
    pub fn parameters_mut(&mut self) -> MapStringString<'_> {
        let ptr = unsafe { ffi::webrtc_SdpAudioFormat_get_parameters(self.raw.as_ptr()) };
        MapStringString::from_raw(expect_non_null(ptr, "webrtc_SdpAudioFormat_get_parameters"))
    }

    pub(crate) fn as_ptr(&self) -> *mut ffi::webrtc_SdpAudioFormat {
        self.raw.as_ptr()
    }

    /// コピーを生成する。
    pub fn to_owned(&self) -> SdpAudioFormat {
        let raw = unsafe { ffi::webrtc_SdpAudioFormat_copy(self.raw.as_ptr()) };
        SdpAudioFormat {
            raw_unique: expect_non_null(raw, "webrtc_SdpAudioFormat_copy"),
        }
    }
}

/// webrtc::AudioCodecInfo のラッパー。
pub struct AudioCodecInfo {
    raw: NonNull<ffi::webrtc_AudioCodecInfo>,
}

unsafe impl Send for AudioCodecInfo {}

impl AudioCodecInfo {
    /// 新しい AudioCodecInfo を生成する。
    pub fn new(
        sample_rate_hz: i32,
        num_channels: usize,
        default_bitrate_bps: i32,
        min_bitrate_bps: i32,
        max_bitrate_bps: i32,
    ) -> Self {
        let raw = unsafe {
            ffi::webrtc_AudioCodecInfo_new(
                sample_rate_hz,
                num_channels,
                default_bitrate_bps,
                min_bitrate_bps,
                max_bitrate_bps,
            )
        };
        Self {
            raw: expect_non_null(raw, "webrtc_AudioCodecInfo_new"),
        }
    }

    /// サンプルレート (Hz) を返す。
    pub fn sample_rate_hz(&self) -> i32 {
        unsafe { ffi::webrtc_AudioCodecInfo_get_sample_rate_hz(self.raw()) }
    }

    /// サンプルレート (Hz) を設定する。
    pub fn set_sample_rate_hz(&mut self, value: i32) {
        unsafe { ffi::webrtc_AudioCodecInfo_set_sample_rate_hz(self.raw(), value) }
    }

    /// チャンネル数を返す。
    pub fn num_channels(&self) -> usize {
        unsafe { ffi::webrtc_AudioCodecInfo_get_num_channels(self.raw()) }
    }

    /// チャンネル数を設定する。
    pub fn set_num_channels(&mut self, value: usize) {
        unsafe { ffi::webrtc_AudioCodecInfo_set_num_channels(self.raw(), value) }
    }

    /// デフォルトビットレート (bps) を返す。
    pub fn default_bitrate_bps(&self) -> i32 {
        unsafe { ffi::webrtc_AudioCodecInfo_get_default_bitrate_bps(self.raw()) }
    }

    /// デフォルトビットレート (bps) を設定する。
    pub fn set_default_bitrate_bps(&mut self, value: i32) {
        unsafe { ffi::webrtc_AudioCodecInfo_set_default_bitrate_bps(self.raw(), value) }
    }

    /// 最小ビットレート (bps) を返す。
    pub fn min_bitrate_bps(&self) -> i32 {
        unsafe { ffi::webrtc_AudioCodecInfo_get_min_bitrate_bps(self.raw()) }
    }

    /// 最小ビットレート (bps) を設定する。
    pub fn set_min_bitrate_bps(&mut self, value: i32) {
        unsafe { ffi::webrtc_AudioCodecInfo_set_min_bitrate_bps(self.raw(), value) }
    }

    /// 最大ビットレート (bps) を返す。
    pub fn max_bitrate_bps(&self) -> i32 {
        unsafe { ffi::webrtc_AudioCodecInfo_get_max_bitrate_bps(self.raw()) }
    }

    /// 最大ビットレート (bps) を設定する。
    pub fn set_max_bitrate_bps(&mut self, value: i32) {
        unsafe { ffi::webrtc_AudioCodecInfo_set_max_bitrate_bps(self.raw(), value) }
    }

    /// 快音生成と併用可能かどうかを返す。
    pub fn allow_comfort_noise(&self) -> bool {
        unsafe { ffi::webrtc_AudioCodecInfo_get_allow_comfort_noise(self.raw()) != 0 }
    }

    /// 快音生成と併用可能かどうかを設定する。
    pub fn set_allow_comfort_noise(&mut self, value: bool) {
        unsafe { ffi::webrtc_AudioCodecInfo_set_allow_comfort_noise(self.raw(), value.into()) }
    }

    /// ネットワーク適応をサポートするかどうかを返す。
    pub fn supports_network_adaption(&self) -> bool {
        unsafe { ffi::webrtc_AudioCodecInfo_get_supports_network_adaption(self.raw()) != 0 }
    }

    /// ネットワーク適応をサポートするかどうかを設定する。
    pub fn set_supports_network_adaption(&mut self, value: bool) {
        unsafe {
            ffi::webrtc_AudioCodecInfo_set_supports_network_adaption(self.raw(), value.into())
        }
    }

    fn raw(&self) -> *mut ffi::webrtc_AudioCodecInfo {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: NonNull<ffi::webrtc_AudioCodecInfo>) -> Self {
        Self { raw }
    }

    fn into_raw(self) -> *mut ffi::webrtc_AudioCodecInfo {
        std::mem::ManuallyDrop::new(self).raw.as_ptr()
    }
}

impl Clone for AudioCodecInfo {
    fn clone(&self) -> Self {
        let copied = unsafe { ffi::webrtc_AudioCodecInfo_copy(self.raw.as_ptr()) };
        Self {
            raw: expect_non_null(copied, "webrtc_AudioCodecInfo_copy"),
        }
    }
}

impl Drop for AudioCodecInfo {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_AudioCodecInfo_delete(self.raw.as_ptr()) };
    }
}

/// webrtc::AudioCodecSpec のラッパー。
pub struct AudioCodecSpec {
    raw: NonNull<ffi::webrtc_AudioCodecSpec>,
}

unsafe impl Send for AudioCodecSpec {}

impl AudioCodecSpec {
    /// フォーマットと情報から AudioCodecSpec を生成する。
    ///
    /// `format` / `info` はコピーされるだけで所有権は奪われない。
    pub fn new(format: SdpAudioFormat, info: AudioCodecInfo) -> Self {
        let raw = unsafe { ffi::webrtc_AudioCodecSpec_new(format.raw().as_ptr(), info.raw()) };
        Self {
            raw: expect_non_null(raw, "webrtc_AudioCodecSpec_new"),
        }
    }

    /// SDP フォーマットを返す。
    pub fn format(&self) -> SdpAudioFormat {
        let ptr = unsafe { ffi::webrtc_AudioCodecSpec_get_format(self.raw()) };
        let ptr = expect_non_null(ptr, "webrtc_AudioCodecSpec_get_format");
        let raw = unsafe { ffi::webrtc_SdpAudioFormat_copy(ptr.as_ptr()) };
        SdpAudioFormat {
            raw_unique: expect_non_null(raw, "webrtc_SdpAudioFormat_copy"),
        }
    }

    /// コーデック情報を返す。
    pub fn info(&self) -> AudioCodecInfo {
        let ptr = unsafe { ffi::webrtc_AudioCodecSpec_get_info(self.raw()) };
        let ptr = expect_non_null(ptr, "webrtc_AudioCodecSpec_get_info");
        let copied = unsafe { ffi::webrtc_AudioCodecInfo_copy(ptr.as_ptr()) };
        AudioCodecInfo {
            raw: expect_non_null(copied, "webrtc_AudioCodecInfo_copy"),
        }
    }

    /// SDP フォーマットを設定する。
    pub fn set_format(&mut self, format: &SdpAudioFormat) {
        unsafe { ffi::webrtc_AudioCodecSpec_set_format(self.raw(), format.raw().as_ptr()) }
    }

    /// コーデック情報を設定する。
    pub fn set_info(&mut self, info: &AudioCodecInfo) {
        unsafe { ffi::webrtc_AudioCodecSpec_set_info(self.raw(), info.raw()) }
    }

    fn raw(&self) -> *mut ffi::webrtc_AudioCodecSpec {
        self.raw.as_ptr()
    }
}

impl Clone for AudioCodecSpec {
    fn clone(&self) -> Self {
        let copied = unsafe { ffi::webrtc_AudioCodecSpec_copy(self.raw.as_ptr()) };
        Self {
            raw: expect_non_null(copied, "webrtc_AudioCodecSpec_copy"),
        }
    }
}

impl Drop for AudioCodecSpec {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_AudioCodecSpec_delete(self.raw.as_ptr()) };
    }
}

/// エンコード結果の補助情報 (webrtc::AudioEncoder::EncodedInfo) のラッパー。
pub struct AudioEncoderEncodedInfo {
    raw_unique: NonNull<ffi::webrtc_AudioEncoder_EncodedInfo_unique>,
}

unsafe impl Send for AudioEncoderEncodedInfo {}

impl AudioEncoderEncodedInfo {
    /// 新しい EncodedInfo を生成する。
    pub fn new() -> Self {
        let raw = unsafe { ffi::webrtc_AudioEncoder_EncodedInfo_new() };
        Self {
            raw_unique: expect_non_null(raw, "webrtc_AudioEncoder_EncodedInfo_new"),
        }
    }

    /// エンコード済みバイト数を返す。
    pub fn encoded_bytes(&self) -> usize {
        unsafe { ffi::webrtc_AudioEncoder_EncodedInfo_get_encoded_bytes(self.raw()) }
    }

    /// エンコード済みバイト数を設定する。
    pub fn set_encoded_bytes(&mut self, value: usize) {
        unsafe { ffi::webrtc_AudioEncoder_EncodedInfo_set_encoded_bytes(self.raw(), value) }
    }

    /// エンコード済みタイムスタンプを返す。
    pub fn encoded_timestamp(&self) -> u32 {
        unsafe { ffi::webrtc_AudioEncoder_EncodedInfo_get_encoded_timestamp(self.raw()) }
    }

    /// エンコード済みタイムスタンプを設定する。
    pub fn set_encoded_timestamp(&mut self, value: u32) {
        unsafe { ffi::webrtc_AudioEncoder_EncodedInfo_set_encoded_timestamp(self.raw(), value) }
    }

    /// ペイロードタイプを返す。
    pub fn payload_type(&self) -> i32 {
        unsafe { ffi::webrtc_AudioEncoder_EncodedInfo_get_payload_type(self.raw()) }
    }

    /// ペイロードタイプを設定する。
    pub fn set_payload_type(&mut self, value: i32) {
        unsafe { ffi::webrtc_AudioEncoder_EncodedInfo_set_payload_type(self.raw(), value) }
    }

    /// 空でも送信するかどうかを返す。
    pub fn send_even_if_empty(&self) -> bool {
        unsafe { ffi::webrtc_AudioEncoder_EncodedInfo_get_send_even_if_empty(self.raw()) != 0 }
    }

    /// 空でも送信するかどうかを設定する。
    pub fn set_send_even_if_empty(&mut self, value: bool) {
        unsafe {
            ffi::webrtc_AudioEncoder_EncodedInfo_set_send_even_if_empty(self.raw(), value.into())
        }
    }

    /// 音声かどうかを返す。
    pub fn speech(&self) -> bool {
        unsafe { ffi::webrtc_AudioEncoder_EncodedInfo_get_speech(self.raw()) != 0 }
    }

    /// 音声かどうかを設定する。
    pub fn set_speech(&mut self, value: bool) {
        unsafe { ffi::webrtc_AudioEncoder_EncodedInfo_set_speech(self.raw(), value.into()) }
    }

    /// エンコーダータイプを返す。
    pub fn encoder_type(&self) -> AudioCodecType {
        unsafe {
            AudioCodecType::from_raw(ffi::webrtc_AudioEncoder_EncodedInfo_get_encoder_type(
                self.raw(),
            ))
        }
    }

    /// エンコーダータイプを設定する。
    ///
    /// # Panics
    /// `value` が既知のコーデックタイプでない場合、libwebrtc 内部のヒストグラム配列が
    /// 範囲外アクセスになるため panic する。
    pub fn set_encoder_type(&mut self, value: AudioCodecType) {
        // codec_histogram_bins_log_ は既知の CodecType 分しか持たないため、未知値
        // (Unknown) は libwebrtc 内部で配列 OOB になる。ここで拒否する。
        assert!(
            !matches!(value, AudioCodecType::Unknown(_)),
            "encoder_type は既知のコーデックタイプで指定してください"
        );
        unsafe { ffi::webrtc_AudioEncoder_EncodedInfo_set_encoder_type(self.raw(), value.to_raw()) }
    }

    /// 冗長符号化の要素をコピーして返す。
    pub fn redundant(&self) -> Vec<AudioEncoderEncodedInfoLeaf> {
        let vec = unsafe { ffi::webrtc_AudioEncoder_EncodedInfo_get_redundant(self.raw()) };
        let vec = expect_non_null(vec, "webrtc_AudioEncoder_EncodedInfo_get_redundant");
        let size = unsafe { ffi::webrtc_AudioEncoder_EncodedInfoLeaf_vector_size(vec.as_ptr()) };
        let size = size.max(0) as usize;
        let mut redundant = Vec::with_capacity(size);
        for i in 0..size {
            let raw = unsafe {
                ffi::webrtc_AudioEncoder_EncodedInfoLeaf_vector_get(vec.as_ptr(), i as i32)
            };
            let raw = expect_non_null(raw, "webrtc_AudioEncoder_EncodedInfoLeaf_vector_get");
            // Safety: vector が保持する leaf への借用ポインタを返す。_copy で複製して所有する。
            let copied = unsafe { ffi::webrtc_AudioEncoder_EncodedInfoLeaf_copy(raw.as_ptr()) };
            redundant.push(unsafe {
                AudioEncoderEncodedInfoLeaf::from_raw(expect_non_null(
                    copied,
                    "webrtc_AudioEncoder_EncodedInfoLeaf_copy",
                ))
            });
        }
        redundant
    }

    /// 冗長符号化の要素を設定する (コピー)。
    pub fn set_redundant(&mut self, redundant: Vec<AudioEncoderEncodedInfoLeaf>) {
        let vec = unsafe { ffi::webrtc_AudioEncoder_EncodedInfoLeaf_vector_new(0) };
        let vec = expect_non_null(vec, "webrtc_AudioEncoder_EncodedInfoLeaf_vector_new");
        for leaf in &redundant {
            unsafe {
                ffi::webrtc_AudioEncoder_EncodedInfoLeaf_vector_push_back(
                    vec.as_ptr(),
                    leaf.raw.as_ptr(),
                )
            };
        }
        unsafe { ffi::webrtc_AudioEncoder_EncodedInfo_set_redundant(self.raw(), vec.as_ptr()) };
        unsafe { ffi::webrtc_AudioEncoder_EncodedInfoLeaf_vector_delete(vec.as_ptr()) };
    }

    fn raw(&self) -> *mut ffi::webrtc_AudioEncoder_EncodedInfo {
        unsafe { ffi::webrtc_AudioEncoder_EncodedInfo_unique_get(self.raw_unique.as_ptr()) }
    }

    fn into_raw(self) -> *mut ffi::webrtc_AudioEncoder_EncodedInfo_unique {
        std::mem::ManuallyDrop::new(self).raw_unique.as_ptr()
    }
}

impl Default for AudioEncoderEncodedInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for AudioEncoderEncodedInfo {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_AudioEncoder_EncodedInfo_unique_delete(self.raw_unique.as_ptr()) };
    }
}

/// 冗長符号化の 1 要素 (webrtc::AudioEncoder::EncodedInfoLeaf) のラッパー。
pub struct AudioEncoderEncodedInfoLeaf {
    raw: NonNull<ffi::webrtc_AudioEncoder_EncodedInfoLeaf>,
}

unsafe impl Send for AudioEncoderEncodedInfoLeaf {}

impl AudioEncoderEncodedInfoLeaf {
    /// 新しい EncodedInfoLeaf を生成する。
    pub fn new() -> Self {
        let raw = unsafe { ffi::webrtc_AudioEncoder_EncodedInfoLeaf_new() };
        Self {
            raw: expect_non_null(raw, "webrtc_AudioEncoder_EncodedInfoLeaf_new"),
        }
    }

    /// # Safety
    /// `raw` は C 側で生成された有効な leaf を指し、所有権をこの型が引き受ける必要があります。
    pub(crate) unsafe fn from_raw(raw: NonNull<ffi::webrtc_AudioEncoder_EncodedInfoLeaf>) -> Self {
        Self { raw }
    }

    /// エンコード済みバイト数を返す。
    pub fn encoded_bytes(&self) -> usize {
        unsafe { ffi::webrtc_AudioEncoder_EncodedInfoLeaf_get_encoded_bytes(self.raw.as_ptr()) }
    }

    /// エンコード済みバイト数を設定する。
    pub fn set_encoded_bytes(&mut self, value: usize) {
        unsafe {
            ffi::webrtc_AudioEncoder_EncodedInfoLeaf_set_encoded_bytes(self.raw.as_ptr(), value)
        }
    }

    /// エンコード済みタイムスタンプを返す。
    pub fn encoded_timestamp(&self) -> u32 {
        unsafe { ffi::webrtc_AudioEncoder_EncodedInfoLeaf_get_encoded_timestamp(self.raw.as_ptr()) }
    }

    /// エンコード済みタイムスタンプを設定する。
    pub fn set_encoded_timestamp(&mut self, value: u32) {
        unsafe {
            ffi::webrtc_AudioEncoder_EncodedInfoLeaf_set_encoded_timestamp(self.raw.as_ptr(), value)
        }
    }

    /// ペイロードタイプを返す。
    pub fn payload_type(&self) -> i32 {
        unsafe { ffi::webrtc_AudioEncoder_EncodedInfoLeaf_get_payload_type(self.raw.as_ptr()) }
    }

    /// ペイロードタイプを設定する。
    pub fn set_payload_type(&mut self, value: i32) {
        unsafe {
            ffi::webrtc_AudioEncoder_EncodedInfoLeaf_set_payload_type(self.raw.as_ptr(), value)
        }
    }

    /// 空でも送信するかどうかを返す。
    pub fn send_even_if_empty(&self) -> bool {
        unsafe {
            ffi::webrtc_AudioEncoder_EncodedInfoLeaf_get_send_even_if_empty(self.raw.as_ptr()) != 0
        }
    }

    /// 空でも送信するかどうかを設定する。
    pub fn set_send_even_if_empty(&mut self, value: bool) {
        unsafe {
            ffi::webrtc_AudioEncoder_EncodedInfoLeaf_set_send_even_if_empty(
                self.raw.as_ptr(),
                value.into(),
            )
        }
    }

    /// 音声かどうかを返す。
    pub fn speech(&self) -> bool {
        unsafe { ffi::webrtc_AudioEncoder_EncodedInfoLeaf_get_speech(self.raw.as_ptr()) != 0 }
    }

    /// 音声かどうかを設定する。
    pub fn set_speech(&mut self, value: bool) {
        unsafe {
            ffi::webrtc_AudioEncoder_EncodedInfoLeaf_set_speech(self.raw.as_ptr(), value.into())
        }
    }

    /// エンコーダータイプを返す。
    pub fn encoder_type(&self) -> AudioCodecType {
        unsafe {
            AudioCodecType::from_raw(ffi::webrtc_AudioEncoder_EncodedInfoLeaf_get_encoder_type(
                self.raw.as_ptr(),
            ))
        }
    }

    /// エンコーダータイプを設定する。
    ///
    /// # Panics
    /// `value` が既知のコーデックタイプでない場合、libwebrtc 内部のヒストグラム配列が
    /// 範囲外アクセスになるため panic する。
    pub fn set_encoder_type(&mut self, value: AudioCodecType) {
        assert!(
            !matches!(value, AudioCodecType::Unknown(_)),
            "encoder_type は既知のコーデックタイプで指定してください"
        );
        unsafe {
            ffi::webrtc_AudioEncoder_EncodedInfoLeaf_set_encoder_type(
                self.raw.as_ptr(),
                value.to_raw(),
            )
        }
    }
}

impl Default for AudioEncoderEncodedInfoLeaf {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for AudioEncoderEncodedInfoLeaf {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_AudioEncoder_EncodedInfoLeaf_delete(self.raw.as_ptr()) };
    }
}

impl Clone for AudioEncoderEncodedInfoLeaf {
    fn clone(&self) -> Self {
        let raw = unsafe { ffi::webrtc_AudioEncoder_EncodedInfoLeaf_copy(self.raw.as_ptr()) };
        Self {
            raw: expect_non_null(raw, "webrtc_AudioEncoder_EncodedInfoLeaf_copy"),
        }
    }
}

/// `webrtc::BitrateAllocationUpdate` の所有ラッパー。
pub struct BitrateAllocationUpdate {
    raw: NonNull<ffi::webrtc_BitrateAllocationUpdate>,
}

unsafe impl Send for BitrateAllocationUpdate {}

impl BitrateAllocationUpdate {
    /// 新しい BitrateAllocationUpdate を生成する。
    pub fn new() -> Self {
        let raw = unsafe { ffi::webrtc_BitrateAllocationUpdate_new() };
        Self {
            raw: expect_non_null(raw, "webrtc_BitrateAllocationUpdate_new"),
        }
    }

    /// # Safety
    /// `raw` は C 側で生成された有効な `webrtc_BitrateAllocationUpdate` を指し、
    /// 所有権をこの型が引き受ける必要があります。
    pub(crate) unsafe fn from_raw(raw: NonNull<ffi::webrtc_BitrateAllocationUpdate>) -> Self {
        Self { raw }
    }

    /// 割り当てられたターゲットビットレート (bps) を返す。
    pub fn target_bitrate_bps(&self) -> i64 {
        unsafe { ffi::webrtc_BitrateAllocationUpdate_get_target_bitrate_bps(self.raw.as_ptr()) }
    }

    /// 割り当てられたターゲットビットレート (bps) を設定する。
    pub fn set_target_bitrate_bps(&mut self, value: i64) {
        unsafe {
            ffi::webrtc_BitrateAllocationUpdate_set_target_bitrate_bps(self.raw.as_ptr(), value)
        }
    }

    /// 予測パケットロス率を返す。
    pub fn packet_loss_ratio(&self) -> f64 {
        unsafe { ffi::webrtc_BitrateAllocationUpdate_get_packet_loss_ratio(self.raw.as_ptr()) }
    }

    /// 予測パケットロス率を設定する。
    pub fn set_packet_loss_ratio(&mut self, value: f64) {
        unsafe {
            ffi::webrtc_BitrateAllocationUpdate_set_packet_loss_ratio(self.raw.as_ptr(), value)
        }
    }

    /// 予測ラウンドトリップ時間 (マイクロ秒) を返す。
    ///
    /// 値が未設定の場合は `i64::MAX` として表現される。
    pub fn round_trip_time_us(&self) -> i64 {
        unsafe { ffi::webrtc_BitrateAllocationUpdate_get_round_trip_time_us(self.raw.as_ptr()) }
    }

    /// 予測ラウンドトリップ時間 (マイクロ秒) を設定する。
    ///
    /// `i64::MAX` を指定すると `webrtc::TimeDelta::PlusInfinity()` となる。
    pub fn set_round_trip_time_us(&mut self, value: i64) {
        unsafe {
            ffi::webrtc_BitrateAllocationUpdate_set_round_trip_time_us(self.raw.as_ptr(), value)
        }
    }

    /// 輻輳ウィンドウ pushback によるビットレート削減率を返す。
    pub fn cwnd_reduce_ratio(&self) -> f64 {
        unsafe { ffi::webrtc_BitrateAllocationUpdate_get_cwnd_reduce_ratio(self.raw.as_ptr()) }
    }

    /// 輻輳ウィンドウ pushback によるビットレート削減率を設定する。
    pub fn set_cwnd_reduce_ratio(&mut self, value: f64) {
        unsafe {
            ffi::webrtc_BitrateAllocationUpdate_set_cwnd_reduce_ratio(self.raw.as_ptr(), value)
        }
    }

    /// パケットあたりのトランスポートオーバーヘッド (バイト) を返す。
    pub fn packet_overhead_bytes(&self) -> i64 {
        unsafe { ffi::webrtc_BitrateAllocationUpdate_get_packet_overhead_bytes(self.raw.as_ptr()) }
    }

    /// パケットあたりのトランスポートオーバーヘッド (バイト) を設定する。
    pub fn set_packet_overhead_bytes(&mut self, value: i64) {
        unsafe {
            ffi::webrtc_BitrateAllocationUpdate_set_packet_overhead_bytes(self.raw.as_ptr(), value)
        }
    }

    fn raw(&self) -> *mut ffi::webrtc_BitrateAllocationUpdate {
        self.raw.as_ptr()
    }
}

impl Clone for BitrateAllocationUpdate {
    fn clone(&self) -> Self {
        let raw = unsafe { ffi::webrtc_BitrateAllocationUpdate_copy(self.raw.as_ptr()) };
        Self {
            raw: expect_non_null(raw, "webrtc_BitrateAllocationUpdate_copy"),
        }
    }
}

impl Default for BitrateAllocationUpdate {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for BitrateAllocationUpdate {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_BitrateAllocationUpdate_delete(self.raw.as_ptr()) };
    }
}

/// `AudioEncoder` のコールバックハンドラ。
///
/// 各メソッドは WebRTC の `webrtc::AudioEncoder` の仮想関数に対応する。
/// デフォルト実装は WebRTC 側のデフォルト挙動に近づけたものである。
pub trait AudioEncoderHandler: Send {
    /// 入力サンプルレート (Hz) を返す。
    fn sample_rate_hz(&mut self) -> i32;

    /// 入力チャンネル数を返す。
    fn num_channels(&mut self) -> usize;

    /// RTP タイムスタンプのレート (Hz) を返す。デフォルトは sample_rate_hz()。
    fn rtp_timestamp_rate_hz(&mut self) -> i32 {
        self.sample_rate_hz()
    }

    /// 次のパケットに入れる 10ms フレーム数を返す。
    fn num_10ms_frames_in_next_packet(&mut self) -> usize;

    /// 1 パケットに入れられる最大 10ms フレーム数を返す。
    fn max_10ms_frames_in_a_packet(&mut self) -> usize;

    /// 現在のターゲットビットレート (bps) を返す。`-1` は自動適応を意味する。
    fn get_target_bitrate(&mut self) -> i32;

    /// オーディオフレームをエンコードし、`encoded` へ書き込み、補助情報を返す。
    ///
    /// `encoded` は WebRTC 側が用意した空のバッファで、`append_data` で書き込む。
    ///
    /// # 契約
    ///
    /// WebRTC の `AudioEncoder::Encode` (api/audio_codecs/audio_encoder.cc) は以下の
    /// 2 つを `RTC_CHECK_EQ` で検証するため、違反するとデバッグ・リリースを問わず
    /// プロセスが abort する。
    ///
    /// - `audio.len()` は `sample_rate_hz() * num_channels() / 100` でなければならない
    /// - `encoded` へ `append_data` したバイト数の合計と `set_encoded_bytes` で設定した値が
    ///   一致しなければならない
    fn encode(
        &mut self,
        rtp_timestamp: u32,
        audio: &[i16],
        encoded: &mut BufferRef<'_>,
    ) -> AudioEncoderEncodedInfo;

    /// エンコーダーを初期状態へ戻す。
    fn reset(&mut self);

    /// FEC を有効化/無効化する。
    fn set_fec(&mut self, enable: bool) -> bool {
        !enable
    }

    /// DTX を有効化/無効化する。
    fn set_dtx(&mut self, enable: bool) -> bool {
        !enable
    }

    /// DTX の状態を返す。
    fn get_dtx(&mut self) -> bool {
        false
    }

    /// アプリケーションモードを設定する。
    #[expect(unused_variables)]
    fn set_application(&mut self, application: i32) -> bool {
        false
    }

    /// デコーダーが想定する最大再生レートを設定する。
    #[expect(unused_variables)]
    fn set_max_playback_rate(&mut self, frequency_hz: i32) {}

    /// 内包エンコーダーを解放して返す。
    fn reclaim_contained_encoders(&mut self) -> Vec<AudioEncoder> {
        Vec::new()
    }

    /// ANA (Audio Network Adaptation) を有効化する。
    #[expect(unused_variables)]
    fn enable_audio_network_adaptor(&mut self, config: &[u8]) -> bool {
        false
    }

    /// ANA (Audio Network Adaptation) を無効化する。
    fn disable_audio_network_adaptor(&mut self) {}

    /// 上りパケットロス率を通知する。
    #[expect(unused_variables)]
    fn on_received_uplink_packet_loss_fraction(&mut self, fraction: f32) {}

    /// ターゲット音声ビットレートを通知する。
    #[expect(unused_variables)]
    fn on_received_target_audio_bitrate(&mut self, target_bps: i32) {}

    /// 上りビットレート割り当てを通知する。
    #[expect(unused_variables)]
    fn on_received_uplink_allocation(&mut self, allocation: BitrateAllocationUpdate) {}

    /// RTT を通知する。
    #[expect(unused_variables)]
    fn on_received_rtt(&mut self, rtt_ms: i32) {}

    /// パケットのオーバーヘッドを通知する。
    #[expect(unused_variables)]
    fn on_received_overhead(&mut self, overhead_bytes_per_packet: usize) {}

    /// 受信側が受け入れられるフレーム長の範囲を設定する。
    #[expect(unused_variables)]
    fn set_receiver_frame_length_range(&mut self, min_ms: i32, max_ms: i32) {}

    /// ANA 統計を返す。
    fn get_ana_stats(&mut self) -> AudioEncoderAnaStats {
        AudioEncoderAnaStats::new()
    }

    /// サポートされるフレーム長範囲 (マイクロ秒) を返す。
    fn get_frame_length_range(&mut self) -> Option<(i64, i64)>;

    /// サポートされるビットレート範囲 (bps) を返す。
    fn get_bitrate_range(&mut self) -> Option<(i64, i64)> {
        None
    }
}

/// `webrtc::AudioEncoder::ANAStats` のラッパー。
pub struct AudioEncoderAnaStats {
    raw: NonNull<ffi::webrtc_AudioEncoder_ANAStats>,
}

unsafe impl Send for AudioEncoderAnaStats {}

impl AudioEncoderAnaStats {
    /// 新しい空の ANA 統計を生成する。
    pub fn new() -> Self {
        let raw = unsafe { ffi::webrtc_AudioEncoder_ANAStats_new() };
        Self {
            raw: expect_non_null(raw, "webrtc_AudioEncoder_ANAStats_new"),
        }
    }

    /// ANA ビットレートコントローラーが動作した回数を返す。
    pub fn bitrate_action_counter(&self) -> Option<u32> {
        get_optional(|has, value| unsafe {
            ffi::webrtc_AudioEncoder_ANAStats_get_bitrate_action_counter(
                self.raw.as_ptr(),
                has,
                value,
            )
        })
    }

    /// ANA ビットレートコントローラーが動作した回数を設定する。
    pub fn set_bitrate_action_counter(&mut self, value: Option<u32>) {
        set_optional(value, |has, ptr| unsafe {
            ffi::webrtc_AudioEncoder_ANAStats_set_bitrate_action_counter(
                self.raw.as_ptr(),
                has,
                ptr,
            )
        })
    }

    /// ANA チャンネルコントローラーが動作した回数を返す。
    pub fn channel_action_counter(&self) -> Option<u32> {
        get_optional(|has, value| unsafe {
            ffi::webrtc_AudioEncoder_ANAStats_get_channel_action_counter(
                self.raw.as_ptr(),
                has,
                value,
            )
        })
    }

    /// ANA チャンネルコントローラーが動作した回数を設定する。
    pub fn set_channel_action_counter(&mut self, value: Option<u32>) {
        set_optional(value, |has, ptr| unsafe {
            ffi::webrtc_AudioEncoder_ANAStats_set_channel_action_counter(
                self.raw.as_ptr(),
                has,
                ptr,
            )
        })
    }

    /// ANA DTX コントローラーが動作した回数を返す。
    pub fn dtx_action_counter(&self) -> Option<u32> {
        get_optional(|has, value| unsafe {
            ffi::webrtc_AudioEncoder_ANAStats_get_dtx_action_counter(self.raw.as_ptr(), has, value)
        })
    }

    /// ANA DTX コントローラーが動作した回数を設定する。
    pub fn set_dtx_action_counter(&mut self, value: Option<u32>) {
        set_optional(value, |has, ptr| unsafe {
            ffi::webrtc_AudioEncoder_ANAStats_set_dtx_action_counter(self.raw.as_ptr(), has, ptr)
        })
    }

    /// ANA FEC コントローラーが動作した回数を返す。
    pub fn fec_action_counter(&self) -> Option<u32> {
        get_optional(|has, value| unsafe {
            ffi::webrtc_AudioEncoder_ANAStats_get_fec_action_counter(self.raw.as_ptr(), has, value)
        })
    }

    /// ANA FEC コントローラーが動作した回数を設定する。
    pub fn set_fec_action_counter(&mut self, value: Option<u32>) {
        set_optional(value, |has, ptr| unsafe {
            ffi::webrtc_AudioEncoder_ANAStats_set_fec_action_counter(self.raw.as_ptr(), has, ptr)
        })
    }

    /// ANA フレーム長コントローラーが増加を決定した回数を返す。
    pub fn frame_length_increase_counter(&self) -> Option<u32> {
        get_optional(|has, value| unsafe {
            ffi::webrtc_AudioEncoder_ANAStats_get_frame_length_increase_counter(
                self.raw.as_ptr(),
                has,
                value,
            )
        })
    }

    /// ANA フレーム長コントローラーが増加を決定した回数を設定する。
    pub fn set_frame_length_increase_counter(&mut self, value: Option<u32>) {
        set_optional(value, |has, ptr| unsafe {
            ffi::webrtc_AudioEncoder_ANAStats_set_frame_length_increase_counter(
                self.raw.as_ptr(),
                has,
                ptr,
            )
        })
    }

    /// ANA フレーム長コントローラーが減少を決定した回数を返す。
    pub fn frame_length_decrease_counter(&self) -> Option<u32> {
        get_optional(|has, value| unsafe {
            ffi::webrtc_AudioEncoder_ANAStats_get_frame_length_decrease_counter(
                self.raw.as_ptr(),
                has,
                value,
            )
        })
    }

    /// ANA フレーム長コントローラーが減少を決定した回数を設定する。
    pub fn set_frame_length_decrease_counter(&mut self, value: Option<u32>) {
        set_optional(value, |has, ptr| unsafe {
            ffi::webrtc_AudioEncoder_ANAStats_set_frame_length_decrease_counter(
                self.raw.as_ptr(),
                has,
                ptr,
            )
        })
    }

    /// ANA FEC コントローラーが設定した上りパケットロス率を返す。
    pub fn uplink_packet_loss_fraction(&self) -> Option<f32> {
        get_optional(|has, value| unsafe {
            ffi::webrtc_AudioEncoder_ANAStats_get_uplink_packet_loss_fraction(
                self.raw.as_ptr(),
                has,
                value,
            )
        })
    }

    /// ANA FEC コントローラーが設定した上りパケットロス率を設定する。
    pub fn set_uplink_packet_loss_fraction(&mut self, value: Option<f32>) {
        set_optional(value, |has, ptr| unsafe {
            ffi::webrtc_AudioEncoder_ANAStats_set_uplink_packet_loss_fraction(
                self.raw.as_ptr(),
                has,
                ptr,
            )
        })
    }

    pub(crate) fn raw(&self) -> *mut ffi::webrtc_AudioEncoder_ANAStats {
        self.raw.as_ptr()
    }
}

impl Default for AudioEncoderAnaStats {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for AudioEncoderAnaStats {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_AudioEncoder_ANAStats_delete(self.raw.as_ptr()) };
    }
}

type AudioEncoderHandlerState = HandlerState<dyn AudioEncoderHandler>;

unsafe extern "C" fn audio_encoder_sample_rate_hz(user_data: *mut c_void) -> i32 {
    assert!(
        !user_data.is_null(),
        "audio_encoder_sample_rate_hz: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioEncoderHandlerState) };
    state.handler.sample_rate_hz()
}

unsafe extern "C" fn audio_encoder_num_channels(user_data: *mut c_void) -> usize {
    assert!(
        !user_data.is_null(),
        "audio_encoder_num_channels: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioEncoderHandlerState) };
    state.handler.num_channels()
}

unsafe extern "C" fn audio_encoder_rtp_timestamp_rate_hz(user_data: *mut c_void) -> i32 {
    assert!(
        !user_data.is_null(),
        "audio_encoder_rtp_timestamp_rate_hz: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioEncoderHandlerState) };
    state.handler.rtp_timestamp_rate_hz()
}

unsafe extern "C" fn audio_encoder_num_10ms_frames_in_next_packet(user_data: *mut c_void) -> usize {
    assert!(
        !user_data.is_null(),
        "audio_encoder_num_10ms_frames_in_next_packet: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioEncoderHandlerState) };
    state.handler.num_10ms_frames_in_next_packet()
}

unsafe extern "C" fn audio_encoder_max_10ms_frames_in_a_packet(user_data: *mut c_void) -> usize {
    assert!(
        !user_data.is_null(),
        "audio_encoder_max_10ms_frames_in_a_packet: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioEncoderHandlerState) };
    state.handler.max_10ms_frames_in_a_packet()
}

unsafe extern "C" fn audio_encoder_get_target_bitrate(user_data: *mut c_void) -> i32 {
    assert!(
        !user_data.is_null(),
        "audio_encoder_get_target_bitrate: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioEncoderHandlerState) };
    state.handler.get_target_bitrate()
}

unsafe extern "C" fn audio_encoder_encode(
    rtp_timestamp: u32,
    audio: *const i16,
    audio_size: usize,
    encoded: *mut ffi::webrtc_Buffer,
    user_data: *mut c_void,
) -> *mut ffi::webrtc_AudioEncoder_EncodedInfo_unique {
    assert!(
        !user_data.is_null(),
        "audio_encoder_encode: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioEncoderHandlerState) };
    let audio = if audio.is_null() || audio_size == 0 {
        &[]
    } else {
        unsafe { slice::from_raw_parts(audio, audio_size) }
    };
    let mut encoded =
        unsafe { BufferRef::from_raw(expect_non_null(encoded, "audio_encoder_encode (encoded)")) };
    state
        .handler
        .encode(rtp_timestamp, audio, &mut encoded)
        .into_raw()
}

unsafe extern "C" fn audio_encoder_reset(user_data: *mut c_void) {
    assert!(
        !user_data.is_null(),
        "audio_encoder_reset: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioEncoderHandlerState) };
    state.handler.reset();
}

unsafe extern "C" fn audio_encoder_set_fec(enable: i32, user_data: *mut c_void) -> i32 {
    assert!(
        !user_data.is_null(),
        "audio_encoder_set_fec: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioEncoderHandlerState) };
    if state.handler.set_fec(enable != 0) {
        1
    } else {
        0
    }
}

unsafe extern "C" fn audio_encoder_set_dtx(enable: i32, user_data: *mut c_void) -> i32 {
    assert!(
        !user_data.is_null(),
        "audio_encoder_set_dtx: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioEncoderHandlerState) };
    if state.handler.set_dtx(enable != 0) {
        1
    } else {
        0
    }
}

unsafe extern "C" fn audio_encoder_get_dtx(user_data: *mut c_void) -> i32 {
    assert!(
        !user_data.is_null(),
        "audio_encoder_get_dtx: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioEncoderHandlerState) };
    if state.handler.get_dtx() { 1 } else { 0 }
}

unsafe extern "C" fn audio_encoder_set_application(
    application: i32,
    user_data: *mut c_void,
) -> i32 {
    assert!(
        !user_data.is_null(),
        "audio_encoder_set_application: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioEncoderHandlerState) };
    if state.handler.set_application(application) {
        1
    } else {
        0
    }
}

unsafe extern "C" fn audio_encoder_set_max_playback_rate(
    frequency_hz: i32,
    user_data: *mut c_void,
) {
    assert!(
        !user_data.is_null(),
        "audio_encoder_set_max_playback_rate: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioEncoderHandlerState) };
    state.handler.set_max_playback_rate(frequency_hz);
}

unsafe extern "C" fn audio_encoder_reclaim_contained_encoders(
    user_data: *mut c_void,
) -> *mut ffi::webrtc_AudioEncoder_unique_vector {
    assert!(
        !user_data.is_null(),
        "audio_encoder_reclaim_contained_encoders: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioEncoderHandlerState) };
    let reclaim = state.handler.reclaim_contained_encoders();
    let vec = expect_non_null(
        unsafe { ffi::webrtc_AudioEncoder_unique_vector_new() },
        "webrtc_AudioEncoder_unique_vector_new",
    )
    .as_ptr();
    for encoder in reclaim {
        unsafe { ffi::webrtc_AudioEncoder_unique_vector_push_back(vec, encoder.into_raw()) };
    }
    vec
}

unsafe extern "C" fn audio_encoder_enable_audio_network_adaptor(
    config: *const u8,
    config_len: usize,
    user_data: *mut c_void,
) -> i32 {
    assert!(
        !user_data.is_null(),
        "audio_encoder_enable_audio_network_adaptor: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioEncoderHandlerState) };
    let config = if config.is_null() || config_len == 0 {
        &[]
    } else {
        unsafe { slice::from_raw_parts(config, config_len) }
    };
    if state.handler.enable_audio_network_adaptor(config) {
        1
    } else {
        0
    }
}

unsafe extern "C" fn audio_encoder_disable_audio_network_adaptor(user_data: *mut c_void) {
    assert!(
        !user_data.is_null(),
        "audio_encoder_disable_audio_network_adaptor: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioEncoderHandlerState) };
    state.handler.disable_audio_network_adaptor();
}

unsafe extern "C" fn audio_encoder_on_received_uplink_packet_loss_fraction(
    fraction: f32,
    user_data: *mut c_void,
) {
    assert!(
        !user_data.is_null(),
        "audio_encoder_on_received_uplink_packet_loss_fraction: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioEncoderHandlerState) };
    state
        .handler
        .on_received_uplink_packet_loss_fraction(fraction);
}

unsafe extern "C" fn audio_encoder_on_received_target_audio_bitrate(
    target_bps: i32,
    user_data: *mut c_void,
) {
    assert!(
        !user_data.is_null(),
        "audio_encoder_on_received_target_audio_bitrate: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioEncoderHandlerState) };
    state.handler.on_received_target_audio_bitrate(target_bps);
}

unsafe extern "C" fn audio_encoder_on_received_uplink_allocation(
    update: *const ffi::webrtc_BitrateAllocationUpdate,
    user_data: *mut c_void,
) {
    assert!(
        !user_data.is_null(),
        "audio_encoder_on_received_uplink_allocation: user_data is null"
    );
    let update = expect_non_null(
        update.cast_mut(),
        "audio_encoder_on_received_uplink_allocation (update)",
    );
    let state = unsafe { &mut *(user_data as *mut AudioEncoderHandlerState) };
    // C++ 側の update はコールバック期間だけ生きるため、コピーして所有する。
    let copied = unsafe { ffi::webrtc_BitrateAllocationUpdate_copy(update.as_ptr()) };
    let update = unsafe {
        BitrateAllocationUpdate::from_raw(expect_non_null(
            copied,
            "webrtc_BitrateAllocationUpdate_copy",
        ))
    };
    state.handler.on_received_uplink_allocation(update);
}

unsafe extern "C" fn audio_encoder_on_received_rtt(rtt_ms: i32, user_data: *mut c_void) {
    assert!(
        !user_data.is_null(),
        "audio_encoder_on_received_rtt: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioEncoderHandlerState) };
    state.handler.on_received_rtt(rtt_ms);
}

unsafe extern "C" fn audio_encoder_on_received_overhead(
    overhead_bytes_per_packet: usize,
    user_data: *mut c_void,
) {
    assert!(
        !user_data.is_null(),
        "audio_encoder_on_received_overhead: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioEncoderHandlerState) };
    state
        .handler
        .on_received_overhead(overhead_bytes_per_packet);
}

unsafe extern "C" fn audio_encoder_set_receiver_frame_length_range(
    min_frame_length_ms: i32,
    max_frame_length_ms: i32,
    user_data: *mut c_void,
) {
    assert!(
        !user_data.is_null(),
        "audio_encoder_set_receiver_frame_length_range: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioEncoderHandlerState) };
    state
        .handler
        .set_receiver_frame_length_range(min_frame_length_ms, max_frame_length_ms);
}

unsafe extern "C" fn audio_encoder_get_ana_stats(
    user_data: *mut c_void,
) -> *mut ffi::webrtc_AudioEncoder_ANAStats {
    assert!(
        !user_data.is_null(),
        "audio_encoder_get_ana_stats: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioEncoderHandlerState) };
    let stats = state.handler.get_ana_stats();
    // ハンドラーが保持する ANAStats のコピーを C++ 側へ返す。
    unsafe { ffi::webrtc_AudioEncoder_ANAStats_copy(stats.raw.as_ptr()) }
}

unsafe extern "C" fn audio_encoder_get_frame_length_range(
    out_has: *mut i32,
    out_min_us: *mut i64,
    out_max_us: *mut i64,
    user_data: *mut c_void,
) {
    assert!(
        !user_data.is_null(),
        "audio_encoder_get_frame_length_range: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioEncoderHandlerState) };
    set_optional2(
        state.handler.get_frame_length_range(),
        |has, min, max| unsafe {
            *out_has = has;
            if has != 0 {
                *out_min_us = *min;
                *out_max_us = *max;
            }
        },
    );
}

unsafe extern "C" fn audio_encoder_get_bitrate_range(
    out_has: *mut i32,
    out_min_bps: *mut i64,
    out_max_bps: *mut i64,
    user_data: *mut c_void,
) {
    assert!(
        !user_data.is_null(),
        "audio_encoder_get_bitrate_range: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioEncoderHandlerState) };
    set_optional2(state.handler.get_bitrate_range(), |has, min, max| unsafe {
        *out_has = has;
        if has != 0 {
            *out_min_bps = *min;
            *out_max_bps = *max;
        }
    });
}

unsafe extern "C" fn audio_encoder_on_destroy(user_data: *mut c_void) {
    unsafe { destroy_handler::<AudioEncoderHandlerState>("audio_encoder_on_destroy", user_data) };
}

/// webrtc::AudioEncoder のラッパー。
pub struct AudioEncoder {
    raw_unique: NonNull<ffi::webrtc_AudioEncoder_unique>,
}

unsafe impl Send for AudioEncoder {}

impl AudioEncoder {
    /// ハンドラから AudioEncoder を生成する。
    pub fn new_with_handler(handler: Box<dyn AudioEncoderHandler>) -> Self {
        let user_data = Box::into_raw(Box::new(HandlerState::new(handler))) as *mut c_void;
        let cbs = ffi::webrtc_AudioEncoder_cbs {
            SampleRateHz: Some(audio_encoder_sample_rate_hz),
            NumChannels: Some(audio_encoder_num_channels),
            RtpTimestampRateHz: Some(audio_encoder_rtp_timestamp_rate_hz),
            Num10MsFramesInNextPacket: Some(audio_encoder_num_10ms_frames_in_next_packet),
            Max10MsFramesInAPacket: Some(audio_encoder_max_10ms_frames_in_a_packet),
            GetTargetBitrate: Some(audio_encoder_get_target_bitrate),
            Encode: Some(audio_encoder_encode),
            Reset: Some(audio_encoder_reset),
            SetFec: Some(audio_encoder_set_fec),
            SetDtx: Some(audio_encoder_set_dtx),
            GetDtx: Some(audio_encoder_get_dtx),
            SetApplication: Some(audio_encoder_set_application),
            SetMaxPlaybackRate: Some(audio_encoder_set_max_playback_rate),
            ReclaimContainedEncoders: Some(audio_encoder_reclaim_contained_encoders),
            EnableAudioNetworkAdaptor: Some(audio_encoder_enable_audio_network_adaptor),
            DisableAudioNetworkAdaptor: Some(audio_encoder_disable_audio_network_adaptor),
            OnReceivedUplinkPacketLossFraction: Some(
                audio_encoder_on_received_uplink_packet_loss_fraction,
            ),
            OnReceivedTargetAudioBitrate: Some(audio_encoder_on_received_target_audio_bitrate),
            OnReceivedUplinkAllocation: Some(audio_encoder_on_received_uplink_allocation),
            OnReceivedRtt: Some(audio_encoder_on_received_rtt),
            OnReceivedOverhead: Some(audio_encoder_on_received_overhead),
            SetReceiverFrameLengthRange: Some(audio_encoder_set_receiver_frame_length_range),
            GetANAStats: Some(audio_encoder_get_ana_stats),
            GetFrameLengthRange: Some(audio_encoder_get_frame_length_range),
            GetBitrateRange: Some(audio_encoder_get_bitrate_range),
            OnDestroy: Some(audio_encoder_on_destroy),
        };
        let raw_unique = unsafe {
            create_with_handler::<AudioEncoderHandlerState, _>(
                "webrtc_AudioEncoder_new",
                user_data,
                |user_data| ffi::webrtc_AudioEncoder_new(&cbs, user_data),
            )
        };
        Self { raw_unique }
    }

    pub(crate) fn as_ptr(&self) -> *mut ffi::webrtc_AudioEncoder {
        unsafe { ffi::webrtc_AudioEncoder_unique_get(self.raw_unique.as_ptr()) }
    }

    pub(crate) fn into_raw(self) -> *mut ffi::webrtc_AudioEncoder_unique {
        std::mem::ManuallyDrop::new(self).raw_unique.as_ptr()
    }

    /// 入力サンプルレート (Hz) を返す。
    pub fn sample_rate_hz(&self) -> i32 {
        unsafe { ffi::webrtc_AudioEncoder_SampleRateHz(self.as_ptr()) }
    }

    /// 入力チャンネル数を返す。
    pub fn num_channels(&self) -> usize {
        unsafe { ffi::webrtc_AudioEncoder_NumChannels(self.as_ptr()) }
    }

    /// RTP タイムスタンプレート (Hz) を返す。
    pub fn rtp_timestamp_rate_hz(&self) -> i32 {
        unsafe { ffi::webrtc_AudioEncoder_RtpTimestampRateHz(self.as_ptr()) }
    }

    /// 次のパケットに入れる 10ms フレーム数を返す。
    pub fn num_10ms_frames_in_next_packet(&self) -> usize {
        unsafe { ffi::webrtc_AudioEncoder_Num10MsFramesInNextPacket(self.as_ptr()) }
    }

    /// 1 パケットに入れられる最大 10ms フレーム数を返す。
    pub fn max_10ms_frames_in_a_packet(&self) -> usize {
        unsafe { ffi::webrtc_AudioEncoder_Max10MsFramesInAPacket(self.as_ptr()) }
    }

    /// 現在のターゲットビットレート (bps) を返す。
    pub fn get_target_bitrate(&self) -> i32 {
        unsafe { ffi::webrtc_AudioEncoder_GetTargetBitrate(self.as_ptr()) }
    }

    /// サポートされるフレーム長範囲（マイクロ秒）を返す。
    pub fn get_frame_length_range(&self) -> Option<(i64, i64)> {
        get_optional2(|out_has, out_min_us, out_max_us| unsafe {
            ffi::webrtc_AudioEncoder_GetFrameLengthRange(
                self.as_ptr(),
                out_has,
                out_min_us,
                out_max_us,
            );
        })
    }

    /// サポートされるビットレート範囲 (bps) を返す。
    pub fn get_bitrate_range(&self) -> Option<(i64, i64)> {
        get_optional2(|out_has, out_min_bps, out_max_bps| unsafe {
            ffi::webrtc_AudioEncoder_GetBitrateRange(
                self.as_ptr(),
                out_has,
                out_min_bps,
                out_max_bps,
            );
        })
    }

    /// オーディオフレームをエンコードする。
    pub fn encode(
        &mut self,
        rtp_timestamp: u32,
        audio: &[i16],
        encoded: &mut Buffer,
    ) -> AudioEncoderEncodedInfo {
        let raw = unsafe {
            ffi::webrtc_AudioEncoder_Encode(
                self.as_ptr(),
                rtp_timestamp,
                audio.as_ptr(),
                audio.len(),
                encoded.as_ptr(),
            )
        };
        AudioEncoderEncodedInfo {
            raw_unique: expect_non_null(raw, "webrtc_AudioEncoder_Encode"),
        }
    }

    /// エンコーダーを初期状態へ戻す。
    pub fn reset(&mut self) {
        unsafe { ffi::webrtc_AudioEncoder_Reset(self.as_ptr()) }
    }

    /// FEC を有効化/無効化する。
    pub fn set_fec(&mut self, enable: bool) -> bool {
        unsafe { ffi::webrtc_AudioEncoder_SetFec(self.as_ptr(), enable.into()) != 0 }
    }

    /// DTX を有効化/無効化する。
    pub fn set_dtx(&mut self, enable: bool) -> bool {
        unsafe { ffi::webrtc_AudioEncoder_SetDtx(self.as_ptr(), enable.into()) != 0 }
    }

    /// DTX の状態を返す。
    pub fn get_dtx(&self) -> bool {
        unsafe { ffi::webrtc_AudioEncoder_GetDtx(self.as_ptr()) != 0 }
    }

    /// アプリケーションモードを設定する。
    pub fn set_application(&mut self, application: i32) -> bool {
        unsafe { ffi::webrtc_AudioEncoder_SetApplication(self.as_ptr(), application) != 0 }
    }

    /// 最大再生レート (Hz) を設定する。
    pub fn set_max_playback_rate(&mut self, frequency_hz: i32) {
        unsafe { ffi::webrtc_AudioEncoder_SetMaxPlaybackRate(self.as_ptr(), frequency_hz) }
    }

    /// 内包するエンコーダーを回収する。
    pub fn reclaim_contained_encoders(&mut self) -> Vec<AudioEncoder> {
        let raw_vec = unsafe { ffi::webrtc_AudioEncoder_ReclaimContainedEncoders(self.as_ptr()) };
        let raw_vec = expect_non_null(raw_vec, "webrtc_AudioEncoder_ReclaimContainedEncoders");
        let size = unsafe { ffi::webrtc_AudioEncoder_unique_vector_size(raw_vec.as_ptr()) };
        let mut encoders = Vec::with_capacity(size);
        for i in 0..size {
            let raw = unsafe { ffi::webrtc_AudioEncoder_unique_vector_take(raw_vec.as_ptr(), i) };
            encoders.push(AudioEncoder {
                raw_unique: expect_non_null(raw, "webrtc_AudioEncoder_unique_vector_take"),
            });
        }
        unsafe { ffi::webrtc_AudioEncoder_unique_vector_delete(raw_vec.as_ptr()) };
        encoders
    }

    /// オーディオネットワークアダプターを有効化する。
    pub fn enable_audio_network_adaptor(&mut self, config: &[u8]) -> bool {
        unsafe {
            ffi::webrtc_AudioEncoder_EnableAudioNetworkAdaptor(
                self.as_ptr(),
                config.as_ptr(),
                config.len(),
            ) != 0
        }
    }

    /// オーディオネットワークアダプターを無効化する。
    pub fn disable_audio_network_adaptor(&mut self) {
        unsafe { ffi::webrtc_AudioEncoder_DisableAudioNetworkAdaptor(self.as_ptr()) }
    }

    /// 上りパケットロス率を通知する。
    pub fn on_received_uplink_packet_loss_fraction(&mut self, fraction: f32) {
        unsafe {
            ffi::webrtc_AudioEncoder_OnReceivedUplinkPacketLossFraction(self.as_ptr(), fraction)
        }
    }

    /// ターゲット音声ビットレートを通知する。
    pub fn on_received_target_audio_bitrate(&mut self, target_bps: i32) {
        unsafe { ffi::webrtc_AudioEncoder_OnReceivedTargetAudioBitrate(self.as_ptr(), target_bps) }
    }

    /// 上りビットレート割り当てを通知する。
    pub fn on_received_uplink_allocation(&mut self, allocation: BitrateAllocationUpdate) {
        unsafe {
            ffi::webrtc_AudioEncoder_OnReceivedUplinkAllocation(self.as_ptr(), allocation.raw())
        }
    }

    /// RTT を通知する。
    pub fn on_received_rtt(&mut self, rtt_ms: i32) {
        unsafe { ffi::webrtc_AudioEncoder_OnReceivedRtt(self.as_ptr(), rtt_ms) }
    }

    /// パケットのオーバーヘッドを通知する。
    pub fn on_received_overhead(&mut self, overhead_bytes_per_packet: usize) {
        unsafe {
            ffi::webrtc_AudioEncoder_OnReceivedOverhead(self.as_ptr(), overhead_bytes_per_packet)
        }
    }

    /// 受信側が受け入れられるフレーム長の範囲を設定する。
    pub fn set_receiver_frame_length_range(&mut self, min_ms: i32, max_ms: i32) {
        unsafe {
            ffi::webrtc_AudioEncoder_SetReceiverFrameLengthRange(self.as_ptr(), min_ms, max_ms)
        }
    }

    /// ANA 統計を返す。
    pub fn get_ana_stats(&self) -> AudioEncoderAnaStats {
        let stats = AudioEncoderAnaStats::new();
        unsafe {
            ffi::webrtc_AudioEncoder_GetANAStats(self.as_ptr(), stats.raw());
        }
        stats
    }
}

impl Drop for AudioEncoder {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_AudioEncoder_unique_delete(self.raw_unique.as_ptr()) };
    }
}

/// デコード結果の音声タイプ。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioSpeechType {
    /// 音声。
    Speech,
    /// 快音。
    ComfortNoise,
    /// 未知の音声タイプ。
    Unknown(i32),
}

impl AudioSpeechType {
    fn to_raw(self) -> i32 {
        match self {
            Self::Speech => unsafe { ffi::webrtc_AudioDecoder_SpeechType_kSpeech },
            Self::ComfortNoise => unsafe { ffi::webrtc_AudioDecoder_SpeechType_kComfortNoise },
            Self::Unknown(value) => value,
        }
    }

    fn from_raw(raw: i32) -> Self {
        if raw == unsafe { ffi::webrtc_AudioDecoder_SpeechType_kComfortNoise } {
            Self::ComfortNoise
        } else if raw == unsafe { ffi::webrtc_AudioDecoder_SpeechType_kSpeech } {
            Self::Speech
        } else {
            Self::Unknown(raw)
        }
    }
}

/// `AudioDecoder` のコールバックハンドラ。
///
/// 各メソッドは WebRTC の `webrtc::AudioDecoder` の仮想関数に対応する。
pub trait AudioDecoderHandler: Send {
    /// ペイロードをデコードし、`decoded` へ書き込んでサンプル数と音声タイプを返す。
    fn decode(
        &mut self,
        encoded: &[u8],
        sample_rate_hz: i32,
        decoded: &mut RawBufferWriter<'_, i16>,
    ) -> (i32, AudioSpeechType);

    /// 冗長ペイロードをデコードする。デフォルトは decode() を呼ぶ。
    fn decode_redundant(
        &mut self,
        encoded: &[u8],
        sample_rate_hz: i32,
        decoded: &mut RawBufferWriter<'_, i16>,
    ) -> (i32, AudioSpeechType) {
        self.decode(encoded, sample_rate_hz, decoded)
    }

    /// PLC を実装するかどうかを返す。
    fn has_decode_plc(&mut self) -> bool {
        false
    }

    /// パケットロス隠蔽を実行する。
    #[expect(unused_variables)]
    fn decode_plc(&mut self, num_frames: usize, decoded: &mut RawBufferWriter<'_, i16>) -> usize {
        0
    }

    /// パケットロス隠蔽を生成する。
    ///
    /// 生成できた場合は `concealment_audio` へ追記する。生成しなかった場合は
    /// 追記せず、WebRTC 側が他の手段でロスを隠蔽する。
    #[expect(unused_variables)]
    fn generate_plc(
        &mut self,
        requested_samples_per_channel: usize,
        concealment_audio: &mut BufferS16Ref<'_>,
    ) {
    }

    /// デコーダーを初期状態へ戻す。
    fn reset(&mut self);

    /// 最後のエラーコードを返す。
    fn error_code(&mut self) -> i32 {
        0
    }

    /// ペイロードの継続時間 (サンプル/チャンネル) を返す。
    #[expect(unused_variables)]
    fn packet_duration(&mut self, encoded: &[u8]) -> i32 {
        -2
    }

    /// 冗長ペイロードの継続時間 (サンプル/チャンネル) を返す。
    #[expect(unused_variables)]
    fn packet_duration_redundant(&mut self, encoded: &[u8]) -> i32 {
        -2
    }

    /// ペイロードが FEC を含むかどうかを返す。
    #[expect(unused_variables)]
    fn packet_has_fec(&mut self, encoded: &[u8]) -> bool {
        false
    }

    /// 出力サンプルレート (Hz) を返す。
    fn sample_rate_hz(&mut self) -> i32;

    /// 出力チャンネル数を返す。
    fn channels(&mut self) -> usize;
}

type AudioDecoderHandlerState = HandlerState<dyn AudioDecoderHandler>;

unsafe extern "C" fn audio_decoder_decode(
    encoded: *const u8,
    encoded_len: usize,
    sample_rate_hz: i32,
    decoded: *mut i16,
    speech_type: *mut i32,
    user_data: *mut c_void,
) -> i32 {
    assert!(
        !user_data.is_null(),
        "audio_decoder_decode: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioDecoderHandlerState) };
    let encoded = if encoded.is_null() || encoded_len == 0 {
        &[]
    } else {
        unsafe { slice::from_raw_parts(encoded, encoded_len) }
    };
    let decoded = expect_non_null(decoded, "audio_decoder_decode (decoded)");
    let speech_type = expect_non_null(speech_type, "audio_decoder_decode (speech_type)");
    let mut decoded = unsafe { RawBufferWriter::from_raw(decoded) };
    let (samples, speech) = state.handler.decode(encoded, sample_rate_hz, &mut decoded);
    unsafe {
        *speech_type.as_ptr() = speech.to_raw();
    }
    samples
}

unsafe extern "C" fn audio_decoder_decode_redundant(
    encoded: *const u8,
    encoded_len: usize,
    sample_rate_hz: i32,
    decoded: *mut i16,
    speech_type: *mut i32,
    user_data: *mut c_void,
) -> i32 {
    assert!(
        !user_data.is_null(),
        "audio_decoder_decode_redundant: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioDecoderHandlerState) };
    let encoded = if encoded.is_null() || encoded_len == 0 {
        &[]
    } else {
        unsafe { slice::from_raw_parts(encoded, encoded_len) }
    };
    let decoded = expect_non_null(decoded, "audio_decoder_decode_redundant (decoded)");
    let speech_type = expect_non_null(speech_type, "audio_decoder_decode_redundant (speech_type)");
    let mut decoded = unsafe { RawBufferWriter::from_raw(decoded) };
    let (samples, speech) = state
        .handler
        .decode_redundant(encoded, sample_rate_hz, &mut decoded);
    unsafe {
        *speech_type.as_ptr() = speech.to_raw();
    }
    samples
}

unsafe extern "C" fn audio_decoder_has_decode_plc(user_data: *mut c_void) -> i32 {
    assert!(
        !user_data.is_null(),
        "audio_decoder_has_decode_plc: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioDecoderHandlerState) };
    if state.handler.has_decode_plc() { 1 } else { 0 }
}

unsafe extern "C" fn audio_decoder_decode_plc(
    num_frames: usize,
    decoded: *mut i16,
    user_data: *mut c_void,
) -> usize {
    assert!(
        !user_data.is_null(),
        "audio_decoder_decode_plc: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioDecoderHandlerState) };
    let decoded = expect_non_null(decoded, "audio_decoder_decode_plc (decoded)");
    let mut decoded = unsafe { RawBufferWriter::from_raw(decoded) };
    state.handler.decode_plc(num_frames, &mut decoded)
}

unsafe extern "C" fn audio_decoder_generate_plc(
    requested_samples_per_channel: usize,
    concealment_audio: *mut ffi::webrtc_BufferS16,
    user_data: *mut c_void,
) {
    assert!(
        !user_data.is_null(),
        "audio_decoder_generate_plc: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioDecoderHandlerState) };
    let concealment_audio = expect_non_null(
        concealment_audio,
        "audio_decoder_generate_plc (concealment_audio)",
    );
    let mut concealment_audio = unsafe { BufferS16Ref::from_raw(concealment_audio) };
    state
        .handler
        .generate_plc(requested_samples_per_channel, &mut concealment_audio);
}

unsafe extern "C" fn audio_decoder_reset(user_data: *mut c_void) {
    assert!(
        !user_data.is_null(),
        "audio_decoder_reset: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioDecoderHandlerState) };
    state.handler.reset();
}

unsafe extern "C" fn audio_decoder_error_code(user_data: *mut c_void) -> i32 {
    assert!(
        !user_data.is_null(),
        "audio_decoder_error_code: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioDecoderHandlerState) };
    state.handler.error_code()
}

unsafe extern "C" fn audio_decoder_packet_duration(
    encoded: *const u8,
    encoded_len: usize,
    user_data: *mut c_void,
) -> i32 {
    assert!(
        !user_data.is_null(),
        "audio_decoder_packet_duration: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioDecoderHandlerState) };
    let encoded = if encoded.is_null() || encoded_len == 0 {
        &[]
    } else {
        unsafe { slice::from_raw_parts(encoded, encoded_len) }
    };
    state.handler.packet_duration(encoded)
}

unsafe extern "C" fn audio_decoder_packet_duration_redundant(
    encoded: *const u8,
    encoded_len: usize,
    user_data: *mut c_void,
) -> i32 {
    assert!(
        !user_data.is_null(),
        "audio_decoder_packet_duration_redundant: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioDecoderHandlerState) };
    let encoded = if encoded.is_null() || encoded_len == 0 {
        &[]
    } else {
        unsafe { slice::from_raw_parts(encoded, encoded_len) }
    };
    state.handler.packet_duration_redundant(encoded)
}

unsafe extern "C" fn audio_decoder_packet_has_fec(
    encoded: *const u8,
    encoded_len: usize,
    user_data: *mut c_void,
) -> i32 {
    assert!(
        !user_data.is_null(),
        "audio_decoder_packet_has_fec: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioDecoderHandlerState) };
    let encoded = if encoded.is_null() || encoded_len == 0 {
        &[]
    } else {
        unsafe { slice::from_raw_parts(encoded, encoded_len) }
    };
    if state.handler.packet_has_fec(encoded) {
        1
    } else {
        0
    }
}

unsafe extern "C" fn audio_decoder_sample_rate_hz(user_data: *mut c_void) -> i32 {
    assert!(
        !user_data.is_null(),
        "audio_decoder_sample_rate_hz: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioDecoderHandlerState) };
    state.handler.sample_rate_hz()
}

unsafe extern "C" fn audio_decoder_channels(user_data: *mut c_void) -> usize {
    assert!(
        !user_data.is_null(),
        "audio_decoder_channels: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioDecoderHandlerState) };
    state.handler.channels()
}

unsafe extern "C" fn audio_decoder_on_destroy(user_data: *mut c_void) {
    unsafe { destroy_handler::<AudioDecoderHandlerState>("audio_decoder_on_destroy", user_data) };
}

/// webrtc::AudioDecoder のラッパー。
pub struct AudioDecoder {
    raw_unique: NonNull<ffi::webrtc_AudioDecoder_unique>,
}

unsafe impl Send for AudioDecoder {}

impl AudioDecoder {
    /// ハンドラから AudioDecoder を生成する。
    pub fn new_with_handler(handler: Box<dyn AudioDecoderHandler>) -> Self {
        let user_data = Box::into_raw(Box::new(HandlerState::new(handler))) as *mut c_void;
        let cbs = ffi::webrtc_AudioDecoder_cbs {
            Decode: Some(audio_decoder_decode),
            DecodeRedundant: Some(audio_decoder_decode_redundant),
            HasDecodePlc: Some(audio_decoder_has_decode_plc),
            DecodePlc: Some(audio_decoder_decode_plc),
            GeneratePlc: Some(audio_decoder_generate_plc),
            Reset: Some(audio_decoder_reset),
            ErrorCode: Some(audio_decoder_error_code),
            PacketDuration: Some(audio_decoder_packet_duration),
            PacketDurationRedundant: Some(audio_decoder_packet_duration_redundant),
            PacketHasFec: Some(audio_decoder_packet_has_fec),
            SampleRateHz: Some(audio_decoder_sample_rate_hz),
            Channels: Some(audio_decoder_channels),
            OnDestroy: Some(audio_decoder_on_destroy),
        };
        let raw_unique = unsafe {
            create_with_handler::<AudioDecoderHandlerState, _>(
                "webrtc_AudioDecoder_new",
                user_data,
                |user_data| ffi::webrtc_AudioDecoder_new(&cbs, user_data),
            )
        };
        Self { raw_unique }
    }

    pub(crate) fn as_ptr(&self) -> *mut ffi::webrtc_AudioDecoder {
        unsafe { ffi::webrtc_AudioDecoder_unique_get(self.raw_unique.as_ptr()) }
    }

    pub(crate) fn into_raw(self) -> *mut ffi::webrtc_AudioDecoder_unique {
        std::mem::ManuallyDrop::new(self).raw_unique.as_ptr()
    }

    /// サンプルレート (Hz) を返す。
    pub fn sample_rate_hz(&self) -> i32 {
        unsafe { ffi::webrtc_AudioDecoder_SampleRateHz(self.as_ptr()) }
    }

    /// チャンネル数を返す。
    pub fn channels(&self) -> usize {
        unsafe { ffi::webrtc_AudioDecoder_Channels(self.as_ptr()) }
    }

    /// PLC を実装するかどうかを返す。
    pub fn has_decode_plc(&self) -> bool {
        unsafe { ffi::webrtc_AudioDecoder_HasDecodePlc(self.as_ptr()) != 0 }
    }

    /// ペイロードをデコードする。
    ///
    /// `decoded` にはデコード結果を書き込み、書き込んだサンプル数と音声タイプを返す。
    pub fn decode(
        &mut self,
        encoded: &[u8],
        sample_rate_hz: i32,
        decoded: &mut [i16],
    ) -> (i32, AudioSpeechType) {
        let mut speech_type = 0i32;
        let samples = unsafe {
            ffi::webrtc_AudioDecoder_Decode(
                self.as_ptr(),
                encoded.as_ptr(),
                encoded.len(),
                sample_rate_hz,
                decoded.as_mut_ptr(),
                std::mem::size_of_val(decoded),
                &mut speech_type,
            )
        };
        (samples, AudioSpeechType::from_raw(speech_type))
    }

    /// 冗長ペイロードをデコードする。
    pub fn decode_redundant(
        &mut self,
        encoded: &[u8],
        sample_rate_hz: i32,
        decoded: &mut [i16],
    ) -> (i32, AudioSpeechType) {
        let mut speech_type = 0i32;
        let samples = unsafe {
            ffi::webrtc_AudioDecoder_DecodeRedundant(
                self.as_ptr(),
                encoded.as_ptr(),
                encoded.len(),
                sample_rate_hz,
                decoded.as_mut_ptr(),
                std::mem::size_of_val(decoded),
                &mut speech_type,
            )
        };
        (samples, AudioSpeechType::from_raw(speech_type))
    }

    /// パケットロス隠蔽を実行する。
    pub fn decode_plc(&mut self, num_frames: usize, decoded: &mut [i16]) -> usize {
        unsafe {
            ffi::webrtc_AudioDecoder_DecodePlc(self.as_ptr(), num_frames, decoded.as_mut_ptr())
        }
    }

    /// パケットロス隠蔽を生成する。
    ///
    /// 生成しなかった場合、`concealment_audio` は空のまま残る。
    pub fn generate_plc(
        &mut self,
        requested_samples_per_channel: usize,
        concealment_audio: &mut BufferS16Ref<'_>,
    ) {
        unsafe {
            ffi::webrtc_AudioDecoder_GeneratePlc(
                self.as_ptr(),
                requested_samples_per_channel,
                concealment_audio.raw(),
            )
        }
    }

    /// デコーダーを初期状態へ戻す。
    pub fn reset(&mut self) {
        unsafe { ffi::webrtc_AudioDecoder_Reset(self.as_ptr()) }
    }

    /// 最後のエラーコードを返す。
    pub fn error_code(&self) -> i32 {
        unsafe { ffi::webrtc_AudioDecoder_ErrorCode(self.as_ptr()) }
    }

    /// ペイロードの継続時間 (サンプル/チャンネル) を返す。
    pub fn packet_duration(&self, encoded: &[u8]) -> i32 {
        unsafe {
            ffi::webrtc_AudioDecoder_PacketDuration(self.as_ptr(), encoded.as_ptr(), encoded.len())
        }
    }

    /// 冗長ペイロードの継続時間 (サンプル/チャンネル) を返す。
    pub fn packet_duration_redundant(&self, encoded: &[u8]) -> i32 {
        unsafe {
            ffi::webrtc_AudioDecoder_PacketDurationRedundant(
                self.as_ptr(),
                encoded.as_ptr(),
                encoded.len(),
            )
        }
    }

    /// ペイロードが FEC を含むかどうかを返す。
    pub fn packet_has_fec(&self, encoded: &[u8]) -> bool {
        unsafe {
            ffi::webrtc_AudioDecoder_PacketHasFec(self.as_ptr(), encoded.as_ptr(), encoded.len())
                != 0
        }
    }
}

impl Drop for AudioDecoder {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_AudioDecoder_unique_delete(self.raw_unique.as_ptr()) };
    }
}

/// `webrtc::AudioCodecPairId` のラッパー。
pub struct AudioCodecPairId {
    raw: NonNull<ffi::webrtc_AudioCodecPairId>,
}

unsafe impl Send for AudioCodecPairId {}

impl AudioCodecPairId {
    /// 新しい ID を生成する。
    pub fn create() -> Self {
        let raw = unsafe { ffi::webrtc_AudioCodecPairId_Create() };
        Self {
            raw: expect_non_null(raw, "webrtc_AudioCodecPairId_Create"),
        }
    }

    /// 数値表現を返す。
    pub fn numeric_representation(&self) -> u64 {
        unsafe { ffi::webrtc_AudioCodecPairId_NumericRepresentation(self.raw.as_ptr()) }
    }

    /// # Safety
    /// `raw` は C 側で生成された有効な `webrtc_AudioCodecPairId` を指し、
    /// 所有権をこの型が引き受ける必要があります。
    pub(crate) unsafe fn from_raw(raw: NonNull<ffi::webrtc_AudioCodecPairId>) -> Self {
        Self { raw }
    }

    fn raw(&self) -> *mut ffi::webrtc_AudioCodecPairId {
        self.raw.as_ptr()
    }
}

impl Clone for AudioCodecPairId {
    fn clone(&self) -> Self {
        let raw = unsafe { ffi::webrtc_AudioCodecPairId_copy(self.raw.as_ptr()) };
        Self {
            raw: expect_non_null(raw, "webrtc_AudioCodecPairId_copy"),
        }
    }
}

impl PartialEq for AudioCodecPairId {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ffi::webrtc_AudioCodecPairId_is_equal(self.raw.as_ptr(), other.raw.as_ptr()) != 0 }
    }
}

impl Eq for AudioCodecPairId {}

impl PartialOrd for AudioCodecPairId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AudioCodecPairId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_less = unsafe {
            ffi::webrtc_AudioCodecPairId_less(self.raw.as_ptr(), other.raw.as_ptr()) != 0
        };
        let other_less = unsafe {
            ffi::webrtc_AudioCodecPairId_less(other.raw.as_ptr(), self.raw.as_ptr()) != 0
        };
        match (self_less, other_less) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => std::cmp::Ordering::Equal,
        }
    }
}

impl Drop for AudioCodecPairId {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_AudioCodecPairId_delete(self.raw.as_ptr()) };
    }
}

/// `webrtc::AudioEncoderFactory::Options` のラッパー。
pub struct AudioEncoderFactoryOptions {
    raw: NonNull<ffi::webrtc_AudioEncoderFactory_Options>,
}

unsafe impl Send for AudioEncoderFactoryOptions {}

impl AudioEncoderFactoryOptions {
    /// 新しい Options を生成する。
    pub fn new() -> Self {
        let raw = unsafe { ffi::webrtc_AudioEncoderFactory_Options_new() };
        Self {
            raw: expect_non_null(raw, "webrtc_AudioEncoderFactory_Options_new"),
        }
    }

    /// ペイロードタイプを返す。
    pub fn payload_type(&self) -> i32 {
        unsafe { ffi::webrtc_AudioEncoderFactory_Options_get_payload_type(self.raw.as_ptr()) }
    }

    /// ペイロードタイプを設定する。
    pub fn set_payload_type(&mut self, value: i32) {
        unsafe {
            ffi::webrtc_AudioEncoderFactory_Options_set_payload_type(self.raw.as_ptr(), value)
        }
    }

    /// コーデックペア ID を返す。
    pub fn codec_pair_id(&self) -> Option<AudioCodecPairId> {
        let raw =
            unsafe { ffi::webrtc_AudioEncoderFactory_Options_get_codec_pair_id(self.raw.as_ptr()) };
        NonNull::new(raw).map(|raw| unsafe { AudioCodecPairId::from_raw(raw) })
    }

    /// コーデックペア ID を設定 / 解除する。
    pub fn set_codec_pair_id(&mut self, value: Option<&AudioCodecPairId>) {
        match value {
            Some(v) => unsafe {
                ffi::webrtc_AudioEncoderFactory_Options_set_codec_pair_id(
                    self.raw.as_ptr(),
                    1,
                    v.raw(),
                )
            },
            None => unsafe {
                ffi::webrtc_AudioEncoderFactory_Options_set_codec_pair_id(
                    self.raw.as_ptr(),
                    0,
                    std::ptr::null(),
                )
            },
        }
    }

    fn raw(&self) -> *mut ffi::webrtc_AudioEncoderFactory_Options {
        self.raw.as_ptr()
    }
}

impl Default for AudioEncoderFactoryOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for AudioEncoderFactoryOptions {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_AudioEncoderFactory_Options_delete(self.raw.as_ptr()) };
    }
}

/// `AudioEncoderFactory` のコールバックハンドラ。
pub trait AudioEncoderFactoryHandler: Send {
    /// サポートされるエンコーダーの一覧を返す。
    fn get_supported_encoders(&mut self) -> Vec<AudioCodecSpec> {
        Vec::new()
    }

    /// エンコーダーがフォーマットに対応するかを問い合わせる。
    #[expect(unused_variables)]
    fn query_audio_encoder(&mut self, format: SdpAudioFormatRef<'_>) -> Option<AudioCodecInfo> {
        None
    }

    /// エンコーダーを生成する。
    #[expect(unused_variables)]
    fn create(
        &mut self,
        env: EnvironmentRef<'_>,
        format: SdpAudioFormatRef<'_>,
        options: &AudioEncoderFactoryOptions,
    ) -> Option<AudioEncoder> {
        None
    }
}

type AudioEncoderFactoryHandlerState = HandlerState<dyn AudioEncoderFactoryHandler>;

unsafe extern "C" fn audio_encoder_factory_get_supported_encoders(
    user_data: *mut c_void,
) -> *mut ffi::webrtc_AudioCodecSpec_vector {
    assert!(
        !user_data.is_null(),
        "audio_encoder_factory_get_supported_encoders: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioEncoderFactoryHandlerState) };
    let specs = state.handler.get_supported_encoders();
    let vec = expect_non_null(
        unsafe { ffi::webrtc_AudioCodecSpec_vector_new() },
        "webrtc_AudioCodecSpec_vector_new",
    )
    .as_ptr();
    for spec in &specs {
        unsafe { ffi::webrtc_AudioCodecSpec_vector_push_back(vec, spec.raw()) };
    }
    vec
}

unsafe extern "C" fn audio_encoder_factory_query_audio_encoder(
    format: *const ffi::webrtc_SdpAudioFormat,
    user_data: *mut c_void,
) -> *mut ffi::webrtc_AudioCodecInfo {
    assert!(
        !user_data.is_null(),
        "audio_encoder_factory_query_audio_encoder: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioEncoderFactoryHandlerState) };
    let format = expect_non_null(
        format.cast_mut(),
        "audio_encoder_factory_query_audio_encoder (format)",
    );
    let format = unsafe { SdpAudioFormatRef::from_raw(format) };
    match state.handler.query_audio_encoder(format) {
        Some(info) => info.into_raw(),
        None => std::ptr::null_mut(),
    }
}

unsafe extern "C" fn audio_encoder_factory_create(
    env: *const ffi::webrtc_Environment,
    format: *const ffi::webrtc_SdpAudioFormat,
    options: *mut ffi::webrtc_AudioEncoderFactory_Options,
    user_data: *mut c_void,
) -> *mut ffi::webrtc_AudioEncoder_unique {
    assert!(
        !user_data.is_null(),
        "audio_encoder_factory_create: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioEncoderFactoryHandlerState) };
    let env = expect_non_null(env.cast_mut(), "audio_encoder_factory_create (env)");
    let format = expect_non_null(format.cast_mut(), "audio_encoder_factory_create (format)");
    let options = expect_non_null(options, "audio_encoder_factory_create (options)");
    let env = unsafe { EnvironmentRef::from_raw(env) };
    let format = unsafe { SdpAudioFormatRef::from_raw(format) };
    // options は C++ 側が保有する借用ポインタのため、Drop しないよう ManuallyDrop で包む。
    let options = std::mem::ManuallyDrop::new(AudioEncoderFactoryOptions { raw: options });
    match state.handler.create(env, format, &options) {
        Some(encoder) => encoder.into_raw(),
        None => std::ptr::null_mut(),
    }
}

unsafe extern "C" fn audio_encoder_factory_on_destroy(user_data: *mut c_void) {
    unsafe {
        destroy_handler::<AudioEncoderFactoryHandlerState>(
            "audio_encoder_factory_on_destroy",
            user_data,
        )
    };
}

impl AudioEncoderFactory {
    /// ハンドラから AudioEncoderFactory を生成する。
    pub fn new_with_handler(handler: Box<dyn AudioEncoderFactoryHandler>) -> Self {
        let user_data = Box::into_raw(Box::new(HandlerState::new(handler))) as *mut c_void;
        let cbs = ffi::webrtc_AudioEncoderFactory_cbs {
            GetSupportedEncoders: Some(audio_encoder_factory_get_supported_encoders),
            QueryAudioEncoder: Some(audio_encoder_factory_query_audio_encoder),
            Create: Some(audio_encoder_factory_create),
            OnDestroy: Some(audio_encoder_factory_on_destroy),
        };
        let raw_ref = unsafe {
            create_with_handler::<AudioEncoderFactoryHandlerState, _>(
                "webrtc_AudioEncoderFactory_make_ref_counted",
                user_data,
                |user_data| ffi::webrtc_AudioEncoderFactory_make_ref_counted(&cbs, user_data),
            )
        };
        let raw_ref = ScopedRef::<AudioEncoderFactoryHandle>::from_raw(raw_ref);
        Self { raw_ref }
    }

    /// サポートされるエンコーダーの一覧を返す。
    pub fn get_supported_encoders(&self) -> Vec<AudioCodecSpec> {
        let raw_vec =
            unsafe { ffi::webrtc_AudioEncoderFactory_GetSupportedEncoders(self.as_ptr()) };
        let raw_vec = expect_non_null(raw_vec, "webrtc_AudioEncoderFactory_GetSupportedEncoders");
        let size = unsafe { ffi::webrtc_AudioCodecSpec_vector_size(raw_vec.as_ptr()) };
        let mut specs = Vec::with_capacity(size.max(0) as usize);
        for i in 0..size {
            let raw = unsafe { ffi::webrtc_AudioCodecSpec_vector_get(raw_vec.as_ptr(), i) };
            let raw = expect_non_null(raw, "webrtc_AudioCodecSpec_vector_get");
            let copied = unsafe { ffi::webrtc_AudioCodecSpec_copy(raw.as_ptr()) };
            specs.push(AudioCodecSpec {
                raw: expect_non_null(copied, "webrtc_AudioCodecSpec_copy"),
            });
        }
        unsafe { ffi::webrtc_AudioCodecSpec_vector_delete(raw_vec.as_ptr()) };
        specs
    }

    /// エンコーダーがフォーマットに対応するかを問い合わせる。
    pub fn query_audio_encoder(&self, format: SdpAudioFormatRef<'_>) -> Option<AudioCodecInfo> {
        let raw = unsafe {
            ffi::webrtc_AudioEncoderFactory_QueryAudioEncoder(self.as_ptr(), format.as_ptr())
        };
        NonNull::new(raw).map(AudioCodecInfo::from_raw)
    }

    /// エンコーダーを生成する。
    pub fn create(
        &self,
        env: EnvironmentRef<'_>,
        format: SdpAudioFormatRef<'_>,
        options: &AudioEncoderFactoryOptions,
    ) -> Option<AudioEncoder> {
        let raw = unsafe {
            ffi::webrtc_AudioEncoderFactory_MakeAudioEncoder(
                self.as_ptr(),
                env.as_ptr(),
                format.as_ptr(),
                options.raw(),
            )
        };
        Some(AudioEncoder {
            raw_unique: NonNull::new(raw)?,
        })
    }
}

/// `AudioDecoderFactory` のコールバックハンドラ。
pub trait AudioDecoderFactoryHandler: Send {
    /// サポートされるデコーダーの一覧を返す。
    fn get_supported_decoders(&mut self) -> Vec<AudioCodecSpec> {
        Vec::new()
    }

    /// デコーダーがフォーマットに対応するかを返す。
    #[expect(unused_variables)]
    fn is_supported_decoder(&mut self, format: SdpAudioFormatRef<'_>) -> bool {
        false
    }

    /// デコーダーを生成する。
    #[expect(unused_variables)]
    fn create(
        &mut self,
        env: EnvironmentRef<'_>,
        format: SdpAudioFormatRef<'_>,
    ) -> Option<AudioDecoder> {
        None
    }
}

type AudioDecoderFactoryHandlerState = HandlerState<dyn AudioDecoderFactoryHandler>;

unsafe extern "C" fn audio_decoder_factory_get_supported_decoders(
    user_data: *mut c_void,
) -> *mut ffi::webrtc_AudioCodecSpec_vector {
    assert!(
        !user_data.is_null(),
        "audio_decoder_factory_get_supported_decoders: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioDecoderFactoryHandlerState) };
    let specs = state.handler.get_supported_decoders();
    let vec = expect_non_null(
        unsafe { ffi::webrtc_AudioCodecSpec_vector_new() },
        "webrtc_AudioCodecSpec_vector_new",
    )
    .as_ptr();
    for spec in &specs {
        unsafe { ffi::webrtc_AudioCodecSpec_vector_push_back(vec, spec.raw()) };
    }
    vec
}

unsafe extern "C" fn audio_decoder_factory_is_supported_decoder(
    format: *const ffi::webrtc_SdpAudioFormat,
    user_data: *mut c_void,
) -> i32 {
    assert!(
        !user_data.is_null(),
        "audio_decoder_factory_is_supported_decoder: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioDecoderFactoryHandlerState) };
    let format = expect_non_null(
        format.cast_mut(),
        "audio_decoder_factory_is_supported_decoder (format)",
    );
    let format = unsafe { SdpAudioFormatRef::from_raw(format) };
    if state.handler.is_supported_decoder(format) {
        1
    } else {
        0
    }
}

unsafe extern "C" fn audio_decoder_factory_create(
    env: *const ffi::webrtc_Environment,
    format: *const ffi::webrtc_SdpAudioFormat,
    user_data: *mut c_void,
) -> *mut ffi::webrtc_AudioDecoder_unique {
    assert!(
        !user_data.is_null(),
        "audio_decoder_factory_create: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut AudioDecoderFactoryHandlerState) };
    let env = expect_non_null(env.cast_mut(), "audio_decoder_factory_create (env)");
    let format = expect_non_null(format.cast_mut(), "audio_decoder_factory_create (format)");
    let env = unsafe { EnvironmentRef::from_raw(env) };
    let format = unsafe { SdpAudioFormatRef::from_raw(format) };
    match state.handler.create(env, format) {
        Some(decoder) => decoder.into_raw(),
        None => std::ptr::null_mut(),
    }
}

unsafe extern "C" fn audio_decoder_factory_on_destroy(user_data: *mut c_void) {
    unsafe {
        destroy_handler::<AudioDecoderFactoryHandlerState>(
            "audio_decoder_factory_on_destroy",
            user_data,
        )
    };
}

impl AudioDecoderFactory {
    /// ハンドラから AudioDecoderFactory を生成する。
    pub fn new_with_handler(handler: Box<dyn AudioDecoderFactoryHandler>) -> Self {
        let user_data = Box::into_raw(Box::new(HandlerState::new(handler))) as *mut c_void;
        let cbs = ffi::webrtc_AudioDecoderFactory_cbs {
            GetSupportedDecoders: Some(audio_decoder_factory_get_supported_decoders),
            IsSupportedDecoder: Some(audio_decoder_factory_is_supported_decoder),
            Create: Some(audio_decoder_factory_create),
            OnDestroy: Some(audio_decoder_factory_on_destroy),
        };
        let raw_ref = unsafe {
            create_with_handler::<AudioDecoderFactoryHandlerState, _>(
                "webrtc_AudioDecoderFactory_make_ref_counted",
                user_data,
                |user_data| ffi::webrtc_AudioDecoderFactory_make_ref_counted(&cbs, user_data),
            )
        };
        let raw_ref = ScopedRef::<AudioDecoderFactoryHandle>::from_raw(raw_ref);
        Self { raw_ref }
    }

    /// サポートされるデコーダーの一覧を返す。
    pub fn get_supported_decoders(&self) -> Vec<AudioCodecSpec> {
        let raw_vec =
            unsafe { ffi::webrtc_AudioDecoderFactory_GetSupportedDecoders(self.as_ptr()) };
        let raw_vec = expect_non_null(raw_vec, "webrtc_AudioDecoderFactory_GetSupportedDecoders");
        let size = unsafe { ffi::webrtc_AudioCodecSpec_vector_size(raw_vec.as_ptr()) };
        let mut specs = Vec::with_capacity(size.max(0) as usize);
        for i in 0..size {
            let raw = unsafe { ffi::webrtc_AudioCodecSpec_vector_get(raw_vec.as_ptr(), i) };
            let raw = expect_non_null(raw, "webrtc_AudioCodecSpec_vector_get");
            let copied = unsafe { ffi::webrtc_AudioCodecSpec_copy(raw.as_ptr()) };
            specs.push(AudioCodecSpec {
                raw: expect_non_null(copied, "webrtc_AudioCodecSpec_copy"),
            });
        }
        unsafe { ffi::webrtc_AudioCodecSpec_vector_delete(raw_vec.as_ptr()) };
        specs
    }

    /// デコーダーがフォーマットに対応するかを返す。
    pub fn is_supported_decoder(&self, format: SdpAudioFormatRef<'_>) -> bool {
        unsafe {
            ffi::webrtc_AudioDecoderFactory_IsSupportedDecoder(self.as_ptr(), format.as_ptr()) != 0
        }
    }

    /// デコーダーを生成する。
    pub fn create(
        &self,
        env: EnvironmentRef<'_>,
        format: SdpAudioFormatRef<'_>,
    ) -> Option<AudioDecoder> {
        let raw = unsafe {
            ffi::webrtc_AudioDecoderFactory_MakeAudioDecoder(
                self.as_ptr(),
                env.as_ptr(),
                format.as_ptr(),
            )
        };
        Some(AudioDecoder {
            raw_unique: NonNull::new(raw)?,
        })
    }
}
