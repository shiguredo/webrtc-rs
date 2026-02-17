#pragma once

#include <assert.h>

#include <optional>

namespace webrtc_c {

template <typename T>
inline void OptionalGet(const std::optional<T>& src,
                        int* out_has,
                        T* out_value) {
  const bool has = src.has_value();
  if (out_has != nullptr) {
    *out_has = has ? 1 : 0;
  }
  if (out_value != nullptr && has) {
    *out_value = *src;
  }
}

template <typename T>
inline void OptionalSet(std::optional<T>& dst, int has, const T* value) {
  if (has == 0) {
    dst.reset();
    return;
  }
  assert(value != nullptr);
  if (value == nullptr) {
    return;
  }
  dst = *value;
}

template <typename SrcT, typename DstT, typename Builder>
inline void OptionalGetAs(const std::optional<SrcT>& src,
                          int* out_has,
                          DstT* out_value,
                          Builder&& build_value) {
  const bool has = src.has_value();
  if (out_has != nullptr) {
    *out_has = has ? 1 : 0;
  }
  if (out_value != nullptr && has) {
    *out_value = build_value();
  }
}

template <typename DstT, typename SrcT, typename Builder>
inline void OptionalSetAs(std::optional<DstT>& dst,
                          int has,
                          const SrcT* value,
                          Builder&& build_value) {
  if (has == 0) {
    dst.reset();
    return;
  }
  assert(value != nullptr);
  if (value == nullptr) {
    return;
  }
  dst = build_value();
}

}  // namespace webrtc_c
