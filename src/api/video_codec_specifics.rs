use super::video_encoder::H264PacketizationMode;
use crate::ffi;
use std::marker::PhantomData;
use std::ptr::NonNull;

/// H.264 のパケタイズ種別を表す。
///
/// libwebrtc の `H264PacketizationTypes` に対応する。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum H264PacketizationType {
    /// 単一の NAL ユニットを含むパケット。
    SingleNalu,
    /// STAP-A (単一時間アグリゲーション) パケット。
    StapA,
    /// FU-A (フラグメンテーションユニット) パケット。
    FuA,
    Unknown(i32),
}

impl H264PacketizationType {
    pub(crate) fn from_raw(value: i32) -> Self {
        if value == unsafe { ffi::webrtc_H264PacketizationType_SingleNalu } {
            Self::SingleNalu
        } else if value == unsafe { ffi::webrtc_H264PacketizationType_StapA } {
            Self::StapA
        } else if value == unsafe { ffi::webrtc_H264PacketizationType_FuA } {
            Self::FuA
        } else {
            Self::Unknown(value)
        }
    }

    pub(crate) fn to_raw(self) -> i32 {
        match self {
            Self::SingleNalu => unsafe { ffi::webrtc_H264PacketizationType_SingleNalu },
            Self::StapA => unsafe { ffi::webrtc_H264PacketizationType_StapA },
            Self::FuA => unsafe { ffi::webrtc_H264PacketizationType_FuA },
            Self::Unknown(v) => v,
        }
    }
}

/// webrtc::GofInfoVP9 の所有ラッパー。
pub struct GofInfoVP9 {
    raw: NonNull<ffi::webrtc_GofInfoVP9>,
}

unsafe impl Send for GofInfoVP9 {}

impl GofInfoVP9 {
    /// 新しく生成する。
    pub fn new() -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_GofInfoVP9_new() })
            .expect("BUG: webrtc_GofInfoVP9_new が null を返しました");
        Self { raw }
    }

    /// 借用ポインタからコピーを生成する。
    ///
    /// # Safety
    /// `raw` は有効な `webrtc_GofInfoVP9` を指している必要があります。
    pub(crate) unsafe fn copy_from_raw(raw: *mut ffi::webrtc_GofInfoVP9) -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_GofInfoVP9_copy(raw) })
            .expect("BUG: webrtc_GofInfoVP9_copy が null を返しました");
        Self { raw }
    }

    pub(crate) fn as_ptr(&self) -> *mut ffi::webrtc_GofInfoVP9 {
        self.raw.as_ptr()
    }

    pub fn num_frames_in_gof(&self) -> usize {
        unsafe { ffi::webrtc_GofInfoVP9_get_num_frames_in_gof(self.raw.as_ptr()) }
    }

    pub fn set_num_frames_in_gof(&mut self, value: usize) {
        unsafe { ffi::webrtc_GofInfoVP9_set_num_frames_in_gof(self.raw.as_ptr(), value) };
    }

    pub fn temporal_idx(&self, index: usize) -> u8 {
        unsafe { ffi::webrtc_GofInfoVP9_get_temporal_idx(self.raw.as_ptr(), index) }
    }

    pub fn set_temporal_idx(&mut self, index: usize, value: u8) {
        unsafe { ffi::webrtc_GofInfoVP9_set_temporal_idx(self.raw.as_ptr(), index, value) };
    }

    pub fn temporal_up_switch(&self, index: usize) -> bool {
        unsafe { ffi::webrtc_GofInfoVP9_get_temporal_up_switch(self.raw.as_ptr(), index) != 0 }
    }

    pub fn set_temporal_up_switch(&mut self, index: usize, value: bool) {
        unsafe {
            ffi::webrtc_GofInfoVP9_set_temporal_up_switch(
                self.raw.as_ptr(),
                index,
                if value { 1 } else { 0 },
            )
        };
    }

    pub fn num_ref_pics(&self, index: usize) -> u8 {
        unsafe { ffi::webrtc_GofInfoVP9_get_num_ref_pics(self.raw.as_ptr(), index) }
    }

    pub fn set_num_ref_pics(&mut self, index: usize, value: u8) {
        unsafe { ffi::webrtc_GofInfoVP9_set_num_ref_pics(self.raw.as_ptr(), index, value) };
    }

    pub fn pid_diff(&self, index: usize, ref_index: usize) -> u8 {
        unsafe { ffi::webrtc_GofInfoVP9_get_pid_diff(self.raw.as_ptr(), index, ref_index) }
    }

    pub fn set_pid_diff(&mut self, index: usize, ref_index: usize, value: u8) {
        unsafe { ffi::webrtc_GofInfoVP9_set_pid_diff(self.raw.as_ptr(), index, ref_index, value) };
    }

    pub fn pid_start(&self) -> u16 {
        unsafe { ffi::webrtc_GofInfoVP9_get_pid_start(self.raw.as_ptr()) }
    }

    pub fn set_pid_start(&mut self, value: u16) {
        unsafe { ffi::webrtc_GofInfoVP9_set_pid_start(self.raw.as_ptr(), value) };
    }
}

