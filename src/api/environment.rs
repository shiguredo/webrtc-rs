use crate::ffi;
use std::marker::PhantomData;
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
        self.as_ref().as_ptr()
    }

    pub fn as_ref(&self) -> EnvironmentRef<'_> {
        // Safety: self.raw は Environment の生存中は常に有効です。
        unsafe { EnvironmentRef::from_raw(self.raw) }
    }
}

impl Drop for Environment {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_Environment_delete(self.raw.as_ptr()) };
    }
}

#[allow(dead_code)]
pub struct EnvironmentRef<'a> {
    raw: NonNull<ffi::webrtc_Environment>,
    _marker: PhantomData<&'a ffi::webrtc_Environment>,
}

impl<'a> EnvironmentRef<'a> {
    /// # Safety
    /// `raw` は有効な `webrtc_Environment` を指している必要があります。
    pub unsafe fn from_raw(raw: NonNull<ffi::webrtc_Environment>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn as_ptr(&self) -> *mut ffi::webrtc_Environment {
        self.raw.as_ptr()
    }
}

unsafe impl<'a> Send for EnvironmentRef<'a> {}
