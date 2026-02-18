use crate::ref_count::{
    AdaptedVideoTrackSourceHandle, EncodedImageBufferHandle, I420BufferHandle,
    MediaStreamTrackHandle, VideoTrackHandle, VideoTrackSourceHandle,
};
use crate::{
    CxxString, CxxStringRef, EnvironmentRef, MapStringString, MediaStreamTrack, Result, ScopedRef,
    ffi,
};
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
        self.as_ref().name()
    }

    pub fn parameters_mut(&mut self) -> MapStringString<'_> {
        self.as_ref().parameters_mut()
    }

    pub fn is_equal(&self, other: &SdpVideoFormat) -> bool {
        self.as_ref().is_equal(other.as_ref())
    }

    pub fn as_ref(&self) -> SdpVideoFormatRef<'_> {
        // Safety: self.raw() は SdpVideoFormat の生存中は常に有効です。
        unsafe { SdpVideoFormatRef::from_raw(self.raw()) }
    }

    fn raw(&self) -> NonNull<ffi::webrtc_SdpVideoFormat> {
        let raw = unsafe { ffi::webrtc_SdpVideoFormat_unique_get(self.raw_unique.as_ptr()) };
        NonNull::new(raw).expect("BUG: webrtc_SdpVideoFormat_unique_get が null を返しました")
    }
}

pub struct SdpVideoFormatRef<'a> {
    raw: NonNull<ffi::webrtc_SdpVideoFormat>,
    _marker: PhantomData<&'a ffi::webrtc_SdpVideoFormat>,
}

impl<'a> SdpVideoFormatRef<'a> {
    /// # Safety
    /// `raw` は有効な `webrtc_SdpVideoFormat` を指している必要があります。
    pub unsafe fn from_raw(raw: NonNull<ffi::webrtc_SdpVideoFormat>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn name(&self) -> Result<String> {
        let ptr = unsafe { ffi::webrtc_SdpVideoFormat_get_name(self.raw.as_ptr()) };
        CxxStringRef::from_ptr(
            NonNull::new(ptr).expect("BUG: webrtc_SdpVideoFormat_get_name が null を返しました"),
        )
        .to_string()
    }

    pub fn parameters_mut(&self) -> MapStringString<'a> {
        let ptr = unsafe { ffi::webrtc_SdpVideoFormat_get_parameters(self.raw.as_ptr()) };
        MapStringString::from_raw(NonNull::new(ptr).expect("BUG: ptr が null"))
    }

    pub fn is_equal(&self, other: SdpVideoFormatRef<'_>) -> bool {
        unsafe { ffi::webrtc_SdpVideoFormat_is_equal(self.raw.as_ptr(), other.raw.as_ptr()) != 0 }
    }
}

unsafe impl<'a> Send for SdpVideoFormatRef<'a> {}

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
        self.as_ref().width()
    }

    pub fn height(&self) -> i32 {
        self.as_ref().height()
    }

    pub fn timestamp_us(&self) -> i64 {
        self.as_ref().timestamp_us()
    }

    /// I420Buffer を取得する。
    pub fn buffer(&self) -> I420Buffer {
        self.as_ref().buffer()
    }

    pub fn as_ref(&self) -> VideoFrameRef<'_> {
        // Safety: self.raw() は VideoFrame の生存中は常に有効です。
        unsafe { VideoFrameRef::from_raw(self.raw()) }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoFrameType {
    Empty,
    Key,
    Delta,
    Unknown(i32),
}

impl VideoFrameType {
    fn from_raw(value: i32) -> Self {
        if value == unsafe { ffi::webrtc_VideoFrameType_Empty } {
            Self::Empty
        } else if value == unsafe { ffi::webrtc_VideoFrameType_Key } {
            Self::Key
        } else if value == unsafe { ffi::webrtc_VideoFrameType_Delta } {
            Self::Delta
        } else {
            Self::Unknown(value)
        }
    }

    fn to_raw(self) -> i32 {
        match self {
            Self::Empty => unsafe { ffi::webrtc_VideoFrameType_Empty },
            Self::Key => unsafe { ffi::webrtc_VideoFrameType_Key },
            Self::Delta => unsafe { ffi::webrtc_VideoFrameType_Delta },
            Self::Unknown(v) => v,
        }
    }
}

pub struct VideoFrameTypeVector {
    raw: NonNull<ffi::webrtc_VideoFrameType_vector>,
}

impl Default for VideoFrameTypeVector {
    fn default() -> Self {
        Self::new(0)
    }
}

impl VideoFrameTypeVector {
    pub fn new(size: i32) -> Self {
        let raw = unsafe { ffi::webrtc_VideoFrameType_vector_new(size) };
        Self {
            raw: NonNull::new(raw)
                .expect("BUG: webrtc_VideoFrameType_vector_new が null を返しました"),
        }
    }

    pub fn len(&self) -> usize {
        self.as_ref().len()
    }

    pub fn is_empty(&self) -> bool {
        self.as_ref().is_empty()
    }

    pub fn get(&self, index: usize) -> Option<VideoFrameType> {
        self.as_ref().get(index)
    }

    pub fn push(&mut self, value: VideoFrameType) {
        self.as_ref().push(value);
    }

    pub fn as_ref(&self) -> VideoFrameTypeVectorRef<'_> {
        unsafe { VideoFrameTypeVectorRef::from_raw(self.raw) }
    }
}

impl Drop for VideoFrameTypeVector {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_VideoFrameType_vector_delete(self.raw.as_ptr()) };
    }
}

unsafe impl Send for VideoFrameTypeVector {}

pub struct VideoFrameTypeVectorRef<'a> {
    raw: NonNull<ffi::webrtc_VideoFrameType_vector>,
    _marker: PhantomData<&'a ffi::webrtc_VideoFrameType_vector>,
}

impl<'a> VideoFrameTypeVectorRef<'a> {
    /// # Safety
    /// `raw` は有効な `webrtc_VideoFrameType_vector` を指している必要があります。
    pub unsafe fn from_raw(raw: NonNull<ffi::webrtc_VideoFrameType_vector>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        let len = unsafe { ffi::webrtc_VideoFrameType_vector_size(self.raw.as_ptr()) };
        len.max(0) as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: usize) -> Option<VideoFrameType> {
        if index >= self.len() {
            return None;
        }
        let raw = unsafe { ffi::webrtc_VideoFrameType_vector_get(self.raw.as_ptr(), index as i32) };
        let raw = NonNull::new(raw)?;
        let value = unsafe { ffi::webrtc_VideoFrameType_value(raw.as_ptr()) };
        Some(VideoFrameType::from_raw(value))
    }

    pub fn push(&self, value: VideoFrameType) {
        unsafe {
            ffi::webrtc_VideoFrameType_vector_push_back_value(self.raw.as_ptr(), value.to_raw())
        };
    }

    pub(crate) fn as_ptr(&self) -> *mut ffi::webrtc_VideoFrameType_vector {
        self.raw.as_ptr()
    }
}

unsafe impl<'a> Send for VideoFrameTypeVectorRef<'a> {}

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

pub struct VideoEncoderEncoderInfo {
    raw_unique: NonNull<ffi::webrtc_VideoEncoder_EncoderInfo_unique>,
}

impl Default for VideoEncoderEncoderInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl VideoEncoderEncoderInfo {
    pub fn new() -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_VideoEncoder_EncoderInfo_new() })
            .expect("webrtc_VideoEncoder_EncoderInfo_new が null を返しました");
        Self { raw_unique: raw }
    }

    pub fn implementation_name(&self) -> Result<String> {
        let raw =
            unsafe { ffi::webrtc_VideoEncoder_EncoderInfo_get_implementation_name(self.as_ptr()) };
        let raw = NonNull::new(raw)
            .expect("webrtc_VideoEncoder_EncoderInfo_get_implementation_name が null を返しました");
        CxxString::from_unique(raw).to_string()
    }

    pub fn set_implementation_name(&mut self, name: &str) {
        let name = CxxString::from_str(name);
        unsafe {
            ffi::webrtc_VideoEncoder_EncoderInfo_set_implementation_name(
                self.as_ptr(),
                name.into_raw(),
            );
        }
    }

    pub fn is_hardware_accelerated(&self) -> bool {
        unsafe {
            ffi::webrtc_VideoEncoder_EncoderInfo_get_is_hardware_accelerated(self.as_ptr()) != 0
        }
    }

    pub fn set_is_hardware_accelerated(&mut self, value: bool) {
        unsafe {
            ffi::webrtc_VideoEncoder_EncoderInfo_set_is_hardware_accelerated(
                self.as_ptr(),
                if value { 1 } else { 0 },
            );
        }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_VideoEncoder_EncoderInfo {
        unsafe { ffi::webrtc_VideoEncoder_EncoderInfo_unique_get(self.raw_unique.as_ptr()) }
    }

    pub fn into_raw(self) -> *mut ffi::webrtc_VideoEncoder_EncoderInfo_unique {
        std::mem::ManuallyDrop::new(self).raw_unique.as_ptr()
    }
}

impl Drop for VideoEncoderEncoderInfo {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_VideoEncoder_EncoderInfo_unique_delete(self.raw_unique.as_ptr()) };
    }
}

unsafe impl Send for VideoEncoderEncoderInfo {}

pub struct VideoDecoderDecoderInfo {
    raw_unique: NonNull<ffi::webrtc_VideoDecoder_DecoderInfo_unique>,
}