impl Default for GofInfoVP9 {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for GofInfoVP9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num_frames = self.num_frames_in_gof();
        f.debug_struct("GofInfoVP9")
            .field("num_frames_in_gof", &num_frames)
            .field(
                "temporal_idx",
                &(0..num_frames)
                    .map(|i| self.temporal_idx(i))
                    .collect::<Vec<_>>(),
            )
            .field(
                "temporal_up_switch",
                &(0..num_frames)
                    .map(|i| self.temporal_up_switch(i))
                    .collect::<Vec<_>>(),
            )
            .field(
                "num_ref_pics",
                &(0..num_frames)
                    .map(|i| self.num_ref_pics(i))
                    .collect::<Vec<_>>(),
            )
            .field(
                "pid_diff",
                &(0..num_frames)
                    .map(|i| {
                        let num_ref = self.num_ref_pics(i);
                        (0..num_ref as usize)
                            .map(|r| self.pid_diff(i, r))
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>(),
            )
            .field("pid_start", &self.pid_start())
            .finish()
    }
}

impl Drop for GofInfoVP9 {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_GofInfoVP9_delete(self.raw.as_ptr()) };
    }
}

/// webrtc::RTPVideoHeaderVP8 の所有ラッパー。
pub struct RTPVideoHeaderVP8 {
    raw: NonNull<ffi::webrtc_RTPVideoHeaderVP8>,
}

unsafe impl Send for RTPVideoHeaderVP8 {}

