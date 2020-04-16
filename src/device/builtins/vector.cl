#include <builtins/vector.h>


#define DEFINE_BUILTIN_VECTOR(T, N) \
void xv_ctor_##T##N##_##T(T *v, T x) { \
    *(T##N*)v = (T##N)(x); \
} \
void xv_load_##T##N(T *v, const T *d) { \
    *(T##N*)v = vload##N(0, d); \
} \
void xv_load_##T##N##_local(T *v, __local const T *d) { \
    *(T##N*)v = vload##N(0, d); \
} \
void xv_load_##T##N##_global(T *v, __global const T *d) { \
    *(T##N*)v = vload##N(0, d); \
} \
void xv_load_##T##N##_constant(T *v, __constant const T *d) { \
    *(T##N*)v = vload##N(0, d); \
} \
void xv_store_##T##N(const T *v, T *d) { \
    vstore##N(*(const T##N*)v, 0, d); \
} \
void xv_store_##T##N##_local(const T *v, __local T *d) { \
    vstore##N(*(const T##N*)v, 0, d); \
} \
void xv_store_##T##N##_global(const T *v, __global T *d) { \
    vstore##N(*(const T##N*)v, 0, d); \
} \
void xv_neg_##T##N(T *o, const T *a) { \
    *(T##N*)o = -*(const T##N*)a; \
} \
void xv_add_##T##N##_##T##N(T *o, const T *a, const T *b) { \
    *(T##N*)o = *(const T##N*)a + *(const T##N*)b; \
} \
void xv_add_##T##N##_##T(T *o, const T *a, T b) { \
    *(T##N*)o = *(const T##N*)a + b; \
} \
void xv_add_##T##_##T##N(T *o, T a, const T *b) { \
    *(T##N*)o = a + *(const T##N*)b; \
} \
void xv_sub_##T##N##_##T##N(T *o, const T *a, const T *b) { \
    *(T##N*)o = *(const T##N*)a - *(const T##N*)b; \
} \
void xv_sub_##T##N##_##T(T *o, const T *a, T b) { \
    *(T##N*)o = *(const T##N*)a - b; \
} \
void xv_sub_##T##_##T##N(T *o, T a, const T *b) { \
    *(T##N*)o = a - *(const T##N*)b; \
} \
void xv_mul_##T##N##_##T##N(T *o, const T *a, const T *b) { \
    *(T##N*)o = *(const T##N*)a * *(const T##N*)b; \
} \
void xv_mul_##T##N##_##T(T *o, const T *a, T b) { \
    *(T##N*)o = *(const T##N*)a * b; \
} \
void xv_mul_##T##_##T##N(T *o, T a, const T *b) { \
    *(T##N*)o = a * *(const T##N*)b; \
} \
void xv_div_##T##N##_##T##N(T *o, const T *a, const T *b) { \
    *(T##N*)o = *(const T##N*)a / *(const T##N*)b; \
} \
void xv_div_##T##N##_##T(T *o, const T *a, T b) { \
    *(T##N*)o = *(const T##N*)a / b; \
} \
void xv_div_##T##_##T##N(T *o, T a, const T *b) { \
    *(T##N*)o = a / *(const T##N*)b; \
} \
void xv_addassign_##T##N##_##T##N(T *o, const T *a) { \
    *(T##N*)o += *(const T##N*)a; \
} \
void xv_addassign_##T##N##_##T(T *o, T a) { \
    *(T##N*)o += a; \
} \
void xv_subassign_##T##N##_##T##N(T *o, const T *a) { \
    *(T##N*)o -= *(const T##N*)a; \
} \
void xv_subassign_##T##N##_##T(T *o, T a) { \
    *(T##N*)o -= a; \
} \
void xv_mulassign_##T##N##_##T##N(T *o, const T *a) { \
    *(T##N*)o *= *(const T##N*)a; \
} \
void xv_mulassign_##T##N##_##T(T *o, T a) { \
    *(T##N*)o *= a; \
} \
void xv_divassign_##T##N##_##T##N(T *o, const T *a) { \
    *(T##N*)o /= *(const T##N*)a; \
} \
void xv_divassign_##T##N##_##T(T *o, T a) { \
    *(T##N*)o /= a; \
} \