impl Default for VideoDecoderDecoderInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl VideoDecoderDecoderInfo {
    pub fn new() -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_VideoDecoder_DecoderInfo_new() })
            .expect("webrtc_VideoDecoder_DecoderInfo_new が null を返しました");
        Self { raw_unique: raw }
    }

    pub fn implementation_name(&self) -> Result<String> {
        let raw =
            unsafe { ffi::webrtc_VideoDecoder_DecoderInfo_get_implementation_name(self.as_ptr()) };
        let raw = NonNull::new(raw)
            .expect("webrtc_VideoDecoder_DecoderInfo_get_implementation_name が null を返しました");
        CxxString::from_unique(raw).to_string()
    }

    pub fn set_implementation_name(&mut self, name: &str) {
        let name = CxxString::from_str(name);
        unsafe {
            ffi::webrtc_VideoDecoder_DecoderInfo_set_implementation_name(
                self.as_ptr(),
                name.into_raw(),
            );
        }
    }

    pub fn is_hardware_accelerated(&self) -> bool {
        unsafe {
            ffi::webrtc_VideoDecoder_DecoderInfo_get_is_hardware_accelerated(self.as_ptr()) != 0
        }
    }

    pub fn set_is_hardware_accelerated(&mut self, value: bool) {
        unsafe {
            ffi::webrtc_VideoDecoder_DecoderInfo_set_is_hardware_accelerated(
                self.as_ptr(),
                if value { 1 } else { 0 },
            );
        }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_VideoDecoder_DecoderInfo {
        unsafe { ffi::webrtc_VideoDecoder_DecoderInfo_unique_get(self.raw_unique.as_ptr()) }
    }

    pub fn into_raw(self) -> *mut ffi::webrtc_VideoDecoder_DecoderInfo_unique {
        std::mem::ManuallyDrop::new(self).raw_unique.as_ptr()
    }
}

impl Drop for VideoDecoderDecoderInfo {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_VideoDecoder_DecoderInfo_unique_delete(self.raw_unique.as_ptr()) };
    }
}

unsafe impl Send for VideoDecoderDecoderInfo {}

pub struct VideoCodecRef<'a> {
    raw: NonNull<ffi::webrtc_VideoCodec>,
    _marker: PhantomData<&'a ffi::webrtc_VideoCodec>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoCodecType {
    Generic,
    Vp8,
    Vp9,
    Av1,
    H264,
    H265,
    Unknown(i32),
}

impl VideoCodecType {
    fn from_raw(value: i32) -> Self {
        if value == unsafe { ffi::webrtc_VideoCodecType_Generic } {
            Self::Generic
        } else if value == unsafe { ffi::webrtc_VideoCodecType_VP8 } {
            Self::Vp8
        } else if value == unsafe { ffi::webrtc_VideoCodecType_VP9 } {
            Self::Vp9
        } else if value == unsafe { ffi::webrtc_VideoCodecType_AV1 } {
            Self::Av1
        } else if value == unsafe { ffi::webrtc_VideoCodecType_H264 } {
            Self::H264
        } else if value == unsafe { ffi::webrtc_VideoCodecType_H265 } {
            Self::H265
        } else {
            Self::Unknown(value)
        }
    }

    fn to_raw(self) -> i32 {
        match self {
            Self::Generic => unsafe { ffi::webrtc_VideoCodecType_Generic },
            Self::Vp8 => unsafe { ffi::webrtc_VideoCodecType_VP8 },
            Self::Vp9 => unsafe { ffi::webrtc_VideoCodecType_VP9 },
            Self::Av1 => unsafe { ffi::webrtc_VideoCodecType_AV1 },
            Self::H264 => unsafe { ffi::webrtc_VideoCodecType_H264 },
            Self::H265 => unsafe { ffi::webrtc_VideoCodecType_H265 },
            Self::Unknown(v) => v,
        }
    }
}

impl<'a> VideoCodecRef<'a> {
    /// # Safety
    /// `raw` は有効な `webrtc_VideoCodec` を指している必要があります。
    pub unsafe fn from_raw(raw: NonNull<ffi::webrtc_VideoCodec>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn codec_type(&self) -> VideoCodecType {
        let value = unsafe { ffi::webrtc_VideoCodec_codec_type(self.raw.as_ptr()) };
        VideoCodecType::from_raw(value)
    }

    pub fn width(&self) -> i32 {
        unsafe { ffi::webrtc_VideoCodec_width(self.raw.as_ptr()) }
    }

    pub fn height(&self) -> i32 {
        unsafe { ffi::webrtc_VideoCodec_height(self.raw.as_ptr()) }
    }

    pub fn start_bitrate_kbps(&self) -> u32 {
        unsafe { ffi::webrtc_VideoCodec_start_bitrate_kbps(self.raw.as_ptr()) }
    }

    pub fn max_bitrate_kbps(&self) -> u32 {
        unsafe { ffi::webrtc_VideoCodec_max_bitrate_kbps(self.raw.as_ptr()) }
    }

    pub fn min_bitrate_kbps(&self) -> u32 {
        unsafe { ffi::webrtc_VideoCodec_min_bitrate_kbps(self.raw.as_ptr()) }
    }

    pub fn max_framerate(&self) -> u32 {
        unsafe { ffi::webrtc_VideoCodec_max_framerate(self.raw.as_ptr()) }
    }
}

unsafe impl<'a> Send for VideoCodecRef<'a> {}

pub struct VideoEncoderSettingsRef<'a> {
    raw: NonNull<ffi::webrtc_VideoEncoder_Settings>,
    _marker: PhantomData<&'a ffi::webrtc_VideoEncoder_Settings>,
}

impl<'a> VideoEncoderSettingsRef<'a> {
    /// # Safety
    /// `raw` は有効な `webrtc_VideoEncoder_Settings` を指している必要があります。
    pub unsafe fn from_raw(raw: NonNull<ffi::webrtc_VideoEncoder_Settings>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn number_of_cores(&self) -> i32 {
        unsafe { ffi::webrtc_VideoEncoder_Settings_number_of_cores(self.raw.as_ptr()) }
    }

    pub fn max_payload_size(&self) -> usize {
        unsafe { ffi::webrtc_VideoEncoder_Settings_max_payload_size(self.raw.as_ptr()) }
    }

    pub fn loss_notification(&self) -> bool {
        unsafe { ffi::webrtc_VideoEncoder_Settings_loss_notification(self.raw.as_ptr()) != 0 }
    }

    pub fn encoder_thread_limit(&self) -> Option<i32> {
        if unsafe { ffi::webrtc_VideoEncoder_Settings_has_encoder_thread_limit(self.raw.as_ptr()) }
            == 0
        {
            return None;
        }
        Some(unsafe { ffi::webrtc_VideoEncoder_Settings_encoder_thread_limit(self.raw.as_ptr()) })
    }
}

unsafe impl<'a> Send for VideoEncoderSettingsRef<'a> {}

pub struct VideoEncoderRateControlParametersRef<'a> {
    raw: NonNull<ffi::webrtc_VideoEncoder_RateControlParameters>,
    _marker: PhantomData<&'a ffi::webrtc_VideoEncoder_RateControlParameters>,
}

impl<'a> VideoEncoderRateControlParametersRef<'a> {
    /// # Safety
    /// `raw` は有効な `webrtc_VideoEncoder_RateControlParameters` を指している必要があります。
    pub unsafe fn from_raw(raw: NonNull<ffi::webrtc_VideoEncoder_RateControlParameters>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn framerate_fps(&self) -> f64 {
        unsafe { ffi::webrtc_VideoEncoder_RateControlParameters_framerate_fps(self.raw.as_ptr()) }
    }

    pub fn target_bitrate_sum_bps(&self) -> u32 {
        unsafe {
            ffi::webrtc_VideoEncoder_RateControlParameters_target_bitrate_sum_bps(self.raw.as_ptr())
        }
    }

    pub fn bitrate_sum_bps(&self) -> u32 {
        unsafe { ffi::webrtc_VideoEncoder_RateControlParameters_bitrate_sum_bps(self.raw.as_ptr()) }
    }

    pub fn bandwidth_allocation_bps(&self) -> i64 {
        unsafe {
            ffi::webrtc_VideoEncoder_RateControlParameters_bandwidth_allocation_bps(
                self.raw.as_ptr(),
            )
        }
    }
}

unsafe impl<'a> Send for VideoEncoderRateControlParametersRef<'a> {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VideoEncoderEncodedImageCallbackResultError {
    Ok,
    ErrorSendFailed,
}

impl VideoEncoderEncodedImageCallbackResultError {
    fn from_raw(value: i32) -> Self {
        if value == 0 {
            Self::Ok
        } else if value == 1 {
            Self::ErrorSendFailed
        } else {
            panic!(
                "BUG: 未知の EncodedImageCallback::Result::Error 値: {}",
                value
            );
        }
    }

    fn to_raw(self) -> i32 {
        match self {
            Self::Ok => 0,
            Self::ErrorSendFailed => 1,
        }
    }
}

pub struct VideoEncoderEncodedImageCallbackResult {
    raw_unique: NonNull<ffi::webrtc_VideoEncoder_EncodedImageCallback_Result_unique>,
}

impl VideoEncoderEncodedImageCallbackResult {
    pub fn new(error: VideoEncoderEncodedImageCallbackResultError) -> Self {
        let raw_unique =
            unsafe { ffi::webrtc_VideoEncoder_EncodedImageCallback_Result_new(error.to_raw()) };
        let raw_unique = NonNull::new(raw_unique).expect(
            "BUG: webrtc_VideoEncoder_EncodedImageCallback_Result_new が null を返しました",
        );
        Self { raw_unique }
    }

    pub fn new_with_frame_id(
        error: VideoEncoderEncodedImageCallbackResultError,
        frame_id: u32,
    ) -> Self {
        let raw_unique = unsafe {
            ffi::webrtc_VideoEncoder_EncodedImageCallback_Result_new_with_frame_id(
                error.to_raw(),
                frame_id,
            )
        };
        let raw_unique = NonNull::new(raw_unique).expect(
            "BUG: webrtc_VideoEncoder_EncodedImageCallback_Result_new_with_frame_id が null を返しました",
        );
        Self { raw_unique }
    }

    /// # Safety
    /// `raw_unique` は有効な `webrtc_VideoEncoder_EncodedImageCallback_Result_unique` を
    /// 指している必要があります。
    unsafe fn from_raw_unique(
        raw_unique: NonNull<ffi::webrtc_VideoEncoder_EncodedImageCallback_Result_unique>,
    ) -> Self {
        Self { raw_unique }
    }

    fn as_ptr(&self) -> *mut ffi::webrtc_VideoEncoder_EncodedImageCallback_Result {
        let raw = unsafe {
            ffi::webrtc_VideoEncoder_EncodedImageCallback_Result_unique_get(
                self.raw_unique.as_ptr(),
            )
        };
        NonNull::new(raw)
            .expect("BUG: webrtc_VideoEncoder_EncodedImageCallback_Result_unique_get が null を返しました")
            .as_ptr()
    }

