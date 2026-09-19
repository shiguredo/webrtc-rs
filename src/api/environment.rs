use crate::const_non_null::ConstNonNull;
use crate::ffi;
use crate::helper::non_null::{expect_non_null, expect_non_null_const};
use crate::{Error, Result};
use std::marker::PhantomData;
use std::ptr::NonNull;

/// webrtc_c の webrtc_Environment を安全に扱うラッパー。
pub struct Environment {
    raw: NonNull<ffi::webrtc_Environment>,
}

unsafe impl Send for Environment {}

impl Environment {
    /// フィールドトライアルを指定しない Environment を生成する。
    ///
    /// フィールドトライアルを指定する場合は [EnvironmentFactory] を使う。
    pub fn new() -> Self {
        let raw = unsafe { ffi::webrtc_CreateEnvironment() };
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

    /// フィールドトライアルを参照する。
    pub fn field_trials(&self) -> FieldTrialsViewRef<'_> {
        self.as_ref().field_trials()
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Environment {
    fn clone(&self) -> Self {
        let raw = unsafe { ffi::webrtc_Environment_copy(self.raw.as_ptr()) };
        Self {
            raw: expect_non_null(raw, "webrtc_Environment_copy"),
        }
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

    /// フィールドトライアルを参照する。
    pub fn field_trials(&self) -> FieldTrialsViewRef<'a> {
        // Safety: 借用元の Environment が生きている間は、field_trials() が返す
        // FieldTrialsView も有効です。
        FieldTrialsViewRef::from_raw(expect_non_null_const(
            unsafe { ffi::webrtc_Environment_field_trials(self.raw.as_ptr()) },
            "webrtc_Environment_field_trials",
        ))
    }
}

/// webrtc::FieldTrialsView への借用ラッパー。
#[derive(Clone, Copy)]
pub struct FieldTrialsViewRef<'a> {
    raw: ConstNonNull<ffi::webrtc_FieldTrialsView>,
    _marker: PhantomData<&'a ffi::webrtc_FieldTrialsView>,
}

// FieldTrialsView は不変であり、複数スレッドから参照しても問題ない。
unsafe impl<'a> Send for FieldTrialsViewRef<'a> {}

impl<'a> FieldTrialsViewRef<'a> {
    pub(crate) fn from_raw(raw: ConstNonNull<ffi::webrtc_FieldTrialsView>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    /// フィールドトライアルが有効かどうかを返す。
    pub fn is_enabled(&self, key: &str) -> bool {
        unsafe {
            ffi::webrtc_FieldTrialsView_IsEnabled(self.raw.as_ptr(), key.as_ptr().cast(), key.len())
                != 0
        }
    }
}

/// webrtc_c の webrtc_FieldTrials を所有するラッパー。
pub struct FieldTrials {
    raw: NonNull<ffi::webrtc_FieldTrials_unique>,
}

unsafe impl Send for FieldTrials {}

impl FieldTrials {
    /// フィールドトライアル文字列をパースする。
    ///
    /// 文字列が不正な場合は [Error::InvalidFieldTrials] を返す。空文字は不正ではなく、
    /// フィールドトライアルを 1 つも含まない `FieldTrials` になる。
    pub fn new(field_trials: &str) -> Result<Self> {
        let raw = NonNull::new(unsafe {
            ffi::webrtc_FieldTrials_Create(field_trials.as_ptr().cast(), field_trials.len())
        })
        .ok_or_else(|| Error::InvalidFieldTrials(field_trials.to_string()))?;
        Ok(Self { raw })
    }

    /// C API に所有権を移すための生ポインタを取り出す。
    pub(crate) fn into_raw(self) -> *mut ffi::webrtc_FieldTrials_unique {
        std::mem::ManuallyDrop::new(self).raw.as_ptr()
    }
}

impl Drop for FieldTrials {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_FieldTrials_unique_delete(self.raw.as_ptr()) };
    }
}

/// webrtc_c の webrtc_EnvironmentFactory を所有するラッパー。
pub struct EnvironmentFactory {
    raw: NonNull<ffi::webrtc_EnvironmentFactory>,
}

unsafe impl Send for EnvironmentFactory {}

impl EnvironmentFactory {
    /// webrtc_EnvironmentFactory を生成する。
    pub fn new() -> Self {
        let raw = unsafe { ffi::webrtc_EnvironmentFactory_new() };
        Self {
            raw: expect_non_null(raw, "webrtc_EnvironmentFactory_new"),
        }
    }

    /// この EnvironmentFactory で生成する Environment にフィールドトライアルを設定する。
    ///
    /// 設定した `FieldTrials` の所有権は EnvironmentFactory に移り、[Self::create] で
    /// 生成した [Environment] が保持する。
    pub fn set_field_trials(&mut self, field_trials: FieldTrials) {
        unsafe {
            ffi::webrtc_EnvironmentFactory_Set_field_trials(
                self.raw.as_ptr(),
                field_trials.into_raw(),
            );
        }
    }

    /// 設定したフィールドトライアルを反映した Environment を生成する。
    ///
    /// フィールドトライアルは生成した Environment が保持するため、この
    /// EnvironmentFactory を drop した後も Environment 側で有効です。
    pub fn create(&self) -> Environment {
        let raw = unsafe { ffi::webrtc_EnvironmentFactory_Create(self.raw.as_ptr()) };
        Environment {
            raw: expect_non_null(raw, "webrtc_EnvironmentFactory_Create"),
        }
    }
}

impl Default for EnvironmentFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for EnvironmentFactory {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_EnvironmentFactory_delete(self.raw.as_ptr()) };
    }
}
