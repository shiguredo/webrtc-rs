use crate::ffi;
use crate::helper::non_null::expect_non_null;
use std::ptr::NonNull;

/// webrtc::RtcEventLogFactory のラッパー。
pub struct RtcEventLogFactory {
    raw_unique: NonNull<ffi::webrtc_RtcEventLogFactory_unique>,
}

unsafe impl Send for RtcEventLogFactory {}

impl RtcEventLogFactory {
    pub fn new() -> Self {
        let raw = expect_non_null(
            unsafe { ffi::webrtc_RtcEventLogFactory_Create() },
            "webrtc_RtcEventLogFactory_Create",
        );
        Self { raw_unique: raw }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_RtcEventLogFactory {
        unsafe { ffi::webrtc_RtcEventLogFactory_unique_get(self.raw_unique.as_ptr()) }
    }

    pub fn into_raw(self) -> *mut ffi::webrtc_RtcEventLogFactory_unique {
        std::mem::ManuallyDrop::new(self).raw_unique.as_ptr()
    }
}

impl Default for RtcEventLogFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for RtcEventLogFactory {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_RtcEventLogFactory_unique_delete(self.raw_unique.as_ptr()) };
    }
}