    fn into_raw_unique(self) -> *mut ffi::webrtc_VideoEncoder_EncodedImageCallback_Result_unique {
        let ptr = self.raw_unique.as_ptr();
        std::mem::forget(self);
        ptr
    }

    pub fn error(&self) -> VideoEncoderEncodedImageCallbackResultError {
        let value =
            unsafe { ffi::webrtc_VideoEncoder_EncodedImageCallback_Result_error(self.as_ptr()) };
        VideoEncoderEncodedImageCallbackResultError::from_raw(value)
    }

    pub fn set_error(&mut self, error: VideoEncoderEncodedImageCallbackResultError) {
        unsafe {
            ffi::webrtc_VideoEncoder_EncodedImageCallback_Result_set_error(
                self.as_ptr(),
                error.to_raw(),
            )
        };
    }

    pub fn frame_id(&self) -> u32 {
        unsafe { ffi::webrtc_VideoEncoder_EncodedImageCallback_Result_frame_id(self.as_ptr()) }
    }

    pub fn set_frame_id(&mut self, frame_id: u32) {
        unsafe {
            ffi::webrtc_VideoEncoder_EncodedImageCallback_Result_set_frame_id(
                self.as_ptr(),
                frame_id,
            )
        };
    }

    pub fn drop_next_frame(&self) -> bool {
        unsafe {
            ffi::webrtc_VideoEncoder_EncodedImageCallback_Result_drop_next_frame(self.as_ptr()) != 0
        }
    }

    pub fn set_drop_next_frame(&mut self, drop_next_frame: bool) {
        unsafe {
            ffi::webrtc_VideoEncoder_EncodedImageCallback_Result_set_drop_next_frame(
                self.as_ptr(),
                if drop_next_frame { 1 } else { 0 },
            )
        };
    }
}

impl Drop for VideoEncoderEncodedImageCallbackResult {
    fn drop(&mut self) {
        unsafe {
            ffi::webrtc_VideoEncoder_EncodedImageCallback_Result_unique_delete(
                self.raw_unique.as_ptr(),
            )
        };
    }
}

unsafe impl Send for VideoEncoderEncodedImageCallbackResult {}

#[derive(Clone, Copy)]
pub struct VideoEncoderEncodedImageCallbackPtr {
    raw: NonNull<ffi::webrtc_VideoEncoder_EncodedImageCallback>,
}

impl VideoEncoderEncodedImageCallbackPtr {
    /// # Safety
    /// `callback` が指すオブジェクトは有効であり続ける必要があります。
    pub unsafe fn from_ref(callback: VideoEncoderEncodedImageCallbackRef<'_>) -> Self {
        Self { raw: callback.raw }
    }

    /// # Safety
    /// `raw` は有効な `webrtc_VideoEncoder_EncodedImageCallback` を指し、
    /// 呼び出し時点でも破棄されていない必要があります。
    pub unsafe fn from_raw(raw: NonNull<ffi::webrtc_VideoEncoder_EncodedImageCallback>) -> Self {
        Self { raw }
    }

    /// # Safety
    /// `self` が保持するポインタは有効である必要があります。
    /// `register` の再呼び出しや `release` 後に使ってはいけません。
    pub unsafe fn on_encoded_image(
        &self,
        image: EncodedImageRef<'_>,
        codec_specific_info: Option<CodecSpecificInfoRef<'_>>,
    ) -> VideoEncoderEncodedImageCallbackResult {
        let image = image.as_ptr();
        let codec_specific_info = codec_specific_info.map_or(std::ptr::null_mut(), |v| v.as_ptr());
        let raw_unique = unsafe {
            ffi::webrtc_VideoEncoder_EncodedImageCallback_OnEncodedImage(
                self.raw.as_ptr(),
                image,
                codec_specific_info,
            )
        };
        let raw_unique = NonNull::new(raw_unique).expect(
            "BUG: webrtc_VideoEncoder_EncodedImageCallback_OnEncodedImage が null を返しました",
        );
        unsafe { VideoEncoderEncodedImageCallbackResult::from_raw_unique(raw_unique) }
    }
}

unsafe impl Send for VideoEncoderEncodedImageCallbackPtr {}

pub struct VideoDecoderSettingsRef<'a> {
    raw: NonNull<ffi::webrtc_VideoDecoder_Settings>,
    _marker: PhantomData<&'a ffi::webrtc_VideoDecoder_Settings>,
}

impl<'a> VideoDecoderSettingsRef<'a> {
    /// # Safety
    /// `raw` は有効な `webrtc_VideoDecoder_Settings` を指している必要があります。
    pub unsafe fn from_raw(raw: NonNull<ffi::webrtc_VideoDecoder_Settings>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn number_of_cores(&self) -> i32 {
        unsafe { ffi::webrtc_VideoDecoder_Settings_number_of_cores(self.raw.as_ptr()) }
    }

    pub fn codec_type(&self) -> VideoCodecType {
        let value = unsafe { ffi::webrtc_VideoDecoder_Settings_codec_type(self.raw.as_ptr()) };
        VideoCodecType::from_raw(value)
    }

    pub fn buffer_pool_size(&self) -> Option<i32> {
        if unsafe { ffi::webrtc_VideoDecoder_Settings_has_buffer_pool_size(self.raw.as_ptr()) } == 0
        {
            return None;
        }
        Some(unsafe { ffi::webrtc_VideoDecoder_Settings_buffer_pool_size(self.raw.as_ptr()) })
    }

    pub fn max_render_resolution_width(&self) -> i32 {
        unsafe { ffi::webrtc_VideoDecoder_Settings_max_render_resolution_width(self.raw.as_ptr()) }
    }

    pub fn max_render_resolution_height(&self) -> i32 {
        unsafe { ffi::webrtc_VideoDecoder_Settings_max_render_resolution_height(self.raw.as_ptr()) }
    }
}

unsafe impl<'a> Send for VideoDecoderSettingsRef<'a> {}

pub struct EncodedImageBuffer {
    raw_ref: ScopedRef<EncodedImageBufferHandle>,
}

impl EncodedImageBuffer {
    pub fn from_bytes(data: &[u8]) -> Self {
        let raw_ref = if data.is_empty() {
            NonNull::new(unsafe { ffi::webrtc_EncodedImageBuffer_Create() })
                .expect("BUG: webrtc_EncodedImageBuffer_Create が null を返しました")
        } else {
            NonNull::new(unsafe {
                ffi::webrtc_EncodedImageBuffer_Create_from_data(data.as_ptr(), data.len())
            })
            .expect("BUG: webrtc_EncodedImageBuffer_Create_from_data が null を返しました")
        };
        Self {
            raw_ref: ScopedRef::<EncodedImageBufferHandle>::from_raw(raw_ref),
        }
    }

    fn from_raw_ref(raw_ref: NonNull<ffi::webrtc_EncodedImageBuffer_refcounted>) -> Self {
        Self {
            raw_ref: ScopedRef::<EncodedImageBufferHandle>::from_raw(raw_ref),
        }
    }

    pub fn data(&self) -> &[u8] {
        let size = unsafe { ffi::webrtc_EncodedImageBuffer_size(self.as_ptr()) };
        let ptr = unsafe { ffi::webrtc_EncodedImageBuffer_data(self.as_ptr()) };
        assert!(
            !(size > 0 && ptr.is_null()),
            "BUG: EncodedImageBuffer の size > 0 なのに data が null です"
        );
        if size == 0 || ptr.is_null() {
            return &[];
        }
        unsafe { slice::from_raw_parts(ptr, size) }
    }

    fn as_ptr(&self) -> *mut ffi::webrtc_EncodedImageBuffer {
        self.raw_ref.as_ptr()
    }
}

unsafe impl Send for EncodedImageBuffer {}

pub struct EncodedImage {
    raw_unique: NonNull<ffi::webrtc_EncodedImage_unique>,
}

impl Default for EncodedImage {
    fn default() -> Self {
        Self::new()
    }
}

impl EncodedImage {
    pub fn new() -> Self {
        let raw_unique = NonNull::new(unsafe { ffi::webrtc_EncodedImage_new() })
            .expect("BUG: webrtc_EncodedImage_new が null を返しました");
        Self { raw_unique }
    }

    pub fn set_encoded_data(&mut self, encoded_data: &EncodedImageBuffer) {
        self.as_ref().set_encoded_data(encoded_data);
    }

    pub fn set_rtp_timestamp(&mut self, rtp_timestamp: u32) {
        self.as_ref().set_rtp_timestamp(rtp_timestamp);
    }

    pub fn set_encoded_width(&mut self, encoded_width: u32) {
        self.as_ref().set_encoded_width(encoded_width);
    }

    pub fn set_encoded_height(&mut self, encoded_height: u32) {
        self.as_ref().set_encoded_height(encoded_height);
    }

    pub fn set_frame_type(&mut self, frame_type: VideoFrameType) {
        self.as_ref().set_frame_type(frame_type);
    }

    pub fn set_qp(&mut self, qp: i32) {
        self.as_ref().set_qp(qp);
    }

    pub fn as_ref(&self) -> EncodedImageRef<'_> {
        unsafe { EncodedImageRef::from_raw(self.raw()) }
    }

    pub fn encoded_data(&self) -> Option<EncodedImageBuffer> {
        self.as_ref().encoded_data()
    }

    pub fn rtp_timestamp(&self) -> u32 {
        self.as_ref().rtp_timestamp()
    }

    pub fn encoded_width(&self) -> u32 {
        self.as_ref().encoded_width()
    }

    pub fn encoded_height(&self) -> u32 {
        self.as_ref().encoded_height()
    }

    pub fn frame_type(&self) -> VideoFrameType {
        self.as_ref().frame_type()
    }

    pub fn qp(&self) -> i32 {
        self.as_ref().qp()
    }

    fn raw(&self) -> NonNull<ffi::webrtc_EncodedImage> {
        let raw = unsafe { ffi::webrtc_EncodedImage_unique_get(self.raw_unique.as_ptr()) };
        NonNull::new(raw).expect("BUG: webrtc_EncodedImage_unique_get が null を返しました")
    }
}

impl Drop for EncodedImage {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_EncodedImage_unique_delete(self.raw_unique.as_ptr()) };
    }
}

