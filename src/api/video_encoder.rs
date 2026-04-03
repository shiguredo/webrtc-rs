use super::video_codec_common::{
    EncodedImageRef, SdpVideoFormat, SdpVideoFormatRef, VideoCodecRef, VideoCodecStatus,
    VideoCodecType, VideoFrameBufferKind, VideoFrameRef, VideoFrameTypeVectorRef,
};
use crate::{CxxString, EnvironmentRef, Result, ffi};
use std::marker::PhantomData;
use std::os::raw::c_void;
use std::ptr::NonNull;

pub struct VideoEncoderFramerateFractionInlinedVectorRef<'a> {
    raw: NonNull<ffi::webrtc_VideoEncoder_FramerateFraction_inlined_vector>,
    _marker: PhantomData<&'a ffi::webrtc_VideoEncoder_EncoderInfo>,
}

impl<'a> VideoEncoderFramerateFractionInlinedVectorRef<'a> {
    /// # Safety
    /// `raw` は有効な `webrtc_VideoEncoder_FramerateFraction_inlined_vector` を指す必要があります。
    pub unsafe fn from_raw(
        raw: NonNull<ffi::webrtc_VideoEncoder_FramerateFraction_inlined_vector>,
    ) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        let len = unsafe {
            ffi::webrtc_VideoEncoder_FramerateFraction_inlined_vector_size(self.raw.as_ptr())
        };
        len.max(0) as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: usize) -> Option<u8> {
        if index >= self.len() {
            return None;
        }
        let raw = unsafe {
            ffi::webrtc_VideoEncoder_FramerateFraction_inlined_vector_get(
                self.raw.as_ptr(),
                index as i32,
            )
        };
        let raw = NonNull::new(raw)?;
        let value = unsafe { ffi::webrtc_VideoEncoder_FramerateFraction_value(raw.as_ptr()) };
        Some(value.clamp(0, u8::MAX as i32) as u8)
    }

    pub fn push(&mut self, value: u8) {
        unsafe {
            ffi::webrtc_VideoEncoder_FramerateFraction_inlined_vector_push_back_value(
                self.raw.as_ptr(),
                value as i32,
            )
        };
    }

    pub fn set(&mut self, index: usize, value: u8) -> bool {
        if index >= self.len() {
            return false;
        }
        unsafe {
            ffi::webrtc_VideoEncoder_FramerateFraction_inlined_vector_set_value(
                self.raw.as_ptr(),
                index as i32,
                value as i32,
            )
        };
        true
    }

    pub fn resize(&mut self, len: usize) {
        let len = i32::try_from(len).unwrap_or(i32::MAX);
        unsafe {
            ffi::webrtc_VideoEncoder_FramerateFraction_inlined_vector_resize(self.raw.as_ptr(), len)
        };
    }

    pub fn clear(&mut self) {
        unsafe {
            ffi::webrtc_VideoEncoder_FramerateFraction_inlined_vector_clear(self.raw.as_ptr())
        };
    }
}

unsafe impl<'a> Send for VideoEncoderFramerateFractionInlinedVectorRef<'a> {}

pub struct VideoFrameBufferKindInlinedVectorRef<'a> {
    raw: NonNull<ffi::webrtc_VideoFrameBuffer_Type_inlined_vector>,
    _marker: PhantomData<&'a ffi::webrtc_VideoEncoder_EncoderInfo>,
}

impl<'a> VideoFrameBufferKindInlinedVectorRef<'a> {
    /// # Safety
    /// `raw` は有効な `webrtc_VideoFrameBuffer_Type_inlined_vector` を指す必要があります。
    pub unsafe fn from_raw(raw: NonNull<ffi::webrtc_VideoFrameBuffer_Type_inlined_vector>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        let len =
            unsafe { ffi::webrtc_VideoFrameBuffer_Type_inlined_vector_size(self.raw.as_ptr()) };
        len.max(0) as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: usize) -> Option<VideoFrameBufferKind> {
        if index >= self.len() {
            return None;
        }
        let raw = unsafe {
            ffi::webrtc_VideoFrameBuffer_Type_inlined_vector_get(self.raw.as_ptr(), index as i32)
        };
        let raw = NonNull::new(raw)?;
        let value = unsafe { ffi::webrtc_VideoFrameBuffer_Type_value(raw.as_ptr()) };
        Some(VideoFrameBufferKind::from_raw(value))
    }

    pub fn push(&mut self, value: VideoFrameBufferKind) {
        unsafe {
            ffi::webrtc_VideoFrameBuffer_Type_inlined_vector_push_back_value(
                self.raw.as_ptr(),
                value.to_raw(),
            )
        };
    }

    pub fn set(&mut self, index: usize, value: VideoFrameBufferKind) -> bool {
        if index >= self.len() {
            return false;
        }
        unsafe {
            ffi::webrtc_VideoFrameBuffer_Type_inlined_vector_set_value(
                self.raw.as_ptr(),
                index as i32,
                value.to_raw(),
            )
        };
        true
    }

    pub fn resize(&mut self, len: usize) {
        let len = i32::try_from(len).unwrap_or(i32::MAX);
        unsafe { ffi::webrtc_VideoFrameBuffer_Type_inlined_vector_resize(self.raw.as_ptr(), len) };
    }

    pub fn clear(&mut self) {
        unsafe { ffi::webrtc_VideoFrameBuffer_Type_inlined_vector_clear(self.raw.as_ptr()) };
    }
}

unsafe impl<'a> Send for VideoFrameBufferKindInlinedVectorRef<'a> {}

pub struct VideoEncoderQpThresholdsRef<'a> {
    raw: NonNull<ffi::webrtc_VideoEncoder_QpThresholds>,
    _marker: PhantomData<&'a ffi::webrtc_VideoEncoder_QpThresholds>,
}

