use crate::ffi;
use crate::helper::non_null::expect_non_null;
use std::ptr::NonNull;

/// webrtc::TimestampAligner の安全ラッパー。
pub struct TimestampAligner {
    raw_unique: NonNull<ffi::webrtc_TimestampAligner_unique>,
}

unsafe impl Send for TimestampAligner {}

impl TimestampAligner {
    /// 新しい TimestampAligner を生成する。
    pub fn new() -> Self {
        let raw = unsafe { ffi::webrtc_TimestampAligner_new() };
        Self {
            raw_unique: expect_non_null(raw, "webrtc_TimestampAligner_new"),
        }
    }

    /// タイムスタンプを調整する。
    pub fn translate(&mut self, timestamp_us: i64, now_us: i64) -> i64 {
        let raw = self.raw();
        unsafe {
            ffi::webrtc_TimestampAligner_TranslateTimestamp(raw.as_ptr(), timestamp_us, now_us)
        }
    }

    fn raw(&self) -> NonNull<ffi::webrtc_TimestampAligner> {
        let raw = unsafe { ffi::webrtc_TimestampAligner_unique_get(self.raw_unique.as_ptr()) };
        expect_non_null(raw, "webrtc_TimestampAligner_unique_get")
    }
}

impl Default for TimestampAligner {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for TimestampAligner {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_TimestampAligner_unique_delete(self.raw_unique.as_ptr()) };
    }
}
