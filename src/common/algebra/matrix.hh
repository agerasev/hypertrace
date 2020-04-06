#pragma once


// FIXME: Implement using C++ templates when migrated to OpenCL 2.1

// Name generators

#define MAT_(T,N,M) T##N##x##M
#define MATF_(T,N,M,F) T##N##x##M##_##F
#define TF_(T,F) T##_##F


// Type declaration

#define MAT_TYPE_H(T,M,N) \
typedef struct MAT_(T,M,N) MAT_(T,M,N); \


// Mapping functionality

#define MAT_MAP_H(T,M,N,F) \
MAT_(T,M,N) MATF_(T,M,N,F)(MAT_(T,M,N) m) \

#define MAT_MAP2_H(T,M,N,F) \
MAT_(T,M,N) MATF_(T,M,N,F)(MAT_(T,M,N) a, MAT_(T,M,N) b) \

#define MAT_MAPC_H(T,M,N,F) \
MAT_(T,M,N) MATF_(T,M,N,F)(MAT_(T,M,N) a, T b) \


// Common functionality

#define MAT_COMMON_H(T,M,N) \
\
struct MAT_(T,M,N) { \
    T s[M*N]; \
}; \
\
MAT_(T,M,N) MATF_(T,M,N,zero)(); \
MAT_(T,M,N) MATF_(T,M,N,from_data)( \
    const T *data, \
    const int pitch, const int stride \
); \
\
MAT_(T,N,M) MATF_(T,M,N,transpose)(MAT_(T,M,N) m); \
\
MAT_MAP_H(T,M,N,neg); \
MAT_MAP2_H(T,M,N,add); \
MAT_MAP2_H(T,M,N,sub); \
MAT_MAPC_H(T,M,N,mul); \
MAT_MAPC_H(T,M,N,div); \

// Square matrix functionality

#define MAT_SQUARE_H(T,N) \
MAT_(T,N,N) MATF_(T,N,N,one)(); \

#define MAT_SQUARE2_H(T) \
MAT_SQUARE_H(T,2) \
\
T MATF_(T,2,2,det)(MAT_(T,2,2) m); \
MAT_(T,2,2) MATF_(T,2,2,inverse)(MAT_(T,2,2) m); \

#define MAT_SQUARE3_H(T) \
MAT_SQUARE_H(T,3) \

#define MAT_SQUARE4_H(T) \
MAT_SQUARE_H(T,4) \


// Dot product

#define MAT_DOT_H(T,L,M,N) \
MAT_(T,L,N) T##L##x##M##x##N##_dot(MAT_(T,L,M) a, MAT_(T,M,N) b)


// All in one up to specified dimension

#define MAT2_H(T) \
\
MAT_TYPE_H(T,1,1) \
MAT_TYPE_H(T,1,2) \
MAT_TYPE_H(T,2,1) \
MAT_TYPE_H(T,2,2) \
\
MAT_COMMON_H(T,1,1) \
MAT_COMMON_H(T,1,2) \
MAT_COMMON_H(T,2,1) \
MAT_COMMON_H(T,2,2) \
\
MAT_DOT_H(T,1,1,1); \
MAT_DOT_H(T,1,1,2); \
MAT_DOT_H(T,1,2,1); \
MAT_DOT_H(T,1,2,2); \
MAT_DOT_H(T,2,1,1); \
MAT_DOT_H(T,2,1,2); \
MAT_DOT_H(T,2,2,1); \
MAT_DOT_H(T,2,2,2); \
\
MAT_SQUARE2_H(T); \

#define MAT3_H(T) \
\
MAT2_H(T) \
\
MAT_TYPE_H(T,1,3) \
MAT_TYPE_H(T,2,3) \
MAT_TYPE_H(T,3,1) \
MAT_TYPE_H(T,3,2) \
MAT_TYPE_H(T,3,3) \
\
MAT_COMMON_H(T,1,3) \
MAT_COMMON_H(T,2,3) \
MAT_COMMON_H(T,3,1) \
MAT_COMMON_H(T,3,2) \
MAT_COMMON_H(T,3,3) \
\
MAT_DOT_H(T,1,1,3); \
MAT_DOT_H(T,1,2,3); \
MAT_DOT_H(T,1,3,1); \
MAT_DOT_H(T,1,3,2); \
MAT_DOT_H(T,1,3,3); \
MAT_DOT_H(T,2,1,3); \
MAT_DOT_H(T,2,2,3); \
MAT_DOT_H(T,2,3,1); \
MAT_DOT_H(T,2,3,2); \
MAT_DOT_H(T,2,3,3); \
MAT_DOT_H(T,3,1,1); \
MAT_DOT_H(T,3,1,2); \
MAT_DOT_H(T,3,1,3); \
MAT_DOT_H(T,3,2,1); \
MAT_DOT_H(T,3,2,2); \
MAT_DOT_H(T,3,2,3); \
MAT_DOT_H(T,3,3,1); \
MAT_DOT_H(T,3,3,2); \
MAT_DOT_H(T,3,3,3); \
\
MAT_SQUARE3_H(T); \

