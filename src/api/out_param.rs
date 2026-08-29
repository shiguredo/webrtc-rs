//! C API の out 引数 (出力ポインタ) と out_error の扱いを共通化するヘルパー。
//!
//! C API (`webrtc/src/webrtc_c/api/*.h`) は生成したオブジェクトを out 引数で
//! 返し、エラーを `out_error` (`*_unique` ポインタ) で返す方式が広く使われる。
//! この定型的な null 検査 / エラー検査を各ヘルパーに集約し、呼び出しごとの
//! 繰り返しを無くす。

use crate::non_null::expect_non_null;
use crate::{Error, Result};
use std::ptr::NonNull;

/// out 引数方式の C API を呼び出し、`NonNull` にして返す。
///
/// `call` には FFI の関数を呼び出すクロージャを渡す。クロージャは out ポインタ
/// のアドレス (`*mut *mut T`) を受け取り、C API がそこへ生成結果を書き込む。
/// C API が out に null を返した場合は実装バグとみなし `Error::NullPointer` を
/// 返す。
///
/// `what` には out を書き込む関数名を渡すこと。これはエラーメッセージに使われる。
pub(crate) fn call_with_out<T>(
    what: &'static str,
    call: impl FnOnce(*mut *mut T),
) -> Result<NonNull<T>> {
    let mut out: *mut T = std::ptr::null_mut();
    call(&mut out);
    NonNull::new(out).ok_or(Error::NullPointer(what))
}

/// 結果を out 引数で、エラーを out_error で返す C API を呼び出し、`NonNull` にして返す。
///
/// `call` には FFI の関数を呼び出すクロージャを渡す。クロージャは out ポインタの
/// アドレス (`*mut *mut TObj`) と out_error のアドレス (`*mut *mut TE`) を
/// 受け取り、C API を呼び出す。C API が out_error へ null 以外を書き込んだ場合
/// は `make_err` でエラーへ変換して `Err` を返す。
///
/// C API はエラー時に out_error を設定し、成功時に out へ null 以外を書き込む
/// 規約に基づくため、out_error が null の場合に out に含まれる値は常に
/// null 以外であり、`NonNull` へ変換して返す。
///
/// `make_err` には out_error のポインタを `Error` へ変換するクロージャを渡す。
///
/// # Panics
///
/// out_error が null にも関わらず out が null を返した場合、C API の規約違反と
/// みなして panic する。
pub(crate) fn call_with_out_and_error<TObj, TE>(
    call: impl FnOnce(*mut *mut TObj, *mut *mut TE),
    make_err: impl FnOnce(NonNull<TE>) -> Error,
) -> Result<NonNull<TObj>> {
    let mut out: *mut TObj = std::ptr::null_mut();
    let mut out_error: *mut TE = std::ptr::null_mut();
    call(&mut out, &mut out_error);
    if !out_error.is_null() {
        let err = make_err(NonNull::new(out_error).unwrap());
        return Err(err);
    }
    Ok(expect_non_null(out, "call"))
}

/// 結果をクロージャの戻り値で、エラーを out_error で返す C API を呼び出し、
/// エラーを検査する。
///
/// `call` には FFI の関数を呼び出すクロージャを渡す。クロージャは out_error の
/// アドレス (`*mut *mut TE`) を受け取り、C API を呼び出して任意の値 `R` を返す。
/// `R` には生成したオブジェクトのポインタや `()` など、呼び出し側が必要とする
/// 値を指定する。
///
/// C API が out_error へ null 以外を書き込んだ場合は `make_err` でエラーへ変換して
/// `Err` を返す。C API が out_error に null を設定した場合 (成功)、クロージャの
/// 戻り値 `R` を `Ok` で返す。
pub(crate) fn call_with_return_and_error<TE, R>(
    call: impl FnOnce(*mut *mut TE) -> R,
    make_err: impl FnOnce(NonNull<TE>) -> Error,
) -> Result<R> {
    let mut out_error: *mut TE = std::ptr::null_mut();
    let ret = call(&mut out_error);
    if !out_error.is_null() {
        let err = make_err(NonNull::new(out_error).unwrap());
        return Err(err);
    }
    Ok(ret)
}
