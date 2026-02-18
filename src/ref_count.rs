use crate::ffi;
use std::marker::PhantomData;
use std::ptr::NonNull;

/// webrtc の参照カウンタベースハンドル用トレイト。
///
/// # Safety
///
/// この参照カウンタ自体はスレッドセーフに操作しているので基本的には Send 可能となる。
/// ref: https://source.chromium.org/chromium/chromium/src/+/main:third_party/webrtc/rtc_base/ref_counter.h;l=25-52;drc=b610a104128a46d031dc5ae3e6d486430b61efa6
///
/// ただし全てのオブジェクトが他のスレッドに移動して正しく動作するとは限らないため、
/// RefCountedHandle を実装する型ごとに Send 可能かどうかを判断する必要がある。
#[allow(clippy::missing_safety_doc)]
pub trait RefCountedHandle {
    type Refcounted;
    type Raw;

    unsafe fn get(raw_ref: *mut Self::Refcounted) -> *mut Self::Raw;
    unsafe fn add_ref(raw: *mut Self::Raw);
    unsafe fn release(raw: *mut Self::Raw);
}

/// webrtc::scoped_refptr 相当の簡易ラッパー。
#[derive(Debug)]
pub struct ScopedRef<H: RefCountedHandle> {
    raw_ref: NonNull<H::Refcounted>,
    _marker: PhantomData<H>,
}

impl<H: RefCountedHandle> ScopedRef<H> {
    /// 生の refcounted ポインタから生成する。
    ///
    /// # Safety
    /// - `raw_ref` は有効な refcounted ポインタで、呼び出し元が所有権を持っていること。
    pub fn from_raw(raw_ref: NonNull<H::Refcounted>) -> Self {
        Self {
            raw_ref,
            _marker: PhantomData,
        }
    }

    pub fn as_refcounted_ptr(&self) -> *mut H::Refcounted {
        self.raw_ref.as_ptr()
    }

    pub fn as_ptr(&self) -> *mut H::Raw {
        self.raw().as_ptr()
    }

    pub(crate) fn raw(&self) -> NonNull<H::Raw> {
        let raw = unsafe { H::get(self.raw_ref.as_ptr()) };
        NonNull::new(raw).expect("BUG: RefCountedHandle::get が null を返しました")
    }
}

impl<H: RefCountedHandle> Clone for ScopedRef<H> {
    fn clone(&self) -> Self {
        let raw = self.raw();
        unsafe { H::add_ref(raw.as_ptr()) };
        Self {
            raw_ref: self.raw_ref,
            _marker: PhantomData,
        }
    }
}

impl<H: RefCountedHandle> Drop for ScopedRef<H> {
    fn drop(&mut self) {
        let raw = self.raw();
        unsafe { H::release(raw.as_ptr()) };
    }
}

// RefCountedHandle を実装する各ハンドル。
pub(crate) struct AudioDecoderFactoryHandle;
impl RefCountedHandle for AudioDecoderFactoryHandle {
    type Refcounted = ffi::webrtc_AudioDecoderFactory_refcounted;
    type Raw = ffi::webrtc_AudioDecoderFactory;

    unsafe fn get(raw_ref: *mut Self::Refcounted) -> *mut Self::Raw {
        unsafe { ffi::webrtc_AudioDecoderFactory_refcounted_get(raw_ref) }
    }
    unsafe fn add_ref(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_AudioDecoderFactory_AddRef(raw) };
    }
    unsafe fn release(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_AudioDecoderFactory_Release(raw) };
    }
}

pub(crate) struct AudioEncoderFactoryHandle;
impl RefCountedHandle for AudioEncoderFactoryHandle {
    type Refcounted = ffi::webrtc_AudioEncoderFactory_refcounted;
    type Raw = ffi::webrtc_AudioEncoderFactory;

    unsafe fn get(raw_ref: *mut Self::Refcounted) -> *mut Self::Raw {
        unsafe { ffi::webrtc_AudioEncoderFactory_refcounted_get(raw_ref) }
    }
    unsafe fn add_ref(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_AudioEncoderFactory_AddRef(raw) };
    }
    unsafe fn release(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_AudioEncoderFactory_Release(raw) };
    }
}

