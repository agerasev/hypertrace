#pragma once

#include <types.h>


#ifdef OPENCL

#define DEFINE_REAL_VECTOR_TYPES(T) \
typedef T##2 real##2; \
typedef T##3 real##3; \
typedef T##4 real##4; \
typedef T##8 real##8; \
typedef T##16 real##16; \

#else
#define DEFINE_REAL_VECTOR_TYPES(T)
#endif

#if defined(DEVICE) && !defined(DEVICE_DOUBLE)

typedef float real;
DEFINE_REAL_VECTOR_TYPES(float)
#define PI 3.14159265358979323846f
#define EPS 1e-6f

#else

typedef double real;
DEFINE_REAL_VECTOR_TYPES(double)
#define PI 3.14159265358979323846
#define EPS 1e-8

#endif


#ifdef __cplusplus

inline real operator ""_r(unsigned long long x) {
    return real(x);
}
inline real operator ""_r(long double x) {
    return real(x);
}

#endif // __cplusplus
