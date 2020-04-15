#include "memory.h"

#define DEFINE_BUILTIN_POINTER_CONST(NS) \
uchar xm_is_null_##NS(size_t p) { \
    return (__##NS uchar *)p == 0; \
} \
void xm_load_##NS(size_t p, size_t i, size_t s, uchar *v) { \
    __##NS const uchar *pp = (__##NS const uchar *)p + i; \
    for (size_t j = 0; j < s; ++j) { \
        v[j] = pp[j]; \
    } \
} \

#define DEFINE_BUILTIN_POINTER(NS) \
DEFINE_BUILTIN_POINTER_CONST(NS) \
void xm_store_##NS(size_t p, size_t i, size_t s, const uchar *v) { \
    __##NS uchar *pp = (__##NS uchar *)p + i; \
    for (size_t j = 0; j < s; ++j) { \
        pp[j] = v[j]; \
    } \
} \


DEFINE_BUILTIN_POINTER(private)
DEFINE_BUILTIN_POINTER(local)
DEFINE_BUILTIN_POINTER(global)
DEFINE_BUILTIN_POINTER_CONST(constant)
