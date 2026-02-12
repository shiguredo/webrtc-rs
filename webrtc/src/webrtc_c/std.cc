#include "std.h"

#include <stddef.h>
#include <map>
#include <memory>
#include <string>
#include <vector>  // IWYU pragma: keep

#include "common.impl.h"

// -------------------------
// std::string
// -------------------------

extern "C" {
WEBRTC_DEFINE_UNIQUE(std_string, std::string);

int std_string_size(struct std_string* self) {
  auto str = reinterpret_cast<std::string*>(self);
  return static_cast<int>(str->size());
}
const char* std_string_c_str(struct std_string* self) {
  auto str = reinterpret_cast<std::string*>(self);
  return str->c_str();
}
void std_string_append(struct std_string* self, const char* str, size_t len) {
  auto s = reinterpret_cast<std::string*>(self);
  s->append(str, len);
}
struct std_string_unique* std_string_new_empty() {
  auto str = std::make_unique<std::string>();
  return reinterpret_cast<struct std_string_unique*>(str.release());
}
struct std_string_unique* std_string_new_from_cstr(const char* str) {
  auto s = std::make_unique<std::string>(str);
  return reinterpret_cast<struct std_string_unique*>(s.release());
}
struct std_string_unique* std_string_new_from_bytes(const char* bytes,
                                                    size_t len) {
  auto s = std::make_unique<std::string>(bytes, len);
  return reinterpret_cast<struct std_string_unique*>(s.release());
}
}

// -------------------------
// std::vector<T>
// -------------------------

extern "C" {
WEBRTC_DEFINE_VECTOR(std_string, std::string);
}

// -------------------------
// std::map<std::string, std::string>
// -------------------------

extern "C" {
struct std_map_string_string_iter {
  std::map<std::string, std::string>* map;
  std::map<std::string, std::string>::iterator it;
  bool started;
};

void std_map_string_string_set(struct std_map_string_string* self,
                               const char* key,
                               size_t key_len,
                               const char* value,
                               size_t value_len) {
  auto map = reinterpret_cast<std::map<std::string, std::string>*>(self);
  (*map)[std::string(key, key_len)] = std::string(value, value_len);
}
int std_map_string_string_size(struct std_map_string_string* self) {
  auto map = reinterpret_cast<std::map<std::string, std::string>*>(self);
  return static_cast<int>(map->size());
}
struct std_map_string_string_iter* std_map_string_string_iter_new(
    struct std_map_string_string* map) {
  if (map == nullptr) {
    return nullptr;
  }
  auto m = reinterpret_cast<std::map<std::string, std::string>*>(map);
  auto iter = new std_map_string_string_iter{m, {}, false};
  return reinterpret_cast<struct std_map_string_string_iter*>(iter);
}
void std_map_string_string_iter_delete(
    struct std_map_string_string_iter* iter) {
  auto impl = reinterpret_cast<struct std_map_string_string_iter*>(iter);
  delete impl;
}
// イテレータを一つ進めて、キーと値を新しく確保して返す。
int std_map_string_string_iter_next(struct std_map_string_string_iter* iter,
                                    struct std_string_unique** key,
                                    struct std_string_unique** value) {
  auto impl = reinterpret_cast<struct std_map_string_string_iter*>(iter);
  if (impl == nullptr || impl->map == nullptr) {
    return 0;
  }
  if (!impl->started) {
    impl->it = impl->map->begin();
    impl->started = true;
  } else if (impl->it != impl->map->end()) {
    ++impl->it;
  }
  if (impl->it == impl->map->end()) {
    return 0;
  }
  if (key != nullptr) {
    auto k = std::make_unique<std::string>(impl->it->first);
    *key = reinterpret_cast<struct std_string_unique*>(k.release());
  }
  if (value != nullptr) {
    auto v = std::make_unique<std::string>(impl->it->second);
    *value = reinterpret_cast<struct std_string_unique*>(v.release());
  }
  return 1;
}
}
