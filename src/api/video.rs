use crate::ref_count::{
    AdaptedVideoTrackSourceHandle, I420BufferHandle, MediaStreamTrackHandle, VideoTrackHandle,
    VideoTrackSourceHandle,
};
use crate::{CxxStringRef, MapStringString, MediaStreamTrack, Result, ScopedRef, ffi};
use std::marker::PhantomData;
use std::os::raw::c_void;
use std::ptr::NonNull;
use std::slice;

/// webrtc::SdpVideoFormat のラッパー。
pub struct SdpVideoFormat {
    raw_unique: NonNull<ffi::webrtc_SdpVideoFormat_unique>,
}

impl SdpVideoFormat {
    pub fn new(name: &str) -> Self {
        let raw = unsafe {
            ffi::webrtc_SdpVideoFormat_new(
                name.as_ptr() as *const _,
                name.len(),
                std::ptr::null_mut(),
            )
        };
        Self {
            raw_unique: NonNull::new(raw)
                .expect("BUG: webrtc_SdpVideoFormat_new が null を返しました"),
        }
    }

    pub fn name(&self) -> Result<String> {
        let raw = self.raw();
        let ptr = unsafe { ffi::webrtc_SdpVideoFormat_get_name(raw.as_ptr()) };
        CxxStringRef::from_ptr(
            NonNull::new(ptr).expect("BUG: webrtc_SdpVideoFormat_get_name が null を返しました"),
        )
        .to_string()
    }

    pub fn parameters_mut(&mut self) -> MapStringString<'_> {
        let raw = self.raw();
        let ptr = unsafe { ffi::webrtc_SdpVideoFormat_get_parameters(raw.as_ptr()) };
        MapStringString::from_raw(NonNull::new(ptr).expect("BUF: ptr が null"))
    }

    pub fn is_equal(&self, other: &SdpVideoFormat) -> bool {
        let lhs = self.raw();
        let rhs = other.raw();
        unsafe { ffi::webrtc_SdpVideoFormat_is_equal(lhs.as_ptr(), rhs.as_ptr()) != 0 }
    }

    fn raw(&self) -> NonNull<ffi::webrtc_SdpVideoFormat> {
        let raw = unsafe { ffi::webrtc_SdpVideoFormat_unique_get(self.raw_unique.as_ptr()) };
        NonNull::new(raw).expect("BUG: webrtc_SdpVideoFormat_unique_get が null を返しました")
    }
}

impl Drop for SdpVideoFormat {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_SdpVideoFormat_unique_delete(self.raw_unique.as_ptr()) };
    }
}

/// webrtc::I420Buffer のラッパー。
pub struct I420Buffer {
    raw_ref: ScopedRef<I420BufferHandle>,
    width: i32,
    height: i32,
}