#define DEFINE_BUILTIN_VECTOR_CTOR_2(T) \
void xv_ctor_##T##2##_##T##_2(T *v, T x, T y) { \
    *(T##2*)v = (T##2)(x, y); \
} \

#define DEFINE_BUILTIN_VECTOR_CTOR_3(T) \
void xv_ctor_##T##3##_##T##_3(T *v, T x, T y, T z) { \
    *(T##3*)v = (T##3)(x, y, z); \
} \

#define DEFINE_BUILTIN_VECTOR_CTOR_4(T) \
void xv_ctor_##T##4##_##T##_4(T *v, T x, T y, T z, T w) { \
    *(T##4*)v = (T##4)(x, y, z, w); \
} \

#define DEFINE_BUILTIN_VECTOR_CTOR_8(T) \
void xv_ctor_##T##8##_##T##_8( \
    T *v, \
    T s0, T s1, T s2, T s3, T s4, T s5, T s6, T s7 \
) { \
    *(T##8*)v = (T##8)( \
        s0, s1, s2, s3, s4, s5, s6, s7 \
    ); \
} \

#define DEFINE_BUILTIN_VECTOR_CTOR_16(T) \
void xv_ctor_##T##16##_##T##_16( \
    T *v, \
    T s0, T s1, T s2, T s3, T s4, T s5, T s6, T s7, \
    T s8, T s9, T sA, T sB, T sC, T sD, T sE, T sF \
) { \
    *(T##16*)v = (T##16)( \
        s0, s1, s2, s3, s4, s5, s6, s7, \
        s8, s9, sA, sB, sC, sD, sE, sF \
    ); \
} \

#define DEFINE_BUILTIN_VECTOR_FUNCTIONS_COMMON(T, N) \
void xv_clamp_##T##N(T *o, const T *a, const T *b, const T *c) { \
    *(T##N*)o = clamp(*(const T##N*)a, *(const T##N*)b, *(const T##N*)c); \
} \
void xv_clamp_##T##N##_(T *o, const T *a, T b, T c) { \
    *(T##N*)o = clamp(*(const T##N*)a, b, c); \
} \
void xv_max_##T##N(T *o, const T *a, const T *b) { \
    *(T##N*)o = max(*(const T##N*)a, *(const T##N*)b); \
} \
void xv_max_##T##N##_(T *o, const T *a, T b) { \
    *(T##N*)o = max(*(const T##N*)a, b); \
} \
void xv_min_##T##N(T *o, const T *a, const T *b) { \
    *(T##N*)o = min(*(const T##N*)a, *(const T##N*)b); \
} \
void xv_min_##T##N##_(T *o, const T *a, T b) { \
    *(T##N*)o = min(*(const T##N*)a, b); \
} \

#define DEFINE_BUILTIN_VECTOR_FUNCTIONS_UINT(T, N) \
DEFINE_BUILTIN_VECTOR_FUNCTIONS_COMMON(T, N) \
void xv_abs_##T##N(T *o, const T *a) { \
    *(T##N*)o = abs(*(const T##N*)a); \
} \

#define DEFINE_BUILTIN_VECTOR_FUNCTIONS_INT(T, N) \
DEFINE_BUILTIN_VECTOR_FUNCTIONS_COMMON(T, N) \
void xv_abs_##T##N(T *o, const T *a) { \
    *(u##T##N*)o = abs(*(const T##N*)a); \
} \