impl<'a> VideoEncoderQpThresholdsRef<'a> {
    /// # Safety
    /// `raw` は有効な `webrtc_VideoEncoder_QpThresholds` を指す必要があります。
    pub unsafe fn from_raw(raw: NonNull<ffi::webrtc_VideoEncoder_QpThresholds>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn low(&self) -> i32 {
        unsafe { ffi::webrtc_VideoEncoder_QpThresholds_get_low(self.raw.as_ptr()) }
    }

    pub fn set_low(&mut self, value: i32) {
        unsafe { ffi::webrtc_VideoEncoder_QpThresholds_set_low(self.raw.as_ptr(), value) };
    }

    pub fn high(&self) -> i32 {
        unsafe { ffi::webrtc_VideoEncoder_QpThresholds_get_high(self.raw.as_ptr()) }
    }

    pub fn set_high(&mut self, value: i32) {
        unsafe { ffi::webrtc_VideoEncoder_QpThresholds_set_high(self.raw.as_ptr(), value) };
    }
}

unsafe impl<'a> Send for VideoEncoderQpThresholdsRef<'a> {}

pub struct VideoEncoderQpThresholds {
    raw: NonNull<ffi::webrtc_VideoEncoder_QpThresholds>,
}

impl Default for VideoEncoderQpThresholds {
    fn default() -> Self {
        Self::new()
    }
}

impl VideoEncoderQpThresholds {
    pub fn new() -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_VideoEncoder_QpThresholds_new() })
            .expect("BUG: webrtc_VideoEncoder_QpThresholds_new が null を返しました");
        Self { raw }
    }

    pub fn low(&self) -> i32 {
        self.as_ref().low()
    }

    pub fn set_low(&mut self, value: i32) {
        self.as_ref().set_low(value);
    }

    pub fn high(&self) -> i32 {
        self.as_ref().high()
    }

    pub fn set_high(&mut self, value: i32) {
        self.as_ref().set_high(value);
    }

    pub fn as_ref(&self) -> VideoEncoderQpThresholdsRef<'_> {
        unsafe { VideoEncoderQpThresholdsRef::from_raw(self.raw) }
    }

    pub(crate) fn as_ptr(&self) -> *mut ffi::webrtc_VideoEncoder_QpThresholds {
        self.raw.as_ptr()
    }
}

impl Drop for VideoEncoderQpThresholds {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_VideoEncoder_QpThresholds_delete(self.raw.as_ptr()) };
    }
}

unsafe impl Send for VideoEncoderQpThresholds {}

pub struct VideoEncoderScalingSettingsRef<'a> {
    raw: NonNull<ffi::webrtc_VideoEncoder_ScalingSettings>,
    _marker: PhantomData<&'a ffi::webrtc_VideoEncoder_ScalingSettings>,
}

impl<'a> VideoEncoderScalingSettingsRef<'a> {
    /// # Safety
    /// `raw` は有効な `webrtc_VideoEncoder_ScalingSettings` を指す必要があります。
    pub unsafe fn from_raw(raw: NonNull<ffi::webrtc_VideoEncoder_ScalingSettings>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn thresholds(&self) -> Option<VideoEncoderQpThresholds> {
        let mut has = 0;
        let value = VideoEncoderQpThresholds::new();
        unsafe {
            ffi::webrtc_VideoEncoder_ScalingSettings_get_thresholds(
                self.raw.as_ptr(),
                &mut has,
                value.as_ptr(),
            );
        }
        if has == 0 { None } else { Some(value) }
    }

    pub fn set_thresholds(&mut self, value: Option<&VideoEncoderQpThresholds>) {
        match value {
            Some(v) => unsafe {
                ffi::webrtc_VideoEncoder_ScalingSettings_set_thresholds(
                    self.raw.as_ptr(),
                    1,
                    v.as_ptr(),
                );
            },
            None => unsafe {
                ffi::webrtc_VideoEncoder_ScalingSettings_set_thresholds(
                    self.raw.as_ptr(),
                    0,
                    std::ptr::null(),
                );
            },
        }
    }

    pub fn min_pixels_per_frame(&self) -> i32 {
        unsafe {
            ffi::webrtc_VideoEncoder_ScalingSettings_get_min_pixels_per_frame(self.raw.as_ptr())
        }
    }

    pub fn set_min_pixels_per_frame(&mut self, value: i32) {
        unsafe {
            ffi::webrtc_VideoEncoder_ScalingSettings_set_min_pixels_per_frame(
                self.raw.as_ptr(),
                value,
            )
        };
    }
}

unsafe impl<'a> Send for VideoEncoderScalingSettingsRef<'a> {}

pub struct VideoEncoderScalingSettings {
    raw: NonNull<ffi::webrtc_VideoEncoder_ScalingSettings>,
}

impl Default for VideoEncoderScalingSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl VideoEncoderScalingSettings {
    pub fn new() -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_VideoEncoder_ScalingSettings_new() })
            .expect("BUG: webrtc_VideoEncoder_ScalingSettings_new が null を返しました");
        Self { raw }
    }

    pub fn thresholds(&self) -> Option<VideoEncoderQpThresholds> {
        self.as_ref().thresholds()
    }

    pub fn set_thresholds(&mut self, value: Option<&VideoEncoderQpThresholds>) {
        self.as_ref().set_thresholds(value);
    }

    pub fn min_pixels_per_frame(&self) -> i32 {
        self.as_ref().min_pixels_per_frame()
    }

    pub fn set_min_pixels_per_frame(&mut self, value: i32) {
        self.as_ref().set_min_pixels_per_frame(value);
    }

    pub fn as_ref(&self) -> VideoEncoderScalingSettingsRef<'_> {
        unsafe { VideoEncoderScalingSettingsRef::from_raw(self.raw) }
    }

    pub(crate) fn as_ptr(&self) -> *mut ffi::webrtc_VideoEncoder_ScalingSettings {
        self.raw.as_ptr()
    }
}

impl Drop for VideoEncoderScalingSettings {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_VideoEncoder_ScalingSettings_delete(self.raw.as_ptr()) };
    }
}

unsafe impl Send for VideoEncoderScalingSettings {}

pub struct VideoEncoderResolutionBitrateLimitsRef<'a> {
    raw: NonNull<ffi::webrtc_VideoEncoder_ResolutionBitrateLimits>,
    _marker: PhantomData<&'a ffi::webrtc_VideoEncoder_ResolutionBitrateLimits>,
}