impl I420Buffer {
    pub fn new(width: i32, height: i32) -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_I420Buffer_Create(width, height) })
            .expect("BUG: webrtc_I420Buffer_Create が null を返しました");
        let raw_ref = ScopedRef::<I420BufferHandle>::from_raw(raw);
        Self {
            raw_ref,
            width,
            height,
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn stride_y(&self) -> i32 {
        let raw = self.raw();
        unsafe { ffi::webrtc_I420Buffer_StrideY(raw.as_ptr()) }
    }

    pub fn stride_u(&self) -> i32 {
        let raw = self.raw();
        unsafe { ffi::webrtc_I420Buffer_StrideU(raw.as_ptr()) }
    }

    pub fn stride_v(&self) -> i32 {
        let raw = self.raw();
        unsafe { ffi::webrtc_I420Buffer_StrideV(raw.as_ptr()) }
    }

    /// Y 平面を単一値で塗りつぶす。
    pub fn fill_y(&mut self, value: u8) {
        let ptr = unsafe { ffi::webrtc_I420Buffer_MutableDataY(self.raw().as_ptr()) };
        let stride = unsafe { ffi::webrtc_I420Buffer_StrideY(self.raw().as_ptr()) } as usize;
        let len = stride * self.height as usize;
        unsafe { slice::from_raw_parts_mut(ptr, len) }.fill(value);
    }

    /// U/V 平面を単一値で塗りつぶす。
    pub fn fill_uv(&mut self, u: u8, v: u8) {
        let raw = self.raw();
        let stride_u = unsafe { ffi::webrtc_I420Buffer_StrideU(raw.as_ptr()) } as usize;
        let stride_v = unsafe { ffi::webrtc_I420Buffer_StrideV(raw.as_ptr()) } as usize;
        let h = (self.height as usize).div_ceil(2);
        let ptr_u = unsafe { ffi::webrtc_I420Buffer_MutableDataU(raw.as_ptr()) };
        let ptr_v = unsafe { ffi::webrtc_I420Buffer_MutableDataV(raw.as_ptr()) };
        unsafe { slice::from_raw_parts_mut(ptr_u, stride_u * h) }.fill(u);
        unsafe { slice::from_raw_parts_mut(ptr_v, stride_v * h) }.fill(v);
    }

    /// 別の I420Buffer からスケールして埋める。
    pub fn scale_from(&mut self, src: &I420Buffer) {
        let raw = self.raw();
        let src_raw = src.raw();
        unsafe { ffi::webrtc_I420Buffer_ScaleFrom(raw.as_ptr(), src_raw.as_ptr()) };
    }

    /// Y 平面を参照する。
    pub fn y_data(&self) -> &[u8] {
        let raw = self.raw();
        let ptr = unsafe { ffi::webrtc_I420Buffer_MutableDataY(raw.as_ptr()) };
        let stride = unsafe { ffi::webrtc_I420Buffer_StrideY(raw.as_ptr()) } as usize;
        let len = stride * self.height as usize;
        unsafe { slice::from_raw_parts(ptr, len) }
    }

    /// U 平面を参照する。
    pub fn u_data(&self) -> &[u8] {
        let raw = self.raw();
        let ptr = unsafe { ffi::webrtc_I420Buffer_MutableDataU(raw.as_ptr()) };
        let stride = unsafe { ffi::webrtc_I420Buffer_StrideU(raw.as_ptr()) } as usize;
        let h = (self.height as usize).div_ceil(2);
        unsafe { slice::from_raw_parts(ptr, stride * h) }
    }

    /// V 平面を参照する。
    pub fn v_data(&self) -> &[u8] {
        let raw = self.raw();
        let ptr = unsafe { ffi::webrtc_I420Buffer_MutableDataV(raw.as_ptr()) };
        let stride = unsafe { ffi::webrtc_I420Buffer_StrideV(raw.as_ptr()) } as usize;
        let h = (self.height as usize).div_ceil(2);
        unsafe { slice::from_raw_parts(ptr, stride * h) }
    }

    pub fn as_refcounted_ptr(&self) -> *mut ffi::webrtc_I420Buffer_refcounted {
        self.raw_ref.as_refcounted_ptr()
    }

    pub(crate) fn raw(&self) -> NonNull<ffi::webrtc_I420Buffer> {
        self.raw_ref.raw()
    }
}

/// webrtc::VideoFrame のラッパー。
pub struct VideoFrame {
    raw_unique: NonNull<ffi::webrtc_VideoFrame_unique>,
}

impl VideoFrame {
    pub fn from_i420(buffer: &I420Buffer, timestamp_us: i64) -> Self {
        let raw = NonNull::new(unsafe {
            ffi::webrtc_VideoFrame_Create(
                buffer.as_refcounted_ptr(),
                ffi::webrtc_VideoRotation_0,
                timestamp_us,
            )
        })
        .expect("BUG: webrtc_VideoFrame_Create が null を返しました");
        Self { raw_unique: raw }
    }

    pub fn width(&self) -> i32 {
        let raw = self.raw();
        unsafe { ffi::webrtc_VideoFrame_width(raw.as_ptr()) }
    }

