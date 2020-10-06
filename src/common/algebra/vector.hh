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

#define VECTOR_INTEROP(T, S, N) \
template <> \
struct Interop<T##N> { \
    typedef T##N Host; \
    typedef S##N Dev; \
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

#define VECTOR_INTEROP_N(T, S) \
VECTOR_INTEROP(T, S, 2) \
VECTOR_INTEROP(T, S, 3) \
VECTOR_INTEROP(T, S, 4) \
VECTOR_INTEROP(T, S, 8) \
VECTOR_INTEROP(T, S, 16)

VECTOR_INTEROP_N(uchar,  cl_uchar)
VECTOR_INTEROP_N(ushort, cl_ushort)
VECTOR_INTEROP_N(uint,   cl_uint)
VECTOR_INTEROP_N(ulong,  cl_ulong)
VECTOR_INTEROP_N(char,   cl_char)
VECTOR_INTEROP_N(short,  cl_short)
VECTOR_INTEROP_N(int,    cl_int)
VECTOR_INTEROP_N(long,   cl_long)

VECTOR_INTEROP_N(float,  cl_float)
#ifdef DEV_F64
VECTOR_INTEROP_N(double, cl_double)
VECTOR_INTEROP_N(real,   cl_double)
#else // DEV_F64
VECTOR_INTEROP_N(real,   cl_float)
#endif // DEV_F64

#endif // INTEROP

#else // HOST

#define MAKE(x) (x)

#endif // HOST


#define r2_new MAKE(real2)
#define r3_new MAKE(real3)
#define r4_new MAKE(real4)

// Shortcuts

#define length2(x) dot((x), (x))


#ifdef TEST_CATCH

#include <catch.hpp>

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
class VecApprox {
private:
    typedef vec<T, N> vtype;
    vtype v;
    T eps = -1;

public:
    VecApprox(vtype c) : v(c) {}
    VecApprox &epsilon(T e) {
        eps = e;
        return *this;
    }

    friend bool operator==(vtype a, VecApprox b) {
        for (int i = 0; i < N; ++i) {
            auto ap = Approx(b.v[i]);
            if (b.eps > 0) {
                ap.epsilon(b.eps);
            } 
            if (a[i] != ap) {
                return false;
            }
        }
        return true;
    }
    friend bool operator==(VecApprox a, vtype b){
        return b == a;
    }
    friend bool operator!=(vtype a, VecApprox b){
        return !(a == b);
    }
    friend bool operator!=(VecApprox a, vtype b){
        return a != b;
    }
    friend std::ostream &operator<<(std::ostream &s, VecApprox a) {
        return s << a.v;
    }
};

template <typename T, int N>
VecApprox<T, N> approx(vec<T, N> v) {
    return VecApprox<T, N>(v);
}

#endif // TEST_CATCH

#ifdef TEST_DEV

#include <host/opencl/opencl.hpp>

void test_dev_vector(
    cl_device_id device,
    cl_context context,
    cl_command_queue queue
);

#endif // TEST_DEV