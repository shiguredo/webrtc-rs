use crate::ffi;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LibyuvFourcc {
    Argb,
    Bgra,
}

impl LibyuvFourcc {
    fn as_raw(self) -> u32 {
        match self {
            LibyuvFourcc::Argb => unsafe { ffi::libyuv_FOURCC_ARGB },
            LibyuvFourcc::Bgra => unsafe { ffi::libyuv_FOURCC_BGRA },
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