    pub fn height(&self) -> i32 {
        let raw = self.raw();
        unsafe { ffi::webrtc_VideoFrame_height(raw.as_ptr()) }
    }

    pub fn timestamp_us(&self) -> i64 {
        let raw = self.raw();
        unsafe { ffi::webrtc_VideoFrame_timestamp_us(raw.as_ptr()) }
    }

    /// I420Buffer を取得する。
    pub fn buffer(&self) -> I420Buffer {
        let raw = self.raw();
        let buf = NonNull::new(unsafe { ffi::webrtc_VideoFrame_video_frame_buffer(raw.as_ptr()) })
            .expect("BUG: webrtc_VideoFrame_video_frame_buffer が null を返しました");
        let raw_ref = ScopedRef::<I420BufferHandle>::from_raw(buf);
        I420Buffer {
            raw_ref,
            width: self.width(),
            height: self.height(),
        }
    }

    fn raw(&self) -> NonNull<ffi::webrtc_VideoFrame> {
        let raw = unsafe { ffi::webrtc_VideoFrame_unique_get(self.raw_unique.as_ptr()) };
        NonNull::new(raw).expect("BUG: webrtc_VideoFrame_unique_get が null を返しました")
    }
}

impl Drop for VideoFrame {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_VideoFrame_unique_delete(self.raw_unique.as_ptr()) };
    }
}

/// webrtc::VideoFrame の借用ラッパー。
pub struct VideoFrameRef<'a> {
    raw: NonNull<ffi::webrtc_VideoFrame>,
    _marker: PhantomData<&'a ffi::webrtc_VideoFrame>,
}

impl<'a> VideoFrameRef<'a> {
    /// C 側から渡された生ポインタを借用する。
    ///
    /// # Safety
    /// `raw` は有効な webrtc_VideoFrame ポインタであること。
    pub unsafe fn from_raw(raw: NonNull<ffi::webrtc_VideoFrame>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn width(&self) -> i32 {
        unsafe { ffi::webrtc_VideoFrame_width(self.raw.as_ptr()) }
    }

    pub fn height(&self) -> i32 {
        unsafe { ffi::webrtc_VideoFrame_height(self.raw.as_ptr()) }
    }

    pub fn timestamp_us(&self) -> i64 {
        unsafe { ffi::webrtc_VideoFrame_timestamp_us(self.raw.as_ptr()) }
    }

    pub fn buffer(&self) -> I420Buffer {
        let buf =
            NonNull::new(unsafe { ffi::webrtc_VideoFrame_video_frame_buffer(self.raw.as_ptr()) })
                .expect("BUG: webrtc_VideoFrame_video_frame_buffer が null を返しました");
        let raw_ref = ScopedRef::<I420BufferHandle>::from_raw(buf);
        I420Buffer {
            raw_ref,
            width: self.width(),
            height: self.height(),
        }
    }
}

struct VideoSinkCallbacks {
    on_frame: Box<dyn FnMut(VideoFrameRef) + Send + 'static>,
    on_discarded_frame: Option<Box<dyn FnMut() + Send + 'static>>,
}

unsafe extern "C" fn video_sink_on_frame(
    frame: *const ffi::webrtc_VideoFrame,
    user_data: *mut c_void,
) {
    let callbacks = unsafe { &mut *(user_data as *mut VideoSinkCallbacks) };
    let frame = NonNull::new(frame as *mut ffi::webrtc_VideoFrame).expect("BUG: frame が null");
    let frame = unsafe { VideoFrameRef::from_raw(frame) };
    (callbacks.on_frame)(frame);
}

unsafe extern "C" fn video_sink_on_discarded_frame(user_data: *mut c_void) {
    if user_data.is_null() {
        return;
    }
    let callbacks = unsafe { &mut *(user_data as *mut VideoSinkCallbacks) };
    if let Some(cb) = callbacks.on_discarded_frame.as_mut() {
        cb();
    }
}

/// webrtc::VideoSinkWants のラッパー。
pub struct VideoSinkWants {
    raw: NonNull<ffi::webrtc_VideoSinkWants>,
}

impl Default for VideoSinkWants {
    fn default() -> Self {
        Self::new()
    }
}

impl VideoSinkWants {
    pub fn new() -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_VideoSinkWants_new() })
            .expect("BUG: webrtc_VideoSinkWants_new が null を返しました");
        Self { raw }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_VideoSinkWants {
        self.raw.as_ptr()
    }
}

