use crate::ffi;

/// 固定配列の最大サイズや特殊値などの定数を集約するモジュール。
///
/// libwebrtc の `webrtc::` namespace 直下に定義されている定数に対応する。
/// クラスのメンバーとして定義されている定数は、該当クラスの関連関数として
/// 定義する。
///
/// このモジュールの関数は `shiguredo_webrtc::constants::` 経由でアクセスする。
/// `pub use constants::*` は行わないこと。
/// `simulcastStream` 配列の最大サイズ。
///
/// libwebrtc の `kMaxSimulcastStreams` に対応する。
pub fn max_simulcast_streams() -> usize {
    unsafe { ffi::webrtc_kMaxSimulcastStreams }
}

/// `spatialLayers` 配列の最大サイズ。
///
/// libwebrtc の `kMaxSpatialLayers` に対応する。
pub fn max_spatial_layers() -> usize {
    unsafe { ffi::webrtc_kMaxSpatialLayers }
}

/// VP9 の GOF (Group of Frames) に含められる最大フレーム数。
///
/// libwebrtc の `kMaxVp9FramesInGof` に対応する。
pub fn max_vp9_frames_in_gof() -> usize {
    unsafe { ffi::webrtc_kMaxVp9FramesInGof }
}

/// VP9 の参照フレーム数 (ref_index) の最大値。
///
/// libwebrtc の `kMaxVp9RefPics` に対応し、`GofInfoVP9` と
/// `RTPVideoHeaderVP9` の両方の配列サイズで共有する。
pub fn max_vp9_ref_pics() -> usize {
    unsafe { ffi::webrtc_kMaxVp9RefPics }
}

/// VP9 の空間レイヤー数の最大値。
///
/// libwebrtc の `kMaxVp9NumberOfSpatialLayers` に対応する。
pub fn max_vp9_num_spatial_layers() -> usize {
    unsafe { ffi::webrtc_kMaxVp9NumberOfSpatialLayers }
}

/// ピクチャ ID が存在しないことを表す特殊値。
///
/// libwebrtc の `kNoPictureId` に対応する。
pub fn no_picture_id() -> i16 {
    unsafe { ffi::webrtc_kNoPictureId }
}

/// TL0PICIDX が未指定であることを表す特殊値。
///
/// libwebrtc の `kNoTl0PicIdx` に対応する。
pub fn no_tl0_pic_idx() -> i16 {
    unsafe { ffi::webrtc_kNoTl0PicIdx }
}

/// 時間レイヤーインデックスが未指定であることを表す特殊値。
///
/// libwebrtc の `kNoTemporalIdx` に対応する。
pub fn no_temporal_idx() -> u8 {
    unsafe { ffi::webrtc_kNoTemporalIdx }
}

/// キーフレームインデックスが未使用であることを表す特殊値。
///
/// libwebrtc の `kNoKeyIdx` に対応する。
pub fn no_key_idx() -> i32 {
    unsafe { ffi::webrtc_kNoKeyIdx }
}
