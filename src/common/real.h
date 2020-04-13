#pragma once

#include <types.h>


#ifdef OPENCL

#define DEFINE_REAL_VECTOR_TYPES(T) \
typedef T##2 real##2; \
typedef T##3 real##3; \
typedef T##4 real##4; \
typedef T##8 real##8; \
typedef T##16 real##16; \

#define __CONSTANT__ __constant

#else
#define DEFINE_REAL_VECTOR_TYPES(T)
#define __CONSTANT__
#endif

#define DEFINE_REAL_FLOAT() \
typedef float real; \
DEFINE_REAL_VECTOR_TYPES(float) \
static const __CONSTANT__ float PI = 3.14159265358979323846f; \
static const __CONSTANT__ float EPS = 1e-6f; \

#define DEFINE_REAL_DOUBLE() \
typedef double real; \
DEFINE_REAL_VECTOR_TYPES(double) \
static const __CONSTANT__ double PI = 3.14159265358979323846; \
static const __CONSTANT__ double EPS = 1e-8; \


#ifdef __cplusplus

namespace host {
DEFINE_REAL_DOUBLE()
}

namespace device {
#endif // __cplusplus

#ifdef DEVICE_DOUBLE
DEFINE_REAL_DOUBLE()
#else
DEFINE_REAL_FLOAT()
#endif

#ifdef __cplusplus
}

#ifdef HOST
using namespace host;
#endif
#ifdef DEVICE
using namespace device;
#endif


inline real operator ""_r(unsigned long long x) {
    return real(x);
}
inline real operator ""_r(long double x) {
    return real(x);
}

#endif // __cplusplus
