pub mod log {
    use crate::Result;
    use crate::ffi;
    use crate::non_null::expect_non_null;
    use std::ffi::CString;
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
