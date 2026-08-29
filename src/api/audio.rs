use crate::handler::{HandlerState, create_with_handler, destroy_handler};
use crate::non_null::expect_non_null;
use crate::ref_count::{
    AudioDecoderFactoryHandle, AudioEncoderFactoryHandle, AudioTrackHandle, AudioTrackSourceHandle,
    MediaStreamTrackHandle,
};
use crate::{MediaStreamTrack, ScopedRef, ffi};
use std::os::raw::c_void;
use std::ptr::NonNull;

use super::optional::{get_optional, get_optional_bool, set_optional, set_optional_bool};

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

/// AudioTrackSink のコールバック状態の型。
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
