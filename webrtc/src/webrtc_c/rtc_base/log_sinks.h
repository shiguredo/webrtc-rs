#pragma once

#include <stdbool.h>
#include <stddef.h>

#include "../common.h"
#include "../std.h"
#include "logging.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// rtc_base/log_sinks
// -------------------------

// webrtc::FileRotatingLogSink の C ラッパー。
//
// 現在のファイルが max_log_size に達するとローテーションし、常に num_log_files 個
// （各 max_log_size 以下）のファイルを保持する。ローテーション時は最古のファイルが
// 削除される。対照的に CallSessionFileRotatingLogSink はファイル数ではなく総出力
// サイズでローテーションし、中間のログを捨てて先頭と末尾を残す。
//
// 利用する前に webrtc_FileRotatingLogSink_Init を呼ぶ必要がある。

WEBRTC_DECLARE_UNIQUE(webrtc_FileRotatingLogSink);
WEBRTC_DECLARE_CAST(webrtc_FileRotatingLogSink, webrtc_LogSink);

WEBRTC_EXPORT struct webrtc_FileRotatingLogSink_unique*
webrtc_FileRotatingLogSink_new(const char* log_dir_path,
                               size_t log_dir_path_len,
                               const char* log_prefix,
                               size_t log_prefix_len,
                               size_t max_log_size,
                               size_t num_log_files);

// Init() を呼んでファイルを準備する。成功したら true を返す。
WEBRTC_EXPORT bool webrtc_FileRotatingLogSink_Init(
    struct webrtc_FileRotatingLogSink_unique* self);

// 基盤のストリームのバッファリングを無効化する。成功したら true を返す。
WEBRTC_EXPORT bool webrtc_FileRotatingLogSink_DisableBuffering(
    struct webrtc_FileRotatingLogSink_unique* self);

// webrtc::CallSessionFileRotatingLogSink の C ラッパー。
//
// FileRotatingLogSink との違いはローテーションの基準と、上限超過時に削除される部分で、
// CallSessionFileRotatingLogSink は「ファイルごとのサイズ上限 (max_log_size) と
// ファイル数 (num_log_files)」ではなく「出力の総サイズ上限 (max_total_log_size)」
// でローテーションする。総サイズが上限を超えると中間のログが削除され、先頭と
// 末尾のログが残る（コール診断ではログの先頭と末尾が有用なため）。対照的に
// FileRotatingLogSink はローテーション時に最古のファイルを削除する。
//
// 利用する前に webrtc_CallSessionFileRotatingLogSink_Init を呼ぶ必要がある。

WEBRTC_DECLARE_UNIQUE(webrtc_CallSessionFileRotatingLogSink);
WEBRTC_DECLARE_CAST(webrtc_CallSessionFileRotatingLogSink, webrtc_LogSink);

WEBRTC_EXPORT struct webrtc_CallSessionFileRotatingLogSink_unique*
webrtc_CallSessionFileRotatingLogSink_new(const char* log_dir_path,
                                          size_t log_dir_path_len,
                                          size_t max_total_log_size);

WEBRTC_EXPORT bool webrtc_CallSessionFileRotatingLogSink_Init(
    struct webrtc_CallSessionFileRotatingLogSink_unique* self);

WEBRTC_EXPORT bool webrtc_CallSessionFileRotatingLogSink_DisableBuffering(
    struct webrtc_CallSessionFileRotatingLogSink_unique* self);

#if defined(__cplusplus)
}
#endif