impl RTPVideoHeaderVP8 {
    /// 新しく生成する。
    pub fn new() -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_RTPVideoHeaderVP8_new() })
            .expect("BUG: webrtc_RTPVideoHeaderVP8_new が null を返しました");
        Self { raw }
    }

    /// 借用ポインタからコピーを生成する。
    ///
    /// # Safety
    /// `raw` は有効な `webrtc_RTPVideoHeaderVP8` を指している必要があります。
    pub(crate) unsafe fn copy_from_raw(raw: *mut ffi::webrtc_RTPVideoHeaderVP8) -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_RTPVideoHeaderVP8_copy(raw) })
            .expect("BUG: webrtc_RTPVideoHeaderVP8_copy が null を返しました");
        Self { raw }
    }

    pub(crate) fn as_ptr(&self) -> *mut ffi::webrtc_RTPVideoHeaderVP8 {
        self.raw.as_ptr()
    }

    pub fn non_reference(&self) -> bool {
        unsafe { ffi::webrtc_RTPVideoHeaderVP8_get_nonReference(self.raw.as_ptr()) != 0 }
    }

    pub fn set_non_reference(&mut self, value: bool) {
        unsafe {
            ffi::webrtc_RTPVideoHeaderVP8_set_nonReference(
                self.raw.as_ptr(),
                if value { 1 } else { 0 },
            )
        };
    }

    pub fn picture_id(&self) -> i16 {
        unsafe { ffi::webrtc_RTPVideoHeaderVP8_get_pictureId(self.raw.as_ptr()) }
    }

    pub fn set_picture_id(&mut self, value: i16) {
        unsafe { ffi::webrtc_RTPVideoHeaderVP8_set_pictureId(self.raw.as_ptr(), value) };
    }

    pub fn tl0_pic_idx(&self) -> i16 {
        unsafe { ffi::webrtc_RTPVideoHeaderVP8_get_tl0PicIdx(self.raw.as_ptr()) }
    }

    pub fn set_tl0_pic_idx(&mut self, value: i16) {
        unsafe { ffi::webrtc_RTPVideoHeaderVP8_set_tl0PicIdx(self.raw.as_ptr(), value) };
    }

    pub fn temporal_idx(&self) -> u8 {
        unsafe { ffi::webrtc_RTPVideoHeaderVP8_get_temporalIdx(self.raw.as_ptr()) }
    }

    pub fn set_temporal_idx(&mut self, value: u8) {
        unsafe { ffi::webrtc_RTPVideoHeaderVP8_set_temporalIdx(self.raw.as_ptr(), value) };
    }

    pub fn layer_sync(&self) -> bool {
        unsafe { ffi::webrtc_RTPVideoHeaderVP8_get_layerSync(self.raw.as_ptr()) != 0 }
    }

    pub fn set_layer_sync(&mut self, value: bool) {
        unsafe {
            ffi::webrtc_RTPVideoHeaderVP8_set_layerSync(
                self.raw.as_ptr(),
                if value { 1 } else { 0 },
            )
        };
    }

    pub fn key_idx(&self) -> i32 {
        unsafe { ffi::webrtc_RTPVideoHeaderVP8_get_keyIdx(self.raw.as_ptr()) }
    }

    pub fn set_key_idx(&mut self, value: i32) {
        unsafe { ffi::webrtc_RTPVideoHeaderVP8_set_keyIdx(self.raw.as_ptr(), value) };
    }

    pub fn partition_id(&self) -> i32 {
        unsafe { ffi::webrtc_RTPVideoHeaderVP8_get_partitionId(self.raw.as_ptr()) }
    }

    pub fn set_partition_id(&mut self, value: i32) {
        unsafe { ffi::webrtc_RTPVideoHeaderVP8_set_partitionId(self.raw.as_ptr(), value) };
    }

    pub fn beginning_of_partition(&self) -> bool {
        unsafe { ffi::webrtc_RTPVideoHeaderVP8_get_beginningOfPartition(self.raw.as_ptr()) != 0 }
    }

    pub fn set_beginning_of_partition(&mut self, value: bool) {
        unsafe {
            ffi::webrtc_RTPVideoHeaderVP8_set_beginningOfPartition(
                self.raw.as_ptr(),
                if value { 1 } else { 0 },
            )
        };
    }
}

impl Default for RTPVideoHeaderVP8 {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for RTPVideoHeaderVP8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RTPVideoHeaderVP8")
            .field("non_reference", &self.non_reference())
            .field("picture_id", &self.picture_id())
            .field("tl0_pic_idx", &self.tl0_pic_idx())
            .field("temporal_idx", &self.temporal_idx())
            .field("layer_sync", &self.layer_sync())
            .field("key_idx", &self.key_idx())
            .field("partition_id", &self.partition_id())
            .field("beginning_of_partition", &self.beginning_of_partition())
            .finish()
    }
}

impl Drop for RTPVideoHeaderVP8 {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_RTPVideoHeaderVP8_delete(self.raw.as_ptr()) };
    }
}

/// webrtc::RTPVideoHeaderVP9 の所有ラッパー。
pub struct RTPVideoHeaderVP9 {
    raw: NonNull<ffi::webrtc_RTPVideoHeaderVP9>,
}

unsafe impl Send for RTPVideoHeaderVP9 {}

