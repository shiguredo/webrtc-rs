pub mod log {
    use crate::Result;
    use crate::ffi;
    use crate::non_null::{expect_non_null, expect_non_null_with_cleanup};
    use std::ffi::CString;
    use std::marker::PhantomData;
    use std::os::raw::{c_char, c_void};
    use std::ptr::NonNull;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Severity {
        Verbose,
        Info,
        Warning,
        Error,
        None,
        Raw(i32),
    }

    impl Severity {
        pub fn from_int(value: i32) -> Self {
            if value == unsafe { ffi::webrtc_LogSeverity_LS_VERBOSE } {
                Severity::Verbose
            } else if value == unsafe { ffi::webrtc_LogSeverity_LS_INFO } {
                Severity::Info
            } else if value == unsafe { ffi::webrtc_LogSeverity_LS_WARNING } {
                Severity::Warning
            } else if value == unsafe { ffi::webrtc_LogSeverity_LS_ERROR } {
                Severity::Error
            } else if value == unsafe { ffi::webrtc_LogSeverity_LS_NONE } {
                Severity::None
            } else {
                Severity::Raw(value)
            }
        }

        pub fn to_int(self) -> i32 {
            match self {
                Severity::Verbose => unsafe { ffi::webrtc_LogSeverity_LS_VERBOSE },
                Severity::Info => unsafe { ffi::webrtc_LogSeverity_LS_INFO },
                Severity::Warning => unsafe { ffi::webrtc_LogSeverity_LS_WARNING },
                Severity::Error => unsafe { ffi::webrtc_LogSeverity_LS_ERROR },
                Severity::None => unsafe { ffi::webrtc_LogSeverity_LS_NONE },
                Severity::Raw(v) => v,
            }
        }
    }

    /// `webrtc::LoggingConfig` のラッパー。
    pub struct LoggingConfig {
        raw: NonNull<ffi::webrtc_LoggingConfig>,
    }

    unsafe impl Send for LoggingConfig {}

    impl LoggingConfig {
        pub fn new() -> Self {
            let raw = expect_non_null(
                unsafe { ffi::webrtc_LoggingConfig_new() },
                "webrtc_LoggingConfig_new",
            );
            Self { raw }
        }

        /// 重大度を取得する。この重大度より軽いメッセージは出力されない
        pub fn min_severity(&self) -> Severity {
            Severity::from_int(unsafe { ffi::webrtc_LoggingConfig_min_severity(self.raw.as_ptr()) })
        }

        /// 重大度を設定する。この重大度より軽いメッセージは出力されない
        pub fn set_min_severity(&mut self, severity: Severity) {
            unsafe {
                ffi::webrtc_LoggingConfig_set_min_severity(self.raw.as_ptr(), severity.to_int())
            };
        }

        /// 標準エラーへ出力する重大度を取得する。
        pub fn debug_severity(&self) -> Severity {
            Severity::from_int(unsafe {
                ffi::webrtc_LoggingConfig_debug_severity(self.raw.as_ptr())
            })
        }

        /// 標準エラーへ出力する重大度を設定する。
        pub fn set_debug_severity(&mut self, severity: Severity) {
            unsafe {
                ffi::webrtc_LoggingConfig_set_debug_severity(self.raw.as_ptr(), severity.to_int())
            };
        }

        /// スレッド ID の出力有無を取得する。
        pub fn log_thread(&self) -> bool {
            unsafe { ffi::webrtc_LoggingConfig_log_thread(self.raw.as_ptr()) != 0 }
        }

        /// スレッド ID の出力有無を設定する。
        pub fn set_log_thread(&mut self, enable: bool) {
            unsafe {
                ffi::webrtc_LoggingConfig_set_log_thread(
                    self.raw.as_ptr(),
                    if enable { 1 } else { 0 },
                )
            };
        }

        /// タイムスタンプの出力有無を取得する。
        pub fn log_timestamp(&self) -> bool {
            unsafe { ffi::webrtc_LoggingConfig_log_timestamp(self.raw.as_ptr()) != 0 }
        }

        /// タイムスタンプの出力有無を設定する。
        pub fn set_log_timestamp(&mut self, enable: bool) {
            unsafe {
                ffi::webrtc_LoggingConfig_set_log_timestamp(
                    self.raw.as_ptr(),
                    if enable { 1 } else { 0 },
                )
            };
        }

        /// キュー名の出力有無を取得する。
        pub fn log_queue_name(&self) -> bool {
            unsafe { ffi::webrtc_LoggingConfig_log_queue_name(self.raw.as_ptr()) != 0 }
        }

        /// キュー名の出力有無を設定する。
        pub fn set_log_queue_name(&mut self, enable: bool) {
            unsafe {
                ffi::webrtc_LoggingConfig_set_log_queue_name(
                    self.raw.as_ptr(),
                    if enable { 1 } else { 0 },
                )
            };
        }

        /// 標準エラーへの出力有無を取得する。
        pub fn log_to_stderr(&self) -> bool {
            unsafe { ffi::webrtc_LoggingConfig_log_to_stderr(self.raw.as_ptr()) != 0 }
        }

        /// 標準エラーへの出力有無を設定する。
        pub fn set_log_to_stderr(&mut self, enable: bool) {
            unsafe {
                ffi::webrtc_LoggingConfig_set_log_to_stderr(
                    self.raw.as_ptr(),
                    if enable { 1 } else { 0 },
                )
            };
        }

        /// ログ行の先頭に付けるプレフィックスを取得する。
        pub fn log_prefix(&self) -> Result<&str> {
            let mut ptr = std::ptr::null();
            let mut len = 0usize;
            unsafe { ffi::webrtc_LoggingConfig_log_prefix(self.raw.as_ptr(), &mut ptr, &mut len) };
            assert!(
                !ptr.is_null(),
                "BUG: webrtc_LoggingConfig_log_prefix が null を返しました"
            );
            let bytes = unsafe { std::slice::from_raw_parts(ptr.cast::<u8>(), len) };
            let prefix = std::str::from_utf8(bytes)?;
            Ok(prefix)
        }

        /// ログ行の先頭に付けるプレフィックスを設定する。
        pub fn set_log_prefix(&mut self, prefix: &str) {
            unsafe {
                ffi::webrtc_LoggingConfig_set_log_prefix(
                    self.raw.as_ptr(),
                    prefix.as_ptr().cast(),
                    prefix.len(),
                )
            };
        }

        pub fn as_ptr(&self) -> *mut ffi::webrtc_LoggingConfig {
            self.raw.as_ptr()
        }

        /// ログ受信用のカスタム sink を登録する。
        pub fn add_sink(&mut self, sink: LogSink) {
            unsafe { ffi::webrtc_LoggingConfig_AddSink(self.raw.as_ptr(), sink.into_raw()) };
        }
    }

    impl Default for LoggingConfig {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Drop for LoggingConfig {
        fn drop(&mut self) {
            unsafe { ffi::webrtc_LoggingConfig_delete(self.raw.as_ptr()) };
        }
    }

    /// ログの設定を初期化する。
    ///
    /// 最初のログ出力前に 1 回だけ呼ぶこと。すでに初期化済みの場合（明示的な
    /// 初期化あるいは最初のログ出力による暗黙の初期化を含む）は `false` を
    /// 返し、設定は反映されない。
    pub fn initialize_logging(config: LoggingConfig) -> bool {
        unsafe { ffi::webrtc_LogMessage_InitializeLogging(config.as_ptr()) }
    }

    /// 任意メッセージを出力する。
    pub fn print(severity: Severity, file: &str, line: i32, message: &str) {
        let Ok(file) = CString::new(file) else {
            return;
        };
        let Ok(msg) = CString::new(message) else {
            return;
        };
        unsafe {
            ffi::webrtc_LogMessage_Print(severity.to_int(), file.as_ptr(), line, msg.as_ptr())
        };
    }

    /// `webrtc::LogLineRef` の借用ラッパー。
    ///
    /// libwebrtc から sink へ渡されるログ 1 行分の情報を表す。各アクセサは
    /// 一時的な文字列ビューを返す。
    #[derive(Clone, Copy)]
    pub struct LogLineRef<'a> {
        raw: NonNull<ffi::webrtc_LogLineRef>,
        _marker: PhantomData<&'a ffi::webrtc_LogLineRef>,
    }

    unsafe impl<'a> Send for LogLineRef<'a> {}

    impl<'a> LogLineRef<'a> {
        pub fn from_raw(raw: NonNull<ffi::webrtc_LogLineRef>) -> Self {
            Self {
                raw,
                _marker: PhantomData,
            }
        }

        pub fn as_ptr(&self) -> *mut ffi::webrtc_LogLineRef {
            self.raw.as_ptr()
        }

        /// ログ本体のメッセージを返す。
        ///
        /// 非 UTF-8 が渡された場合は空文字列を返す。
        pub fn message(&self) -> &str {
            let mut ptr = std::ptr::null();
            let mut len = 0usize;
            unsafe { ffi::webrtc_LogLineRef_message(self.raw.as_ptr(), &mut ptr, &mut len) };
            cstr_slice(ptr, len)
        }

        /// ログ 1 行をデフォルト形式で整形した文字列を返す。
        pub fn default_log_line(&self) -> Result<String> {
            let raw = expect_non_null(
                unsafe { ffi::webrtc_LogLineRef_DefaultLogLine(self.raw.as_ptr()) },
                "webrtc_LogLineRef_DefaultLogLine",
            );
            crate::CxxString::from_unique(raw).to_string()
        }

        /// ログを発行したソースファイル名を返す。
        pub fn filename(&self) -> &str {
            let mut ptr = std::ptr::null();
            let mut len = 0usize;
            unsafe { ffi::webrtc_LogLineRef_filename(self.raw.as_ptr(), &mut ptr, &mut len) };
            cstr_slice(ptr, len)
        }

        /// ログを発行したソース行番号を返す。
        pub fn line(&self) -> i32 {
            unsafe { ffi::webrtc_LogLineRef_line(self.raw.as_ptr()) }
        }

        /// ログを発行したスレッド ID を返す。
        pub fn thread_id(&self) -> Option<i64> {
            let mut has = 0;
            let mut value = 0i64;
            unsafe { ffi::webrtc_LogLineRef_thread_id(self.raw.as_ptr(), &mut has, &mut value) };
            if has != 0 { Some(value) } else { None }
        }

        /// ログ発行時刻をマイクロ秒 (エポック起点) で返す。
        pub fn timestamp_us(&self) -> i64 {
            unsafe { ffi::webrtc_LogLineRef_timestamp(self.raw.as_ptr()) }
        }

        /// ログのタグを返す。
        pub fn tag(&self) -> &str {
            let mut ptr = std::ptr::null();
            let mut len = 0usize;
            unsafe { ffi::webrtc_LogLineRef_tag(self.raw.as_ptr(), &mut ptr, &mut len) };
            cstr_slice(ptr, len)
        }

        /// ログの重大度を返す。
        pub fn severity(&self) -> Severity {
            Severity::from_int(unsafe { ffi::webrtc_LogLineRef_severity(self.raw.as_ptr()) })
        }

        /// ログを発行したキュー名を返す。
        pub fn queue_name(&self) -> &str {
            let mut ptr = std::ptr::null();
            let mut len = 0usize;
            unsafe { ffi::webrtc_LogLineRef_queue_name(self.raw.as_ptr(), &mut ptr, &mut len) };
            cstr_slice(ptr, len)
        }
    }

    /// カスタム sink にログが届いたときのハンドラ。
    ///
    /// [LogSinkHandler::on_log_message] には、ログ 1 行分の全情報を持つ
    /// [LogLineRef] が渡される。
    pub trait LogSinkHandler: Send {
        #[expect(unused_variables)]
        fn on_log_message(&mut self, line: LogLineRef<'_>) {}
    }

    struct LogSinkHandlerState {
        handler: Box<dyn LogSinkHandler>,
    }

    unsafe impl Send for LogSinkHandlerState {}

    /// `webrtc::LogSink` のラッパー。
    pub struct LogSink {
        raw_unique: NonNull<ffi::webrtc_LogSink_unique>,
    }

    unsafe impl Send for LogSink {}

    impl LogSink {
        /// ハンドラを登録した sink を生成する。
        pub fn new_with_handler(handler: Box<dyn LogSinkHandler>) -> Self {
            let state = Box::new(LogSinkHandlerState { handler });
            let user_data = Box::into_raw(state) as *mut c_void;
            let cbs = ffi::webrtc_LogSink_cbs {
                OnLogMessage_log_line_ref: Some(log_sink_on_log_line_ref),
                OnDestroy: Some(log_sink_on_destroy),
            };
            let raw = unsafe { ffi::webrtc_LogSink_new(&cbs, user_data) };
            let raw_unique = expect_non_null_with_cleanup(raw, "webrtc_LogSink_new", || {
                // 生成に失敗した場合は渡したハンドラを回収する。
                let _ = unsafe { Box::from_raw(user_data as *mut LogSinkHandlerState) };
            });
            Self { raw_unique }
        }

        pub fn into_raw(self) -> *mut ffi::webrtc_LogSink_unique {
            std::mem::ManuallyDrop::new(self).raw_unique.as_ptr()
        }
    }

    impl Drop for LogSink {
        fn drop(&mut self) {
            unsafe { ffi::webrtc_LogSink_unique_delete(self.raw_unique.as_ptr()) };
        }
    }

    /// C 側が返す文字列ビュー (ポインタ + 長さ) を `&str` へ変換する。
    ///
    /// 非 UTF-8 が渡された場合は空文字列を返す。
    fn cstr_slice<'a>(ptr: *const c_char, len: usize) -> &'a str {
        if len == 0 {
            return "";
        }
        assert!(
            !ptr.is_null(),
            "C 側から null の文字列ビューを受け取りました"
        );
        let bytes = unsafe { std::slice::from_raw_parts(ptr.cast::<u8>(), len) };
        std::str::from_utf8(bytes).unwrap_or("")
    }

    fn handler_state<'a>(user_data: *mut c_void) -> &'a mut LogSinkHandlerState {
        assert!(
            !user_data.is_null(),
            "LogSink コールバックに null の user_data を渡しました"
        );
        unsafe { &mut *(user_data as *mut LogSinkHandlerState) }
    }

    unsafe extern "C" fn log_sink_on_destroy(user_data: *mut c_void) {
        let _ = unsafe { Box::from_raw(user_data as *mut LogSinkHandlerState) };
    }

    unsafe extern "C" fn log_sink_on_log_line_ref(
        line: *const ffi::webrtc_LogLineRef,
        user_data: *mut c_void,
    ) {
        let state = handler_state(user_data);
        let line = expect_non_null(line as *mut ffi::webrtc_LogLineRef, "webrtc_LogLineRef");
        state.handler.on_log_message(LogLineRef::from_raw(line));
    }
}

#[doc(hidden)]
pub fn rtc_log_format_file(crate_name: &str, file: &str) -> String {
    let file_name = std::path::Path::new(file)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(file);
    format!("{crate_name}::{file_name}")
}

#[macro_export]
macro_rules! rtc_log_verbose {
    ($($arg:tt)*) => {
        {
            let file = $crate::rtc_log_format_file(env!("CARGO_PKG_NAME"), file!());
            $crate::log::print(
                $crate::log::Severity::Verbose,
                &file,
                line!() as i32,
                &format!($($arg)*),
            )
        }
    };
}

#[macro_export]
macro_rules! rtc_log_info {
    ($($arg:tt)*) => {
        {
            let file = $crate::rtc_log_format_file(env!("CARGO_PKG_NAME"), file!());
            $crate::log::print(
                $crate::log::Severity::Info,
                &file,
                line!() as i32,
                &format!($($arg)*),
            )
        }
    };
}

#[macro_export]
macro_rules! rtc_log_warning {
    ($($arg:tt)*) => {
        {
            let file = $crate::rtc_log_format_file(env!("CARGO_PKG_NAME"), file!());
            $crate::log::print(
                $crate::log::Severity::Warning,
                &file,
                line!() as i32,
                &format!($($arg)*),
            )
        }
    };
}

#[macro_export]
macro_rules! rtc_log_error {
    ($($arg:tt)*) => {
        {
            let file = $crate::rtc_log_format_file(env!("CARGO_PKG_NAME"), file!());
            $crate::log::print(
                $crate::log::Severity::Error,
                &file,
                line!() as i32,
                &format!($($arg)*),
            )
        }
    };
}
