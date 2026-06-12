#include "libyuv.h"

#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>

// libyuv
#include <libyuv/convert.h>
#include <libyuv/convert_from.h>
#include <libyuv/mjpeg_decoder.h>
#include <libyuv/planar_functions.h>
#include <libyuv/rotate.h>
#include <libyuv/video_common.h>

#include "common.h"

// -------------------------
// libyuv
// -------------------------

extern "C" {

WEBRTC_EXPORT const uint32_t libyuv_FOURCC_ARGB =
    static_cast<uint32_t>(libyuv::FOURCC_ARGB);
WEBRTC_EXPORT const uint32_t libyuv_FOURCC_BGRA =
    static_cast<uint32_t>(libyuv::FOURCC_BGRA);
WEBRTC_EXPORT const uint32_t libyuv_FOURCC_MJPG =
    static_cast<uint32_t>(libyuv::FOURCC_MJPG);

WEBRTC_EXPORT const int libyuv_kRotate0 = static_cast<int>(libyuv::kRotate0);
WEBRTC_EXPORT const int libyuv_kRotate90 = static_cast<int>(libyuv::kRotate90);
WEBRTC_EXPORT const int libyuv_kRotate180 =
    static_cast<int>(libyuv::kRotate180);
WEBRTC_EXPORT const int libyuv_kRotate270 =
    static_cast<int>(libyuv::kRotate270);

WEBRTC_EXPORT int libyuv_ABGRToI420(const uint8_t* src_abgr,
                                    int src_stride_abgr,
                                    uint8_t* dst_y,
                                    int dst_stride_y,
                                    uint8_t* dst_u,
                                    int dst_stride_u,
                                    uint8_t* dst_v,
                                    int dst_stride_v,
                                    int width,
                                    int height) {
  return libyuv::ABGRToI420(src_abgr, src_stride_abgr, dst_y, dst_stride_y,
                            dst_u, dst_stride_u, dst_v, dst_stride_v, width,
                            height);
}
WEBRTC_EXPORT int libyuv_ConvertFromI420(const uint8_t* src_y,
                                         int src_stride_y,
                                         const uint8_t* src_u,
                                         int src_stride_u,
                                         const uint8_t* src_v,
                                         int src_stride_v,
                                         uint8_t* dst_argb,
                                         int dst_stride_argb,
                                         int width,
                                         int height,
                                         uint32_t fourcc) {
  return libyuv::ConvertFromI420(src_y, src_stride_y, src_u, src_stride_u,
                                 src_v, src_stride_v, dst_argb, dst_stride_argb,
                                 width, height, fourcc);
}

WEBRTC_EXPORT int libyuv_NV12ToI420(const uint8_t* src_y,
                                    int src_stride_y,
                                    const uint8_t* src_uv,
                                    int src_stride_uv,
                                    uint8_t* dst_y,
                                    int dst_stride_y,
                                    uint8_t* dst_u,
                                    int dst_stride_u,
                                    uint8_t* dst_v,
                                    int dst_stride_v,
                                    int width,
                                    int height) {
  return libyuv::NV12ToI420(src_y, src_stride_y, src_uv, src_stride_uv, dst_y,
                            dst_stride_y, dst_u, dst_stride_u, dst_v,
                            dst_stride_v, width, height);
}

WEBRTC_EXPORT int libyuv_I420ToNV12(const uint8_t* src_y,
                                    int src_stride_y,
                                    const uint8_t* src_u,
                                    int src_stride_u,
                                    const uint8_t* src_v,
                                    int src_stride_v,
                                    uint8_t* dst_y,
                                    int dst_stride_y,
                                    uint8_t* dst_uv,
                                    int dst_stride_uv,
                                    int width,
                                    int height) {
  return libyuv::I420ToNV12(src_y, src_stride_y, src_u, src_stride_u, src_v,
                            src_stride_v, dst_y, dst_stride_y, dst_uv,
                            dst_stride_uv, width, height);
}

WEBRTC_EXPORT int libyuv_I420Copy(const uint8_t* src_y,
                                  int src_stride_y,
                                  const uint8_t* src_u,
                                  int src_stride_u,
                                  const uint8_t* src_v,
                                  int src_stride_v,
                                  uint8_t* dst_y,
                                  int dst_stride_y,
                                  uint8_t* dst_u,
                                  int dst_stride_u,
                                  uint8_t* dst_v,
                                  int dst_stride_v,
                                  int width,
                                  int height) {
  return libyuv::I420Copy(src_y, src_stride_y, src_u, src_stride_u, src_v,
                          src_stride_v, dst_y, dst_stride_y, dst_u,
                          dst_stride_u, dst_v, dst_stride_v, width, height);
}

WEBRTC_EXPORT int libyuv_NV12Copy(const uint8_t* src_y,
                                  int src_stride_y,
                                  const uint8_t* src_uv,
                                  int src_stride_uv,
                                  uint8_t* dst_y,
                                  int dst_stride_y,
                                  uint8_t* dst_uv,
                                  int dst_stride_uv,
                                  int width,
                                  int height) {
  return libyuv::NV12Copy(src_y, src_stride_y, src_uv, src_stride_uv, dst_y,
                          dst_stride_y, dst_uv, dst_stride_uv, width, height);
}

WEBRTC_EXPORT int libyuv_YUY2ToI420(const uint8_t* src_yuy2,
                                    int src_stride_yuy2,
                                    uint8_t* dst_y,
                                    int dst_stride_y,
                                    uint8_t* dst_u,
                                    int dst_stride_u,
                                    uint8_t* dst_v,
                                    int dst_stride_v,
                                    int width,
                                    int height) {
  return libyuv::YUY2ToI420(src_yuy2, src_stride_yuy2, dst_y, dst_stride_y,
                            dst_u, dst_stride_u, dst_v, dst_stride_v, width,
                            height);
}

WEBRTC_EXPORT int libyuv_I420Rotate(const uint8_t* src_y,
                                    int src_stride_y,
                                    const uint8_t* src_u,
                                    int src_stride_u,
                                    const uint8_t* src_v,
                                    int src_stride_v,
                                    uint8_t* dst_y,
                                    int dst_stride_y,
                                    uint8_t* dst_u,
                                    int dst_stride_u,
                                    uint8_t* dst_v,
                                    int dst_stride_v,
                                    int width,
                                    int height,
                                    int mode) {
  return libyuv::I420Rotate(src_y, src_stride_y, src_u, src_stride_u, src_v,
                            src_stride_v, dst_y, dst_stride_y, dst_u,
                            dst_stride_u, dst_v, dst_stride_v, width, height,
                            static_cast<libyuv::RotationMode>(mode));
}

WEBRTC_EXPORT int libyuv_MJPGToI420(const uint8_t* sample,
                                    size_t sample_size,
                                    uint8_t* dst_y,
                                    int dst_stride_y,
                                    uint8_t* dst_u,
                                    int dst_stride_u,
                                    uint8_t* dst_v,
                                    int dst_stride_v,
                                    int src_width,
                                    int src_height,
                                    int dst_width,
                                    int dst_height) {
#if defined(WEBRTC_IOS)
  return 1;
#else
  return libyuv::MJPGToI420(sample, sample_size, dst_y, dst_stride_y, dst_u,
                            dst_stride_u, dst_v, dst_stride_v, src_width,
                            src_height, dst_width, dst_height);
#endif
}

WEBRTC_EXPORT int libyuv_MJPGToNV12(const uint8_t* sample,
                                    size_t sample_size,
                                    uint8_t* dst_y,
                                    int dst_stride_y,
                                    uint8_t* dst_uv,
                                    int dst_stride_uv,
                                    int src_width,
                                    int src_height,
                                    int dst_width,
                                    int dst_height) {
#if defined(WEBRTC_IOS)
  return 1;
#else
  return libyuv::MJPGToNV12(sample, sample_size, dst_y, dst_stride_y, dst_uv,
                            dst_stride_uv, src_width, src_height, dst_width,
                            dst_height);
#endif
}

WEBRTC_EXPORT int libyuv_MJPGSize(const uint8_t* sample,
                                  size_t sample_size,
                                  int* width,
                                  int* height) {
#if defined(WEBRTC_IOS)
  return 1;
#else
  return libyuv::MJPGSize(sample, sample_size, width, height);
#endif
}

WEBRTC_EXPORT int libyuv_ConvertToI420(const uint8_t* src_frame,
                                       size_t src_size,
                                       uint8_t* dst_y,
                                       int dst_stride_y,
                                       uint8_t* dst_u,
                                       int dst_stride_u,
                                       uint8_t* dst_v,
                                       int dst_stride_v,
                                       int crop_x,
                                       int crop_y,
                                       int src_width,
                                       int src_height,
                                       int crop_width,
                                       int crop_height,
                                       int rotation,
                                       uint32_t fourcc) {
  return libyuv::ConvertToI420(
      src_frame, src_size, dst_y, dst_stride_y, dst_u, dst_stride_u, dst_v,
      dst_stride_v, crop_x, crop_y, src_width, src_height, crop_width,
      crop_height, static_cast<libyuv::RotationMode>(rotation), fourcc);
}
}
