#pragma once

#include <types.h>


#define DECLARE_BUILTIN_POINTER_CONST(A, T) \
uchar xm_is_null_##A##_##T(size_t p); \
void xm_load_##A##_##T(size_t p, size_t i, size_t s, T *v); \

#define DECLARE_BUILTIN_POINTER(A, T) \
DECLARE_BUILTIN_POINTER_CONST(A, T) \
void xm_store_##A##_##T(size_t p, size_t i, size_t s, const T *v); \


#define DECLARE_BUILTIN_POINTER_ALL(T) \
DECLARE_BUILTIN_POINTER(local, T) \
DECLARE_BUILTIN_POINTER(global, T) \
DECLARE_BUILTIN_POINTER_CONST(constant, T) \


#ifdef __cplusplus
extern "C" {
#endif

DECLARE_BUILTIN_POINTER_ALL(uchar)
DECLARE_BUILTIN_POINTER_ALL(ushort)
DECLARE_BUILTIN_POINTER_ALL(uint)
DECLARE_BUILTIN_POINTER_ALL(ulong)

#ifdef __cplusplus
};
#endif
