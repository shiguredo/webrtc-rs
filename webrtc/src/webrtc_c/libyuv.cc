#include "libyuv.h"

#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>

// libyuv
#include <libyuv/convert.h>
#include <libyuv/convert_from.h>
#include <libyuv/video_common.h>

// -------------------------
// libyuv
// -------------------------

extern "C" {
int libyuv_ABGRToI420(const uint8_t* src_abgr,
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
const uint32_t libyuv_FOURCC_ARGB = static_cast<uint32_t>(libyuv::FOURCC_ARGB);
const uint32_t libyuv_FOURCC_BGRA = static_cast<uint32_t>(libyuv::FOURCC_BGRA);

int libyuv_ConvertFromI420(const uint8_t* src_y,
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

int libyuv_NV12ToI420(const uint8_t* src_y,
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

int libyuv_YUY2ToI420(const uint8_t* src_yuy2,
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
}