impl RTPVideoHeaderVP9 {
    /// 新しく生成する。
    pub fn new() -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_RTPVideoHeaderVP9_new() })
            .expect("BUG: webrtc_RTPVideoHeaderVP9_new が null を返しました");
        Self { raw }
    }

    /// 借用ポインタからコピーを生成する。
    ///
    /// # Safety
    /// `raw` は有効な `webrtc_RTPVideoHeaderVP9` を指している必要があります。
    pub(crate) unsafe fn copy_from_raw(raw: *mut ffi::webrtc_RTPVideoHeaderVP9) -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_RTPVideoHeaderVP9_copy(raw) })
            .expect("BUG: webrtc_RTPVideoHeaderVP9_copy が null を返しました");
        Self { raw }
    }

    pub(crate) fn as_ptr(&self) -> *mut ffi::webrtc_RTPVideoHeaderVP9 {
        self.raw.as_ptr()
    }

    pub fn inter_pic_predicted(&self) -> bool {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_get_inter_pic_predicted(self.raw.as_ptr()) != 0 }
    }

    pub fn set_inter_pic_predicted(&mut self, value: bool) {
        unsafe {
            ffi::webrtc_RTPVideoHeaderVP9_set_inter_pic_predicted(
                self.raw.as_ptr(),
                if value { 1 } else { 0 },
            )
        };
    }

    pub fn flexible_mode(&self) -> bool {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_get_flexible_mode(self.raw.as_ptr()) != 0 }
    }

    pub fn set_flexible_mode(&mut self, value: bool) {
        unsafe {
            ffi::webrtc_RTPVideoHeaderVP9_set_flexible_mode(
                self.raw.as_ptr(),
                if value { 1 } else { 0 },
            )
        };
    }

    pub fn beginning_of_frame(&self) -> bool {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_get_beginning_of_frame(self.raw.as_ptr()) != 0 }
    }

    pub fn set_beginning_of_frame(&mut self, value: bool) {
        unsafe {
            ffi::webrtc_RTPVideoHeaderVP9_set_beginning_of_frame(
                self.raw.as_ptr(),
                if value { 1 } else { 0 },
            )
        };
    }

    pub fn end_of_frame(&self) -> bool {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_get_end_of_frame(self.raw.as_ptr()) != 0 }
    }

    pub fn set_end_of_frame(&mut self, value: bool) {
        unsafe {
            ffi::webrtc_RTPVideoHeaderVP9_set_end_of_frame(
                self.raw.as_ptr(),
                if value { 1 } else { 0 },
            )
        };
    }

    pub fn ss_data_available(&self) -> bool {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_get_ss_data_available(self.raw.as_ptr()) != 0 }
    }

    pub fn set_ss_data_available(&mut self, value: bool) {
        unsafe {
            ffi::webrtc_RTPVideoHeaderVP9_set_ss_data_available(
                self.raw.as_ptr(),
                if value { 1 } else { 0 },
            )
        };
    }

    pub fn non_ref_for_inter_layer_pred(&self) -> bool {
        unsafe {
            ffi::webrtc_RTPVideoHeaderVP9_get_non_ref_for_inter_layer_pred(self.raw.as_ptr()) != 0
        }
    }

    pub fn set_non_ref_for_inter_layer_pred(&mut self, value: bool) {
        unsafe {
            ffi::webrtc_RTPVideoHeaderVP9_set_non_ref_for_inter_layer_pred(
                self.raw.as_ptr(),
                if value { 1 } else { 0 },
            )
        };
    }

    pub fn picture_id(&self) -> i16 {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_get_picture_id(self.raw.as_ptr()) }
    }

    pub fn set_picture_id(&mut self, value: i16) {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_set_picture_id(self.raw.as_ptr(), value) };
    }

    pub fn max_picture_id(&self) -> i16 {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_get_max_picture_id(self.raw.as_ptr()) }
    }

    pub fn set_max_picture_id(&mut self, value: i16) {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_set_max_picture_id(self.raw.as_ptr(), value) };
    }

    pub fn tl0_pic_idx(&self) -> i16 {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_get_tl0_pic_idx(self.raw.as_ptr()) }
    }

    pub fn set_tl0_pic_idx(&mut self, value: i16) {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_set_tl0_pic_idx(self.raw.as_ptr(), value) };
    }

    pub fn temporal_idx(&self) -> u8 {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_get_temporal_idx(self.raw.as_ptr()) }
    }

    pub fn set_temporal_idx(&mut self, value: u8) {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_set_temporal_idx(self.raw.as_ptr(), value) };
    }

    pub fn spatial_idx(&self) -> u8 {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_get_spatial_idx(self.raw.as_ptr()) }
    }

    pub fn set_spatial_idx(&mut self, value: u8) {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_set_spatial_idx(self.raw.as_ptr(), value) };
    }

    pub fn temporal_up_switch(&self) -> bool {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_get_temporal_up_switch(self.raw.as_ptr()) != 0 }
    }

    pub fn set_temporal_up_switch(&mut self, value: bool) {
        unsafe {
            ffi::webrtc_RTPVideoHeaderVP9_set_temporal_up_switch(
                self.raw.as_ptr(),
                if value { 1 } else { 0 },
            )
        };
    }

    pub fn inter_layer_predicted(&self) -> bool {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_get_inter_layer_predicted(self.raw.as_ptr()) != 0 }
    }

    pub fn set_inter_layer_predicted(&mut self, value: bool) {
        unsafe {
            ffi::webrtc_RTPVideoHeaderVP9_set_inter_layer_predicted(
                self.raw.as_ptr(),
                if value { 1 } else { 0 },
            )
        };
    }

    pub fn gof_idx(&self) -> u8 {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_get_gof_idx(self.raw.as_ptr()) }
    }

    pub fn set_gof_idx(&mut self, value: u8) {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_set_gof_idx(self.raw.as_ptr(), value) };
    }

    pub fn num_ref_pics(&self) -> u8 {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_get_num_ref_pics(self.raw.as_ptr()) }
    }

    pub fn set_num_ref_pics(&mut self, value: u8) {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_set_num_ref_pics(self.raw.as_ptr(), value) };
    }

    pub fn pid_diff(&self, index: usize) -> u8 {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_get_pid_diff(self.raw.as_ptr(), index) }
    }

    pub fn set_pid_diff(&mut self, index: usize, value: u8) {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_set_pid_diff(self.raw.as_ptr(), index, value) };
    }

    pub fn ref_picture_id(&self, index: usize) -> i16 {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_get_ref_picture_id(self.raw.as_ptr(), index) }
    }

    pub fn set_ref_picture_id(&mut self, index: usize, value: i16) {
        unsafe {
            ffi::webrtc_RTPVideoHeaderVP9_set_ref_picture_id(self.raw.as_ptr(), index, value)
        };
    }

    pub fn num_spatial_layers(&self) -> usize {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_get_num_spatial_layers(self.raw.as_ptr()) }
    }

    pub fn set_num_spatial_layers(&mut self, value: usize) {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_set_num_spatial_layers(self.raw.as_ptr(), value) };
    }

    pub fn first_active_layer(&self) -> usize {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_get_first_active_layer(self.raw.as_ptr()) }
    }

    pub fn set_first_active_layer(&mut self, value: usize) {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_set_first_active_layer(self.raw.as_ptr(), value) };
    }

    pub fn spatial_layer_resolution_present(&self) -> bool {
        unsafe {
            ffi::webrtc_RTPVideoHeaderVP9_get_spatial_layer_resolution_present(self.raw.as_ptr())
                != 0
        }
    }

    pub fn set_spatial_layer_resolution_present(&mut self, value: bool) {
        unsafe {
            ffi::webrtc_RTPVideoHeaderVP9_set_spatial_layer_resolution_present(
                self.raw.as_ptr(),
                if value { 1 } else { 0 },
            )
        };
    }

    pub fn width(&self, index: usize) -> u16 {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_get_width(self.raw.as_ptr(), index) }
    }

    pub fn set_width(&mut self, index: usize, value: u16) {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_set_width(self.raw.as_ptr(), index, value) };
    }

    pub fn height(&self, index: usize) -> u16 {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_get_height(self.raw.as_ptr(), index) }
    }

    pub fn set_height(&mut self, index: usize, value: u16) {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_set_height(self.raw.as_ptr(), index, value) };
    }

    pub fn gof(&self) -> GofInfoVP9 {
        let raw = unsafe { ffi::webrtc_RTPVideoHeaderVP9_get_gof(self.raw.as_ptr()) };
        unsafe { GofInfoVP9::copy_from_raw(raw) }
    }

    pub fn set_gof(&mut self, gof: &GofInfoVP9) {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_set_gof(self.raw.as_ptr(), gof.as_ptr()) };
    }

    pub fn end_of_picture(&self) -> bool {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_get_end_of_picture(self.raw.as_ptr()) != 0 }
    }

    pub fn set_end_of_picture(&mut self, value: bool) {
        unsafe {
            ffi::webrtc_RTPVideoHeaderVP9_set_end_of_picture(
                self.raw.as_ptr(),
                if value { 1 } else { 0 },
            )
        };
    }
}

