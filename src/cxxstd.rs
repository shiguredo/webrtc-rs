use crate::const_non_null::ConstNonNull;
use crate::ffi;
use crate::helper::non_null::{expect_non_null, expect_non_null_const};
use crate::{Error, Result};
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::slice;

/// std::string を安全に扱うラッパー。
pub struct CxxString {
    raw_unique: NonNull<ffi::std_string_unique>,
}

unsafe impl Send for CxxString {}

impl CxxString {
    /// 空文字列を生成する。
    pub fn new() -> Self {
        let raw = unsafe { ffi::std_string_new_empty() };
        Self {
            raw_unique: expect_non_null(raw, "std_string_new_empty"),
        }
    }

    /// &str から生成する。
    #[expect(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Self {
        let raw = unsafe { ffi::std_string_new_from_bytes(s.as_ptr() as *const _, s.len()) };
        Self {
            raw_unique: expect_non_null(raw, "std_string_new_from_bytes"),
        }
    }

    /// webrtc 側で生成されたユニークポインタを引き取る。
    pub(crate) fn from_unique(raw: NonNull<ffi::std_string_unique>) -> Self {
        Self { raw_unique: raw }
    }

    /// 長さを取得する。
    pub fn len(&self) -> usize {
        self.as_ref().len()
    }

    /// 空かどうかを返す。
    pub fn is_empty(&self) -> bool {
        self.as_ref().is_empty()
    }

    /// Rust の String として取得する。
    /// UTF-8 に変換できない場合はエラーを返す。
    pub fn to_string(&self) -> Result<String> {
        self.as_ref().to_string()
    }

    /// バイト列として取得する。
    pub fn to_bytes(&self) -> Vec<u8> {
        self.as_ref().to_bytes()
    }

    /// 末尾に追記する。
    /// s に null バイトが含まれていてもエラーにしない。
    pub fn append(&mut self, s: &str) {
        self.as_mut().append(s);
    }

    /// FFI に渡す生ポインタ。
    pub fn as_ptr(&self) -> *mut ffi::std_string {
        self.raw_string().as_ptr()
    }

    pub fn as_ref(&self) -> CxxStringRef<'_> {
        CxxStringRef::from_ptr(ConstNonNull::from(self.raw_string()))
    }

    pub fn as_mut(&mut self) -> CxxStringRefMut<'_> {
        CxxStringRefMut::from_raw(self.raw_string())
    }

    /// FFI へ所有権を移譲する。
    pub fn into_raw(self) -> *mut ffi::std_string_unique {
        std::mem::ManuallyDrop::new(self).raw_unique.as_ptr()
    }

    fn raw_string(&self) -> NonNull<ffi::std_string> {
        let raw = unsafe { ffi::std_string_unique_get(self.raw_unique.as_ptr()) };
        expect_non_null(raw, "std_string_unique_get")
    }
}

impl Default for CxxString {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for CxxString {
    fn drop(&mut self) {
        unsafe { ffi::std_string_unique_delete(self.raw_unique.as_ptr()) };
    }
}

#[derive(Clone, Copy)]
pub struct CxxStringRef<'a> {
    raw: ConstNonNull<ffi::std_string>,
    _marker: PhantomData<&'a ffi::std_string>,
}

unsafe impl<'a> Send for CxxStringRef<'a> {}

impl<'a> CxxStringRef<'a> {
    pub(crate) fn from_ptr(raw: ConstNonNull<ffi::std_string>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    /// 長さを取得する。
    pub fn len(&self) -> usize {
        unsafe { ffi::std_string_size(self.as_ptr()) }
    }

    /// 空かどうかを返す。
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Rust の String として取得する。
    /// UTF-8 に変換できない場合はエラーを返す。
    pub fn to_string(&self) -> Result<String> {
        let len = self.len();
        let ptr = unsafe { ffi::std_string_c_str(self.as_ptr()) }.cast::<u8>();
        assert!(!ptr.is_null(), "BUG: std_string_c_str が null を返しました");
        let bytes = unsafe { slice::from_raw_parts(ptr, len) };
        let s = std::str::from_utf8(bytes)?;
        Ok(s.to_owned())
    }

    /// バイト列として取得する。
    pub fn to_bytes(&self) -> Vec<u8> {
        let len = self.len();
        let ptr = unsafe { ffi::std_string_c_str(self.as_ptr()) }.cast::<u8>();
        unsafe { slice::from_raw_parts(ptr, len) }.to_vec()
    }

    /// FFI に渡す生ポインタ。
    pub fn as_ptr(&self) -> *const ffi::std_string {
        self.raw.as_ptr()
    }
}

/// std_string の可変借用ラッパー。
pub struct CxxStringRefMut<'a> {
    raw: NonNull<ffi::std_string>,
    _marker: PhantomData<&'a mut ffi::std_string>,
    cref: CxxStringRef<'a>,
}

