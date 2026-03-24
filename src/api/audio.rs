use crate::ref_count::{
    AudioDecoderFactoryHandle, AudioEncoderFactoryHandle, AudioTrackHandle, AudioTrackSourceHandle,
    MediaStreamTrackHandle,
};
use crate::{MediaStreamTrack, ScopedRef, ffi};
use std::os::raw::c_void;
use std::ptr::NonNull;

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

    /// AudioTrack に AudioSink を登録する。
    pub fn add_sink(&mut self, sink: &AudioSink) {
        unsafe {
            ffi::webrtc_AudioTrackInterface_AddSink(self.raw_ref.as_ptr(), sink.as_ptr());
        }
    }

    /// AudioTrack から AudioSink を解除する。
    pub fn remove_sink(&mut self, sink: &AudioSink) {
        unsafe {
            ffi::webrtc_AudioTrackInterface_RemoveSink(self.raw_ref.as_ptr(), sink.as_ptr());
        }
    }
}

unsafe impl Send for AudioTrack {}
// AudioTrackInterface の実体はシーケンシャルにする Proxy 経由で
// アクセスするためスレッドセーフに使用できる。
// ref: https://source.chromium.org/chromium/chromium/src/+/main:third_party/webrtc/pc/media_stream_track_proxy.h;l=26-40;drc=ef55be496e45889ace33ace4b05094ca19cb499b
unsafe impl Sync for AudioTrack {}

/// 音声データを受信するためのコールバックハンドラ。
pub trait AudioSinkHandler: Send {
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

struct AudioSinkHandlerState {
    handler: Box<dyn AudioSinkHandler>,
}

unsafe extern "C" fn audio_sink_on_data(
    audio_data: *const c_void,
    bits_per_sample: i32,
    sample_rate: i32,
    number_of_channels: usize,
    number_of_frames: usize,
    user_data: *mut c_void,
) {
    let state = unsafe { &mut *(user_data as *mut AudioSinkHandlerState) };
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

unsafe extern "C" fn audio_sink_on_destroy(user_data: *mut c_void) {
    assert!(
        !user_data.is_null(),
        "audio_sink_on_destroy: user_data is null"
    );
    let _ = unsafe { Box::from_raw(user_data as *mut AudioSinkHandlerState) };
}

/// webrtc::AudioTrackSinkInterface のラッパー。
pub struct AudioSink {
    raw: NonNull<ffi::webrtc_AudioTrackSinkInterface>,
}

impl AudioSink {
    pub fn new_with_handler(handler: Box<dyn AudioSinkHandler>) -> Self {
        let state = Box::new(AudioSinkHandlerState { handler });
        let user_data = Box::into_raw(state) as *mut c_void;
        let cbs = ffi::webrtc_AudioTrackSinkInterface_cbs {
            OnData: Some(audio_sink_on_data),
            OnDestroy: Some(audio_sink_on_destroy),
        };
        let raw =
            match NonNull::new(unsafe { ffi::webrtc_AudioTrackSinkInterface_new(&cbs, user_data) })
            {
                Some(raw) => raw,
                None => {
                    let _ = unsafe { Box::from_raw(user_data as *mut AudioSinkHandlerState) };
                    panic!("BUG: webrtc_AudioTrackSinkInterface_new が null を返しました");
                }
            };
        Self { raw }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_AudioTrackSinkInterface {
        self.raw.as_ptr()
    }
}

impl Drop for AudioSink {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_AudioTrackSinkInterface_delete(self.raw.as_ptr()) };
    }
}

unsafe impl Send for AudioSink {}

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
