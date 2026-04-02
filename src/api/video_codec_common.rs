use crate::ref_count::{
    EncodedImageBufferHandle, I420BufferHandle, NV12BufferHandle, VideoFrameBufferHandle,
};
use crate::{CxxString, CxxStringRef, Error, MapStringString, Result, ScopedRef, ffi};
use std::any::Any;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::os::raw::c_void;
use std::ptr::NonNull;
use std::slice;
use std::time::Duration;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScalabilityMode {
    L1T1,
    L1T2,
    L1T3,
    L2T1,
    L2T1h,
    L2T1Key,
    L2T2,
    L2T2h,
    L2T2Key,
    L2T2KeyShift,
    L2T3,
    L2T3h,
    L2T3Key,
    L3T1,
    L3T1h,
    L3T1Key,
    L3T2,
    L3T2h,
    L3T2Key,
    L3T3,
    L3T3h,
    L3T3Key,
    S2T1,
    S2T1h,
    S2T2,
    S2T2h,
    S2T3,
    S2T3h,
    S3T1,
    S3T1h,
    S3T2,
    S3T2h,
    S3T3,
    S3T3h,
}

impl ScalabilityMode {
    pub fn as_str(self) -> Result<String> {
        let raw = unsafe { ffi::webrtc_ScalabilityModeToString(self.to_raw()) };
        let raw =
            NonNull::new(raw).expect("BUG: webrtc_ScalabilityModeToString が null を返しました");
        CxxString::from_unique(raw).to_string()
    }

    fn from_raw(value: i32) -> Option<Self> {
        unsafe {
            match value {
                x if x == ffi::webrtc_ScalabilityMode_L1T1 => Some(Self::L1T1),
                x if x == ffi::webrtc_ScalabilityMode_L1T2 => Some(Self::L1T2),
                x if x == ffi::webrtc_ScalabilityMode_L1T3 => Some(Self::L1T3),
                x if x == ffi::webrtc_ScalabilityMode_L2T1 => Some(Self::L2T1),
                x if x == ffi::webrtc_ScalabilityMode_L2T1h => Some(Self::L2T1h),
                x if x == ffi::webrtc_ScalabilityMode_L2T1_KEY => Some(Self::L2T1Key),
                x if x == ffi::webrtc_ScalabilityMode_L2T2 => Some(Self::L2T2),
                x if x == ffi::webrtc_ScalabilityMode_L2T2h => Some(Self::L2T2h),
                x if x == ffi::webrtc_ScalabilityMode_L2T2_KEY => Some(Self::L2T2Key),
                x if x == ffi::webrtc_ScalabilityMode_L2T2_KEY_SHIFT => Some(Self::L2T2KeyShift),
                x if x == ffi::webrtc_ScalabilityMode_L2T3 => Some(Self::L2T3),
                x if x == ffi::webrtc_ScalabilityMode_L2T3h => Some(Self::L2T3h),
                x if x == ffi::webrtc_ScalabilityMode_L2T3_KEY => Some(Self::L2T3Key),
                x if x == ffi::webrtc_ScalabilityMode_L3T1 => Some(Self::L3T1),
                x if x == ffi::webrtc_ScalabilityMode_L3T1h => Some(Self::L3T1h),
                x if x == ffi::webrtc_ScalabilityMode_L3T1_KEY => Some(Self::L3T1Key),
                x if x == ffi::webrtc_ScalabilityMode_L3T2 => Some(Self::L3T2),
                x if x == ffi::webrtc_ScalabilityMode_L3T2h => Some(Self::L3T2h),
                x if x == ffi::webrtc_ScalabilityMode_L3T2_KEY => Some(Self::L3T2Key),
                x if x == ffi::webrtc_ScalabilityMode_L3T3 => Some(Self::L3T3),
                x if x == ffi::webrtc_ScalabilityMode_L3T3h => Some(Self::L3T3h),
                x if x == ffi::webrtc_ScalabilityMode_L3T3_KEY => Some(Self::L3T3Key),
                x if x == ffi::webrtc_ScalabilityMode_S2T1 => Some(Self::S2T1),
                x if x == ffi::webrtc_ScalabilityMode_S2T1h => Some(Self::S2T1h),
                x if x == ffi::webrtc_ScalabilityMode_S2T2 => Some(Self::S2T2),
                x if x == ffi::webrtc_ScalabilityMode_S2T2h => Some(Self::S2T2h),
                x if x == ffi::webrtc_ScalabilityMode_S2T3 => Some(Self::S2T3),
                x if x == ffi::webrtc_ScalabilityMode_S2T3h => Some(Self::S2T3h),
                x if x == ffi::webrtc_ScalabilityMode_S3T1 => Some(Self::S3T1),
                x if x == ffi::webrtc_ScalabilityMode_S3T1h => Some(Self::S3T1h),
                x if x == ffi::webrtc_ScalabilityMode_S3T2 => Some(Self::S3T2),
                x if x == ffi::webrtc_ScalabilityMode_S3T2h => Some(Self::S3T2h),
                x if x == ffi::webrtc_ScalabilityMode_S3T3 => Some(Self::S3T3),
                x if x == ffi::webrtc_ScalabilityMode_S3T3h => Some(Self::S3T3h),
                _ => None,
            }
        }
    }

    fn to_raw(self) -> i32 {
        unsafe {
            match self {
                Self::L1T1 => ffi::webrtc_ScalabilityMode_L1T1,
                Self::L1T2 => ffi::webrtc_ScalabilityMode_L1T2,
                Self::L1T3 => ffi::webrtc_ScalabilityMode_L1T3,
                Self::L2T1 => ffi::webrtc_ScalabilityMode_L2T1,
                Self::L2T1h => ffi::webrtc_ScalabilityMode_L2T1h,
                Self::L2T1Key => ffi::webrtc_ScalabilityMode_L2T1_KEY,
                Self::L2T2 => ffi::webrtc_ScalabilityMode_L2T2,
                Self::L2T2h => ffi::webrtc_ScalabilityMode_L2T2h,
                Self::L2T2Key => ffi::webrtc_ScalabilityMode_L2T2_KEY,
                Self::L2T2KeyShift => ffi::webrtc_ScalabilityMode_L2T2_KEY_SHIFT,
                Self::L2T3 => ffi::webrtc_ScalabilityMode_L2T3,
                Self::L2T3h => ffi::webrtc_ScalabilityMode_L2T3h,
                Self::L2T3Key => ffi::webrtc_ScalabilityMode_L2T3_KEY,
                Self::L3T1 => ffi::webrtc_ScalabilityMode_L3T1,
                Self::L3T1h => ffi::webrtc_ScalabilityMode_L3T1h,
                Self::L3T1Key => ffi::webrtc_ScalabilityMode_L3T1_KEY,
                Self::L3T2 => ffi::webrtc_ScalabilityMode_L3T2,
                Self::L3T2h => ffi::webrtc_ScalabilityMode_L3T2h,
                Self::L3T2Key => ffi::webrtc_ScalabilityMode_L3T2_KEY,
                Self::L3T3 => ffi::webrtc_ScalabilityMode_L3T3,
                Self::L3T3h => ffi::webrtc_ScalabilityMode_L3T3h,
                Self::L3T3Key => ffi::webrtc_ScalabilityMode_L3T3_KEY,
                Self::S2T1 => ffi::webrtc_ScalabilityMode_S2T1,
                Self::S2T1h => ffi::webrtc_ScalabilityMode_S2T1h,
                Self::S2T2 => ffi::webrtc_ScalabilityMode_S2T2,
                Self::S2T2h => ffi::webrtc_ScalabilityMode_S2T2h,
                Self::S2T3 => ffi::webrtc_ScalabilityMode_S2T3,
                Self::S2T3h => ffi::webrtc_ScalabilityMode_S2T3h,
                Self::S3T1 => ffi::webrtc_ScalabilityMode_S3T1,
                Self::S3T1h => ffi::webrtc_ScalabilityMode_S3T1h,
                Self::S3T2 => ffi::webrtc_ScalabilityMode_S3T2,
                Self::S3T2h => ffi::webrtc_ScalabilityMode_S3T2h,
                Self::S3T3 => ffi::webrtc_ScalabilityMode_S3T3,
                Self::S3T3h => ffi::webrtc_ScalabilityMode_S3T3h,
            }
        }
    }
}