#define DEFINE_BUILTIN_VECTOR_FUNCTIONS_FLOAT(T, N) \
DEFINE_BUILTIN_VECTOR_FUNCTIONS_COMMON(T, N) \
void xv_acos_##T##N(T *o, const T *a) { \
    *(T##N*)o = acos(*(const T##N*)a); \
} \
void xv_acosh_##T##N(T *o, const T *a) { \
    *(T##N*)o = acosh(*(const T##N*)a); \
} \
void xv_asin_##T##N(T *o, const T *a) { \
    *(T##N*)o = asin(*(const T##N*)a); \
} \
void xv_asinh_##T##N(T *o, const T *a) { \
    *(T##N*)o = asinh(*(const T##N*)a); \
} \
void xv_atan_##T##N(T *o, const T *a) { \
    *(T##N*)o = atan(*(const T##N*)a); \
} \
void xv_atan2_##T##N(T *o, const T *a, const T *b) { \
    *(T##N*)o = atan2(*(const T##N*)a, *(const T##N*)b); \
} \
void xv_atanh_##T##N(T *o, const T *a) { \
    *(T##N*)o = atanh(*(const T##N*)a); \
} \
void xv_ceil_##T##N(T *o, const T *a) { \
    *(T##N*)o = ceil(*(const T##N*)a); \
} \
void xv_cos_##T##N(T *o, const T *a) { \
    *(T##N*)o = cos(*(const T##N*)a); \
} \
void xv_cosh_##T##N(T *o, const T *a) { \
    *(T##N*)o = cosh(*(const T##N*)a); \
} \
void xv_erfc_##T##N(T *o, const T *a) { \
    *(T##N*)o = erfc(*(const T##N*)a); \
} \
void xv_erf_##T##N(T *o, const T *a) { \
    *(T##N*)o = erf(*(const T##N*)a); \
} \
void xv_exp_##T##N(T *o, const T *a) { \
    *(T##N*)o = exp(*(const T##N*)a); \
} \
void xv_fabs_##T##N(T *o, const T *a) { \
    *(T##N*)o = fabs(*(const T##N*)a); \
} \
void xv_floor_##T##N(T *o, const T *a) { \
    *(T##N*)o = floor(*(const T##N*)a); \
} \
void xv_fmax_##T##N(T *o, const T *a, const T *b) { \
    *(T##N*)o = fmax(*(const T##N*)a, *(const T##N*)b); \
} \
void xv_fmin_##T##N(T *o, const T *a, const T *b) { \
    *(T##N*)o = fmin(*(const T##N*)a, *(const T##N*)b); \
} \
void xv_fmod_##T##N(T *o, const T *a, const T *b) { \
    *(T##N*)o = fmod(*(const T##N*)a, *(const T##N*)b); \
} \
void xv_fract_##T##N(T *o, const T *a, T *b) { \
    *(T##N*)o = fract(*(const T##N*)a, (T##N*)b); \
} \
void xv_log_##T##N(T *o, const T *a) { \
    *(T##N*)o = log(*(const T##N*)a); \
} \
void xv_pow_##T##N(T *o, const T *a, const T *b) { \
    *(T##N*)o = pow(*(const T##N*)a, *(const T##N*)b); \
} \
void xv_round_##T##N(T *o, const T *a) { \
    *(T##N*)o = round(*(const T##N*)a); \
} \
void xv_sqrt_##T##N(T *o, const T *a) { \
    *(T##N*)o = sqrt(*(const T##N*)a); \
} \
void xv_sin_##T##N(T *o, const T *a) { \
    *(T##N*)o = sin(*(const T##N*)a); \
} \
void xv_sinh_##T##N(T *o, const T *a) { \
    *(T##N*)o = sinh(*(const T##N*)a); \
} \
void xv_tan_##T##N(T *o, const T *a) { \
    *(T##N*)o = tan(*(const T##N*)a); \
} \
void xv_tanh_##T##N(T *o, const T *a) { \
    *(T##N*)o = tanh(*(const T##N*)a); \
} \
void xv_tgamma_##T##N(T *o, const T *a) { \
    *(T##N*)o = tgamma(*(const T##N*)a); \
} \


#define DEFINE_BUILTIN_VECTOR_UINT(T, N) \
DEFINE_BUILTIN_VECTOR(T, N) \
DEFINE_BUILTIN_VECTOR_FUNCTIONS_UINT(T, N) \

#define DEFINE_BUILTIN_VECTOR_INT(T, N) \
DEFINE_BUILTIN_VECTOR(T, N) \
DEFINE_BUILTIN_VECTOR_FUNCTIONS_INT(T, N) \

