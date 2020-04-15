#pragma once

#include <types.h>


#define DECLARE_BUILTIN_POINTER_CONST(NS) \
uchar xm_is_null_##NS(size_t p); \
void xm_load_##NS(size_t p, size_t i, size_t s, uchar *v); \

#define DECLARE_BUILTIN_POINTER(NS) \
DECLARE_BUILTIN_POINTER_CONST(NS) \
void xm_store_##NS(size_t p, size_t i, size_t s, const uchar *v); \


#ifdef __cplusplus
extern "C" {
#endif

DECLARE_BUILTIN_POINTER(private)
DECLARE_BUILTIN_POINTER(local)
DECLARE_BUILTIN_POINTER(global)
DECLARE_BUILTIN_POINTER_CONST(constant)

#ifdef __cplusplus
};
#endif