pub struct SdpVideoFormat {
    raw_unique: NonNull<ffi::webrtc_SdpVideoFormat_unique>,
}

impl SdpVideoFormat {
    pub fn new(name: &str) -> Self {
        let raw = unsafe { ffi::webrtc_SdpVideoFormat_new(name.as_ptr() as *const _, name.len()) };
        Self {
            raw_unique: NonNull::new(raw)
                .expect("BUG: webrtc_SdpVideoFormat_new が null を返しました"),
        }
    }

    pub fn new_with_parameters(
        name: &str,
        parameters: &HashMap<String, String>,
        scalability_modes: &[ScalabilityMode],
    ) -> Self {
        let raw_modes = scalability_modes
            .iter()
            .copied()
            .map(ScalabilityMode::to_raw)
            .collect::<Vec<_>>();
        let raw = unsafe {
            ffi::webrtc_SdpVideoFormat_new_with_parameters(
                name.as_ptr() as *const _,
                name.len(),
                std::ptr::null_mut(),
                if raw_modes.is_empty() {
                    std::ptr::null()
                } else {
                    raw_modes.as_ptr()
                },
                raw_modes.len(),
            )
        };
        let mut format = Self {
            raw_unique: NonNull::new(raw)
                .expect("BUG: webrtc_SdpVideoFormat_new_with_parameters が null を返しました"),
        };
        for (key, value) in parameters {
            format.parameters_mut().set(key.as_str(), value.as_str());
        }
        format
    }

    pub fn name(&self) -> Result<String> {
        self.as_ref().name()
    }

    pub fn parameters_mut(&mut self) -> MapStringString<'_> {
        self.as_ref().parameters_mut()
    }

    pub fn is_equal(&self, other: SdpVideoFormatRef<'_>) -> bool {
        unsafe { ffi::webrtc_SdpVideoFormat_is_equal(self.raw().as_ptr(), other.raw.as_ptr()) != 0 }
    }

    pub fn is_same_codec(&self, other: SdpVideoFormatRef<'_>) -> bool {
        unsafe {
            ffi::webrtc_SdpVideoFormat_IsSameCodec(self.raw().as_ptr(), other.raw.as_ptr()) != 0
        }
    }

    pub fn scalability_modes(&self) -> Vec<ScalabilityMode> {
        self.as_ref().scalability_modes()
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

impl Clone for SdpVideoFormat {
    fn clone(&self) -> Self {
        let raw = unsafe { ffi::webrtc_SdpVideoFormat_copy(self.raw().as_ptr()) };
        Self {
            raw_unique: NonNull::new(raw)
                .expect("BUG: webrtc_SdpVideoFormat_copy が null を返しました"),
        }
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

    pub fn parameters_mut(&mut self) -> MapStringString<'a> {
        let ptr = unsafe { ffi::webrtc_SdpVideoFormat_get_parameters(self.raw.as_ptr()) };
        MapStringString::from_raw(NonNull::new(ptr).expect("BUG: ptr が null"))
    }

    pub fn scalability_modes(&self) -> Vec<ScalabilityMode> {
        let len =
            unsafe { ffi::webrtc_SdpVideoFormat_get_scalability_modes_size(self.raw.as_ptr()) };
        if len == 0 {
            return Vec::new();
        }
        let mut raw_modes = vec![unsafe { ffi::webrtc_ScalabilityMode_L1T1 }; len];
        let copied = unsafe {
            ffi::webrtc_SdpVideoFormat_copy_scalability_modes(
                self.raw.as_ptr(),
                raw_modes.as_mut_ptr(),
                raw_modes.len(),
            )
        };
        raw_modes
            .into_iter()
            .take(copied)
            .filter_map(ScalabilityMode::from_raw)
            .collect()
    }

    pub(crate) fn as_ptr(&self) -> *mut ffi::webrtc_SdpVideoFormat {
        self.raw.as_ptr()
    }

    pub fn to_owned(&self) -> SdpVideoFormat {
        let raw = unsafe { ffi::webrtc_SdpVideoFormat_copy(self.raw.as_ptr()) };
        SdpVideoFormat {
            raw_unique: NonNull::new(raw)
                .expect("BUG: webrtc_SdpVideoFormat_copy が null を返しました"),
        }
    }
}

unsafe impl<'a> Send for SdpVideoFormatRef<'a> {}

impl Drop for SdpVideoFormat {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_SdpVideoFormat_unique_delete(self.raw_unique.as_ptr()) };
    }
}

