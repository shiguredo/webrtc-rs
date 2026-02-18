#pragma once

#include "common.h"

#define WEBRTC_DEFINE_CAST(type, cast_to, cpptype, cpp_cast_to)             \
  struct cast_to* WEBRTC_CONCAT(                                            \
      type, WEBRTC_CONCAT(_cast_to_, cast_to))(struct type * self) {        \
    auto s = reinterpret_cast<cpptype*>(self);                              \
    return reinterpret_cast<struct cast_to*>(static_cast<cpp_cast_to*>(s)); \
  }

#define WEBRTC_DEFINE_CAST_REFCOUNTED(type, cast_to, cpptype, cpp_cast_to) \
  struct WEBRTC_CONCAT(cast_to, _refcounted) *                             \
      WEBRTC_CONCAT(type, WEBRTC_CONCAT(_refcounted_cast_to_, cast_to))(   \
          struct WEBRTC_CONCAT(type, _refcounted) * self) {                \
    auto s = reinterpret_cast<cpptype*>(                                   \
        WEBRTC_CONCAT(type, _refcounted_get)(self));                       \
    webrtc::scoped_refptr<cpp_cast_to> ptr(static_cast<cpp_cast_to*>(s));  \
    return reinterpret_cast<struct WEBRTC_CONCAT(cast_to, _refcounted)*>(  \
        ptr.release());                                                    \
  }

// -------------------------
// webrtc::RefCountedInterface based types
// -------------------------

#define WEBRTC_DEFINE_REFCOUNTED(type, cpptype)                             \
  struct type* WEBRTC_CONCAT(                                               \
      type, _refcounted_get)(struct WEBRTC_CONCAT(type, _refcounted) * p) { \
    return reinterpret_cast<struct type*>(p);                               \
  }                                                                         \
  void WEBRTC_CONCAT(type, _AddRef)(struct type * p) {                      \
    auto self = reinterpret_cast<struct cpptype*>(p);                       \
    self->AddRef();                                                         \
  }                                                                         \
  void WEBRTC_CONCAT(type, _Release)(struct type * p) {                     \
    auto self = reinterpret_cast<struct cpptype*>(p);                       \
    self->Release();                                                        \
  }

// -------------------------
// std::unique_ptr<T>
// -------------------------

#define WEBRTC_DEFINE_UNIQUE(type, cpptype)                            \
  struct type* WEBRTC_CONCAT(                                          \
      type, _unique_get)(struct WEBRTC_CONCAT(type, _unique) * p) {    \
    return reinterpret_cast<type*>(p);                                 \
  }                                                                    \
  void WEBRTC_CONCAT(                                                  \
      type, _unique_delete)(struct WEBRTC_CONCAT(type, _unique) * p) { \
    auto self = reinterpret_cast<cpptype*>(p);                         \
    delete self;                                                       \
  }

// -------------------------
// std::vector<T>
// -------------------------

#define WEBRTC_DEFINE_VECTOR(type, cpptype)                               \
  struct WEBRTC_CONCAT(type, _vector) *                                   \
      WEBRTC_CONCAT(type, _vector_new)(int size) {                        \
    auto vec = new std::vector<cpptype>(size);                            \
    return reinterpret_cast<struct WEBRTC_CONCAT(type, _vector)*>(vec);   \
  }                                                                       \
  void WEBRTC_CONCAT(                                                     \
      type, _vector_delete)(struct WEBRTC_CONCAT(type, _vector) * self) { \
    auto vec = reinterpret_cast<std::vector<cpptype>*>(self);             \
    delete vec;                                                           \
  }                                                                       \
  struct type* WEBRTC_CONCAT(type, _vector_get)(                          \
      struct WEBRTC_CONCAT(type, _vector) * self, int index) {            \
    auto vec = reinterpret_cast<std::vector<cpptype>*>(self);             \
    auto& cpp = (*vec)[index];                                            \
    return reinterpret_cast<struct type*>(&cpp);                          \
  }                                                                       \
  int WEBRTC_CONCAT(                                                      \
      type, _vector_size)(struct WEBRTC_CONCAT(type, _vector) * self) {   \
    auto vec = reinterpret_cast<std::vector<cpptype>*>(self);             \
    return static_cast<int>(vec->size());                                 \
  }                                                                       \
  void WEBRTC_CONCAT(type, _vector_resize)(                               \
      struct WEBRTC_CONCAT(type, _vector) * self, int size) {             \
    auto vec = reinterpret_cast<std::vector<cpptype>*>(self);             \
    vec->resize(size);                                                    \
  }                                                                       \
  void WEBRTC_CONCAT(type, _vector_set)(                                  \
      struct WEBRTC_CONCAT(type, _vector) * self, int index,              \
      struct type* caps) {                                                \
    auto vec = reinterpret_cast<std::vector<cpptype>*>(self);             \
    auto cpp = reinterpret_cast<cpptype*>(caps);                          \
    (*vec)[index] = *cpp;                                                 \
  }                                                                       \
  void WEBRTC_CONCAT(type, _vector_push_back)(                            \
      struct WEBRTC_CONCAT(type, _vector) * self, struct type * value) {  \
    auto vec = reinterpret_cast<std::vector<cpptype>*>(self);             \
    auto cpp = reinterpret_cast<cpptype*>(value);                         \
    vec->push_back(*cpp);                                                 \
  }

#define WEBRTC_DEFINE_VECTOR_NO_DEFAULT_CTOR(type, cpptype)               \
  struct WEBRTC_CONCAT(type, _vector) *                                   \
      WEBRTC_CONCAT(type, _vector_new)(void) {                            \
    auto vec = new std::vector<cpptype>();                                \
    return reinterpret_cast<struct WEBRTC_CONCAT(type, _vector)*>(vec);   \
  }                                                                       \
  void WEBRTC_CONCAT(                                                     \
      type, _vector_delete)(struct WEBRTC_CONCAT(type, _vector) * self) { \
    auto vec = reinterpret_cast<std::vector<cpptype>*>(self);             \
    delete vec;                                                           \
  }                                                                       \
  struct type* WEBRTC_CONCAT(type, _vector_get)(                          \
      struct WEBRTC_CONCAT(type, _vector) * self, int index) {            \
    auto vec = reinterpret_cast<std::vector<cpptype>*>(self);             \
    auto& cpp = (*vec)[index];                                            \
    return reinterpret_cast<struct type*>(&cpp);                          \
  }                                                                       \
  int WEBRTC_CONCAT(                                                      \
      type, _vector_size)(struct WEBRTC_CONCAT(type, _vector) * self) {   \
    auto vec = reinterpret_cast<std::vector<cpptype>*>(self);             \
    return static_cast<int>(vec->size());                                 \
  }                                                                       \
  void WEBRTC_CONCAT(type, _vector_set)(                                  \
      struct WEBRTC_CONCAT(type, _vector) * self, int index,              \
      struct type* caps) {                                                \
    auto vec = reinterpret_cast<std::vector<cpptype>*>(self);             \
    auto cpp = reinterpret_cast<cpptype*>(caps);                          \
    (*vec)[index] = *cpp;                                                 \
  }                                                                       \
  void WEBRTC_CONCAT(type, _vector_push_back)(                            \
      struct WEBRTC_CONCAT(type, _vector) * self, struct type * value) {  \
    auto vec = reinterpret_cast<std::vector<cpptype>*>(self);             \
    auto cpp = reinterpret_cast<cpptype*>(value);                         \
    vec->push_back(*cpp);                                                 \
  }
