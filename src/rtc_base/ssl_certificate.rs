use crate::helper::handler::{HandlerState, create_with_handler, destroy_handler};
use crate::helper::non_null::expect_non_null;
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
        let raw = expect_non_null(
            unsafe { ffi::webrtc_SSLCertificate_ToPEMString(self.raw.as_ptr()) },
            "webrtc_SSLCertificate_ToPEMString",
        );
        CxxString::from_unique(raw).to_string()
    }

    pub fn to_der(&self) -> Vec<u8> {
        let raw = expect_non_null(
            unsafe { ffi::webrtc_SSLCertificate_ToDER(self.raw.as_ptr()) },
            "webrtc_SSLCertificate_ToDER",
        );
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
        let raw = expect_non_null(
            unsafe {
                ffi::webrtc_SSLCertChain_Get(self.raw.as_ptr(), index as i32)
                    as *mut ffi::webrtc_SSLCertificate
            },
            "webrtc_SSLCertChain_Get",
        );
        Some(SSLCertificateRef::from_raw(raw))
    }
}

pub trait SSLCertificateVerifierHandler: Send {
    #[expect(unused_variables)]
    fn verify_chain(&mut self, chain: SSLCertChainRef<'_>) -> bool {
        false
    }
}

type SSLCertificateVerifierHandlerState = HandlerState<dyn SSLCertificateVerifierHandler>;

unsafe extern "C" fn ssl_certificate_verifier_verify_chain(
    chain: *const ffi::webrtc_SSLCertChain,
    user_data: *mut c_void,
) -> i32 {
    assert!(
        !user_data.is_null(),
        "ssl_certificate_verifier_verify_chain: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut SSLCertificateVerifierHandlerState) };
    let chain = expect_non_null(chain as *mut ffi::webrtc_SSLCertChain, "SSLCertChain");
    let chain = SSLCertChainRef::from_raw(chain);
    if state.handler.verify_chain(chain) {
        1
    } else {
        0
    }
}

unsafe extern "C" fn ssl_certificate_verifier_on_destroy(user_data: *mut c_void) {
    unsafe {
        destroy_handler::<SSLCertificateVerifierHandlerState>(
            "ssl_certificate_verifier_on_destroy",
            user_data,
        )
    };
}

/// webrtc::SSLCertificateVerifier のラッパー。
pub struct SSLCertificateVerifier {
    raw_unique: NonNull<ffi::webrtc_SSLCertificateVerifier_unique>,
}

unsafe impl Send for SSLCertificateVerifier {}

impl SSLCertificateVerifier {
    pub fn new_with_handler(handler: Box<dyn SSLCertificateVerifierHandler>) -> Self {
        let user_data = Box::into_raw(Box::new(HandlerState::new(handler))) as *mut c_void;
        let cbs = ffi::webrtc_SSLCertificateVerifier_cbs {
            VerifyChain: Some(ssl_certificate_verifier_verify_chain),
            OnDestroy: Some(ssl_certificate_verifier_on_destroy),
        };
        let raw_unique = unsafe {
            create_with_handler::<SSLCertificateVerifierHandlerState, _>(
                "webrtc_SSLCertificateVerifier_new",
                user_data,
                |user_data| ffi::webrtc_SSLCertificateVerifier_new(&cbs, user_data),
            )
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
