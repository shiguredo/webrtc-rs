use crate::{CxxString, Result, ffi};
use std::marker::PhantomData;
use std::os::raw::c_void;
use std::ptr::NonNull;

/// webrtc::SSLCertificate の借用ラッパー。
#[derive(Clone, Copy)]
pub struct SSLCertificateRef<'a> {
    raw: NonNull<ffi::webrtc_SSLCertificate>,
    _marker: PhantomData<&'a ffi::webrtc_SSLCertificate>,
}

unsafe impl<'a> Send for SSLCertificateRef<'a> {}

impl<'a> SSLCertificateRef<'a> {
    pub fn from_raw(raw: NonNull<ffi::webrtc_SSLCertificate>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_SSLCertificate {
        self.raw.as_ptr()
    }

    pub fn to_pem_string(&self) -> Result<String> {
        let raw =
            NonNull::new(unsafe { ffi::webrtc_SSLCertificate_ToPEMString(self.raw.as_ptr()) })
                .expect("BUG: webrtc_SSLCertificate_ToPEMString が null を返しました");
        CxxString::from_unique(raw).to_string()
    }

    pub fn to_der(&self) -> Vec<u8> {
        let raw = NonNull::new(unsafe { ffi::webrtc_SSLCertificate_ToDER(self.raw.as_ptr()) })
            .expect("BUG: webrtc_SSLCertificate_ToDER が null を返しました");
        CxxString::from_unique(raw).to_bytes()
    }

    pub fn certificate_expiration_time(&self) -> i64 {
        unsafe { ffi::webrtc_SSLCertificate_CertificateExpirationTime(self.raw.as_ptr()) }
    }
}

/// webrtc::SSLCertChain の借用ラッパー。
#[derive(Clone, Copy)]
pub struct SSLCertChainRef<'a> {
    raw: NonNull<ffi::webrtc_SSLCertChain>,
    _marker: PhantomData<&'a ffi::webrtc_SSLCertChain>,
}

unsafe impl<'a> Send for SSLCertChainRef<'a> {}

impl<'a> SSLCertChainRef<'a> {
    pub fn from_raw(raw: NonNull<ffi::webrtc_SSLCertChain>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_SSLCertChain {
        self.raw.as_ptr()
    }

    pub fn len(&self) -> usize {
        let len = unsafe { ffi::webrtc_SSLCertChain_GetSize(self.raw.as_ptr()) };
        len.max(0) as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: usize) -> Option<SSLCertificateRef<'a>> {
        if index >= self.len() {
            return None;
        }
        let raw = NonNull::new(unsafe {
            ffi::webrtc_SSLCertChain_Get(self.raw.as_ptr(), index as i32)
                as *mut ffi::webrtc_SSLCertificate
        })
        .expect("BUG: webrtc_SSLCertChain_Get が null を返しました");
        Some(SSLCertificateRef::from_raw(raw))
    }
}

pub trait SSLCertificateVerifierHandler: Send {
    #[expect(unused_variables)]
    fn verify_chain(&mut self, chain: SSLCertChainRef<'_>) -> bool {
        false
    }
}

struct SSLCertificateVerifierHandlerState {
    handler: Box<dyn SSLCertificateVerifierHandler>,
}

unsafe impl Send for SSLCertificateVerifierHandlerState {}

unsafe extern "C" fn ssl_certificate_verifier_verify_chain(
    chain: *const ffi::webrtc_SSLCertChain,
    user_data: *mut c_void,
) -> i32 {
    assert!(
        !user_data.is_null(),
        "ssl_certificate_verifier_verify_chain: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut SSLCertificateVerifierHandlerState) };
    let chain =
        NonNull::new(chain as *mut ffi::webrtc_SSLCertChain).expect("BUG: chain が null です");
    let chain = SSLCertChainRef::from_raw(chain);
    if state.handler.verify_chain(chain) {
        1
    } else {
        0
    }
}

unsafe extern "C" fn ssl_certificate_verifier_on_destroy(user_data: *mut c_void) {
    assert!(
        !user_data.is_null(),
        "ssl_certificate_verifier_on_destroy: user_data is null"
    );
    let _ = unsafe { Box::from_raw(user_data as *mut SSLCertificateVerifierHandlerState) };
}

/// webrtc::SSLCertificateVerifier のラッパー。
pub struct SSLCertificateVerifier {
    raw_unique: NonNull<ffi::webrtc_SSLCertificateVerifier_unique>,
}

unsafe impl Send for SSLCertificateVerifier {}

impl SSLCertificateVerifier {
    pub fn new_with_handler(handler: Box<dyn SSLCertificateVerifierHandler>) -> Self {
        let state = Box::new(SSLCertificateVerifierHandlerState { handler });
        let user_data = Box::into_raw(state) as *mut c_void;
        let cbs = ffi::webrtc_SSLCertificateVerifier_cbs {
            VerifyChain: Some(ssl_certificate_verifier_verify_chain),
            OnDestroy: Some(ssl_certificate_verifier_on_destroy),
        };
        let raw = unsafe { ffi::webrtc_SSLCertificateVerifier_new(&cbs, user_data) };
        let raw_unique = match NonNull::new(raw) {
            Some(raw_unique) => raw_unique,
            None => {
                let _ =
                    unsafe { Box::from_raw(user_data as *mut SSLCertificateVerifierHandlerState) };
                panic!("BUG: webrtc_SSLCertificateVerifier_new が null を返しました");
            }
        };
        Self { raw_unique }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_SSLCertificateVerifier {
        unsafe { ffi::webrtc_SSLCertificateVerifier_unique_get(self.raw_unique.as_ptr()) }
    }

    pub fn into_raw(self) -> *mut ffi::webrtc_SSLCertificateVerifier_unique {
        std::mem::ManuallyDrop::new(self).raw_unique.as_ptr()
    }
}

impl Drop for SSLCertificateVerifier {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_SSLCertificateVerifier_unique_delete(self.raw_unique.as_ptr()) };
    }
}
