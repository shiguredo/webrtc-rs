use crate::ffi;

/// webrtc_TimeMillis の安全ラッパー。
pub fn time_millis() -> i64 {
    unsafe { ffi::webrtc_TimeMillis() }
}
