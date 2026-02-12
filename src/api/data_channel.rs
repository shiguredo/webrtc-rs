use crate::ref_count::DataChannelHandle;
use crate::{CxxString, Result, ScopedRef, ffi};
use std::os::raw::c_void;
use std::ptr::NonNull;
use std::slice;

/// DataChannel の状態。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataChannelState {
    Connecting,
    Open,
    Closing,
    Closed,
    Unknown(i32),
}

impl DataChannelState {
    pub fn from_int(value: i32) -> Self {
        unsafe {
            if value == ffi::webrtc_DataChannelInterface_DataState_kConnecting {
                DataChannelState::Connecting
            } else if value == ffi::webrtc_DataChannelInterface_DataState_kOpen {
                DataChannelState::Open
            } else if value == ffi::webrtc_DataChannelInterface_DataState_kClosing {
                DataChannelState::Closing
            } else if value == ffi::webrtc_DataChannelInterface_DataState_kClosed {
                DataChannelState::Closed
            } else {
                DataChannelState::Unknown(value)
            }
        }
    }
}

/// DataChannel のラッパー。
pub struct DataChannel {
    raw_ref: ScopedRef<DataChannelHandle>,
}

impl DataChannel {
    pub(crate) fn from_scoped_ref(raw_ref: ScopedRef<DataChannelHandle>) -> Self {
        Self { raw_ref }
    }

    /// DataChannel のラベルを取得する。
    pub fn label(&self) -> Result<String> {
        let ptr =
            NonNull::new(unsafe { ffi::webrtc_DataChannelInterface_label(self.raw_ref.as_ptr()) })
                .expect("BUG: webrtc_DataChannelInterface_label が null を返しました");
        CxxString::from_unique(ptr).to_string()
    }

    /// DataChannel の状態を取得する。
    pub fn state(&self) -> DataChannelState {
        let state = unsafe { ffi::webrtc_DataChannelInterface_state(self.raw_ref.as_ptr()) };
        DataChannelState::from_int(state)
    }

    /// データを送信する。
    pub fn send(&self, data: &[u8], is_binary: bool) -> bool {
        let result = unsafe {
            ffi::webrtc_DataChannelInterface_Send(
                self.raw_ref.as_ptr(),
                data.as_ptr(),
                data.len(),
                if is_binary { 1 } else { 0 },
            )
        };
        result != 0
    }

    /// DataChannel を閉じる。
    pub fn close(&self) {
        unsafe { ffi::webrtc_DataChannelInterface_Close(self.raw_ref.as_ptr()) };
    }

    /// Observer を登録する。
    pub fn register_observer(&self, observer: &DataChannelObserver) {
        unsafe {
            ffi::webrtc_DataChannelInterface_RegisterObserver(
                self.raw_ref.as_ptr(),
                observer.as_ptr(),
            )
        };
    }

    /// Observer を解除する。
    pub fn unregister_observer(&self) {
        unsafe { ffi::webrtc_DataChannelInterface_UnregisterObserver(self.raw_ref.as_ptr()) };
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_DataChannelInterface {
        self.raw_ref.as_ptr()
    }
}

unsafe impl Send for DataChannel {}
// SAFETY: DataChannelInterface の実体はシーケンシャルにする Proxy 経由で
// アクセスするためスレッドセーフに使用できる。
// https://source.chromium.org/chromium/chromium/src/+/main:third_party/webrtc/pc/sctp_data_channel.cc;l=56-84;drc=b610a104128a46d031dc5ae3e6d486430b61efa6
unsafe impl Sync for DataChannel {}

// -------------------------
// DataChannelObserver
// -------------------------

#[allow(clippy::type_complexity)]
struct DataChannelCallbacks {
    on_state_change: Option<Box<dyn FnMut() + Send + 'static>>,
    on_message: Option<Box<dyn FnMut(&[u8], bool) + Send + 'static>>,
}

/// DataChannelObserver 用のコールバック設定。
#[allow(clippy::type_complexity)]
pub struct DataChannelObserverBuilder {
    on_state_change: Option<Box<dyn FnMut() + Send + 'static>>,
    on_message: Option<Box<dyn FnMut(&[u8], bool) + Send + 'static>>,
}

impl DataChannelObserverBuilder {
    pub fn new() -> Self {
        Self {
            on_state_change: None,
            on_message: None,
        }
    }

    pub fn on_state_change<F>(mut self, on_state_change: F) -> Self
    where
        F: FnMut() + Send + 'static,
    {
        self.on_state_change = Some(Box::new(on_state_change));
        self
    }