pub(crate) struct AudioDeviceModuleHandle;
impl RefCountedHandle for AudioDeviceModuleHandle {
    type Refcounted = ffi::webrtc_AudioDeviceModule_refcounted;
    type Raw = ffi::webrtc_AudioDeviceModule;

    unsafe fn get(raw_ref: *mut Self::Refcounted) -> *mut Self::Raw {
        unsafe { ffi::webrtc_AudioDeviceModule_refcounted_get(raw_ref) }
    }
    unsafe fn add_ref(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_AudioDeviceModule_AddRef(raw) };
    }
    unsafe fn release(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_AudioDeviceModule_Release(raw) };
    }
}

pub(crate) struct AudioTrackSourceHandle;
impl RefCountedHandle for AudioTrackSourceHandle {
    type Refcounted = ffi::webrtc_AudioSourceInterface_refcounted;
    type Raw = ffi::webrtc_AudioSourceInterface;

    unsafe fn get(raw_ref: *mut Self::Refcounted) -> *mut Self::Raw {
        unsafe { ffi::webrtc_AudioSourceInterface_refcounted_get(raw_ref) }
    }
    unsafe fn add_ref(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_AudioSourceInterface_AddRef(raw) };
    }
    unsafe fn release(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_AudioSourceInterface_Release(raw) };
    }
}

pub(crate) struct AudioTrackHandle;
impl RefCountedHandle for AudioTrackHandle {
    type Refcounted = ffi::webrtc_AudioTrackInterface_refcounted;
    type Raw = ffi::webrtc_AudioTrackInterface;

    unsafe fn get(raw_ref: *mut Self::Refcounted) -> *mut Self::Raw {
        unsafe { ffi::webrtc_AudioTrackInterface_refcounted_get(raw_ref) }
    }
    unsafe fn add_ref(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_AudioTrackInterface_AddRef(raw) };
    }
    unsafe fn release(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_AudioTrackInterface_Release(raw) };
    }
}

pub(crate) struct I420BufferHandle;
impl RefCountedHandle for I420BufferHandle {
    type Refcounted = ffi::webrtc_I420Buffer_refcounted;
    type Raw = ffi::webrtc_I420Buffer;

    unsafe fn get(raw_ref: *mut Self::Refcounted) -> *mut Self::Raw {
        unsafe { ffi::webrtc_I420Buffer_refcounted_get(raw_ref) }
    }
    unsafe fn add_ref(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_I420Buffer_AddRef(raw) };
    }
    unsafe fn release(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_I420Buffer_Release(raw) };
    }
}

pub(crate) struct EncodedImageBufferHandle;
impl RefCountedHandle for EncodedImageBufferHandle {
    type Refcounted = ffi::webrtc_EncodedImageBuffer_refcounted;
    type Raw = ffi::webrtc_EncodedImageBuffer;

    unsafe fn get(raw_ref: *mut Self::Refcounted) -> *mut Self::Raw {
        unsafe { ffi::webrtc_EncodedImageBuffer_refcounted_get(raw_ref) }
    }
    unsafe fn add_ref(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_EncodedImageBuffer_AddRef(raw) };
    }
    unsafe fn release(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_EncodedImageBuffer_Release(raw) };
    }
}

pub(crate) struct AdaptedVideoTrackSourceHandle;
impl RefCountedHandle for AdaptedVideoTrackSourceHandle {
    type Refcounted = ffi::webrtc_AdaptedVideoTrackSource_refcounted;
    type Raw = ffi::webrtc_AdaptedVideoTrackSource;

    unsafe fn get(raw_ref: *mut Self::Refcounted) -> *mut Self::Raw {
        unsafe { ffi::webrtc_AdaptedVideoTrackSource_refcounted_get(raw_ref) }
    }
    unsafe fn add_ref(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_AdaptedVideoTrackSource_AddRef(raw) };
    }
    unsafe fn release(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_AdaptedVideoTrackSource_Release(raw) };
    }
}

