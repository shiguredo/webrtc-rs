use crate::ref_count::{
    MediaStreamTrackHandle, RtpReceiverHandle, RtpSenderHandle, RtpTransceiverHandle,
    VideoTrackHandle,
};
use crate::{
    CxxString, CxxStringRef, Error, MapStringString, MediaType, Result, RtcError, ScopedRef,
    StringVectorRef, VideoTrack, ffi,
};
use std::marker::PhantomData;
use std::ptr::NonNull;

/// webrtc::RtpCapabilities のラッパー。
pub struct RtpCapabilities {
    raw: NonNull<ffi::webrtc_RtpCapabilities>,
}

impl RtpCapabilities {
    pub fn from_raw(raw: NonNull<ffi::webrtc_RtpCapabilities>) -> Self {
        Self { raw }
    }

    /// codecs の個数を返す。
    pub fn codec_len(&self) -> i32 {
        let codecs = unsafe { ffi::webrtc_RtpCapabilities_get_codecs(self.raw.as_ptr()) };
        if codecs.is_null() {
            return 0;
        }
        unsafe { ffi::webrtc_RtpCodecCapability_vector_size(codecs) }
    }

    /// codecs のベクタを借用する。
    pub fn codecs(&self) -> RtpCodecCapabilityVectorRef<'_> {
        let raw =
            NonNull::new(unsafe { ffi::webrtc_RtpCapabilities_get_codecs(self.raw.as_ptr()) })
                .expect("BUG: webrtc_RtpCapabilities_get_codecs が null を返しました");
        RtpCodecCapabilityVectorRef::from_raw(raw)
    }
}

impl Drop for RtpCapabilities {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_RtpCapabilities_delete(self.raw.as_ptr()) };
    }
}

/// webrtc::RtpCodec のラッパー。
pub struct RtpCodec {
    raw: NonNull<ffi::webrtc_RtpCodec>,
}

impl Default for RtpCodec {
    fn default() -> Self {
        Self::new()
    }
}

impl RtpCodec {
    pub fn new() -> Self {
        let raw = unsafe { ffi::webrtc_RtpCodec_new() };
        Self {
            raw: NonNull::new(raw).expect("BUG: webrtc_RtpCodec_new が null を返しました"),
        }
    }

    pub fn as_ref(&self) -> RtpCodecRef<'_> {
        RtpCodecRef::from_raw(self.raw)
    }

    pub fn set_kind(&mut self, media_type: MediaType) {
        self.as_ref().set_kind(media_type);
    }

    pub fn set_name(&mut self, name: &str) {
        self.as_ref().set_name(name);
    }

    pub fn name(&self) -> Result<String> {
        self.as_ref().name()
    }

    pub fn clock_rate(&self) -> Option<i32> {
        self.as_ref().clock_rate()
    }

    pub fn set_clock_rate(&mut self, value: Option<i32>) {
        self.as_ref().set_clock_rate(value);
    }

    pub fn num_channels(&self) -> Option<i32> {
        self.as_ref().num_channels()
    }

    pub fn set_num_channels(&mut self, value: Option<i32>) {
        self.as_ref().set_num_channels(value);
    }

    pub fn parameters(&mut self) -> MapStringString<'_> {
        self.as_ref().parameters()
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_RtpCodec {
        self.raw.as_ptr()
    }
}

impl Drop for RtpCodec {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_RtpCodec_delete(self.raw.as_ptr()) };
    }
}

unsafe impl Send for RtpCodec {}

/// webrtc::RtpCodec の借用ラッパー。
pub struct RtpCodecRef<'a> {
    raw: NonNull<ffi::webrtc_RtpCodec>,
    _marker: PhantomData<&'a ffi::webrtc_RtpCodec>,
}

impl<'a> RtpCodecRef<'a> {
    pub fn from_raw(raw: NonNull<ffi::webrtc_RtpCodec>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn name(&self) -> Result<String> {
        let ptr = unsafe { ffi::webrtc_RtpCodec_get_name(self.raw.as_ptr()) };
        CxxStringRef::from_ptr(
            NonNull::new(ptr).expect("BUG: webrtc_RtpCodec_get_name が null を返しました"),
        )
        .to_string()
    }

    pub fn set_kind(&mut self, media_type: MediaType) {
        unsafe { ffi::webrtc_RtpCodec_set_kind(self.raw.as_ptr(), media_type.to_int()) };
    }

    pub fn set_name(&mut self, name: &str) {
        unsafe {
            ffi::webrtc_RtpCodec_set_name(self.raw.as_ptr(), name.as_ptr() as *const _, name.len());
        }
    }

    pub fn clock_rate(&self) -> Option<i32> {
        let mut has = 0;
        let mut value = 0;
        unsafe {
            ffi::webrtc_RtpCodec_get_clock_rate(self.raw.as_ptr(), &mut has, &mut value);
        }
        if has == 0 { None } else { Some(value) }
    }

    pub fn num_channels(&self) -> Option<i32> {
        let mut has = 0;
        let mut value = 0;
        unsafe {
            ffi::webrtc_RtpCodec_get_num_channels(self.raw.as_ptr(), &mut has, &mut value);
        }
        if has == 0 { None } else { Some(value) }
    }

    pub fn set_clock_rate(&mut self, value: Option<i32>) {
        match value {
            Some(v) => unsafe {
                ffi::webrtc_RtpCodec_set_clock_rate(self.raw.as_ptr(), 1, &v);
            },
            None => unsafe {
                ffi::webrtc_RtpCodec_set_clock_rate(self.raw.as_ptr(), 0, std::ptr::null());
            },
        }
    }

    pub fn set_num_channels(&mut self, value: Option<i32>) {
        match value {
            Some(v) => unsafe {
                ffi::webrtc_RtpCodec_set_num_channels(self.raw.as_ptr(), 1, &v);
            },
            None => unsafe {
                ffi::webrtc_RtpCodec_set_num_channels(self.raw.as_ptr(), 0, std::ptr::null());
            },
        }
    }

    pub fn parameters(&mut self) -> MapStringString<'a> {
        let raw = unsafe { ffi::webrtc_RtpCodec_get_parameters(self.raw.as_ptr()) };
        MapStringString::from_raw(
            NonNull::new(raw).expect("BUG: webrtc_RtpCodec_get_parameters が null を返しました"),
        )
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_RtpCodec {
        self.raw.as_ptr()
    }
}

/// webrtc::RtpCodecCapability のラッパー。
pub struct RtpCodecCapability {
    raw: NonNull<ffi::webrtc_RtpCodecCapability>,
}

impl Default for RtpCodecCapability {
    fn default() -> Self {
        Self::new()
    }
}

impl RtpCodecCapability {
    pub fn new() -> Self {
        let raw = unsafe { ffi::webrtc_RtpCodecCapability_new() };
        Self {
            raw: NonNull::new(raw)
                .expect("BUG: webrtc_RtpCodecCapability_new が null を返しました"),
        }
    }

