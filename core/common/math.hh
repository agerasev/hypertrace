#pragma once

#include "types.hh"


#ifdef HOST

#include <cmath>
#include <type_traits>

template <typename T>
T clamp(T a, T b, T c) {
    return std::min(std::max(a, b), c);
}
template <typename T>
T max(T a, T b) {
    return std::max(a, b);
}
template <typename T>
T min(T a, T b) {
    return std::min(a, b);
}

template <typename T>
T acos(T a) {
    return std::acos(a);
}
template <typename T>
T acosh(T a) {
    return std::acosh(a);
}
template <typename T>
T asin(T a) {
    return std::asin(a);
}
template <typename T>
T asinh(T a) {
    return std::asinh(a);
}
template <typename T>
T atan(T a) {
    return std::atan(a);
}
template <typename T>
T atan2(T a, T b) {
    return std::atan2(a, b);
}
template <typename T>
T atanh(T a) {
    return std::atanh(a);
}
template <typename T>
T ceil(T a) {
    return std::ceil(a);
}
template <typename T>
T cos(T a) {
    return std::cos(a);
}
template <typename T>
T cosh(T a) {
    return std::cosh(a);
}
template <typename T>
T erfc(T a) {
    return std::erfc(a);
}
template <typename T>
T erf(T a) {
    return std::erf(a);
}
template <typename T>
T exp(T a) {
    return std::exp(a);
}
template <typename T>
T fabs(T a) {
    return std::fabs(a);
}
template <typename T>
T floor(T a) {
    return std::floor(a);
}
template <typename T>
T fmax(T a, T b) {
    return std::fmax(a, b);
}
template <typename T>
T fmin(T a, T b) {
    return std::fmin(a, b);
}
template <typename T>
T fmod(T a, T b) {
    return std::fmod(a, b);
}
template <typename T>
T fract(T a, T *b) {
    *b = std::floor(a);
    return a - *b;
}
template <typename T>
T log(T a) {
    return std::log(a);
}
template <typename T>
T pow(T a, T b) {
    return std::pow(a, b);
}
template <typename T>
T round(T a) {
    return std::round(a);
}
template <typename T>
T sqrt(T a) {
    return std::sqrt(a);
}
template <typename T>
T sin(T a) {
    return std::sin(a);
}
template <typename T>
T sinh(T a) {
    return std::sinh(a);
}
template <typename T>
T tan(T a) {
    return std::tan(a);
}
template <typename T>
T tanh(T a) {
    return std::tanh(a);
}
template <typename T>
T tgamma(T a) {
    return std::tgamma(a);
}
template <typename T>
T sign(T a) {
    return ((T(0) < a) - (a < T(0)));
}

template <typename T>
T abs(T a) {
    return std::abs(a);
}


template <typename T, typename U>
std::common_type_t<T, U> upper_multiple(T m, U x) {
    return ((x + m - 1)/m)*m;
}

#else // !HOST

#define upper_multiple(m, x) \
    (((x + m - 1)/m)*m)

#endif // HOST

int mod(int a, int b);
