//! C API の optional (has / value) 方式の getter/setter を共通化するヘルパー。
//!
//! C API (`webrtc/src/webrtc_c/api/*.h`) は optional 値を getter では
//! `out_has` + `out_value`、setter では `has` + 値で表現する。setter は
//! `has` が 0 のとき値を読まないため、値がない場合は null を渡す。
//!
//! 使う C API のシグネチャに合うヘルパーを選ぶこと。対応するシグネチャは各ヘルパーの
//! ドキュメントに書いてある。
//!
//! - スカラー: [get_optional_scalar] / [set_optional_scalar]
//! - スカラー 2 値: [get_optional_scalar2] / [set_optional_scalar2]
//! - bool (c_int の 1 / 0): [get_optional_bool] / [set_optional_bool]
//! - C オブジェクト: [get_optional_object] / [set_optional_object]
//! - 生ポインタ: [get_optional_ptr] / [set_optional_ptr]
//! - 生ポインタ (const): [get_optional_ptr_const]
//! - ポインタ + 長さ: [get_optional_slice] / [set_optional_slice]

use crate::const_non_null::ConstNonNull;
use crate::helper::non_null::{expect_non_null, expect_non_null_const};
use std::os::raw::c_int;
use std::ptr::NonNull;

/// has / value 方式 getter の共通処理。
///
/// `get_fn` に `has` と `out_value` を渡し、`has` が 1 なら `out_value` を返す。
fn get_optional<T>(out_value: T, get_fn: impl FnOnce(*mut c_int, &mut T)) -> Option<T> {
    let mut out_value = out_value;
    let mut has = 0;
    get_fn(&mut has, &mut out_value);
    if has == 0 { None } else { Some(out_value) }
}

/// has / value 方式 setter の共通処理。
///
/// `value` が `Some` なら `has = 1` と `as_ptr` の結果、`None` なら `has = 0` と
/// `null_ptr` を `set_fn` に渡す。
fn set_optional<T, P>(
    value: Option<T>,
    as_ptr: impl FnOnce(&T) -> P,
    null_ptr: P,
    set_fn: impl FnOnce(c_int, P),
) {
    match value {
        Some(v) => set_fn(1, as_ptr(&v)),
        None => set_fn(0, null_ptr),
    }
}

/// スカラーの getter。
///
/// C API のシグネチャが `void get(int* out_has, T* out_value)` のときに使う。
pub(crate) fn get_optional_scalar<T: Default>(
    get_fn: impl FnOnce(*mut c_int, *mut T),
) -> Option<T> {
    get_optional(T::default(), |has, out_value| get_fn(has, out_value))
}

/// スカラーの setter。
///
/// C API のシグネチャが `void set(int has, const T* value)` のときに使う。
pub(crate) fn set_optional_scalar<T>(value: Option<T>, set_fn: impl FnOnce(c_int, *const T)) {
    set_optional(value, std::ptr::from_ref, std::ptr::null(), set_fn)
}

/// bool の getter。
///
/// C API のシグネチャが `void get(int* out_has, int* out_value)` のときに使う。
/// c_int の 1 / 0 を bool に変換する。
pub(crate) fn get_optional_bool(get_fn: impl FnOnce(*mut c_int, *mut c_int)) -> Option<bool> {
    get_optional(0, |has, out_value| get_fn(has, out_value)).map(|value| value != 0)
}

/// bool の setter。
///
/// C API のシグネチャが `void set(int has, const int* value)` のときに使う。
/// bool を c_int の 1 / 0 に変換して渡す。
pub(crate) fn set_optional_bool(value: Option<bool>, set_fn: impl FnOnce(c_int, *const c_int)) {
    set_optional(
        value,
        |v| if *v { &1 } else { &0 },
        std::ptr::null(),
        set_fn,
    )
}

/// 2 つのスカラーの getter。
///
/// C API のシグネチャが `void get(int* out_has, A* out_a, B* out_b)` のときに使う。
pub(crate) fn get_optional_scalar2<A: Default, B: Default>(
    get_fn: impl FnOnce(*mut c_int, *mut A, *mut B),
) -> Option<(A, B)> {
    get_optional((A::default(), B::default()), |has, out_value| {
        let (a, b) = out_value;
        get_fn(has, a, b)
    })
}

/// 2 つのスカラーの setter。
///
/// C API のシグネチャが `void set(int has, const A* a, const B* b)` のときに使う。
pub(crate) fn set_optional_scalar2<A, B>(
    value: Option<(A, B)>,
    set_fn: impl FnOnce(c_int, *const A, *const B),
) {
    set_optional(
        value,
        |(a, b)| (std::ptr::from_ref(a), std::ptr::from_ref(b)),
        (std::ptr::null(), std::ptr::null()),
        |has, (a, b)| set_fn(has, a, b),
    )
}

