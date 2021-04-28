#pragma once

#include "real.hh"

#ifdef HOST

#include "vector_struct.hpp"
#include "vector_math.hpp"

// Alias

#define VECTOR_ALIAS(T, A, N) \
typedef vec<T, N> A##N;

// Make

#define VECTOR_MAKE(T, A, N) \
template <typename... Args> \
inline vec<T, N> make_##A##N(Args ...args) { \
    return vec<T, N>(args...); \
}

// Cast

#define VECTOR_CAST(T, A, N) \
template <typename S> \
inline vec<T, N> convert_##A##N(vec<S, N> v) { \
    return v.template cast<T>(); \
}

// All

#define VECTOR_ALL(T, A, N) \
VECTOR_ALIAS(T, A, N) \
VECTOR_MAKE(T, A, N) \
VECTOR_CAST(T, A, N)

#define VECTOR_ALL_N(T, A) \
VECTOR_ALL(T, A, 2) \
VECTOR_ALL(T, A, 3) \
VECTOR_ALL(T, A, 4) \
VECTOR_ALL(T, A, 8) \
VECTOR_ALL(T, A, 16)

VECTOR_ALL_N(uchar,  uchar)
VECTOR_ALL_N(ushort, ushort)
VECTOR_ALL_N(uint,   uint)
VECTOR_ALL_N(ulong,  ulong)
VECTOR_ALL_N(char,   char)
VECTOR_ALL_N(short,  short)
VECTOR_ALL_N(int,    int)
VECTOR_ALL_N(long_,  long)

VECTOR_ALL_N(float,  float)
#ifdef DEV_F64
VECTOR_ALL_N(double, double)
#endif // DEV_F64
VECTOR_ALL_N(real,   real)

#define MAKE(x) x

#else // !HOST

#define MAKE(x) (x)

#ifdef DOUBLE_PRECISION
typedef double2  real2;
typedef double3  real3;
typedef double4  real4;
typedef double8  real8;
typedef double16 real16;
#else
typedef float2  real2;
typedef float3  real3;
typedef float4  real4;
typedef float8  real8;
typedef float16 real16;
#endif

#endif // HOST


#define r2_new MAKE(real2)
#define r3_new MAKE(real3)
#define r4_new MAKE(real4)

#ifdef DOUBLE_PRECISION
#define convert_real2  convert_double2
#define convert_real3  convert_double3
#define convert_real4  convert_double4
#define convert_real8  convert_double8
#define convert_real16 convert_double16
#else
#define convert_real2  convert_float2
#define convert_real3  convert_float3
#define convert_real4  convert_float4
#define convert_real8  convert_float8
#define convert_real16 convert_float16
#endif

// Shortcuts

#define length2(x) dot((x), (x))


#ifdef UNITTEST

template <typename T, int N>
class TestRng<vec<T, N>> {
private:
    TestRng<T> rng;

public:
    inline TestRng() = default;
    inline explicit TestRng(uint32_t seed) : rng(seed) {}

    vec<T, N> normal() {
        return vec<T, N>::map([this]() { return rng.normal(); });
    }
    vec<T, N> uniform() {
        return vec<T, N>::map([this]() { return rng.uniform(); });
    }
    vec<T, N> nonzero() {
        vec<T, N> a;
        do {
            a = normal();
        } while(length2(a) < EPS);
        return a;
    }
    vec<T, N> unit() {
        return normalize(nonzero());
    }
};


template <typename T, int N>
class Distance<vec<T, N>> {
public:
    static real distance(vec<T, N> a, vec<T, N> b) {
        return ::distance(a, b);
    }
};

#endif // UNITTEST


#ifndef HOST
#include "vector.cc"
#endif // !HOST
