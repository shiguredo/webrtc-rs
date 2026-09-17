use crate::const_non_null::ConstNonNull;
use crate::ffi;
use crate::helper::non_null::expect_non_null;
use std::marker::PhantomData;
use std::ptr::NonNull;

/// webrtc_c の webrtc_Environment を安全に扱うラッパー。
pub struct Environment {
    raw: NonNull<ffi::webrtc_Environment>,
}

unsafe impl Send for Environment {}

impl Environment {
    /// webrtc_Environment を生成する
    pub fn new() -> Self {
        let raw = unsafe { ffi::webrtc_CreateEnvironment() };
        // NULL が返されるのはメモリ不足の場合のみ
        Self {
            raw: expect_non_null(raw, "webrtc_CreateEnvironment"),
        }
    }

    /// 生ポインタを取得する。FFI 呼び出し用。
    pub fn as_ptr(&self) -> *mut ffi::webrtc_Environment {
        self.raw.as_ptr()
    }

    pub fn as_ref(&self) -> EnvironmentRef<'_> {
        // Safety: self.raw は Environment の生存中は常に有効です。
        EnvironmentRef::from_raw(ConstNonNull::from(self.raw))
    }

    pub fn as_mut(&mut self) -> EnvironmentRefMut<'_> {
        // Safety: self.raw は Environment の生存中は常に有効です。
        EnvironmentRefMut::from_raw(self.raw)
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Environment {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_Environment_delete(self.raw.as_ptr()) };
    }
}

#[derive(Clone, Copy)]
pub struct EnvironmentRef<'a> {
    raw: ConstNonNull<ffi::webrtc_Environment>,
    _marker: PhantomData<&'a ffi::webrtc_Environment>,
}

unsafe impl<'a> Send for EnvironmentRef<'a> {}

impl<'a> EnvironmentRef<'a> {
    pub(crate) fn from_raw(raw: ConstNonNull<ffi::webrtc_Environment>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub(crate) fn as_ptr(&self) -> *const ffi::webrtc_Environment {
        self.raw.as_ptr()
    }
}

/// webrtc_Environment の可変借用ラッパー。
pub struct EnvironmentRefMut<'a> {
    raw: NonNull<ffi::webrtc_Environment>,
    _marker: PhantomData<&'a mut ffi::webrtc_Environment>,
    cref: EnvironmentRef<'a>,
}

unsafe impl<'a> Send for EnvironmentRefMut<'a> {}

impl<'a> EnvironmentRefMut<'a> {
    pub(crate) fn from_raw(raw: NonNull<ffi::webrtc_Environment>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
            cref: EnvironmentRef::from_raw(ConstNonNull::from(raw)),
        }
    }

    pub fn as_mut_ptr(&self) -> *mut ffi::webrtc_Environment {
        self.raw.as_ptr()
    }
}

impl<'a> std::ops::Deref for EnvironmentRefMut<'a> {
    type Target = EnvironmentRef<'a>;

    fn deref(&self) -> &EnvironmentRef<'a> {
        &self.cref
    }
}