#define DEFINE_BUILTIN_VECTOR_FLOAT(T, N) \
DEFINE_BUILTIN_VECTOR(T, N) \
DEFINE_BUILTIN_VECTOR_FUNCTIONS_FLOAT(T, N) \

#define DEFINE_BUILTIN_VECTOR_GEOMETRY(T, N) \
T xv_dot_##T##N(const T *a, const T *b) { \
    return dot(*(const T##N*)a, *(const T##N*)b); \
} \
T xv_distance_##T##N(const T *a, const T *b) { \
    return distance(*(const T##N*)a, *(const T##N*)b); \
} \
T xv_length_##T##N(const T *a) { \
    return length(*(const T##N*)a); \
} \
void xv_normalize_##T##N(T *o, const T *a) { \
    *(T##N*)o = normalize(*(const T##N*)a); \
} \

#define DEFINE_BUILTIN_VECTOR_GEOMETRY_CROSS(T, N) \
void xv_cross_##T##N(T *o, const T *a, const T *b) { \
    *(T##N*)o = cross(*(const T##N*)a, *(const T##N*)b); \
} \

#define DEFINE_BUILTIN_VECTOR_CONVERT(S, T, N) \
void xv_convert_##S##N##_##T##N(S *o, const T *a) { \
    *(S##N*)o = convert_##S##N(*(const T##N*)a); \
} \

#define DEFINE_BUILTIN_VECTOR_UINT_ALL(T) \
DEFINE_BUILTIN_VECTOR_UINT(T, 2) \
DEFINE_BUILTIN_VECTOR_UINT(T, 3) \
DEFINE_BUILTIN_VECTOR_UINT(T, 4) \
DEFINE_BUILTIN_VECTOR_UINT(T, 8) \
DEFINE_BUILTIN_VECTOR_UINT(T, 16) \
DEFINE_BUILTIN_VECTOR_CTOR_2(T) \
DEFINE_BUILTIN_VECTOR_CTOR_3(T) \
DEFINE_BUILTIN_VECTOR_CTOR_4(T) \
DEFINE_BUILTIN_VECTOR_CTOR_8(T) \
DEFINE_BUILTIN_VECTOR_CTOR_16(T) \

#define DEFINE_BUILTIN_VECTOR_INT_ALL(T) \
DEFINE_BUILTIN_VECTOR_INT(T, 2) \
DEFINE_BUILTIN_VECTOR_INT(T, 3) \
DEFINE_BUILTIN_VECTOR_INT(T, 4) \
DEFINE_BUILTIN_VECTOR_INT(T, 8) \
DEFINE_BUILTIN_VECTOR_INT(T, 16) \
DEFINE_BUILTIN_VECTOR_CTOR_2(T) \
DEFINE_BUILTIN_VECTOR_CTOR_3(T) \
DEFINE_BUILTIN_VECTOR_CTOR_4(T) \
DEFINE_BUILTIN_VECTOR_CTOR_8(T) \
DEFINE_BUILTIN_VECTOR_CTOR_16(T) \

#define DEFINE_BUILTIN_VECTOR_FLOAT_ALL(T) \
DEFINE_BUILTIN_VECTOR_FLOAT(T, 2) \
DEFINE_BUILTIN_VECTOR_FLOAT(T, 3) \
DEFINE_BUILTIN_VECTOR_FLOAT(T, 4) \
DEFINE_BUILTIN_VECTOR_FLOAT(T, 8) \
DEFINE_BUILTIN_VECTOR_FLOAT(T, 16) \
DEFINE_BUILTIN_VECTOR_CTOR_2(T) \
DEFINE_BUILTIN_VECTOR_CTOR_3(T) \
DEFINE_BUILTIN_VECTOR_CTOR_4(T) \
DEFINE_BUILTIN_VECTOR_CTOR_8(T) \
DEFINE_BUILTIN_VECTOR_CTOR_16(T) \
DEFINE_BUILTIN_VECTOR_GEOMETRY(T, 2) \
DEFINE_BUILTIN_VECTOR_GEOMETRY(T, 3) \
DEFINE_BUILTIN_VECTOR_GEOMETRY(T, 4) \
DEFINE_BUILTIN_VECTOR_GEOMETRY_CROSS(T, 3) \
DEFINE_BUILTIN_VECTOR_GEOMETRY_CROSS(T, 4) \


#ifdef DEVICE_DOUBLE
#define DEFINE_BUILTIN_VECTOR_IF_DOUBLE_ENABLED(DEF) DEF
#else
#define DEFINE_BUILTIN_VECTOR_IF_DOUBLE_ENABLED(DEF)
#endif

#define DEFINE_BUILTIN_VECTOR_CONVERT_DST(T, N) \
DEFINE_BUILTIN_VECTOR_CONVERT(uchar, T, N) \
DEFINE_BUILTIN_VECTOR_CONVERT(ushort, T, N) \
DEFINE_BUILTIN_VECTOR_CONVERT(uint, T, N) \
DEFINE_BUILTIN_VECTOR_CONVERT(ulong, T, N) \
DEFINE_BUILTIN_VECTOR_CONVERT(char, T, N) \
DEFINE_BUILTIN_VECTOR_CONVERT(short, T, N) \
DEFINE_BUILTIN_VECTOR_CONVERT(int, T, N) \
DEFINE_BUILTIN_VECTOR_CONVERT(long, T, N) \
DEFINE_BUILTIN_VECTOR_CONVERT(float, T, N) \
DEFINE_BUILTIN_VECTOR_IF_DOUBLE_ENABLED( \
    DEFINE_BUILTIN_VECTOR_CONVERT(double, T, N) \
) \

#define DEFINE_BUILTIN_VECTOR_CONVERT_ALL(N) \
DEFINE_BUILTIN_VECTOR_CONVERT_DST(uchar, N) \
DEFINE_BUILTIN_VECTOR_CONVERT_DST(ushort, N) \
DEFINE_BUILTIN_VECTOR_CONVERT_DST(uint, N) \
DEFINE_BUILTIN_VECTOR_CONVERT_DST(ulong, N) \
DEFINE_BUILTIN_VECTOR_CONVERT_DST(char, N) \
DEFINE_BUILTIN_VECTOR_CONVERT_DST(short, N) \
DEFINE_BUILTIN_VECTOR_CONVERT_DST(int, N) \
DEFINE_BUILTIN_VECTOR_CONVERT_DST(long, N) \
DEFINE_BUILTIN_VECTOR_CONVERT_DST(float, N) \
DEFINE_BUILTIN_VECTOR_IF_DOUBLE_ENABLED( \
    DEFINE_BUILTIN_VECTOR_CONVERT_DST(double, N) \
) \

DEFINE_BUILTIN_VECTOR_UINT_ALL(uchar)
DEFINE_BUILTIN_VECTOR_UINT_ALL(ushort)
DEFINE_BUILTIN_VECTOR_UINT_ALL(uint)
DEFINE_BUILTIN_VECTOR_UINT_ALL(ulong)

DEFINE_BUILTIN_VECTOR_INT_ALL(char)
DEFINE_BUILTIN_VECTOR_INT_ALL(short)
DEFINE_BUILTIN_VECTOR_INT_ALL(int)
DEFINE_BUILTIN_VECTOR_INT_ALL(long)

DEFINE_BUILTIN_VECTOR_FLOAT_ALL(float)
DEFINE_BUILTIN_VECTOR_IF_DOUBLE_ENABLED(
    DEFINE_BUILTIN_VECTOR_FLOAT_ALL(double)
)

DEFINE_BUILTIN_VECTOR_CONVERT_ALL(2)
DEFINE_BUILTIN_VECTOR_CONVERT_ALL(3)
DEFINE_BUILTIN_VECTOR_CONVERT_ALL(4)
DEFINE_BUILTIN_VECTOR_CONVERT_ALL(8)
DEFINE_BUILTIN_VECTOR_CONVERT_ALL(16)