pub fn fuzzy_match_sdp_video_format(
    supported_formats: &[SdpVideoFormat],
    format: SdpVideoFormatRef<'_>,
) -> Option<SdpVideoFormat> {
    let raw_formats = unsafe { ffi::webrtc_SdpVideoFormat_vector_new() };
    let raw_formats =
        NonNull::new(raw_formats).expect("BUG: webrtc_SdpVideoFormat_vector_new returned null");

    for supported_format in supported_formats {
        unsafe {
            ffi::webrtc_SdpVideoFormat_vector_push_back(
                raw_formats.as_ptr(),
                supported_format.raw().as_ptr(),
            )
        };
    }

    let matched =
        unsafe { ffi::webrtc_FuzzyMatchSdpVideoFormat(raw_formats.as_ptr(), format.as_ptr()) };

    unsafe { ffi::webrtc_SdpVideoFormat_vector_delete(raw_formats.as_ptr()) };

    NonNull::new(matched).map(|raw_unique| SdpVideoFormat { raw_unique })
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

    pub fn new_with_strides(
        width: i32,
        height: i32,
        stride_y: i32,
        stride_u: i32,
        stride_v: i32,
    ) -> Self {
        let raw = NonNull::new(unsafe {
            ffi::webrtc_I420Buffer_CreateWithStrides(width, height, stride_y, stride_u, stride_v)
        })
        .expect("BUG: webrtc_I420Buffer_CreateWithStrides returned null");
        let raw_ref = ScopedRef::<I420BufferHandle>::from_raw(raw);
        Self { raw_ref }
    }

    pub fn width(&self) -> i32 {
        unsafe { ffi::webrtc_I420Buffer_width(self.raw().as_ptr()) }
    }

    pub fn height(&self) -> i32 {
        unsafe { ffi::webrtc_I420Buffer_height(self.raw().as_ptr()) }
    }

    pub fn chroma_width(&self) -> i32 {
        unsafe { ffi::webrtc_I420Buffer_chroma_width(self.raw().as_ptr()) }
    }

    pub fn chroma_height(&self) -> i32 {
        unsafe { ffi::webrtc_I420Buffer_chroma_height(self.raw().as_ptr()) }
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

    /// Y 平面を可変参照する。
    pub fn y_data_mut(&mut self) -> &mut [u8] {
        let raw = self.raw();
        let ptr = unsafe { ffi::webrtc_I420Buffer_MutableDataY(raw.as_ptr()) };
        let stride = unsafe { ffi::webrtc_I420Buffer_StrideY(raw.as_ptr()) } as usize;
        let len = stride * self.height() as usize;
        unsafe { slice::from_raw_parts_mut(ptr, len) }
    }

    /// 連続したメモリとして Y/U/V 全体を参照する。
    pub fn data(&self) -> &[u8] {
        let raw = self.raw();
        let height = self.height() as usize;
        let chroma_height = self.chroma_height() as usize;
        let ptr_y = unsafe { ffi::webrtc_I420Buffer_MutableDataY(raw.as_ptr()) };
        let ptr_u = unsafe { ffi::webrtc_I420Buffer_MutableDataU(raw.as_ptr()) };
        let ptr_v = unsafe { ffi::webrtc_I420Buffer_MutableDataV(raw.as_ptr()) };
        let stride_y = unsafe { ffi::webrtc_I420Buffer_StrideY(raw.as_ptr()) } as usize;
        let stride_u = unsafe { ffi::webrtc_I420Buffer_StrideU(raw.as_ptr()) } as usize;
        let stride_v = unsafe { ffi::webrtc_I420Buffer_StrideV(raw.as_ptr()) } as usize;
        let len_y = stride_y * height;
        let len_u = stride_u * chroma_height;
        let len_v = stride_v * chroma_height;

        debug_assert_eq!(ptr_y.wrapping_add(len_y), ptr_u);
        debug_assert_eq!(ptr_u.wrapping_add(len_u), ptr_v);

        let total_len = len_y
            .checked_add(len_u)
            .and_then(|v| v.checked_add(len_v))
            .expect("I420Buffer data length overflow");
        unsafe { slice::from_raw_parts(ptr_y, total_len) }
    }

    /// 連続したメモリとして Y/U/V 全体を可変参照する。
    pub fn data_mut(&mut self) -> &mut [u8] {
        let raw = self.raw();
        let height = self.height() as usize;
        let chroma_height = self.chroma_height() as usize;
        let ptr_y = unsafe { ffi::webrtc_I420Buffer_MutableDataY(raw.as_ptr()) };
        let ptr_u = unsafe { ffi::webrtc_I420Buffer_MutableDataU(raw.as_ptr()) };
        let ptr_v = unsafe { ffi::webrtc_I420Buffer_MutableDataV(raw.as_ptr()) };
        let stride_y = unsafe { ffi::webrtc_I420Buffer_StrideY(raw.as_ptr()) } as usize;
        let stride_u = unsafe { ffi::webrtc_I420Buffer_StrideU(raw.as_ptr()) } as usize;
        let stride_v = unsafe { ffi::webrtc_I420Buffer_StrideV(raw.as_ptr()) } as usize;
        let len_y = stride_y * height;
        let len_u = stride_u * chroma_height;
        let len_v = stride_v * chroma_height;

        debug_assert_eq!(ptr_y.wrapping_add(len_y), ptr_u);
        debug_assert_eq!(ptr_u.wrapping_add(len_u), ptr_v);

        let total_len = len_y
            .checked_add(len_u)
            .and_then(|v| v.checked_add(len_v))
            .expect("I420Buffer data length overflow");
        unsafe { slice::from_raw_parts_mut(ptr_y, total_len) }
    }

    /// Y/U/V 平面を同時に可変参照する。
    pub fn planes_mut(&mut self) -> (&mut [u8], &mut [u8], &mut [u8]) {
        let raw = self.raw();
        let height = self.height() as usize;
        let chroma_height = self.chroma_height() as usize;
        let ptr_y = unsafe { ffi::webrtc_I420Buffer_MutableDataY(raw.as_ptr()) };
        let ptr_u = unsafe { ffi::webrtc_I420Buffer_MutableDataU(raw.as_ptr()) };
        let ptr_v = unsafe { ffi::webrtc_I420Buffer_MutableDataV(raw.as_ptr()) };
        let stride_y = unsafe { ffi::webrtc_I420Buffer_StrideY(raw.as_ptr()) } as usize;
        let stride_u = unsafe { ffi::webrtc_I420Buffer_StrideU(raw.as_ptr()) } as usize;
        let stride_v = unsafe { ffi::webrtc_I420Buffer_StrideV(raw.as_ptr()) } as usize;

        unsafe {
            (
                slice::from_raw_parts_mut(ptr_y, stride_y * height),
                slice::from_raw_parts_mut(ptr_u, stride_u * chroma_height),
                slice::from_raw_parts_mut(ptr_v, stride_v * chroma_height),
            )
        }
    }

    /// U 平面を参照する。
    pub fn u_data(&self) -> &[u8] {
        let raw = self.raw();
        let ptr = unsafe { ffi::webrtc_I420Buffer_MutableDataU(raw.as_ptr()) };
        let stride = unsafe { ffi::webrtc_I420Buffer_StrideU(raw.as_ptr()) } as usize;
        let h = self.chroma_height() as usize;
        unsafe { slice::from_raw_parts(ptr, stride * h) }
    }

    /// U 平面を可変参照する。
    pub fn u_data_mut(&mut self) -> &mut [u8] {
        let raw = self.raw();
        let ptr = unsafe { ffi::webrtc_I420Buffer_MutableDataU(raw.as_ptr()) };
        let stride = unsafe { ffi::webrtc_I420Buffer_StrideU(raw.as_ptr()) } as usize;
        let h = self.chroma_height() as usize;
        unsafe { slice::from_raw_parts_mut(ptr, stride * h) }
    }

    /// V 平面を参照する。
    pub fn v_data(&self) -> &[u8] {
        let raw = self.raw();
        let ptr = unsafe { ffi::webrtc_I420Buffer_MutableDataV(raw.as_ptr()) };
        let stride = unsafe { ffi::webrtc_I420Buffer_StrideV(raw.as_ptr()) } as usize;
        let h = self.chroma_height() as usize;
        unsafe { slice::from_raw_parts(ptr, stride * h) }
    }

    /// V 平面を可変参照する。
    pub fn v_data_mut(&mut self) -> &mut [u8] {
        let raw = self.raw();
        let ptr = unsafe { ffi::webrtc_I420Buffer_MutableDataV(raw.as_ptr()) };
        let stride = unsafe { ffi::webrtc_I420Buffer_StrideV(raw.as_ptr()) } as usize;
        let h = self.chroma_height() as usize;
        unsafe { slice::from_raw_parts_mut(ptr, stride * h) }
    }

    pub fn as_refcounted_ptr(&self) -> *mut ffi::webrtc_I420Buffer_refcounted {
        self.raw_ref.as_refcounted_ptr()
    }

    fn from_raw_ref(raw_ref: NonNull<ffi::webrtc_I420Buffer_refcounted>) -> Self {
        let raw_ref = ScopedRef::<I420BufferHandle>::from_raw(raw_ref);
        Self { raw_ref }
    }

    fn into_raw_refcounted(self) -> *mut ffi::webrtc_I420Buffer_refcounted {
        let this = std::mem::ManuallyDrop::new(self);
        this.raw_ref.as_refcounted_ptr()
    }

    pub fn cast_to_video_frame_buffer(&self) -> VideoFrameBuffer {
        let raw_ref = NonNull::new(unsafe {
            ffi::webrtc_I420Buffer_refcounted_cast_to_webrtc_VideoFrameBuffer(
                self.raw_ref.as_refcounted_ptr(),
            )
        })
        .expect("BUG: webrtc_I420Buffer_refcounted_cast_to_webrtc_VideoFrameBuffer returned null");
        let raw_ref = ScopedRef::<VideoFrameBufferHandle>::from_raw(raw_ref);
        VideoFrameBuffer { raw_ref }
    }

    pub(crate) fn raw(&self) -> NonNull<ffi::webrtc_I420Buffer> {
        self.raw_ref.raw()
    }
}

/// webrtc::NV12Buffer のラッパー。
pub struct NV12Buffer {
    raw_ref: ScopedRef<NV12BufferHandle>,
}

impl NV12Buffer {
    pub fn new(width: i32, height: i32) -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_NV12Buffer_Create(width, height) })
            .expect("BUG: webrtc_NV12Buffer_Create returned null");
        let raw_ref = ScopedRef::<NV12BufferHandle>::from_raw(raw);
        Self { raw_ref }
    }

    pub fn new_with_strides(width: i32, height: i32, stride_y: i32, stride_uv: i32) -> Self {
        let raw = NonNull::new(unsafe {
            ffi::webrtc_NV12Buffer_CreateWithStrides(width, height, stride_y, stride_uv)
        })
        .expect("BUG: webrtc_NV12Buffer_CreateWithStrides returned null");
        let raw_ref = ScopedRef::<NV12BufferHandle>::from_raw(raw);
        Self { raw_ref }
    }

    pub fn width(&self) -> i32 {
        unsafe { ffi::webrtc_NV12Buffer_width(self.raw().as_ptr()) }
    }

    pub fn height(&self) -> i32 {
        unsafe { ffi::webrtc_NV12Buffer_height(self.raw().as_ptr()) }
    }

    pub fn chroma_width(&self) -> i32 {
        unsafe { ffi::webrtc_NV12Buffer_chroma_width(self.raw().as_ptr()) }
    }

    pub fn chroma_height(&self) -> i32 {
        unsafe { ffi::webrtc_NV12Buffer_chroma_height(self.raw().as_ptr()) }
    }

    pub fn stride_y(&self) -> i32 {
        let raw = self.raw();
        unsafe { ffi::webrtc_NV12Buffer_StrideY(raw.as_ptr()) }
    }

    pub fn stride_uv(&self) -> i32 {
        let raw = self.raw();
        unsafe { ffi::webrtc_NV12Buffer_StrideUV(raw.as_ptr()) }
    }

    pub fn crop_and_scale_from(
        &mut self,
        src: &NV12Buffer,
        offset_x: i32,
        offset_y: i32,
        crop_width: i32,
        crop_height: i32,
    ) {
        let raw = self.raw();
        let src_raw = src.raw();
        unsafe {
            ffi::webrtc_NV12Buffer_CropAndScaleFrom(
                raw.as_ptr(),
                src_raw.as_ptr(),
                offset_x,
                offset_y,
                crop_width,
                crop_height,
            )
        };
    }

    /// Y 平面を参照する。
    pub fn y_data(&self) -> &[u8] {
        let raw = self.raw();
        let ptr = unsafe { ffi::webrtc_NV12Buffer_MutableDataY(raw.as_ptr()) };
        let stride = unsafe { ffi::webrtc_NV12Buffer_StrideY(raw.as_ptr()) } as usize;
        let len = stride * self.height() as usize;
        unsafe { slice::from_raw_parts(ptr, len) }
    }

    /// Y 平面を可変参照する。
    pub fn y_data_mut(&mut self) -> &mut [u8] {
        let raw = self.raw();
        let ptr = unsafe { ffi::webrtc_NV12Buffer_MutableDataY(raw.as_ptr()) };
        let stride = unsafe { ffi::webrtc_NV12Buffer_StrideY(raw.as_ptr()) } as usize;
        let len = stride * self.height() as usize;
        unsafe { slice::from_raw_parts_mut(ptr, len) }
    }

    /// 連続したメモリとして Y/UV 全体を参照する。
    pub fn data(&self) -> &[u8] {
        let raw = self.raw();
        let height = self.height() as usize;
        let chroma_height = self.chroma_height() as usize;
        let ptr_y = unsafe { ffi::webrtc_NV12Buffer_MutableDataY(raw.as_ptr()) };
        let ptr_uv = unsafe { ffi::webrtc_NV12Buffer_MutableDataUV(raw.as_ptr()) };
        let stride_y = unsafe { ffi::webrtc_NV12Buffer_StrideY(raw.as_ptr()) } as usize;
        let stride_uv = unsafe { ffi::webrtc_NV12Buffer_StrideUV(raw.as_ptr()) } as usize;
        let len_y = stride_y * height;
        let len_uv = stride_uv * chroma_height;

        debug_assert_eq!(ptr_y.wrapping_add(len_y), ptr_uv);

        let total_len = len_y
            .checked_add(len_uv)
            .expect("NV12Buffer data length overflow");
        unsafe { slice::from_raw_parts(ptr_y, total_len) }
    }

    /// 連続したメモリとして Y/UV 全体を可変参照する。
    pub fn data_mut(&mut self) -> &mut [u8] {
        let raw = self.raw();
        let height = self.height() as usize;
        let chroma_height = self.chroma_height() as usize;
        let ptr_y = unsafe { ffi::webrtc_NV12Buffer_MutableDataY(raw.as_ptr()) };
        let ptr_uv = unsafe { ffi::webrtc_NV12Buffer_MutableDataUV(raw.as_ptr()) };
        let stride_y = unsafe { ffi::webrtc_NV12Buffer_StrideY(raw.as_ptr()) } as usize;
        let stride_uv = unsafe { ffi::webrtc_NV12Buffer_StrideUV(raw.as_ptr()) } as usize;
        let len_y = stride_y * height;
        let len_uv = stride_uv * chroma_height;

        debug_assert_eq!(ptr_y.wrapping_add(len_y), ptr_uv);

        let total_len = len_y
            .checked_add(len_uv)
            .expect("NV12Buffer data length overflow");
        unsafe { slice::from_raw_parts_mut(ptr_y, total_len) }
    }

    /// UV 平面を参照する。
    pub fn uv_data(&self) -> &[u8] {
        let raw = self.raw();
        let ptr = unsafe { ffi::webrtc_NV12Buffer_MutableDataUV(raw.as_ptr()) };
        let stride = unsafe { ffi::webrtc_NV12Buffer_StrideUV(raw.as_ptr()) } as usize;
        let h = self.chroma_height() as usize;
        unsafe { slice::from_raw_parts(ptr, stride * h) }
    }

    /// UV 平面を可変参照する。
    pub fn uv_data_mut(&mut self) -> &mut [u8] {
        let raw = self.raw();
        let ptr = unsafe { ffi::webrtc_NV12Buffer_MutableDataUV(raw.as_ptr()) };
        let stride = unsafe { ffi::webrtc_NV12Buffer_StrideUV(raw.as_ptr()) } as usize;
        let h = self.chroma_height() as usize;
        unsafe { slice::from_raw_parts_mut(ptr, stride * h) }
    }

    /// Y/UV 平面を同時に可変参照する。
    pub fn planes_mut(&mut self) -> (&mut [u8], &mut [u8]) {
        let raw = self.raw();
        let height = self.height() as usize;
        let chroma_height = self.chroma_height() as usize;
        let ptr_y = unsafe { ffi::webrtc_NV12Buffer_MutableDataY(raw.as_ptr()) };
        let ptr_uv = unsafe { ffi::webrtc_NV12Buffer_MutableDataUV(raw.as_ptr()) };
        let stride_y = unsafe { ffi::webrtc_NV12Buffer_StrideY(raw.as_ptr()) } as usize;
        let stride_uv = unsafe { ffi::webrtc_NV12Buffer_StrideUV(raw.as_ptr()) } as usize;

        unsafe {
            (
                slice::from_raw_parts_mut(ptr_y, stride_y * height),
                slice::from_raw_parts_mut(ptr_uv, stride_uv * chroma_height),
            )
        }
    }

    pub fn as_refcounted_ptr(&self) -> *mut ffi::webrtc_NV12Buffer_refcounted {
        self.raw_ref.as_refcounted_ptr()
    }

    fn from_raw_ref(raw_ref: NonNull<ffi::webrtc_NV12Buffer_refcounted>) -> Self {
        let raw_ref = ScopedRef::<NV12BufferHandle>::from_raw(raw_ref);
        Self { raw_ref }
    }

    pub fn cast_to_video_frame_buffer(&self) -> VideoFrameBuffer {
        let raw_ref = NonNull::new(unsafe {
            ffi::webrtc_NV12Buffer_refcounted_cast_to_webrtc_VideoFrameBuffer(
                self.raw_ref.as_refcounted_ptr(),
            )
        })
        .expect("BUG: webrtc_NV12Buffer_refcounted_cast_to_webrtc_VideoFrameBuffer returned null");
        let raw_ref = ScopedRef::<VideoFrameBufferHandle>::from_raw(raw_ref);
        VideoFrameBuffer { raw_ref }
    }

    pub(crate) fn raw(&self) -> NonNull<ffi::webrtc_NV12Buffer> {
        self.raw_ref.raw()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoFrameBufferKind {
    Native,
    I420,
    I420A,
    I422,
    I444,
    I010,
    I210,
    I410,
    Nv12,
    Unknown(i32),
}

impl VideoFrameBufferKind {
    pub(crate) fn from_raw(value: i32) -> Self {
        unsafe {
            if value == ffi::webrtc_VideoFrameBuffer_Type_kNative {
                Self::Native
            } else if value == ffi::webrtc_VideoFrameBuffer_Type_kI420 {
                Self::I420
            } else if value == ffi::webrtc_VideoFrameBuffer_Type_kI420A {
                Self::I420A
            } else if value == ffi::webrtc_VideoFrameBuffer_Type_kI422 {
                Self::I422
            } else if value == ffi::webrtc_VideoFrameBuffer_Type_kI444 {
                Self::I444
            } else if value == ffi::webrtc_VideoFrameBuffer_Type_kI010 {
                Self::I010
            } else if value == ffi::webrtc_VideoFrameBuffer_Type_kI210 {
                Self::I210
            } else if value == ffi::webrtc_VideoFrameBuffer_Type_kI410 {
                Self::I410
            } else if value == ffi::webrtc_VideoFrameBuffer_Type_kNV12 {
                Self::Nv12
            } else {
                Self::Unknown(value)
            }
        }
    }

    pub(crate) fn to_raw(self) -> i32 {
        unsafe {
            match self {
                Self::Native => ffi::webrtc_VideoFrameBuffer_Type_kNative,
                Self::I420 => ffi::webrtc_VideoFrameBuffer_Type_kI420,
                Self::I420A => ffi::webrtc_VideoFrameBuffer_Type_kI420A,
                Self::I422 => ffi::webrtc_VideoFrameBuffer_Type_kI422,
                Self::I444 => ffi::webrtc_VideoFrameBuffer_Type_kI444,
                Self::I010 => ffi::webrtc_VideoFrameBuffer_Type_kI010,
                Self::I210 => ffi::webrtc_VideoFrameBuffer_Type_kI210,
                Self::I410 => ffi::webrtc_VideoFrameBuffer_Type_kI410,
                Self::Nv12 => ffi::webrtc_VideoFrameBuffer_Type_kNV12,
                Self::Unknown(v) => v,
            }
        }
    }
}

#[doc(hidden)]
pub trait VideoFrameBufferHandlerAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any> VideoFrameBufferHandlerAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub trait VideoFrameBufferHandler: Send + VideoFrameBufferHandlerAny {
    fn kind(&self) -> VideoFrameBufferKind {
        VideoFrameBufferKind::Native
    }
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn to_i420(&mut self) -> Option<I420Buffer>;
    // None を返すとデフォルトの実装が呼ばれる。
    // デフォルトの実装は to_i420() で I420Buffer を作成してからそれをクロップ＆スケールする。
    fn crop_and_scale(
        &mut self,
        _offset_x: i32,
        _offset_y: i32,
        _crop_width: i32,
        _crop_height: i32,
        _scaled_width: i32,
        _scaled_height: i32,
    ) -> Option<VideoFrameBuffer> {
        None
    }
}

struct VideoFrameBufferHandlerState {
    handler: Box<dyn VideoFrameBufferHandler>,
    #[cfg(debug_assertions)]
    callback_thread: Option<std::thread::ThreadId>,
}

#[cfg(debug_assertions)]
fn assert_video_frame_buffer_handler_thread(state: &mut VideoFrameBufferHandlerState) {
    let current = std::thread::current().id();
    if let Some(thread) = state.callback_thread {
        assert_eq!(
            thread, current,
            "video_frame_buffer callback called from multiple threads",
        );
    } else {
        state.callback_thread = Some(current);
    }
}

unsafe extern "C" fn video_frame_buffer_type(user_data: *mut c_void) -> i32 {
    assert!(
        !user_data.is_null(),
        "video_frame_buffer_type: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoFrameBufferHandlerState) };
    #[cfg(debug_assertions)]
    assert_video_frame_buffer_handler_thread(state);
    state.handler.kind().to_raw()
}

unsafe extern "C" fn video_frame_buffer_width(user_data: *mut c_void) -> i32 {
    assert!(
        !user_data.is_null(),
        "video_frame_buffer_width: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoFrameBufferHandlerState) };
    #[cfg(debug_assertions)]
    assert_video_frame_buffer_handler_thread(state);
    state.handler.width()
}

unsafe extern "C" fn video_frame_buffer_height(user_data: *mut c_void) -> i32 {
    assert!(
        !user_data.is_null(),
        "video_frame_buffer_height: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoFrameBufferHandlerState) };
    #[cfg(debug_assertions)]
    assert_video_frame_buffer_handler_thread(state);
    state.handler.height()
}

unsafe extern "C" fn video_frame_buffer_to_i420(
    user_data: *mut c_void,
) -> *mut ffi::webrtc_I420Buffer_refcounted {
    assert!(
        !user_data.is_null(),
        "video_frame_buffer_to_i420: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoFrameBufferHandlerState) };
    #[cfg(debug_assertions)]
    assert_video_frame_buffer_handler_thread(state);
    match state.handler.to_i420() {
        Some(buffer) => buffer.into_raw_refcounted(),
        None => std::ptr::null_mut(),
    }
}

unsafe extern "C" fn video_frame_buffer_crop_and_scale(
    raw: *mut ffi::webrtc_VideoFrameBuffer,
    offset_x: i32,
    offset_y: i32,
    crop_width: i32,
    crop_height: i32,
    scaled_width: i32,
    scaled_height: i32,
    user_data: *mut c_void,
) -> *mut ffi::webrtc_VideoFrameBuffer_refcounted {
    assert!(
        !raw.is_null(),
        "video_frame_buffer_crop_and_scale: raw is null"
    );
    assert!(
        !user_data.is_null(),
        "video_frame_buffer_crop_and_scale: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoFrameBufferHandlerState) };
    #[cfg(debug_assertions)]
    assert_video_frame_buffer_handler_thread(state);
    if let Some(buffer) = state.handler.crop_and_scale(
        offset_x,
        offset_y,
        crop_width,
        crop_height,
        scaled_width,
        scaled_height,
    ) {
        return buffer.into_raw_refcounted();
    }
    std::ptr::null_mut()
}

unsafe extern "C" fn video_frame_buffer_on_destroy(user_data: *mut c_void) {
    assert!(
        !user_data.is_null(),
        "video_frame_buffer_on_destroy: user_data is null"
    );
    let _ = unsafe { Box::from_raw(user_data as *mut VideoFrameBufferHandlerState) };
}

/// webrtc::VideoFrameBuffer のラッパー。
pub struct VideoFrameBuffer {
    raw_ref: ScopedRef<VideoFrameBufferHandle>,
}

impl VideoFrameBuffer {
    pub fn new_with_handler(handler: Box<dyn VideoFrameBufferHandler>) -> Self {
        let state = Box::new(VideoFrameBufferHandlerState {
            handler,
            #[cfg(debug_assertions)]
            callback_thread: None,
        });
        let user_data = Box::into_raw(state) as *mut c_void;
        let cbs = ffi::webrtc_VideoFrameBuffer_cbs {
            type_: Some(video_frame_buffer_type),
            width: Some(video_frame_buffer_width),
            height: Some(video_frame_buffer_height),
            ToI420: Some(video_frame_buffer_to_i420),
            CropAndScale: Some(video_frame_buffer_crop_and_scale),
            OnDestroy: Some(video_frame_buffer_on_destroy),
        };
        let raw_ref = match NonNull::new(unsafe {
            ffi::webrtc_VideoFrameBuffer_make_ref_counted(&cbs, user_data)
        }) {
            Some(raw_ref) => raw_ref,
            None => {
                let _ = unsafe { Box::from_raw(user_data as *mut VideoFrameBufferHandlerState) };
                panic!("BUG: webrtc_VideoFrameBuffer_make_ref_counted returned null");
            }
        };
        let raw_ref = ScopedRef::<VideoFrameBufferHandle>::from_raw(raw_ref);
        Self { raw_ref }
    }

    pub fn width(&self) -> i32 {
        unsafe { ffi::webrtc_VideoFrameBuffer_width(self.raw().as_ptr()) }
    }

    pub fn height(&self) -> i32 {
        unsafe { ffi::webrtc_VideoFrameBuffer_height(self.raw().as_ptr()) }
    }

    pub fn kind(&self) -> VideoFrameBufferKind {
        let value = unsafe { ffi::webrtc_VideoFrameBuffer_type(self.raw().as_ptr()) };
        VideoFrameBufferKind::from_raw(value)
    }

    /// # Safety
    /// 同一実体の `VideoFrameBuffer` へ同時アクセスしないこと。
    /// 特に callback 側や別 clone から mutable にアクセスされないことを呼び出し側が保証する必要があります。
    pub unsafe fn as_native_ref<T: VideoFrameBufferHandler + 'static>(&self) -> Option<&T> {
        let user_data = unsafe { ffi::webrtc_VideoFrameBuffer_get_user_data(self.raw().as_ptr()) };
        if user_data.is_null() {
            return None;
        }
        let state = unsafe { &*(user_data as *const VideoFrameBufferHandlerState) };
        state.handler.as_ref().as_any().downcast_ref::<T>()
    }

    /// # Safety
    /// 同一実体の `VideoFrameBuffer` への参照が存在しないこと。
    /// 特に callback 側や別 clone から同時に参照されないことを呼び出し側が保証する必要があります。
    pub unsafe fn as_native_mut<T: VideoFrameBufferHandler + 'static>(&mut self) -> Option<&mut T> {
        let user_data = unsafe { ffi::webrtc_VideoFrameBuffer_get_user_data(self.raw().as_ptr()) };
        if user_data.is_null() {
            return None;
        }
        let state = unsafe { &mut *(user_data as *mut VideoFrameBufferHandlerState) };
        state.handler.as_mut().as_any_mut().downcast_mut::<T>()
    }

    pub fn as_i420(&self) -> Option<I420Buffer> {
        let raw_ref = NonNull::new(unsafe {
            ffi::webrtc_VideoFrameBuffer_cast_to_webrtc_I420Buffer(self.raw().as_ptr())
        })?;
        Some(I420Buffer::from_raw_ref(raw_ref))
    }

    pub fn as_nv12(&self) -> Option<NV12Buffer> {
        let raw_ref = NonNull::new(unsafe {
            ffi::webrtc_VideoFrameBuffer_cast_to_webrtc_NV12Buffer(self.raw().as_ptr())
        })?;
        Some(NV12Buffer::from_raw_ref(raw_ref))
    }

    pub fn to_i420(&mut self) -> Option<I420Buffer> {
        let raw_ref =
            NonNull::new(unsafe { ffi::webrtc_VideoFrameBuffer_ToI420(self.raw().as_ptr()) })?;
        Some(I420Buffer::from_raw_ref(raw_ref))
    }

    pub fn crop_and_scale(
        &mut self,
        offset_x: i32,
        offset_y: i32,
        crop_width: i32,
        crop_height: i32,
        scaled_width: i32,
        scaled_height: i32,
    ) -> Option<VideoFrameBuffer> {
        let raw_ref = NonNull::new(unsafe {
            ffi::webrtc_VideoFrameBuffer_CropAndScale(
                self.raw().as_ptr(),
                offset_x,
                offset_y,
                crop_width,
                crop_height,
                scaled_width,
                scaled_height,
            )
        })
        .or_else(|| {
            NonNull::new(unsafe {
                ffi::webrtc_VideoFrameBuffer_DefaultCropAndScale(
                    self.raw().as_ptr(),
                    offset_x,
                    offset_y,
                    crop_width,
                    crop_height,
                    scaled_width,
                    scaled_height,
                )
            })
        })?;
        let raw_ref = ScopedRef::<VideoFrameBufferHandle>::from_raw(raw_ref);
        Some(VideoFrameBuffer { raw_ref })
    }

    pub fn scale(&mut self, scaled_width: i32, scaled_height: i32) -> Option<VideoFrameBuffer> {
        self.crop_and_scale(
            0,
            0,
            self.width(),
            self.height(),
            scaled_width,
            scaled_height,
        )
    }

    pub fn as_refcounted_ptr(&self) -> *mut ffi::webrtc_VideoFrameBuffer_refcounted {
        self.raw_ref.as_refcounted_ptr()
    }

    pub(crate) fn raw(&self) -> NonNull<ffi::webrtc_VideoFrameBuffer> {
        self.raw_ref.raw()
    }

    fn into_raw_refcounted(self) -> *mut ffi::webrtc_VideoFrameBuffer_refcounted {
        let this = std::mem::ManuallyDrop::new(self);
        this.raw_ref.as_refcounted_ptr()
    }
}

impl Clone for VideoFrameBuffer {
    fn clone(&self) -> Self {
        Self {
            raw_ref: ScopedRef::clone(&self.raw_ref),
        }
    }
}

fn duration_to_timestamp_us(value: Duration) -> i64 {
    i64::try_from(value.as_micros()).expect("Duration microseconds overflowed i64")
}

fn timestamp_us_to_duration(value: i64) -> Duration {
    let micros = u64::try_from(value).expect("BUG: timestamp_us must be non-negative");
    Duration::from_micros(micros)
}

pub struct ColorSpace {
    raw_unique: NonNull<ffi::webrtc_ColorSpace_unique>,
}

impl Default for ColorSpace {
    fn default() -> Self {
        Self::new()
    }
}

impl ColorSpace {
    pub fn new() -> Self {
        let raw_unique = NonNull::new(unsafe { ffi::webrtc_ColorSpace_new() })
            .expect("BUG: webrtc_ColorSpace_new returned null");
        Self { raw_unique }
    }

    /// # Safety
    /// `raw_unique` は有効な `webrtc_ColorSpace_unique` を指す必要があります。
    unsafe fn from_raw_unique(raw_unique: NonNull<ffi::webrtc_ColorSpace_unique>) -> Self {
        Self { raw_unique }
    }

    pub fn as_string(&self) -> Result<String> {
        let raw = unsafe { ffi::webrtc_ColorSpace_AsString(self.raw().as_ptr()) };
        let raw = NonNull::new(raw).expect("BUG: webrtc_ColorSpace_AsString returned null");
        CxxString::from_unique(raw).to_string()
    }

    pub(crate) fn as_ptr(&self) -> *mut ffi::webrtc_ColorSpace {
        self.raw().as_ptr()
    }

    pub(crate) fn raw(&self) -> NonNull<ffi::webrtc_ColorSpace> {
        let raw = unsafe { ffi::webrtc_ColorSpace_unique_get(self.raw_unique.as_ptr()) };
        NonNull::new(raw).expect("BUG: webrtc_ColorSpace_unique_get returned null")
    }
}

impl Drop for ColorSpace {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_ColorSpace_unique_delete(self.raw_unique.as_ptr()) };
    }
}

unsafe impl Send for ColorSpace {}

pub struct VideoFrameUpdateRect {
    raw_unique: NonNull<ffi::webrtc_VideoFrame_UpdateRect_unique>,
}

impl Default for VideoFrameUpdateRect {
    fn default() -> Self {
        Self::new()
    }
}

impl VideoFrameUpdateRect {
    pub fn new() -> Self {
        let raw_unique = NonNull::new(unsafe { ffi::webrtc_VideoFrame_UpdateRect_new() })
            .expect("BUG: webrtc_VideoFrame_UpdateRect_new returned null");
        Self { raw_unique }
    }

    /// # Safety
    /// `raw_unique` は有効な `webrtc_VideoFrame_UpdateRect_unique` を指す必要があります。
    unsafe fn from_raw_unique(
        raw_unique: NonNull<ffi::webrtc_VideoFrame_UpdateRect_unique>,
    ) -> Self {
        Self { raw_unique }
    }

    pub fn offset_x(&self) -> i32 {
        unsafe { ffi::webrtc_VideoFrame_UpdateRect_get_offset_x(self.raw().as_ptr()) }
    }

    pub fn set_offset_x(&mut self, value: i32) {
        unsafe { ffi::webrtc_VideoFrame_UpdateRect_set_offset_x(self.raw().as_ptr(), value) };
    }

    pub fn offset_y(&self) -> i32 {
        unsafe { ffi::webrtc_VideoFrame_UpdateRect_get_offset_y(self.raw().as_ptr()) }
    }

    pub fn set_offset_y(&mut self, value: i32) {
        unsafe { ffi::webrtc_VideoFrame_UpdateRect_set_offset_y(self.raw().as_ptr(), value) };
    }

    pub fn width(&self) -> i32 {
        unsafe { ffi::webrtc_VideoFrame_UpdateRect_get_width(self.raw().as_ptr()) }
    }

    pub fn set_width(&mut self, value: i32) {
        unsafe { ffi::webrtc_VideoFrame_UpdateRect_set_width(self.raw().as_ptr(), value) };
    }

    pub fn height(&self) -> i32 {
        unsafe { ffi::webrtc_VideoFrame_UpdateRect_get_height(self.raw().as_ptr()) }
    }

    pub fn set_height(&mut self, value: i32) {
        unsafe { ffi::webrtc_VideoFrame_UpdateRect_set_height(self.raw().as_ptr(), value) };
    }

    pub(crate) fn as_ptr(&self) -> *mut ffi::webrtc_VideoFrame_UpdateRect {
        self.raw().as_ptr()
    }

    fn raw(&self) -> NonNull<ffi::webrtc_VideoFrame_UpdateRect> {
        let raw = unsafe { ffi::webrtc_VideoFrame_UpdateRect_unique_get(self.raw_unique.as_ptr()) };
        NonNull::new(raw).expect("BUG: webrtc_VideoFrame_UpdateRect_unique_get returned null")
    }
}

impl Drop for VideoFrameUpdateRect {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_VideoFrame_UpdateRect_unique_delete(self.raw_unique.as_ptr()) };
    }
}