    pub fn as_ref(&self) -> RtpCodecCapabilityRef<'_> {
        RtpCodecCapabilityRef::from_raw(self.raw)
    }

    pub fn cast_to_codec(&self) -> RtpCodecRef<'_> {
        self.as_ref().cast_to_codec()
    }

    pub fn set_kind(&mut self, media_type: MediaType) {
        self.as_ref().set_kind(media_type);
    }

    pub fn set_name(&mut self, name: &str) {
        self.as_ref().set_name(name);
    }

    pub fn name(&self) -> Result<String> {
        self.as_ref().name()
    }

    pub fn clock_rate(&self) -> Option<i32> {
        self.as_ref().clock_rate()
    }

    pub fn set_clock_rate(&mut self, value: Option<i32>) {
        self.as_ref().set_clock_rate(value);
    }

    pub fn num_channels(&self) -> Option<i32> {
        self.as_ref().num_channels()
    }

    pub fn set_num_channels(&mut self, value: Option<i32>) {
        self.as_ref().set_num_channels(value);
    }

    pub fn parameters(&mut self) -> MapStringString<'_> {
        self.as_ref().parameters()
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_RtpCodecCapability {
        self.raw.as_ptr()
    }
}

impl Drop for RtpCodecCapability {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_RtpCodecCapability_delete(self.raw.as_ptr()) };
    }
}

/// RtpCodecCapability の借用ラッパー。
pub struct RtpCodecCapabilityRef<'a> {
    raw: NonNull<ffi::webrtc_RtpCodecCapability>,
    _marker: PhantomData<&'a ffi::webrtc_RtpCodecCapability_vector>,
}

impl<'a> RtpCodecCapabilityRef<'a> {
    pub fn from_raw(raw: NonNull<ffi::webrtc_RtpCodecCapability>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn cast_to_codec(&self) -> RtpCodecRef<'a> {
        let raw = NonNull::new(unsafe {
            ffi::webrtc_RtpCodecCapability_cast_to_webrtc_RtpCodec(self.raw.as_ptr())
        })
        .expect("BUG: webrtc_RtpCodecCapability_cast_to_webrtc_RtpCodec が null を返しました");
        RtpCodecRef::from_raw(raw)
    }

    pub fn set_kind(&mut self, media_type: MediaType) {
        self.cast_to_codec().set_kind(media_type);
    }

    pub fn set_name(&mut self, name: &str) {
        self.cast_to_codec().set_name(name);
    }

    pub fn name(&self) -> Result<String> {
        self.cast_to_codec().name()
    }

    pub fn clock_rate(&self) -> Option<i32> {
        self.cast_to_codec().clock_rate()
    }

    pub fn set_clock_rate(&mut self, value: Option<i32>) {
        self.cast_to_codec().set_clock_rate(value);
    }

    pub fn num_channels(&self) -> Option<i32> {
        self.cast_to_codec().num_channels()
    }

    pub fn set_num_channels(&mut self, value: Option<i32>) {
        self.cast_to_codec().set_num_channels(value);
    }

    pub fn parameters(&mut self) -> MapStringString<'a> {
        self.cast_to_codec().parameters()
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_RtpCodecCapability {
        self.raw.as_ptr()
    }
}

/// std::vector<RtpCodecCapability> の所有ラッパー。
pub struct RtpCodecCapabilityVector {
    raw: NonNull<ffi::webrtc_RtpCodecCapability_vector>,
}

