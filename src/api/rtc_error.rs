use crate::{Result, ffi};
use std::ptr::NonNull;
use std::slice;

/// webrtc::RTCError のラッパー。
#[derive(Debug)]
pub struct RtcError {
    raw_unique: NonNull<ffi::webrtc_RTCError_unique>,
}

impl RtcError {
    // 生ポインタから生成する。null ポインタの場合はエラーを返す。
    pub fn from_unique_ptr(raw: NonNull<ffi::webrtc_RTCError_unique>) -> Self {
        Self { raw_unique: raw }
    }

    /// 成功かどうか。
    pub fn ok(&self) -> bool {
        let raw = self.raw();
        unsafe { ffi::webrtc_RTCError_ok(raw.as_ptr()) != 0 }
    }

    /// エラーメッセージを取得する。
    pub fn message(&self) -> Result<String> {
        let raw = self.raw();
        let mut ptr = std::ptr::null();
        let mut len = 0usize;
        unsafe { ffi::webrtc_RTCError_message(raw.as_ptr(), &mut ptr, &mut len) };
        assert!(!ptr.is_null());
        let bytes = unsafe { slice::from_raw_parts(ptr as *const u8, len) };
        let msg = std::str::from_utf8(bytes)?;
        Ok(msg.to_owned())
    }

    fn raw(&self) -> NonNull<ffi::webrtc_RTCError> {
        let raw = unsafe { ffi::webrtc_RTCError_unique_get(self.raw_unique.as_ptr()) };
        NonNull::new(raw).expect("BUG: webrtc_RTCError_unique_get が null を返しました")
    }
}

impl Drop for RtcError {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_RTCError_unique_delete(self.raw_unique.as_ptr()) };
    }
}

unsafe impl Send for RtcError {}