unsafe impl Send for EncodedImage {}

pub struct EncodedImageRef<'a> {
    raw: NonNull<ffi::webrtc_EncodedImage>,
    _marker: PhantomData<&'a ffi::webrtc_EncodedImage>,
}

impl<'a> EncodedImageRef<'a> {
    /// # Safety
    /// `raw` は有効な `webrtc_EncodedImage` を指している必要があります。
    pub unsafe fn from_raw(raw: NonNull<ffi::webrtc_EncodedImage>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn encoded_data(&self) -> Option<EncodedImageBuffer> {
        let raw_ref = unsafe { ffi::webrtc_EncodedImage_encoded_data(self.raw.as_ptr()) };
        let raw_ref = NonNull::new(raw_ref)?;
        Some(EncodedImageBuffer::from_raw_ref(raw_ref))
    }

    pub fn set_encoded_data(&self, encoded_data: &EncodedImageBuffer) {
        unsafe { ffi::webrtc_EncodedImage_set_encoded_data(self.raw.as_ptr(), encoded_data.as_ptr()) };
    }

    pub fn set_rtp_timestamp(&self, rtp_timestamp: u32) {
        unsafe { ffi::webrtc_EncodedImage_set_rtp_timestamp(self.raw.as_ptr(), rtp_timestamp) };
    }

    pub fn set_encoded_width(&self, encoded_width: u32) {
        unsafe { ffi::webrtc_EncodedImage_set_encoded_width(self.raw.as_ptr(), encoded_width) };
    }

    pub fn set_encoded_height(&self, encoded_height: u32) {
        unsafe { ffi::webrtc_EncodedImage_set_encoded_height(self.raw.as_ptr(), encoded_height) };
    }

    pub fn set_frame_type(&self, frame_type: VideoFrameType) {
        unsafe { ffi::webrtc_EncodedImage_set_frame_type(self.raw.as_ptr(), frame_type.to_raw()) };
    }

    pub fn set_qp(&self, qp: i32) {
        unsafe { ffi::webrtc_EncodedImage_set_qp(self.raw.as_ptr(), qp) };
    }

    pub fn rtp_timestamp(&self) -> u32 {
        unsafe { ffi::webrtc_EncodedImage_rtp_timestamp(self.raw.as_ptr()) }
    }

    pub fn encoded_width(&self) -> u32 {
        unsafe { ffi::webrtc_EncodedImage_encoded_width(self.raw.as_ptr()) }
    }

    pub fn encoded_height(&self) -> u32 {
        unsafe { ffi::webrtc_EncodedImage_encoded_height(self.raw.as_ptr()) }
    }

    pub fn frame_type(&self) -> VideoFrameType {
        let value = unsafe { ffi::webrtc_EncodedImage_frame_type(self.raw.as_ptr()) };
        VideoFrameType::from_raw(value)
    }

    pub fn qp(&self) -> i32 {
        unsafe { ffi::webrtc_EncodedImage_qp(self.raw.as_ptr()) }
    }

    pub(crate) fn as_ptr(&self) -> *mut ffi::webrtc_EncodedImage {
        self.raw.as_ptr()
    }
}

unsafe impl<'a> Send for EncodedImageRef<'a> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum H264PacketizationMode {
    NonInterleaved,
    SingleNalUnit,
    Unknown(i32),
}

impl H264PacketizationMode {
    fn from_raw(value: i32) -> Self {
        if value == unsafe { ffi::webrtc_H264PacketizationMode_NonInterleaved } {
            Self::NonInterleaved
        } else if value == unsafe { ffi::webrtc_H264PacketizationMode_SingleNalUnit } {
            Self::SingleNalUnit
        } else {
            Self::Unknown(value)
        }
    }

    fn to_raw(self) -> i32 {
        match self {
            Self::NonInterleaved => unsafe { ffi::webrtc_H264PacketizationMode_NonInterleaved },
            Self::SingleNalUnit => unsafe { ffi::webrtc_H264PacketizationMode_SingleNalUnit },
            Self::Unknown(v) => v,
        }
    }
}

pub struct CodecSpecificInfoRef<'a> {
    raw: NonNull<ffi::webrtc_CodecSpecificInfo>,
    _marker: PhantomData<&'a ffi::webrtc_CodecSpecificInfo>,
}

impl<'a> CodecSpecificInfoRef<'a> {
    /// # Safety
    /// `raw` は有効な `webrtc_CodecSpecificInfo` を指している必要があります。
    pub unsafe fn from_raw(raw: NonNull<ffi::webrtc_CodecSpecificInfo>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn codec_type(&self) -> VideoCodecType {
        let value = unsafe { ffi::webrtc_CodecSpecificInfo_codec_type(self.raw.as_ptr()) };
        VideoCodecType::from_raw(value)
    }

    pub fn end_of_picture(&self) -> bool {
        unsafe { ffi::webrtc_CodecSpecificInfo_end_of_picture(self.raw.as_ptr()) != 0 }
    }

    pub fn vp8_non_reference(&self) -> bool {
        unsafe { ffi::webrtc_CodecSpecificInfo_vp8_non_reference(self.raw.as_ptr()) != 0 }
    }

    pub fn vp8_temporal_idx(&self) -> i32 {
        unsafe { ffi::webrtc_CodecSpecificInfo_vp8_temporal_idx(self.raw.as_ptr()) }
    }

    pub fn vp8_layer_sync(&self) -> bool {
        unsafe { ffi::webrtc_CodecSpecificInfo_vp8_layer_sync(self.raw.as_ptr()) != 0 }
    }

    pub fn vp8_key_idx(&self) -> i32 {
        unsafe { ffi::webrtc_CodecSpecificInfo_vp8_key_idx(self.raw.as_ptr()) }
    }

    pub fn vp9_temporal_idx(&self) -> i32 {
        unsafe { ffi::webrtc_CodecSpecificInfo_vp9_temporal_idx(self.raw.as_ptr()) }
    }

    pub fn vp9_inter_pic_predicted(&self) -> bool {
        unsafe { ffi::webrtc_CodecSpecificInfo_vp9_inter_pic_predicted(self.raw.as_ptr()) != 0 }
    }

    pub fn vp9_flexible_mode(&self) -> bool {
        unsafe { ffi::webrtc_CodecSpecificInfo_vp9_flexible_mode(self.raw.as_ptr()) != 0 }
    }

    pub fn vp9_inter_layer_predicted(&self) -> bool {
        unsafe { ffi::webrtc_CodecSpecificInfo_vp9_inter_layer_predicted(self.raw.as_ptr()) != 0 }
    }

    pub fn h264_packetization_mode(&self) -> H264PacketizationMode {
        let value =
            unsafe { ffi::webrtc_CodecSpecificInfo_h264_packetization_mode(self.raw.as_ptr()) };
        H264PacketizationMode::from_raw(value)
    }

    pub fn h264_temporal_idx(&self) -> i32 {
        unsafe { ffi::webrtc_CodecSpecificInfo_h264_temporal_idx(self.raw.as_ptr()) }
    }

    pub fn h264_base_layer_sync(&self) -> bool {
        unsafe { ffi::webrtc_CodecSpecificInfo_h264_base_layer_sync(self.raw.as_ptr()) != 0 }
    }

    pub fn h264_idr_frame(&self) -> bool {
        unsafe { ffi::webrtc_CodecSpecificInfo_h264_idr_frame(self.raw.as_ptr()) != 0 }
    }

    pub fn set_codec_type(&self, codec_type: VideoCodecType) {
        unsafe { ffi::webrtc_CodecSpecificInfo_set_codec_type(self.raw.as_ptr(), codec_type.to_raw()) };
    }

    pub fn set_end_of_picture(&self, end_of_picture: bool) {
        unsafe {
            ffi::webrtc_CodecSpecificInfo_set_end_of_picture(
                self.raw.as_ptr(),
                if end_of_picture { 1 } else { 0 },
            )
        };
    }

    pub fn set_vp8_non_reference(&self, non_reference: bool) {
        unsafe {
            ffi::webrtc_CodecSpecificInfo_set_vp8_non_reference(
                self.raw.as_ptr(),
                if non_reference { 1 } else { 0 },
            )
        };
    }

    pub fn set_vp8_temporal_idx(&self, temporal_idx: i32) {
        unsafe { ffi::webrtc_CodecSpecificInfo_set_vp8_temporal_idx(self.raw.as_ptr(), temporal_idx) };
    }

    pub fn set_vp8_layer_sync(&self, layer_sync: bool) {
        unsafe {
            ffi::webrtc_CodecSpecificInfo_set_vp8_layer_sync(
                self.raw.as_ptr(),
                if layer_sync { 1 } else { 0 },
            )
        };
    }

    pub fn set_vp8_key_idx(&self, key_idx: i32) {
        unsafe { ffi::webrtc_CodecSpecificInfo_set_vp8_key_idx(self.raw.as_ptr(), key_idx) };
    }

    pub fn set_vp9_temporal_idx(&self, temporal_idx: i32) {
        unsafe { ffi::webrtc_CodecSpecificInfo_set_vp9_temporal_idx(self.raw.as_ptr(), temporal_idx) };
    }

    pub fn set_vp9_inter_pic_predicted(&self, inter_pic_predicted: bool) {
        unsafe {
            ffi::webrtc_CodecSpecificInfo_set_vp9_inter_pic_predicted(
                self.raw.as_ptr(),
                if inter_pic_predicted { 1 } else { 0 },
            )
        };
    }

    pub fn set_vp9_flexible_mode(&self, flexible_mode: bool) {
        unsafe {
            ffi::webrtc_CodecSpecificInfo_set_vp9_flexible_mode(
                self.raw.as_ptr(),
                if flexible_mode { 1 } else { 0 },
            )
        };
    }

    pub fn set_vp9_inter_layer_predicted(&self, inter_layer_predicted: bool) {
        unsafe {
            ffi::webrtc_CodecSpecificInfo_set_vp9_inter_layer_predicted(
                self.raw.as_ptr(),
                if inter_layer_predicted { 1 } else { 0 },
            )
        };
    }

    pub fn set_h264_packetization_mode(&self, packetization_mode: H264PacketizationMode) {
        unsafe {
            ffi::webrtc_CodecSpecificInfo_set_h264_packetization_mode(
                self.raw.as_ptr(),
                packetization_mode.to_raw(),
            )
        };
    }

    pub fn set_h264_temporal_idx(&self, temporal_idx: i32) {
        unsafe { ffi::webrtc_CodecSpecificInfo_set_h264_temporal_idx(self.raw.as_ptr(), temporal_idx) };
    }

    pub fn set_h264_base_layer_sync(&self, base_layer_sync: bool) {
        unsafe {
            ffi::webrtc_CodecSpecificInfo_set_h264_base_layer_sync(
                self.raw.as_ptr(),
                if base_layer_sync { 1 } else { 0 },
            )
        };
    }

    pub fn set_h264_idr_frame(&self, idr_frame: bool) {
        unsafe {
            ffi::webrtc_CodecSpecificInfo_set_h264_idr_frame(
                self.raw.as_ptr(),
                if idr_frame { 1 } else { 0 },
            )
        };
    }

    pub(crate) fn as_ptr(&self) -> *mut ffi::webrtc_CodecSpecificInfo {
        self.raw.as_ptr()
    }
}

unsafe impl<'a> Send for CodecSpecificInfoRef<'a> {}

pub struct CodecSpecificInfo {
    raw_unique: NonNull<ffi::webrtc_CodecSpecificInfo_unique>,
}

impl Default for CodecSpecificInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl CodecSpecificInfo {
    pub fn new() -> Self {
        let raw_unique = NonNull::new(unsafe { ffi::webrtc_CodecSpecificInfo_new() })
            .expect("BUG: webrtc_CodecSpecificInfo_new が null を返しました");
        Self { raw_unique }
    }