impl Drop for VideoSinkWants {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_VideoSinkWants_delete(self.raw.as_ptr()) };
    }
}

pub struct VideoSinkBuilder {
    on_frame: Box<dyn FnMut(VideoFrameRef) + Send + 'static>,
    on_discarded_frame: Option<Box<dyn FnMut() + Send + 'static>>,
}

impl VideoSinkBuilder {
    pub fn new<F>(on_frame: F) -> Self
    where
        F: FnMut(VideoFrameRef) + Send + 'static,
    {
        Self {
            on_frame: Box::new(on_frame),
            on_discarded_frame: None,
        }
    }

    pub fn on_discarded_frame<F>(self, on_discarded_frame: F) -> Self
    where
        F: FnMut() + Send + 'static,
    {
        Self {
            on_discarded_frame: Some(Box::new(on_discarded_frame)),
            ..self
        }
    }

    pub fn build(self) -> VideoSink {
        VideoSink::new(self.on_frame, self.on_discarded_frame)
    }
}

/// webrtc::VideoSinkInterface のラッパー。
pub struct VideoSink {
    raw: NonNull<ffi::webrtc_VideoSinkInterface>,
    _cbs: Box<ffi::webrtc_VideoSinkInterface_cbs>,
    _user_data: Box<VideoSinkCallbacks>,
}

impl VideoSink {
    fn new(
        on_frame: Box<dyn FnMut(VideoFrameRef) + Send + 'static>,
        on_discarded_frame: Option<Box<dyn FnMut() + Send + 'static>>,
    ) -> Self {
        let has_on_discarded = on_discarded_frame.is_some();
        let mut callbacks = Box::new(VideoSinkCallbacks {
            on_frame: Box::new(on_frame),
            on_discarded_frame,
        });
        let user_data = callbacks.as_mut() as *mut VideoSinkCallbacks as *mut c_void;
        let mut cbs = Box::new(ffi::webrtc_VideoSinkInterface_cbs {
            OnFrame: Some(video_sink_on_frame),
            OnDiscardedFrame: if has_on_discarded {
                Some(video_sink_on_discarded_frame)
            } else {
                None
            },
        });
        let cbs_ptr = cbs.as_mut() as *mut ffi::webrtc_VideoSinkInterface_cbs;
        let raw = NonNull::new(unsafe { ffi::webrtc_VideoSinkInterface_new(cbs_ptr, user_data) })
            .expect("BUG: webrtc_VideoSinkInterface_new が null を返しました");
        Self {
            raw,
            _cbs: cbs,
            _user_data: callbacks,
        }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_VideoSinkInterface {
        self.raw.as_ptr()
    }
}

impl Drop for VideoSink {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_VideoSinkInterface_delete(self.raw.as_ptr()) };
    }
}

unsafe impl Send for VideoSink {}

/// webrtc::AdaptedVideoTrackSource のラッパー。
pub struct AdaptedVideoTrackSource {
    raw_ref: ScopedRef<AdaptedVideoTrackSourceHandle>,
}

impl Default for AdaptedVideoTrackSource {
    fn default() -> Self {
        Self::new()
    }
}

