//! webrtc::Buffer (rtc::Buffer) の Rust ラッパー。
use crate::const_non_null::ConstNonNull;
use crate::ffi;
use crate::helper::non_null::expect_non_null;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::slice;

/// `webrtc::Buffer` への借用ラッパー。
#[derive(Clone, Copy)]
pub struct BufferRef<'a> {
    raw: ConstNonNull<ffi::webrtc_Buffer>,
    _marker: PhantomData<&'a ffi::webrtc_Buffer>,
}

unsafe impl<'a> Send for BufferRef<'a> {}

impl<'a> BufferRef<'a> {
    pub(crate) fn from_raw(raw: ConstNonNull<ffi::webrtc_Buffer>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    /// バッファサイズを返す。
    pub fn size(&self) -> usize {
        unsafe { ffi::webrtc_Buffer_size(self.raw.as_ptr()) }
    }

    /// バッファ内容を返す。
    pub fn data(&self) -> &[u8] {
        let size = self.size();
        if size == 0 {
            return &[];
        }
        let ptr = unsafe { ffi::webrtc_Buffer_data_const(self.raw.as_ptr()) };
        assert!(
            !ptr.is_null(),
            "BUG: webrtc_Buffer_data_const returned null"
        );
        unsafe { slice::from_raw_parts(ptr, size) }
    }
}

/// `webrtc::Buffer` への可変借用ラッパー。
pub struct BufferRefMut<'a> {
    raw: NonNull<ffi::webrtc_Buffer>,
    _marker: PhantomData<&'a mut ffi::webrtc_Buffer>,
    cref: BufferRef<'a>,
}

unsafe impl<'a> Send for BufferRefMut<'a> {}

impl<'a> BufferRefMut<'a> {
    pub(crate) fn from_raw(raw: NonNull<ffi::webrtc_Buffer>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
            cref: BufferRef::from_raw(ConstNonNull::from(raw)),
        }
    }

    pub fn as_mut_ptr(&self) -> *mut ffi::webrtc_Buffer {
        self.raw.as_ptr()
    }

    /// バッファを空にする。
    pub fn clear(&mut self) {
        unsafe { ffi::webrtc_Buffer_Clear(self.raw.as_ptr()) }
    }

    /// バイト列を末尾へ追記する。
    pub fn append_data(&mut self, data: &[u8]) {
        unsafe { ffi::webrtc_Buffer_AppendData(self.raw.as_ptr(), data.as_ptr(), data.len()) }
    }
    pub fn as_ref(&self) -> BufferRef<'_> {
        self.cref
    }

    pub fn size(&self) -> usize {
        self.cref.size()
    }

    pub fn data(&self) -> &[u8] {
        self.cref.data()
    }

    /// バッファ内容を書き換え用に返す。
    pub fn data_mut(&mut self) -> &mut [u8] {
        let size = self.size();
        if size == 0 {
            return &mut [];
        }
        let ptr = unsafe { ffi::webrtc_Buffer_data(self.raw.as_ptr()) };
        assert!(!ptr.is_null(), "BUG: webrtc_Buffer_data returned null");
        unsafe { slice::from_raw_parts_mut(ptr, size) }
    }
}

/// `webrtc::Buffer` の所有ラッパー。
pub struct Buffer {
    raw: NonNull<ffi::webrtc_Buffer>,
}

unsafe impl Send for Buffer {}

impl Buffer {
    /// 新しい空のバッファを生成する。
    pub fn new() -> Self {
        let raw = unsafe { ffi::webrtc_Buffer_new() };
        Self {
            raw: expect_non_null(raw, "webrtc_Buffer_new"),
        }
    }

