#pragma once

#include "vector.hpp"

template <typename C>
class complex {
private:
    typedef typename BaseType<C>::type T;
    static const int CN = Dim<C>::N;
    static const int N = 2*CN;
    vector<T, N> v;

public:
    complex() = default;
    complex(C r, C i) {
        re() = r;
        im() = i;
    }
    template <typename ...Args>
    complex(Args ...args) : v(args...) {}

    C &re() {
        return *reinterpret_cast<C*>(&v.slice<0, CN>());
    }
    const C &re() const {
        return *reinterpret_cast<const C*>(&v.slice<0, CN>());
    }
    C &im() {
        return *reinterpret_cast<C*>(&v.slice<CN, CN>());
    }
    const C &im() const {
        return *reinterpret_cast<const C*>(&v.slice<CN, CN>());
    }

    vector<T, N> &vec() {
        return v;
    }
    const vector<T, N> &vec() const {
        return v;
    }
};

template <typename C>
class Dim<complex<C>> {
    static const int N = 2*Dim<C>::N;
};
template <typename C>
class BaseType<complex<C>> {
    typedef BaseType<C>::type type;
};

/*
typedef real2 complex;
#define make_complex make_real2
#define c_new make_complex

#ifdef OPENCL_INTEROP
typedef real2_pk complex_pk;
#define pack_complex pack_real2
#define unpack_complex unpack_real2
#define c_pack pack_complex
#define c_unpack unpack_complex
#endif // OPENCL_INTEROP

#define C0 c_new(R0, R0)
#define C1 c_new(R1, R0)
#define CI c_new(R0, R1)


complex c_conj(complex a);
real c_abs2(complex a);
real c_abs(complex a);
real c_fabs(complex a);

complex c_mul(complex a, complex b);
complex c_inverse(complex a);
complex c_div(complex a, complex b);

complex c_exp(complex p);
complex c_powr(complex a, real p);
complex c_sqrt(complex a);


#ifdef UNIT_TEST
#include <catch.hpp>

complex rand_c_normal(TestRng &rng);
complex rand_c_unit(TestRng &rng);
complex rand_c_nonzero(TestRng &rng);

#endif // UNIT_TEST
*/
