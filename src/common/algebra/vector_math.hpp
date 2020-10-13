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


#define VECTOR_MATH_F1(F) \
template <typename T, int N> \
vec<T, N> F(vec<T, N> a) { \
    return vec<T, N>::map([](T x) { return F(x); }, a); \
} \

#define VECTOR_MATH_F2(F) \
template <typename T, int N> \
vec<T, N> F(vec<T, N> a, vec<T, N> b) { \
    return vec<T, N>::map([](T x, T y) { return F(x, y); }, a, b); \
} \

VECTOR_MATH_F1(fabs)
VECTOR_MATH_F2(fmax)
VECTOR_MATH_F2(fmin)
VECTOR_MATH_F1(acos)
VECTOR_MATH_F1(acosh)
VECTOR_MATH_F1(asin)
VECTOR_MATH_F1(asinh)
VECTOR_MATH_F1(atan)
VECTOR_MATH_F2(atan2)
VECTOR_MATH_F1(atanh)
VECTOR_MATH_F1(ceil)
VECTOR_MATH_F1(cos)
VECTOR_MATH_F1(cosh)
VECTOR_MATH_F1(erfc)
VECTOR_MATH_F1(erf)
VECTOR_MATH_F1(exp)
VECTOR_MATH_F1(floor)
VECTOR_MATH_F2(fmod)
VECTOR_MATH_F1(log)
VECTOR_MATH_F2(pow)
VECTOR_MATH_F1(round)
VECTOR_MATH_F1(sqrt)
VECTOR_MATH_F1(sin)
VECTOR_MATH_F1(sinh)
VECTOR_MATH_F1(tan)
VECTOR_MATH_F1(tanh)
VECTOR_MATH_F1(tgamma)
VECTOR_MATH_F1(sign)

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