unsafe impl<'a> Send for CxxStringRefMut<'a> {}

impl<'a> CxxStringRefMut<'a> {
    pub(crate) fn from_raw(raw: NonNull<ffi::std_string>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
            cref: CxxStringRef::from_ptr(ConstNonNull::from(raw)),
        }
    }

    pub fn as_mut_ptr(&self) -> *mut ffi::std_string {
        self.raw.as_ptr()
    }

    /// 末尾に追記する。
    /// s に null バイトが含まれていてもエラーにしない。
    pub fn append(&mut self, s: &str) {
        unsafe {
            ffi::std_string_append(self.as_mut_ptr(), s.as_ptr() as *const _, s.len());
        }
    }
    pub fn as_ref(&self) -> CxxStringRef<'_> {
        self.cref
    }

    pub fn len(&self) -> usize {
        self.cref.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cref.is_empty()
    }

    pub fn to_string(&self) -> Result<String> {
        self.cref.to_string()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.cref.to_bytes()
    }
}

/// `std::vector<std::string>` の安全ラッパー。
pub struct StringVector {
    raw: NonNull<ffi::std_string_vector>,
}

unsafe impl Send for StringVector {}

impl StringVector {
    /// size 要素で初期化したベクタを生成する。
    pub fn new(size: usize) -> Self {
        let raw = unsafe { ffi::std_string_vector_new(size) };
        Self {
            raw: expect_non_null(raw, "std_string_vector_new"),
        }
    }

    /// 要素数を取得する。
    pub fn len(&self) -> usize {
        self.as_ref().len()
    }

    /// 空かどうかを返す。
    pub fn is_empty(&self) -> bool {
        self.as_ref().is_empty()
    }

    /// 末尾に要素を追加する。
    pub fn push(&mut self, value: &CxxString) {
        self.as_mut().push(value);
    }

    pub fn as_ptr(&self) -> *mut ffi::std_string_vector {
        self.raw.as_ptr()
    }

    /// index の要素を Rust の String として取得する。
    pub fn get(&self, index: usize) -> Result<String> {
        self.as_ref().get(index)
    }

    /// FFI が返した所有権付きベクタを引き取る。
    pub(crate) fn from_raw(raw: NonNull<ffi::std_string_vector>) -> Self {
        Self { raw }
    }

    pub fn as_ref(&self) -> StringVectorRef<'_> {
        StringVectorRef::from_raw(ConstNonNull::from(self.raw))
    }

    pub fn as_mut(&mut self) -> StringVectorRefMut<'_> {
        StringVectorRefMut::from_raw(self.raw)
    }
}

impl Drop for StringVector {
    fn drop(&mut self) {
        unsafe { ffi::std_string_vector_delete(self.raw.as_ptr()) };
    }
}

/// `std::vector<std::string>` への借用ラッパー。
#[derive(Clone, Copy)]
pub struct StringVectorRef<'a> {
    raw: ConstNonNull<ffi::std_string_vector>,
    _marker: PhantomData<&'a ffi::std_string_vector>,
}

unsafe impl<'a> Send for StringVectorRef<'a> {}

impl<'a> StringVectorRef<'a> {
    pub(crate) fn from_raw(raw: ConstNonNull<ffi::std_string_vector>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        let len = unsafe { ffi::std_string_vector_size(self.raw.as_ptr()) };
        len.max(0) as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: usize) -> Result<String> {
        let len = self.len();
        if index >= len {
            return Err(Error::OutOfIndex(index));
        }
        let ptr = unsafe { ffi::std_string_vector_get_const(self.raw.as_ptr(), index as i32) };
        CxxStringRef::from_ptr(expect_non_null_const(ptr, "std_string_vector_get_const"))
            .to_string()
    }
}

/// `std::vector<std::string>` への可変借用ラッパー。
pub struct StringVectorRefMut<'a> {
    raw: NonNull<ffi::std_string_vector>,
    _marker: PhantomData<&'a mut ffi::std_string_vector>,
    cref: StringVectorRef<'a>,
}

unsafe impl<'a> Send for StringVectorRefMut<'a> {}

