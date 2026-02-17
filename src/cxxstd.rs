use crate::ffi;
use crate::{Error, Result};
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::slice;

/// std::string を安全に扱うラッパー。
pub struct CxxString {
    raw_unique: NonNull<ffi::std_string_unique>,
}

impl Default for CxxString {
    fn default() -> Self {
        Self::new()
    }
}

impl CxxString {
    /// 空文字列を生成する。
    pub fn new() -> Self {
        let raw = unsafe { ffi::std_string_new_empty() };
        Self {
            raw_unique: NonNull::new(raw).expect("BUG: std_string_new_empty が null を返しました"),
        }
    }

    /// &str から生成する。
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Self {
        let raw = unsafe { ffi::std_string_new_from_bytes(s.as_ptr() as *const _, s.len()) };
        Self {
            raw_unique: NonNull::new(raw)
                .expect("BUG: std_string_new_from_bytes が null を返しました"),
        }
    }

    /// webrtc 側で生成されたユニークポインタを引き取る。
    pub fn from_unique(raw: NonNull<ffi::std_string_unique>) -> Self {
        Self { raw_unique: raw }
    }

    /// 長さを取得する。
    pub fn len(&self) -> usize {
        let raw = self.raw_string();
        let len = unsafe { ffi::std_string_size(raw.as_ptr()) };
        len.max(0) as usize
    }

    /// 空かどうかを返す。
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Rust の String として取得する。
    /// UTF-8 に変換できない場合はエラーを返す。
    pub fn to_string(&self) -> Result<String> {
        let raw = self.raw_string();
        let len = self.len();
        let ptr = unsafe { ffi::std_string_c_str(raw.as_ptr()) }.cast::<u8>();
        assert!(!ptr.is_null(), "BUG: std_string_c_str が null を返しました");
        let bytes = unsafe { slice::from_raw_parts(ptr, len) };
        let s = std::str::from_utf8(bytes)?;
        Ok(s.to_owned())
    }

    /// バイト列として取得する。
    pub fn to_bytes(&self) -> Vec<u8> {
        let raw = self.raw_string();
        let len = self.len();
        let ptr = unsafe { ffi::std_string_c_str(raw.as_ptr()) }.cast::<u8>();
        unsafe { slice::from_raw_parts(ptr, len) }.to_vec()
    }

    /// 末尾に追記する。
    /// s に null バイトが含まれていてもエラーにしない。
    pub fn append(&mut self, s: &str) {
        let raw = self.raw_string();
        unsafe {
            ffi::std_string_append(raw.as_ptr(), s.as_ptr() as *const _, s.len());
        }
    }

    /// FFI に渡す生ポインタ。
    pub fn as_ptr(&self) -> *mut ffi::std_string {
        self.raw_string().as_ptr()
    }

    fn raw_string(&self) -> NonNull<ffi::std_string> {
        let raw = unsafe { ffi::std_string_unique_get(self.raw_unique.as_ptr()) };
        NonNull::new(raw).expect("BUG: std_string_unique_get が null を返しました")
    }
}

impl Drop for CxxString {
    fn drop(&mut self) {
        unsafe { ffi::std_string_unique_delete(self.raw_unique.as_ptr()) };
    }
}

pub struct CxxStringRef<'a> {
    raw: NonNull<ffi::std_string>,
    _marker: PhantomData<&'a ffi::std_string>,
}