    /// [BufferRef] として借用する。
    pub fn as_ref(&self) -> BufferRef<'_> {
        // Safety: self.raw は Buffer の生存中は常に有効です。
        BufferRef::from_raw(ConstNonNull::from(self.raw))
    }

    pub fn as_mut(&mut self) -> BufferRefMut<'_> {
        // Safety: self.raw は Buffer の生存中は常に有効です。
        BufferRefMut::from_raw(self.raw)
    }

    /// バッファを空にする。
    pub fn clear(&mut self) {
        self.as_mut().clear()
    }

    /// バイト列を末尾へ追記する。
    pub fn append_data(&mut self, data: &[u8]) {
        self.as_mut().append_data(data)
    }

    /// バッファサイズを返す。
    pub fn size(&self) -> usize {
        self.as_ref().size()
    }

    /// バッファ内容を返す。
    pub fn data(&self) -> &[u8] {
        let size = self.size();
        if size == 0 {
            return &[];
        }
        let ptr = unsafe { ffi::webrtc_Buffer_data_const(self.raw.as_ptr()) };
        assert!(
            !ptr.is_null(),
            "BUG: webrtc_Buffer_data_const returned null"
        );
        unsafe { slice::from_raw_parts(ptr, size) }
    }

    pub(crate) fn as_ptr(&self) -> *mut ffi::webrtc_Buffer {
        self.raw.as_ptr()
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_Buffer_delete(self.raw.as_ptr()) };
    }
}

/// `webrtc::BufferT<int16_t>` (int16 サンプルバッファ) への借用ラッパー。
#[derive(Clone, Copy)]
pub struct BufferS16Ref<'a> {
    raw: ConstNonNull<ffi::webrtc_BufferS16>,
    _marker: PhantomData<&'a ffi::webrtc_BufferS16>,
}

unsafe impl<'a> Send for BufferS16Ref<'a> {}

impl<'a> BufferS16Ref<'a> {
    /// サンプル数を返す。
    pub fn size(&self) -> usize {
        unsafe { ffi::webrtc_BufferS16_size(self.raw.as_ptr()) }
    }

    /// バッファ内容を返す。
    pub fn data(&self) -> &[i16] {
        let size = self.size();
        if size == 0 {
            return &[];
        }
        let ptr = unsafe { ffi::webrtc_BufferS16_data_const(self.raw.as_ptr()) };
        assert!(
            !ptr.is_null(),
            "BUG: webrtc_BufferS16_data_const returned null"
        );
        unsafe { slice::from_raw_parts(ptr, size) }
    }
}

/// `webrtc::BufferT<int16_t>` (int16 サンプルバッファ) への可変借用ラッパー。
pub struct BufferS16RefMut<'a> {
    raw: NonNull<ffi::webrtc_BufferS16>,
    _marker: PhantomData<&'a mut ffi::webrtc_BufferS16>,
    cref: BufferS16Ref<'a>,
}

unsafe impl<'a> Send for BufferS16RefMut<'a> {}

impl<'a> BufferS16RefMut<'a> {
    pub(crate) fn from_raw(raw: NonNull<ffi::webrtc_BufferS16>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
            cref: BufferS16Ref {
                raw: ConstNonNull::from(raw),
                _marker: PhantomData,
            },
        }
    }

    pub fn as_mut_ptr(&self) -> *mut ffi::webrtc_BufferS16 {
        self.raw.as_ptr()
    }

    /// サンプル列を末尾へ追記する。
    pub fn append_data(&mut self, samples: &[i16]) {
        unsafe {
            ffi::webrtc_BufferS16_AppendData(self.raw.as_ptr(), samples.as_ptr(), samples.len())
        }
    }

    /// バッファを空にする。
    pub fn clear(&mut self) {
        unsafe { ffi::webrtc_BufferS16_Clear(self.raw.as_ptr()) }
    }
    pub fn as_ref(&self) -> BufferS16Ref<'_> {
        self.cref
    }

    pub fn size(&self) -> usize {
        self.cref.size()
    }

    pub fn data(&self) -> &[i16] {
        self.cref.data()
    }

    /// バッファ内容を書き換え用に返す。
    pub fn data_mut(&mut self) -> &mut [i16] {
        let size = self.size();
        if size == 0 {
            return &mut [];
        }
        let ptr = unsafe { ffi::webrtc_BufferS16_data(self.raw.as_ptr()) };
        assert!(!ptr.is_null(), "BUG: webrtc_BufferS16_data returned null");
        unsafe { slice::from_raw_parts_mut(ptr, size) }
    }
}
