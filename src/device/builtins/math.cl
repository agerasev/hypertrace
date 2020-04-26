#include <builtins/math.h>


#define DEFINE_BUILTIN_FUNCTIONS_COMMON(T) \
T x_clamp_##T(T a, T b, T c) { \
    return clamp(a, b, c); \
} \
T x_max_##T(T a, T b) { \
    return max(a, b); \
} \
T x_min_##T(T a, T b) { \
    return min(a, b); \
} \

#define DEFINE_BUILTIN_FUNCTIONS_FLOAT(T) \
DEFINE_BUILTIN_FUNCTIONS_COMMON(T) \
T x_acos_##T(T a) { \
    return acos(a); \
} \
T x_acosh_##T(T a) { \
    return acosh(a); \
} \
T x_asin_##T(T a) { \
    return asin(a); \
} \
T x_asinh_##T(T a) { \
    return asinh(a); \
} \
T x_atan_##T(T a) { \
    return atan(a); \
} \
T x_atan2_##T(T a, T b) { \
    return atan2(a, b); \
} \
T x_atanh_##T(T a) { \
    return atanh(a); \
} \
T x_ceil_##T(T a) { \
    return ceil(a); \
} \
T x_cos_##T(T a) { \
    return cos(a); \
} \
T x_cosh_##T(T a) { \
    return cosh(a); \
} \
T x_erfc_##T(T a) { \
    return erfc(a); \
} \
T x_erf_##T(T a) { \
    return erf(a); \
} \
T x_exp_##T(T a) { \
    return exp(a); \
} \
T x_fabs_##T(T a) { \
    return fabs(a); \
} \
T x_floor_##T(T a) { \
    return floor(a); \
} \
T x_fmax_##T(T a, T b) { \
    return fmax(a, b); \
} \
T x_fmin_##T(T a, T b) { \
    return fmin(a, b); \
} \
T x_fmod_##T(T a, T b) { \
    return fmod(a, b); \
} \
T x_fract_##T(T a, T *b) { \
    return fract(a, b); \
} \
T x_log_##T(T a) { \
    return log(a); \
} \
T x_pow_##T(T a, T b) { \
    return pow(a, b); \
} \
T x_round_##T(T a) { \
    return round(a); \
} \
T x_sqrt_##T(T a) { \
    return sqrt(a); \
} \
T x_sin_##T(T a) { \
    return sin(a); \
} \
T x_sinh_##T(T a) { \
    return sinh(a); \
} \
T x_tan_##T(T a) { \
    return tan(a); \
} \
T x_tanh_##T(T a) { \
    return tanh(a); \
} \
T x_tgamma_##T(T a) { \
    return tgamma(a); \
} \
T x_sign_##T(T a) { \
    return sign(a); \
} \

#define DEFINE_BUILTIN_FUNCTIONS_UINT(T) \
DEFINE_BUILTIN_FUNCTIONS_COMMON(T) \
T x_abs_##T(T a) { \
    return abs(a); \
} \

#define DEFINE_BUILTIN_FUNCTIONS_INT(T) \
DEFINE_BUILTIN_FUNCTIONS_COMMON(T) \
T x_abs_##T(T a) { \
    return abs(a); \
} \


DEFINE_BUILTIN_FUNCTIONS_UINT(uchar)
DEFINE_BUILTIN_FUNCTIONS_UINT(ushort)
DEFINE_BUILTIN_FUNCTIONS_UINT(uint)
DEFINE_BUILTIN_FUNCTIONS_UINT(ulong)

DEFINE_BUILTIN_FUNCTIONS_INT(char)
DEFINE_BUILTIN_FUNCTIONS_INT(short)
DEFINE_BUILTIN_FUNCTIONS_INT(int)
DEFINE_BUILTIN_FUNCTIONS_INT(long)

DEFINE_BUILTIN_FUNCTIONS_FLOAT(float)
#ifdef DEVICE_DOUBLE
DEFINE_BUILTIN_FUNCTIONS_FLOAT(double)
#endif
