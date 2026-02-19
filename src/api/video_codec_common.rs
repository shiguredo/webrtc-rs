use crate::ref_count::{EncodedImageBufferHandle, I420BufferHandle};
use crate::{CxxStringRef, MapStringString, Result, ScopedRef, ffi};
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::slice;

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

    pub(crate) fn raw(&self) -> NonNull<ffi::webrtc_SdpVideoFormat> {
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
}

impl I420Buffer {
    pub fn new(width: i32, height: i32) -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_I420Buffer_Create(width, height) })
            .expect("BUG: webrtc_I420Buffer_Create が null を返しました");
        let raw_ref = ScopedRef::<I420BufferHandle>::from_raw(raw);
        Self { raw_ref }
    }

    pub fn width(&self) -> i32 {
        unsafe { ffi::webrtc_I420Buffer_width(self.raw().as_ptr()) }
    }

    pub fn height(&self) -> i32 {
        unsafe { ffi::webrtc_I420Buffer_height(self.raw().as_ptr()) }
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
        let len = stride * self.height() as usize;
        unsafe { slice::from_raw_parts_mut(ptr, len) }.fill(value);
    }

    /// U/V 平面を単一値で塗りつぶす。
    pub fn fill_uv(&mut self, u: u8, v: u8) {
        let raw = self.raw();
        let stride_u = unsafe { ffi::webrtc_I420Buffer_StrideU(raw.as_ptr()) } as usize;
        let stride_v = unsafe { ffi::webrtc_I420Buffer_StrideV(raw.as_ptr()) } as usize;
        let h = (self.height() as usize).div_ceil(2);
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
        let len = stride * self.height() as usize;
        unsafe { slice::from_raw_parts(ptr, len) }
    }

    /// U 平面を参照する。
    pub fn u_data(&self) -> &[u8] {
        let raw = self.raw();
        let ptr = unsafe { ffi::webrtc_I420Buffer_MutableDataU(raw.as_ptr()) };
        let stride = unsafe { ffi::webrtc_I420Buffer_StrideU(raw.as_ptr()) } as usize;
        let h = (self.height() as usize).div_ceil(2);
        unsafe { slice::from_raw_parts(ptr, stride * h) }
    }

    /// V 平面を参照する。
    pub fn v_data(&self) -> &[u8] {
        let raw = self.raw();
        let ptr = unsafe { ffi::webrtc_I420Buffer_MutableDataV(raw.as_ptr()) };
        let stride = unsafe { ffi::webrtc_I420Buffer_StrideV(raw.as_ptr()) } as usize;
        let h = (self.height() as usize).div_ceil(2);
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

    pub(crate) fn raw(&self) -> NonNull<ffi::webrtc_VideoFrame> {
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
        I420Buffer { raw_ref }
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
    pub(crate) fn from_raw(value: i32) -> Self {
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

    pub(crate) fn to_raw(self) -> i32 {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoCodecStatus {
    TargetBitrateOvershoot,
    OkRequestKeyframe,
    NoOutput,
    Ok,
    Error,
    Memory,
    ErrParameter,
    Timeout,
    Uninitialized,
    FallbackSoftware,
    ErrSimulcastParametersNotSupported,
    EncoderFailure,
    Unknown(i32),
}

impl VideoCodecStatus {
    pub(crate) fn from_raw(value: i32) -> Self {
        if value == unsafe { ffi::webrtc_VideoCodecStatus_TargetBitrateOvershoot } {
            Self::TargetBitrateOvershoot
        } else if value == unsafe { ffi::webrtc_VideoCodecStatus_OkRequestKeyframe } {
            Self::OkRequestKeyframe
        } else if value == unsafe { ffi::webrtc_VideoCodecStatus_NoOutput } {
            Self::NoOutput
        } else if value == unsafe { ffi::webrtc_VideoCodecStatus_Ok } {
            Self::Ok
        } else if value == unsafe { ffi::webrtc_VideoCodecStatus_Error } {
            Self::Error
        } else if value == unsafe { ffi::webrtc_VideoCodecStatus_Memory } {
            Self::Memory
        } else if value == unsafe { ffi::webrtc_VideoCodecStatus_ErrParameter } {
            Self::ErrParameter
        } else if value == unsafe { ffi::webrtc_VideoCodecStatus_Timeout } {
            Self::Timeout
        } else if value == unsafe { ffi::webrtc_VideoCodecStatus_Uninitialized } {
            Self::Uninitialized
        } else if value == unsafe { ffi::webrtc_VideoCodecStatus_FallbackSoftware } {
            Self::FallbackSoftware
        } else if value
            == unsafe { ffi::webrtc_VideoCodecStatus_ErrSimulcastParametersNotSupported }
        {
            Self::ErrSimulcastParametersNotSupported
        } else if value == unsafe { ffi::webrtc_VideoCodecStatus_EncoderFailure } {
            Self::EncoderFailure
        } else {
            Self::Unknown(value)
        }
    }

    pub(crate) fn to_raw(self) -> i32 {
        match self {
            Self::TargetBitrateOvershoot => unsafe {
                ffi::webrtc_VideoCodecStatus_TargetBitrateOvershoot
            },
            Self::OkRequestKeyframe => unsafe { ffi::webrtc_VideoCodecStatus_OkRequestKeyframe },
            Self::NoOutput => unsafe { ffi::webrtc_VideoCodecStatus_NoOutput },
            Self::Ok => unsafe { ffi::webrtc_VideoCodecStatus_Ok },
            Self::Error => unsafe { ffi::webrtc_VideoCodecStatus_Error },
            Self::Memory => unsafe { ffi::webrtc_VideoCodecStatus_Memory },
            Self::ErrParameter => unsafe { ffi::webrtc_VideoCodecStatus_ErrParameter },
            Self::Timeout => unsafe { ffi::webrtc_VideoCodecStatus_Timeout },
            Self::Uninitialized => unsafe { ffi::webrtc_VideoCodecStatus_Uninitialized },
            Self::FallbackSoftware => unsafe { ffi::webrtc_VideoCodecStatus_FallbackSoftware },
            Self::ErrSimulcastParametersNotSupported => unsafe {
                ffi::webrtc_VideoCodecStatus_ErrSimulcastParametersNotSupported
            },
            Self::EncoderFailure => unsafe { ffi::webrtc_VideoCodecStatus_EncoderFailure },
            Self::Unknown(v) => v,
        }
    }
}

impl VideoCodecType {
    pub(crate) fn from_raw(value: i32) -> Self {
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

    pub(crate) fn to_raw(self) -> i32 {
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

pub struct EncodedImageBuffer {
    raw_ref: ScopedRef<EncodedImageBufferHandle>,
}

impl EncodedImageBuffer {
    pub fn from_bytes(data: &[u8]) -> Self {
        let raw_ref = NonNull::new(unsafe {
            ffi::webrtc_EncodedImageBuffer_Create_from_data(data.as_ptr(), data.len())
        })
        .expect("BUG: webrtc_EncodedImageBuffer_Create_from_data が null を返しました");
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
        unsafe {
            ffi::webrtc_EncodedImage_set_encoded_data(self.raw.as_ptr(), encoded_data.as_ptr())
        };
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