impl AdaptedVideoTrackSource {
    pub fn new() -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_AdaptedVideoTrackSource_Create() })
            .expect("BUG: webrtc_AdaptedVideoTrackSource_Create が null を返しました");
        let raw_ref = ScopedRef::<AdaptedVideoTrackSourceHandle>::from_raw(raw);
        Self { raw_ref }
    }

    /// フレームをアダプトし、適用の有無と結果を返す。
    pub fn adapt_frame(&mut self, width: i32, height: i32, timestamp_us: i64) -> AdaptFrameResult {
        let raw = self.raw();
        let mut out = AdaptedSize::default();
        let ok = unsafe {
            ffi::webrtc_AdaptedVideoTrackSource_AdaptFrame(
                raw.as_ptr(),
                width,
                height,
                timestamp_us,
                &mut out.adapted_width,
                &mut out.adapted_height,
                &mut out.crop_width,
                &mut out.crop_height,
                &mut out.crop_x,
                &mut out.crop_y,
            )
        };
        AdaptFrameResult {
            applied: ok != 0,
            size: out,
        }
    }

    /// フレームをソースに投入する。
    pub fn on_frame(&mut self, frame: &VideoFrame) {
        let raw = self.raw();
        let frame_raw = frame.raw();
        unsafe { ffi::webrtc_AdaptedVideoTrackSource_OnFrame(raw.as_ptr(), frame_raw.as_ptr()) };
    }

    /// VideoTrackSourceInterface へキャストする。
    pub fn cast_to_video_track_source(&self) -> VideoTrackSource {
        let raw_ref = NonNull::new(unsafe {
            ffi::webrtc_AdaptedVideoTrackSource_refcounted_cast_to_webrtc_VideoTrackSourceInterface(
                self.raw_ref.as_refcounted_ptr(),
            )
        })
        .expect("BUG: webrtc_AdaptedVideoTrackSource_refcounted_cast_to_webrtc_VideoTrackSourceInterface が null を返しました");
        let raw_ref = ScopedRef::<VideoTrackSourceHandle>::from_raw(raw_ref);
        VideoTrackSource { raw_ref }
    }

    fn raw(&self) -> NonNull<ffi::webrtc_AdaptedVideoTrackSource> {
        self.raw_ref.raw()
    }
}

impl Clone for AdaptedVideoTrackSource {
    fn clone(&self) -> Self {
        Self {
            raw_ref: ScopedRef::clone(&self.raw_ref),
        }
    }
}

// WebRTC 側でスレッドセーフに設計されているため Send/Sync として扱う。
// ref: https://source.chromium.org/chromium/chromium/src/+/main:third_party/webrtc/media/base/adapted_video_track_source.h;l=33-36;drc=0bdeb7818cb6248017867b5e7d4e1cba33500dfc
unsafe impl Send for AdaptedVideoTrackSource {}
unsafe impl Sync for AdaptedVideoTrackSource {}

/// AdaptFrame の出力結果。
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct AdaptedSize {
    pub adapted_width: i32,
    pub adapted_height: i32,
    pub crop_width: i32,
    pub crop_height: i32,
    pub crop_x: i32,
    pub crop_y: i32,
}

/// AdaptFrame の結果。
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct AdaptFrameResult {
    pub applied: bool,
    pub size: AdaptedSize,
}

/// webrtc::VideoTrackSourceInterface のラッパー。
pub struct VideoTrackSource {
    raw_ref: ScopedRef<VideoTrackSourceHandle>,
}

impl VideoTrackSource {
    pub fn as_ptr(&self) -> *mut ffi::webrtc_VideoTrackSourceInterface {
        self.raw_ref.as_ptr()
    }

    pub fn as_refcounted_ptr(&self) -> *mut ffi::webrtc_VideoTrackSourceInterface_refcounted {
        self.raw_ref.as_refcounted_ptr()
    }
}

impl Clone for VideoTrackSource {
    fn clone(&self) -> Self {
        Self {
            raw_ref: ScopedRef::clone(&self.raw_ref),
        }
    }
}

unsafe impl Send for VideoTrackSource {}

/// webrtc::VideoTrackInterface のラッパー。
pub struct VideoTrack {
    raw_ref: ScopedRef<VideoTrackHandle>,
}

