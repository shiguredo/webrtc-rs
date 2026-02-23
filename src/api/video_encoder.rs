use super::video_codec_common::{
    EncodedImageRef, SdpVideoFormat, SdpVideoFormatRef, VideoCodecRef, VideoCodecStatus,
    VideoCodecType, VideoFrame, VideoFrameRef, VideoFrameTypeVectorRef,
};
use crate::{CxxString, EnvironmentRef, Result, ffi};
use std::marker::PhantomData;
use std::os::raw::c_void;
use std::ptr::NonNull;

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

type VideoEncoderInitEncodeCallback = Box<
    dyn for<'a> FnMut(VideoCodecRef<'a>, VideoEncoderSettingsRef<'a>) -> VideoCodecStatus
        + Send
        + 'static,
>;
type VideoEncoderEncodeCallback = Box<
    dyn for<'a> FnMut(VideoFrameRef<'a>, Option<VideoFrameTypeVectorRef<'a>>) -> VideoCodecStatus
        + Send
        + 'static,
>;
type VideoEncoderRegisterEncodeCompleteCallback = Box<
    dyn for<'a> FnMut(Option<VideoEncoderEncodedImageCallbackRef<'a>>) -> VideoCodecStatus
        + Send
        + 'static,
>;
type VideoEncoderReleaseCallback = Box<dyn FnMut() -> VideoCodecStatus + Send + 'static>;
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
            self.init_encode = Some(Box::new(|_, _| VideoCodecStatus::Ok));
        }
        if self.encode.is_none() {
            self.encode = Some(Box::new(|_, _| VideoCodecStatus::Ok));
        }
        if self.register_encode_complete_callback.is_none() {
            self.register_encode_complete_callback = Some(Box::new(|_| VideoCodecStatus::Ok));
        }
        if self.release.is_none() {
            self.release = Some(Box::new(|| VideoCodecStatus::Ok));
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

struct VideoEncoderCallbackState {
    callbacks: VideoEncoderCallbacks,
}

struct VideoEncoderEncodedImageCallbackState {
    callbacks: VideoEncoderEncodedImageCallbackCallbacks,
}

struct VideoEncoderFactoryCallbackState {
    callbacks: VideoEncoderFactoryCallbacks,
}

unsafe extern "C" fn video_encoder_on_destroy(user_data: *mut c_void) {
    assert!(
        !user_data.is_null(),
        "video_encoder_on_destroy: user_data is null"
    );
    let _ = unsafe { Box::from_raw(user_data as *mut VideoEncoderCallbackState) };
}

unsafe extern "C" fn video_encoder_encoded_image_callback_on_destroy(user_data: *mut c_void) {
    assert!(
        !user_data.is_null(),
        "video_encoder_encoded_image_callback_on_destroy: user_data is null"
    );
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
    cb(codec_settings, settings).to_raw()
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
    cb(frame, frame_types).to_raw()
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
    cb(callback).to_raw()
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
    cb().to_raw()
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

unsafe extern "C" fn video_encoder_factory_on_destroy(user_data: *mut c_void) {
    assert!(
        !user_data.is_null(),
        "video_encoder_factory_on_destroy: user_data is null"
    );
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

    pub fn init_encode(&mut self) -> VideoCodecStatus {
        let value = unsafe {
            ffi::webrtc_VideoEncoder_InitEncode(
                self.as_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        };
        VideoCodecStatus::from_raw(value)
    }

    pub fn encode(&mut self, frame: &VideoFrame) -> VideoCodecStatus {
        self.encode_with_frame_types(frame, None)
    }

    pub fn encode_with_frame_types(
        &mut self,
        frame: &VideoFrame,
        frame_types: Option<VideoFrameTypeVectorRef<'_>>,
    ) -> VideoCodecStatus {
        let frame_types =
            frame_types.map_or(std::ptr::null_mut(), |frame_types| frame_types.as_ptr());
        let value = unsafe {
            ffi::webrtc_VideoEncoder_Encode(self.as_ptr(), frame.raw().as_ptr(), frame_types)
        };
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

    pub fn set_rates(&mut self) {
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
        self.create_from_ref(env.as_ref(), format.as_ref())
    }

    pub fn create_from_ref(
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
        let Some(raw_vec) = NonNull::new(raw_vec) else {
            return Vec::new();
        };
        let size = unsafe { ffi::webrtc_SdpVideoFormat_vector_size(raw_vec.as_ptr()) };
        let mut formats = Vec::with_capacity(size.max(0) as usize);
        for i in 0..size {
            let raw_format = unsafe { ffi::webrtc_SdpVideoFormat_vector_get(raw_vec.as_ptr(), i) };
            let Some(raw_format) = NonNull::new(raw_format) else {
                continue;
            };
            let format_ref = unsafe { SdpVideoFormatRef::from_raw(raw_format) };
            formats.push(clone_sdp_video_format(format_ref));
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

fn clone_sdp_video_format(mut format: SdpVideoFormatRef<'_>) -> SdpVideoFormat {
    let name = format
        .name()
        .expect("SdpVideoFormatRef::name の取得に失敗しました");
    let mut out = SdpVideoFormat::new(&name);
    let mut out_params = out.parameters_mut();
    for (key, value) in format.parameters_mut().iter() {
        out_params.set(&key, &value);
    }
    out
}