unsafe impl Send for VideoFrameUpdateRect {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoRotation {
    R0,
    R90,
    R180,
    R270,
    Unknown(i32),
}

impl VideoRotation {
    pub(crate) fn from_raw(value: i32) -> Self {
        unsafe {
            if value == ffi::webrtc_VideoRotation_0 {
                Self::R0
            } else if value == ffi::webrtc_VideoRotation_90 {
                Self::R90
            } else if value == ffi::webrtc_VideoRotation_180 {
                Self::R180
            } else if value == ffi::webrtc_VideoRotation_270 {
                Self::R270
            } else {
                Self::Unknown(value)
            }
        }
    }

    pub(crate) fn to_raw(self) -> i32 {
        unsafe {
            match self {
                Self::R0 => ffi::webrtc_VideoRotation_0,
                Self::R90 => ffi::webrtc_VideoRotation_90,
                Self::R180 => ffi::webrtc_VideoRotation_180,
                Self::R270 => ffi::webrtc_VideoRotation_270,
                Self::Unknown(v) => v,
            }
        }
    }
}

pub struct VideoFrameBuilder {
    raw_unique: NonNull<ffi::webrtc_VideoFrameBuilder_unique>,
}

impl VideoFrameBuilder {
    fn new(buffer: &VideoFrameBuffer) -> Self {
        let raw_unique =
            NonNull::new(unsafe { ffi::webrtc_VideoFrameBuilder_new(buffer.as_refcounted_ptr()) })
                .expect("BUG: webrtc_VideoFrameBuilder_new returned null");
        Self { raw_unique }
    }

