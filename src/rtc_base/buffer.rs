//! webrtc::Buffer (rtc::Buffer) の Rust ラッパー。
use crate::ffi;
use crate::helper::non_null::expect_non_null;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::slice;

/// `webrtc::Buffer` への借用ラッパー。
pub struct BufferRef<'a> {
    raw: NonNull<ffi::webrtc_Buffer>,
    _marker: PhantomData<&'a ffi::webrtc_Buffer>,
}

unsafe impl<'a> Send for BufferRef<'a> {}

impl<'a> BufferRef<'a> {
    /// # Safety
    /// `raw` は有効な `webrtc_Buffer` を指している必要があります。
    pub(crate) unsafe fn from_raw(raw: NonNull<ffi::webrtc_Buffer>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    /// バッファを空にする。
    pub fn clear(&mut self) {
        unsafe { ffi::webrtc_Buffer_Clear(self.raw.as_ptr()) }
    }

    /// バイト列を末尾へ追記する。
    pub fn append_data(&mut self, data: &[u8]) {
        unsafe { ffi::webrtc_Buffer_AppendData(self.raw.as_ptr(), data.as_ptr(), data.len()) }
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
        let ptr = unsafe { ffi::webrtc_Buffer_data(self.raw.as_ptr()) };
        assert!(!ptr.is_null(), "webrtc_Buffer_data が null を返しました");
        unsafe { slice::from_raw_parts(ptr, size) }
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
        unsafe { BufferRef::from_raw(self.raw) }
    }

    /// バッファを空にする。
    pub fn clear(&mut self) {
        self.as_ref().clear()
    }

    /// バイト列を末尾へ追記する。
    pub fn append_data(&mut self, data: &[u8]) {
        self.as_ref().append_data(data)
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
        let ptr = unsafe { ffi::webrtc_Buffer_data(self.raw.as_ptr()) };
        assert!(!ptr.is_null(), "webrtc_Buffer_data が null を返しました");
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
pub struct BufferS16Ref<'a> {
    raw: NonNull<ffi::webrtc_BufferS16>,
    _marker: PhantomData<&'a mut ffi::webrtc_BufferS16>,
}

unsafe impl<'a> Send for BufferS16Ref<'a> {}

impl<'a> BufferS16Ref<'a> {
    /// # Safety
    /// `raw` は有効な `webrtc_BufferS16` を指す必要があります。
    pub(crate) unsafe fn from_raw(raw: NonNull<ffi::webrtc_BufferS16>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    /// サンプル列を末尾へ追記する。
    pub fn append_data(&mut self, samples: &[i16]) {
        unsafe {
            ffi::webrtc_BufferS16_AppendData(self.raw.as_ptr(), samples.as_ptr(), samples.len())
        }
    }

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
        let ptr = unsafe { ffi::webrtc_BufferS16_data(self.raw.as_ptr()) };
        assert!(!ptr.is_null(), "webrtc_BufferS16_data が null を返しました");
        unsafe { slice::from_raw_parts(ptr, size) }
    }

    /// バッファを空にする。
    pub fn clear(&mut self) {
        unsafe { ffi::webrtc_BufferS16_Clear(self.raw.as_ptr()) }
    }

    pub(crate) fn raw(&self) -> *mut ffi::webrtc_BufferS16 {
        self.raw.as_ptr()
    }
}