impl<'a> VideoEncoderResolutionBitrateLimitsRef<'a> {
    /// # Safety
    /// `raw` は有効な `webrtc_VideoEncoder_ResolutionBitrateLimits` を指す必要があります。
    pub unsafe fn from_raw(raw: NonNull<ffi::webrtc_VideoEncoder_ResolutionBitrateLimits>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn frame_size_pixels(&self) -> i32 {
        unsafe {
            ffi::webrtc_VideoEncoder_ResolutionBitrateLimits_get_frame_size_pixels(
                self.raw.as_ptr(),
            )
        }
    }

    pub fn set_frame_size_pixels(&mut self, value: i32) {
        unsafe {
            ffi::webrtc_VideoEncoder_ResolutionBitrateLimits_set_frame_size_pixels(
                self.raw.as_ptr(),
                value,
            )
        };
    }

    pub fn min_start_bitrate_bps(&self) -> i32 {
        unsafe {
            ffi::webrtc_VideoEncoder_ResolutionBitrateLimits_get_min_start_bitrate_bps(
                self.raw.as_ptr(),
            )
        }
    }

    pub fn set_min_start_bitrate_bps(&mut self, value: i32) {
        unsafe {
            ffi::webrtc_VideoEncoder_ResolutionBitrateLimits_set_min_start_bitrate_bps(
                self.raw.as_ptr(),
                value,
            )
        };
    }

    pub fn min_bitrate_bps(&self) -> i32 {
        unsafe {
            ffi::webrtc_VideoEncoder_ResolutionBitrateLimits_get_min_bitrate_bps(self.raw.as_ptr())
        }
    }

    pub fn set_min_bitrate_bps(&mut self, value: i32) {
        unsafe {
            ffi::webrtc_VideoEncoder_ResolutionBitrateLimits_set_min_bitrate_bps(
                self.raw.as_ptr(),
                value,
            )
        };
    }

    pub fn max_bitrate_bps(&self) -> i32 {
        unsafe {
            ffi::webrtc_VideoEncoder_ResolutionBitrateLimits_get_max_bitrate_bps(self.raw.as_ptr())
        }
    }

    pub fn set_max_bitrate_bps(&mut self, value: i32) {
        unsafe {
            ffi::webrtc_VideoEncoder_ResolutionBitrateLimits_set_max_bitrate_bps(
                self.raw.as_ptr(),
                value,
            )
        };
    }
}

unsafe impl<'a> Send for VideoEncoderResolutionBitrateLimitsRef<'a> {}

pub struct VideoEncoderResolutionBitrateLimits {
    raw: NonNull<ffi::webrtc_VideoEncoder_ResolutionBitrateLimits>,
}

impl VideoEncoderResolutionBitrateLimits {
    pub fn new(
        frame_size_pixels: i32,
        min_start_bitrate_bps: i32,
        min_bitrate_bps: i32,
        max_bitrate_bps: i32,
    ) -> Self {
        let raw = unsafe {
            ffi::webrtc_VideoEncoder_ResolutionBitrateLimits_new(
                frame_size_pixels,
                min_start_bitrate_bps,
                min_bitrate_bps,
                max_bitrate_bps,
            )
        };
        let raw = NonNull::new(raw)
            .expect("BUG: webrtc_VideoEncoder_ResolutionBitrateLimits_new が null を返しました");
        Self { raw }
    }

    pub fn frame_size_pixels(&self) -> i32 {
        self.as_ref().frame_size_pixels()
    }

    pub fn set_frame_size_pixels(&mut self, value: i32) {
        self.as_ref().set_frame_size_pixels(value);
    }

    pub fn min_start_bitrate_bps(&self) -> i32 {
        self.as_ref().min_start_bitrate_bps()
    }

    pub fn set_min_start_bitrate_bps(&mut self, value: i32) {
        self.as_ref().set_min_start_bitrate_bps(value);
    }

    pub fn min_bitrate_bps(&self) -> i32 {
        self.as_ref().min_bitrate_bps()
    }

    pub fn set_min_bitrate_bps(&mut self, value: i32) {
        self.as_ref().set_min_bitrate_bps(value);
    }

    pub fn max_bitrate_bps(&self) -> i32 {
        self.as_ref().max_bitrate_bps()
    }

    pub fn set_max_bitrate_bps(&mut self, value: i32) {
        self.as_ref().set_max_bitrate_bps(value);
    }

    pub fn as_ref(&self) -> VideoEncoderResolutionBitrateLimitsRef<'_> {
        unsafe { VideoEncoderResolutionBitrateLimitsRef::from_raw(self.raw) }
    }

    pub(crate) fn as_ptr(&self) -> *mut ffi::webrtc_VideoEncoder_ResolutionBitrateLimits {
        self.raw.as_ptr()
    }
}

impl Drop for VideoEncoderResolutionBitrateLimits {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_VideoEncoder_ResolutionBitrateLimits_delete(self.raw.as_ptr()) };
    }
}

unsafe impl Send for VideoEncoderResolutionBitrateLimits {}

pub struct VideoEncoderResolutionBitrateLimitsVectorRef<'a> {
    raw: NonNull<ffi::webrtc_VideoEncoder_ResolutionBitrateLimits_vector>,
    _marker: PhantomData<&'a ffi::webrtc_VideoEncoder_EncoderInfo>,
}

impl<'a> VideoEncoderResolutionBitrateLimitsVectorRef<'a> {
    /// # Safety
    /// `raw` は有効な `webrtc_VideoEncoder_ResolutionBitrateLimits_vector` を指す必要があります。
    pub unsafe fn from_raw(
        raw: NonNull<ffi::webrtc_VideoEncoder_ResolutionBitrateLimits_vector>,
    ) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        let len = unsafe {
            ffi::webrtc_VideoEncoder_ResolutionBitrateLimits_vector_size(self.raw.as_ptr())
        };
        len.max(0) as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: usize) -> Option<VideoEncoderResolutionBitrateLimitsRef<'_>> {
        if index >= self.len() {
            return None;
        }
        let raw = unsafe {
            ffi::webrtc_VideoEncoder_ResolutionBitrateLimits_vector_get(
                self.raw.as_ptr(),
                index as i32,
            )
        };
        let raw = NonNull::new(raw)?;
        Some(unsafe { VideoEncoderResolutionBitrateLimitsRef::from_raw(raw) })
    }

    pub fn set(&mut self, index: usize, value: &VideoEncoderResolutionBitrateLimits) -> bool {
        if index >= self.len() {
            return false;
        }
        unsafe {
            ffi::webrtc_VideoEncoder_ResolutionBitrateLimits_vector_set(
                self.raw.as_ptr(),
                index as i32,
                value.as_ptr(),
            )
        };
        true
    }

    pub fn push(&mut self, value: &VideoEncoderResolutionBitrateLimits) {
        unsafe {
            ffi::webrtc_VideoEncoder_ResolutionBitrateLimits_vector_push_back(
                self.raw.as_ptr(),
                value.as_ptr(),
            )
        };
    }

    pub fn clear(&mut self) {
        unsafe { ffi::webrtc_VideoEncoder_ResolutionBitrateLimits_vector_clear(self.raw.as_ptr()) };
    }
}

unsafe impl<'a> Send for VideoEncoderResolutionBitrateLimitsVectorRef<'a> {}

pub struct VideoEncoderResolutionRef<'a> {
    raw: NonNull<ffi::webrtc_VideoEncoder_Resolution>,
    _marker: PhantomData<&'a ffi::webrtc_VideoEncoder_Resolution>,
}