impl Default for RTPVideoHeaderVP9 {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for RTPVideoHeaderVP9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num_spatial_layers = self.num_spatial_layers();
        let num_ref_pics = self.num_ref_pics();
        f.debug_struct("RTPVideoHeaderVP9")
            .field("inter_pic_predicted", &self.inter_pic_predicted())
            .field("flexible_mode", &self.flexible_mode())
            .field("beginning_of_frame", &self.beginning_of_frame())
            .field("end_of_frame", &self.end_of_frame())
            .field("ss_data_available", &self.ss_data_available())
            .field(
                "non_ref_for_inter_layer_pred",
                &self.non_ref_for_inter_layer_pred(),
            )
            .field("picture_id", &self.picture_id())
            .field("max_picture_id", &self.max_picture_id())
            .field("tl0_pic_idx", &self.tl0_pic_idx())
            .field("temporal_idx", &self.temporal_idx())
            .field("spatial_idx", &self.spatial_idx())
            .field("temporal_up_switch", &self.temporal_up_switch())
            .field("inter_layer_predicted", &self.inter_layer_predicted())
            .field("gof_idx", &self.gof_idx())
            .field("num_ref_pics", &num_ref_pics)
            .field(
                "pid_diff",
                &(0..num_ref_pics as usize)
                    .map(|i| self.pid_diff(i))
                    .collect::<Vec<_>>(),
            )
            .field(
                "ref_picture_id",
                &(0..num_ref_pics as usize)
                    .map(|i| self.ref_picture_id(i))
                    .collect::<Vec<_>>(),
            )
            .field("num_spatial_layers", &num_spatial_layers)
            .field("first_active_layer", &self.first_active_layer())
            .field(
                "spatial_layer_resolution_present",
                &self.spatial_layer_resolution_present(),
            )
            .field(
                "width",
                &(0..num_spatial_layers)
                    .map(|i| self.width(i))
                    .collect::<Vec<_>>(),
            )
            .field(
                "height",
                &(0..num_spatial_layers)
                    .map(|i| self.height(i))
                    .collect::<Vec<_>>(),
            )
            .field("gof", &self.gof())
            .field("end_of_picture", &self.end_of_picture())
            .finish()
    }
}