    pub fn codec_type(&self) -> VideoCodecType {
        self.as_ref().codec_type()
    }

    pub fn end_of_picture(&self) -> bool {
        self.as_ref().end_of_picture()
    }

    pub fn vp8_non_reference(&self) -> bool {
        self.as_ref().vp8_non_reference()
    }

    pub fn vp8_temporal_idx(&self) -> i32 {
        self.as_ref().vp8_temporal_idx()
    }

    pub fn vp8_layer_sync(&self) -> bool {
        self.as_ref().vp8_layer_sync()
    }

    pub fn vp8_key_idx(&self) -> i32 {
        self.as_ref().vp8_key_idx()
    }

    pub fn vp9_temporal_idx(&self) -> i32 {
        self.as_ref().vp9_temporal_idx()
    }

    pub fn vp9_inter_pic_predicted(&self) -> bool {
        self.as_ref().vp9_inter_pic_predicted()
    }

    pub fn vp9_flexible_mode(&self) -> bool {
        self.as_ref().vp9_flexible_mode()
    }

    pub fn vp9_inter_layer_predicted(&self) -> bool {
        self.as_ref().vp9_inter_layer_predicted()
    }

    pub fn h264_packetization_mode(&self) -> H264PacketizationMode {
        self.as_ref().h264_packetization_mode()
    }

    pub fn h264_temporal_idx(&self) -> i32 {
        self.as_ref().h264_temporal_idx()
    }

    pub fn h264_base_layer_sync(&self) -> bool {
        self.as_ref().h264_base_layer_sync()
    }

    pub fn h264_idr_frame(&self) -> bool {
        self.as_ref().h264_idr_frame()
    }

    pub fn set_codec_type(&mut self, codec_type: VideoCodecType) {
        self.as_ref().set_codec_type(codec_type);
    }

    pub fn set_end_of_picture(&mut self, end_of_picture: bool) {
        self.as_ref().set_end_of_picture(end_of_picture);
    }

    pub fn set_vp8_non_reference(&mut self, non_reference: bool) {
        self.as_ref().set_vp8_non_reference(non_reference);
    }

    pub fn set_vp8_temporal_idx(&mut self, temporal_idx: i32) {
        self.as_ref().set_vp8_temporal_idx(temporal_idx);
    }

    pub fn set_vp8_layer_sync(&mut self, layer_sync: bool) {
        self.as_ref().set_vp8_layer_sync(layer_sync);
    }

    pub fn set_vp8_key_idx(&mut self, key_idx: i32) {
        self.as_ref().set_vp8_key_idx(key_idx);
    }

    pub fn set_vp9_temporal_idx(&mut self, temporal_idx: i32) {
        self.as_ref().set_vp9_temporal_idx(temporal_idx);
    }

    pub fn set_vp9_inter_pic_predicted(&mut self, inter_pic_predicted: bool) {
        self.as_ref()
            .set_vp9_inter_pic_predicted(inter_pic_predicted);
    }

    pub fn set_vp9_flexible_mode(&mut self, flexible_mode: bool) {
        self.as_ref().set_vp9_flexible_mode(flexible_mode);
    }

    pub fn set_vp9_inter_layer_predicted(&mut self, inter_layer_predicted: bool) {
        self.as_ref()
            .set_vp9_inter_layer_predicted(inter_layer_predicted);
    }

    pub fn set_h264_packetization_mode(&mut self, packetization_mode: H264PacketizationMode) {
        self.as_ref().set_h264_packetization_mode(packetization_mode);
    }

    pub fn set_h264_temporal_idx(&mut self, temporal_idx: i32) {
        self.as_ref().set_h264_temporal_idx(temporal_idx);
    }

    pub fn set_h264_base_layer_sync(&mut self, base_layer_sync: bool) {
        self.as_ref().set_h264_base_layer_sync(base_layer_sync);
    }

    pub fn set_h264_idr_frame(&mut self, idr_frame: bool) {
        self.as_ref().set_h264_idr_frame(idr_frame);
    }

    pub fn as_ref(&self) -> CodecSpecificInfoRef<'_> {
        unsafe { CodecSpecificInfoRef::from_raw(self.raw()) }
    }

    fn raw(&self) -> NonNull<ffi::webrtc_CodecSpecificInfo> {
        let raw = unsafe { ffi::webrtc_CodecSpecificInfo_unique_get(self.raw_unique.as_ptr()) };
        NonNull::new(raw).expect("BUG: webrtc_CodecSpecificInfo_unique_get が null を返しました")
    }
}

impl Drop for CodecSpecificInfo {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_CodecSpecificInfo_unique_delete(self.raw_unique.as_ptr()) };
    }
}

unsafe impl Send for CodecSpecificInfo {}

type VideoEncoderEncodedImageCallbackFn = Box<
    dyn for<'a> FnMut(
            EncodedImageRef<'a>,
            Option<CodecSpecificInfoRef<'a>>,
        ) -> VideoEncoderEncodedImageCallbackResult
        + Send
        + 'static,
>;

#[derive(Default)]
pub struct VideoEncoderEncodedImageCallbackCallbacks {
    pub on_encoded_image: Option<VideoEncoderEncodedImageCallbackFn>,
}

impl VideoEncoderEncodedImageCallbackCallbacks {
    fn with_defaults(mut self) -> Self {
        if self.on_encoded_image.is_none() {
            self.on_encoded_image = Some(Box::new(|_, _| {
                VideoEncoderEncodedImageCallbackResult::new(
                    VideoEncoderEncodedImageCallbackResultError::Ok,
                )
            }));
        }
        self
    }
}

pub struct VideoEncoderEncodedImageCallback {
    raw: NonNull<ffi::webrtc_VideoEncoder_EncodedImageCallback>,
}

impl VideoEncoderEncodedImageCallback {
    pub fn new_with_callbacks(callbacks: VideoEncoderEncodedImageCallbackCallbacks) -> Self {
        let state = Box::new(VideoEncoderEncodedImageCallbackState {
            callbacks: callbacks.with_defaults(),
        });
        let user_data = Box::into_raw(state) as *mut c_void;
        let cbs = ffi::webrtc_VideoEncoder_EncodedImageCallback_cbs {
            OnEncodedImage: Some(video_encoder_encoded_image_callback_on_encoded_image),
            OnDestroy: Some(video_encoder_encoded_image_callback_on_destroy),
        };
        let raw = unsafe { ffi::webrtc_VideoEncoder_EncodedImageCallback_new(&cbs, user_data) };
        let raw = match NonNull::new(raw) {
            Some(raw) => raw,
            None => {
                let _ = unsafe {
                    Box::from_raw(user_data as *mut VideoEncoderEncodedImageCallbackState)
                };
                panic!("BUG: webrtc_VideoEncoder_EncodedImageCallback_new が null を返しました");
            }
        };
        Self { raw }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_VideoEncoder_EncodedImageCallback {
        self.raw.as_ptr()
    }

    pub fn as_ref(&self) -> VideoEncoderEncodedImageCallbackRef<'_> {
        unsafe { VideoEncoderEncodedImageCallbackRef::from_raw(self.raw) }
    }

    pub fn on_encoded_image(
        &self,
        image: EncodedImageRef<'_>,
        codec_specific_info: Option<CodecSpecificInfoRef<'_>>,
    ) -> VideoEncoderEncodedImageCallbackResult {
        self.as_ref().on_encoded_image(image, codec_specific_info)
    }
}

impl Drop for VideoEncoderEncodedImageCallback {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_VideoEncoder_EncodedImageCallback_delete(self.raw.as_ptr()) };
    }
}

unsafe impl Send for VideoEncoderEncodedImageCallback {}

pub struct VideoEncoderEncodedImageCallbackRef<'a> {
    raw: NonNull<ffi::webrtc_VideoEncoder_EncodedImageCallback>,
    _marker: PhantomData<&'a ffi::webrtc_VideoEncoder_EncodedImageCallback>,
}