    fn raw(&self) -> NonNull<ffi::webrtc_VideoFrameBuilder> {
        let raw = unsafe { ffi::webrtc_VideoFrameBuilder_unique_get(self.raw_unique.as_ptr()) };
        NonNull::new(raw).expect("BUG: webrtc_VideoFrameBuilder_unique_get returned null")
    }

    pub fn set_timestamp_ms(&mut self, timestamp_ms: i64) -> &mut Self {
        unsafe {
            ffi::webrtc_VideoFrameBuilder_set_timestamp_ms(self.raw().as_ptr(), timestamp_ms)
        };
        self
    }

    pub fn set_timestamp_us(&mut self, timestamp_us: i64) -> &mut Self {
        unsafe {
            ffi::webrtc_VideoFrameBuilder_set_timestamp_us(self.raw().as_ptr(), timestamp_us)
        };
        self
    }

    pub fn set_presentation_timestamp(&mut self, value: Option<Duration>) -> &mut Self {
        match value {
            Some(value) => unsafe {
                ffi::webrtc_VideoFrameBuilder_set_presentation_timestamp_us(
                    self.raw().as_ptr(),
                    1,
                    duration_to_timestamp_us(value),
                )
            },
            None => unsafe {
                ffi::webrtc_VideoFrameBuilder_set_presentation_timestamp_us(
                    self.raw().as_ptr(),
                    0,
                    0,
                )
            },
        }
        self
    }

