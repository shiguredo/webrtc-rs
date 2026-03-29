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

fn plane_len(stride: i32, rows: i32) -> Option<usize> {
    if stride < 0 || rows < 0 {
        return None;
    }
    (stride as usize).checked_mul(rows as usize)
}

/// `libyuv::ConvertFromI420` を呼び出して、I420 を指定フォーマットへ変換する。
/// 変換に失敗した場合は `None` を返す。
#[allow(clippy::too_many_arguments)]
pub fn convert_from_i420(
    src_y: &[u8],
    src_stride_y: i32,
    src_u: &[u8],
    src_stride_u: i32,
    src_v: &[u8],
    src_stride_v: i32,
    width: i32,
    height: i32,
    fourcc: LibyuvFourcc,
) -> Option<Vec<u8>> {
    if width <= 0 || height <= 0 {
        return None;
    }
    let chroma_width = chroma_dimension(width)?;
    let chroma_height = chroma_dimension(height)?;
    if src_stride_y < width || src_stride_u < chroma_width || src_stride_v < chroma_width {
        return None;
    }
    if src_y.len() < plane_len(src_stride_y, height)?
        || src_u.len() < plane_len(src_stride_u, chroma_height)?
        || src_v.len() < plane_len(src_stride_v, chroma_height)?
    {
        return None;
    }
    let dst_stride = width.checked_mul(4)?;
    let dst_size = plane_len(dst_stride, height)?;
    let mut dst = vec![0u8; dst_size];

    let ret = unsafe {
        ffi::libyuv_ConvertFromI420(
            src_y.as_ptr(),
            src_stride_y,
            src_u.as_ptr(),
            src_stride_u,
            src_v.as_ptr(),
            src_stride_v,
            dst.as_mut_ptr(),
            dst_stride,
            width,
            height,
            fourcc.as_raw(),
        )
    };
    if ret != 0 {
        return None;
    }
    Some(dst)
}

/// `libyuv::I420ToNV12` を呼び出して、I420 を NV12 へ変換する。
/// 変換に失敗した場合は `None` を返す。
#[allow(clippy::too_many_arguments)]
pub fn i420_to_nv12(
    src_y: &[u8],
    src_stride_y: i32,
    src_u: &[u8],
    src_stride_u: i32,
    src_v: &[u8],
    src_stride_v: i32,
    width: i32,
    height: i32,
) -> Option<(Vec<u8>, Vec<u8>)> {
    if width <= 0 || height <= 0 {
        return None;
    }
    let chroma_width = chroma_dimension(width)?;
    let chroma_height = chroma_dimension(height)?;
    if src_stride_y < width || src_stride_u < chroma_width || src_stride_v < chroma_width {
        return None;
    }
    if src_y.len() < plane_len(src_stride_y, height)?
        || src_u.len() < plane_len(src_stride_u, chroma_height)?
        || src_v.len() < plane_len(src_stride_v, chroma_height)?
    {
        return None;
    }

    let dst_stride_y = width;
    let dst_stride_uv = chroma_width.checked_mul(2)?;
    let mut dst_y = vec![0u8; plane_len(dst_stride_y, height)?];
    let mut dst_uv = vec![0u8; plane_len(dst_stride_uv, chroma_height)?];

    let ret = unsafe {
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
        )
    };
    if ret != 0 {
        return None;
    }
    Some((dst_y, dst_uv))
}

/// `libyuv::ABGRToI420` を呼び出して、ABGR から I420 へ変換する。
/// 変換に失敗した場合は `None` を返す。
pub fn abgr_to_i420(
    src_abgr: &[u8],
    src_stride_abgr: i32,
    width: i32,
    height: i32,
) -> Option<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    if width <= 0 || height <= 0 {
        return None;
    }
    let src_min_stride_abgr = width.checked_mul(4)?;
    if src_stride_abgr < src_min_stride_abgr {
        return None;
    }
    if src_abgr.len() < plane_len(src_stride_abgr, height)? {
        return None;
    }
    let chroma_width = chroma_dimension(width)?;
    let chroma_height = chroma_dimension(height)?;

    let dst_stride_y = width;
    let dst_stride_u = chroma_width;
    let dst_stride_v = chroma_width;
    let mut dst_y = vec![0u8; plane_len(dst_stride_y, height)?];
    let mut dst_u = vec![0u8; plane_len(dst_stride_u, chroma_height)?];
    let mut dst_v = vec![0u8; plane_len(dst_stride_v, chroma_height)?];

    let ret = unsafe {
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
        )
    };
    if ret != 0 {
        return None;
    }
    Some((dst_y, dst_u, dst_v))
}

/// `libyuv::NV12ToI420` を呼び出して、NV12 から I420 へ変換する。
/// 変換に失敗した場合は `None` を返す。
pub fn nv12_to_i420(
    src_y: &[u8],
    src_stride_y: i32,
    src_uv: &[u8],
    src_stride_uv: i32,
    width: i32,
    height: i32,
) -> Option<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    if width <= 0 || height <= 0 {
        return None;
    }
    let chroma_width = chroma_dimension(width)?;
    let chroma_height = chroma_dimension(height)?;
    let src_min_stride_uv = chroma_width.checked_mul(2)?;
    if src_stride_y < width || src_stride_uv < src_min_stride_uv {
        return None;
    }
    if src_y.len() < plane_len(src_stride_y, height)?
        || src_uv.len() < plane_len(src_stride_uv, chroma_height)?
    {
        return None;
    }

    let dst_stride_y = width;
    let dst_stride_u = chroma_width;
    let dst_stride_v = chroma_width;
    let mut dst_y = vec![0u8; plane_len(dst_stride_y, height)?];
    let mut dst_u = vec![0u8; plane_len(dst_stride_u, chroma_height)?];
    let mut dst_v = vec![0u8; plane_len(dst_stride_v, chroma_height)?];

    let ret = unsafe {
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
        )
    };
    if ret != 0 {
        return None;
    }
    Some((dst_y, dst_u, dst_v))
}

/// `libyuv::YUY2ToI420` を呼び出して、YUY2 から I420 へ変換する。
/// 変換に失敗した場合は `None` を返す。
pub fn yuy2_to_i420(
    src_yuy2: &[u8],
    src_stride: i32,
    width: i32,
    height: i32,
) -> Option<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    if width <= 0 || height <= 0 {
        return None;
    }
    let src_min_stride = width.checked_mul(2)?;
    if src_stride < src_min_stride {
        return None;
    }
    if src_yuy2.len() < plane_len(src_stride, height)? {
        return None;
    }
    let chroma_width = chroma_dimension(width)?;
    let chroma_height = chroma_dimension(height)?;

    let dst_stride_y = width;
    let dst_stride_u = chroma_width;
    let dst_stride_v = chroma_width;
    let mut dst_y = vec![0u8; plane_len(dst_stride_y, height)?];
    let mut dst_u = vec![0u8; plane_len(dst_stride_u, chroma_height)?];
    let mut dst_v = vec![0u8; plane_len(dst_stride_v, chroma_height)?];

    let ret = unsafe {
        ffi::libyuv_YUY2ToI420(
            src_yuy2.as_ptr(),
            src_stride,
            dst_y.as_mut_ptr(),
            dst_stride_y,
            dst_u.as_mut_ptr(),
            dst_stride_u,
            dst_v.as_mut_ptr(),
            dst_stride_v,
            width,
            height,
        )
    };
    if ret != 0 {
        return None;
    }
    Some((dst_y, dst_u, dst_v))
}
