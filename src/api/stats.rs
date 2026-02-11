use crate::ref_count::RTCStatsReportHandle;
use crate::{CxxString, Result, ScopedRef, ffi};
use std::ptr::NonNull;

/// webrtc::RTCStatsReport のラッパー。
pub struct RTCStatsReport {
    raw_ref: ScopedRef<RTCStatsReportHandle>,
}

impl RTCStatsReport {
    pub fn from_refcounted_ptr(raw_ref: NonNull<ffi::webrtc_RTCStatsReport_refcounted>) -> Self {
        let raw_ref = ScopedRef::<RTCStatsReportHandle>::from_raw(raw_ref);
        Self { raw_ref }
    }

    pub fn to_json(&self) -> Result<String> {
        let raw = self.raw();
        let json = unsafe { ffi::webrtc_RTCStatsReport_ToJson(raw.as_ptr()) };
        let json =
            NonNull::new(json).expect("BUG: webrtc_RTCStatsReport_ToJson が null を返しました");
        let json = CxxString::from_unique(json);
        json.to_string()
    }

    fn raw(&self) -> NonNull<ffi::webrtc_RTCStatsReport> {
        self.raw_ref.raw()
    }
}
