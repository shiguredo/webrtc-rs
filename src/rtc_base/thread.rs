use crate::ffi;
use crate::helper::non_null::expect_non_null;
use std::os::raw::c_void;
use std::ptr::NonNull;

unsafe extern "C" fn thread_trampoline<F, R>(data: *mut c_void)
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let arg = unsafe { &mut *(data as *mut Option<F>) };
    // クロージャの所有権は呼び出し元が持ったまま渡され、ここで取り出す。
    // 停止中のスレッドではこの関数が呼ばれないため、呼び出し元が回収して破棄する。
    let func = arg.take().expect("thread_trampoline: closure is not set");
    func();
}

unsafe extern "C" fn thread_trampoline_r<F, R>(data: *mut c_void) -> *mut c_void
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let arg = unsafe { &mut *(data as *mut Option<F>) };
    let func = arg.take().expect("thread_trampoline_r: closure is not set");
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
        let raw = expect_non_null(
            unsafe { ffi::webrtc_Thread_Create() },
            "webrtc_Thread_Create",
        );
        Self { raw_unique: raw }
    }

    /// ソケットサーバー付きでスレッドを生成する。
    pub fn new_with_socket_server() -> Self {
        let raw = expect_non_null(
            unsafe { ffi::webrtc_Thread_CreateWithSocketServer() },
            "webrtc_Thread_CreateWithSocketServer",
        );
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
    /// `stop` とは異なり join しないため、停止要求を出した直後に `start` で再開することは
    /// できない。再開する場合は `stop` で join してから `start` を呼び出すこと。このスレッドに
    /// 対する以降の Post / Send は失敗する。停止後に [Thread::blocking_call] を呼び出しては
    /// ならない。
    pub fn quit(&mut self) {
        let raw = self.raw();
        unsafe { ffi::webrtc_Thread_Quit(raw.as_ptr()) };
    }

    /// スレッドに停止要求が出ているかどうかを返す。
    ///
    /// `quit` / `stop` の後は `true` になる。`stop` で join したスレッドを `start` で再開すると
    /// `false` に戻る。停止中のスレッドでは [Thread::blocking_call] を呼び出せないため、
    /// 呼び出す前にこの関数で確認する。
    pub fn is_quitting(&self) -> bool {
        let raw = self.raw();
        unsafe { ffi::webrtc_Thread_IsQuitting(raw.as_ptr()) != 0 }
    }

    /// スレッド内で関数を実行し、結果を待つ。
    ///
    /// `f` は `self` のスレッドで実行される。基本的に呼び出し元と異なるスレッドで
    /// 実行されるため `F: Send` が必要である。また、`f` は `Box::into_raw` で
    /// C++ 側のタスクとして渡され、FFI 越しに消費されるため `F: 'static` を要求する。
    ///
    /// 停止中 (`quit` / `stop` 後) のスレッドに対して呼び出してはならない。libwebrtc の
    /// `BlockingCall` は停止中のスレッドからの呼び出しを契約違反として扱い、デバッグビルドでは
    /// `RTC_DCHECK` により検出されてプロセスが異常終了する。停止中かどうかは
    /// [Thread::is_quitting] で確認できる。
    ///
    /// 契約違反の呼び出しではクロージャは実行されず、その場合は `f` の結果の代わりに
    /// `R::default()` が返るため `R: Default` を要求する。クロージャは実行の有無にかかわらず
    /// 破棄される。
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
        R: Default,
    {
        // クロージャは Option に包んで渡し、対象スレッドで実行された場合はそこで取り出される。
        // 停止中のスレッドではクロージャが実行されないため、実行されなかったクロージャは
        // ここで回収して破棄する（リークさせない）。
        //
        // R が () の場合は BlockingCall、そうでない場合は BlockingCall_r を使う。
        // 単純化のため、() とそれ以外を分ける。
        if std::mem::size_of::<R>() == 0 {
            let raw = self.raw();
            let arg = Box::into_raw(Box::new(Some(f)));
            unsafe {
                ffi::webrtc_Thread_BlockingCall(
                    raw.as_ptr(),
                    Some(thread_trampoline::<F, R>),
                    arg as *mut c_void,
                );
            }
            drop(unsafe { Box::from_raw(arg) });
            // () を返す
            R::default()
        } else {
            let raw = self.raw();
            let arg = Box::into_raw(Box::new(Some(f)));
            let res_ptr = unsafe {
                ffi::webrtc_Thread_BlockingCall_r(
                    raw.as_ptr(),
                    Some(thread_trampoline_r::<F, R>),
                    arg as *mut c_void,
                )
            };
            drop(unsafe { Box::from_raw(arg) });
            // 停止中スレッドでは functor が実行されず nullptr（未実行）が返る。C++ 側の非 void
            // テンプレートは未実行時に未初期化の値を返すため、C ラッパーが nullptr に正規化して
            // いる（webrtc_c/rtc_base/thread.cc）。実行経路 (thread_trampoline_r) は常に非 null を
            // 返すため、nullptr は未実行を一意に表す。未実行時はデフォルト値を返す。
            if res_ptr.is_null() {
                R::default()
            } else {
                let boxed_res: Box<R> = unsafe { Box::from_raw(res_ptr as *mut R) };
                *boxed_res
            }
        }
    }

    pub fn raw(&self) -> NonNull<ffi::webrtc_Thread> {
        let raw = unsafe { ffi::webrtc_Thread_unique_get(self.raw_unique.as_ptr()) };
        expect_non_null(raw, "webrtc_Thread_unique_get")
    }

    /// このスレッドへの、所有権を持たないハンドルを返す。
    ///
    /// 返り値はこのスレッドの停止や破棄を行わない。
    ///
    /// # Safety
    ///
    /// 返り値は `self` より長く生存させないこと。
    pub(crate) unsafe fn handle(&self) -> ThreadHandle {
        ThreadHandle::from_raw(self.raw())
    }

    /// スレッドを一定時間スリープさせるヘルパー。
    ///
    /// スリープがシグナルで中断された場合は `false` を返す（POSIX のみ）。
    pub fn sleep_ms(millis: i32) -> bool {
        let result = unsafe { ffi::webrtc_Thread_SleepMs(millis) };
        result != 0
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

/// [Thread] が所有するスレッドへの、所有権を持たないハンドル。
///
/// スレッドの停止や破棄は行わない。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ThreadHandle {
    raw: NonNull<ffi::webrtc_Thread>,
}

unsafe impl Send for ThreadHandle {}

impl ThreadHandle {
    pub(crate) fn from_raw(raw: NonNull<ffi::webrtc_Thread>) -> Self {
        Self { raw }
    }

    /// スレッドの生ポインタを返す。
    pub(crate) fn raw(&self) -> NonNull<ffi::webrtc_Thread> {
        self.raw
    }

    /// スレッド内で関数を実行し、結果を待つ。
    ///
    /// `f` はこのハンドルが指すスレッドで実行される。基本的に呼び出し元と異なるスレッドで
    /// 実行されるため `F: Send` を要求し、`f` は FFI 越しに消費されるため `F: 'static` を
    /// 要求する。
    ///
    /// 停止中 (`quit` / `stop` 後) のスレッドに対して呼び出してはならない。libwebrtc の
    /// `BlockingCall` は停止中のスレッドからの呼び出しを契約違反として扱い、デバッグビルドでは
    /// `RTC_DCHECK` により検出されてプロセスが異常終了する。
    ///
    /// 契約違反の呼び出しではクロージャは実行されず、その場合は `f` の結果の代わりに
    /// `R::default()` が返るため `R: Default` を要求する。クロージャは実行の有無にかかわらず
    /// 破棄される。
    pub(crate) fn blocking_call<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R,
        F: Send + 'static,
        R: Send + 'static,
        R: Default,
    {
        if std::mem::size_of::<R>() == 0 {
            let arg = Box::into_raw(Box::new(Some(f)));
            unsafe {
                ffi::webrtc_Thread_BlockingCall(
                    self.raw.as_ptr(),
                    Some(thread_trampoline::<F, R>),
                    arg as *mut c_void,
                );
            }
            drop(unsafe { Box::from_raw(arg) });
            R::default()
        } else {
            let arg = Box::into_raw(Box::new(Some(f)));
            let res_ptr = unsafe {
                ffi::webrtc_Thread_BlockingCall_r(
                    self.raw.as_ptr(),
                    Some(thread_trampoline_r::<F, R>),
                    arg as *mut c_void,
                )
            };
            drop(unsafe { Box::from_raw(arg) });
            if res_ptr.is_null() {
                R::default()
            } else {
                let boxed_res: Box<R> = unsafe { Box::from_raw(res_ptr as *mut R) };
                *boxed_res
            }
        }
    }
}
