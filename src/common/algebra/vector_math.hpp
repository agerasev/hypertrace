#pragma once

#include <common/math.hh>

#include "vector_struct.hpp"


template <typename T, int N>
T dot(vector<T, N> a, vector<T, N> b) {
    T c = (T)0;
    for (int i = 0; i < N; ++i) {
        c += a[i]*b[i];
    }
    return c;
}
template <typename T>
vector<T, 3> cross(vector<T, 3> a, vector<T, 3> b) {
    return vector<T, 3>(
        a.y*b.z - a.z*b.y,
        a.z*b.x - a.x*b.z,
        a.x*b.y - a.y*b.x
    );
}
template <typename T>
vector<T, 4> cross(vector<T, 4> a, vector<T, 4> b) {
    return vector<T, 4>(cross(a.xyz, b.xyz), (T)0);
}

template <typename T, int N>
T length(vector<T, N> a) {
    return sqrt(dot(a, a));
}

template <typename T, int N>
vector<T, N> normalize(vector<T, N> a) {
    return a/length(a);
}


#define VECTOR_MATH_FA(F) \
template <typename T, int N> \
vector<T, N> F(vector<T, N> a) { \
    return vmap([](T x) { return F(x); }, a); \
} \

#define VECTOR_MATH_FAB(F) \
template <typename T, int N> \
vector<T, N> F(vector<T, N> a, vector<T, N> b) { \
    return vmap([](T x, T y) { return F(x, y); }, a, b); \
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
vector<T, N> fmin(vector<T, N> a, T b) {
    return vmap([b](T x) { return fmin(x, b); }, a);
}
template <typename T, int N>
vector<T, N> fmax(vector<T, N> a, T b) {
    return vmap([b](T x) { return fmax(x, b); }, a);
}
template <typename T, int N>
vector<T, N> pow(vector<T, N> a, T b) {
    return vmap([b](T x) { return pow(x, b); }, a);
}

template <typename T, int N>
vector<T, N> clamp(vector<T, N> a, vector<T, N> b, vector<T, N> c) {
    return vmap(
        [](T x, T y, T z) { return clamp(x, y, z); },
        a, b, c
    );
}
template <typename T, int N>
vector<T, N> clamp(vector<T, N> a, T b, T c) {
    return vmap(
        [b, c](T x) { return clamp(x, b, c); },
        a
    );
}

template <typename T, int N>
vector<T, N> fract(vector<T, N> x, vector<T, N> *intpart) {
    vector<T, N> r;
    for (int i = 0; i < N; ++i) {
        r[i] = fract(x[i], intpart->s + i);
    }
    return r;
}
