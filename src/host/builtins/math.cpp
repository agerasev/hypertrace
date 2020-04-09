#include <builtins/math.h>

#include <cmath>
#include <algorithm>


#define DEFINE_BUILTIN_FUNCTIONS_COMMON(T) \
T x_clamp_##T(T a, T b, T c) { \
    return std::min(std::max(a, b), c); \
} \
T x_max_##T(T a, T b) { \
    return std::max(a, b); \
} \
T x_min_##T(T a, T b) { \
    return std::min(a, b); \
} \

#define DEFINE_BUILTIN_FUNCTIONS_FLOAT(T) \
DEFINE_BUILTIN_FUNCTIONS_COMMON(T) \
T x_acos_##T(T a) { \
    return std::acos(a); \
} \
T x_acosh_##T(T a) { \
    return std::acosh(a); \
} \
T x_asin_##T(T a) { \
    return std::asin(a); \
} \
T x_asinh_##T(T a) { \
    return std::asinh(a); \
} \
T x_atan_##T(T a) { \
    return std::atan(a); \
} \
T x_atan2_##T(T a, T b) { \
    return std::atan2(a, b); \
} \
T x_atanh_##T(T a) { \
    return std::atanh(a); \
} \
T x_ceil_##T(T a) { \
    return std::ceil(a); \
} \
T x_cos_##T(T a) { \
    return std::cos(a); \
} \
T x_cosh_##T(T a) { \
    return std::cosh(a); \
} \
T x_erfc_##T(T a) { \
    return std::erfc(a); \
} \
T x_erf_##T(T a) { \
    return std::erf(a); \
} \
T x_exp_##T(T a) { \
    return std::exp(a); \
} \
T x_fabs_##T(T a) { \
    return std::fabs(a); \
} \
T x_floor_##T(T a) { \
    return std::floor(a); \
} \
T x_fmax_##T(T a, T b) { \
    return std::fmax(a, b); \
} \
T x_fmin_##T(T a, T b) { \
    return std::fmin(a, b); \
} \
T x_fmod_##T(T a, T b) { \
    return std::fmod(a, b); \
} \
T x_fract_##T(T a, T *b) { \
    *b = std::floor(a); \
    return a - *b; \
} \
T x_log_##T(T a) { \
    return std::log(a); \
} \
T x_pow_##T(T a, T b) { \
    return std::pow(a, b); \
} \
T x_round_##T(T a) { \
    return std::round(a); \
} \
T x_sqrt_##T(T a) { \
    return std::sqrt(a); \
} \
T x_sin_##T(T a) { \
    return std::sin(a); \
} \
T x_sinh_##T(T a) { \
    return std::sinh(a); \
} \
T x_tan_##T(T a) { \
    return std::tan(a); \
} \
T x_tanh_##T(T a) { \
    return std::tanh(a); \
} \
T x_tgamma_##T(T a) { \
    return std::tgamma(a); \
} \

#define DEFINE_BUILTIN_FUNCTIONS_UINT(T) \
DEFINE_BUILTIN_FUNCTIONS_COMMON(T) \
T x_abs_##T(T a) { \
    return a; \
} \

#define DEFINE_BUILTIN_FUNCTIONS_INT(T) \
DEFINE_BUILTIN_FUNCTIONS_COMMON(T) \
T x_abs_##T(T a) { \
    return std::abs(a); \
} \


extern "C" {

DEFINE_BUILTIN_FUNCTIONS_UINT(uchar)
DEFINE_BUILTIN_FUNCTIONS_UINT(ushort)
DEFINE_BUILTIN_FUNCTIONS_UINT(uint)
DEFINE_BUILTIN_FUNCTIONS_UINT(ulong)

DEFINE_BUILTIN_FUNCTIONS_INT(char)
DEFINE_BUILTIN_FUNCTIONS_INT(short)
DEFINE_BUILTIN_FUNCTIONS_INT(int)
DEFINE_BUILTIN_FUNCTIONS_INT(long)

DEFINE_BUILTIN_FUNCTIONS_FLOAT(float)
DEFINE_BUILTIN_FUNCTIONS_FLOAT(double)

};
