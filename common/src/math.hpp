#pragma once

#include <types.hpp>

#ifndef KERNEL

#include <cmath>
#include <algorithm>

namespace math {

using std::max;
using std::min;
template <typename T>
inline T clamp(T a, T b, T c) {
    return std::min(std::max(a, b), c);
}

using std::acos;
using std::acosh;
using std::asin;
using std::asinh;
using std::atan;
using std::atan2;
using std::atanh;
using std::ceil;
using std::cos;
using std::cosh;
using std::erfc;
using std::erf;
using std::exp;
using std::fabs;
using std::floor;
using std::fmax;
using std::fmin;
using std::fmod;
template <typename T>
inline T fract(T a, T *b) {
    *b = std::floor(a);
    return a - *b;
}
using std::log;
using std::pow;
using std::round;
using std::sqrt;
using std::sin;
using std::sinh;
using std::tan;
using std::tanh;
using std::tgamma;
template <typename T>
inline T sign(T a) {
    return ((T(0) < a) - (a - T(0)));
}

using std::abs;

#endif // !KERNEL

template <typename T>
T mod(T x, T y) {
    return x % y;
}
template <typename T>
T rem(T x, T y) {
    return mod(mod(x, y) + y, y);
}

} // namespace math

using namespace math;
