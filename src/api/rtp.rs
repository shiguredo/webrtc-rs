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
}

impl Drop for RtpCapabilities {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_RtpCapabilities_delete(self.raw.as_ptr()) };
    }
}

impl RtpCapabilities {
    /// codecs のベクタを借用する。
    pub fn codecs(&self) -> RtpCodecCapabilityVectorRef<'_> {
        let raw =
            NonNull::new(unsafe { ffi::webrtc_RtpCapabilities_get_codecs(self.raw.as_ptr()) })
                .expect("BUG: webrtc_RtpCapabilities_get_codecs が null を返しました");
        RtpCodecCapabilityVectorRef::from_raw(raw)
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

    pub fn set_kind(&mut self, media_type: MediaType) {
        unsafe {
            ffi::webrtc_RtpCodecCapability_set_kind(self.raw.as_ptr(), media_type.to_int());
        }
    }

    pub fn set_name(&mut self, name: &str) {
        unsafe {
            ffi::webrtc_RtpCodecCapability_set_name(
                self.raw.as_ptr(),
                name.as_ptr() as *const _,
                name.len(),
            );
        }
    }

    pub fn set_clock_rate(&mut self, rate: i32) {
        unsafe { ffi::webrtc_RtpCodecCapability_set_clock_rate(self.raw.as_ptr(), rate) };
    }

    pub fn name(&self) -> Result<String> {
        let ptr = unsafe { ffi::webrtc_RtpCodecCapability_get_name(self.raw.as_ptr()) };
        CxxStringRef::from_ptr(
            NonNull::new(ptr)
                .expect("BUG: webrtc_RtpCodecCapability_get_name が null を返しました"),
        )
        .to_string()
    }

    pub fn parameters(&mut self) -> MapStringString<'_> {
        let raw = unsafe { ffi::webrtc_RtpCodecCapability_get_parameters(self.raw.as_ptr()) };
        MapStringString::from_raw(
            NonNull::new(raw)
                .expect("BUG: webrtc_RtpCodecCapability_get_parameters が null を返しました"),
        )
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
        let len = unsafe { ffi::webrtc_RtpCodecCapability_vector_size(self.raw.as_ptr()) };
        len.max(0) as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: usize) -> Option<RtpCodecCapabilityRef<'_>> {
        if index >= self.len() {
            return None;
        }
        let raw = NonNull::new(unsafe {
            ffi::webrtc_RtpCodecCapability_vector_get(self.raw.as_ptr(), index as i32)
        })
        .expect("BUG: webrtc_RtpCodecCapability_vector_get が null を返しました");
        Some(RtpCodecCapabilityRef::from_raw(raw))
    }

    pub fn push(&mut self, cap: &RtpCodecCapability) {
        unsafe {
            ffi::webrtc_RtpCodecCapability_vector_push_back(self.raw.as_ptr(), cap.as_ptr());
        }
    }

    pub fn resize(&mut self, len: usize) {
        let len = i32::try_from(len).unwrap_or(i32::MAX);
        unsafe { ffi::webrtc_RtpCodecCapability_vector_resize(self.raw.as_ptr(), len) };
    }

    pub fn push_ref(&mut self, cap: &RtpCodecCapabilityRef<'_>) {
        unsafe {
            ffi::webrtc_RtpCodecCapability_vector_push_back(self.raw.as_ptr(), cap.as_ptr());
        }
    }

    pub fn set(&mut self, index: usize, cap: &RtpCodecCapability) -> bool {
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

    pub fn as_ptr(&self) -> *mut ffi::webrtc_RtpCodecCapability_vector {
        self.raw.as_ptr()
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

    pub fn get(&self, index: usize) -> Option<RtpCodecCapabilityRef<'_>> {
        if index >= self.len() {
            return None;
        }
        let raw = NonNull::new(unsafe {
            ffi::webrtc_RtpCodecCapability_vector_get(self.raw.as_ptr(), index as i32)
        })
        .expect("BUG: webrtc_RtpCodecCapability_vector_get が null を返しました");
        Some(RtpCodecCapabilityRef::from_raw(raw))
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

    pub fn name(&self) -> Result<String> {
        let ptr = unsafe { ffi::webrtc_RtpCodecCapability_get_name(self.raw.as_ptr()) };
        CxxStringRef::from_ptr(
            NonNull::new(ptr)
                .expect("BUG: webrtc_RtpCodecCapability_get_name が null を返しました"),
        )
        .to_string()
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_RtpCodecCapability {
        self.raw.as_ptr()
    }
}
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

    pub fn set_rid(&mut self, rid: &str) {
        unsafe {
            ffi::webrtc_RtpEncodingParameters_set_rid(
                self.raw.as_ptr(),
                rid.as_ptr() as *const _,
                rid.len(),
            );
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

    pub fn set_scale_resolution_down_by(&mut self, scale: f64) {
        unsafe {
            ffi::webrtc_RtpEncodingParameters_set_scale_resolution_down_by(self.raw.as_ptr(), scale)
        };
    }

    pub fn has_codec(&self) -> bool {
        unsafe { ffi::webrtc_RtpEncodingParameters_has_codec(self.raw.as_ptr()) != 0 }
    }

    pub fn set_codec(&mut self, codec: &RtpCodecCapability) {
        unsafe { ffi::webrtc_RtpEncodingParameters_set_codec(self.raw.as_ptr(), codec.as_ptr()) };
    }

    pub fn codec(&self) -> RtpCodecCapabilityRef<'_> {
        let raw =
            NonNull::new(unsafe { ffi::webrtc_RtpEncodingParameters_get_codec(self.raw.as_ptr()) })
                .expect("BUG: webrtc_RtpEncodingParameters_get_codec が null を返しました");
        RtpCodecCapabilityRef::from_raw(raw)
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

    pub fn has_codec(&self) -> bool {
        unsafe { ffi::webrtc_RtpEncodingParameters_has_codec(self.raw.as_ptr()) != 0 }
    }

    pub fn codec(&self) -> RtpCodecCapabilityRef<'_> {
        let raw =
            NonNull::new(unsafe { ffi::webrtc_RtpEncodingParameters_get_codec(self.raw.as_ptr()) })
                .expect("BUG: webrtc_RtpEncodingParameters_get_codec が null を返しました");
        RtpCodecCapabilityRef::from_raw(raw)
    }

    pub fn set_codec(&mut self, codec: &RtpCodecCapability) {
        unsafe { ffi::webrtc_RtpEncodingParameters_set_codec(self.raw.as_ptr(), codec.as_ptr()) };
    }
}

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

    pub fn set_codec_preferences(&self, codecs: &RtpCodecCapabilityVector) -> Result<()> {
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
}

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