impl RtpCodecCapabilityVector {
    pub fn new(size: i32) -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_RtpCodecCapability_vector_new(size) })
            .expect("BUG: webrtc_RtpCodecCapability_vector_new が null を返しました");
        Self { raw }
    }

    pub fn len(&self) -> usize {
        self.as_ref().len()
    }

    pub fn is_empty(&self) -> bool {
        self.as_ref().is_empty()
    }

    pub fn get(&self, index: usize) -> Option<RtpCodecCapabilityRef<'_>> {
        self.as_ref().get(index)
    }

    pub fn push(&mut self, cap: &RtpCodecCapabilityRef<'_>) {
        self.as_ref().push(cap);
    }

    pub fn resize(&mut self, len: usize) {
        self.as_ref().resize(len);
    }

    pub fn set(&mut self, index: usize, cap: &RtpCodecCapabilityRef<'_>) -> bool {
        self.as_ref().set(index, cap)
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_RtpCodecCapability_vector {
        self.raw.as_ptr()
    }

    pub fn as_ref(&self) -> RtpCodecCapabilityVectorRef<'_> {
        RtpCodecCapabilityVectorRef::from_raw(self.raw)
    }
}

impl Drop for RtpCodecCapabilityVector {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_RtpCodecCapability_vector_delete(self.raw.as_ptr()) };
    }
}

/// std::vector<RtpCodecCapability> の借用ラッパー。
pub struct RtpCodecCapabilityVectorRef<'a> {
    raw: NonNull<ffi::webrtc_RtpCodecCapability_vector>,
    _marker: PhantomData<&'a ffi::webrtc_RtpCapabilities>,
}

impl<'a> RtpCodecCapabilityVectorRef<'a> {
    pub fn from_raw(raw: NonNull<ffi::webrtc_RtpCodecCapability_vector>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        let len = unsafe { ffi::webrtc_RtpCodecCapability_vector_size(self.raw.as_ptr()) };
        len.max(0) as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: usize) -> Option<RtpCodecCapabilityRef<'a>> {
        if index >= self.len() {
            return None;
        }
        let raw = NonNull::new(unsafe {
            ffi::webrtc_RtpCodecCapability_vector_get(self.raw.as_ptr(), index as i32)
        })
        .expect("BUG: webrtc_RtpCodecCapability_vector_get が null を返しました");
        Some(RtpCodecCapabilityRef::from_raw(raw))
    }

    pub fn push(&mut self, cap: &RtpCodecCapabilityRef<'_>) {
        unsafe {
            ffi::webrtc_RtpCodecCapability_vector_push_back(self.raw.as_ptr(), cap.as_ptr());
        }
    }

    pub fn resize(&mut self, len: usize) {
        let len = i32::try_from(len).unwrap_or(i32::MAX);
        unsafe { ffi::webrtc_RtpCodecCapability_vector_resize(self.raw.as_ptr(), len) };
    }

    pub fn set(&mut self, index: usize, cap: &RtpCodecCapabilityRef<'_>) -> bool {
        if index >= self.len() {
            return false;
        }
        unsafe {
            ffi::webrtc_RtpCodecCapability_vector_set(
                self.raw.as_ptr(),
                index as i32,
                cap.as_ptr(),
            );
        }
        true
    }
}
pub struct Resolution {
    raw: NonNull<ffi::webrtc_Resolution>,
}

impl Default for Resolution {
    fn default() -> Self {
        Self::new()
    }
}

impl Resolution {
    pub fn new() -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_Resolution_new() })
            .expect("BUG: webrtc_Resolution_new が null を返しました");
        Self { raw }
    }

    pub fn width(&self) -> i32 {
        unsafe { ffi::webrtc_Resolution_get_width(self.raw.as_ptr()) }
    }

    pub fn set_width(&mut self, width: i32) {
        unsafe { ffi::webrtc_Resolution_set_width(self.raw.as_ptr(), width) };
    }

    pub fn height(&self) -> i32 {
        unsafe { ffi::webrtc_Resolution_get_height(self.raw.as_ptr()) }
    }

    pub fn set_height(&mut self, height: i32) {
        unsafe { ffi::webrtc_Resolution_set_height(self.raw.as_ptr(), height) };
    }

    fn as_ptr(&self) -> *mut ffi::webrtc_Resolution {
        self.raw.as_ptr()
    }
}

impl Drop for Resolution {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_Resolution_delete(self.raw.as_ptr()) };
    }
}

unsafe impl Send for Resolution {}

/// RtpEncodingParameters のラッパー。
pub struct RtpEncodingParameters {
    raw: NonNull<ffi::webrtc_RtpEncodingParameters>,
}

impl Default for RtpEncodingParameters {
    fn default() -> Self {
        Self::new()
    }
}