impl<'a> VideoEncoderEncodedImageCallbackRef<'a> {
    /// # Safety
    /// `raw` は有効な `webrtc_VideoEncoder_EncodedImageCallback` を指している必要があります。
    pub unsafe fn from_raw(raw: NonNull<ffi::webrtc_VideoEncoder_EncodedImageCallback>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut ffi::webrtc_VideoEncoder_EncodedImageCallback {
        self.raw.as_ptr()
    }

    pub fn on_encoded_image(
        &self,
        image: EncodedImageRef<'_>,
        codec_specific_info: Option<CodecSpecificInfoRef<'_>>,
    ) -> VideoEncoderEncodedImageCallbackResult {
        let image = image.as_ptr();
        let codec_specific_info = codec_specific_info.map_or(std::ptr::null_mut(), |v| v.as_ptr());
        let raw_unique = unsafe {
            ffi::webrtc_VideoEncoder_EncodedImageCallback_OnEncodedImage(
                self.as_ptr(),
                image,
                codec_specific_info,
            )
        };
        let raw_unique = NonNull::new(raw_unique).expect(
            "BUG: webrtc_VideoEncoder_EncodedImageCallback_OnEncodedImage が null を返しました",
        );
        unsafe { VideoEncoderEncodedImageCallbackResult::from_raw_unique(raw_unique) }
    }
}

unsafe impl<'a> Send for VideoEncoderEncodedImageCallbackRef<'a> {}

#[allow(dead_code)]
pub struct VideoDecoderDecodedImageCallbackRef<'a> {
    raw: NonNull<ffi::webrtc_VideoDecoder_DecodedImageCallback>,
    _marker: PhantomData<&'a ffi::webrtc_VideoDecoder_DecodedImageCallback>,
}

impl<'a> VideoDecoderDecodedImageCallbackRef<'a> {
    /// # Safety
    /// `raw` は有効な `webrtc_VideoDecoder_DecodedImageCallback` を指している必要があります。
    pub unsafe fn from_raw(raw: NonNull<ffi::webrtc_VideoDecoder_DecodedImageCallback>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn as_ptr(&self) -> *mut ffi::webrtc_VideoDecoder_DecodedImageCallback {
        self.raw.as_ptr()
    }
}

unsafe impl<'a> Send for VideoDecoderDecodedImageCallbackRef<'a> {}

type VideoEncoderInitEncodeCallback =
    Box<dyn for<'a> FnMut(VideoCodecRef<'a>, VideoEncoderSettingsRef<'a>) -> i32 + Send + 'static>;
type VideoEncoderEncodeCallback = Box<
    dyn for<'a> FnMut(VideoFrameRef<'a>, Option<VideoFrameTypeVectorRef<'a>>) -> i32
        + Send
        + 'static,
>;
type VideoEncoderRegisterEncodeCompleteCallback =
    Box<dyn for<'a> FnMut(Option<VideoEncoderEncodedImageCallbackRef<'a>>) -> i32 + Send + 'static>;
type VideoEncoderReleaseCallback = Box<dyn FnMut() -> i32 + Send + 'static>;
type VideoEncoderSetRatesCallback =
    Box<dyn for<'a> FnMut(VideoEncoderRateControlParametersRef<'a>) + Send + 'static>;
type VideoEncoderGetEncoderInfoCallback =
    Box<dyn FnMut() -> VideoEncoderEncoderInfo + Send + 'static>;

#[derive(Default)]
pub struct VideoEncoderCallbacks {
    pub init_encode: Option<VideoEncoderInitEncodeCallback>,
    pub encode: Option<VideoEncoderEncodeCallback>,
    pub register_encode_complete_callback: Option<VideoEncoderRegisterEncodeCompleteCallback>,
    pub release: Option<VideoEncoderReleaseCallback>,
    pub set_rates: Option<VideoEncoderSetRatesCallback>,
    pub get_encoder_info: Option<VideoEncoderGetEncoderInfoCallback>,
}

impl VideoEncoderCallbacks {
    fn with_defaults(mut self) -> Self {
        if self.init_encode.is_none() {
            self.init_encode = Some(Box::new(|_, _| 0));
        }
        if self.encode.is_none() {
            self.encode = Some(Box::new(|_, _| 0));
        }
        if self.register_encode_complete_callback.is_none() {
            self.register_encode_complete_callback = Some(Box::new(|_| 0));
        }
        if self.release.is_none() {
            self.release = Some(Box::new(|| 0));
        }
        if self.set_rates.is_none() {
            self.set_rates = Some(Box::new(|_| {}));
        }
        if self.get_encoder_info.is_none() {
            self.get_encoder_info = Some(Box::new(VideoEncoderEncoderInfo::new));
        }
        self
    }
}

type VideoDecoderConfigureCallback =
    Box<dyn for<'a> FnMut(VideoDecoderSettingsRef<'a>) -> bool + Send + 'static>;
type VideoDecoderDecodeCallback =
    Box<dyn for<'a> FnMut(EncodedImageRef<'a>, i64) -> i32 + Send + 'static>;
type VideoDecoderRegisterDecodeCompleteCallback =
    Box<dyn for<'a> FnMut(Option<VideoDecoderDecodedImageCallbackRef<'a>>) -> i32 + Send + 'static>;
type VideoDecoderReleaseCallback = Box<dyn FnMut() -> i32 + Send + 'static>;
type VideoDecoderGetDecoderInfoCallback =
    Box<dyn FnMut() -> VideoDecoderDecoderInfo + Send + 'static>;

#[derive(Default)]
pub struct VideoDecoderCallbacks {
    pub configure: Option<VideoDecoderConfigureCallback>,
    pub decode: Option<VideoDecoderDecodeCallback>,
    pub register_decode_complete_callback: Option<VideoDecoderRegisterDecodeCompleteCallback>,
    pub release: Option<VideoDecoderReleaseCallback>,
    pub get_decoder_info: Option<VideoDecoderGetDecoderInfoCallback>,
}

impl VideoDecoderCallbacks {
    fn with_defaults(mut self) -> Self {
        if self.configure.is_none() {
            self.configure = Some(Box::new(|_| true));
        }
        if self.decode.is_none() {
            self.decode = Some(Box::new(|_, _| 0));
        }
        if self.register_decode_complete_callback.is_none() {
            self.register_decode_complete_callback = Some(Box::new(|_| 0));
        }
        if self.release.is_none() {
            self.release = Some(Box::new(|| 0));
        }
        if self.get_decoder_info.is_none() {
            self.get_decoder_info = Some(Box::new(VideoDecoderDecoderInfo::new));
        }
        self
    }
}

type VideoEncoderFactoryGetSupportedFormatsCallback =
    Box<dyn FnMut() -> Vec<SdpVideoFormat> + Send + 'static>;
type VideoEncoderFactoryCreateCallback = Box<
    dyn for<'a> FnMut(EnvironmentRef<'a>, SdpVideoFormatRef<'a>) -> Option<VideoEncoder>
        + Send
        + 'static,
>;

#[derive(Default)]
pub struct VideoEncoderFactoryCallbacks {
    pub get_supported_formats: Option<VideoEncoderFactoryGetSupportedFormatsCallback>,
    pub create: Option<VideoEncoderFactoryCreateCallback>,
}

impl VideoEncoderFactoryCallbacks {
    fn with_defaults(mut self) -> Self {
        if self.get_supported_formats.is_none() {
            self.get_supported_formats = Some(Box::new(Vec::new));
        }
        if self.create.is_none() {
            self.create = Some(Box::new(|_, _| None));
        }
        self
    }
}

type VideoDecoderFactoryGetSupportedFormatsCallback =
    Box<dyn FnMut() -> Vec<SdpVideoFormat> + Send + 'static>;
type VideoDecoderFactoryCreateCallback = Box<
    dyn for<'a> FnMut(EnvironmentRef<'a>, SdpVideoFormatRef<'a>) -> Option<VideoDecoder>
        + Send
        + 'static,
>;

#[derive(Default)]
pub struct VideoDecoderFactoryCallbacks {
    pub get_supported_formats: Option<VideoDecoderFactoryGetSupportedFormatsCallback>,
    pub create: Option<VideoDecoderFactoryCreateCallback>,
}

impl VideoDecoderFactoryCallbacks {
    fn with_defaults(mut self) -> Self {
        if self.get_supported_formats.is_none() {
            self.get_supported_formats = Some(Box::new(Vec::new));
        }
        if self.create.is_none() {
            self.create = Some(Box::new(|_, _| None));
        }
        self
    }
}

struct VideoEncoderCallbackState {
    callbacks: VideoEncoderCallbacks,
}

struct VideoEncoderEncodedImageCallbackState {
    callbacks: VideoEncoderEncodedImageCallbackCallbacks,
}

struct VideoDecoderCallbackState {
    callbacks: VideoDecoderCallbacks,
}

struct VideoEncoderFactoryCallbackState {
    callbacks: VideoEncoderFactoryCallbacks,
}

struct VideoDecoderFactoryCallbackState {
    callbacks: VideoDecoderFactoryCallbacks,
}

unsafe extern "C" fn video_encoder_on_destroy(user_data: *mut c_void) {
    if user_data.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(user_data as *mut VideoEncoderCallbackState) };
}

unsafe extern "C" fn video_encoder_encoded_image_callback_on_destroy(user_data: *mut c_void) {
    if user_data.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(user_data as *mut VideoEncoderEncodedImageCallbackState) };
}

