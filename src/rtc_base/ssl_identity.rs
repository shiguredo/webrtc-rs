use crate::ffi;
use std::ptr::NonNull;

/// webrtc::SSLIdentity のラッパー。
///
/// PEM 形式の秘密鍵と証明書からクライアント証明書用の SSLIdentity を生成し、
/// TURN-TLS の mTLS (クライアント認証) に使用する。
pub struct SSLIdentity {
    raw_unique: NonNull<ffi::webrtc_SSLIdentity_unique>,
}

unsafe impl Send for SSLIdentity {}

impl SSLIdentity {
    /// PEM 形式の秘密鍵と証明書から SSLIdentity を生成する。
    pub fn create_from_pem_strings(private_key: &str, certificate: &str) -> Option<Self> {
        let raw = unsafe {
            ffi::webrtc_SSLIdentity_CreateFromPEMStrings(
                private_key.as_ptr() as *const _,
                private_key.len(),
                certificate.as_ptr() as *const _,
                certificate.len(),
            )
        };
        NonNull::new(raw).map(|raw_unique| Self { raw_unique })
    }

    /// PEM 形式の秘密鍵と証明書チェーンから SSLIdentity を生成する。
    pub fn create_from_pem_chain_strings(
        private_key: &str,
        certificate_chain: &str,
    ) -> Option<Self> {
        let raw = unsafe {
            ffi::webrtc_SSLIdentity_CreateFromPEMChainStrings(
                private_key.as_ptr() as *const _,
                private_key.len(),
                certificate_chain.as_ptr() as *const _,
                certificate_chain.len(),
            )
        };
        NonNull::new(raw).map(|raw_unique| Self { raw_unique })
    }

    /// 所有権を移転して生ポインタを返す。
    /// IceServer::set_tls_client_identity に渡すために使用する。
    pub fn into_raw(self) -> *mut ffi::webrtc_SSLIdentity_unique {
        std::mem::ManuallyDrop::new(self).raw_unique.as_ptr()
    }
}

impl Drop for SSLIdentity {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_SSLIdentity_unique_delete(self.raw_unique.as_ptr()) };
    }
}
