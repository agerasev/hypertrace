#pragma once

#include "traits.hpp"
#include "complex.hpp"
#include "matrix.hpp"


template <typename T>
class Moebius;
template <typename T>
class MoebiusDeriv;

inline constexpr int smax(int a, int b) {
    return a > b ? a : b;
}
inline constexpr int smin(int a, int b) {
    return a < b ? a : b;
}

template <typename C>
class Moebius {
private:
    typedef base_type<C> T;
    static const int D = degree<C>();
    matrix<C, 2, 2> m;
public:
    Moebius() = default;
    template <typename ...Args>
    explicit Moebius(Args ...args) : m(args...) {}
    explicit Moebius(matrix<C, 2, 2> m) : m(m) {}

    static Moebius identity() {
        return Moebius(one<matrix<C, 2, 2>>());
    }

    C &a() { return m[0]; }
    const C &a() const { return m[0]; }
    C &b() { return m[1]; }
    const C &b() const { return m[1]; }
    C &c() { return m[2]; }
    const C &c() const { return m[2]; }
    C &d() { return m[3]; }
    const C &d() const { return m[3]; }

    matrix<C, 2, 2> &mat() {
        return m;
    }
    const matrix<C, 2, 2> &mat() const {
        return m;
    }

    C apply(T p) {
        return (a()*p + b())/(c()*p + d());
    }
    complex<T> apply(complex<T> p) {
        return (a()*p + b())/(c()*p + d());
    }
    quaternion<T> apply(quaternion<T> p) {
        return (a()*p + b())/(c()*p + d());
    }

    C deriv(T p) {
        C num = a()*p + b();
        C den = c()*p + d();
        return (a()*den - num*c())/(den*den);
    }
    complex<T> deriv(complex<T> p) {
        complex<T> num = a()*p + b();
        complex<T> den = c()*p + d();
        return (a()*den - num*c())/(den*den);
    }
    // For quaternion we can only compute directional derivative
    quaternion<T> deriv(quaternion<T> p, quaternion<T> v) {
        quaternion<T> num = a()*p + b();
        quaternion<T> den = c()*p + d();
        T den2 = length2(den);
        quaternion<T> s1 = (a()*v)/den;
        quaternion<T> s21 = ~(c()*v);
        quaternion<T> s22 = (2*dot(den, c()*v)/den2)*~den;
        quaternion<T> s2 = num*(s21 - s22)/den2;
        return s1 + s2;
    }
};

// Moebius transformation with quaternion coefficients is not supported yet.
template <typename T>
class Moebius<quaternion<T>> {};

template <typename C>
Moebius<C> chain(Moebius<C> a, Moebius<C> b) {
    return Moebius<C>(dot(a.mat(), b.mat()));
}
template <typename C>
Moebius<C> operator*(Moebius<C> a, Moebius<C> b) {
    return chain(a, b);
}

template <typename C>
Moebius<C> inverse(Moebius<C> a) {
    // We use adjugate matrix instead of true inverse
    // because all Moebius transformations should be normalized.
    return Moebius<C>(adj(a.mat()));
}
template <typename C>
Moebius<C> operator!(Moebius<C> a) {
    return inverse(a);
}

template <typename C>
Moebius<C> pow(Moebius<C> a, base_type<C> p) {
    return pow(a.mat(), p, true);
}

template <typename C>
Moebius<C> interpolate(Moebius<C> a, Moebius<C> b, base_type<C> t) {
    return a*pow(!a*b, t);
}


#ifdef UNIT_TEST
#include <catch.hpp>
#include "test.hpp"

namespace test {

template <typename C>
class Distrib<Moebius<C>> : public Rng {
public:
    Moebius<C> normal() {
        return Moebius<C>(distrib<matrix<C, 2, 2>>().normalized());
    }
};

} // namespace test

#endif