unsafe extern "C" fn video_encoder_encoded_image_callback_on_encoded_image(
    encoded_image: *mut ffi::webrtc_EncodedImage,
    codec_specific_info: *mut ffi::webrtc_CodecSpecificInfo,
    user_data: *mut c_void,
) -> *mut ffi::webrtc_VideoEncoder_EncodedImageCallback_Result_unique {
    assert!(
        !user_data.is_null(),
        "video_encoder_encoded_image_callback_on_encoded_image: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoEncoderEncodedImageCallbackState) };
    let cb = state
        .callbacks
        .on_encoded_image
        .as_mut()
        .expect("video_encoder_encoded_image_callback_on_encoded_image: callback is None");
    let encoded_image = NonNull::new(encoded_image)
        .expect("video_encoder_encoded_image_callback_on_encoded_image: encoded_image is null");
    let encoded_image = unsafe { EncodedImageRef::from_raw(encoded_image) };
    let codec_specific_info =
        NonNull::new(codec_specific_info).map(|v| unsafe { CodecSpecificInfoRef::from_raw(v) });
    let result = cb(encoded_image, codec_specific_info);
    result.into_raw_unique()
}

unsafe extern "C" fn video_encoder_init_encode(
    codec_settings: *mut ffi::webrtc_VideoCodec,
    settings: *mut ffi::webrtc_VideoEncoder_Settings,
    user_data: *mut c_void,
) -> i32 {
    assert!(
        !user_data.is_null(),
        "video_encoder_init_encode: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoEncoderCallbackState) };
    let cb = state
        .callbacks
        .init_encode
        .as_mut()
        .expect("video_encoder_init_encode: callback is None");
    let codec_settings =
        NonNull::new(codec_settings).expect("video_encoder_init_encode: codec_settings is null");
    let settings = NonNull::new(settings).expect("video_encoder_init_encode: settings is null");
    let codec_settings = unsafe { VideoCodecRef::from_raw(codec_settings) };
    let settings = unsafe { VideoEncoderSettingsRef::from_raw(settings) };
    cb(codec_settings, settings)
}

unsafe extern "C" fn video_encoder_encode(
    frame: *mut ffi::webrtc_VideoFrame,
    frame_types: *mut ffi::webrtc_VideoFrameType_vector,
    user_data: *mut c_void,
) -> i32 {
    assert!(
        !user_data.is_null(),
        "video_encoder_encode: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoEncoderCallbackState) };
    let cb = state
        .callbacks
        .encode
        .as_mut()
        .expect("video_encoder_encode: callback is None");
    let frame = NonNull::new(frame).expect("video_encoder_encode: frame is null");
    let frame = unsafe { VideoFrameRef::from_raw(frame) };
    let frame_types = NonNull::new(frame_types)
        .map(|frame_types| unsafe { VideoFrameTypeVectorRef::from_raw(frame_types) });
    cb(frame, frame_types)
}

unsafe extern "C" fn video_encoder_register_encode_complete_callback(
    callback: *mut ffi::webrtc_VideoEncoder_EncodedImageCallback,
    user_data: *mut c_void,
) -> i32 {
    assert!(
        !user_data.is_null(),
        "video_encoder_register_encode_complete_callback: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoEncoderCallbackState) };
    let cb = state
        .callbacks
        .register_encode_complete_callback
        .as_mut()
        .expect("video_encoder_register_encode_complete_callback: callback is None");
    let callback = NonNull::new(callback)
        .map(|callback| unsafe { VideoEncoderEncodedImageCallbackRef::from_raw(callback) });
    cb(callback)
}

unsafe extern "C" fn video_encoder_release(user_data: *mut c_void) -> i32 {
    assert!(
        !user_data.is_null(),
        "video_encoder_release: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoEncoderCallbackState) };
    let cb = state
        .callbacks
        .release
        .as_mut()
        .expect("video_encoder_release: callback is None");
    cb()
}

unsafe extern "C" fn video_encoder_set_rates(
    parameters: *mut ffi::webrtc_VideoEncoder_RateControlParameters,
    user_data: *mut c_void,
) {
    assert!(
        !user_data.is_null(),
        "video_encoder_set_rates: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoEncoderCallbackState) };
    let cb = state
        .callbacks
        .set_rates
        .as_mut()
        .expect("video_encoder_set_rates: callback is None");
    let parameters = NonNull::new(parameters).expect("video_encoder_set_rates: parameters is null");
    let parameters = unsafe { VideoEncoderRateControlParametersRef::from_raw(parameters) };
    cb(parameters);
}

unsafe extern "C" fn video_encoder_get_encoder_info(
    user_data: *mut c_void,
) -> *mut ffi::webrtc_VideoEncoder_EncoderInfo_unique {
    assert!(
        !user_data.is_null(),
        "video_encoder_get_encoder_info: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoEncoderCallbackState) };
    let cb = state
        .callbacks
        .get_encoder_info
        .as_mut()
        .expect("video_encoder_get_encoder_info: callback is None");
    cb().into_raw()
}

unsafe extern "C" fn video_decoder_on_destroy(user_data: *mut c_void) {
    if user_data.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(user_data as *mut VideoDecoderCallbackState) };
}

unsafe extern "C" fn video_decoder_configure(
    settings: *mut ffi::webrtc_VideoDecoder_Settings,
    user_data: *mut c_void,
) -> i32 {
    assert!(
        !user_data.is_null(),
        "video_decoder_configure: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoDecoderCallbackState) };
    let cb = state
        .callbacks
        .configure
        .as_mut()
        .expect("video_decoder_configure: callback is None");
    let settings = NonNull::new(settings).expect("video_decoder_configure: settings is null");
    let settings = unsafe { VideoDecoderSettingsRef::from_raw(settings) };
    if cb(settings) { 1 } else { 0 }
}

unsafe extern "C" fn video_decoder_decode(
    input_image: *mut ffi::webrtc_EncodedImage,
    render_time_ms: i64,
    user_data: *mut c_void,
) -> i32 {
    assert!(
        !user_data.is_null(),
        "video_decoder_decode: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoDecoderCallbackState) };
    let cb = state
        .callbacks
        .decode
        .as_mut()
        .expect("video_decoder_decode: callback is None");
    let input_image = NonNull::new(input_image).expect("video_decoder_decode: input_image is null");
    let input_image = unsafe { EncodedImageRef::from_raw(input_image) };
    cb(input_image, render_time_ms)
}

unsafe extern "C" fn video_decoder_register_decode_complete_callback(
    callback: *mut ffi::webrtc_VideoDecoder_DecodedImageCallback,
    user_data: *mut c_void,
) -> i32 {
    assert!(
        !user_data.is_null(),
        "video_decoder_register_decode_complete_callback: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoDecoderCallbackState) };
    let cb = state
        .callbacks
        .register_decode_complete_callback
        .as_mut()
        .expect("video_decoder_register_decode_complete_callback: callback is None");
    let callback = NonNull::new(callback)
        .map(|callback| unsafe { VideoDecoderDecodedImageCallbackRef::from_raw(callback) });
    cb(callback)
}

unsafe extern "C" fn video_decoder_release(user_data: *mut c_void) -> i32 {
    assert!(
        !user_data.is_null(),
        "video_decoder_release: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoDecoderCallbackState) };
    let cb = state
        .callbacks
        .release
        .as_mut()
        .expect("video_decoder_release: callback is None");
    cb()
}

unsafe extern "C" fn video_decoder_get_decoder_info(
    user_data: *mut c_void,
) -> *mut ffi::webrtc_VideoDecoder_DecoderInfo_unique {
    assert!(
        !user_data.is_null(),
        "video_decoder_get_decoder_info: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoDecoderCallbackState) };
    let cb = state
        .callbacks
        .get_decoder_info
        .as_mut()
        .expect("video_decoder_get_decoder_info: callback is None");
    cb().into_raw()
}

unsafe extern "C" fn video_encoder_factory_on_destroy(user_data: *mut c_void) {
    if user_data.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(user_data as *mut VideoEncoderFactoryCallbackState) };
}

unsafe extern "C" fn video_encoder_factory_get_supported_formats(
    user_data: *mut c_void,
) -> *mut ffi::webrtc_SdpVideoFormat_vector {
    let empty = || unsafe { ffi::webrtc_SdpVideoFormat_vector_new() };
    assert!(
        !user_data.is_null(),
        "video_encoder_factory_get_supported_formats: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoEncoderFactoryCallbackState) };
    let cb = state
        .callbacks
        .get_supported_formats
        .as_mut()
        .expect("video_encoder_factory_get_supported_formats: callback is None");
    let formats = cb();
    let vec = empty();
    if vec.is_null() {
        return std::ptr::null_mut();
    }
    for format in &formats {
        unsafe { ffi::webrtc_SdpVideoFormat_vector_push_back(vec, format.raw().as_ptr()) };
    }
    vec
}

unsafe extern "C" fn video_encoder_factory_create(
    env: *mut ffi::webrtc_Environment,
    format: *mut ffi::webrtc_SdpVideoFormat,
    user_data: *mut c_void,
) -> *mut ffi::webrtc_VideoEncoder_unique {
    assert!(
        !user_data.is_null(),
        "video_encoder_factory_create: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoEncoderFactoryCallbackState) };
    let cb = state
        .callbacks
        .create
        .as_mut()
        .expect("video_encoder_factory_create: callback is None");
    let env = NonNull::new(env).expect("video_encoder_factory_create: env is null");
    let format = NonNull::new(format).expect("video_encoder_factory_create: format is null");
    let env = unsafe { EnvironmentRef::from_raw(env) };
    let format = unsafe { SdpVideoFormatRef::from_raw(format) };
    match cb(env, format) {
        Some(encoder) => encoder.into_raw(),
        None => std::ptr::null_mut(),
    }
}

unsafe extern "C" fn video_decoder_factory_on_destroy(user_data: *mut c_void) {
    if user_data.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(user_data as *mut VideoDecoderFactoryCallbackState) };
}

unsafe extern "C" fn video_decoder_factory_get_supported_formats(
    user_data: *mut c_void,
) -> *mut ffi::webrtc_SdpVideoFormat_vector {
    let empty = || unsafe { ffi::webrtc_SdpVideoFormat_vector_new() };
    assert!(
        !user_data.is_null(),
        "video_decoder_factory_get_supported_formats: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoDecoderFactoryCallbackState) };
    let cb = state
        .callbacks
        .get_supported_formats
        .as_mut()
        .expect("video_decoder_factory_get_supported_formats: callback is None");
    let formats = cb();
    let vec = empty();
    if vec.is_null() {
        return std::ptr::null_mut();
    }
    for format in &formats {
        unsafe { ffi::webrtc_SdpVideoFormat_vector_push_back(vec, format.raw().as_ptr()) };
    }
    vec
}

unsafe extern "C" fn video_decoder_factory_create(
    env: *mut ffi::webrtc_Environment,
    format: *mut ffi::webrtc_SdpVideoFormat,
    user_data: *mut c_void,
) -> *mut ffi::webrtc_VideoDecoder_unique {
    assert!(
        !user_data.is_null(),
        "video_decoder_factory_create: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoDecoderFactoryCallbackState) };
    let cb = state
        .callbacks
        .create
        .as_mut()
        .expect("video_decoder_factory_create: callback is None");
    let env = NonNull::new(env).expect("video_decoder_factory_create: env is null");
    let format = NonNull::new(format).expect("video_decoder_factory_create: format is null");
    let env = unsafe { EnvironmentRef::from_raw(env) };
    let format = unsafe { SdpVideoFormatRef::from_raw(format) };
    match cb(env, format) {
        Some(decoder) => decoder.into_raw(),
        None => std::ptr::null_mut(),
    }
}

/// webrtc::VideoEncoder のラッパー。
pub struct VideoEncoder {
    raw_unique: NonNull<ffi::webrtc_VideoEncoder_unique>,
}

impl VideoEncoder {
    pub fn new_with_callbacks(callbacks: VideoEncoderCallbacks) -> Self {
        let state = Box::new(VideoEncoderCallbackState {
            callbacks: callbacks.with_defaults(),
        });
        let user_data = Box::into_raw(state) as *mut c_void;
        let cbs = ffi::webrtc_VideoEncoder_cbs {
            InitEncode: Some(video_encoder_init_encode),
            Encode: Some(video_encoder_encode),
            RegisterEncodeCompleteCallback: Some(video_encoder_register_encode_complete_callback),
            Release: Some(video_encoder_release),
            SetRates: Some(video_encoder_set_rates),
            GetEncoderInfo: Some(video_encoder_get_encoder_info),
            OnDestroy: Some(video_encoder_on_destroy),
        };
        let raw = unsafe { ffi::webrtc_VideoEncoder_new(&cbs, user_data) };
        let raw_unique = match NonNull::new(raw) {
            Some(raw_unique) => raw_unique,
            None => {
                let _ = unsafe { Box::from_raw(user_data as *mut VideoEncoderCallbackState) };
                panic!("BUG: webrtc_VideoEncoder_new が null を返しました");
            }
        };
        Self { raw_unique }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_VideoEncoder {
        unsafe { ffi::webrtc_VideoEncoder_unique_get(self.raw_unique.as_ptr()) }
    }

    pub fn into_raw(self) -> *mut ffi::webrtc_VideoEncoder_unique {
        std::mem::ManuallyDrop::new(self).raw_unique.as_ptr()
    }

    pub fn init_encode(&self) -> i32 {
        unsafe {
            ffi::webrtc_VideoEncoder_InitEncode(
                self.as_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        }
    }

    pub fn encode(&self, frame: &VideoFrame) -> i32 {
        self.encode_with_frame_types(frame, None)
    }

    pub fn encode_with_frame_types(
        &self,
        frame: &VideoFrame,
        frame_types: Option<VideoFrameTypeVectorRef<'_>>,
    ) -> i32 {
        let frame_types =
            frame_types.map_or(std::ptr::null_mut(), |frame_types| frame_types.as_ptr());
        unsafe { ffi::webrtc_VideoEncoder_Encode(self.as_ptr(), frame.raw().as_ptr(), frame_types) }
    }

    pub fn register_encode_complete_callback(
        &self,
        callback: Option<VideoEncoderEncodedImageCallbackRef<'_>>,
    ) -> i32 {
        let callback = callback.map_or(std::ptr::null_mut(), |callback| callback.as_ptr());
        unsafe { ffi::webrtc_VideoEncoder_RegisterEncodeCompleteCallback(self.as_ptr(), callback) }
    }

    pub fn set_rates(&self) {
        unsafe { ffi::webrtc_VideoEncoder_SetRates(self.as_ptr(), std::ptr::null_mut()) };
    }

    pub fn get_encoder_info(&self) -> VideoEncoderEncoderInfo {
        let raw = unsafe { ffi::webrtc_VideoEncoder_GetEncoderInfo(self.as_ptr()) };
        let raw_unique =
            NonNull::new(raw).expect("webrtc_VideoEncoder_GetEncoderInfo が null を返しました");
        VideoEncoderEncoderInfo { raw_unique }
    }
}

impl Drop for VideoEncoder {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_VideoEncoder_unique_delete(self.raw_unique.as_ptr()) };
    }
}

/// webrtc::VideoDecoder のラッパー。
pub struct VideoDecoder {
    raw_unique: NonNull<ffi::webrtc_VideoDecoder_unique>,
}

impl VideoDecoder {
    pub fn new_with_callbacks(callbacks: VideoDecoderCallbacks) -> Self {
        let state = Box::new(VideoDecoderCallbackState {
            callbacks: callbacks.with_defaults(),
        });
        let user_data = Box::into_raw(state) as *mut c_void;
        let cbs = ffi::webrtc_VideoDecoder_cbs {
            Configure: Some(video_decoder_configure),
            Decode: Some(video_decoder_decode),
            RegisterDecodeCompleteCallback: Some(video_decoder_register_decode_complete_callback),
            Release: Some(video_decoder_release),
            GetDecoderInfo: Some(video_decoder_get_decoder_info),
            OnDestroy: Some(video_decoder_on_destroy),
        };
        let raw = unsafe { ffi::webrtc_VideoDecoder_new(&cbs, user_data) };
        let raw_unique = match NonNull::new(raw) {
            Some(raw_unique) => raw_unique,
            None => {
                let _ = unsafe { Box::from_raw(user_data as *mut VideoDecoderCallbackState) };
                panic!("BUG: webrtc_VideoDecoder_new が null を返しました");
            }
        };
        Self { raw_unique }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_VideoDecoder {
        unsafe { ffi::webrtc_VideoDecoder_unique_get(self.raw_unique.as_ptr()) }
    }

    pub fn into_raw(self) -> *mut ffi::webrtc_VideoDecoder_unique {
        std::mem::ManuallyDrop::new(self).raw_unique.as_ptr()
    }

    pub fn configure(&self) -> bool {
        unsafe { ffi::webrtc_VideoDecoder_Configure(self.as_ptr(), std::ptr::null_mut()) != 0 }
    }

    pub fn decode(&self, input_image: Option<EncodedImageRef<'_>>, render_time_ms: i64) -> i32 {
        let input_image =
            input_image.map_or(std::ptr::null_mut(), |input_image| input_image.as_ptr());
        unsafe { ffi::webrtc_VideoDecoder_Decode(self.as_ptr(), input_image, render_time_ms) }
    }

    pub fn get_decoder_info(&self) -> VideoDecoderDecoderInfo {
        let raw = unsafe { ffi::webrtc_VideoDecoder_GetDecoderInfo(self.as_ptr()) };
        let raw_unique =
            NonNull::new(raw).expect("webrtc_VideoDecoder_GetDecoderInfo が null を返しました");
        VideoDecoderDecoderInfo { raw_unique }
    }
}

impl Drop for VideoDecoder {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_VideoDecoder_unique_delete(self.raw_unique.as_ptr()) };
    }
}

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

    pub fn new_with_callbacks(callbacks: VideoEncoderFactoryCallbacks) -> Self {
        let state = Box::new(VideoEncoderFactoryCallbackState {
            callbacks: callbacks.with_defaults(),
        });
        let user_data = Box::into_raw(state) as *mut c_void;
        let cbs = ffi::webrtc_VideoEncoderFactory_cbs {
            GetSupportedFormats: Some(video_encoder_factory_get_supported_formats),
            Create: Some(video_encoder_factory_create),
            OnDestroy: Some(video_encoder_factory_on_destroy),
        };
        let raw = unsafe { ffi::webrtc_VideoEncoderFactory_new(&cbs, user_data) };
        let raw_unique = match NonNull::new(raw) {
            Some(raw_unique) => raw_unique,
            None => {
                let _ =
                    unsafe { Box::from_raw(user_data as *mut VideoEncoderFactoryCallbackState) };
                panic!("BUG: webrtc_VideoEncoderFactory_new が null を返しました");
            }
        };
        Self { raw_unique }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_VideoEncoderFactory {
        unsafe { ffi::webrtc_VideoEncoderFactory_unique_get(self.raw_unique.as_ptr()) }
    }

    pub fn into_raw(self) -> *mut ffi::webrtc_VideoEncoderFactory_unique {
        std::mem::ManuallyDrop::new(self).raw_unique.as_ptr()
    }

    pub fn create(
        &self,
        env: &crate::Environment,
        format: &SdpVideoFormat,
    ) -> Option<VideoEncoder> {
        let raw = unsafe {
            ffi::webrtc_VideoEncoderFactory_Create(
                self.as_ptr(),
                env.as_ptr(),
                format.raw().as_ptr(),
            )
        };
        Some(VideoEncoder {
            raw_unique: NonNull::new(raw)?,
        })
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

    pub fn new_with_callbacks(callbacks: VideoDecoderFactoryCallbacks) -> Self {
        let state = Box::new(VideoDecoderFactoryCallbackState {
            callbacks: callbacks.with_defaults(),
        });
        let user_data = Box::into_raw(state) as *mut c_void;
        let cbs = ffi::webrtc_VideoDecoderFactory_cbs {
            GetSupportedFormats: Some(video_decoder_factory_get_supported_formats),
            Create: Some(video_decoder_factory_create),
            OnDestroy: Some(video_decoder_factory_on_destroy),
        };
        let raw = unsafe { ffi::webrtc_VideoDecoderFactory_new(&cbs, user_data) };
        let raw_unique = match NonNull::new(raw) {
            Some(raw_unique) => raw_unique,
            None => {
                let _ =
                    unsafe { Box::from_raw(user_data as *mut VideoDecoderFactoryCallbackState) };
                panic!("BUG: webrtc_VideoDecoderFactory_new が null を返しました");
            }
        };
        Self { raw_unique }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_VideoDecoderFactory {
        unsafe { ffi::webrtc_VideoDecoderFactory_unique_get(self.raw_unique.as_ptr()) }
    }

    pub fn into_raw(self) -> *mut ffi::webrtc_VideoDecoderFactory_unique {
        std::mem::ManuallyDrop::new(self).raw_unique.as_ptr()
    }

    pub fn create(
        &self,
        env: &crate::Environment,
        format: &SdpVideoFormat,
    ) -> Option<VideoDecoder> {
        let raw = unsafe {
            ffi::webrtc_VideoDecoderFactory_Create(
                self.as_ptr(),
                env.as_ptr(),
                format.raw().as_ptr(),
            )
        };
        Some(VideoDecoder {
            raw_unique: NonNull::new(raw)?,
        })
    }
}

impl Drop for VideoDecoderFactory {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_VideoDecoderFactory_unique_delete(self.raw_unique.as_ptr()) };
    }
}
