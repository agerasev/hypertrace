#pragma once

#include <types.hpp>

#define DECLARE_BUILTIN_VECTOR(T, N) \
void ocl_ctor_##T##N##_##T(T *v, T x); \
void ocl_load_##T##N(T *v, const T *d); \
void ocl_store_##T##N(const T *v, T *d); \
void ocl_neg_##T##N(T *o, const T *a); \
void ocl_add_##T##N##_##T##N(T *o, const T *a, const T *b); \
void ocl_add_##T##N##_##T(T *o, const T *a, T b); \
void ocl_add_##T##_##T##N(T *o, T a, const T *b); \
void ocl_sub_##T##N##_##T##N(T *o, const T *a, const T *b); \
void ocl_sub_##T##N##_##T(T *o, const T *a, T b); \
void ocl_sub_##T##_##T##N(T *o, T a, const T *b); \
void ocl_mul_##T##N##_##T##N(T *o, const T *a, const T *b); \
void ocl_mul_##T##N##_##T(T *o, const T *a, T b); \
void ocl_mul_##T##_##T##N(T *o, T a, const T *b); \
void ocl_div_##T##N##_##T##N(T *o, const T *a, const T *b); \
void ocl_div_##T##N##_##T(T *o, const T *a, T b); \
void ocl_div_##T##_##T##N(T *o, T a, const T *b); \
void ocl_addassign_##T##N##_##T##N(T *o, const T *a); \
void ocl_addassign_##T##N##_##T(T *o, T a); \
void ocl_subassign_##T##N##_##T##N(T *o, const T *a); \
void ocl_subassign_##T##N##_##T(T *o, T a); \
void ocl_mulassign_##T##N##_##T##N(T *o, const T *a); \
void ocl_mulassign_##T##N##_##T(T *o, T a); \
void ocl_divassign_##T##N##_##T##N(T *o, const T *a); \
void ocl_divassign_##T##N##_##T(T *o, T a); \

#define DECLARE_BUILTIN_VECTOR_CTOR_2(T) \
void ocl_ctor_##T##2_##T##_2(T *v, T x, T y); \

#define DECLARE_BUILTIN_VECTOR_CTOR_3(T) \
void ocl_ctor_##T##3_##T##_3(T *v, T x, T y, T z); \

#define DECLARE_BUILTIN_VECTOR_CTOR_4(T) \
void ocl_ctor_##T##4_##T##_4(T *v, T x, T y, T z, T w); \

#define DECLARE_BUILTIN_VECTOR_CTOR_8(T) \
void ocl_ctor_##T##8_##T##_8( \
    T *v, \
    T s0, T s1, T s2, T s3, T s4, T s5, T s6, T s7 \
); \

#define DECLARE_BUILTIN_VECTOR_CTOR_16(T) \
void ocl_ctor_##T##16_##T##_16( \
    T *v, \
    T s0, T s1, T s2, T s3, T s4, T s5, T s6, T s7, \
    T s8, T s9, T sA, T sB, T sC, T sD, T sE, T sF \
); \

#define DECLARE_BUILTIN_VECTOR_UINT(T, N) \
DECLARE_BUILTIN_VECTOR(T, N) \

#define DECLARE_BUILTIN_VECTOR_INT(T, N) \
DECLARE_BUILTIN_VECTOR(T, N) \
void ocl_abs_##T##N(u##T *o, const T *a); \

#define DECLARE_BUILTIN_VECTOR_FLOAT(T, N) \
DECLARE_BUILTIN_VECTOR(T, N) \
void ocl_fabs_##T##N(T *o, const T *a); \

#define DECLARE_BUILTIN_VECTOR_GEOMETRY(T, N) \
T ocl_dot_##T##N(const T *a, const T *b); \
T ocl_length_##T##N(const T *a); \
void ocl_normalize_##T##N(T *o, const T *a); \

#define DECLARE_BUILTIN_VECTOR_GEOMETRY_CROSS(T, N) \
void ocl_cross_##T##N(T *o, const T *a, const T *b); \