impl<'a> VideoEncoderResolutionRef<'a> {
    /// # Safety
    /// `raw` は有効な `webrtc_VideoEncoder_Resolution` を指す必要があります。
    pub unsafe fn from_raw(raw: NonNull<ffi::webrtc_VideoEncoder_Resolution>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn width(&self) -> i32 {
        unsafe { ffi::webrtc_VideoEncoder_Resolution_get_width(self.raw.as_ptr()) }
    }

    pub fn set_width(&mut self, value: i32) {
        unsafe { ffi::webrtc_VideoEncoder_Resolution_set_width(self.raw.as_ptr(), value) };
    }

    pub fn height(&self) -> i32 {
        unsafe { ffi::webrtc_VideoEncoder_Resolution_get_height(self.raw.as_ptr()) }
    }

    pub fn set_height(&mut self, value: i32) {
        unsafe { ffi::webrtc_VideoEncoder_Resolution_set_height(self.raw.as_ptr(), value) };
    }
}

unsafe impl<'a> Send for VideoEncoderResolutionRef<'a> {}

pub struct VideoEncoderResolution {
    raw: NonNull<ffi::webrtc_VideoEncoder_Resolution>,
}

impl Default for VideoEncoderResolution {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl VideoEncoderResolution {
    pub fn new(width: i32, height: i32) -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_VideoEncoder_Resolution_new(width, height) })
            .expect("BUG: webrtc_VideoEncoder_Resolution_new が null を返しました");
        Self { raw }
    }

    pub fn width(&self) -> i32 {
        self.as_ref().width()
    }

    pub fn set_width(&mut self, value: i32) {
        self.as_ref().set_width(value);
    }

    pub fn height(&self) -> i32 {
        self.as_ref().height()
    }

    pub fn set_height(&mut self, value: i32) {
        self.as_ref().set_height(value);
    }

    pub fn as_ref(&self) -> VideoEncoderResolutionRef<'_> {
        unsafe { VideoEncoderResolutionRef::from_raw(self.raw) }
    }

    pub(crate) fn as_ptr(&self) -> *mut ffi::webrtc_VideoEncoder_Resolution {
        self.raw.as_ptr()
    }
}

impl Drop for VideoEncoderResolution {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_VideoEncoder_Resolution_delete(self.raw.as_ptr()) };
    }
}

unsafe impl Send for VideoEncoderResolution {}

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

    pub fn max_framerate_fraction() -> u8 {
        unsafe { ffi::webrtc_VideoEncoder_EncoderInfo_MaxFramerateFraction.clamp(0, 255) as u8 }
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

    pub fn scaling_settings(&self) -> VideoEncoderScalingSettingsRef<'_> {
        let raw =
            unsafe { ffi::webrtc_VideoEncoder_EncoderInfo_get_scaling_settings(self.as_ptr()) };
        let raw = NonNull::new(raw).expect(
            "BUG: webrtc_VideoEncoder_EncoderInfo_get_scaling_settings が null を返しました",
        );
        unsafe { VideoEncoderScalingSettingsRef::from_raw(raw) }
    }

    pub fn set_scaling_settings(&mut self, value: &VideoEncoderScalingSettings) {
        unsafe {
            ffi::webrtc_VideoEncoder_EncoderInfo_set_scaling_settings(self.as_ptr(), value.as_ptr())
        };
    }

    pub fn requested_resolution_alignment(&self) -> u32 {
        unsafe {
            ffi::webrtc_VideoEncoder_EncoderInfo_get_requested_resolution_alignment(self.as_ptr())
        }
    }

    pub fn set_requested_resolution_alignment(&mut self, value: u32) {
        unsafe {
            ffi::webrtc_VideoEncoder_EncoderInfo_set_requested_resolution_alignment(
                self.as_ptr(),
                value,
            );
        }
    }

    pub fn apply_alignment_to_all_simulcast_layers(&self) -> bool {
        unsafe {
            ffi::webrtc_VideoEncoder_EncoderInfo_get_apply_alignment_to_all_simulcast_layers(
                self.as_ptr(),
            ) != 0
        }
    }

    pub fn set_apply_alignment_to_all_simulcast_layers(&mut self, value: bool) {
        unsafe {
            ffi::webrtc_VideoEncoder_EncoderInfo_set_apply_alignment_to_all_simulcast_layers(
                self.as_ptr(),
                if value { 1 } else { 0 },
            )
        };
    }

    pub fn supports_native_handle(&self) -> bool {
        unsafe {
            ffi::webrtc_VideoEncoder_EncoderInfo_get_supports_native_handle(self.as_ptr()) != 0
        }
    }

    pub fn set_supports_native_handle(&mut self, value: bool) {
        unsafe {
            ffi::webrtc_VideoEncoder_EncoderInfo_set_supports_native_handle(
                self.as_ptr(),
                if value { 1 } else { 0 },
            )
        };
    }

    pub fn has_trusted_rate_controller(&self) -> bool {
        unsafe {
            ffi::webrtc_VideoEncoder_EncoderInfo_get_has_trusted_rate_controller(self.as_ptr()) != 0
        }
    }

    pub fn set_has_trusted_rate_controller(&mut self, value: bool) {
        unsafe {
            ffi::webrtc_VideoEncoder_EncoderInfo_set_has_trusted_rate_controller(
                self.as_ptr(),
                if value { 1 } else { 0 },
            )
        };
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

    pub fn fps_allocation(
        &self,
        spatial_index: usize,
    ) -> Option<VideoEncoderFramerateFractionInlinedVectorRef<'_>> {
        let spatial_index = i32::try_from(spatial_index).ok()?;
        let raw = unsafe {
            ffi::webrtc_VideoEncoder_EncoderInfo_get_fps_allocation(self.as_ptr(), spatial_index)
        };
        let raw = NonNull::new(raw)?;
        Some(unsafe { VideoEncoderFramerateFractionInlinedVectorRef::from_raw(raw) })
    }

    pub fn resolution_bitrate_limits(
        &mut self,
    ) -> VideoEncoderResolutionBitrateLimitsVectorRef<'_> {
        let raw = unsafe {
            ffi::webrtc_VideoEncoder_EncoderInfo_get_resolution_bitrate_limits(self.as_ptr())
        };
        let raw = NonNull::new(raw).expect(
            "BUG: webrtc_VideoEncoder_EncoderInfo_get_resolution_bitrate_limits が null を返しました",
        );
        unsafe { VideoEncoderResolutionBitrateLimitsVectorRef::from_raw(raw) }
    }

    pub fn get_encoder_bitrate_limits_for_resolution(
        &self,
        frame_size_pixels: i32,
    ) -> Option<VideoEncoderResolutionBitrateLimits> {
        let mut has = 0;
        let value = VideoEncoderResolutionBitrateLimits::new(0, 0, 0, 0);
        unsafe {
            ffi::webrtc_VideoEncoder_EncoderInfo_GetEncoderBitrateLimitsForResolution(
                self.as_ptr(),
                frame_size_pixels,
                &mut has,
                value.as_ptr(),
            );
        }
        if has == 0 { None } else { Some(value) }
    }

    pub fn supports_simulcast(&self) -> bool {
        unsafe { ffi::webrtc_VideoEncoder_EncoderInfo_get_supports_simulcast(self.as_ptr()) != 0 }
    }

    pub fn set_supports_simulcast(&mut self, value: bool) {
        unsafe {
            ffi::webrtc_VideoEncoder_EncoderInfo_set_supports_simulcast(
                self.as_ptr(),
                if value { 1 } else { 0 },
            )
        };
    }

    pub fn preferred_pixel_formats(&self) -> VideoFrameBufferKindInlinedVectorRef<'_> {
        let raw = unsafe {
            ffi::webrtc_VideoEncoder_EncoderInfo_get_preferred_pixel_formats(self.as_ptr())
        };
        let raw = NonNull::new(raw).expect(
            "BUG: webrtc_VideoEncoder_EncoderInfo_get_preferred_pixel_formats が null を返しました",
        );
        unsafe { VideoFrameBufferKindInlinedVectorRef::from_raw(raw) }
    }

    pub fn is_qp_trusted(&self) -> Option<bool> {
        let mut has = 0;
        let mut value = 0;
        unsafe {
            ffi::webrtc_VideoEncoder_EncoderInfo_get_is_qp_trusted(
                self.as_ptr(),
                &mut has,
                &mut value,
            );
        }
        if has == 0 { None } else { Some(value != 0) }
    }

    pub fn set_is_qp_trusted(&mut self, value: Option<bool>) {
        match value {
            Some(v) => {
                let value = if v { 1 } else { 0 };
                unsafe {
                    ffi::webrtc_VideoEncoder_EncoderInfo_set_is_qp_trusted(
                        self.as_ptr(),
                        1,
                        &value,
                    );
                }
            }
            None => unsafe {
                ffi::webrtc_VideoEncoder_EncoderInfo_set_is_qp_trusted(
                    self.as_ptr(),
                    0,
                    std::ptr::null(),
                );
            },
        }
    }

    pub fn min_qp(&self) -> Option<i32> {
        let mut has = 0;
        let mut value = 0;
        unsafe {
            ffi::webrtc_VideoEncoder_EncoderInfo_get_min_qp(self.as_ptr(), &mut has, &mut value)
        };
        if has == 0 { None } else { Some(value) }
    }

    pub fn set_min_qp(&mut self, value: Option<i32>) {
        match value {
            Some(v) => unsafe {
                ffi::webrtc_VideoEncoder_EncoderInfo_set_min_qp(self.as_ptr(), 1, &v);
            },
            None => unsafe {
                ffi::webrtc_VideoEncoder_EncoderInfo_set_min_qp(self.as_ptr(), 0, std::ptr::null());
            },
        }
    }

    pub fn mapped_resolution(&self) -> Option<VideoEncoderResolution> {
        let mut has = 0;
        let value = VideoEncoderResolution::new(0, 0);
        unsafe {
            ffi::webrtc_VideoEncoder_EncoderInfo_get_mapped_resolution(
                self.as_ptr(),
                &mut has,
                value.as_ptr(),
            )
        };
        if has == 0 { None } else { Some(value) }
    }

    pub fn set_mapped_resolution(&mut self, value: Option<&VideoEncoderResolution>) {
        match value {
            Some(v) => unsafe {
                ffi::webrtc_VideoEncoder_EncoderInfo_set_mapped_resolution(
                    self.as_ptr(),
                    1,
                    v.as_ptr(),
                );
            },
            None => unsafe {
                ffi::webrtc_VideoEncoder_EncoderInfo_set_mapped_resolution(
                    self.as_ptr(),
                    0,
                    std::ptr::null(),
                );
            },
        }
    }

    pub fn to_string(&self) -> Result<String> {
        let raw = unsafe { ffi::webrtc_VideoEncoder_EncoderInfo_ToString(self.as_ptr()) };
        let raw = NonNull::new(raw)
            .expect("BUG: webrtc_VideoEncoder_EncoderInfo_ToString が null を返しました");
        CxxString::from_unique(raw).to_string()
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

    pub(crate) fn as_ptr(&self) -> *mut ffi::webrtc_VideoEncoder_Settings {
        self.raw.as_ptr()
    }
}