impl<'a> StringVectorRefMut<'a> {
    pub(crate) fn from_raw(raw: NonNull<ffi::std_string_vector>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
            cref: StringVectorRef::from_raw(ConstNonNull::from(raw)),
        }
    }

    pub fn as_mut_ptr(&self) -> *mut ffi::std_string_vector {
        self.raw.as_ptr()
    }

    pub fn push(&mut self, value: &CxxString) {
        unsafe { ffi::std_string_vector_push_back(self.raw.as_ptr(), value.as_ptr()) };
    }
    pub fn as_ref(&self) -> StringVectorRef<'_> {
        self.cref
    }

    pub fn len(&self) -> usize {
        self.cref.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cref.is_empty()
    }

    pub fn get(&self, index: usize) -> Result<String> {
        self.cref.get(index)
    }
}

/// std::map<std::string, std::string> の借用ラッパー。
#[derive(Clone, Copy)]
pub struct MapStringStringRef<'a> {
    raw: ConstNonNull<ffi::std_map_string_string>,
    _marker: PhantomData<&'a ffi::std_map_string_string>,
}

unsafe impl<'a> Send for MapStringStringRef<'a> {}

impl<'a> MapStringStringRef<'a> {
    /// C 側のポインタから生成する。
    pub(crate) fn from_raw(raw: ConstNonNull<ffi::std_map_string_string>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    /// FFI に渡す生ポインタ。
    pub(crate) fn as_ptr(&self) -> *const ffi::std_map_string_string {
        self.raw.as_ptr()
    }

    /// 要素数を取得する。
    pub fn len(&self) -> usize {
        let len = unsafe { ffi::std_map_string_string_size(self.raw.as_ptr()) };
        len.max(0) as usize
    }

    /// 空かどうかを返す。
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// イテレータを生成する。
    pub fn iter(&self) -> MapStringStringIter<'_> {
        let iter = unsafe { ffi::std_map_string_string_iter_new(self.raw.as_ptr()) };
        let raw_iter = expect_non_null(iter, "std_map_string_string_iter_new");
        MapStringStringIter {
            raw: raw_iter,
            _marker: PhantomData,
        }
    }
}

/// std::map<std::string, std::string> の可変借用ラッパー。
pub struct MapStringStringRefMut<'a> {
    raw: NonNull<ffi::std_map_string_string>,
    _marker: PhantomData<&'a mut ffi::std_map_string_string>,
    cref: MapStringStringRef<'a>,
}

unsafe impl<'a> Send for MapStringStringRefMut<'a> {}

impl<'a> MapStringStringRefMut<'a> {
    /// C 側のポインタから生成する。
    pub(crate) fn from_raw(raw: NonNull<ffi::std_map_string_string>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
            cref: MapStringStringRef::from_raw(ConstNonNull::from(raw)),
        }
    }

    /// FFI に渡す生ポインタ。
    pub(crate) fn as_mut_ptr(&self) -> *mut ffi::std_map_string_string {
        self.raw.as_ptr()
    }

    /// キーと値を設定する。
    /// null バイトを含んでいてもエラーにしない。
    pub fn set(&mut self, key: &str, value: &str) {
        unsafe {
            ffi::std_map_string_string_set(
                self.as_mut_ptr(),
                key.as_ptr() as *const _,
                key.len(),
                value.as_ptr() as *const _,
                value.len(),
            );
        }
    }
    pub fn as_ref(&self) -> MapStringStringRef<'_> {
        self.cref
    }

    pub fn len(&self) -> usize {
        self.cref.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cref.is_empty()
    }

    pub fn iter(&self) -> MapStringStringIter<'_> {
        self.cref.iter()
    }
}

/// MapStringStringRef のイテレータ。
pub struct MapStringStringIter<'a> {
    raw: NonNull<ffi::std_map_string_string_iter>,
    _marker: PhantomData<&'a ffi::std_map_string_string>,
}

unsafe impl<'a> Send for MapStringStringIter<'a> {}

impl<'a> Iterator for MapStringStringIter<'a> {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        let mut key = std::ptr::null_mut();
        let mut value = std::ptr::null_mut();
        let has_next = unsafe {
            ffi::std_map_string_string_iter_next(self.raw.as_ptr(), &mut key, &mut value)
        };
        if has_next == 0 {
            return None;
        }
        // key, value が null になることは無いはず
        let key = CxxString::from_unique(expect_non_null(key, "key"));
        let value = CxxString::from_unique(expect_non_null(value, "value"));
        // key, value が UTF-8 に変換できないことはあるはずなので、ちゃんと処理する
        Some((key.to_string().ok()?, value.to_string().ok()?))
    }
}

impl<'a> Drop for MapStringStringIter<'a> {
    fn drop(&mut self) {
        unsafe { ffi::std_map_string_string_iter_delete(self.raw.as_ptr()) };
    }
}
