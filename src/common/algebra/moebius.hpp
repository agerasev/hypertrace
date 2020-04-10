#pragma once

#include "traits.hpp"
#include "complex.hpp"
#include "matrix.hpp"


template <typename T>
class Moebius;
template <typename T>
class MoebiusDeriv;

template <typename T>
struct CommonType {
    typedef type;
};

template <typename T>
class Moebius {
private:
    matrix<T, 2, 2> m;
public:
    template <typename ...Args>
    explicit Moebius(Args ...args) : m(args...) {}

    static Moebius identity() {
        // todo
    }

    T &a() { return m[0]; }
    const T &a() const { return m[0]; }
    T &b() { return m[1]; }
    const T &b() const { return m[1]; }
    T &c() { return m[2]; }
    const T &c() const { return m[2]; }
    T &d() { return m[3]; }
    const T &d() const { return m[3]; }

    quaternion<T> apply(quaternion<T> p) {
        return (a()*p + b())/(c()*p + d());
    }
}

// Moebius transformation with quaternion coefficients is not supported yet.
template <typename T>
class Moebius<quaternion<T>>;

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


#ifdef UNIT_TEST
#include <mat.hpp>
typedef MatApprox<complex,2,2,VecApprox<real,2>> ApproxMo;
Moebius random_moebius(TestRng &rng);
#endif // UNIT_TEST