unsafe impl<'a> Send for VideoEncoderSettingsRef<'a> {}

pub struct VideoEncoderSettings {
    raw: NonNull<ffi::webrtc_VideoEncoder_Settings>,
}

impl VideoEncoderSettings {
    pub fn new(number_of_cores: i32, max_payload_size: usize) -> Self {
        let raw = NonNull::new(unsafe {
            ffi::webrtc_VideoEncoder_Settings_new(number_of_cores, max_payload_size)
        })
        .expect("BUG: webrtc_VideoEncoder_Settings_new が null を返しました");
        Self { raw }
    }

    pub fn as_ref(&self) -> VideoEncoderSettingsRef<'_> {
        unsafe { VideoEncoderSettingsRef::from_raw(self.raw) }
    }
}

impl Drop for VideoEncoderSettings {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_VideoEncoder_Settings_delete(self.raw.as_ptr()) };
    }
}

unsafe impl Send for VideoEncoderSettings {}

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

    pub(crate) fn as_ptr(&self) -> *mut ffi::webrtc_VideoEncoder_RateControlParameters {
        self.raw.as_ptr()
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

    pub fn set_codec_type(&mut self, codec_type: VideoCodecType) {
        unsafe {
            ffi::webrtc_CodecSpecificInfo_set_codec_type(self.raw.as_ptr(), codec_type.to_raw())
        };
    }

    pub fn set_end_of_picture(&mut self, end_of_picture: bool) {
        unsafe {
            ffi::webrtc_CodecSpecificInfo_set_end_of_picture(
                self.raw.as_ptr(),
                if end_of_picture { 1 } else { 0 },
            )
        };
    }

    pub fn set_vp8_non_reference(&mut self, non_reference: bool) {
        unsafe {
            ffi::webrtc_CodecSpecificInfo_set_vp8_non_reference(
                self.raw.as_ptr(),
                if non_reference { 1 } else { 0 },
            )
        };
    }

    pub fn set_vp8_temporal_idx(&mut self, temporal_idx: i32) {
        unsafe {
            ffi::webrtc_CodecSpecificInfo_set_vp8_temporal_idx(self.raw.as_ptr(), temporal_idx)
        };
    }

    pub fn set_vp8_layer_sync(&mut self, layer_sync: bool) {
        unsafe {
            ffi::webrtc_CodecSpecificInfo_set_vp8_layer_sync(
                self.raw.as_ptr(),
                if layer_sync { 1 } else { 0 },
            )
        };
    }

    pub fn set_vp8_key_idx(&mut self, key_idx: i32) {
        unsafe { ffi::webrtc_CodecSpecificInfo_set_vp8_key_idx(self.raw.as_ptr(), key_idx) };
    }

    pub fn set_vp9_temporal_idx(&mut self, temporal_idx: i32) {
        unsafe {
            ffi::webrtc_CodecSpecificInfo_set_vp9_temporal_idx(self.raw.as_ptr(), temporal_idx)
        };
    }

    pub fn set_vp9_inter_pic_predicted(&mut self, inter_pic_predicted: bool) {
        unsafe {
            ffi::webrtc_CodecSpecificInfo_set_vp9_inter_pic_predicted(
                self.raw.as_ptr(),
                if inter_pic_predicted { 1 } else { 0 },
            )
        };
    }

    pub fn set_vp9_flexible_mode(&mut self, flexible_mode: bool) {
        unsafe {
            ffi::webrtc_CodecSpecificInfo_set_vp9_flexible_mode(
                self.raw.as_ptr(),
                if flexible_mode { 1 } else { 0 },
            )
        };
    }

    pub fn set_vp9_inter_layer_predicted(&mut self, inter_layer_predicted: bool) {
        unsafe {
            ffi::webrtc_CodecSpecificInfo_set_vp9_inter_layer_predicted(
                self.raw.as_ptr(),
                if inter_layer_predicted { 1 } else { 0 },
            )
        };
    }

    pub fn set_h264_packetization_mode(&mut self, packetization_mode: H264PacketizationMode) {
        unsafe {
            ffi::webrtc_CodecSpecificInfo_set_h264_packetization_mode(
                self.raw.as_ptr(),
                packetization_mode.to_raw(),
            )
        };
    }

    pub fn set_h264_temporal_idx(&mut self, temporal_idx: i32) {
        unsafe {
            ffi::webrtc_CodecSpecificInfo_set_h264_temporal_idx(self.raw.as_ptr(), temporal_idx)
        };
    }

    pub fn set_h264_base_layer_sync(&mut self, base_layer_sync: bool) {
        unsafe {
            ffi::webrtc_CodecSpecificInfo_set_h264_base_layer_sync(
                self.raw.as_ptr(),
                if base_layer_sync { 1 } else { 0 },
            )
        };
    }

    pub fn set_h264_idr_frame(&mut self, idr_frame: bool) {
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
        self.as_ref()
            .set_h264_packetization_mode(packetization_mode);
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

pub trait VideoEncoderEncodedImageCallbackHandler: Send {
    #[expect(unused_variables)]
    fn on_encoded_image(
        &mut self,
        encoded_image: EncodedImageRef<'_>,
        codec_specific_info: Option<CodecSpecificInfoRef<'_>>,
    ) -> VideoEncoderEncodedImageCallbackResult {
        VideoEncoderEncodedImageCallbackResult::new(VideoEncoderEncodedImageCallbackResultError::Ok)
    }
}

pub struct VideoEncoderEncodedImageCallback {
    raw: NonNull<ffi::webrtc_VideoEncoder_EncodedImageCallback>,
}

impl VideoEncoderEncodedImageCallback {
    pub fn new_with_handler(handler: Box<dyn VideoEncoderEncodedImageCallbackHandler>) -> Self {
        let state = Box::new(VideoEncoderEncodedImageHandlerState { handler });
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
                    Box::from_raw(user_data as *mut VideoEncoderEncodedImageHandlerState)
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

pub trait VideoEncoderHandler: Send {
    #[expect(unused_variables)]
    fn init_encode(
        &mut self,
        codec_settings: VideoCodecRef<'_>,
        settings: VideoEncoderSettingsRef<'_>,
    ) -> VideoCodecStatus {
        VideoCodecStatus::Ok
    }

    #[expect(unused_variables)]
    fn encode(
        &mut self,
        frame: VideoFrameRef<'_>,
        frame_types: Option<VideoFrameTypeVectorRef<'_>>,
    ) -> VideoCodecStatus {
        VideoCodecStatus::Ok
    }

    #[expect(unused_variables)]
    fn register_encode_complete_callback(
        &mut self,
        callback: Option<VideoEncoderEncodedImageCallbackRef<'_>>,
    ) -> VideoCodecStatus {
        VideoCodecStatus::Ok
    }

    fn release(&mut self) -> VideoCodecStatus {
        VideoCodecStatus::Ok
    }

    #[expect(unused_variables)]
    fn set_rates(&mut self, parameters: VideoEncoderRateControlParametersRef<'_>) {}

    fn get_encoder_info(&mut self) -> VideoEncoderEncoderInfo {
        VideoEncoderEncoderInfo::new()
    }
}

pub trait VideoEncoderFactoryHandler: Send {
    fn get_supported_formats(&mut self) -> Vec<SdpVideoFormat> {
        Vec::new()
    }

    #[expect(unused_variables)]
    fn create(
        &mut self,
        env: EnvironmentRef<'_>,
        format: SdpVideoFormatRef<'_>,
    ) -> Option<Box<dyn VideoEncoderHandler>> {
        None
    }
}

struct VideoEncoderHandlerState {
    handler: Box<dyn VideoEncoderHandler>,
}

struct VideoEncoderEncodedImageHandlerState {
    handler: Box<dyn VideoEncoderEncodedImageCallbackHandler>,
}

struct VideoEncoderFactoryHandlerState {
    handler: Box<dyn VideoEncoderFactoryHandler>,
}

unsafe extern "C" fn video_encoder_on_destroy(user_data: *mut c_void) {
    assert!(
        !user_data.is_null(),
        "video_encoder_on_destroy: user_data is null"
    );
    let _ = unsafe { Box::from_raw(user_data as *mut VideoEncoderHandlerState) };
}

unsafe extern "C" fn video_encoder_encoded_image_callback_on_destroy(user_data: *mut c_void) {
    assert!(
        !user_data.is_null(),
        "video_encoder_encoded_image_callback_on_destroy: user_data is null"
    );
    let _ = unsafe { Box::from_raw(user_data as *mut VideoEncoderEncodedImageHandlerState) };
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
    let state = unsafe { &mut *(user_data as *mut VideoEncoderEncodedImageHandlerState) };
    let encoded_image = NonNull::new(encoded_image)
        .expect("video_encoder_encoded_image_callback_on_encoded_image: encoded_image is null");
    let encoded_image = unsafe { EncodedImageRef::from_raw(encoded_image) };
    let codec_specific_info =
        NonNull::new(codec_specific_info).map(|v| unsafe { CodecSpecificInfoRef::from_raw(v) });
    let result = state
        .handler
        .on_encoded_image(encoded_image, codec_specific_info);
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
    let state = unsafe { &mut *(user_data as *mut VideoEncoderHandlerState) };
    let codec_settings =
        NonNull::new(codec_settings).expect("video_encoder_init_encode: codec_settings is null");
    let settings = NonNull::new(settings).expect("video_encoder_init_encode: settings is null");
    let codec_settings = unsafe { VideoCodecRef::from_raw(codec_settings) };
    let settings = unsafe { VideoEncoderSettingsRef::from_raw(settings) };
    state.handler.init_encode(codec_settings, settings).to_raw()
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
    let state = unsafe { &mut *(user_data as *mut VideoEncoderHandlerState) };
    let frame = NonNull::new(frame).expect("video_encoder_encode: frame is null");
    let frame = unsafe { VideoFrameRef::from_raw(frame) };
    let frame_types = NonNull::new(frame_types)
        .map(|frame_types| unsafe { VideoFrameTypeVectorRef::from_raw(frame_types) });
    state.handler.encode(frame, frame_types).to_raw()
}

unsafe extern "C" fn video_encoder_register_encode_complete_callback(
    callback: *mut ffi::webrtc_VideoEncoder_EncodedImageCallback,
    user_data: *mut c_void,
) -> i32 {
    assert!(
        !user_data.is_null(),
        "video_encoder_register_encode_complete_callback: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoEncoderHandlerState) };
    let callback = NonNull::new(callback)
        .map(|callback| unsafe { VideoEncoderEncodedImageCallbackRef::from_raw(callback) });
    state
        .handler
        .register_encode_complete_callback(callback)
        .to_raw()
}

unsafe extern "C" fn video_encoder_release(user_data: *mut c_void) -> i32 {
    assert!(
        !user_data.is_null(),
        "video_encoder_release: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoEncoderHandlerState) };
    state.handler.release().to_raw()
}

unsafe extern "C" fn video_encoder_set_rates(
    parameters: *mut ffi::webrtc_VideoEncoder_RateControlParameters,
    user_data: *mut c_void,
) {
    assert!(
        !user_data.is_null(),
        "video_encoder_set_rates: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoEncoderHandlerState) };
    let parameters = NonNull::new(parameters).expect("video_encoder_set_rates: parameters is null");
    let parameters = unsafe { VideoEncoderRateControlParametersRef::from_raw(parameters) };
    state.handler.set_rates(parameters);
}

unsafe extern "C" fn video_encoder_get_encoder_info(
    user_data: *mut c_void,
) -> *mut ffi::webrtc_VideoEncoder_EncoderInfo_unique {
    assert!(
        !user_data.is_null(),
        "video_encoder_get_encoder_info: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoEncoderHandlerState) };
    state.handler.get_encoder_info().into_raw()
}

unsafe extern "C" fn video_encoder_factory_on_destroy(user_data: *mut c_void) {
    assert!(
        !user_data.is_null(),
        "video_encoder_factory_on_destroy: user_data is null"
    );
    let _ = unsafe { Box::from_raw(user_data as *mut VideoEncoderFactoryHandlerState) };
}

unsafe extern "C" fn video_encoder_factory_get_supported_formats(
    user_data: *mut c_void,
) -> *mut ffi::webrtc_SdpVideoFormat_vector {
    let empty = || unsafe { ffi::webrtc_SdpVideoFormat_vector_new() };
    assert!(
        !user_data.is_null(),
        "video_encoder_factory_get_supported_formats: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut VideoEncoderFactoryHandlerState) };
    let formats = state.handler.get_supported_formats();
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
    let state = unsafe { &mut *(user_data as *mut VideoEncoderFactoryHandlerState) };
    let env = NonNull::new(env).expect("video_encoder_factory_create: env is null");
    let format = NonNull::new(format).expect("video_encoder_factory_create: format is null");
    let env = unsafe { EnvironmentRef::from_raw(env) };
    let format = unsafe { SdpVideoFormatRef::from_raw(format) };
    match state.handler.create(env, format) {
        Some(handler) => VideoEncoder::new_with_handler(handler).into_raw(),
        None => std::ptr::null_mut(),
    }
}

/// webrtc::VideoEncoder のラッパー。
pub struct VideoEncoder {
    raw_unique: NonNull<ffi::webrtc_VideoEncoder_unique>,
}

impl VideoEncoder {
    pub fn new_with_handler(handler: Box<dyn VideoEncoderHandler>) -> Self {
        let state = Box::new(VideoEncoderHandlerState { handler });
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
                let _ = unsafe { Box::from_raw(user_data as *mut VideoEncoderHandlerState) };
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

    pub fn init_encode(
        &mut self,
        codec_settings: VideoCodecRef<'_>,
        settings: VideoEncoderSettingsRef<'_>,
    ) -> VideoCodecStatus {
        let value = unsafe {
            ffi::webrtc_VideoEncoder_InitEncode(
                self.as_ptr(),
                codec_settings.as_ptr(),
                settings.as_ptr(),
            )
        };
        VideoCodecStatus::from_raw(value)
    }

    pub fn encode(
        &mut self,
        frame: VideoFrameRef<'_>,
        frame_types: Option<VideoFrameTypeVectorRef<'_>>,
    ) -> VideoCodecStatus {
        let frame_types =
            frame_types.map_or(std::ptr::null_mut(), |frame_types| frame_types.as_ptr());
        let value =
            unsafe { ffi::webrtc_VideoEncoder_Encode(self.as_ptr(), frame.as_ptr(), frame_types) };
        VideoCodecStatus::from_raw(value)
    }

    pub fn register_encode_complete_callback(
        &mut self,
        callback: Option<VideoEncoderEncodedImageCallbackRef<'_>>,
    ) -> VideoCodecStatus {
        let callback = callback.map_or(std::ptr::null_mut(), |callback| callback.as_ptr());
        let value = unsafe {
            ffi::webrtc_VideoEncoder_RegisterEncodeCompleteCallback(self.as_ptr(), callback)
        };
        VideoCodecStatus::from_raw(value)
    }

    pub fn set_rates(&mut self, parameters: VideoEncoderRateControlParametersRef<'_>) {
        unsafe { ffi::webrtc_VideoEncoder_SetRates(self.as_ptr(), parameters.as_ptr()) };
    }

    pub fn release(&mut self) -> VideoCodecStatus {
        let value = unsafe { ffi::webrtc_VideoEncoder_Release(self.as_ptr()) };
        VideoCodecStatus::from_raw(value)
    }

    pub fn get_encoder_info(&self) -> VideoEncoderEncoderInfo {
        let raw = unsafe { ffi::webrtc_VideoEncoder_GetEncoderInfo(self.as_ptr()) };
        let raw_unique =
            NonNull::new(raw).expect("webrtc_VideoEncoder_GetEncoderInfo が null を返しました");
        VideoEncoderEncoderInfo { raw_unique }
    }
}

impl VideoEncoderHandler for VideoEncoder {
    fn init_encode(
        &mut self,
        codec_settings: VideoCodecRef<'_>,
        settings: VideoEncoderSettingsRef<'_>,
    ) -> VideoCodecStatus {
        VideoEncoder::init_encode(self, codec_settings, settings)
    }

    fn encode(
        &mut self,
        frame: VideoFrameRef<'_>,
        frame_types: Option<VideoFrameTypeVectorRef<'_>>,
    ) -> VideoCodecStatus {
        VideoEncoder::encode(self, frame, frame_types)
    }

    fn register_encode_complete_callback(
        &mut self,
        callback: Option<VideoEncoderEncodedImageCallbackRef<'_>>,
    ) -> VideoCodecStatus {
        VideoEncoder::register_encode_complete_callback(self, callback)
    }

    fn release(&mut self) -> VideoCodecStatus {
        VideoEncoder::release(self)
    }

    fn set_rates(&mut self, parameters: VideoEncoderRateControlParametersRef<'_>) {
        VideoEncoder::set_rates(self, parameters);
    }

    fn get_encoder_info(&mut self) -> VideoEncoderEncoderInfo {
        VideoEncoder::get_encoder_info(self)
    }
}

impl Drop for VideoEncoder {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_VideoEncoder_unique_delete(self.raw_unique.as_ptr()) };
    }
}

unsafe impl Send for VideoEncoder {}

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

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    pub fn from_objc_default() -> Option<Self> {
        let objc_factory = unsafe { ffi::webrtc_objc_RTCDefaultVideoEncoderFactory_new() };
        if objc_factory.is_null() {
            return None;
        }

        let raw_unique = unsafe { ffi::webrtc_ObjCToNativeVideoEncoderFactory(objc_factory) };
        unsafe { ffi::webrtc_objc_RTCVideoEncoderFactory_release(objc_factory) };
        let raw_unique = NonNull::new(raw_unique)?;
        Some(Self { raw_unique })
    }

    #[cfg(target_os = "android")]
    pub fn from_android_default() -> Option<Self> {
        let env = unsafe { ffi::webrtc_jni_AttachCurrentThreadIfNeeded() };
        if env.is_null() {
            return None;
        }

        let class = unsafe {
            ffi::webrtc_GetClass(
                env,
                b"org/webrtc/DefaultVideoEncoderFactory\0".as_ptr().cast(),
            )
        };
        if class.is_null() {
            if unsafe { ffi::jni_JNIEnv_ExceptionCheck(env) != 0 } {
                unsafe { ffi::jni_JNIEnv_ExceptionClear(env) };
            }
            return None;
        }

        let ctor = unsafe {
            ffi::jni_JNIEnv_GetMethodID(
                env,
                class,
                b"<init>\0".as_ptr().cast(),
                b"(Lorg/webrtc/EglBase$Context;ZZ)V\0".as_ptr().cast(),
            )
        };
        if ctor.is_null() {
            unsafe { ffi::jni_JNIEnv_DeleteLocalRef(env, class) };
            if unsafe { ffi::jni_JNIEnv_ExceptionCheck(env) != 0 } {
                unsafe { ffi::jni_JNIEnv_ExceptionClear(env) };
            }
            return None;
        }

        let mut args: [ffi::jvalue; 3] = unsafe { std::mem::zeroed() };
        args[0].l = std::ptr::null_mut();
        args[1].z = 1;
        args[2].z = 0;
        let encoder_factory =
            unsafe { ffi::jni_JNIEnv_NewObjectA(env, class, ctor, args.as_ptr()) };
        if encoder_factory.is_null() {
            unsafe { ffi::jni_JNIEnv_DeleteLocalRef(env, class) };
            if unsafe { ffi::jni_JNIEnv_ExceptionCheck(env) != 0 } {
                unsafe { ffi::jni_JNIEnv_ExceptionClear(env) };
            }
            return None;
        }

        let raw_unique =
            unsafe { ffi::webrtc_JavaToNativeVideoEncoderFactory(env, encoder_factory) };
        unsafe {
            ffi::jni_JNIEnv_DeleteLocalRef(env, encoder_factory);
            ffi::jni_JNIEnv_DeleteLocalRef(env, class);
        }
        if unsafe { ffi::jni_JNIEnv_ExceptionCheck(env) != 0 } {
            unsafe { ffi::jni_JNIEnv_ExceptionClear(env) };
            return None;
        }

        let raw_unique = NonNull::new(raw_unique)?;
        Some(Self { raw_unique })
    }

    pub fn new_with_handler(handler: Box<dyn VideoEncoderFactoryHandler>) -> Self {
        let state = Box::new(VideoEncoderFactoryHandlerState { handler });
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
                let _ = unsafe { Box::from_raw(user_data as *mut VideoEncoderFactoryHandlerState) };
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
        env: EnvironmentRef<'_>,
        format: SdpVideoFormatRef<'_>,
    ) -> Option<VideoEncoder> {
        let raw = unsafe {
            ffi::webrtc_VideoEncoderFactory_Create(self.as_ptr(), env.as_ptr(), format.as_ptr())
        };
        Some(VideoEncoder {
            raw_unique: NonNull::new(raw)?,
        })
    }

    pub fn get_supported_formats(&self) -> Vec<SdpVideoFormat> {
        let raw_vec = unsafe { ffi::webrtc_VideoEncoderFactory_GetSupportedFormats(self.as_ptr()) };
        let raw_vec = NonNull::new(raw_vec)
            .expect("BUG: webrtc_VideoEncoderFactory_GetSupportedFormats が null を返しました");
        let size = unsafe { ffi::webrtc_SdpVideoFormat_vector_size(raw_vec.as_ptr()) };
        let mut formats = Vec::with_capacity(size.max(0) as usize);
        for i in 0..size {
            let raw_format = unsafe { ffi::webrtc_SdpVideoFormat_vector_get(raw_vec.as_ptr(), i) };
            let raw_format = NonNull::new(raw_format)
                .expect("BUG: webrtc_SdpVideoFormat_vector_get が null を返しました");
            let format_ref = unsafe { SdpVideoFormatRef::from_raw(raw_format) };
            formats.push(format_ref.to_owned());
        }
        unsafe { ffi::webrtc_SdpVideoFormat_vector_delete(raw_vec.as_ptr()) };
        formats
    }
}

impl Drop for VideoEncoderFactory {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_VideoEncoderFactory_unique_delete(self.raw_unique.as_ptr()) };
    }
}

unsafe impl Send for VideoEncoderFactory {}
