use crate::CxxString;
use crate::ffi;
use std::ptr::NonNull;

/// webrtc_CreateRandomString の安全ラッパー。
pub fn random_string(len: i32) -> String {
    let raw = unsafe { ffi::webrtc_CreateRandomString(len) };
    CxxString::from_unique(
        NonNull::new(raw).expect("BUG: webrtc_CreateRandomString が null を返しました"),
    )
    .to_string()
    .expect("BUG: webrtc_CreateRandomString が不正な UTF-8 文字列を返しました")
}

/// webrtc::CreateRandomString を byte array として扱うヘルパー。
pub fn random_bytes(len: usize) -> Vec<u8> {
    random_string(len as i32).into_bytes()
}
