#pragma once

#include "real.hh"

#ifdef HOST

#include "vector_struct.hpp"
#include "vector_math.hpp"

// Alias

#define VECTOR_ALIAS(T, N) \
typedef vec<T, N> T##N;

// Make

#define VECTOR_MAKE(T, N) \
template <typename... Args> \
inline vec<T, N> make_##T##N(Args ...args) { \
    return vec<T, N>(args...); \
}

// Cast

#define VECTOR_CAST(T, N) \
template <typename S> \
inline vec<T, N> convert_##T##N(vec<S, N> v) { \
    return v.template cast<T>(); \
}

// All

#define VECTOR_ALL(T, N) \
VECTOR_ALIAS(T, N) \
VECTOR_MAKE(T, N) \
VECTOR_CAST(T, N)

#define VECTOR_ALL_N(T) \
VECTOR_ALL(T, 2) \
VECTOR_ALL(T, 3) \
VECTOR_ALL(T, 4) \
VECTOR_ALL(T, 8) \
VECTOR_ALL(T, 16)

VECTOR_ALL_N(uchar)
VECTOR_ALL_N(ushort)
VECTOR_ALL_N(uint)
VECTOR_ALL_N(ulong)
VECTOR_ALL_N(char)
VECTOR_ALL_N(short)
VECTOR_ALL_N(int)
VECTOR_ALL_N(long)

VECTOR_ALL_N(float)
#ifdef DEV_F64
VECTOR_ALL_N(double)
#endif // DEV_F64
VECTOR_ALL_N(real)

#define MAKE(x) x

#ifdef INTEROP

#include <CL/cl.h>

#define VECTOR_INTEROP(T, S, A, N) \
template <> \
struct Interop<T##N> { \
    typedef T##N Host; \
    typedef S##N Dev; \
    inline static const char *const name = A #N; \
    static void load(Host *dst, const Dev *src) { \
        for (size_t i = 0; i < N; ++i) { \
            (*dst)[i] = (T)src->s[i]; \
        } \
    } \
    static void store(Dev *dst, const Host *src) { \
        for (size_t i = 0; i < N; ++i) { \
            dst->s[i] = (S)(*src)[i]; \
        } \
    } \
};

#define VECTOR_INTEROP_N(T, S, A) \
VECTOR_INTEROP(T, S, A, 2) \
VECTOR_INTEROP(T, S, A, 3) \
VECTOR_INTEROP(T, S, A, 4) \
VECTOR_INTEROP(T, S, A, 8) \
VECTOR_INTEROP(T, S, A, 16)

VECTOR_INTEROP_N(uchar,  cl_uchar,  "uchar" )
VECTOR_INTEROP_N(ushort, cl_ushort, "ushort")
VECTOR_INTEROP_N(uint,   cl_uint,   "uint"  )
VECTOR_INTEROP_N(ulong,  cl_ulong,  "ulong" )
VECTOR_INTEROP_N(char,   cl_char,   "char"  )
VECTOR_INTEROP_N(short,  cl_short,  "short" )
VECTOR_INTEROP_N(int,    cl_int,    "int"   )
VECTOR_INTEROP_N(long,   cl_long,   "long"  )

VECTOR_INTEROP_N(float,  cl_float,  "float" )
#ifdef DEV_F64
VECTOR_INTEROP_N(double, cl_double, "double")
VECTOR_INTEROP_N(real,   cl_double, "double")
#else // DEV_F64
VECTOR_INTEROP_N(real,   cl_float,  "float" )
#endif // DEV_F64

#endif // INTEROP

#else // !HOST

#define MAKE(x) (x)

#ifdef DEV_F64
typedef double2  real2;
typedef double3  real3;
typedef double4  real4;
typedef double8  real8;
typedef double16 real16;
#else // !DEV_F64
typedef float2  real2;
typedef float3  real3;
typedef float4  real4;
typedef float8  real8;
typedef float16 real16;
#endif // DEV_F64

#endif // HOST


#define r2_new MAKE(real2)
#define r3_new MAKE(real3)
#define r4_new MAKE(real4)

// Shortcuts

#define length2(x) dot((x), (x))


#ifdef TEST

template <int N>
class TestRng<vec<real, N>> {
private:
    TestRng<real> rng;

public:
    inline TestRng() = default;
    inline explicit TestRng(uint32_t seed) : rng(seed) {}

    vec<real, N> normal() {
        return vec<real, N>::map([this]() { return rng.normal(); });
    }
    vec<real, N> uniform() {
        return vec<real, N>::map([this]() { return rng.uniform(); });
    }
    vec<real, N> nonzero() {
        vec<real, N> a;
        do {
            a = normal();
        } while(length2(a) < EPS);
        return a;
    }
    vec<real, N> unit() {
        return normalize(nonzero());
    }
};


template <typename T, int N>
class Approx<vec<T, N>> {
public:
    typedef vec<T, N> V;
    real _epsilon = APPROX_EPS;
    V _value = V(R0);

    explicit Approx(V value) :
        _value(value)    
    {}
    Approx epsilon(real eps) const {
        Approx copy_ = *this;
        copy_._epsilon = eps;
        return copy_;
    }
    bool operator==(V x) const {
        for (int i = 0; i < N; ++i) {
            if (std::abs(_value[i] - x[i]) > _epsilon) {
                return false;
            }
        }
        return true;
    }
};

#endif // TEST


#ifndef HOST
#include "vector.cc"
#endif // !HOST