#define MAT4_H(T) \
\
MAT3_H(T) \
\
MAT_TYPE_H(T,1,4) \
MAT_TYPE_H(T,2,4) \
MAT_TYPE_H(T,3,4) \
MAT_TYPE_H(T,4,1) \
MAT_TYPE_H(T,4,2) \
MAT_TYPE_H(T,4,3) \
MAT_TYPE_H(T,4,4) \
\
MAT_COMMON_H(T,1,4) \
MAT_COMMON_H(T,2,4) \
MAT_COMMON_H(T,3,4) \
MAT_COMMON_H(T,4,1) \
MAT_COMMON_H(T,4,2) \
MAT_COMMON_H(T,4,3) \
MAT_COMMON_H(T,4,4) \
\
MAT_DOT_H(T,1,1,4); \
MAT_DOT_H(T,1,2,4); \
MAT_DOT_H(T,1,3,4); \
MAT_DOT_H(T,1,4,1); \
MAT_DOT_H(T,1,4,2); \
MAT_DOT_H(T,1,4,3); \
MAT_DOT_H(T,1,4,4); \
MAT_DOT_H(T,2,1,4); \
MAT_DOT_H(T,2,2,4); \
MAT_DOT_H(T,2,3,4); \
MAT_DOT_H(T,2,4,1); \
MAT_DOT_H(T,2,4,2); \
MAT_DOT_H(T,2,4,3); \
MAT_DOT_H(T,2,4,4); \
MAT_DOT_H(T,3,1,4); \
MAT_DOT_H(T,3,2,4); \
MAT_DOT_H(T,3,3,4); \
MAT_DOT_H(T,3,4,1); \
MAT_DOT_H(T,3,4,2); \
MAT_DOT_H(T,3,4,3); \
MAT_DOT_H(T,3,4,4); \
MAT_DOT_H(T,4,1,1); \
MAT_DOT_H(T,4,1,2); \
MAT_DOT_H(T,4,1,3); \
MAT_DOT_H(T,4,1,4); \
MAT_DOT_H(T,4,2,1); \
MAT_DOT_H(T,4,2,2); \
MAT_DOT_H(T,4,2,3); \
MAT_DOT_H(T,4,2,4); \
MAT_DOT_H(T,4,3,1); \
MAT_DOT_H(T,4,3,2); \
MAT_DOT_H(T,4,3,3); \
MAT_DOT_H(T,4,3,4); \
MAT_DOT_H(T,4,4,1); \
MAT_DOT_H(T,4,4,2); \
MAT_DOT_H(T,4,4,3); \
MAT_DOT_H(T,4,4,4); \
\
MAT_SQUARE4_H(T); \

// Packing

#ifdef OPENCL_INTEROP

#define MAT_PK_(T,N,M) T##N##x##M##_pk
#define MATF_PK_(F,T,N,M) F##_##T##N##x##M
#define TF_PK_(F,T) F##_##T

#define MAT_PK_H(T,M,N) \
typedef struct __attribute__((packed, aligned(8))) MAT_PK_(T,M,N) { \
    T##_pk s[M*N] __attribute__((aligned(8))); \
} MAT_PK_(T,M,N); \
MAT_PK_(T,M,N) MATF_PK_(pack,T,M,N)(MAT_(T,M,N) m); \
MAT_(T,M,N) MATF_PK_(unpack,T,M,N)(MAT_PK_(T,M,N) m); \

#define MAT2_PK_H(T) \
MAT_PK_H(T,1,1) \
MAT_PK_H(T,1,2) \
MAT_PK_H(T,2,1) \
MAT_PK_H(T,2,2) \

#define MAT3_PK_H(T) \
MAT2_PK_H(T) \
MAT_PK_H(T,1,3) \
MAT_PK_H(T,2,3) \
MAT_PK_H(T,3,1) \
MAT_PK_H(T,3,2) \
MAT_PK_H(T,3,3) \

#define MAT4_PK_H(T) \
MAT3_PK_H(T) \
MAT_PK_H(T,1,4) \
MAT_PK_H(T,2,4) \
MAT_PK_H(T,3,4) \
MAT_PK_H(T,4,1) \
MAT_PK_H(T,4,2) \
MAT_PK_H(T,4,3) \
MAT_PK_H(T,4,4) \

#endif // OPENCL_INTEROP

// Instantiation

#include "real.hh"
#include "complex.hh"

MAT4_H(real)
MAT2_H(complex)

#ifdef OPENCL_INTEROP
MAT4_PK_H(real)
MAT2_PK_H(complex)
#endif // OPENCL_INTEROP

complex2x2 complex2x2_new(complex a, complex b, complex c, complex d);
real complex2x2_fabs(complex2x2 m);

complex2x2 complex2x2_normalize(complex2x2 m);
complex2x2 complex2x2_inverse_normalized(complex2x2 m);

void complex2x2_eigen(complex2x2 m, complex2x2 *l, complex2x2 *v);
void complex2x2_eigen_normalized(complex2x2 m, complex2x2 *l, complex2x2 *v);

complex2x2 complex2x2_pow(complex2x2 m, real p);
complex2x2 complex2x2_pow_normalized(complex2x2 m, real p);

#ifdef UNIT_TEST
MAT4_H(int)
#endif // UNIT_TEST

#ifndef OPENCL
#include <mat.hpp>
#endif // OPENCL