impl RtpEncodingParameters {
    pub fn new() -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_RtpEncodingParameters_new() })
            .expect("BUG: webrtc_RtpEncodingParameters_new が null を返しました");
        Self { raw }
    }

    pub fn as_ref(&self) -> RtpEncodingParametersRef<'_> {
        RtpEncodingParametersRef::from_raw(self.raw)
    }

    pub fn set_rid(&mut self, rid: &str) {
        self.as_ref().set_rid(rid);
    }

    pub fn rid(&self) -> Result<String> {
        self.as_ref().rid()
    }

    pub fn ssrc(&self) -> Option<u32> {
        self.as_ref().ssrc()
    }

    pub fn set_ssrc(&mut self, value: Option<u32>) {
        self.as_ref().set_ssrc(value);
    }

    pub fn max_bitrate_bps(&self) -> Option<i32> {
        self.as_ref().max_bitrate_bps()
    }

    pub fn set_max_bitrate_bps(&mut self, value: Option<i32>) {
        self.as_ref().set_max_bitrate_bps(value);
    }

    pub fn min_bitrate_bps(&self) -> Option<i32> {
        self.as_ref().min_bitrate_bps()
    }

    pub fn set_min_bitrate_bps(&mut self, value: Option<i32>) {
        self.as_ref().set_min_bitrate_bps(value);
    }

    pub fn max_framerate(&self) -> Option<f64> {
        self.as_ref().max_framerate()
    }

    pub fn set_max_framerate(&mut self, value: Option<f64>) {
        self.as_ref().set_max_framerate(value);
    }

    pub fn scale_resolution_down_by(&self) -> Option<f64> {
        self.as_ref().scale_resolution_down_by()
    }

    pub fn set_scale_resolution_down_by(&mut self, value: Option<f64>) {
        self.as_ref().set_scale_resolution_down_by(value);
    }

    pub fn scale_resolution_down_to(&self) -> Option<Resolution> {
        self.as_ref().scale_resolution_down_to()
    }

    pub fn set_scale_resolution_down_to(&mut self, value: Option<&Resolution>) {
        self.as_ref().set_scale_resolution_down_to(value);
    }

    pub fn active(&self) -> bool {
        self.as_ref().active()
    }

    pub fn set_active(&mut self, active: bool) {
        self.as_ref().set_active(active);
    }

    pub fn adaptive_ptime(&self) -> bool {
        self.as_ref().adaptive_ptime()
    }

    pub fn set_adaptive_ptime(&mut self, adaptive_ptime: bool) {
        self.as_ref().set_adaptive_ptime(adaptive_ptime);
    }

    pub fn scalability_mode(&self) -> Option<Result<String>> {
        self.as_ref().scalability_mode()
    }

    pub fn set_scalability_mode(&mut self, value: Option<&str>) {
        self.as_ref().set_scalability_mode(value);
    }

    pub fn codec(&self) -> Option<RtpCodecRef<'_>> {
        self.as_ref().codec()
    }

    pub fn set_codec(&mut self, codec: Option<&RtpCodec>) {
        self.as_ref().set_codec(codec);
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_RtpEncodingParameters {
        self.raw.as_ptr()
    }
}

impl Drop for RtpEncodingParameters {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_RtpEncodingParameters_delete(self.raw.as_ptr()) };
    }
}

/// RtpEncodingParameters の借用ラッパー。
pub struct RtpEncodingParametersRef<'a> {
    raw: NonNull<ffi::webrtc_RtpEncodingParameters>,
    _marker: PhantomData<&'a ffi::webrtc_RtpEncodingParameters_vector>,
}