impl<'a> CxxStringRef<'a> {
    pub fn from_ptr(raw: NonNull<ffi::std_string>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    /// 長さを取得する。
    pub fn len(&self) -> usize {
        let len = unsafe { ffi::std_string_size(self.as_ptr()) };
        len.max(0) as usize
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
    pub fn as_ptr(&self) -> *mut ffi::std_string {
        self.raw.as_ptr()
    }
}

/// std::vector<std::string> の安全ラッパー。
pub struct StringVector {
    raw: NonNull<ffi::std_string_vector>,
}

impl StringVector {
    /// size 要素で初期化したベクタを生成する。
    pub fn new(size: i32) -> Self {
        let raw = unsafe { ffi::std_string_vector_new(size) };
        Self {
            raw: NonNull::new(raw).expect("BUG: std_string_vector_new が null を返しました"),
        }
    }

    /// 要素数を取得する。
    pub fn len(&self) -> usize {
        let len = unsafe { ffi::std_string_vector_size(self.raw.as_ptr()) };
        len.max(0) as usize
    }

    /// 空かどうかを返す。
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 末尾に要素を追加する。
    pub fn push(&mut self, value: &CxxString) {
        unsafe { ffi::std_string_vector_push_back(self.raw.as_ptr(), value.as_ptr()) };
    }

    pub fn as_ptr(&self) -> *mut ffi::std_string_vector {
        self.raw.as_ptr()
    }

    /// index の要素を Rust の String として取得する。
    pub fn get(&self, index: usize) -> Result<String> {
        let len = self.len();
        if index >= len {
            return Err(Error::OutOfIndex(index));
        }
        let ptr = unsafe { ffi::std_string_vector_get(self.raw.as_ptr(), index as i32) };
        CxxStringRef::from_ptr(
            NonNull::new(ptr).expect("BUG: std_string_vector_get が null を返しました"),
        )
        .to_string()
    }
}

impl Drop for StringVector {
    fn drop(&mut self) {
        unsafe { ffi::std_string_vector_delete(self.raw.as_ptr()) };
    }
}

/// std::vector<std::string> への借用ラッパー。
pub struct StringVectorRef<'a> {
    raw: NonNull<ffi::std_string_vector>,
    _marker: PhantomData<&'a ()>,
}

impl<'a> StringVectorRef<'a> {
    pub fn from_raw(raw: NonNull<ffi::std_string_vector>) -> Self {
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

    pub fn push(&mut self, value: &CxxString) {
        unsafe { ffi::std_string_vector_push_back(self.raw.as_ptr(), value.as_ptr()) };
    }
}

/// std::map<std::string, std::string> の借用ラッパー。
pub struct MapStringString<'a> {
    raw: NonNull<ffi::std_map_string_string>,
    _marker: PhantomData<&'a mut ffi::std_map_string_string>,
}

impl<'a> MapStringString<'a> {
    /// C 側のポインタから生成する。
    pub fn from_raw(raw: NonNull<ffi::std_map_string_string>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
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

    /// キーと値を設定する。
    /// null バイトを含んでいてもエラーにしない。
    pub fn set(&mut self, key: &str, value: &str) {
        unsafe {
            ffi::std_map_string_string_set(
                self.raw.as_ptr(),
                key.as_ptr() as *const _,
                key.len(),
                value.as_ptr() as *const _,
                value.len(),
            );
        }
    }

    /// イテレータを生成する。
    pub fn iter(&self) -> MapStringStringIter<'_> {
        let iter = unsafe { ffi::std_map_string_string_iter_new(self.raw.as_ptr()) };
        let raw_iter =
            NonNull::new(iter).expect("BUG: std_map_string_string_iter_new が null を返しました");
        MapStringStringIter {
            raw: raw_iter,
            _marker: PhantomData,
        }
    }
}

/// MapStringString のイテレータ。
pub struct MapStringStringIter<'a> {
    raw: NonNull<ffi::std_map_string_string_iter>,
    _marker: PhantomData<&'a ffi::std_map_string_string>,
}

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
        let key = CxxString::from_unique(NonNull::new(key).expect("BUG: key が null を返しました"));
        let value =
            CxxString::from_unique(NonNull::new(value).expect("BUG: value が null を返しました"));
        // key, value が UTF-8 に変換できないことはあるはずなので、ちゃんと処理する
        Some((key.to_string().ok()?, value.to_string().ok()?))
    }
}

impl<'a> Drop for MapStringStringIter<'a> {
    fn drop(&mut self) {
        unsafe { ffi::std_map_string_string_iter_delete(self.raw.as_ptr()) };
    }
}