    pub fn set_reference_time(&mut self, value: Option<Duration>) -> &mut Self {
        match value {
            Some(value) => unsafe {
                ffi::webrtc_VideoFrameBuilder_set_reference_time_us(
                    self.raw().as_ptr(),
                    1,
                    duration_to_timestamp_us(value),
                )
            },
            None => unsafe {
                ffi::webrtc_VideoFrameBuilder_set_reference_time_us(self.raw().as_ptr(), 0, 0)
            },
        }
        self
    }

    pub fn set_rtp_timestamp(&mut self, value: u32) -> &mut Self {
        unsafe { ffi::webrtc_VideoFrameBuilder_set_rtp_timestamp(self.raw().as_ptr(), value) };
        self
    }

    pub fn set_timestamp_rtp(&mut self, value: u32) -> &mut Self {
        unsafe { ffi::webrtc_VideoFrameBuilder_set_timestamp_rtp(self.raw().as_ptr(), value) };
        self
    }

    pub fn set_ntp_time_ms(&mut self, value: i64) -> &mut Self {
        unsafe { ffi::webrtc_VideoFrameBuilder_set_ntp_time_ms(self.raw().as_ptr(), value) };
        self
    }

    pub fn set_rotation(&mut self, value: VideoRotation) -> &mut Self {
        unsafe { ffi::webrtc_VideoFrameBuilder_set_rotation(self.raw().as_ptr(), value.to_raw()) };
        self
    }

