#include "builtin.h"

#define DEFINE_BUILTIN_VECTOR(T, N) \
void ocl_ctor_##T##N##_##T(T *v, T x) { \
    *(T##N*)v = (T##N)(x); \
} \
void ocl_load_##T##N(T *v, const T *d) { \
    *(T##N*)v = vload##N(0, d); \
} \
void ocl_store_##T##N(const T *v, T *d) { \
    vstore##N(*(const T##N*)v, 0, d); \
} \
void ocl_neg_##T##N(T *o, const T *a) { \
    *(T##N*)o = -*(const T##N*)a; \
} \
void ocl_add_##T##N##_##T##N(T *o, const T *a, const T *b) { \
    *(T##N*)o = *(const T##N*)a + *(const T##N*)b; \
} \
void ocl_add_##T##N##_##T(T *o, const T *a, T b) { \
    *(T##N*)o = *(const T##N*)a + b; \
} \
void ocl_add_##T##_##T##N(T *o, T a, const T *b) { \
    *(T##N*)o = a + *(const T##N*)b; \
} \
void ocl_sub_##T##N##_##T##N(T *o, const T *a, const T *b) { \
    *(T##N*)o = *(const T##N*)a - *(const T##N*)b; \
} \
void ocl_sub_##T##N##_##T(T *o, const T *a, T b) { \
    *(T##N*)o = *(const T##N*)a - b; \
} \
void ocl_sub_##T##_##T##N(T *o, T a, const T *b) { \
    *(T##N*)o = a - *(const T##N*)b; \
} \
void ocl_mul_##T##N##_##T##N(T *o, const T *a, const T *b) { \
    *(T##N*)o = *(const T##N*)a * *(const T##N*)b; \
} \
void ocl_mul_##T##N##_##T(T *o, const T *a, T b) { \
    *(T##N*)o = *(const T##N*)a * b; \
} \
void ocl_mul_##T##_##T##N(T *o, T a, const T *b) { \
    *(T##N*)o = a * *(const T##N*)b; \
} \
void ocl_div_##T##N##_##T##N(T *o, const T *a, const T *b) { \
    *(T##N*)o = *(const T##N*)a / *(const T##N*)b; \
} \
void ocl_div_##T##N##_##T(T *o, const T *a, T b) { \
    *(T##N*)o = *(const T##N*)a / b; \
} \
void ocl_div_##T##_##T##N(T *o, T a, const T *b) { \
    *(T##N*)o = a / *(const T##N*)b; \
} \
void ocl_addassign_##T##N##_##T##N(T *o, const T *a) { \
    *(T##N*)o += *(const T##N*)a; \
} \
void ocl_addassign_##T##N##_##T(T *o, T a) { \
    *(T##N*)o += a; \
} \
void ocl_subassign_##T##N##_##T##N(T *o, const T *a) { \
    *(T##N*)o -= *(const T##N*)a; \
} \
void ocl_subassign_##T##N##_##T(T *o, T a) { \
    *(T##N*)o -= a; \
} \
void ocl_mulassign_##T##N##_##T##N(T *o, const T *a) { \
    *(T##N*)o *= *(const T##N*)a; \
} \
void ocl_mulassign_##T##N##_##T(T *o, T a) { \
    *(T##N*)o *= a; \
} \
void ocl_divassign_##T##N##_##T##N(T *o, const T *a) { \
    *(T##N*)o /= *(const T##N*)a; \
} \
void ocl_divassign_##T##N##_##T(T *o, T a) { \
    *(T##N*)o /= a; \
} \

#define DEFINE_BUILTIN_VECTOR_CTOR_2(T) \
void ocl_ctor_##T##2##_##T##_2(T *v, T x, T y) { \
    *(T##2*)v = (T##2)(x, y); \
} \

#define DEFINE_BUILTIN_VECTOR_CTOR_3(T) \
void ocl_ctor_##T##3##_##T##_3(T *v, T x, T y, T z) { \
    *(T##3*)v = (T##3)(x, y, z); \
} \

