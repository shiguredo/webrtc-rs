#pragma once

#include <stddef.h>

#include "common.h"

#if defined(__cplusplus)
extern "C" {
#endif

// -------------------------
// std::string
// -------------------------

WEBRTC_DECLARE_UNIQUE(std_string);
int std_string_size(struct std_string* self);
const char* std_string_c_str(struct std_string* self);
void std_string_append(struct std_string* self, const char* str, size_t len);
struct std_string_unique* std_string_new_empty();
struct std_string_unique* std_string_new_from_cstr(const char* str);
struct std_string_unique* std_string_new_from_bytes(const char* bytes,
                                                    size_t len);

// -------------------------
// std::vector<T>
// -------------------------

WEBRTC_DECLARE_VECTOR(std_string);

// -------------------------
// std::map<std::string, std::string>
// -------------------------

struct std_map_string_string;
void std_map_string_string_set(struct std_map_string_string* self,
                               const char* key,
                               size_t key_len,
                               const char* value,
                               size_t value_len);
int std_map_string_string_size(struct std_map_string_string* self);

struct std_map_string_string_iter;
struct std_map_string_string_iter* std_map_string_string_iter_new(
    struct std_map_string_string* map);
void std_map_string_string_iter_delete(struct std_map_string_string_iter* iter);
int std_map_string_string_iter_next(struct std_map_string_string_iter* iter,
                                    struct std_string_unique** key,
                                    struct std_string_unique** value);

#if defined(__cplusplus)
}
#endif
