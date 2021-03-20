#pragma once

#include <types.hh>
#include <macros.hh>

#define C_PI 3.141592653589793238462643383279502884
#define C_E 2.718281828459045235360287471352662497

#ifdef DOUBLE_PRECISION

#define PI C_PI
#define E C_E
#define EPS 1e-12
//#define EPS 1e-8
#define R0 0.0
#define R1 1.0

#else

#define PI CONCAT(C_PI, f)
#define E CONCAT(C_E, f)
#define EPS 1e-6f
#define R0 0.0f
#define R1 1.0f

#endif


#ifdef UNITTEST

#define TEST_ATTEMPTS 1024
//#define TEST_ATTEMPTS 16

#include <random>

template <typename T>
class TestRng {
private:
    std::minstd_rand rng;
    std::uniform_real_distribution<T> unif;
    std::normal_distribution<T> norm;

public:
    inline explicit TestRng(uint32_t seed=0xdeadbeef) : rng(seed) {}

    inline T uniform() {
        return unif(rng);
    }
    inline T normal() {
        return norm(rng);
    }
};


#include <iostream>
#include <type_traits>

#define APPROX_EPS pow(EPS, (real)2/3)

template <typename T>
class Approx {
public:
    real _epsilon = APPROX_EPS;
    T _value = R0;

    explicit Approx(T value) :
        _value(value)    
    {}
    Approx epsilon(real eps) const {
        Approx copy_ = *this;
        copy_._epsilon = eps;
        return copy_;
    }
    bool operator==(T x) const {
        return std::abs(_value - x) <= _epsilon;
    }
};

template <typename T>
bool operator==(T a, Approx<T> b) {
    return b == a;
}
template <typename T>
bool operator!=(Approx<T> a, T b) {
    return !(a == b);
}
template <typename T>
bool operator!=(T a, Approx<T> b) {
    return b != a;
}
template <typename T>
std::ostream &operator<<(std::ostream &s, const Approx<T> &a) {
    return s << "approx(" << a._value << ", eps=" << a._epsilon << ")";
}

template <typename T, typename X=std::enable_if_t<!std::is_integral_v<T>, void>>
Approx<T> approx(T v) {
    return Approx<T>(v);
}
template <typename T, typename X=std::enable_if_t<std::is_integral_v<T>, void>>
Approx<real> approx(T v) {
    return Approx<real>(real(v));
}

#endif // UNITTEST


#ifndef HOST
#include "real.cc"
#endif // !HOST
