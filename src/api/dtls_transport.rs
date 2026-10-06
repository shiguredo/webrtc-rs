use crate::helper::handler::{HandlerState, create_with_handler, destroy_handler};
use crate::helper::ref_count::DtlsTransportHandle;
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

unsafe impl Send for DtlsTransport {}

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
    ///
    /// この DtlsTransport に登録した `observer` は、`unregister_observer` で登録を解除する
    /// まで drop してはならない。
    ///
    /// # Safety
    /// この transport を所有する native network thread 上で呼び出すこと。
    /// 登録中の observer 操作と通知は同じ network thread で直列に行うこと。
    /// 登録解除が完了するまで observer を生存させること。1 observer を 1 登録先だけで使い、
    /// callback 中の操作で同じ handler に再入させないこと。
    ///
    /// ```compile_fail,E0133
    /// use shiguredo_webrtc::*;
    /// fn check(channel: &DtlsTransport, observer: &DtlsTransportObserver) {
    ///     channel.register_observer(observer);
    /// }
    /// ```
    pub unsafe fn register_observer(&self, observer: &DtlsTransportObserver) {
        unsafe {
            ffi::webrtc_DtlsTransportInterface_RegisterObserver(
                self.raw_ref.as_ptr(),
                observer.as_ptr(),
            )
        };
    }

    /// Observer を解除する。
    ///
    /// # Safety
    /// この transport を所有する native network thread 上で呼び出すこと。
    /// observer の callback を実行中ではないこと。解除が戻るまで observer を生存させること。
    ///
    /// ```compile_fail,E0133
    /// use shiguredo_webrtc::DtlsTransport;
    /// fn check(transport: &DtlsTransport) {
    ///     transport.unregister_observer();
    /// }
    /// ```
    pub unsafe fn unregister_observer(&self) {
        unsafe { ffi::webrtc_DtlsTransportInterface_UnregisterObserver(self.raw_ref.as_ptr()) };
    }
}

impl Clone for DtlsTransport {
    fn clone(&self) -> Self {
        Self {
            raw_ref: ScopedRef::clone(&self.raw_ref),
        }
    }
}

// -------------------------
// DtlsTransportObserver
// -------------------------

/// network thread で直列に呼ばれる observer。
/// 登録先をまたぐ排他は保証されず、同時呼び出しと再入は登録側で排除する。
pub trait DtlsTransportObserverHandler: Send {
    #[expect(unused_variables)]
    fn on_state_change(&mut self, new_state: DtlsTransportState) {}
    fn on_error(&mut self) {}
}

type DtlsTransportObserverHandlerState = HandlerState<dyn DtlsTransportObserverHandler>;

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
    unsafe {
        destroy_handler::<DtlsTransportObserverHandlerState>("dtls_observer_on_destroy", user_data)
    };
}

/// DtlsTransportObserver のラッパー。
pub struct DtlsTransportObserver {
    raw: NonNull<ffi::webrtc_DtlsTransportObserver>,
}

unsafe impl Send for DtlsTransportObserver {}

impl DtlsTransportObserver {
    pub fn new_with_handler(handler: Box<dyn DtlsTransportObserverHandler>) -> Self {
        let user_data = Box::into_raw(Box::new(HandlerState::new(handler))) as *mut c_void;
        let cbs = ffi::webrtc_DtlsTransportObserver_cbs {
            OnStateChange: Some(dtls_observer_on_state_change),
            OnError: Some(dtls_observer_on_error),
            OnDestroy: Some(dtls_observer_on_destroy),
        };
        let raw = unsafe {
            create_with_handler::<DtlsTransportObserverHandlerState, _>(
                "webrtc_DtlsTransportObserver_new",
                user_data,
                |user_data| ffi::webrtc_DtlsTransportObserver_new(&cbs, user_data),
            )
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