/// C オブジェクトの getter。
///
/// C API のシグネチャが `void get(int* out_has, U* out_value)` のときに使う。
pub(crate) fn get_optional_object<T, U>(
    value: T,
    as_ptr: impl FnOnce(&T) -> *mut U,
    get_fn: impl FnOnce(*mut c_int, *mut U),
) -> Option<T> {
    get_optional(value, |has, value| get_fn(has, as_ptr(value)))
}

/// C オブジェクトの setter。
///
/// C API のシグネチャが `void set(int has, const U* value)` のときに使う。
/// `as_ptr` が返すポインタを渡す。
pub(crate) fn set_optional_object<T, U>(
    value: Option<T>,
    as_ptr: impl FnOnce(&T) -> *mut U,
    set_fn: impl FnOnce(c_int, *const U),
) {
    set_optional(value, |v| as_ptr(v).cast_const(), std::ptr::null(), set_fn)
}

/// 生ポインタが出力の getter。
///
/// C API のシグネチャが `void get(int* out_has, U** out_value)` のときに使う。
/// `out_has` が 1 なのに `out_value` が null の場合は panic する (`what` には関数名を渡す)。
/// 返すポインタの所有 / 借用は C API の契約に従う。
pub(crate) fn get_optional_ptr<U>(
    what: &'static str,
    get_fn: impl FnOnce(*mut c_int, *mut *mut U),
) -> Option<NonNull<U>> {
    get_optional(std::ptr::null_mut::<U>(), |has, out_value| {
        get_fn(has, out_value)
    })
    .map(|raw| expect_non_null(raw, what))
}

/// 生ポインタを渡す setter。
///
/// C API のシグネチャが `void set(int has, const U* value)` のときに使う。Rust 側の値が
/// `NonNull<U>` のときに使う。
#[expect(dead_code)]
pub(crate) fn set_optional_ptr<U>(value: Option<NonNull<U>>, set_fn: impl FnOnce(c_int, *const U)) {
    set_optional(value, |p| p.as_ptr(), std::ptr::null(), set_fn)
}

/// 生ポインタ (const) が出力の getter。
///
/// C API のシグネチャが `void get(int* out_has, const U** out_value)` のときに使う。
/// `out_has` が 1 なのに `out_value` が null の場合は panic する (`what` には関数名を渡す)。
/// 返すポインタの所有 / 借用は C API の契約に従う。
pub(crate) fn get_optional_ptr_const<U>(
    what: &'static str,
    get_fn: impl FnOnce(*mut c_int, *mut *const U),
) -> Option<ConstNonNull<U>> {
    get_optional(std::ptr::null::<U>(), |has, out_value| {
        get_fn(has, out_value)
    })
    .map(|raw| expect_non_null_const(raw, what))
}

/// ポインタ + 長さが出力の getter。
///
/// C API のシグネチャが `void get(int* out_has, const T** out_data, size_t* out_len)` のときに使う。
/// 返すスライスは `out_data` が指すデータを借用するため、`'a` はそのデータの寿命に合わせること。
/// `out_len` が 0 でないのに `out_data` が null の場合は panic する。
pub(crate) fn get_optional_slice<'a, T>(
    get_fn: impl FnOnce(*mut c_int, *mut *const T, *mut usize),
) -> Option<&'a [T]> {
    get_optional((std::ptr::null::<T>(), 0), |has, out_value| {
        let (data, len) = out_value;
        get_fn(has, data, len)
    })
    .map(|(data, len)| -> &'a [T] {
        if len == 0 {
            // 空の場合 C API は data に null を設定しうる。`from_raw_parts` は長さ 0 でも
            // null ポインタを受け付けないため、呼び出さずに空スライスを返す。
            return &[];
        }
        assert!(!data.is_null(), "get_optional_slice: data is null");
        // SAFETY: `data` は C API が設定した `len` 個の有効な要素を指す。
        unsafe { std::slice::from_raw_parts(data, len) }
    })
}

/// ポインタ + 長さで渡す setter。
///
/// C API のシグネチャが `void set(int has, const T* value, size_t value_len)` のときに使う。
pub(crate) fn set_optional_slice<T>(
    value: Option<&[T]>,
    set_fn: impl FnOnce(c_int, *const T, usize),
) {
    set_optional(
        value,
        |v| (v.as_ptr(), v.len()),
        (std::ptr::null(), 0),
        |has, (data, len)| set_fn(has, data, len),
    )
}
