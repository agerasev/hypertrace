#pragma once

#ifdef __cplusplus
#include <iostream>
#endif // __cplusplus

#include "real.h"



#ifdef OPENCL_DEVICE

typedef float2 complex;

#ifdef OPENCL_INTEROP
typedef complex complex_packed;
#endif // OPENCL_INTEROP

#else // OPENCL_DEVICE

typedef struct complex complex;

#ifdef OPENCL_INTEROP
typedef cl_float2 complex_packed;
#endif // OPENCL_INTEROP

#endif // OPENCL_DEVICE


#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

complex c_new(real r, real i);
complex c_new_r(real r);

complex c_neg(complex a);
complex c_conj(complex a);
real c_abs2(complex a);
real c_abs(complex a);
complex c_norm(complex a);

complex cc_add(complex a, complex b);
complex cr_add(complex a, real b);
complex rc_add(real a, complex b);

complex cc_sub(complex a, complex b);
complex cr_sub(complex a, real b);
complex rc_sub(real a, complex b);

complex cr_mul(complex a, real b);
complex rc_mul(real a, complex b);
complex cc_mul(complex a, complex b);

complex cr_div(complex a, real b);
complex c_inv(complex a);
complex cc_div(complex a, complex b);
complex rc_div(real a, complex b);

complex c_sqrt(complex a);
real cc_dot(complex a, complex b);

#ifdef OPENCL_INTEROP
complex_packed c_pack(complex a);
complex c_unpack(complex_packed a);
#endif // OPENCL_INTEROP

#ifdef __cplusplus
};
#endif // __cplusplus


#ifdef OPENCL_DEVICE

#else // OPENCL_DEVICE

struct complex {
    real x, y;

#ifdef __cplusplus
    static const complex i;

    complex();
    complex(real r, real i);
    complex(real r);

    complex conj() const;
    real abs2() const;
    real abs() const;
    complex norm() const;
    complex inv() const;

    complex &operator+=(complex b);
    complex &operator+=(real b);
    complex &operator-=(complex b);
    complex &operator-=(real b);
    complex &operator*=(complex b);
    complex &operator*=(real b);
    complex &operator/=(complex b);
    complex &operator/=(real b);

#ifdef OPENCL_INTEROP
    complex_packed pack() const;
#endif // OPENCL_INTEROP

#endif // __cplusplus
};

#ifdef __cplusplus
complex operator+(complex a);
complex operator-(complex a);

complex operator+(complex a, complex b);
complex operator+(complex a, real b);
complex operator+(real a, complex b);

complex operator-(complex a, complex b);
complex operator-(complex a, real b);
complex operator-(real a, complex b);

complex operator*(complex a, real b);
complex operator*(real a, complex b);
complex operator*(complex a, complex b);

complex operator/(complex a, real b);
complex operator/(complex a, complex b);
complex operator/(real a, complex b);

complex sqrt(complex a);
real dot(complex a, complex b);

std::ostream &operator<<(std::ostream &s, complex c);

complex operator "" _i(long double i);
complex operator "" _i(unsigned long long i);

#endif // __cplusplus

#endif // OPENCL_DEVICE


#ifdef OPENCL_DEVICE

#include "complex.c"

#endif // OPENCL_DEVICE


#ifdef UNIT_TEST
#ifdef __cplusplus

complex rand_c_normal(Rng &rng);
complex rand_c_unit(Rng &rng);
complex rand_c_nonzero(Rng &rng);

class c_approx {
private:
    complex v;
public:
    c_approx(complex c);
    friend bool operator==(complex a, c_approx b);
    friend bool operator==(c_approx a, complex b);
    friend std::ostream &operator<<(std::ostream &s, c_approx a);
};

#endif // __cplusplus
#endif // UNIT_TEST