impl<'a> RtpEncodingParametersRef<'a> {
    pub fn from_raw(raw: NonNull<ffi::webrtc_RtpEncodingParameters>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn rid(&self) -> Result<String> {
        let ptr = unsafe { ffi::webrtc_RtpEncodingParameters_get_rid(self.raw.as_ptr()) };
        CxxStringRef::from_ptr(
            NonNull::new(ptr)
                .expect("BUG: webrtc_RtpEncodingParameters_get_rid が null を返しました"),
        )
        .to_string()
    }

    pub fn set_rid(&mut self, rid: &str) {
        unsafe {
            ffi::webrtc_RtpEncodingParameters_set_rid(
                self.raw.as_ptr(),
                rid.as_ptr() as *const _,
                rid.len(),
            );
        }
    }

    pub fn ssrc(&self) -> Option<u32> {
        let mut has = 0;
        let mut value: u32 = 0;
        unsafe {
            ffi::webrtc_RtpEncodingParameters_get_ssrc(self.raw.as_ptr(), &mut has, &mut value)
        };
        if has == 0 { None } else { Some(value) }
    }

    pub fn set_ssrc(&mut self, value: Option<u32>) {
        match value {
            Some(v) => unsafe {
                ffi::webrtc_RtpEncodingParameters_set_ssrc(self.raw.as_ptr(), 1, &v);
            },
            None => unsafe {
                ffi::webrtc_RtpEncodingParameters_set_ssrc(self.raw.as_ptr(), 0, std::ptr::null());
            },
        }
    }

    pub fn max_bitrate_bps(&self) -> Option<i32> {
        let mut has = 0;
        let mut value = 0;
        unsafe {
            ffi::webrtc_RtpEncodingParameters_get_max_bitrate_bps(
                self.raw.as_ptr(),
                &mut has,
                &mut value,
            );
        }
        if has == 0 { None } else { Some(value) }
    }

    pub fn set_max_bitrate_bps(&mut self, value: Option<i32>) {
        match value {
            Some(v) => unsafe {
                ffi::webrtc_RtpEncodingParameters_set_max_bitrate_bps(self.raw.as_ptr(), 1, &v);
            },
            None => unsafe {
                ffi::webrtc_RtpEncodingParameters_set_max_bitrate_bps(
                    self.raw.as_ptr(),
                    0,
                    std::ptr::null(),
                );
            },
        }
    }

    pub fn min_bitrate_bps(&self) -> Option<i32> {
        let mut has = 0;
        let mut value = 0;
        unsafe {
            ffi::webrtc_RtpEncodingParameters_get_min_bitrate_bps(
                self.raw.as_ptr(),
                &mut has,
                &mut value,
            );
        }
        if has == 0 { None } else { Some(value) }
    }

    pub fn set_min_bitrate_bps(&mut self, value: Option<i32>) {
        match value {
            Some(v) => unsafe {
                ffi::webrtc_RtpEncodingParameters_set_min_bitrate_bps(self.raw.as_ptr(), 1, &v);
            },
            None => unsafe {
                ffi::webrtc_RtpEncodingParameters_set_min_bitrate_bps(
                    self.raw.as_ptr(),
                    0,
                    std::ptr::null(),
                );
            },
        }
    }

    pub fn max_framerate(&self) -> Option<f64> {
        let mut has = 0;
        let mut value = 0.0;
        unsafe {
            ffi::webrtc_RtpEncodingParameters_get_max_framerate(
                self.raw.as_ptr(),
                &mut has,
                &mut value,
            );
        }
        if has == 0 { None } else { Some(value) }
    }

    pub fn set_max_framerate(&mut self, value: Option<f64>) {
        match value {
            Some(v) => unsafe {
                ffi::webrtc_RtpEncodingParameters_set_max_framerate(self.raw.as_ptr(), 1, &v);
            },
            None => unsafe {
                ffi::webrtc_RtpEncodingParameters_set_max_framerate(
                    self.raw.as_ptr(),
                    0,
                    std::ptr::null(),
                );
            },
        }
    }

    pub fn scale_resolution_down_by(&self) -> Option<f64> {
        let mut has = 0;
        let mut value = 0.0;
        unsafe {
            ffi::webrtc_RtpEncodingParameters_get_scale_resolution_down_by(
                self.raw.as_ptr(),
                &mut has,
                &mut value,
            );
        }
        if has == 0 { None } else { Some(value) }
    }

    pub fn set_scale_resolution_down_by(&mut self, value: Option<f64>) {
        match value {
            Some(v) => unsafe {
                ffi::webrtc_RtpEncodingParameters_set_scale_resolution_down_by(
                    self.raw.as_ptr(),
                    1,
                    &v,
                );
            },
            None => unsafe {
                ffi::webrtc_RtpEncodingParameters_set_scale_resolution_down_by(
                    self.raw.as_ptr(),
                    0,
                    std::ptr::null(),
                );
            },
        }
    }

    pub fn scale_resolution_down_to(&self) -> Option<Resolution> {
        let mut has = 0;
        let resolution = Resolution::new();
        unsafe {
            ffi::webrtc_RtpEncodingParameters_get_scale_resolution_down_to(
                self.raw.as_ptr(),
                &mut has,
                resolution.as_ptr(),
            );
        }
        if has == 0 { None } else { Some(resolution) }
    }

    pub fn set_scale_resolution_down_to(&mut self, value: Option<&Resolution>) {
        match value {
            Some(v) => unsafe {
                ffi::webrtc_RtpEncodingParameters_set_scale_resolution_down_to(
                    self.raw.as_ptr(),
                    1,
                    v.as_ptr(),
                );
            },
            None => unsafe {
                ffi::webrtc_RtpEncodingParameters_set_scale_resolution_down_to(
                    self.raw.as_ptr(),
                    0,
                    std::ptr::null(),
                );
            },
        }
    }

    pub fn active(&self) -> bool {
        unsafe { ffi::webrtc_RtpEncodingParameters_get_active(self.raw.as_ptr()) != 0 }
    }

    pub fn set_active(&mut self, active: bool) {
        unsafe {
            ffi::webrtc_RtpEncodingParameters_set_active(
                self.raw.as_ptr(),
                if active { 1 } else { 0 },
            )
        };
    }

    pub fn adaptive_ptime(&self) -> bool {
        unsafe { ffi::webrtc_RtpEncodingParameters_get_adaptive_ptime(self.raw.as_ptr()) != 0 }
    }

    pub fn set_adaptive_ptime(&mut self, adaptive_ptime: bool) {
        unsafe {
            ffi::webrtc_RtpEncodingParameters_set_adaptive_ptime(
                self.raw.as_ptr(),
                if adaptive_ptime { 1 } else { 0 },
            )
        };
    }

    pub fn scalability_mode(&self) -> Option<Result<String>> {
        let mut has = 0;
        let mut ptr = std::ptr::null_mut();
        unsafe {
            ffi::webrtc_RtpEncodingParameters_get_scalability_mode(
                self.raw.as_ptr(),
                &mut has,
                &mut ptr,
            );
        }
        if has == 0 {
            return None;
        }
        Some(
            CxxStringRef::from_ptr(NonNull::new(ptr).expect(
                "BUG: webrtc_RtpEncodingParameters_get_scalability_mode が null を返しました",
            ))
            .to_string(),
        )
    }

    pub fn set_scalability_mode(&mut self, value: Option<&str>) {
        match value {
            Some(v) => unsafe {
                ffi::webrtc_RtpEncodingParameters_set_scalability_mode(
                    self.raw.as_ptr(),
                    1,
                    v.as_ptr() as *const _,
                    v.len(),
                );
            },
            None => unsafe {
                ffi::webrtc_RtpEncodingParameters_set_scalability_mode(
                    self.raw.as_ptr(),
                    0,
                    std::ptr::null(),
                    0,
                );
            },
        }
    }

    pub fn codec(&self) -> Option<RtpCodecRef<'a>> {
        let mut has = 0;
        let mut ptr = std::ptr::null_mut();
        unsafe {
            ffi::webrtc_RtpEncodingParameters_get_codec(self.raw.as_ptr(), &mut has, &mut ptr);
        }
        if has == 0 {
            None
        } else {
            Some(RtpCodecRef::from_raw(NonNull::new(ptr).expect(
                "BUG: webrtc_RtpEncodingParameters_get_codec が null を返しました",
            )))
        }
    }

