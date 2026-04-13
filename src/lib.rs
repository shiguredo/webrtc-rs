pub mod ffi;

/// クレートのバージョンを返す
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

mod api;
mod cxxstd;
mod error;
mod libyuv;
mod ref_count;
mod rtc_base;

#[cfg(test)]
mod tests;

pub use api::*;
pub use cxxstd::{
    CxxString, CxxStringRef, MapStringString, MapStringStringIter, StringVector, StringVectorRef,
};
pub use error::{Error, Result};
pub use libyuv::{
    LibyuvFourcc, abgr_to_i420, convert_from_i420, i420_copy, i420_to_nv12, nv12_copy,
    nv12_to_i420, yuy2_to_i420,
};
pub use ref_count::{RefCountedHandle, ScopedRef};
pub use rtc_base::{
    SSLCertChainRef, SSLCertificateRef, SSLCertificateVerifier, SSLCertificateVerifierHandler,
    SSLIdentity, Thread, TimestampAligner, log, random_bytes, random_string, rtc_log_format_file,
    time_millis,
};
