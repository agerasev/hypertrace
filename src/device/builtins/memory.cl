#include "memory.h"

#define DEFINE_BUILTIN_POINTER_CONST(A, T) \
uchar xm_is_null_##A##_##T(size_t p) { \
    return (__##A T *)p == 0; \
} \
void xm_load_##A##_##T(size_t p, size_t i, size_t s, T *v) { \
    __##A const T *pp = (__##A const T *)p + i; \
    for (size_t j = 0; j < s; ++j) { \
        v[j] = pp[j]; \
    } \
} \

#define DEFINE_BUILTIN_POINTER(A, T) \
DEFINE_BUILTIN_POINTER_CONST(A, T) \
void xm_store_##A##_##T(size_t p, size_t i, size_t s, const T *v) { \
    __##A T *pp = (__##A T *)p + i; \
    for (size_t j = 0; j < s; ++j) { \
        pp[j] = v[j]; \
    } \
} \


#define DEFINE_BUILTIN_POINTER_ALL(T) \
DEFINE_BUILTIN_POINTER(local, T) \
DEFINE_BUILTIN_POINTER(global, T) \
DEFINE_BUILTIN_POINTER_CONST(constant, T) \


DEFINE_BUILTIN_POINTER_ALL(uchar)
DEFINE_BUILTIN_POINTER_ALL(ushort)
DEFINE_BUILTIN_POINTER_ALL(uint)
DEFINE_BUILTIN_POINTER_ALL(ulong)