pub(crate) struct VideoTrackSourceHandle;
impl RefCountedHandle for VideoTrackSourceHandle {
    type Refcounted = ffi::webrtc_VideoTrackSourceInterface_refcounted;
    type Raw = ffi::webrtc_VideoTrackSourceInterface;

    unsafe fn get(raw_ref: *mut Self::Refcounted) -> *mut Self::Raw {
        unsafe { ffi::webrtc_VideoTrackSourceInterface_refcounted_get(raw_ref) }
    }
    unsafe fn add_ref(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_VideoTrackSourceInterface_AddRef(raw) };
    }
    unsafe fn release(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_VideoTrackSourceInterface_Release(raw) };
    }
}

pub(crate) struct VideoTrackHandle;
impl RefCountedHandle for VideoTrackHandle {
    type Refcounted = ffi::webrtc_VideoTrackInterface_refcounted;
    type Raw = ffi::webrtc_VideoTrackInterface;

    unsafe fn get(raw_ref: *mut Self::Refcounted) -> *mut Self::Raw {
        unsafe { ffi::webrtc_VideoTrackInterface_refcounted_get(raw_ref) }
    }
    unsafe fn add_ref(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_VideoTrackInterface_AddRef(raw) };
    }
    unsafe fn release(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_VideoTrackInterface_Release(raw) };
    }
}

pub(crate) struct MediaStreamTrackHandle;
impl RefCountedHandle for MediaStreamTrackHandle {
    type Refcounted = ffi::webrtc_MediaStreamTrackInterface_refcounted;
    type Raw = ffi::webrtc_MediaStreamTrackInterface;

    unsafe fn get(raw_ref: *mut Self::Refcounted) -> *mut Self::Raw {
        unsafe { ffi::webrtc_MediaStreamTrackInterface_refcounted_get(raw_ref) }
    }
    unsafe fn add_ref(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_MediaStreamTrackInterface_AddRef(raw) };
    }
    unsafe fn release(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_MediaStreamTrackInterface_Release(raw) };
    }
}

pub(crate) struct RtpReceiverHandle;
impl RefCountedHandle for RtpReceiverHandle {
    type Refcounted = ffi::webrtc_RtpReceiverInterface_refcounted;
    type Raw = ffi::webrtc_RtpReceiverInterface;

    unsafe fn get(raw_ref: *mut Self::Refcounted) -> *mut Self::Raw {
        unsafe { ffi::webrtc_RtpReceiverInterface_refcounted_get(raw_ref) }
    }
    unsafe fn add_ref(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_RtpReceiverInterface_AddRef(raw) };
    }
    unsafe fn release(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_RtpReceiverInterface_Release(raw) };
    }
}

pub(crate) struct RtpSenderHandle;
impl RefCountedHandle for RtpSenderHandle {
    type Refcounted = ffi::webrtc_RtpSenderInterface_refcounted;
    type Raw = ffi::webrtc_RtpSenderInterface;

    unsafe fn get(raw_ref: *mut Self::Refcounted) -> *mut Self::Raw {
        unsafe { ffi::webrtc_RtpSenderInterface_refcounted_get(raw_ref) }
    }
    unsafe fn add_ref(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_RtpSenderInterface_AddRef(raw) };
    }
    unsafe fn release(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_RtpSenderInterface_Release(raw) };
    }
}

pub(crate) struct PeerConnectionHandle;
impl RefCountedHandle for PeerConnectionHandle {
    type Refcounted = ffi::webrtc_PeerConnectionInterface_refcounted;
    type Raw = ffi::webrtc_PeerConnectionInterface;

    unsafe fn get(raw_ref: *mut Self::Refcounted) -> *mut Self::Raw {
        unsafe { ffi::webrtc_PeerConnectionInterface_refcounted_get(raw_ref) }
    }
    unsafe fn add_ref(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_PeerConnectionInterface_AddRef(raw) };
    }
    unsafe fn release(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_PeerConnectionInterface_Release(raw) };
    }
}

pub(crate) struct RTCStatsReportHandle;
impl RefCountedHandle for RTCStatsReportHandle {
    type Refcounted = ffi::webrtc_RTCStatsReport_refcounted;
    type Raw = ffi::webrtc_RTCStatsReport;

