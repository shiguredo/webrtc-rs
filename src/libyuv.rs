use crate::{I420Buffer, ffi};

/// I420 から ARGB へ変換する。
/// 変換に失敗した場合は None を返す。
pub fn i420_to_argb(src: &I420Buffer) -> Option<Vec<u8>> {
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
            ffi::libyuv_FOURCC_ARGB,
        )
    };
    if ret != 0 {
        return None;
    }
    Some(dst)
}

/// ABGR から I420 へ変換する。
/// 変換に失敗した場合は None を返す。
pub fn abgr_to_i420(src_abgr: &[u8], width: i32, height: i32) -> Option<I420Buffer> {
    let stride = (width as usize).checked_mul(4)?;
    let needed = stride.checked_mul(height as usize)?;
    if src_abgr.len() < needed {
        return None;
    }
    let buf = I420Buffer::new(width, height);
    let raw = buf.raw().as_ptr();
    let ret = unsafe {
        ffi::libyuv_ABGRToI420(
            src_abgr.as_ptr(),
            stride as i32,
            ffi::webrtc_I420Buffer_MutableDataY(raw),
            ffi::webrtc_I420Buffer_StrideY(raw),
            ffi::webrtc_I420Buffer_MutableDataU(raw),
            ffi::webrtc_I420Buffer_StrideU(raw),
            ffi::webrtc_I420Buffer_MutableDataV(raw),
            ffi::webrtc_I420Buffer_StrideV(raw),
            width,
            height,
        )
    };
    if ret != 0 {
        return None;
    }
    Some(buf)
}

/// NV12 から I420 へ変換する。
/// NV12 は Y プレーンと UV インターリーブプレーンで構成される。
/// 変換に失敗した場合は None を返す。
pub fn nv12_to_i420(
    src_y: &[u8],
    src_stride_y: i32,
    src_uv: &[u8],
    src_stride_uv: i32,
    width: i32,
    height: i32,
) -> Option<I420Buffer> {
    if width <= 0 || height <= 0 {
        return None;
    }
    let buf = I420Buffer::new(width, height);
    let raw = buf.raw().as_ptr();
    let ret = unsafe {
        ffi::libyuv_NV12ToI420(
            src_y.as_ptr(),
            src_stride_y,
            src_uv.as_ptr(),
            src_stride_uv,
            ffi::webrtc_I420Buffer_MutableDataY(raw),
            ffi::webrtc_I420Buffer_StrideY(raw),
            ffi::webrtc_I420Buffer_MutableDataU(raw),
            ffi::webrtc_I420Buffer_StrideU(raw),
            ffi::webrtc_I420Buffer_MutableDataV(raw),
            ffi::webrtc_I420Buffer_StrideV(raw),
            width,
            height,
        )
    };
    if ret != 0 {
        return None;
    }
    Some(buf)
}

/// YUY2 (YUYV) から I420 へ変換する。
/// YUY2 はパックド形式で、Y0 U0 Y1 V0 の順で格納される。
/// 変換に失敗した場合は None を返す。
pub fn yuy2_to_i420(
    src_yuy2: &[u8],
    src_stride: i32,
    width: i32,
    height: i32,
) -> Option<I420Buffer> {
    if width <= 0 || height <= 0 {
        return None;
    }
    let buf = I420Buffer::new(width, height);
    let raw = buf.raw().as_ptr();
    let ret = unsafe {
        ffi::libyuv_YUY2ToI420(
            src_yuy2.as_ptr(),
            src_stride,
            ffi::webrtc_I420Buffer_MutableDataY(raw),
            ffi::webrtc_I420Buffer_StrideY(raw),
            ffi::webrtc_I420Buffer_MutableDataU(raw),
            ffi::webrtc_I420Buffer_StrideU(raw),
            ffi::webrtc_I420Buffer_MutableDataV(raw),
            ffi::webrtc_I420Buffer_StrideV(raw),
            width,
            height,
        )
    };
    if ret != 0 {
        return None;
    }
    Some(buf)
}
