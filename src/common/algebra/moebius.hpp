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

    quaternion<T> apply(quaternion<T> p) {
        return (a()*p + b())/(c()*p + d());
    }
    complex<T> apply(complex<T> p) {
        return (a()*p + b())/(c()*p + d());
    }
    C apply(T p) {
        return (a()*p + b())/(c()*p + d());
    }
};

// Moebius transformation with quaternion coefficients is not supported yet.
template <typename T>
class Moebius<quaternion<T>> {};

template <typename T>
Moebius<T> chain(Moebius<T> a, Moebius<T> b) {
    return Moebius<T>(dot(a.mat(), b.mat()));
}

/*
quaternion mo_apply(Moebius m, quaternion p);
quaternion mo_deriv(Moebius m, quaternion p, quaternion v);

#define mo_det complex2x2_det
#define mo_normalize complex2x2_normalize
#define mo_inverse complex2x2_inverse_normalized

#define mo_chain complex2x2x2_dot

#define mo_eigen complex2x2_eigen_normalized
#define mo_pow complex2x2_pow
Moebius mo_interpolate(Moebius a, Moebius b, real t);

#define mo_add complex2x2_add
#define mo_sub complex2x2_sub
#define mo_mul complex2x2_mul
#define mo_div complex2x2_div

#define mo_fabs complex2x2_fabs
real mo_diff(Moebius a, Moebius b);

#ifdef OPENCL_INTEROP
#define pack_moebius pack_complex2x2
#define unpack_moebius unpack_complex2x2
#define mo_pack pack_moebius
#define mo_unpack unpack_moebius
#endif // OPENCL_INTEROP
*/