#define DECLARE_BUILTIN_VECTOR_UINT_ALL(T) \
DECLARE_BUILTIN_VECTOR_UINT(T, 2) \
DECLARE_BUILTIN_VECTOR_UINT(T, 3) \
DECLARE_BUILTIN_VECTOR_UINT(T, 4) \
DECLARE_BUILTIN_VECTOR_UINT(T, 8) \
DECLARE_BUILTIN_VECTOR_UINT(T, 16) \
DECLARE_BUILTIN_VECTOR_CTOR_2(T) \
DECLARE_BUILTIN_VECTOR_CTOR_3(T) \
DECLARE_BUILTIN_VECTOR_CTOR_4(T) \
DECLARE_BUILTIN_VECTOR_CTOR_8(T) \
DECLARE_BUILTIN_VECTOR_CTOR_16(T) \

#define DECLARE_BUILTIN_VECTOR_INT_ALL(T) \
DECLARE_BUILTIN_VECTOR_INT(T, 2) \
DECLARE_BUILTIN_VECTOR_INT(T, 3) \
DECLARE_BUILTIN_VECTOR_INT(T, 4) \
DECLARE_BUILTIN_VECTOR_INT(T, 8) \
DECLARE_BUILTIN_VECTOR_INT(T, 16) \
DECLARE_BUILTIN_VECTOR_CTOR_2(T) \
DECLARE_BUILTIN_VECTOR_CTOR_3(T) \
DECLARE_BUILTIN_VECTOR_CTOR_4(T) \
DECLARE_BUILTIN_VECTOR_CTOR_8(T) \
DECLARE_BUILTIN_VECTOR_CTOR_16(T) \

#define DECLARE_BUILTIN_VECTOR_FLOAT_ALL(T) \
DECLARE_BUILTIN_VECTOR_FLOAT(T, 2) \
DECLARE_BUILTIN_VECTOR_FLOAT(T, 3) \
DECLARE_BUILTIN_VECTOR_FLOAT(T, 4) \
DECLARE_BUILTIN_VECTOR_FLOAT(T, 8) \
DECLARE_BUILTIN_VECTOR_FLOAT(T, 16) \
DECLARE_BUILTIN_VECTOR_CTOR_2(T) \
DECLARE_BUILTIN_VECTOR_CTOR_3(T) \
DECLARE_BUILTIN_VECTOR_CTOR_4(T) \
DECLARE_BUILTIN_VECTOR_CTOR_8(T) \
DECLARE_BUILTIN_VECTOR_CTOR_16(T) \
DECLARE_BUILTIN_VECTOR_GEOMETRY(T, 2) \
DECLARE_BUILTIN_VECTOR_GEOMETRY(T, 3) \
DECLARE_BUILTIN_VECTOR_GEOMETRY(T, 4) \
DECLARE_BUILTIN_VECTOR_GEOMETRY_CROSS(T, 3) \
DECLARE_BUILTIN_VECTOR_GEOMETRY_CROSS(T, 4) \

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

DECLARE_BUILTIN_VECTOR_UINT_ALL(uchar)
DECLARE_BUILTIN_VECTOR_UINT_ALL(ushort)
DECLARE_BUILTIN_VECTOR_UINT_ALL(uint)
DECLARE_BUILTIN_VECTOR_UINT_ALL(ulong)
DECLARE_BUILTIN_VECTOR_INT_ALL(char)
DECLARE_BUILTIN_VECTOR_INT_ALL(short)
DECLARE_BUILTIN_VECTOR_INT_ALL(int)
DECLARE_BUILTIN_VECTOR_INT_ALL(long)

DECLARE_BUILTIN_VECTOR_FLOAT_ALL(float)
#ifdef DEVICE_DOUBLE
DECLARE_BUILTIN_VECTOR_FLOAT_ALL(double)
#endif // DEVICE_DOUBLE

#ifdef __cplusplus
};
#endif // __cplusplus
