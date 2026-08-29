//! C API の out 引数 (出力ポインタ) と out_error の扱いを共通化するヘルパー。
//!
//! C API (`webrtc/src/webrtc_c/api/*.h`) は生成したオブジェクトを out 引数または
//! 戻り値で返し、エラーを `out_error` (`*_unique` ポインタ) で返す方式が広く使われる。
//! この定型的な null 検査 / エラー検査を各ヘルパーに集約し、呼び出しごとの
//! 繰り返しを無くす。
//!
//! # null ポリシー
//!
//! - 失敗シグナルの無い API (`call_with_out`): out の null は生成失敗そのものを意味する
//!   ため `Error::NullPointer` を返す
//! - `out_error` を持つ API (`call_with_out_and_error` / `call_with_return_and_error`)
//!   : `out_error` が失敗シグナルであり、成功時に対象のオブジェクトが null を返すのは
//!   規約違反とみなして panic する
//! - `out_error` を持ち戻り値の無い API (`call_with_void_and_error`): 検査すべき
//!   オブジェクトが無いため `out_error` の検査のみ行う

use crate::non_null::expect_non_null;
use crate::{Error, Result};
use std::ptr::NonNull;

/// out 引数方式の C API を呼び出し、`NonNull` にして返す。
///
/// `call` には FFI の関数を呼び出すクロージャを渡す。クロージャは out ポインタ
/// のアドレス (`*mut *mut T`) を受け取り、C API がそこへ生成結果を書き込む。
/// C API が out に null を書き込んだ場合は生成失敗とみなし `Error::NullPointer` を
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
/// `what` には out を書き込む関数名を渡すこと。これは panic メッセージに使われる。
///
/// `call` には FFI の関数を呼び出すクロージャを渡す。クロージャは out ポインタの
/// アドレス (`*mut *mut R`) と out_error のアドレス (`*mut *mut TE`) を
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
pub(crate) fn call_with_out_and_error<TE, R>(
    what: &'static str,
    call: impl FnOnce(*mut *mut R, *mut *mut TE),
    make_err: impl FnOnce(NonNull<TE>) -> Error,
) -> Result<NonNull<R>> {
    let mut out: *mut R = std::ptr::null_mut();
    let mut out_error: *mut TE = std::ptr::null_mut();
    call(&mut out, &mut out_error);
    if !out_error.is_null() {
        let err = make_err(NonNull::new(out_error).unwrap());
        return Err(err);
    }
    Ok(expect_non_null(out, what))
}

/// 結果をクロージャの戻り値のオブジェクトポインタで、エラーを out_error で返す
/// C API を呼び出し、`NonNull` にして返す。
///
/// `what` には戻り値のオブジェクトを生成する関数名を渡すこと。これは panic メッセージに
/// 使われる。
///
/// `call` には FFI の関数を呼び出すクロージャを渡す。クロージャは out_error の
/// アドレス (`*mut *mut TE`) を受け取り、C API を呼び出して生成したオブジェクトの
/// 生ポインタ `*mut R` を返す。
///
/// C API が out_error へ null 以外を書き込んだ場合は `make_err` でエラーへ変換して
/// `Err` を返す。C API が out_error に null を設定した場合 (成功)、返された
/// ポインタは `NonNull` へ変換して返す。
///
/// `make_err` には out_error のポインタを `Error` へ変換するクロージャを渡す。
///
/// # Panics
///
/// out_error が null (成功) にも関わらず戻り値のポインタが null を返した場合、
/// C API の規約違反とみなして panic する。
pub(crate) fn call_with_return_and_error<TE, R>(
    what: &'static str,
    call: impl FnOnce(*mut *mut TE) -> *mut R,
    make_err: impl FnOnce(NonNull<TE>) -> Error,
) -> Result<NonNull<R>> {
    let mut out_error: *mut TE = std::ptr::null_mut();
    let ret = call(&mut out_error);
    if !out_error.is_null() {
        let err = make_err(NonNull::new(out_error).unwrap());
        return Err(err);
    }
    Ok(expect_non_null(ret, what))
}

/// 結果を返さず、エラーを out_error で返す C API を呼び出し、エラーを検査する。
///
/// `call` には FFI の関数を呼び出すクロージャを渡す。クロージャは out_error の
/// アドレス (`*mut *mut TE`) を受け取り、C API を呼び出して戻り値の無い操作を
/// 実行する。`call` は検査すべきオブジェクトを返さないため `()` を返す。
///
/// C API が out_error へ null 以外を書き込んだ場合は `make_err` でエラーへ変換して
/// `Err` を返す。C API が out_error に null を設定した場合 (成功)、`Ok(())` を返す。
///
/// `make_err` には out_error のポインタを `Error` へ変換するクロージャを渡す。
pub(crate) fn call_with_void_and_error<TE>(
    call: impl FnOnce(*mut *mut TE),
    make_err: impl FnOnce(NonNull<TE>) -> Error,
) -> Result<()> {
    let mut out_error: *mut TE = std::ptr::null_mut();
    call(&mut out_error);
    if !out_error.is_null() {
        let err = make_err(NonNull::new(out_error).unwrap());
        return Err(err);
    }
    Ok(())
}
