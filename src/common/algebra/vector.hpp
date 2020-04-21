#pragma once


#include "vector_base.hpp" 

#ifdef HOST
#include <iostream>
#endif


#define DEFINE_VECTOR_MATH_FUNCTION_A(F) \
template <typename T, int N> \
vector<T, N> F(vector<T, N> a) { \
    return vector<T, N>::map([](T x) { return F(x); }, a); \
} \

#define DEFINE_VECTOR_MATH_FUNCTION_AB(F) \
template <typename T, int N> \
vector<T, N> F(vector<T, N> a, vector<T, N> b) { \
    return vector<T, N>::map([](T x, T y) { return F(x, y); }, a, b); \
} \

namespace math {

DEFINE_VECTOR_MATH_FUNCTION_A(abs)
DEFINE_VECTOR_MATH_FUNCTION_AB(max)
DEFINE_VECTOR_MATH_FUNCTION_AB(min)
DEFINE_VECTOR_MATH_FUNCTION_A(acos)
DEFINE_VECTOR_MATH_FUNCTION_A(acosh)
DEFINE_VECTOR_MATH_FUNCTION_A(asin)
DEFINE_VECTOR_MATH_FUNCTION_A(asinh)
DEFINE_VECTOR_MATH_FUNCTION_A(atan)
DEFINE_VECTOR_MATH_FUNCTION_AB(atan2)
DEFINE_VECTOR_MATH_FUNCTION_A(atanh)
DEFINE_VECTOR_MATH_FUNCTION_A(ceil)
DEFINE_VECTOR_MATH_FUNCTION_A(cos)
DEFINE_VECTOR_MATH_FUNCTION_A(cosh)
DEFINE_VECTOR_MATH_FUNCTION_A(erfc)
DEFINE_VECTOR_MATH_FUNCTION_A(erf)
DEFINE_VECTOR_MATH_FUNCTION_A(exp)
DEFINE_VECTOR_MATH_FUNCTION_A(floor)
DEFINE_VECTOR_MATH_FUNCTION_AB(mod)
DEFINE_VECTOR_MATH_FUNCTION_A(log)
DEFINE_VECTOR_MATH_FUNCTION_AB(pow)
DEFINE_VECTOR_MATH_FUNCTION_A(round)
DEFINE_VECTOR_MATH_FUNCTION_A(sqrt)
DEFINE_VECTOR_MATH_FUNCTION_A(sin)
DEFINE_VECTOR_MATH_FUNCTION_A(sinh)
DEFINE_VECTOR_MATH_FUNCTION_A(tan)
DEFINE_VECTOR_MATH_FUNCTION_A(tanh)
DEFINE_VECTOR_MATH_FUNCTION_A(tgamma)
template <typename T, int N>
vector<T, N> clamp(vector<T, N> a, vector<T, N> b, vector<T, N> c) {
    return vector<T, N>::map(
        [](T x, T y, T z) { return clamp(x, y, z); },
        a, b, c
    );
}
template <typename T, int N>
pair<vector<T, N>, vector<T, N>> fract(vector<T, N> a) {
    pair<vector<T, N>, vector<T, N>> p;
    for (int i = 0; i < N; ++i) {
        pair<T, T> x = fract(a[i]);
        p.first[i] = x.first;
        p.second[i] = x.second;
    }
    return p;
}

} // namespace math

#ifdef HOST

namespace device {
template <typename T, int N, typename U=void>
struct vector;
template <typename T, int N>
struct __attribute__((aligned(sizeof(T)*(N + (N == 3)))))
vector<T, N, enable_if<is_prim<T>()>> {
    T s[N];
};
template <typename T, int N>
struct vector<T, N, enable_if<!is_prim<T>()>> {
    T s[N];
};
}

template <typename T, int N>
struct ToDevice<vector<T, N>> {
    typedef device::vector<device_type<T>, N> type;
    static type to_device(vector<T, N> v) {
        type o;
        for (int i = 0; i < N; ++i) {
            o.s[i] = ::to_device(v[i]);
        }
        return o;
    }
};

#endif

#ifdef DEVICE
#include "vector_builtin.hpp"
#endif

#define DEFINE_VECTOR_ALIAS(T) \
typedef vector<T, 2> T##2; \
typedef vector<T, 3> T##3; \
typedef vector<T, 4> T##4; \
typedef vector<T, 8> T##8; \
typedef vector<T, 16> T##16; \

DEFINE_VECTOR_ALIAS(uchar)
DEFINE_VECTOR_ALIAS(ushort)
DEFINE_VECTOR_ALIAS(uint)
DEFINE_VECTOR_ALIAS(ulong)

DEFINE_VECTOR_ALIAS(char)
DEFINE_VECTOR_ALIAS(short)
DEFINE_VECTOR_ALIAS(int)
DEFINE_VECTOR_ALIAS(long)

DEFINE_VECTOR_ALIAS(float)
#if defined(HOST) || defined(DEVICE_DOUBLE)
DEFINE_VECTOR_ALIAS(double)
#endif
DEFINE_VECTOR_ALIAS(real)


#ifdef HOST
template <typename T, int N>
std::ostream &operator<<(std::ostream &s, vector<T, N> v) {
    s << "(";
    for (int i = 0; i < N - 1; ++i) {
        s << v[i] << ", ";
    }
    s << v[N - 1] << ")";
    return s;
}
#endif


#ifdef UNIT_TEST
#include <catch.hpp>
#include "test.hpp"

namespace test {

template <typename T, int N>
class Distrib<vector<T, N>> : public Rng {
public:
    vector<T, N> normal() {
        return vector<T, N>::map([this]() { return d<T>().normal(); });
    }
    vector<T, N> uniform() {
        return vector<T, N>::map([this]() { return d<T>().uniform(); });
    }
    vector<T, N> nonzero() {
        vector<T, N> a;
        do {
            a = normal();
        } while(length2(a) < EPS);
        return a;
    }
    vector<T, N> unit() {
        return normalize(nonzero());
    }
};

template <typename T, int N>
class VecApprox {
private:
    vector<T, N> v;

public:
    VecApprox(vector<T, N> c) : v(c) {}

    bool operator==(vector<T, N> a) {
        for (int i = 0; i < N; ++i) {
            if (a[i] != approx(v[i])) {
                return false;
            }
        }
        return true;
    }
    TEST_DEFINE_CMP_OPS(, friend, VecApprox, vector<T COMMA N>)

    friend std::ostream &operator<<(std::ostream &s, VecApprox a) {
        return s << a.v;
    }
};


template <typename T, int N>
VecApprox<T, N> approx(vector<T, N> v) {
    return VecApprox<T, N>(v);
}

} // namespace test

#endif
