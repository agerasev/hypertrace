#pragma once

#include <common/math.hh>

#include "vector_struct.hpp"


template <typename T, int N>
T dot(vec<T, N> a, vec<T, N> b) {
    T c = (T)0;
    for (int i = 0; i < N; ++i) {
        c += a[i]*b[i];
    }
    return c;
}
template <typename T>
vec<T, 3> cross(vec<T, 3> a, vec<T, 3> b) {
    return vec<T, 3>(
        a.y*b.z - a.z*b.y,
        a.z*b.x - a.x*b.z,
        a.x*b.y - a.y*b.x
    );
}
template <typename T>
vec<T, 4> cross(vec<T, 4> a, vec<T, 4> b) {
    return vec<T, 4>(cross(a.xyz, b.xyz), (T)0);
}

template <typename T, int N>
T length(vec<T, N> a) {
    return sqrt(dot(a, a));
}

template <typename T, int N>
vec<T, N> normalize(vec<T, N> a) {
    return a/length(a);
}


#define VECTOR_MATH_FA(F) \
template <typename T, int N> \
vec<T, N> F(vec<T, N> a) { \
    return vec<T, N>::map([](T x) { return F(x); }, a); \
} \

#define VECTOR_MATH_FAB(F) \
template <typename T, int N> \
vec<T, N> F(vec<T, N> a, vec<T, N> b) { \
    return vec<T, N>::map([](T x, T y) { return F(x, y); }, a, b); \
} \

VECTOR_MATH_FA(fabs)
VECTOR_MATH_FAB(fmax)
VECTOR_MATH_FAB(fmin)
VECTOR_MATH_FA(acos)
VECTOR_MATH_FA(acosh)
VECTOR_MATH_FA(asin)
VECTOR_MATH_FA(asinh)
VECTOR_MATH_FA(atan)
VECTOR_MATH_FAB(atan2)
VECTOR_MATH_FA(atanh)
VECTOR_MATH_FA(ceil)
VECTOR_MATH_FA(cos)
VECTOR_MATH_FA(cosh)
VECTOR_MATH_FA(erfc)
VECTOR_MATH_FA(erf)
VECTOR_MATH_FA(exp)
VECTOR_MATH_FA(floor)
VECTOR_MATH_FAB(fmod)
VECTOR_MATH_FA(log)
VECTOR_MATH_FAB(pow)
VECTOR_MATH_FA(round)
VECTOR_MATH_FA(sqrt)
VECTOR_MATH_FA(sin)
VECTOR_MATH_FA(sinh)
VECTOR_MATH_FA(tan)
VECTOR_MATH_FA(tanh)
VECTOR_MATH_FA(tgamma)
VECTOR_MATH_FA(sign)

template <typename T, int N>
vec<T, N> fmin(vec<T, N> a, T b) {
    return vec<T, N>::map([b](T x) { return fmin(x, b); }, a);
}
template <typename T, int N>
vec<T, N> fmax(vec<T, N> a, T b) {
    return vec<T, N>::map([b](T x) { return fmax(x, b); }, a);
}
template <typename T, int N>
vec<T, N> pow(vec<T, N> a, T b) {
    return vec<T, N>::map([b](T x) { return pow(x, b); }, a);
}

template <typename T, int N>
vec<T, N> clamp(vec<T, N> a, vec<T, N> b, vec<T, N> c) {
    return vec<T, N>::map(
        [](T x, T y, T z) { return clamp(x, y, z); },
        a, b, c
    );
}
template <typename T, int N>
vec<T, N> clamp(vec<T, N> a, T b, T c) {
    return vec<T, N>::map(
        [b, c](T x) { return clamp(x, b, c); },
        a
    );
}

template <typename T, int N>
vec<T, N> fract(vec<T, N> x, vec<T, N> *intpart) {
    vec<T, N> r;
    for (int i = 0; i < N; ++i) {
        r[i] = fract(x[i], intpart->s + i);
    }
    return r;
}