    pub fn set_color_space(&mut self, value: Option<&ColorSpace>) -> &mut Self {
        match value {
            Some(value) => unsafe {
                ffi::webrtc_VideoFrameBuilder_set_color_space(
                    self.raw().as_ptr(),
                    1,
                    value.as_ptr(),
                )
            },
            None => unsafe {
                ffi::webrtc_VideoFrameBuilder_set_color_space(
                    self.raw().as_ptr(),
                    0,
                    std::ptr::null(),
                )
            },
        }
        self
    }

    pub fn set_id(&mut self, value: u16) -> &mut Self {
        unsafe { ffi::webrtc_VideoFrameBuilder_set_id(self.raw().as_ptr(), value) };
        self
    }

    pub fn set_update_rect(&mut self, value: Option<&VideoFrameUpdateRect>) -> &mut Self {
        match value {
            Some(value) => unsafe {
                ffi::webrtc_VideoFrameBuilder_set_update_rect(
                    self.raw().as_ptr(),
                    1,
                    value.as_ptr(),
                )
            },
            None => unsafe {
                ffi::webrtc_VideoFrameBuilder_set_update_rect(
                    self.raw().as_ptr(),
                    0,
                    std::ptr::null(),
                )
            },
        }
        self
    }

    pub fn set_is_repeat_frame(&mut self, value: bool) -> &mut Self {
        unsafe {
            ffi::webrtc_VideoFrameBuilder_set_is_repeat_frame(
                self.raw().as_ptr(),
                if value { 1 } else { 0 },
            )
        };
        self
    }

    pub fn build(&mut self) -> VideoFrame {
        let raw_unique =
            NonNull::new(unsafe { ffi::webrtc_VideoFrameBuilder_build(self.raw().as_ptr()) })
                .expect("BUG: webrtc_VideoFrameBuilder_build returned null");
        VideoFrame { raw_unique }
    }
}

impl Drop for VideoFrameBuilder {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_VideoFrameBuilder_unique_delete(self.raw_unique.as_ptr()) };
    }
}