    pub fn set_codec(&mut self, codec: Option<&RtpCodec>) {
        match codec {
            Some(v) => unsafe {
                ffi::webrtc_RtpEncodingParameters_set_codec(self.raw.as_ptr(), 1, v.as_ptr());
            },
            None => unsafe {
                ffi::webrtc_RtpEncodingParameters_set_codec(self.raw.as_ptr(), 0, std::ptr::null());
            },
        }
    }
}

/// std::vector<RtpEncodingParameters> の所有ラッパー。
pub struct RtpEncodingParametersVector {
    raw: NonNull<ffi::webrtc_RtpEncodingParameters_vector>,
}

impl RtpEncodingParametersVector {
    pub fn new(size: i32) -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_RtpEncodingParameters_vector_new(size) })
            .expect("BUG: webrtc_RtpEncodingParameters_vector_new が null を返しました");
        Self { raw }
    }

    pub fn len(&self) -> usize {
        let len = unsafe { ffi::webrtc_RtpEncodingParameters_vector_size(self.raw.as_ptr()) };
        len.max(0) as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: usize) -> Option<RtpEncodingParametersRef<'_>> {
        if index >= self.len() {
            return None;
        }

        let raw = NonNull::new(unsafe {
            ffi::webrtc_RtpEncodingParameters_vector_get(self.raw.as_ptr(), index as i32)
        })
        .expect("BUG: webrtc_RtpEncodingParameters_vector_get が null を返しました");
        Some(RtpEncodingParametersRef::from_raw(raw))
    }

    pub fn push(&mut self, enc: &RtpEncodingParameters) {
        unsafe {
            ffi::webrtc_RtpEncodingParameters_vector_push_back(self.raw.as_ptr(), enc.as_ptr());
        }
    }

    pub fn resize(&mut self, len: usize) {
        let len = i32::try_from(len).unwrap_or(i32::MAX);
        unsafe { ffi::webrtc_RtpEncodingParameters_vector_resize(self.raw.as_ptr(), len) };
    }

    pub fn set(&mut self, index: usize, enc: &RtpEncodingParameters) -> bool {
        if index >= self.len() {
            return false;
        }
        unsafe {
            ffi::webrtc_RtpEncodingParameters_vector_set(
                self.raw.as_ptr(),
                index as i32,
                enc.as_ptr(),
            );
        }
        true
    }

    pub fn clone_from_raw(src: NonNull<ffi::webrtc_RtpEncodingParameters_vector>) -> Self {
        let raw =
            NonNull::new(unsafe { ffi::webrtc_RtpEncodingParameters_vector_clone(src.as_ptr()) })
                .expect("webrtc_RtpEncodingParameters_vector_clone が null を返しました");
        Self { raw }
    }

    pub fn clone_self(&self) -> Self {
        Self::clone_from_raw(self.raw)
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_RtpEncodingParameters_vector {
        self.raw.as_ptr()
    }
}

impl Clone for RtpEncodingParametersVector {
    fn clone(&self) -> Self {
        self.clone_self()
    }
}

impl Drop for RtpEncodingParametersVector {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_RtpEncodingParameters_vector_delete(self.raw.as_ptr()) };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DegradationPreference {
    Disabled,
    MaintainFramerate,
    MaintainResolution,
    Balanced,
    Unknown(i32),
}

impl DegradationPreference {
    pub fn to_int(self) -> i32 {
        match self {
            DegradationPreference::Disabled => unsafe {
                ffi::webrtc_DegradationPreference_DISABLED
            },
            DegradationPreference::MaintainFramerate => unsafe {
                ffi::webrtc_DegradationPreference_MAINTAIN_FRAMERATE
            },
            DegradationPreference::MaintainResolution => unsafe {
                ffi::webrtc_DegradationPreference_MAINTAIN_RESOLUTION
            },
            DegradationPreference::Balanced => unsafe {
                ffi::webrtc_DegradationPreference_BALANCED
            },
            DegradationPreference::Unknown(v) => v,
        }
    }

    pub fn from_int(v: i32) -> Self {
        match v {
            x if x == unsafe { ffi::webrtc_DegradationPreference_DISABLED } => {
                DegradationPreference::Disabled
            }
            x if x == unsafe { ffi::webrtc_DegradationPreference_MAINTAIN_FRAMERATE } => {
                DegradationPreference::MaintainFramerate
            }
            x if x == unsafe { ffi::webrtc_DegradationPreference_MAINTAIN_RESOLUTION } => {
                DegradationPreference::MaintainResolution
            }
            x if x == unsafe { ffi::webrtc_DegradationPreference_BALANCED } => {
                DegradationPreference::Balanced
            }
            _ => DegradationPreference::Unknown(v),
        }
    }
}

unsafe impl Send for DegradationPreference {}

/// webrtc::RtpParameters のラッパー。
pub struct RtpParameters {
    raw: NonNull<ffi::webrtc_RtpParameters>,
}

impl Default for RtpParameters {
    fn default() -> Self {
        Self::new()
    }
}

