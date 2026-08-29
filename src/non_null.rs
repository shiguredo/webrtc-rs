//! C API が返したポインタを null 検査して `NonNull` に包む共通ヘルパー。

use std::ptr::NonNull;

/// C API が返したポインタを null 検査して `NonNull` に包む。
///
/// null の場合、C API が規約違反を起こした実装バグなので panic する。
/// `what` には `ptr` を生成した関数名を渡すこと。
pub(crate) fn expect_non_null<T>(ptr: *mut T, what: &'static str) -> NonNull<T> {
    NonNull::new(ptr).unwrap_or_else(|| panic!("BUG: {what} が null を返しました"))
}

/// C API が返したポインタを null 検査して `NonNull` に包む。
///
/// null の場合、`cleanup` を実行してから panic する。生成に失敗したときに
/// 引数として渡したオブジェクトを回収しなければならない場合に使う。
/// `what` には `ptr` を生成した関数名を渡すこと。
pub(crate) fn expect_non_null_with_cleanup<T>(
    ptr: *mut T,
    what: &'static str,
    cleanup: impl FnOnce(),
) -> NonNull<T> {
    match NonNull::new(ptr) {
        Some(raw) => raw,
        None => {
            cleanup();
            panic!("BUG: {what} が null を返しました")
        }
    }
}
