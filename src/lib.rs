pub mod constants;
pub mod ffi;

/// クレートのバージョンを返す
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

mod api;
mod const_non_null;
mod cxxstd;
mod error;
mod helper;
mod libyuv;
mod rtc_base;
mod util;

#[cfg(test)]
mod tests;

pub use api::*;
pub use const_non_null::ConstNonNull;
pub use cxxstd::{
    CxxString, CxxStringRef, CxxStringRefMut, MapStringStringIter, MapStringStringRef,
    MapStringStringRefMut, StringVector, StringVectorRef, StringVectorRefMut,
};
pub use error::{Error, Result};
use helper::ref_count::{ScopedRef, ScopedRefConst};
pub use libyuv::{
    LibyuvFourcc, LibyuvRotationMode, abgr_to_i420, convert_from_i420, convert_to_i420, i420_copy,
    i420_rotate, i420_to_nv12, mjpg_size, mjpg_to_i420, mjpg_to_nv12, nv12_copy, nv12_to_i420,
    yuy2_to_i420,
};
pub use rtc_base::{
    Buffer, BufferRef, BufferRefMut, BufferS16Ref, BufferS16RefMut, SSLCertChainRef,
    SSLCertificateRef, SSLCertificateVerifier, SSLCertificateVerifierHandler, SSLIdentity, Thread,
    TimestampAligner, log, random_bytes, random_string, rtc_log_format_file, time_millis,
};
pub use util::*;

/// 借用ハンドルの型システム保証を固定するための `compile_fail` doctest。
///
/// ユーザー向けのドキュメントではなく回帰テストなので、`cfg(doctest)` と `#[doc(hidden)]` で
/// 通常のビルドとドキュメントからは見えないようにしている。
#[cfg(doctest)]
#[doc(hidden)]
pub mod compile_fail_doctests {
    //! 借用ハンドルの型システム保証を固定する。
    //!
    //! `XxxRefMut` は `Deref` を実装しないため、借用を外して `XxxRef` を取り出せない (E0614)。
    //!
    //! ```compile_fail,E0614
    //! use shiguredo_webrtc::RtpEncodingParameters;
    //!
    //! let mut parameters = RtpEncodingParameters::new();
    //! let r = parameters.as_mut();
    //! let _: &shiguredo_webrtc::RtpEncodingParametersRef<'_> = &*r;
    //! ```
    //!
    //! 借用ハンドルは取得元の借用にライフタイムが縛られるため、同じオブジェクトへの可変ハンドルを
    //! 2 本作ることはできない (E0499)。
    //!
    //! ```compile_fail,E0499
    //! use shiguredo_webrtc::RtpEncodingParameters;
    //!
    //! let mut parameters = RtpEncodingParameters::new();
    //! let a = parameters.as_mut();
    //! let b = parameters.as_mut();
    //! # let _ = (a, b);
    //! ```
    //!
    //! `&mut self` を取る可変アクセサの戻り値も `'_` に縛られるため、借用を保持したまま同じ
    //! ハンドルを再度書き換えることはできない (E0499)。
    //!
    //! ```compile_fail,E0499
    //! use shiguredo_webrtc::RtpCodecCapability;
    //!
    //! let mut capability = RtpCodecCapability::new();
    //! let mut r = capability.as_mut();
    //! let parameters = r.parameters_mut();
    //! let parameters2 = r.parameters_mut();
    //! # let _ = (parameters, parameters2);
    //! ```
    //!
    //! 所有権や生ポインタを受け取るコンストラクタは crate 内部専用のため、crate 外からは呼べない
    //! (E0624)。
    //!
    //! ```compile_fail,E0624
    //! use std::ptr::NonNull;
    //! use shiguredo_webrtc::{CxxString, ffi};
    //!
    //! let raw = NonNull::<ffi::std_string_unique>::dangling();
    //! let _ = CxxString::from_unique(raw);
    //! ```
    //!
    //! ```compile_fail,E0624
    //! use std::ptr::NonNull;
    //! use shiguredo_webrtc::{RtpCapabilities, ffi};
    //!
    //! let raw = NonNull::<ffi::webrtc_RtpCapabilities>::dangling();
    //! let _ = RtpCapabilities::from_raw(raw);
    //! ```
}