#define DEFINE_BUILTIN_VECTOR_CTOR_4(T) \
void ocl_ctor_##T##4##_##T##_4(T *v, T x, T y, T z, T w) { \
    *(T##4*)v = (T##4)(x, y, z, w); \
} \

#define DEFINE_BUILTIN_VECTOR_CTOR_8(T) \
void ocl_ctor_##T##8##_##T##_8( \
    T *v, \
    T s0, T s1, T s2, T s3, T s4, T s5, T s6, T s7 \
) { \
    *(T##8*)v = (T##8)( \
        s0, s1, s2, s3, s4, s5, s6, s7 \
    ); \
} \

#define DEFINE_BUILTIN_VECTOR_CTOR_16(T) \
void ocl_ctor_##T##16##_##T##_16( \
    T *v, \
    T s0, T s1, T s2, T s3, T s4, T s5, T s6, T s7, \
    T s8, T s9, T sA, T sB, T sC, T sD, T sE, T sF \
) { \
    *(T##16*)v = (T##16)( \
        s0, s1, s2, s3, s4, s5, s6, s7, \
        s8, s9, sA, sB, sC, sD, sE, sF \
    ); \
} \

#define DEFINE_BUILTIN_VECTOR_GEOMETRY(T, N) \
T ocl_dot_##T##N(const T *a, const T *b) { \
    return dot(*(const T##N*)a, *(const T##N*)b); \
} \
T ocl_length_##T##N(const T *a) { \
    return length(*(const T##N*)a); \
} \
void ocl_normalize_##T##N(T *o, const T *a) { \
    *(T##N*)o = normalize(*(const T##N*)a); \
} \

#define DEFINE_BUILTIN_VECTOR_GEOMETRY_CROSS(T, N) \
void ocl_cross_##T##N(T *o, const T *a, const T *b) { \
    *(T##N*)o = cross(*(const T##N*)a, *(const T##N*)b); \
} \

#define DEFGROUP_BUILTIN_VECTOR(T) \
DEFINE_BUILTIN_VECTOR(T, 2) \
DEFINE_BUILTIN_VECTOR(T, 3) \
DEFINE_BUILTIN_VECTOR(T, 4) \
DEFINE_BUILTIN_VECTOR(T, 8) \
DEFINE_BUILTIN_VECTOR(T, 16) \
DEFINE_BUILTIN_VECTOR_CTOR_2(T) \
DEFINE_BUILTIN_VECTOR_CTOR_3(T) \
DEFINE_BUILTIN_VECTOR_CTOR_4(T) \
DEFINE_BUILTIN_VECTOR_CTOR_8(T) \
DEFINE_BUILTIN_VECTOR_CTOR_16(T) \

#define DEFGROUP_BUILTIN_VECTOR_FLOAT(T) \
DEFGROUP_BUILTIN_VECTOR(T) \
DEFINE_BUILTIN_VECTOR_GEOMETRY(T, 2) \
DEFINE_BUILTIN_VECTOR_GEOMETRY(T, 3) \
DEFINE_BUILTIN_VECTOR_GEOMETRY(T, 4) \
DEFINE_BUILTIN_VECTOR_GEOMETRY_CROSS(T, 3) \
DEFINE_BUILTIN_VECTOR_GEOMETRY_CROSS(T, 4) \

DEFGROUP_BUILTIN_VECTOR(uchar)
DEFGROUP_BUILTIN_VECTOR(char)
DEFGROUP_BUILTIN_VECTOR(ushort)
DEFGROUP_BUILTIN_VECTOR(short)
DEFGROUP_BUILTIN_VECTOR(uint)
DEFGROUP_BUILTIN_VECTOR(int)
DEFGROUP_BUILTIN_VECTOR(ulong)
DEFGROUP_BUILTIN_VECTOR(long)

DEFGROUP_BUILTIN_VECTOR_FLOAT(float)
#ifdef DEVICE_DOUBLE
DEFGROUP_BUILTIN_VECTOR_FLOAT(double)
#endif // DEVICE_DOUBLE
