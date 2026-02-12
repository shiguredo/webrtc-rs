use crate::ffi;
use std::ptr::NonNull;

/// webrtc_c の webrtc_Environment を安全に扱うラッパー。
pub struct Environment {
    raw: NonNull<ffi::webrtc_Environment>,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    /// webrtc_Environment を生成する
    pub fn new() -> Self {
        let raw = unsafe { ffi::webrtc_CreateEnvironment() };
        // NULL が返されるのはメモリ不足の場合のみ
        Self {
            raw: NonNull::new(raw).expect("BUG: webrtc_CreateEnvironment が null を返しました"),
        }
    }

    /// 生ポインタを取得する。FFI 呼び出し用。
    pub fn as_ptr(&self) -> *mut ffi::webrtc_Environment {
        self.raw.as_ptr()
    }
}

impl Drop for Environment {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_Environment_delete(self.raw.as_ptr()) };
    }
}