impl RtpParameters {
    pub fn new() -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_RtpParameters_new() })
            .expect("BUG: webrtc_RtpParameters_new が null を返しました");
        Self { raw }
    }

    pub fn from_raw(raw: NonNull<ffi::webrtc_RtpParameters>) -> Self {
        Self { raw }
    }

    pub fn transaction_id(&self) -> Result<String> {
        let ptr = unsafe { ffi::webrtc_RtpParameters_get_transaction_id(self.raw.as_ptr()) };
        CxxStringRef::from_ptr(
            NonNull::new(ptr)
                .expect("BUG: webrtc_RtpParameters_get_transaction_id が null を返しました"),
        )
        .to_string()
    }

    pub fn set_transaction_id(&mut self, value: &str) {
        unsafe {
            ffi::webrtc_RtpParameters_set_transaction_id(
                self.raw.as_ptr(),
                value.as_ptr() as *const _,
                value.len(),
            );
        }
    }

    pub fn mid(&self) -> Result<String> {
        let ptr = unsafe { ffi::webrtc_RtpParameters_get_mid(self.raw.as_ptr()) };
        CxxStringRef::from_ptr(
            NonNull::new(ptr).expect("BUG: webrtc_RtpParameters_get_mid が null を返しました"),
        )
        .to_string()
    }

    pub fn set_mid(&mut self, value: &str) {
        unsafe {
            ffi::webrtc_RtpParameters_set_mid(
                self.raw.as_ptr(),
                value.as_ptr() as *const _,
                value.len(),
            );
        }
    }

    pub fn encodings(&self) -> RtpEncodingParametersVector {
        let ptr = unsafe { ffi::webrtc_RtpParameters_get_encodings(self.raw.as_ptr()) };
        let raw = NonNull::new(ptr).expect("BUG: webrtc_RtpParameters_get_encodings が null");
        RtpEncodingParametersVector::clone_from_raw(raw)
    }

    pub fn set_encodings(&mut self, encodings: &RtpEncodingParametersVector) {
        unsafe { ffi::webrtc_RtpParameters_set_encodings(self.raw.as_ptr(), encodings.as_ptr()) };
    }

    pub fn degradation_preference(&self) -> Option<DegradationPreference> {
        let mut has = 0;
        let mut value = 0;
        unsafe {
            ffi::webrtc_RtpParameters_get_degradation_preference(
                self.raw.as_ptr(),
                &mut has,
                &mut value,
            );
        }
        if has == 0 {
            None
        } else {
            Some(DegradationPreference::from_int(value))
        }
    }

    pub fn set_degradation_preference(&mut self, value: Option<DegradationPreference>) {
        match value {
            Some(v) => {
                let raw = v.to_int();
                unsafe {
                    ffi::webrtc_RtpParameters_set_degradation_preference(
                        self.raw.as_ptr(),
                        1,
                        &raw,
                    );
                }
            }
            None => unsafe {
                ffi::webrtc_RtpParameters_set_degradation_preference(
                    self.raw.as_ptr(),
                    0,
                    std::ptr::null(),
                );
            },
        }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_RtpParameters {
        self.raw.as_ptr()
    }
}

impl Drop for RtpParameters {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_RtpParameters_delete(self.raw.as_ptr()) };
    }
}

// 安全性: 所有ポインタは drop 時に解放されるだけで共有状態を持たない。
unsafe impl Send for RtpParameters {}

/// RtpTransceiverDirection のラッパー。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RtpTransceiverDirection {
    SendRecv,
    SendOnly,
    RecvOnly,
    Unknown(i32),
}

impl RtpTransceiverDirection {
    pub fn to_int(self) -> i32 {
        match self {
            RtpTransceiverDirection::SendRecv => unsafe {
                ffi::webrtc_RtpTransceiverDirection_kSendRecv
            },
            RtpTransceiverDirection::SendOnly => unsafe {
                ffi::webrtc_RtpTransceiverDirection_kSendOnly
            },
            RtpTransceiverDirection::RecvOnly => unsafe {
                ffi::webrtc_RtpTransceiverDirection_kRecvOnly
            },
            RtpTransceiverDirection::Unknown(v) => v,
        }
    }
}

/// RtpTransceiverInit のラッパー。
pub struct RtpTransceiverInit {
    raw: NonNull<ffi::webrtc_RtpTransceiverInit>,
}

impl Default for RtpTransceiverInit {
    fn default() -> Self {
        Self::new()
    }
}

impl RtpTransceiverInit {
    pub fn new() -> Self {
        let raw = unsafe { ffi::webrtc_RtpTransceiverInit_new() };
        Self {
            raw: NonNull::new(raw)
                .expect("BUG: webrtc_RtpTransceiverInit_new が null を返しました"),
        }
    }

    pub fn set_direction(&mut self, direction: RtpTransceiverDirection) {
        unsafe {
            ffi::webrtc_RtpTransceiverInit_set_direction(self.raw.as_ptr(), direction.to_int());
        }
    }

    pub fn stream_ids(&mut self) -> StringVectorRef<'_> {
        let raw = unsafe { ffi::webrtc_RtpTransceiverInit_get_stream_ids(self.raw.as_ptr()) };
        StringVectorRef::from_raw(
            NonNull::new(raw)
                .expect("BUG: webrtc_RtpTransceiverInit_get_stream_ids が null を返しました"),
        )
    }

    pub fn set_send_encodings(&mut self, encodings: &RtpEncodingParametersVector) {
        unsafe {
            ffi::webrtc_RtpTransceiverInit_set_send_encodings(
                self.raw.as_ptr(),
                encodings.as_ptr(),
            );
        }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_RtpTransceiverInit {
        self.raw.as_ptr()
    }
}

impl Drop for RtpTransceiverInit {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_RtpTransceiverInit_delete(self.raw.as_ptr()) };
    }
}
/// RtpTransceiverInterface のラッパー。
pub struct RtpTransceiver {
    raw_ref: ScopedRef<RtpTransceiverHandle>,
}

