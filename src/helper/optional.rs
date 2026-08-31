//! C API の optional (has / value) 方式の getter/setter を共通化するヘルパー。
//!
//! C API (`webrtc/src/webrtc_c/api/*.h`) は optional 値を
//! getter は `out_has` / `out_value`、setter は `has` + 値ポインタで表現する。
//! この定型的なボイラープレートを各ヘルパーに集約し、アクセサごとの
//! 繰り返しを無くす。

use std::os::raw::c_int;

/// has / value 方式の getter を呼び出し、`Option<T>` で返す。
///
/// `get_fn` には FFI の getter を呼び出すクロージャを渡す。クロージャは
/// `out_has` と `out_value` のポインタを受け取り、C API を呼び出す。
/// `has` が 0 なら `None`、そうでなければ `Some(value)` を返す。
///
/// `value` は `T::default()` で初期化してから渡す。C API は `has == 0` の
/// 場合は `out_value` を書き換えない。
pub(crate) fn get_optional<T: Default>(get_fn: impl FnOnce(*mut c_int, *mut T)) -> Option<T> {
    let mut has = 0;
    let mut value = T::default();
    get_fn(&mut has, &mut value);
    if has == 0 { None } else { Some(value) }
}

/// bool 型の has / value 方式 getter を呼び出し、`Option<bool>` で返す。
///
/// C API 側は bool を c_int の 1 / 0 で表現するため、取得結果を
/// `value != 0` で bool に変換する。
pub(crate) fn get_optional_bool(get_fn: impl FnOnce(*mut c_int, *mut c_int)) -> Option<bool> {
    get_optional(get_fn).map(|value| value != 0)
}

/// has / value 方式の setter を呼び出す。
///
/// `value` が `Some` なら `has = 1` で値へのポインタ、`None` なら `has = 0`
/// で null ポインタを渡す。C API は `has == 0` のとき値ポインタを読み取らない。
pub(crate) fn set_optional<T>(value: Option<T>, set_fn: impl FnOnce(c_int, *const T)) {
    match value {
        Some(v) => set_fn(1, &v),
        None => set_fn(0, std::ptr::null()),
    }
}

/// bool 型の has / value 方式 setter を呼び出す。
///
/// C API 側は bool を c_int の 1 / 0 で表現するため、`Some(true)` / `Some(false)`
/// をそれぞれ c_int の 1 / 0 に変換して渡す。`None` は `has = 0` で null を渡す。
pub(crate) fn set_optional_bool(value: Option<bool>, set_fn: impl FnOnce(c_int, *const c_int)) {
    match value {
        Some(true) => set_fn(1, &1),
        Some(false) => set_fn(1, &0),
        None => set_fn(0, std::ptr::null()),
    }
}

/// has / value 方式の getter を呼び出し、2 値の `Option<(A, B)>` で返す。
///
/// C API が `out_has` + 2 つの値 (`out_a` / `out_b`) を出力する getter のために、
/// [get_optional] の 2 値版。`has` が 0 なら `None`、そうでなければ `Some((a, b))` を返す。
pub(crate) fn get_optional2<A: Default, B: Default>(
    get_fn: impl FnOnce(*mut c_int, *mut A, *mut B),
) -> Option<(A, B)> {
    let mut has = 0;
    let mut a = A::default();
    let mut b = B::default();
    get_fn(&mut has, &mut a, &mut b);
    if has == 0 { None } else { Some((a, b)) }
}

/// has / value 方式の setter を呼び出す 2 値版。
///
/// `value` が `Some((a, b))` なら `has = 1` で 2 つの値へのポインタ、`None` なら
/// `has = 0` で null ポインタを渡す。C API は `has == 0` のとき値ポインタを読み取らない。
pub(crate) fn set_optional2<A, B>(
    value: Option<(A, B)>,
    set_fn: impl FnOnce(c_int, *const A, *const B),
) {
    match value {
        Some((a, b)) => set_fn(1, &a, &b),
        None => set_fn(0, std::ptr::null(), std::ptr::null()),
    }
}
