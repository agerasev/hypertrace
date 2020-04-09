#pragma once

#include <types.h>

// Operations

#define DECLARE_BUILTIN_VECTOR(T, N) \
void xv_ctor_##T##N##_##T(T *v, T x); \
void xv_load_##T##N(T *v, const T *d); \
void xv_store_##T##N(const T *v, T *d); \
void xv_neg_##T##N(T *o, const T *a); \
void xv_add_##T##N##_##T##N(T *o, const T *a, const T *b); \
void xv_add_##T##N##_##T(T *o, const T *a, T b); \
void xv_add_##T##_##T##N(T *o, T a, const T *b); \
void xv_sub_##T##N##_##T##N(T *o, const T *a, const T *b); \
void xv_sub_##T##N##_##T(T *o, const T *a, T b); \
void xv_sub_##T##_##T##N(T *o, T a, const T *b); \
void xv_mul_##T##N##_##T##N(T *o, const T *a, const T *b); \
void xv_mul_##T##N##_##T(T *o, const T *a, T b); \
void xv_mul_##T##_##T##N(T *o, T a, const T *b); \
void xv_div_##T##N##_##T##N(T *o, const T *a, const T *b); \
void xv_div_##T##N##_##T(T *o, const T *a, T b); \
void xv_div_##T##_##T##N(T *o, T a, const T *b); \
void xv_addassign_##T##N##_##T##N(T *o, const T *a); \
void xv_addassign_##T##N##_##T(T *o, T a); \
void xv_subassign_##T##N##_##T##N(T *o, const T *a); \
void xv_subassign_##T##N##_##T(T *o, T a); \
void xv_mulassign_##T##N##_##T##N(T *o, const T *a); \
void xv_mulassign_##T##N##_##T(T *o, T a); \
void xv_divassign_##T##N##_##T##N(T *o, const T *a); \
void xv_divassign_##T##N##_##T(T *o, T a); \

// Constructors

#define DECLARE_BUILTIN_VECTOR_CTOR_2(T) \
void xv_ctor_##T##2_##T##_2(T *v, T x, T y); \

#define DECLARE_BUILTIN_VECTOR_CTOR_3(T) \
void xv_ctor_##T##3_##T##_3(T *v, T x, T y, T z); \

#define DECLARE_BUILTIN_VECTOR_CTOR_4(T) \
void xv_ctor_##T##4_##T##_4(T *v, T x, T y, T z, T w); \

#define DECLARE_BUILTIN_VECTOR_CTOR_8(T) \
void xv_ctor_##T##8_##T##_8( \
    T *v, \
    T s0, T s1, T s2, T s3, T s4, T s5, T s6, T s7 \
); \

#define DECLARE_BUILTIN_VECTOR_CTOR_16(T) \
void xv_ctor_##T##16_##T##_16( \
    T *v, \
    T s0, T s1, T s2, T s3, T s4, T s5, T s6, T s7, \
    T s8, T s9, T sA, T sB, T sC, T sD, T sE, T sF \
); \

// Functions

#define DECLARE_BUILTIN_VECTOR_FUNCTIONS_COMMON(T, N) \
void xv_clamp_##T##N(T *o, const T *a, const T *b, const T *c); \
void xv_clamp_##T##N##_(T *o, const T *a, T b, T c); \
void xv_max_##T##N(T *o, const T *a, const T *b); \
void xv_max_##T##N##_(T *o, const T *a, T b); \
void xv_min_##T##N(T *o, const T *a, const T *b); \
void xv_min_##T##N##_(T *o, const T *a, T b); \

#define DECLARE_BUILTIN_VECTOR_FUNCTIONS_UINT(T, N) \
DECLARE_BUILTIN_VECTOR_FUNCTIONS_COMMON(T, N) \
void xv_abs_##T##N(T *o, const T *a); \

#define DECLARE_BUILTIN_VECTOR_FUNCTIONS_INT(T, N) \
DECLARE_BUILTIN_VECTOR_FUNCTIONS_COMMON(T, N) \
void xv_abs_##T##N(T *o, const T *a); \

#define DECLARE_BUILTIN_VECTOR_FUNCTIONS_FLOAT(T, N) \
DECLARE_BUILTIN_VECTOR_FUNCTIONS_COMMON(T, N) \
void xv_acos_##T##N(T *o, const T *a); \
void xv_acosh_##T##N(T *o, const T *a); \
void xv_asin_##T##N(T *o, const T *a); \
void xv_asinh_##T##N(T *o, const T *a); \
void xv_atan_##T##N(T *o, const T *a); \
void xv_atan2_##T##N(T *o, const T *a, const T *b); \
void xv_atanh_##T##N(T *o, const T *a); \
void xv_ceil_##T##N(T *o, const T *a); \
void xv_cos_##T##N(T *o, const T *a); \
void xv_cosh_##T##N(T *o, const T *a); \
void xv_erfc_##T##N(T *o, const T *a); \
void xv_erf_##T##N(T *o, const T *a); \
void xv_exp_##T##N(T *o, const T *a); \
void xv_fabs_##T##N(T *o, const T *a); \
void xv_floor_##T##N(T *o, const T *a); \
void xv_fmax_##T##N(T *o, const T *a, const T *b); \
void xv_fmin_##T##N(T *o, const T *a, const T *b); \
void xv_fmod_##T##N(T *o, const T *a, const T *b); \
void xv_fract_##T##N(T *o, const T *a, T *b); \
void xv_log_##T##N(T *o, const T *a); \
void xv_pow_##T##N(T *o, const T *a, const T *b); \
void xv_round_##T##N(T *o, const T *a); \
void xv_sqrt_##T##N(T *o, const T *a); \
void xv_sin_##T##N(T *o, const T *a); \
void xv_sinh_##T##N(T *o, const T *a); \
void xv_tan_##T##N(T *o, const T *a); \
void xv_tanh_##T##N(T *o, const T *a); \
void xv_tgamma_##T##N(T *o, const T *a); \

#define DECLARE_BUILTIN_VECTOR_UINT(T, N) \
DECLARE_BUILTIN_VECTOR(T, N) \
DECLARE_BUILTIN_VECTOR_FUNCTIONS_UINT(T, N) \

#define DECLARE_BUILTIN_VECTOR_INT(T, N) \
DECLARE_BUILTIN_VECTOR(T, N) \
DECLARE_BUILTIN_VECTOR_FUNCTIONS_INT(T, N) \

#define DECLARE_BUILTIN_VECTOR_FLOAT(T, N) \
DECLARE_BUILTIN_VECTOR(T, N) \
DECLARE_BUILTIN_VECTOR_FUNCTIONS_FLOAT(T, N) \

// Geometry

#define DECLARE_BUILTIN_VECTOR_GEOMETRY(T, N) \
T xv_dot_##T##N(const T *a, const T *b); \
T xv_distance_##T##N(const T *a, const T *b); \
T xv_length_##T##N(const T *a); \
void xv_normalize_##T##N(T *o, const T *a); \

#define DECLARE_BUILTIN_VECTOR_GEOMETRY_CROSS(T, N) \
void xv_cross_##T##N(T *o, const T *a, const T *b); \

// All in one declarations

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
