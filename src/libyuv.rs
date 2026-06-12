use crate::ffi;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LibyuvFourcc {
    Argb,
    Bgra,
    Mjpg,
}

impl LibyuvFourcc {
    fn as_raw(self) -> u32 {
        match self {
            LibyuvFourcc::Argb => unsafe { ffi::libyuv_FOURCC_ARGB },
            LibyuvFourcc::Bgra => unsafe { ffi::libyuv_FOURCC_BGRA },
            LibyuvFourcc::Mjpg => unsafe { ffi::libyuv_FOURCC_MJPG },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LibyuvRotationMode {
    Rotate0,
    Rotate90,
    Rotate180,
    Rotate270,
}

impl LibyuvRotationMode {
    fn as_raw(self) -> i32 {
        match self {
            LibyuvRotationMode::Rotate0 => unsafe { ffi::libyuv_kRotate0 },
            LibyuvRotationMode::Rotate90 => unsafe { ffi::libyuv_kRotate90 },
            LibyuvRotationMode::Rotate180 => unsafe { ffi::libyuv_kRotate180 },
            LibyuvRotationMode::Rotate270 => unsafe { ffi::libyuv_kRotate270 },
        }
    }
}

fn chroma_dimension(value: i32) -> Option<i32> {
    value.checked_add(1).map(|v| v / 2)
}

fn i420_chroma_size(width: i32, height: i32) -> Option<(i32, i32)> {
    Some((chroma_dimension(width)?, chroma_dimension(height)?))
}

fn required_plane_len(stride: i32, rows: i32, row_bytes: i32) -> Option<usize> {
    if stride <= 0 || rows <= 0 || row_bytes <= 0 {
        return None;
    }

    let stride = stride as usize;
    let rows = rows as usize;
    let row_bytes = row_bytes as usize;
    let last_row_offset = stride.checked_mul(rows.checked_sub(1)?)?;
    last_row_offset.checked_add(row_bytes)
}

fn has_required_len(len: usize, stride: i32, rows: i32, row_bytes: i32) -> bool {
    required_plane_len(stride, rows, row_bytes).is_some_and(|need| len >= need)
}

/// `libyuv::ConvertFromI420` を呼び出して、I420 を指定フォーマットへ変換する。
/// 変換に失敗した場合は `false` を返す。
#[allow(clippy::too_many_arguments)]
pub fn convert_from_i420(
    src_y: &[u8],
    src_stride_y: i32,
    src_u: &[u8],
    src_stride_u: i32,
    src_v: &[u8],
    src_stride_v: i32,
    dst_argb: &mut [u8],
    dst_stride_argb: i32,
    width: i32,
    height: i32,
    fourcc: LibyuvFourcc,
) -> bool {
    let Some((chroma_width, chroma_height)) = i420_chroma_size(width, height) else {
        return false;
    };
    let Some(dst_row_bytes_argb) = width.checked_mul(4) else {
        return false;
    };

    if !has_required_len(src_y.len(), src_stride_y, height, width)
        || !has_required_len(src_u.len(), src_stride_u, chroma_height, chroma_width)
        || !has_required_len(src_v.len(), src_stride_v, chroma_height, chroma_width)
        || !has_required_len(dst_argb.len(), dst_stride_argb, height, dst_row_bytes_argb)
    {
        return false;
    }

    unsafe {
        ffi::libyuv_ConvertFromI420(
            src_y.as_ptr(),
            src_stride_y,
            src_u.as_ptr(),
            src_stride_u,
            src_v.as_ptr(),
            src_stride_v,
            dst_argb.as_mut_ptr(),
            dst_stride_argb,
            width,
            height,
            fourcc.as_raw(),
        ) == 0
    }
}

/// `libyuv::I420ToNV12` を呼び出して、I420 を NV12 へ変換する。
/// 変換に失敗した場合は `false` を返す。
#[allow(clippy::too_many_arguments)]
pub fn i420_to_nv12(
    src_y: &[u8],
    src_stride_y: i32,
    src_u: &[u8],
    src_stride_u: i32,
    src_v: &[u8],
    src_stride_v: i32,
    dst_y: &mut [u8],
    dst_stride_y: i32,
    dst_uv: &mut [u8],
    dst_stride_uv: i32,
    width: i32,
    height: i32,
) -> bool {
    let Some((chroma_width, chroma_height)) = i420_chroma_size(width, height) else {
        return false;
    };
    let Some(dst_uv_row_bytes) = chroma_width.checked_mul(2) else {
        return false;
    };

    if !has_required_len(src_y.len(), src_stride_y, height, width)
        || !has_required_len(src_u.len(), src_stride_u, chroma_height, chroma_width)
        || !has_required_len(src_v.len(), src_stride_v, chroma_height, chroma_width)
        || !has_required_len(dst_y.len(), dst_stride_y, height, width)
        || !has_required_len(dst_uv.len(), dst_stride_uv, chroma_height, dst_uv_row_bytes)
    {
        return false;
    }

    unsafe {
        ffi::libyuv_I420ToNV12(
            src_y.as_ptr(),
            src_stride_y,
            src_u.as_ptr(),
            src_stride_u,
            src_v.as_ptr(),
            src_stride_v,
            dst_y.as_mut_ptr(),
            dst_stride_y,
            dst_uv.as_mut_ptr(),
            dst_stride_uv,
            width,
            height,
        ) == 0
    }
}

/// `libyuv::I420Copy` を呼び出して、I420 を I420 へコピーする。
/// コピーに失敗した場合は `false` を返す。
#[allow(clippy::too_many_arguments)]
pub fn i420_copy(
    src_y: &[u8],
    src_stride_y: i32,
    src_u: &[u8],
    src_stride_u: i32,
    src_v: &[u8],
    src_stride_v: i32,
    dst_y: &mut [u8],
    dst_stride_y: i32,
    dst_u: &mut [u8],
    dst_stride_u: i32,
    dst_v: &mut [u8],
    dst_stride_v: i32,
    width: i32,
    height: i32,
) -> bool {
    let Some((chroma_width, chroma_height)) = i420_chroma_size(width, height) else {
        return false;
    };

    if !has_required_len(src_y.len(), src_stride_y, height, width)
        || !has_required_len(src_u.len(), src_stride_u, chroma_height, chroma_width)
        || !has_required_len(src_v.len(), src_stride_v, chroma_height, chroma_width)
        || !has_required_len(dst_y.len(), dst_stride_y, height, width)
        || !has_required_len(dst_u.len(), dst_stride_u, chroma_height, chroma_width)
        || !has_required_len(dst_v.len(), dst_stride_v, chroma_height, chroma_width)
    {
        return false;
    }

    unsafe {
        ffi::libyuv_I420Copy(
            src_y.as_ptr(),
            src_stride_y,
            src_u.as_ptr(),
            src_stride_u,
            src_v.as_ptr(),
            src_stride_v,
            dst_y.as_mut_ptr(),
            dst_stride_y,
            dst_u.as_mut_ptr(),
            dst_stride_u,
            dst_v.as_mut_ptr(),
            dst_stride_v,
            width,
            height,
        ) == 0
    }
}

/// `libyuv::NV12Copy` を呼び出して、NV12 を NV12 へコピーする。
/// コピーに失敗した場合は `false` を返す。
#[allow(clippy::too_many_arguments)]
pub fn nv12_copy(
    src_y: &[u8],
    src_stride_y: i32,
    src_uv: &[u8],
    src_stride_uv: i32,
    dst_y: &mut [u8],
    dst_stride_y: i32,
    dst_uv: &mut [u8],
    dst_stride_uv: i32,
    width: i32,
    height: i32,
) -> bool {
    let Some((chroma_width, chroma_height)) = i420_chroma_size(width, height) else {
        return false;
    };
    let Some(uv_row_bytes) = chroma_width.checked_mul(2) else {
        return false;
    };

    if !has_required_len(src_y.len(), src_stride_y, height, width)
        || !has_required_len(src_uv.len(), src_stride_uv, chroma_height, uv_row_bytes)
        || !has_required_len(dst_y.len(), dst_stride_y, height, width)
        || !has_required_len(dst_uv.len(), dst_stride_uv, chroma_height, uv_row_bytes)
    {
        return false;
    }

    unsafe {
        ffi::libyuv_NV12Copy(
            src_y.as_ptr(),
            src_stride_y,
            src_uv.as_ptr(),
            src_stride_uv,
            dst_y.as_mut_ptr(),
            dst_stride_y,
            dst_uv.as_mut_ptr(),
            dst_stride_uv,
            width,
            height,
        ) == 0
    }
}

/// `libyuv::ABGRToI420` を呼び出して、ABGR から I420 へ変換する。
/// 変換に失敗した場合は `false` を返す。
#[allow(clippy::too_many_arguments)]
pub fn abgr_to_i420(
    src_abgr: &[u8],
    src_stride_abgr: i32,
    dst_y: &mut [u8],
    dst_stride_y: i32,
    dst_u: &mut [u8],
    dst_stride_u: i32,
    dst_v: &mut [u8],
    dst_stride_v: i32,
    width: i32,
    height: i32,
) -> bool {
    let Some((chroma_width, chroma_height)) = i420_chroma_size(width, height) else {
        return false;
    };
    let Some(src_abgr_row_bytes) = width.checked_mul(4) else {
        return false;
    };

    if !has_required_len(src_abgr.len(), src_stride_abgr, height, src_abgr_row_bytes)
        || !has_required_len(dst_y.len(), dst_stride_y, height, width)
        || !has_required_len(dst_u.len(), dst_stride_u, chroma_height, chroma_width)
        || !has_required_len(dst_v.len(), dst_stride_v, chroma_height, chroma_width)
    {
        return false;
    }

    unsafe {
        ffi::libyuv_ABGRToI420(
            src_abgr.as_ptr(),
            src_stride_abgr,
            dst_y.as_mut_ptr(),
            dst_stride_y,
            dst_u.as_mut_ptr(),
            dst_stride_u,
            dst_v.as_mut_ptr(),
            dst_stride_v,
            width,
            height,
        ) == 0
    }
}

/// `libyuv::NV12ToI420` を呼び出して、NV12 から I420 へ変換する。
/// 変換に失敗した場合は `false` を返す。
#[allow(clippy::too_many_arguments)]
pub fn nv12_to_i420(
    src_y: &[u8],
    src_stride_y: i32,
    src_uv: &[u8],
    src_stride_uv: i32,
    dst_y: &mut [u8],
    dst_stride_y: i32,
    dst_u: &mut [u8],
    dst_stride_u: i32,
    dst_v: &mut [u8],
    dst_stride_v: i32,
    width: i32,
    height: i32,
) -> bool {
    let Some((chroma_width, chroma_height)) = i420_chroma_size(width, height) else {
        return false;
    };
    let Some(src_uv_row_bytes) = chroma_width.checked_mul(2) else {
        return false;
    };

    if !has_required_len(src_y.len(), src_stride_y, height, width)
        || !has_required_len(src_uv.len(), src_stride_uv, chroma_height, src_uv_row_bytes)
        || !has_required_len(dst_y.len(), dst_stride_y, height, width)
        || !has_required_len(dst_u.len(), dst_stride_u, chroma_height, chroma_width)
        || !has_required_len(dst_v.len(), dst_stride_v, chroma_height, chroma_width)
    {
        return false;
    }

    unsafe {
        ffi::libyuv_NV12ToI420(
            src_y.as_ptr(),
            src_stride_y,
            src_uv.as_ptr(),
            src_stride_uv,
            dst_y.as_mut_ptr(),
            dst_stride_y,
            dst_u.as_mut_ptr(),
            dst_stride_u,
            dst_v.as_mut_ptr(),
            dst_stride_v,
            width,
            height,
        ) == 0
    }
}

/// `libyuv::YUY2ToI420` を呼び出して、YUY2 から I420 へ変換する。
/// 変換に失敗した場合は `false` を返す。
#[allow(clippy::too_many_arguments)]
pub fn yuy2_to_i420(
    src_yuy2: &[u8],
    src_stride_yuy2: i32,
    dst_y: &mut [u8],
    dst_stride_y: i32,
    dst_u: &mut [u8],
    dst_stride_u: i32,
    dst_v: &mut [u8],
    dst_stride_v: i32,
    width: i32,
    height: i32,
) -> bool {
    let Some((chroma_width, chroma_height)) = i420_chroma_size(width, height) else {
        return false;
    };
    let Some(src_yuy2_row_bytes) = width.checked_mul(2) else {
        return false;
    };

    if !has_required_len(src_yuy2.len(), src_stride_yuy2, height, src_yuy2_row_bytes)
        || !has_required_len(dst_y.len(), dst_stride_y, height, width)
        || !has_required_len(dst_u.len(), dst_stride_u, chroma_height, chroma_width)
        || !has_required_len(dst_v.len(), dst_stride_v, chroma_height, chroma_width)
    {
        return false;
    }

    unsafe {
        ffi::libyuv_YUY2ToI420(
            src_yuy2.as_ptr(),
            src_stride_yuy2,
            dst_y.as_mut_ptr(),
            dst_stride_y,
            dst_u.as_mut_ptr(),
            dst_stride_u,
            dst_v.as_mut_ptr(),
            dst_stride_v,
            width,
            height,
        ) == 0
    }
}

/// MJPEG バイト列を I420 へ変換する。
///
/// - `sample`: MJPG (baseline JPEG) のバイト列。
/// - `src_width` / `src_height`: 入力 MJPG にエンコードされている元解像度。
///   JPEG ヘッダの値と一致しない場合、libyuv 内部で失敗扱いとなり `false` を返す。
/// - `dst_width` / `dst_height`: 出力 I420 の解像度。
///   libyuv は `dst_width == src_width` 必須かつ `dst_height <= src_height` のみ
///   受け付ける (vertical crop のみ可、水平リサイズ・スケーリング不可)。
///
/// 変換に成功した場合 (libyuv が `0` を返した場合) のみ `true` を返す。
/// それ以外 (libyuv が `-1` または `1` を返した場合、事前検証に失敗した場合) は `false`。
/// iOS など MJPEG サポートを含まないビルドでは常に `false` を返す。
#[allow(clippy::too_many_arguments)]
pub fn mjpg_to_i420(
    sample: &[u8],
    dst_y: &mut [u8],
    dst_stride_y: i32,
    dst_u: &mut [u8],
    dst_stride_u: i32,
    dst_v: &mut [u8],
    dst_stride_v: i32,
    src_width: i32,
    src_height: i32,
    dst_width: i32,
    dst_height: i32,
) -> bool {
    let Some((chroma_width, chroma_height)) = i420_chroma_size(dst_width, dst_height) else {
        return false;
    };

    if !has_required_len(dst_y.len(), dst_stride_y, dst_height, dst_width)
        || !has_required_len(dst_u.len(), dst_stride_u, chroma_height, chroma_width)
        || !has_required_len(dst_v.len(), dst_stride_v, chroma_height, chroma_width)
    {
        return false;
    }

    unsafe {
        ffi::libyuv_MJPGToI420(
            sample.as_ptr(),
            sample.len(),
            dst_y.as_mut_ptr(),
            dst_stride_y,
            dst_u.as_mut_ptr(),
            dst_stride_u,
            dst_v.as_mut_ptr(),
            dst_stride_v,
            src_width,
            src_height,
            dst_width,
            dst_height,
        ) == 0
    }
}

/// MJPEG バイト列を NV12 へ変換する。
///
/// 出力形式以外の、入力、制約、戻り値の仕様は `mjpg_to_i420` と同じ。
#[allow(clippy::too_many_arguments)]
pub fn mjpg_to_nv12(
    sample: &[u8],
    dst_y: &mut [u8],
    dst_stride_y: i32,
    dst_uv: &mut [u8],
    dst_stride_uv: i32,
    src_width: i32,
    src_height: i32,
    dst_width: i32,
    dst_height: i32,
) -> bool {
    let Some((chroma_width, chroma_height)) = i420_chroma_size(dst_width, dst_height) else {
        return false;
    };
    let Some(dst_uv_row_bytes) = chroma_width.checked_mul(2) else {
        return false;
    };

    if !has_required_len(dst_y.len(), dst_stride_y, dst_height, dst_width)
        || !has_required_len(dst_uv.len(), dst_stride_uv, chroma_height, dst_uv_row_bytes)
    {
        return false;
    }

    unsafe {
        ffi::libyuv_MJPGToNV12(
            sample.as_ptr(),
            sample.len(),
            dst_y.as_mut_ptr(),
            dst_stride_y,
            dst_uv.as_mut_ptr(),
            dst_stride_uv,
            src_width,
            src_height,
            dst_width,
            dst_height,
        ) == 0
    }
}

/// MJPEG バイト列から画像の幅と高さを取得する。
///
/// 成功した場合は `Some((width, height))` を返す。
/// iOS など MJPEG サポートを含まないビルドでは常に `None` を返す。
pub fn mjpg_size(sample: &[u8]) -> Option<(i32, i32)> {
    let mut width: i32 = 0;
    let mut height: i32 = 0;
    unsafe {
        if ffi::libyuv_MJPGSize(sample.as_ptr(), sample.len(), &mut width, &mut height) == 0 {
            Some((width, height))
        } else {
            None
        }
    }
}

/// 任意のフォーマットから I420 へ変換する。
///
/// `fourcc` で入力フォーマットを指定する。
/// MJPEG 入力の場合、src_width/src_height が JPEG ヘッダの値と一致する必要がある。
/// iOS など MJPEG サポートを含まないビルドでは MJPG fourcc 指定時の変換は常に失敗する
///
/// 変換に成功した場合 (libyuv が `0` を返した場合) のみ `true` を返す。
#[allow(clippy::too_many_arguments)]
pub fn convert_to_i420(
    src_frame: &[u8],
    dst_y: &mut [u8],
    dst_stride_y: i32,
    dst_u: &mut [u8],
    dst_stride_u: i32,
    dst_v: &mut [u8],
    dst_stride_v: i32,
    crop_x: i32,
    crop_y: i32,
    src_width: i32,
    src_height: i32,
    crop_width: i32,
    crop_height: i32,
    rotation: LibyuvRotationMode,
    fourcc: LibyuvFourcc,
) -> bool {
    let Some((chroma_width, chroma_height)) = i420_chroma_size(crop_width, crop_height) else {
        return false;
    };

    if !has_required_len(dst_y.len(), dst_stride_y, crop_height, crop_width)
        || !has_required_len(dst_u.len(), dst_stride_u, chroma_height, chroma_width)
        || !has_required_len(dst_v.len(), dst_stride_v, chroma_height, chroma_width)
    {
        return false;
    }

    unsafe {
        ffi::libyuv_ConvertToI420(
            src_frame.as_ptr(),
            src_frame.len(),
            dst_y.as_mut_ptr(),
            dst_stride_y,
            dst_u.as_mut_ptr(),
            dst_stride_u,
            dst_v.as_mut_ptr(),
            dst_stride_v,
            crop_x,
            crop_y,
            src_width,
            src_height,
            crop_width,
            crop_height,
            rotation.as_raw(),
            fourcc.as_raw(),
        ) == 0
    }
}
