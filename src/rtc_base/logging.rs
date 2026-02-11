pub mod log {
    use crate::ffi;
    use std::ffi::CString;

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

    /// ログ出力先をデバッグに設定する。
    pub fn log_to_debug(severity: Severity) {
        unsafe { ffi::webrtc_LogMessage_LogToDebug(severity.to_int()) };
    }

    /// タイムスタンプ出力を有効化する。
    pub fn enable_timestamps() {
        unsafe { ffi::webrtc_LogMessage_LogTimestamps() };
    }

    /// スレッド名出力を有効化する。
    pub fn enable_threads() {
        unsafe { ffi::webrtc_LogMessage_LogThreads() };
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
