#pragma once

#include <stdint.h>

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// libyuv
// -------------------------

int libyuv_ABGRToI420(const uint8_t* src_abgr,
                      int src_stride_abgr,
                      uint8_t* dst_y,
                      int dst_stride_y,
                      uint8_t* dst_u,
                      int dst_stride_u,
                      uint8_t* dst_v,
                      int dst_stride_v,
                      int width,
                      int height);
extern const uint32_t libyuv_FOURCC_ARGB;
extern const uint32_t libyuv_FOURCC_BGRA;
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
                           uint32_t fourcc);

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
                      int height);

int libyuv_YUY2ToI420(const uint8_t* src_yuy2,
                      int src_stride_yuy2,
                      uint8_t* dst_y,
                      int dst_stride_y,
                      uint8_t* dst_u,
                      int dst_stride_u,
                      uint8_t* dst_v,
                      int dst_stride_v,
                      int width,
                      int height);

#if defined(__cplusplus)
}
#endif
