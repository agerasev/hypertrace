#pragma once

#include "traits.hpp"
#include "complex.hpp"
#include "matrix.hpp"


template <typename C>
class MoebiusT {
private:
    typedef base_type<C> T;
    static const int D = degree<C>();
    matrix<C, 2, 2> m;
public:
    MoebiusT() = default;
    template <typename ...Args>
    explicit MoebiusT(Args ...args) : m(args...) {}
    explicit MoebiusT(matrix<C, 2, 2> m) : m(m) {}

    static MoebiusT identity() {
        return MoebiusT(one<matrix<C, 2, 2>>());
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

    C apply(T p) const {
        return (a()*p + b())/(c()*p + d());
    }
    complex<T> apply(complex<T> p) const {
        return (a()*p + b())/(c()*p + d());
    }
    quaternion<T> apply(quaternion<T> p) const {
        return (a()*p + b())/(c()*p + d());
    }

    C deriv(T p) const {
        C num = a()*p + b();
        C den = c()*p + d();
        return (a()*den - num*c())/(den*den);
    }
    complex<T> deriv(complex<T> p) const {
        complex<T> num = a()*p + b();
        complex<T> den = c()*p + d();
        return (a()*den - num*c())/(den*den);
    }
    // For quaternion we can only compute directional derivative
    quaternion<T> deriv(quaternion<T> p, quaternion<T> v) const {
        quaternion<T> num = a()*p + b();
        quaternion<T> den = c()*p + d();
        T den2 = length2(den);
        quaternion<T> s1 = (a()*v)/den;
        quaternion<T> s21 = ~(c()*v);
        quaternion<T> s22 = (2*dot(den, c()*v)/den2)*~den;
        quaternion<T> s2 = num*(s21 - s22)/den2;
        return s1 + s2;
    }

    MoebiusT &operator*=(MoebiusT a) {
        return *this = chain(*this, a);
    }
};

// MoebiusT transformation with quaternion coefficients is not supported yet.
template <typename T>
class MoebiusT<quaternion<T>> {};

template <typename C>
MoebiusT<C> chain(MoebiusT<C> a, MoebiusT<C> b) {
    return MoebiusT<C>(dot(a.mat(), b.mat()));
}
template <typename C>
MoebiusT<C> operator*(MoebiusT<C> a, MoebiusT<C> b) {
    return chain(a, b);
}

template <typename C>
MoebiusT<C> inverse(MoebiusT<C> a) {
    // We use adjugate matrix instead of true inverse
    // because all MoebiusT transformations should be normalized.
    return MoebiusT<C>(adj(a.mat()));
}
template <typename C>
MoebiusT<C> operator!(MoebiusT<C> a) {
    return inverse(a);
}

template <typename C>
MoebiusT<C> pow(MoebiusT<C> a, base_type<C> p) {
    return MoebiusT<C>(pow(a.mat(), p, true));
}

template <typename C>
MoebiusT<C> interpolate(MoebiusT<C> a, MoebiusT<C> b, base_type<C> t) {
    return a*pow(!a*b, t);
}

typedef MoebiusT<comp> Moebius;

#ifdef UNIT_TEST
#include <catch.hpp>
#include "test.hpp"

namespace test {

template <typename C>
class Distrib<MoebiusT<C>> : public Rng {
public:
    MoebiusT<C> some() {
        return MoebiusT<C>(d<matrix<C, 2, 2>>().normalized());
    }
};

} // namespace test

#endif
