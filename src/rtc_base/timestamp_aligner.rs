use crate::ffi;
use std::ptr::NonNull;

/// webrtc::TimestampAligner の安全ラッパー。
pub struct TimestampAligner {
    raw_unique: NonNull<ffi::webrtc_TimestampAligner_unique>,
}

impl Default for TimestampAligner {
    fn default() -> Self {
        Self::new()
    }
}

impl TimestampAligner {
    /// 新しい TimestampAligner を生成する。
    pub fn new() -> Self {
        let raw = unsafe { ffi::webrtc_TimestampAligner_new() };
        Self {
            raw_unique: NonNull::new(raw)
                .expect("BUG: webrtc_TimestampAligner_new が null を返しました"),
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
        NonNull::new(raw).expect("BUG: webrtc_TimestampAligner_unique_get が null を返しました")
    }
}

impl Drop for TimestampAligner {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_TimestampAligner_unique_delete(self.raw_unique.as_ptr()) };
    }
}

unsafe impl Send for TimestampAligner {}
