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
