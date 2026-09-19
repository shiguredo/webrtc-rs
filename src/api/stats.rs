use crate::const_non_null::ConstNonNull;
use crate::helper::non_null::expect_non_null;
use crate::helper::ref_count::RTCStatsReportHandle;
use crate::{CxxString, Result, ScopedRefConst, ffi};

/// webrtc::RTCStatsReport のラッパー。
pub struct RTCStatsReport {
    raw_ref: ScopedRefConst<RTCStatsReportHandle>,
}

unsafe impl Send for RTCStatsReport {}

impl RTCStatsReport {
    pub(crate) fn from_refcounted_ptr(
        raw_ref: ConstNonNull<ffi::webrtc_RTCStatsReport_refcounted>,
    ) -> Self {
        let raw_ref = ScopedRefConst::<RTCStatsReportHandle>::from_raw(raw_ref);
        Self { raw_ref }
    }

    pub fn to_json(&self) -> Result<String> {
        let raw = self.raw_ref.raw();
        let json = unsafe { ffi::webrtc_RTCStatsReport_ToJson(raw.as_ptr()) };
        let json = expect_non_null(json, "webrtc_RTCStatsReport_ToJson");
        let json = CxxString::from_unique(json);
        json.to_string()
    }
}
