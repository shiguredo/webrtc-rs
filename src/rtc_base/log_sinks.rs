//! ファイルへローテーション出力するログ sink（`rtc_base/log_sinks.h`）のラッパー。

use crate::ffi;
use crate::non_null::expect_non_null;
use crate::{Error, Result};
use std::ffi::CString;
use std::ptr::NonNull;

use crate::rtc_base::log::LogSink;

/// `webrtc::FileRotatingLogSink` のラッパー。
///
/// `log_dir_path` / `log_prefix` / `max_log_size` / `num_log_files` を指定して
/// ログをファイルへローテーション出力する。現在のファイルが `max_log_size` に
/// 達するとローテーションし、常に `num_log_files` 個（各 `max_log_size` 以下）の
/// ファイルを保持する。ローテーション時は最古のファイルが削除される。
/// 対照的に [CallSessionFileRotatingLogSink] はファイル数ではなく総出力サイズ
/// でローテーションし、中間のログを捨てて先頭と末尾を残す。
pub struct FileRotatingLogSink {
    raw_unique: NonNull<ffi::webrtc_FileRotatingLogSink_unique>,
}

unsafe impl Send for FileRotatingLogSink {}

impl FileRotatingLogSink {
    /// ファイルへローテーション出力する sink を生成する。
    ///
    /// 失敗した場合（ディレクトリを作成・開けない等）は `Err` を返す。
    pub fn new(
        log_dir_path: &str,
        log_prefix: &str,
        max_log_size: usize,
        num_log_files: usize,
    ) -> Result<Self> {
        let log_dir_path = CString::new(log_dir_path)?;
        let log_prefix = CString::new(log_prefix)?;
        let raw = unsafe {
            ffi::webrtc_FileRotatingLogSink_new(
                log_dir_path.as_ptr(),
                log_dir_path.as_bytes().len(),
                log_prefix.as_ptr(),
                log_prefix.as_bytes().len(),
                max_log_size,
                num_log_files,
            )
        };
        let raw_unique = expect_non_null(raw, "webrtc_FileRotatingLogSink_new");
        let this = Self { raw_unique };
        if !unsafe { ffi::webrtc_FileRotatingLogSink_Init(this.raw_unique.as_ptr()) } {
            // Init に失敗した場合は Self の Drop が unique_delete を呼ぶ。
            return Err(Error::FileRotatingLogSinkInit(
                log_dir_path.to_string_lossy().into_owned(),
            ));
        }
        Ok(this)
    }

    /// 基盤のストリームのバッファリングを無効化する。
    pub fn disable_buffering(&mut self) -> bool {
        unsafe { ffi::webrtc_FileRotatingLogSink_DisableBuffering(self.raw_unique.as_ptr()) }
    }

    /// 基底 [LogSink] へ変換する。
    pub fn into_base(self) -> LogSink {
        let raw = expect_non_null(self.raw(), "webrtc_FileRotatingLogSink_unique_get");
        let casted = expect_non_null(
            unsafe { ffi::webrtc_FileRotatingLogSink_cast_to_webrtc_LogSink(raw.as_ptr()) },
            "webrtc_FileRotatingLogSink_cast_to_webrtc_LogSink",
        );
        debug_assert_eq!(casted.as_ptr() as usize, raw.as_ptr() as usize);

        let raw_unique = std::mem::ManuallyDrop::new(self)
            .raw_unique
            .as_ptr()
            .cast::<ffi::webrtc_LogSink_unique>();
        LogSink::from_raw_unique(expect_non_null(
            raw_unique,
            "webrtc_FileRotatingLogSink_unique",
        ))
    }

    fn raw(&self) -> *mut ffi::webrtc_FileRotatingLogSink {
        unsafe { ffi::webrtc_FileRotatingLogSink_unique_get(self.raw_unique.as_ptr()) }
    }
}

impl Drop for FileRotatingLogSink {
    fn drop(&mut self) {
        unsafe { ffi::webrtc_FileRotatingLogSink_unique_delete(self.raw_unique.as_ptr()) };
    }
}

/// `webrtc::CallSessionFileRotatingLogSink` のラッパー。
///
/// `log_dir_path` / `max_total_log_size` を指定してログをファイルへローテーション
/// 出力する。[FileRotatingLogSink] との違いはローテーションの基準と、上限超過時に
/// 削除される部分である。本型はファイルごとのサイズ上限とファイル数ではなく
/// 「出力の総サイズ上限」でローテーションし、総サイズが上限を超えると中間の
/// ログが削除されて先頭と末尾のログが残る（コール診断ではログの先頭と末尾が
/// 有用なため）。
pub struct CallSessionFileRotatingLogSink {
    raw_unique: NonNull<ffi::webrtc_CallSessionFileRotatingLogSink_unique>,
}

unsafe impl Send for CallSessionFileRotatingLogSink {}

impl CallSessionFileRotatingLogSink {
    /// ファイルへローテーション出力する sink を生成する。
    ///
    /// 失敗した場合（ディレクトリを作成・開けない等）は `Err` を返す。
    pub fn new(log_dir_path: &str, max_total_log_size: usize) -> Result<Self> {
        let log_dir_path = CString::new(log_dir_path)?;
        let raw = unsafe {
            ffi::webrtc_CallSessionFileRotatingLogSink_new(
                log_dir_path.as_ptr(),
                log_dir_path.as_bytes().len(),
                max_total_log_size,
            )
        };
        let raw_unique = expect_non_null(raw, "webrtc_CallSessionFileRotatingLogSink_new");
        let this = Self { raw_unique };
        if !unsafe { ffi::webrtc_CallSessionFileRotatingLogSink_Init(this.raw_unique.as_ptr()) } {
            // Init に失敗した場合は Self の Drop が unique_delete を呼ぶ。
            return Err(Error::FileRotatingLogSinkInit(
                log_dir_path.to_string_lossy().into_owned(),
            ));
        }
        Ok(this)
    }

    /// 基盤のストリームのバッファリングを無効化する。
    pub fn disable_buffering(&mut self) -> bool {
        unsafe {
            ffi::webrtc_CallSessionFileRotatingLogSink_DisableBuffering(self.raw_unique.as_ptr())
        }
    }

    /// 基底 [LogSink] へ変換する。
    pub fn into_base(self) -> LogSink {
        let raw = expect_non_null(
            self.raw(),
            "webrtc_CallSessionFileRotatingLogSink_unique_get",
        );
        let casted = expect_non_null(
            unsafe {
                ffi::webrtc_CallSessionFileRotatingLogSink_cast_to_webrtc_LogSink(raw.as_ptr())
            },
            "webrtc_CallSessionFileRotatingLogSink_cast_to_webrtc_LogSink",
        );
        debug_assert_eq!(casted.as_ptr() as usize, raw.as_ptr() as usize);

        let raw_unique = std::mem::ManuallyDrop::new(self)
            .raw_unique
            .as_ptr()
            .cast::<ffi::webrtc_LogSink_unique>();
        LogSink::from_raw_unique(expect_non_null(
            raw_unique,
            "webrtc_CallSessionFileRotatingLogSink_unique",
        ))
    }

    fn raw(&self) -> *mut ffi::webrtc_CallSessionFileRotatingLogSink {
        unsafe { ffi::webrtc_CallSessionFileRotatingLogSink_unique_get(self.raw_unique.as_ptr()) }
    }
}

impl Drop for CallSessionFileRotatingLogSink {
    fn drop(&mut self) {
        unsafe {
            ffi::webrtc_CallSessionFileRotatingLogSink_unique_delete(self.raw_unique.as_ptr())
        };
    }
}