unsafe impl Send for VideoFrameBuilder {}

/// webrtc::VideoFrame のラッパー。
pub struct VideoFrame {
    raw_unique: NonNull<ffi::webrtc_VideoFrame_unique>,
}

impl VideoFrame {
    pub fn builder(buffer: &VideoFrameBuffer) -> VideoFrameBuilder {
        VideoFrameBuilder::new(buffer)
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

    pub fn rtp_timestamp(&self) -> u32 {
        self.as_ref().rtp_timestamp()
    }

    pub fn id(&self) -> u16 {
        self.as_ref().id()
    }

    pub fn ntp_time_ms(&self) -> i64 {
        self.as_ref().ntp_time_ms()
    }

    pub fn rotation(&self) -> VideoRotation {
        self.as_ref().rotation()
    }

    pub fn presentation_timestamp(&self) -> Option<Duration> {
        self.as_ref().presentation_timestamp()
    }

    pub fn reference_time(&self) -> Option<Duration> {
        self.as_ref().reference_time()
    }

    pub fn color_space(&self) -> Option<ColorSpace> {
        self.as_ref().color_space()
    }

    pub fn has_update_rect(&self) -> bool {
        self.as_ref().has_update_rect()
    }

    pub fn update_rect(&self) -> VideoFrameUpdateRect {
        self.as_ref().update_rect()
    }

    pub fn is_repeat_frame(&self) -> bool {
        self.as_ref().is_repeat_frame()
    }

    /// VideoFrameBuffer を取得する。
    pub fn buffer(&self) -> VideoFrameBuffer {
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

impl Clone for VideoFrame {
    fn clone(&self) -> Self {
        let raw = unsafe { ffi::webrtc_VideoFrame_copy(self.raw().as_ptr()) };
        Self {
            raw_unique: NonNull::new(raw)
                .expect("BUG: webrtc_VideoFrame_copy が null を返しました"),
        }
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

    pub fn rtp_timestamp(&self) -> u32 {
        unsafe { ffi::webrtc_VideoFrame_timestamp_rtp(self.raw.as_ptr()) }
    }

    pub fn id(&self) -> u16 {
        unsafe { ffi::webrtc_VideoFrame_id(self.raw.as_ptr()) }
    }

    pub fn ntp_time_ms(&self) -> i64 {
        unsafe { ffi::webrtc_VideoFrame_ntp_time_ms(self.raw.as_ptr()) }
    }

    pub fn rotation(&self) -> VideoRotation {
        let value = unsafe { ffi::webrtc_VideoFrame_rotation(self.raw.as_ptr()) };
        VideoRotation::from_raw(value)
    }

    pub fn presentation_timestamp(&self) -> Option<Duration> {
        let mut has = 0;
        let mut value = 0;
        unsafe {
            ffi::webrtc_VideoFrame_presentation_timestamp_us(
                self.raw.as_ptr(),
                &mut has,
                &mut value,
            );
        }
        if has == 0 {
            None
        } else {
            Some(timestamp_us_to_duration(value))
        }
    }

    pub fn reference_time(&self) -> Option<Duration> {
        let mut has = 0;
        let mut value = 0;
        unsafe {
            ffi::webrtc_VideoFrame_reference_time_us(self.raw.as_ptr(), &mut has, &mut value);
        }
        if has == 0 {
            None
        } else {
            Some(timestamp_us_to_duration(value))
        }
    }

    pub fn color_space(&self) -> Option<ColorSpace> {
        let mut has = 0;
        let mut raw_unique: *mut ffi::webrtc_ColorSpace_unique = std::ptr::null_mut();
        unsafe {
            ffi::webrtc_VideoFrame_color_space(self.raw.as_ptr(), &mut has, &mut raw_unique);
        }
        if has == 0 {
            return None;
        }
        let raw_unique =
            NonNull::new(raw_unique).expect("BUG: webrtc_VideoFrame_color_space returned null");
        Some(unsafe { ColorSpace::from_raw_unique(raw_unique) })
    }

    pub fn has_update_rect(&self) -> bool {
        unsafe { ffi::webrtc_VideoFrame_has_update_rect(self.raw.as_ptr()) != 0 }
    }

    pub fn update_rect(&self) -> VideoFrameUpdateRect {
        let raw_unique =
            NonNull::new(unsafe { ffi::webrtc_VideoFrame_update_rect(self.raw.as_ptr()) })
                .expect("BUG: webrtc_VideoFrame_update_rect returned null");
        unsafe { VideoFrameUpdateRect::from_raw_unique(raw_unique) }
    }

    pub fn is_repeat_frame(&self) -> bool {
        unsafe { ffi::webrtc_VideoFrame_is_repeat_frame(self.raw.as_ptr()) != 0 }
    }

    pub fn buffer(&self) -> VideoFrameBuffer {
        let buf =
            NonNull::new(unsafe { ffi::webrtc_VideoFrame_video_frame_buffer(self.raw.as_ptr()) })
                .expect("BUG: webrtc_VideoFrame_video_frame_buffer が null を返しました");
        let raw_ref = ScopedRef::<VideoFrameBufferHandle>::from_raw(buf);
        VideoFrameBuffer { raw_ref }
    }

    pub fn to_owned(&self) -> VideoFrame {
        let raw = unsafe { ffi::webrtc_VideoFrame_copy(self.raw.as_ptr()) };
        VideoFrame {
            raw_unique: NonNull::new(raw)
                .expect("BUG: webrtc_VideoFrame_copy が null を返しました"),
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut ffi::webrtc_VideoFrame {
        self.raw.as_ptr()
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

    pub fn push(&mut self, value: VideoFrameType) {
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
    pub fn as_str(self) -> Option<&'static str> {
        match self {
            Self::Generic => Some("Generic"),
            Self::Vp8 => Some("VP8"),
            Self::Vp9 => Some("VP9"),
            Self::Av1 => Some("AV1"),
            Self::H264 => Some("H264"),
            Self::H265 => Some("H265"),
            Self::Unknown(_) => None,
        }
    }

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

impl TryFrom<&str> for VideoCodecType {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "Generic" => Ok(Self::Generic),
            "VP8" => Ok(Self::Vp8),
            "VP9" => Ok(Self::Vp9),
            "AV1" => Ok(Self::Av1),
            "H264" => Ok(Self::H264),
            "H265" => Ok(Self::H265),
            _ => Err(Error::InvalidVideoCodecType(value.to_string())),
        }
    }
}

impl std::str::FromStr for VideoCodecType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::try_from(s)
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

    pub(crate) fn as_ptr(&self) -> *mut ffi::webrtc_VideoCodec {
        self.raw.as_ptr()
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

    pub fn set_encoded_data(&mut self, encoded_data: &EncodedImageBuffer) {
        unsafe {
            ffi::webrtc_EncodedImage_set_encoded_data(self.raw.as_ptr(), encoded_data.as_ptr())
        };
    }

    pub fn set_rtp_timestamp(&mut self, rtp_timestamp: u32) {
        unsafe { ffi::webrtc_EncodedImage_set_rtp_timestamp(self.raw.as_ptr(), rtp_timestamp) };
    }

    pub fn set_encoded_width(&mut self, encoded_width: u32) {
        unsafe { ffi::webrtc_EncodedImage_set_encoded_width(self.raw.as_ptr(), encoded_width) };
    }

    pub fn set_encoded_height(&mut self, encoded_height: u32) {
        unsafe { ffi::webrtc_EncodedImage_set_encoded_height(self.raw.as_ptr(), encoded_height) };
    }

    pub fn set_frame_type(&mut self, frame_type: VideoFrameType) {
        unsafe { ffi::webrtc_EncodedImage_set_frame_type(self.raw.as_ptr(), frame_type.to_raw()) };
    }

    pub fn set_qp(&mut self, qp: i32) {
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