impl VideoTrack {
    pub(crate) fn from_scoped_ref(raw_ref: ScopedRef<VideoTrackHandle>) -> Self {
        Self { raw_ref }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_VideoTrackInterface {
        self.raw_ref.as_ptr()
    }

    pub fn as_refcounted_ptr(&self) -> *mut ffi::webrtc_VideoTrackInterface_refcounted {
        self.raw_ref.as_refcounted_ptr()
    }

    pub fn cast_to_media_stream_track(&self) -> MediaStreamTrack {
        let raw_ref = NonNull::new(unsafe {
            ffi::webrtc_VideoTrackInterface_refcounted_cast_to_webrtc_MediaStreamTrackInterface(
                self.raw_ref.as_refcounted_ptr(),
            )
        })
        .expect("BUG: webrtc_VideoTrackInterface_refcounted_cast_to_webrtc_MediaStreamTrackInterface が null を返しました");
        MediaStreamTrack::from_scoped_ref(ScopedRef::<MediaStreamTrackHandle>::from_raw(raw_ref))
    }

    pub fn add_or_update_sink(&self, sink: &VideoSink, wants: &VideoSinkWants) {
        unsafe {
            ffi::webrtc_VideoTrackInterface_AddOrUpdateSink(
                self.raw_ref.as_ptr(),
                sink.as_ptr(),
                wants.as_ptr(),
            );
        }
    }

    pub fn remove_sink(&self, sink: &VideoSink) {
        unsafe { ffi::webrtc_VideoTrackInterface_RemoveSink(self.raw_ref.as_ptr(), sink.as_ptr()) };
    }
}

impl Clone for VideoTrack {
    fn clone(&self) -> Self {
        Self {
            raw_ref: ScopedRef::clone(&self.raw_ref),
        }
    }
}

unsafe impl Send for VideoTrack {}
// VideoTracklInterface の実体はシーケンシャルにする Proxy 経由で
// アクセスするためスレッドセーフに使用できる。
// ref: https://source.chromium.org/chromium/chromium/src/+/main:third_party/webrtc/pc/media_stream_track_proxy.h;l=42-61;drc=984699a83cf8728b92819642d256ef14f1611792
unsafe impl Sync for VideoTrack {}

/// webrtc::VideoEncoderFactory のラッパー。
pub struct VideoEncoderFactory {
    raw_unique: NonNull<ffi::webrtc_VideoEncoderFactory_unique>,
}

impl VideoEncoderFactory {
    pub fn builtin() -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_CreateBuiltinVideoEncoderFactory() })
            .expect("webrtc_CreateBuiltinVideoEncoderFactory が null を返しました");
        Self { raw_unique: raw }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_VideoEncoderFactory {
        unsafe { ffi::webrtc_VideoEncoderFactory_unique_get(self.raw_unique.as_ptr()) }
    }

    pub fn into_raw(self) -> *mut ffi::webrtc_VideoEncoderFactory_unique {
        std::mem::ManuallyDrop::new(self).raw_unique.as_ptr()
    }
}

impl Drop for VideoEncoderFactory {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_VideoEncoderFactory_unique_delete(self.raw_unique.as_ptr()) };
    }
}

/// webrtc::VideoDecoderFactory のラッパー。
pub struct VideoDecoderFactory {
    raw_unique: NonNull<ffi::webrtc_VideoDecoderFactory_unique>,
}

impl VideoDecoderFactory {
    pub fn builtin() -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_CreateBuiltinVideoDecoderFactory() })
            .expect("webrtc_CreateBuiltinVideoDecoderFactory が null を返しました");
        Self { raw_unique: raw }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_VideoDecoderFactory {
        unsafe { ffi::webrtc_VideoDecoderFactory_unique_get(self.raw_unique.as_ptr()) }
    }

    pub fn into_raw(self) -> *mut ffi::webrtc_VideoDecoderFactory_unique {
        std::mem::ManuallyDrop::new(self).raw_unique.as_ptr()
    }
}

impl Drop for VideoDecoderFactory {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_VideoDecoderFactory_unique_delete(self.raw_unique.as_ptr()) };
    }
}
