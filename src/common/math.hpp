#pragma once

#include <types.h>
#include <builtins/math.h>


#define DEFINE_PRIMITIVE_MATH_COMMON(T) \
inline T clamp(T a, T b, T c) { \
    return x_clamp_##T(a, b, c); \
} \

#define DEFINE_PRIMITIVE_MATH_FLOAT(T) \
DEFINE_PRIMITIVE_MATH_COMMON(T) \
inline T acos(T a) { \
    return x_acos_##T(a); \
} \
inline T acosh(T a) { \
    return x_acosh_##T(a); \
} \
inline T asin(T a) { \
    return x_asin_##T(a); \
} \
inline T asinh(T a) { \
    return x_asinh_##T(a); \
} \
inline T atan(T a) { \
    return x_atan_##T(a); \
} \
inline T atan2(T a, T b) { \
    return x_atan2_##T(a, b); \
} \
inline T atanh(T a) { \
    return x_atanh_##T(a); \
} \
inline T ceil(T a) { \
    return x_ceil_##T(a); \
} \
inline T cos(T a) { \
    return x_cos_##T(a); \
} \
inline T cosh(T a) { \
    return x_cosh_##T(a); \
} \
inline T erfc(T a) { \
    return x_erfc_##T(a); \
} \
inline T erf(T a) { \
    return x_erf_##T(a); \
} \
inline T exp(T a) { \
    return x_exp_##T(a); \
} \
inline T abs(T a) { \
    return x_fabs_##T(a); \
} \
inline T floor(T a) { \
    return x_floor_##T(a); \
} \
inline T max(T a, T b) { \
    return x_fmax_##T(a, b); \
} \
inline T min(T a, T b) { \
    return x_fmin_##T(a, b); \
} \
inline T mod(T a, T b) { \
    return x_fmod_##T(a, b); \
} \
inline pair<T, T> fract(T a) { \
    pair<T, T> r; \
    r.first = x_fract_##T(a, &r.second); \
    return r; \
} \
inline T log(T a) { \
    return x_log_##T(a); \
} \
inline T pow(T a, T b) { \
    return x_pow_##T(a, b); \
} \
inline T round(T a) { \
    return x_round_##T(a); \
} \
inline T sqrt(T a) { \
    return x_sqrt_##T(a); \
} \
inline T sin(T a) { \
    return x_sin_##T(a); \
} \
inline T sinh(T a) { \
    return x_sinh_##T(a); \
} \
inline T tan(T a) { \
    return x_tan_##T(a); \
} \
inline T tanh(T a) { \
    return x_tanh_##T(a); \
} \
inline T tgamma(T a) { \
    return x_tgamma_##T(a); \
} \

#define DEFINE_PRIMITIVE_MATH_INT(T) \
DEFINE_PRIMITIVE_MATH_COMMON(T) \
inline T abs(T a) { \
    return x_abs_##T(a); \
} \
inline T max(T a, T b) { \
    return x_max_##T(a, b); \
} \
inline T min(T a, T b) { \
    return x_min_##T(a, b); \
} \

#define DEFINE_PRIMITIVE_MATH_UINT(T) \
DEFINE_PRIMITIVE_MATH_INT(T) \


namespace math {

DEFINE_PRIMITIVE_MATH_UINT(uchar)
DEFINE_PRIMITIVE_MATH_UINT(ushort)
DEFINE_PRIMITIVE_MATH_UINT(uint)
DEFINE_PRIMITIVE_MATH_UINT(ulong)

DEFINE_PRIMITIVE_MATH_INT(char)
DEFINE_PRIMITIVE_MATH_INT(short)
DEFINE_PRIMITIVE_MATH_INT(int)
DEFINE_PRIMITIVE_MATH_INT(long)

DEFINE_PRIMITIVE_MATH_FLOAT(float)
#if defined(HOST) || defined(DEVICE_DOUBLE)
DEFINE_PRIMITIVE_MATH_FLOAT(double)
#endif

template <typename T>
T mod(T x, T y) {
    return x % y;
}
template <typename T>
T rem(T x, T y) {
    return math::mod(math::mod(x, y) + y, y);
}

}