    pub fn on_message<F>(mut self, on_message: F) -> Self
    where
        F: FnMut(&[u8], bool) + Send + 'static,
    {
        self.on_message = Some(Box::new(on_message));
        self
    }

    pub fn build(self) -> DataChannelObserver {
        DataChannelObserver::new(self)
    }
}

impl Default for DataChannelObserverBuilder {
    fn default() -> Self {
        Self::new()
    }
}

unsafe extern "C" fn dc_observer_on_state_change(user_data: *mut c_void) {
    if user_data.is_null() {
        return;
    }
    let callbacks = unsafe { &mut *(user_data as *mut DataChannelCallbacks) };
    if let Some(cb) = callbacks.on_state_change.as_mut() {
        cb();
    }
}

unsafe extern "C" fn dc_observer_on_message(
    data: *const u8,
    len: usize,
    is_binary: i32,
    user_data: *mut c_void,
) {
    if user_data.is_null() {
        return;
    }
    let callbacks = unsafe { &mut *(user_data as *mut DataChannelCallbacks) };
    let slice = unsafe { slice::from_raw_parts(data, len) };
    if let Some(cb) = callbacks.on_message.as_mut() {
        cb(slice, is_binary != 0);
    }
}

/// DataChannelObserver のラッパー。
pub struct DataChannelObserver {
    raw: NonNull<ffi::webrtc_DataChannelObserver>,
    _cbs: *mut ffi::webrtc_DataChannelObserver_cbs,
    _user_data: *mut DataChannelCallbacks,
}

impl DataChannelObserver {
    fn new(handlers: DataChannelObserverBuilder) -> Self {
        let DataChannelObserverBuilder {
            on_state_change,
            on_message,
        } = handlers;
        let has_on_state_change = on_state_change.is_some();
        let has_on_message = on_message.is_some();
        let callbacks = Box::new(DataChannelCallbacks {
            on_state_change,
            on_message,
        });
        let user_data = Box::into_raw(callbacks) as *mut c_void;
        let cbs = Box::new(ffi::webrtc_DataChannelObserver_cbs {
            OnStateChange: if has_on_state_change {
                Some(dc_observer_on_state_change)
            } else {
                None
            },
            OnMessage: if has_on_message {
                Some(dc_observer_on_message)
            } else {
                None
            },
        });
        let cbs_ptr = Box::into_raw(cbs);
        let raw = unsafe { ffi::webrtc_DataChannelObserver_new(cbs_ptr, user_data) };
        let raw =
            NonNull::new(raw).expect("BUG: webrtc_DataChannelObserver_new が null を返しました");
        Self {
            raw,
            _cbs: cbs_ptr,
            _user_data: user_data as *mut DataChannelCallbacks,
        }
    }

    pub fn as_ptr(&self) -> *mut ffi::webrtc_DataChannelObserver {
        self.raw.as_ptr()
    }
}

impl Drop for DataChannelObserver {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_DataChannelObserver_delete(self.raw.as_ptr()) };
        // cbs と user_data の解放
        unsafe {
            let _ = Box::from_raw(self._cbs);
            let _ = Box::from_raw(self._user_data);
        };
    }
}

unsafe impl Send for DataChannelObserver {}
unsafe impl Sync for DataChannelObserver {}

// -------------------------
// DataChannelInit
// -------------------------

/// DataChannelInit のラッパー。
pub struct DataChannelInit {
    raw: NonNull<ffi::webrtc_DataChannelInit>,
}

impl DataChannelInit {
    pub fn new() -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_DataChannelInit_new() })
            .expect("BUG: webrtc_DataChannelInit_new が null を返しました");
        Self { raw }
    }

    pub fn set_ordered(&mut self, ordered: bool) {
        unsafe {
            ffi::webrtc_DataChannelInit_set_ordered(self.raw.as_ptr(), if ordered { 1 } else { 0 });
        }
    }

    pub fn set_protocol(&mut self, protocol: &str) {
        unsafe {
            ffi::webrtc_DataChannelInit_set_protocol(
                self.raw.as_ptr(),
                protocol.as_ptr() as *const _,
                protocol.len(),
            );
        }
    }

    pub fn as_ptr(&mut self) -> *mut ffi::webrtc_DataChannelInit {
        self.raw.as_ptr()
    }
}

impl Default for DataChannelInit {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for DataChannelInit {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_DataChannelInit_delete(self.raw.as_ptr()) };
    }
}
