use crate::ref_count::DtlsTransportHandle;
use crate::{ScopedRef, ffi};
use std::os::raw::c_void;
use std::ptr::NonNull;

/// DtlsTransport の状態。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DtlsTransportState {
    New,
    Connecting,
    Connected,
    Closed,
    Failed,
    Unknown(i32),
}

impl DtlsTransportState {
    pub fn from_int(value: i32) -> Self {
        unsafe {
            if value == ffi::webrtc_DtlsTransportState_kNew {
                DtlsTransportState::New
            } else if value == ffi::webrtc_DtlsTransportState_kConnecting {
                DtlsTransportState::Connecting
            } else if value == ffi::webrtc_DtlsTransportState_kConnected {
                DtlsTransportState::Connected
            } else if value == ffi::webrtc_DtlsTransportState_kClosed {
                DtlsTransportState::Closed
            } else if value == ffi::webrtc_DtlsTransportState_kFailed {
                DtlsTransportState::Failed
            } else {
                DtlsTransportState::Unknown(value)
            }
        }
    }
}

/// DtlsTransportInterface のラッパー。
pub struct DtlsTransport {
    raw_ref: ScopedRef<DtlsTransportHandle>,
}

impl DtlsTransport {
    pub(crate) fn from_scoped_ref(raw_ref: ScopedRef<DtlsTransportHandle>) -> Self {
        Self { raw_ref }
    }

    /// DtlsTransport の状態を取得する。
    pub fn state(&self) -> DtlsTransportState {
        let state = unsafe { ffi::webrtc_DtlsTransportInterface_state(self.raw_ref.as_ptr()) };
        DtlsTransportState::from_int(state)
    }

    /// Observer を登録する。
    pub fn register_observer(&self, observer: &DtlsTransportObserver) {
        unsafe {
            ffi::webrtc_DtlsTransportInterface_RegisterObserver(
                self.raw_ref.as_ptr(),
                observer.as_ptr(),
            )
        };
    }

    /// Observer を解除する。
    pub fn unregister_observer(&self) {
        unsafe { ffi::webrtc_DtlsTransportInterface_UnregisterObserver(self.raw_ref.as_ptr()) };
    }
}

unsafe impl Send for DtlsTransport {}

// -------------------------
// DtlsTransportObserver
// -------------------------

pub trait DtlsTransportObserverHandler: Send {
    #[expect(unused_variables)]
    fn on_state_change(&mut self, new_state: DtlsTransportState) {}
    fn on_error(&mut self) {}
}

struct DtlsTransportObserverHandlerState {
    handler: Box<dyn DtlsTransportObserverHandler>,
}

unsafe extern "C" fn dtls_observer_on_state_change(new_state: i32, user_data: *mut c_void) {
    assert!(
        !user_data.is_null(),
        "dtls_observer_on_state_change: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut DtlsTransportObserverHandlerState) };
    state
        .handler
        .on_state_change(DtlsTransportState::from_int(new_state));
}

unsafe extern "C" fn dtls_observer_on_error(user_data: *mut c_void) {
    assert!(
        !user_data.is_null(),
        "dtls_observer_on_error: user_data is null"
    );
    let state = unsafe { &mut *(user_data as *mut DtlsTransportObserverHandlerState) };
    state.handler.on_error();
}

unsafe extern "C" fn dtls_observer_on_destroy(user_data: *mut c_void) {
    assert!(
        !user_data.is_null(),
        "dtls_observer_on_destroy: user_data is null"
    );
    let _ = unsafe { Box::from_raw(user_data as *mut DtlsTransportObserverHandlerState) };
}

/// DtlsTransportObserver のラッパー。
pub struct DtlsTransportObserver {
    raw: NonNull<ffi::webrtc_DtlsTransportObserver>,
}

impl DtlsTransportObserver {
    pub fn new_with_handler(handler: Box<dyn DtlsTransportObserverHandler>) -> Self {
        let state = Box::new(DtlsTransportObserverHandlerState { handler });
        let user_data = Box::into_raw(state) as *mut c_void;
        let cbs = ffi::webrtc_DtlsTransportObserver_cbs {
            OnStateChange: Some(dtls_observer_on_state_change),
            OnError: Some(dtls_observer_on_error),
            OnDestroy: Some(dtls_observer_on_destroy),
        };
        let raw = match NonNull::new(unsafe {
            ffi::webrtc_DtlsTransportObserver_new(&cbs, user_data)
        }) {
            Some(raw) => raw,
            None => {
                let _ =
                    unsafe { Box::from_raw(user_data as *mut DtlsTransportObserverHandlerState) };
                panic!("BUG: webrtc_DtlsTransportObserver_new が null を返しました");
            }
        };
        Self { raw }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_DtlsTransportObserver {
        self.raw.as_ptr()
    }
}

impl Drop for DtlsTransportObserver {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_DtlsTransportObserver_delete(self.raw.as_ptr()) };
    }
}

unsafe impl Send for DtlsTransportObserver {}
