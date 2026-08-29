//! 内部向け共通ヘルパー。
//!
//! C API との FFI を安全に扱うための定型的な処理 (null 検査、コールバック型ハンドラの
//! 登録・破棄、参照カウンタベースハンドル、optional 値、out 引数 / out_error) を集約する。
//! 各ヘルパーは `crate::ffi` の生ポインタと `crate::Error` / `crate::Result` の橋渡しを担い、
//! 公開 API モジュール (`crate::api` / `crate::rtc_base`) から利用される。

pub(crate) mod handler;
pub(crate) mod non_null;
pub(crate) mod optional;
pub(crate) mod out_param;
pub(crate) mod ref_count;
