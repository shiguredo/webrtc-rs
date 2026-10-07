//! 非 null が保証された `*const T` を表す型。

use std::fmt;
use std::ptr::NonNull;

/// 非 null が保証された `*const T`。
///
/// std の `NonNull` は `*mut T` を扱う API しか持たないため、読み取り専用ポインタ用に用意する。
/// stable では `!null` を型として表現できないため `Option` の niche 最適化は効かないが、
/// null でないことが型で分かる。
#[repr(transparent)]
pub(crate) struct ConstNonNull<T: ?Sized> {
    pointer: *const T,
}

impl<T: ?Sized> ConstNonNull<T> {
    /// null の場合は `None` を返す。
    pub(crate) const fn new(pointer: *const T) -> Option<Self> {
        if pointer.is_null() {
            None
        } else {
            // null でないことを確認済み
            Some(Self { pointer })
        }
    }

    /// null 検査を行わずに生成する。
    ///
    /// # Safety
    /// `pointer` は null であってはならない。
    pub(crate) const unsafe fn new_unchecked(pointer: *const T) -> Self {
        Self { pointer }
    }

    /// 生ポインタを取り出す。
    pub(crate) const fn as_ptr(self) -> *const T {
        self.pointer
    }
}

impl<T: ?Sized> Clone for ConstNonNull<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> Copy for ConstNonNull<T> {}

impl<T: ?Sized> fmt::Debug for ConstNonNull<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.pointer, f)
    }
}

impl<T: ?Sized> From<NonNull<T>> for ConstNonNull<T> {
    fn from(pointer: NonNull<T>) -> Self {
        // SAFETY: `NonNull` は null でないことが保証されている
        unsafe { Self::new_unchecked(pointer.as_ptr()) }
    }
}
