//! コールバック型ハンドラの状態骨格と、登録・破棄の共通ヘルパー。

use crate::helper::non_null::expect_non_null_with_cleanup;
use std::os::raw::c_void;
use std::ptr::NonNull;

/// `Box<H>` を保持する、コールバック型ハンドラの状態の共通骨格。
pub(crate) struct HandlerState<H: ?Sized> {
    /// コールバックを実行するハンドラ。各 trampoline から直接利用される。
    pub(crate) handler: Box<H>,
}

impl<H: ?Sized> HandlerState<H> {
    /// ハンドラを保持した状態を生成する。
    pub(crate) fn new(handler: Box<H>) -> Self {
        Self { handler }
    }
}

/// コールバック状態の生ポインタを C の生成関数へ渡し、生成結果を `NonNull` にして返す。
///
/// `user_data` には `Box::into_raw` で変換した `Box<S>` の生ポインタを渡すこと。
/// `create` には `user_data` を受け取って生成結果の生ポインタを返す関数を渡すこと。
/// 生成結果が `null` の場合は `user_data` から `Box<S>` を回収してから panic するため、
/// リークも二重解放も起きない。
///
/// `what` はパニックメッセージで生成箇所を特定できるようにするためのラベル。
///
/// # Safety
///
/// `user_data` は `Box::into_raw` で変換した `Box<S>` の生ポインタであり有効なメモリを
/// 指していること。`S` はそれが指す実際の型と一致していること。
pub(crate) unsafe fn create_with_handler<S, T>(
    what: &'static str,
    user_data: *mut c_void,
    create: impl FnOnce(*mut c_void) -> *mut T,
) -> NonNull<T>
where
    S: 'static,
{
    expect_non_null_with_cleanup(create(user_data), what, || {
        let _ = unsafe { Box::from_raw(user_data as *mut S) };
    })
}

/// コールバック状態の `Box<S>` を回収する。
///
/// `user_data` には `Box::into_raw` で変換した `Box<S>` の生ポインタを渡すこと。
/// `user_data` が `null` の場合は規約違反の実行バグなので panic する。
///
/// `what` はパニックメッセージで回収箇所を特定できるようにするためのラベル。
///
/// # Safety
///
/// `user_data` は `Box::into_raw` で変換した `Box<S>` の生ポインタであり有効なメモリを
/// 指していること。`S` はそれが指す実際の型と一致していること。
pub(crate) unsafe fn destroy_handler<S>(what: &'static str, user_data: *mut c_void)
where
    S: 'static,
{
    assert!(!user_data.is_null(), "{what}: user_data is null");
    let _ = unsafe { Box::from_raw(user_data as *mut S) };
}
