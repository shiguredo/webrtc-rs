use crate::ffi;

/// webrtc::MediaType のラッパー。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaType {
    Audio,
    Video,
    Unknown(i32),
}

impl MediaType {
    pub fn from_int(v: i32) -> Self {
        unsafe {
            if v == ffi::webrtc_MediaType_AUDIO {
                MediaType::Audio
            } else if v == ffi::webrtc_MediaType_VIDEO {
                MediaType::Video
            } else {
                MediaType::Unknown(v)
            }
        }
    }

    pub fn to_int(&self) -> i32 {
        match self {
            MediaType::Audio => unsafe { ffi::webrtc_MediaType_AUDIO },
            MediaType::Video => unsafe { ffi::webrtc_MediaType_VIDEO },
            MediaType::Unknown(v) => *v,
        }
    }
}