impl Drop for RTPVideoHeaderVP9 {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_RTPVideoHeaderVP9_delete(self.raw.as_ptr()) };
    }
}

/// webrtc::NaluInfo の所有ラッパー。
pub struct NaluInfo {
    raw: NonNull<ffi::webrtc_NaluInfo>,
}

unsafe impl Send for NaluInfo {}

impl NaluInfo {
    /// 新しく生成する。
    pub fn new() -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_NaluInfo_new() })
            .expect("BUG: webrtc_NaluInfo_new が null を返しました");
        Self { raw }
    }

    /// 借用ポインタからコピーを生成する。
    ///
    /// # Safety
    /// `raw` は有効な `webrtc_NaluInfo` を指している必要があります。
    pub(crate) unsafe fn copy_from_raw(raw: *mut ffi::webrtc_NaluInfo) -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_NaluInfo_copy(raw) })
            .expect("BUG: webrtc_NaluInfo_copy が null を返しました");
        Self { raw }
    }

    pub(crate) fn as_ptr(&self) -> *mut ffi::webrtc_NaluInfo {
        self.raw.as_ptr()
    }

    pub fn type_(&self) -> u8 {
        unsafe { ffi::webrtc_NaluInfo_get_type(self.raw.as_ptr()) }
    }

    pub fn set_type(&mut self, value: u8) {
        unsafe { ffi::webrtc_NaluInfo_set_type(self.raw.as_ptr(), value) };
    }

    pub fn sps_id(&self) -> i32 {
        unsafe { ffi::webrtc_NaluInfo_get_sps_id(self.raw.as_ptr()) }
    }

    pub fn set_sps_id(&mut self, value: i32) {
        unsafe { ffi::webrtc_NaluInfo_set_sps_id(self.raw.as_ptr(), value) };
    }

    pub fn pps_id(&self) -> i32 {
        unsafe { ffi::webrtc_NaluInfo_get_pps_id(self.raw.as_ptr()) }
    }

    pub fn set_pps_id(&mut self, value: i32) {
        unsafe { ffi::webrtc_NaluInfo_set_pps_id(self.raw.as_ptr(), value) };
    }
}

