#pragma once

#include <stddef.h>
#include <stdint.h>

#if defined(_WIN32) || defined(__CYGWIN__)
#define WEBRTC_EXPORT __declspec(dllexport)
#elif defined(__GNUC__) || defined(__clang__)
#define WEBRTC_EXPORT __attribute__((visibility("default")))
#else
#define WEBRTC_EXPORT
#endif

#define WEBRTC_CONCAT(a, b) WEBRTC_CONCAT_I(a, b)
#define WEBRTC_CONCAT_I(a, b) a##b

#define WEBRTC_DECLARE_CAST(type, cast_to)     \
  WEBRTC_EXPORT struct cast_to* WEBRTC_CONCAT( \
      type, WEBRTC_CONCAT(_cast_to_, cast_to))(struct type * self)

#define WEBRTC_DECLARE_CAST_REFCOUNTED(type, cast_to)                    \
  WEBRTC_EXPORT struct WEBRTC_CONCAT(cast_to, _refcounted) *             \
      WEBRTC_CONCAT(type, WEBRTC_CONCAT(_refcounted_cast_to_, cast_to))( \
          struct WEBRTC_CONCAT(type, _refcounted) * self)

// -------------------------
// webrtc::RefCountedInterface based types
// -------------------------

#define WEBRTC_DECLARE_REFCOUNTED(type)                                    \
  struct type;                                                             \
  struct WEBRTC_CONCAT(type, _refcounted);                                 \
  WEBRTC_EXPORT struct type* WEBRTC_CONCAT(                                \
      type, _refcounted_get)(struct WEBRTC_CONCAT(type, _refcounted) * p); \
  WEBRTC_EXPORT void WEBRTC_CONCAT(type, _AddRef)(struct type * p);        \
  WEBRTC_EXPORT void WEBRTC_CONCAT(type, _Release)(struct type * p)

// -------------------------
// std::unique_ptr<T>
// -------------------------

#define WEBRTC_DECLARE_UNIQUE(type)                                \
  struct type;                                                     \
  struct WEBRTC_CONCAT(type, _unique);                             \
  WEBRTC_EXPORT struct type* WEBRTC_CONCAT(                        \
      type, _unique_get)(struct WEBRTC_CONCAT(type, _unique) * p); \
  WEBRTC_EXPORT void WEBRTC_CONCAT(                                \
      type, _unique_delete)(struct WEBRTC_CONCAT(type, _unique) * p)

// -------------------------
// std::vector<T>
// -------------------------

#define WEBRTC_DECLARE_VECTOR(type)                                      \
  struct type;                                                           \
  struct WEBRTC_CONCAT(type, _vector);                                   \
  WEBRTC_EXPORT struct WEBRTC_CONCAT(type, _vector) *                    \
      WEBRTC_CONCAT(type, _vector_new)(int size);                        \
  WEBRTC_EXPORT void WEBRTC_CONCAT(                                      \
      type, _vector_delete)(struct WEBRTC_CONCAT(type, _vector) * self); \
  WEBRTC_EXPORT struct type* WEBRTC_CONCAT(type, _vector_get)(           \
      struct WEBRTC_CONCAT(type, _vector) * self, int index);            \
  WEBRTC_EXPORT int WEBRTC_CONCAT(                                       \
      type, _vector_size)(struct WEBRTC_CONCAT(type, _vector) * self);   \
  WEBRTC_EXPORT void WEBRTC_CONCAT(type, _vector_resize)(                \
      struct WEBRTC_CONCAT(type, _vector) * self, int size);             \
  WEBRTC_EXPORT void WEBRTC_CONCAT(type, _vector_set)(                   \
      struct WEBRTC_CONCAT(type, _vector) * self, int index,             \
      struct type* caps);                                                \
  WEBRTC_EXPORT void WEBRTC_CONCAT(type, _vector_push_back)(             \
      struct WEBRTC_CONCAT(type, _vector) * self, struct type * value);

// -------------------------
// std::vector<T> (T does not have default constructor)
// -------------------------

#define WEBRTC_DECLARE_VECTOR_NO_DEFAULT_CTOR(type)                      \
  struct type;                                                           \
  struct WEBRTC_CONCAT(type, _vector);                                   \
  WEBRTC_EXPORT struct WEBRTC_CONCAT(type, _vector) *                    \
      WEBRTC_CONCAT(type, _vector_new)(void);                            \
  WEBRTC_EXPORT void WEBRTC_CONCAT(                                      \
      type, _vector_delete)(struct WEBRTC_CONCAT(type, _vector) * self); \
  WEBRTC_EXPORT struct type* WEBRTC_CONCAT(type, _vector_get)(           \
      struct WEBRTC_CONCAT(type, _vector) * self, int index);            \
  WEBRTC_EXPORT int WEBRTC_CONCAT(                                       \
      type, _vector_size)(struct WEBRTC_CONCAT(type, _vector) * self);   \
  WEBRTC_EXPORT void WEBRTC_CONCAT(type, _vector_set)(                   \
      struct WEBRTC_CONCAT(type, _vector) * self, int index,             \
      struct type* caps);                                                \
  WEBRTC_EXPORT void WEBRTC_CONCAT(type, _vector_push_back)(             \
      struct WEBRTC_CONCAT(type, _vector) * self, struct type * value);
