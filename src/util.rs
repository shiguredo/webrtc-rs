//! クレート内外で使う汎用ユーティリティ。

use core::marker::PhantomData;
use core::ptr::NonNull;

/// 長さの不明なメモリ領域へ要素を順に書き込むためのライター。
///
/// C API などから渡される生ポインタの領域がどれだけの容量を持つか分からない、
/// あるいはその情報が隠された場合に、書き込む位置を自身で管理しながら
/// 要素を追記し、書き込んだ要素数を報告する。
///
/// 実際の容量は生成側（危険関数 `from_raw` を呼ぶ側）が保証するものとし、
/// 呼び出し側は `write` で渡す要素数が領域の容量を超えないようにしなければならない。
pub struct RawBufferWriter<'a, T> {
    current: NonNull<T>,
    written: usize,
    _marker: PhantomData<&'a mut T>,
}

unsafe impl<'a, T> Send for RawBufferWriter<'a, T> {}

impl<'a, T> RawBufferWriter<'a, T> {
    /// # Safety
    /// `current` は少なくとも以降に書き込む要素数を格納できる容量を持つ有効なメモリ領域を指す
    /// 必要があります。生成側が容量を保証しなければなりません。
    pub(crate) unsafe fn from_raw(current: NonNull<T>) -> Self {
        Self {
            current,
            written: 0,
            _marker: PhantomData,
        }
    }

    /// サンプル列を現在位置から追記する。
    ///
    /// # Safety
    /// `samples` を書き込んでも領域の容量を超えないことを呼び出し側が保証しなければならない。
    pub fn write(&mut self, samples: &[T]) {
        if samples.is_empty() {
            return;
        }
        let dst = unsafe { self.current.as_ptr().add(self.written) };
        unsafe { core::ptr::copy_nonoverlapping(samples.as_ptr(), dst, samples.len()) };
        self.written += samples.len();
    }

    /// これまでに書き込んだ要素数を返す。
    pub fn written_len(&self) -> usize {
        self.written
    }
}