impl Default for NaluInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for NaluInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NaluInfo")
            .field("type_", &self.type_())
            .field("sps_id", &self.sps_id())
            .field("pps_id", &self.pps_id())
            .finish()
    }
}

impl Drop for NaluInfo {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_NaluInfo_delete(self.raw.as_ptr()) };
    }
}

/// webrtc::NaluInfo の借用ラッパー。
pub struct NaluInfoRef<'a> {
    raw: NonNull<ffi::webrtc_NaluInfo>,
    _marker: PhantomData<&'a ffi::webrtc_NaluInfo>,
}

unsafe impl<'a> Send for NaluInfoRef<'a> {}

impl<'a> NaluInfoRef<'a> {
    /// # Safety
    /// `raw` は有効な `webrtc_NaluInfo` を指し、この参照の利用中は破棄されない必要があります。
    pub(crate) unsafe fn from_raw(raw: NonNull<ffi::webrtc_NaluInfo>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn type_(&self) -> u8 {
        unsafe { ffi::webrtc_NaluInfo_get_type(self.raw.as_ptr()) }
    }

    pub fn sps_id(&self) -> i32 {
        unsafe { ffi::webrtc_NaluInfo_get_sps_id(self.raw.as_ptr()) }
    }

    pub fn pps_id(&self) -> i32 {
        unsafe { ffi::webrtc_NaluInfo_get_pps_id(self.raw.as_ptr()) }
    }
}

impl std::fmt::Debug for NaluInfoRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NaluInfoRef")
            .field("type_", &self.type_())
            .field("sps_id", &self.sps_id())
            .field("pps_id", &self.pps_id())
            .finish()
    }
}

/// std::vector<webrtc::NaluInfo> の所有ラッパー。
pub struct NaluInfoVector {
    raw: NonNull<ffi::webrtc_NaluInfo_vector>,
}

unsafe impl Send for NaluInfoVector {}

impl NaluInfoVector {
    /// 新しく生成する。
    pub fn new(size: usize) -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_NaluInfo_vector_new(size) })
            .expect("BUG: webrtc_NaluInfo_vector_new が null を返しました");
        Self { raw }
    }

    pub fn len(&self) -> usize {
        let len = unsafe { ffi::webrtc_NaluInfo_vector_size(self.raw.as_ptr()) };
        len.max(0) as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: usize) -> Option<NaluInfoRef<'_>> {
        if index >= self.len() {
            return None;
        }
        let raw = unsafe { ffi::webrtc_NaluInfo_vector_get(self.raw.as_ptr(), index as i32) };
        let raw = NonNull::new(raw)?;
        Some(unsafe { NaluInfoRef::from_raw(raw) })
    }

    pub fn push(&mut self, value: &NaluInfo) {
        unsafe { ffi::webrtc_NaluInfo_vector_push_back(self.raw.as_ptr(), value.as_ptr()) };
    }

    pub(crate) fn as_ptr(&self) -> *mut ffi::webrtc_NaluInfo_vector {
        self.raw.as_ptr()
    }
}

impl Drop for NaluInfoVector {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_NaluInfo_vector_delete(self.raw.as_ptr()) };
    }
}

impl std::fmt::Debug for NaluInfoVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut list = f.debug_list();
        for index in 0..self.len() {
            if let Some(value) = self.get(index) {
                list.entry(&value);
            }
        }
        list.finish()
    }
}

/// webrtc::RTPVideoHeaderH264 の所有ラッパー。
pub struct RTPVideoHeaderH264 {
    raw: NonNull<ffi::webrtc_RTPVideoHeaderH264>,
}

unsafe impl Send for RTPVideoHeaderH264 {}

