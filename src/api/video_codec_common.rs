use crate::ffi;
use std::marker::PhantomData;
use std::ptr::NonNull;

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
