use crate::{ffi, CxxString, Error, Result};
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ptr::NonNull;

/// webrtc::SdpType のラッパー。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SdpType {
    Offer,
    PrAnswer,
    Answer,
    Rollback,
    Unknown(i32),
}

impl SdpType {
    pub fn from_int(value: i32) -> Self {
        unsafe {
            if value == ffi::webrtc_SdpType_kOffer {
                SdpType::Offer
            } else if value == ffi::webrtc_SdpType_kPrAnswer {
                SdpType::PrAnswer
            } else if value == ffi::webrtc_SdpType_kAnswer {
                SdpType::Answer
            } else if value == ffi::webrtc_SdpType_kRollback {
                SdpType::Rollback
            } else {
                SdpType::Unknown(value)
            }
        }
    }

    pub fn to_int(&self) -> i32 {
        match self {
            SdpType::Offer => unsafe { ffi::webrtc_SdpType_kOffer },
            SdpType::PrAnswer => unsafe { ffi::webrtc_SdpType_kPrAnswer },
            SdpType::Answer => unsafe { ffi::webrtc_SdpType_kAnswer },
            SdpType::Rollback => unsafe { ffi::webrtc_SdpType_kRollback },
            SdpType::Unknown(v) => *v,
        }
    }
}

/// webrtc::SessionDescriptionInterface のラッパー。
pub struct SessionDescription {
    raw_unique: NonNull<ffi::webrtc_SessionDescriptionInterface_unique>,
}

impl SessionDescription {
    pub fn new(sdp_type: SdpType, sdp: &str) -> Result<Self> {
        let raw = unsafe {
            ffi::webrtc_CreateSessionDescription(
                sdp_type.to_int(),
                sdp.as_ptr() as *const _,
                sdp.len(),
            )
        };
        let raw_unique = NonNull::new(raw).ok_or(Error::NullPointer(
            "webrtc_CreateSessionDescription が null を返しました",
        ))?;
        Ok(Self { raw_unique })
    }

    pub fn from_unique_ptr(raw: NonNull<ffi::webrtc_SessionDescriptionInterface_unique>) -> Self {
        Self { raw_unique: raw }
    }

    pub fn into_raw(self) -> *mut ffi::webrtc_SessionDescriptionInterface_unique {
        ManuallyDrop::new(self).raw_unique.as_ptr()
    }

    pub fn sdp_type(&self) -> SdpType {
        let raw = self.raw();
        let ty = unsafe { ffi::webrtc_SessionDescriptionInterface_GetType(raw.as_ptr()) };
        SdpType::from_int(ty)
    }

    pub fn to_string(&self) -> Result<String> {
        let raw = self.raw();
        let mut out = std::ptr::null_mut();
        let ok =
            unsafe { ffi::webrtc_SessionDescriptionInterface_ToString(raw.as_ptr(), &mut out) };
        if ok == 0 {
            return Err(Error::InvalidSdp);
        }
        CxxString::from_unique(NonNull::new(out).expect("BUG: ok != 0 なのに out が null"))
            .to_string()
    }

    fn raw(&self) -> NonNull<ffi::webrtc_SessionDescriptionInterface> {
        let raw =
            unsafe { ffi::webrtc_SessionDescriptionInterface_unique_get(self.raw_unique.as_ptr()) };
        NonNull::new(raw).expect("BUG: SessionDescriptionInterface が null を返しました")
    }
}

impl Drop for SessionDescription {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_SessionDescriptionInterface_unique_delete(self.raw_unique.as_ptr()) };
    }
}

/// webrtc::IceCandidateInterface の借用ラッパー。
pub struct IceCandidateRef<'a> {
    raw: NonNull<ffi::webrtc_IceCandidateInterface>,
    _marker: PhantomData<&'a ffi::webrtc_IceCandidateInterface>,
}

impl<'a> IceCandidateRef<'a> {
    /// 生ポインタから借用ラップする。
    pub fn from_raw(raw: NonNull<ffi::webrtc_IceCandidateInterface>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_IceCandidateInterface {
        self.raw.as_ptr()
    }

    pub fn sdp_mid(&self) -> Result<String> {
        let mut out: *mut ffi::std_string_unique = std::ptr::null_mut();
        unsafe { ffi::webrtc_IceCandidateInterface_sdp_mid(self.raw.as_ptr(), &mut out) };
        CxxString::from_unique(
            NonNull::new(out)
                .expect("BUG: webrtc_IceCandidateInterface_sdp_mid が null を返しました"),
        )
        .to_string()
    }

    pub fn sdp_mline_index(&self) -> i32 {
        unsafe { ffi::webrtc_IceCandidateInterface_sdp_mline_index(self.raw.as_ptr()) }
    }

    pub fn to_string(&self) -> Result<String> {
        let mut out: *mut ffi::std_string_unique = std::ptr::null_mut();
        let ok = unsafe { ffi::webrtc_IceCandidateInterface_ToString(self.raw.as_ptr(), &mut out) };
        if ok == 0 {
            return Err(Error::InvalidIceCandidate);
        }
        CxxString::from_unique(NonNull::new(out).expect("BUG: ok != 0 なのに out が null"))
            .to_string()
    }
}

/// webrtc::IceCandidateInterface の所有ラッパー。
pub struct IceCandidate {
    raw: NonNull<ffi::webrtc_IceCandidateInterface>,
}

impl IceCandidate {
    /// SDP 文字列から IceCandidate を生成する。
    pub fn new(sdp_mid: &str, sdp_mline_index: i32, candidate: &str) -> Result<Self> {
        let raw = unsafe {
            ffi::webrtc_CreateIceCandidate(
                sdp_mid.as_ptr() as *const _,
                sdp_mid.len(),
                sdp_mline_index,
                candidate.as_ptr() as *const _,
                candidate.len(),
            )
        };
        let raw = NonNull::new(raw).ok_or(Error::InvalidIceCandidate)?;
        Ok(Self { raw })
    }

    pub fn as_ref(&self) -> IceCandidateRef<'_> {
        IceCandidateRef::from_raw(self.raw)
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_IceCandidateInterface {
        self.raw.as_ptr()
    }

    pub fn sdp_mid(&self) -> Result<String> {
        self.as_ref().sdp_mid()
    }

    pub fn sdp_mline_index(&self) -> i32 {
        self.as_ref().sdp_mline_index()
    }

    pub fn to_string(&self) -> Result<String> {
        self.as_ref().to_string()
    }
}

impl Drop for IceCandidate {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_IceCandidateInterface_delete(self.raw.as_ptr()) };
    }
}