impl RTPVideoHeaderH264 {
    /// 新しく生成する。
    pub fn new() -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_RTPVideoHeaderH264_new() })
            .expect("BUG: webrtc_RTPVideoHeaderH264_new が null を返しました");
        Self { raw }
    }

    /// 借用ポインタからコピーを生成する。
    ///
    /// # Safety
    /// `raw` は有効な `webrtc_RTPVideoHeaderH264` を指している必要があります。
    pub(crate) unsafe fn copy_from_raw(raw: *mut ffi::webrtc_RTPVideoHeaderH264) -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_RTPVideoHeaderH264_copy(raw) })
            .expect("BUG: webrtc_RTPVideoHeaderH264_copy が null を返しました");
        Self { raw }
    }

    pub(crate) fn as_ptr(&self) -> *mut ffi::webrtc_RTPVideoHeaderH264 {
        self.raw.as_ptr()
    }

    pub fn nalu_type(&self) -> u8 {
        unsafe { ffi::webrtc_RTPVideoHeaderH264_get_nalu_type(self.raw.as_ptr()) }
    }

    pub fn set_nalu_type(&mut self, value: u8) {
        unsafe { ffi::webrtc_RTPVideoHeaderH264_set_nalu_type(self.raw.as_ptr(), value) };
    }

    pub fn packetization_type(&self) -> H264PacketizationType {
        let value =
            unsafe { ffi::webrtc_RTPVideoHeaderH264_get_packetization_type(self.raw.as_ptr()) };
        H264PacketizationType::from_raw(value)
    }

    pub fn set_packetization_type(&mut self, value: H264PacketizationType) {
        unsafe {
            ffi::webrtc_RTPVideoHeaderH264_set_packetization_type(self.raw.as_ptr(), value.to_raw())
        };
    }

    pub fn nalus(&self) -> NaluInfoVector {
        let raw = unsafe { ffi::webrtc_RTPVideoHeaderH264_get_nalus(self.raw.as_ptr()) };
        let raw = NonNull::new(raw)
            .expect("BUG: webrtc_RTPVideoHeaderH264_get_nalus が null を返しました");
        // 借用ポインタを所有 vector に変換する (要素ごとにコピーする)。
        let mut vec = NaluInfoVector::new(0);
        let len = unsafe { ffi::webrtc_NaluInfo_vector_size(raw.as_ptr()) };
        for index in 0..len {
            let elem = unsafe { ffi::webrtc_NaluInfo_vector_get(raw.as_ptr(), index) };
            let elem = unsafe { NaluInfo::copy_from_raw(elem) };
            vec.push(&elem);
        }
        vec
    }

    pub fn set_nalus(&mut self, nalus: &NaluInfoVector) {
        unsafe { ffi::webrtc_RTPVideoHeaderH264_set_nalus(self.raw.as_ptr(), nalus.as_ptr()) };
    }

    pub fn packetization_mode(&self) -> H264PacketizationMode {
        let value =
            unsafe { ffi::webrtc_RTPVideoHeaderH264_get_packetization_mode(self.raw.as_ptr()) };
        H264PacketizationMode::from_raw(value)
    }

    pub fn set_packetization_mode(&mut self, value: H264PacketizationMode) {
        unsafe {
            ffi::webrtc_RTPVideoHeaderH264_set_packetization_mode(self.raw.as_ptr(), value.to_raw())
        };
    }
}

impl Default for RTPVideoHeaderH264 {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for RTPVideoHeaderH264 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nalus = self.nalus();
        f.debug_struct("RTPVideoHeaderH264")
            .field("nalu_type", &self.nalu_type())
            .field("packetization_type", &self.packetization_type())
            .field("nalus", &nalus)
            .field("packetization_mode", &self.packetization_mode())
            .finish()
    }
}

impl Drop for RTPVideoHeaderH264 {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_RTPVideoHeaderH264_delete(self.raw.as_ptr()) };
    }
}

/// コーデック固有の RTP ビデオヘッダー情報。
///
/// libwebrtc の `RTPVideoHeaderCodecSpecifics`
/// (std::variant<std::monostate, RTPVideoHeaderVP8, RTPVideoHeaderVP9,
/// RTPVideoHeaderH264>) に対応する。
#[derive(Debug)]
pub enum RTPVideoHeaderCodecSpecifics {
    /// コーデック固有情報なし (std::monostate)。
    None,
    VP8(RTPVideoHeaderVP8),
    VP9(RTPVideoHeaderVP9),
    H264(RTPVideoHeaderH264),
}