impl RtpTransceiver {
    pub(crate) fn from_scoped_ref(raw_ref: ScopedRef<RtpTransceiverHandle>) -> Self {
        Self { raw_ref }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_RtpTransceiverInterface {
        self.raw_ref.as_ptr()
    }

    pub fn receiver(&self) -> RtpReceiver {
        let raw = unsafe { ffi::webrtc_RtpTransceiverInterface_receiver(self.raw_ref.as_ptr()) };
        let raw_ref = ScopedRef::<RtpReceiverHandle>::from_raw(
            NonNull::new(raw)
                .expect("BUG: webrtc_RtpTransceiverInterface_receiver が null を返しました"),
        );
        RtpReceiver::from_scoped_ref(raw_ref)
    }

    pub fn set_codec_preferences(&mut self, codecs: &RtpCodecCapabilityVector) -> Result<()> {
        let err = unsafe {
            ffi::webrtc_RtpTransceiverInterface_SetCodecPreferences(
                self.raw_ref.as_ptr(),
                codecs.as_ptr(),
            )
        };
        if !err.is_null() {
            let rtc = RtcError::from_unique_ptr(NonNull::new(err).unwrap());
            return Err(Error::RtcError(rtc));
        }
        Ok(())
    }
}

// 安全性: libwebrtc 側で参照カウント管理されたポインタのみを保持する。
unsafe impl Send for RtpTransceiver {}

/// webrtc::RtpReceiverInterface のラッパー。
pub struct RtpReceiver {
    raw_ref: ScopedRef<RtpReceiverHandle>,
}

impl RtpReceiver {
    pub(crate) fn from_scoped_ref(raw_ref: ScopedRef<RtpReceiverHandle>) -> Self {
        Self { raw_ref }
    }

    pub fn track(&self) -> MediaStreamTrack {
        let raw = unsafe { ffi::webrtc_RtpReceiverInterface_track(self.raw_ref.as_ptr()) };
        let raw_ref = ScopedRef::<MediaStreamTrackHandle>::from_raw(
            NonNull::new(raw).expect("BUG: webrtc_RtpReceiverInterface_track が null を返しました"),
        );
        MediaStreamTrack::from_scoped_ref(raw_ref)
    }
}

// 安全性: libwebrtc 側で参照カウント管理されたポインタのみを保持する。
unsafe impl Send for RtpReceiver {}

/// webrtc::RtpSenderInterface のラッパー。
pub struct RtpSender {
    raw_ref: ScopedRef<RtpSenderHandle>,
}

impl RtpSender {
    pub(crate) fn from_scoped_ref(raw_ref: ScopedRef<RtpSenderHandle>) -> Self {
        Self { raw_ref }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_RtpSenderInterface {
        self.raw_ref.as_ptr()
    }

    pub fn as_refcounted_ptr(&self) -> *mut ffi::webrtc_RtpSenderInterface_refcounted {
        self.raw_ref.as_refcounted_ptr()
    }

    pub fn get_parameters(&self) -> RtpParameters {
        let raw =
            NonNull::new(unsafe { ffi::webrtc_RtpSenderInterface_GetParameters(self.as_ptr()) })
                .expect("BUG: webrtc_RtpSenderInterface_GetParameters が null を返しました");
        RtpParameters::from_raw(raw)
    }

    pub fn set_parameters(&mut self, parameters: &RtpParameters) -> Result<()> {
        let err = unsafe {
            ffi::webrtc_RtpSenderInterface_SetParameters(self.as_ptr(), parameters.as_ptr())
        };
        if !err.is_null() {
            let rtc = RtcError::from_unique_ptr(NonNull::new(err).unwrap());
            return Err(Error::RtcError(rtc));
        }
        Ok(())
    }
}

// 安全性: libwebrtc 側で参照カウント管理されたポインタのみを保持する。
unsafe impl Send for RtpSender {}

/// webrtc::MediaStreamTrackInterface のラッパー。
pub struct MediaStreamTrack {
    raw_ref: ScopedRef<MediaStreamTrackHandle>,
}

impl MediaStreamTrack {
    pub(crate) fn from_scoped_ref(raw_ref: ScopedRef<MediaStreamTrackHandle>) -> Self {
        Self { raw_ref }
    }

    pub fn as_refcounted_ptr(&self) -> *mut ffi::webrtc_MediaStreamTrackInterface_refcounted {
        self.raw_ref.as_refcounted_ptr()
    }

    pub fn kind(&self) -> Result<String> {
        let raw = self.raw_ref.as_ptr();
        let ptr = unsafe { ffi::webrtc_MediaStreamTrackInterface_kind(raw) };
        let kind = CxxString::from_unique(NonNull::new(ptr).expect("BUG: ptr が null"));
        kind.to_string()
    }

    pub fn id(&self) -> Result<String> {
        let raw = self.raw_ref.as_ptr();
        let ptr = unsafe { ffi::webrtc_MediaStreamTrackInterface_id(raw) };
        let id = CxxString::from_unique(NonNull::new(ptr).expect("BUG: ptr が null"));
        id.to_string()
    }

    pub fn cast_to_video_track(&self) -> VideoTrack {
        let raw_ref = unsafe {
            ffi::webrtc_MediaStreamTrackInterface_refcounted_cast_to_webrtc_VideoTrackInterface(
                self.raw_ref.as_refcounted_ptr(),
            )
        };
        assert!(!raw_ref.is_null());
        let raw_ref = ScopedRef::<VideoTrackHandle>::from_raw(NonNull::new(raw_ref).unwrap());
        VideoTrack::from_scoped_ref(raw_ref)
    }
}
