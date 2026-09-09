use crate::CxxString;
use crate::ffi;
use crate::helper::non_null::expect_non_null;

/// webrtc_CreateRandomString の安全ラッパー。
pub fn random_string(len: usize) -> String {
    let raw = unsafe { ffi::webrtc_CreateRandomString(len) };
    CxxString::from_unique(expect_non_null(raw, "webrtc_CreateRandomString"))
        .to_string()
        .expect("BUG: webrtc_CreateRandomString が不正な UTF-8 文字列を返しました")
}

/// webrtc::CreateRandomString を byte array として扱うヘルパー。
pub fn random_bytes(len: usize) -> Vec<u8> {
    random_string(len).into_bytes()
}
