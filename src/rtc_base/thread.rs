use crate::ffi;
use std::os::raw::c_void;
use std::ptr::NonNull;

unsafe extern "C" fn thread_trampoline<F, R>(data: *mut c_void)
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let func: Box<F> = unsafe { Box::from_raw(data as *mut F) };
    func();
}

unsafe extern "C" fn thread_trampoline_r<F, R>(data: *mut c_void) -> *mut c_void
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let func: Box<F> = unsafe { Box::from_raw(data as *mut F) };
    let res = func();
    let boxed_res = Box::new(res);
    Box::into_raw(boxed_res) as *mut c_void
}

/// webrtc::Thread のラッパー。
pub struct Thread {
    raw_unique: NonNull<ffi::webrtc_Thread_unique>,
}

unsafe impl Send for Thread {}

impl Thread {
    pub fn into_raw(self) -> *mut ffi::webrtc_Thread_unique {
        std::mem::ManuallyDrop::new(self).raw_unique.as_ptr()
    }

    /// ソケットサーバーなしでスレッドを生成する。
    pub fn new() -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_Thread_Create() })
            .expect("BUG: webrtc_Thread_Create が null を返しました");
        Self { raw_unique: raw }
    }

    /// ソケットサーバー付きでスレッドを生成する。
    pub fn new_with_socket_server() -> Self {
        let raw = NonNull::new(unsafe { ffi::webrtc_Thread_CreateWithSocketServer() })
            .expect("BUG: webrtc_Thread_CreateWithSocketServer が null を返しました");
        Self { raw_unique: raw }
    }

    /// スレッドを開始する。
    ///
    /// 起動に成功した場合は `true` を返す。
    pub fn start(&mut self) -> bool {
        let raw = self.raw();
        let result = unsafe { ffi::webrtc_Thread_Start(raw.as_ptr()) };
        result != 0
    }

    /// スレッドを停止し join する。
    pub fn stop(&mut self) {
        let raw = self.raw();
        unsafe { ffi::webrtc_Thread_Stop(raw.as_ptr()) };
    }

    /// スレッドのメッセージループを停止させる。
    ///
    /// `stop` とは異なり join しない。このスレッドに対する以降の Post / Send
    /// は失敗する。停止中に実行すると、`()` を返す `blocking_call` は
    /// クロージャを実行せずに即座に戻る。
    pub fn quit(&mut self) {
        let raw = self.raw();
        unsafe { ffi::webrtc_Thread_Quit(raw.as_ptr()) };
    }

    /// スレッド内で関数を実行し、結果を待つ。
    ///
    /// `f` は `self` のスレッドで実行される。基本的に呼び出し元と異なるスレッドで
    /// 実行されるため `F: Send` が必要である。また、`f` は `Box::into_raw` で
    /// C++ 側のタスクとして渡され、FFI 越しに消費されるため `F: 'static` を要求する。
    ///
    /// ```
    /// use shiguredo_webrtc::Thread;
    ///
    /// let mut thread = Thread::new();
    /// assert!(thread.start());
    /// let result = thread.blocking_call(|| 42);
    /// assert_eq!(result, 42);
    /// thread.stop();
    /// ```
    ///
    /// 非 Send な値をキャプチャしたクロージャは `F: Send` によりコンパイルエラーになる。
    ///
    /// ```compile_fail,E0277
    /// use std::rc::Rc;
    /// use shiguredo_webrtc::Thread;
    ///
    /// let mut thread = Thread::new();
    /// let value = Rc::new(42);
    /// thread.blocking_call(move || *value);
    /// ```
    ///
    /// ローカル変数を借用するクロージャは `F: 'static` によりコンパイルエラーになる。
    /// 借用ではなく所有する場合は `move` を使用すること。
    ///
    /// ```compile_fail,E0373
    /// use shiguredo_webrtc::Thread;
    ///
    /// let mut thread = Thread::new();
    /// let value = 42;
    /// thread.blocking_call(|| value);
    /// ```
    pub fn blocking_call<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce() -> R,
        F: Send + 'static,
        R: Send + 'static,
    {
        // R が () の場合は BlockingCall、そうでない場合は BlockingCall_r を使う。
        // 単純化のため、() とそれ以外を分ける。
        if std::mem::size_of::<R>() == 0 {
            let raw = self.raw();
            let boxed: Box<F> = Box::new(f);
            unsafe {
                ffi::webrtc_Thread_BlockingCall(
                    raw.as_ptr(),
                    Some(thread_trampoline::<F, R>),
                    Box::into_raw(boxed) as *mut c_void,
                );
            }
            // () を返す
            unsafe { std::mem::zeroed() }
        } else {
            let raw = self.raw();
            let boxed: Box<F> = Box::new(f);
            let res_ptr = unsafe {
                ffi::webrtc_Thread_BlockingCall_r(
                    raw.as_ptr(),
                    Some(thread_trampoline_r::<F, R>),
                    Box::into_raw(boxed) as *mut c_void,
                )
            };
            assert!(
                !res_ptr.is_null(),
                "webrtc_Thread_BlockingCall_r から null が返りました"
            );
            let boxed_res: Box<R> = unsafe { Box::from_raw(res_ptr as *mut R) };
            *boxed_res
        }
    }

    pub fn raw(&self) -> NonNull<ffi::webrtc_Thread> {
        let raw = unsafe { ffi::webrtc_Thread_unique_get(self.raw_unique.as_ptr()) };
        NonNull::new(raw).expect("BUG: webrtc_Thread_unique_get が null を返しました")
    }

    /// スレッドを一定時間スリープさせるヘルパー。
    pub fn sleep_ms(millis: i32) {
        unsafe { ffi::webrtc_Thread_SleepMs(millis) };
    }
}

impl Default for Thread {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_Thread_unique_delete(self.raw_unique.as_ptr()) };
    }
}
