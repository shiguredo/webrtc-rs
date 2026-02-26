use crate::ref_count::{
    AudioDecoderFactoryHandle, AudioEncoderFactoryHandle, AudioTrackHandle, AudioTrackSourceHandle,
    MediaStreamTrackHandle,
};
use crate::{MediaStreamTrack, ScopedRef, ffi};
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
