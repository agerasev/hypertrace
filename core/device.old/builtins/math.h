#pragma once

#include <types.h>


#define DECLARE_BUILTIN_FUNCTIONS_COMMON(T) \
T x_clamp_##T(T a, T b, T c); \
T x_max_##T(T a, T b); \
T x_min_##T(T a, T b); \

#define DECLARE_BUILTIN_FUNCTIONS_FLOAT(T) \
DECLARE_BUILTIN_FUNCTIONS_COMMON(T) \
T x_acos_##T(T a); \
T x_acosh_##T(T a); \
T x_asin_##T(T a); \
T x_asinh_##T(T a); \
T x_atan_##T(T a); \
T x_atan2_##T(T a, T b); \
T x_atanh_##T(T a); \
T x_ceil_##T(T a); \
T x_cos_##T(T a); \
T x_cosh_##T(T a); \
T x_erfc_##T(T a); \
T x_erf_##T(T a); \
T x_exp_##T(T a); \
T x_fabs_##T(T a); \
T x_floor_##T(T a); \
T x_fmax_##T(T a, T b); \
T x_fmin_##T(T a, T b); \
T x_fmod_##T(T a, T b); \
T x_fract_##T(T a, T *b); \
T x_log_##T(T a); \
T x_pow_##T(T a, T b); \
T x_round_##T(T a); \
T x_sqrt_##T(T a); \
T x_sin_##T(T a); \
T x_sinh_##T(T a); \
T x_tan_##T(T a); \
T x_tanh_##T(T a); \
T x_tgamma_##T(T a); \
T x_sign_##T(T a); \

#define DECLARE_BUILTIN_FUNCTIONS_UINT(T) \
DECLARE_BUILTIN_FUNCTIONS_COMMON(T) \
T x_abs_##T(T a); \

#define DECLARE_BUILTIN_FUNCTIONS_INT(T) \
DECLARE_BUILTIN_FUNCTIONS_COMMON(T) \
T x_abs_##T(T a); \


#ifdef __cplusplus
extern "C" {
#endif

DECLARE_BUILTIN_FUNCTIONS_UINT(uchar)
DECLARE_BUILTIN_FUNCTIONS_UINT(ushort)
DECLARE_BUILTIN_FUNCTIONS_UINT(uint)
DECLARE_BUILTIN_FUNCTIONS_UINT(ulong)

DECLARE_BUILTIN_FUNCTIONS_INT(char)
DECLARE_BUILTIN_FUNCTIONS_INT(short)
DECLARE_BUILTIN_FUNCTIONS_INT(int)
DECLARE_BUILTIN_FUNCTIONS_INT(long)

DECLARE_BUILTIN_FUNCTIONS_FLOAT(float)
#ifdef DEVICE_DOUBLE
DECLARE_BUILTIN_FUNCTIONS_FLOAT(double)
#endif

#ifdef __cplusplus
};
#endif