    unsafe fn get(raw_ref: *mut Self::Refcounted) -> *mut Self::Raw {
        unsafe { ffi::webrtc_RTCStatsReport_refcounted_get(raw_ref) }
    }
    unsafe fn add_ref(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_RTCStatsReport_AddRef(raw) };
    }
    unsafe fn release(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_RTCStatsReport_Release(raw) };
    }
}

pub(crate) struct RtpTransceiverHandle;
impl RefCountedHandle for RtpTransceiverHandle {
    type Refcounted = ffi::webrtc_RtpTransceiverInterface_refcounted;
    type Raw = ffi::webrtc_RtpTransceiverInterface;

    unsafe fn get(raw_ref: *mut Self::Refcounted) -> *mut Self::Raw {
        unsafe { ffi::webrtc_RtpTransceiverInterface_refcounted_get(raw_ref) }
    }
    unsafe fn add_ref(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_RtpTransceiverInterface_AddRef(raw) };
    }
    unsafe fn release(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_RtpTransceiverInterface_Release(raw) };
    }
}

pub(crate) struct SetLocalDescriptionObserverHandle;
impl RefCountedHandle for SetLocalDescriptionObserverHandle {
    type Refcounted = ffi::webrtc_SetLocalDescriptionObserverInterface_refcounted;
    type Raw = ffi::webrtc_SetLocalDescriptionObserverInterface;

    unsafe fn get(raw_ref: *mut Self::Refcounted) -> *mut Self::Raw {
        unsafe { ffi::webrtc_SetLocalDescriptionObserverInterface_refcounted_get(raw_ref) }
    }
    unsafe fn add_ref(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_SetLocalDescriptionObserverInterface_AddRef(raw) };
    }
    unsafe fn release(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_SetLocalDescriptionObserverInterface_Release(raw) };
    }
}

pub(crate) struct SetRemoteDescriptionObserverHandle;
impl RefCountedHandle for SetRemoteDescriptionObserverHandle {
    type Refcounted = ffi::webrtc_SetRemoteDescriptionObserverInterface_refcounted;
    type Raw = ffi::webrtc_SetRemoteDescriptionObserverInterface;

    unsafe fn get(raw_ref: *mut Self::Refcounted) -> *mut Self::Raw {
        unsafe { ffi::webrtc_SetRemoteDescriptionObserverInterface_refcounted_get(raw_ref) }
    }
    unsafe fn add_ref(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_SetRemoteDescriptionObserverInterface_AddRef(raw) };
    }
    unsafe fn release(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_SetRemoteDescriptionObserverInterface_Release(raw) };
    }
}

pub(crate) struct PeerConnectionFactoryHandle;
impl RefCountedHandle for PeerConnectionFactoryHandle {
    type Refcounted = ffi::webrtc_PeerConnectionFactoryInterface_refcounted;
    type Raw = ffi::webrtc_PeerConnectionFactoryInterface;

    unsafe fn get(raw_ref: *mut Self::Refcounted) -> *mut Self::Raw {
        unsafe { ffi::webrtc_PeerConnectionFactoryInterface_refcounted_get(raw_ref) }
    }
    unsafe fn add_ref(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_PeerConnectionFactoryInterface_AddRef(raw) };
    }
    unsafe fn release(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_PeerConnectionFactoryInterface_Release(raw) };
    }
}

pub(crate) struct DataChannelHandle;
impl RefCountedHandle for DataChannelHandle {
    type Refcounted = ffi::webrtc_DataChannelInterface_refcounted;
    type Raw = ffi::webrtc_DataChannelInterface;

    unsafe fn get(raw_ref: *mut Self::Refcounted) -> *mut Self::Raw {
        unsafe { ffi::webrtc_DataChannelInterface_refcounted_get(raw_ref) }
    }
    unsafe fn add_ref(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_DataChannelInterface_AddRef(raw) };
    }
    unsafe fn release(raw: *mut Self::Raw) {
        unsafe { ffi::webrtc_DataChannelInterface_Release(raw) };
    }
}
