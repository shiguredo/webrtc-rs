use crate::{I420Buffer, NV12Buffer, ffi};

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

/// `libyuv::ConvertFromI420` を呼び出して、I420 を指定フォーマットへ変換する。
/// 変換に失敗した場合は `None` を返す。
pub fn convert_from_i420(src: &I420Buffer, fourcc: LibyuvFourcc) -> Option<Vec<u8>> {
    let width = src.width();
    let height = src.height();
    if width <= 0 || height <= 0 {
        return None;
    }
    let stride = width.checked_mul(4)?;
    let size = (stride as usize).checked_mul(height as usize)?;
    let mut dst = vec![0u8; size];
    let y = src.y_data();
    let u = src.u_data();
    let v = src.v_data();
    let ret = unsafe {
        ffi::libyuv_ConvertFromI420(
            y.as_ptr(),
            src.stride_y(),
            u.as_ptr(),
            src.stride_u(),
            v.as_ptr(),
            src.stride_v(),
            dst.as_mut_ptr(),
            stride,
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
pub fn i420_to_nv12(src: &I420Buffer) -> Option<NV12Buffer> {
    let width = src.width();
    let height = src.height();
    if width <= 0 || height <= 0 {
        return None;
    }

    let mut dst = NV12Buffer::new(width, height);
    let dst_stride_y = dst.stride_y();
    let dst_stride_uv = dst.stride_uv();
    let dst_y = dst.y_data_mut().as_mut_ptr();
    let dst_uv = dst.uv_data_mut().as_mut_ptr();

    let ret = unsafe {
        ffi::libyuv_I420ToNV12(
            src.y_data().as_ptr(),
            src.stride_y(),
            src.u_data().as_ptr(),
            src.stride_u(),
            src.v_data().as_ptr(),
            src.stride_v(),
            dst_y,
            dst_stride_y,
            dst_uv,
            dst_stride_uv,
            width,
            height,
        )
    };
    if ret != 0 {
        return None;
    }
    Some(dst)
}

/// `libyuv::ABGRToI420` を呼び出して、ABGR から I420 へ変換する。
/// 変換に失敗した場合は `None` を返す。
pub fn abgr_to_i420(src_abgr: &[u8], width: i32, height: i32) -> Option<I420Buffer> {
    let stride = (width as usize).checked_mul(4)?;
    let needed = stride.checked_mul(height as usize)?;
    if src_abgr.len() < needed {
        return None;
    }

    let mut buf = I420Buffer::new(width, height);
    let dst_stride_y = buf.stride_y();
    let dst_stride_u = buf.stride_u();
    let dst_stride_v = buf.stride_v();
    let dst_y = buf.y_data_mut().as_mut_ptr();
    let dst_u = buf.u_data_mut().as_mut_ptr();
    let dst_v = buf.v_data_mut().as_mut_ptr();

    let ret = unsafe {
        ffi::libyuv_ABGRToI420(
            src_abgr.as_ptr(),
            stride as i32,
            dst_y,
            dst_stride_y,
            dst_u,
            dst_stride_u,
            dst_v,
            dst_stride_v,
            width,
            height,
        )
    };
    if ret != 0 {
        return None;
    }
    Some(buf)
}

/// `libyuv::NV12ToI420` を呼び出して、NV12 から I420 へ変換する。
/// 変換に失敗した場合は `None` を返す。
pub fn nv12_to_i420(src: &NV12Buffer) -> Option<I420Buffer> {
    let width = src.width();
    let height = src.height();
    if width <= 0 || height <= 0 {
        return None;
    }

    let mut buf = I420Buffer::new(width, height);
    let dst_stride_y = buf.stride_y();
    let dst_stride_u = buf.stride_u();
    let dst_stride_v = buf.stride_v();
    let dst_y = buf.y_data_mut().as_mut_ptr();
    let dst_u = buf.u_data_mut().as_mut_ptr();
    let dst_v = buf.v_data_mut().as_mut_ptr();

    let ret = unsafe {
        ffi::libyuv_NV12ToI420(
            src.y_data().as_ptr(),
            src.stride_y(),
            src.uv_data().as_ptr(),
            src.stride_uv(),
            dst_y,
            dst_stride_y,
            dst_u,
            dst_stride_u,
            dst_v,
            dst_stride_v,
            width,
            height,
        )
    };
    if ret != 0 {
        return None;
    }
    Some(buf)
}

/// `libyuv::YUY2ToI420` を呼び出して、YUY2 から I420 へ変換する。
/// 変換に失敗した場合は `None` を返す。
pub fn yuy2_to_i420(
    src_yuy2: &[u8],
    src_stride: i32,
    width: i32,
    height: i32,
) -> Option<I420Buffer> {
    if width <= 0 || height <= 0 {
        return None;
    }

    let mut buf = I420Buffer::new(width, height);
    let dst_stride_y = buf.stride_y();
    let dst_stride_u = buf.stride_u();
    let dst_stride_v = buf.stride_v();
    let dst_y = buf.y_data_mut().as_mut_ptr();
    let dst_u = buf.u_data_mut().as_mut_ptr();
    let dst_v = buf.v_data_mut().as_mut_ptr();

    let ret = unsafe {
        ffi::libyuv_YUY2ToI420(
            src_yuy2.as_ptr(),
            src_stride,
            dst_y,
            dst_stride_y,
            dst_u,
            dst_stride_u,
            dst_v,
            dst_stride_v,
            width,
            height,
        )
    };
    if ret != 0 {
        return None;
    }
    Some(buf)
}
