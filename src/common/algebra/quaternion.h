#pragma once

#ifdef __cplusplus
#include <iostream>
#endif // __cplusplus

#include "complex.h"


#ifdef OPENCL_DEVICE

typedef float4 quaternion;

#ifdef OPENCL_INTEROP
typedef quaternion quaternion_packed;
#endif // OPENCL_INTEROP

#else // OPENCL_DEVICE

typedef struct quaternion quaternion;

#ifdef OPENCL_INTEROP
typedef cl_float4 quaternion_packed;
#endif // OPENCL_INTEROP

#endif // OPENCL_DEVICE


#ifdef __cplusplus
extern "C" {
#endif // __cplusplus


quaternion q_new(real r, real i, real j, real k);
quaternion q_new_r(real r);
quaternion q_new_c(complex a);

quaternion q_neg(quaternion a);
quaternion q_conj(quaternion a);
real q_abs2(quaternion a);
real q_abs(quaternion a);
quaternion q_norm(quaternion a);

quaternion qq_add(quaternion a, quaternion b);
quaternion qr_add(quaternion a, real b);
quaternion rq_add(real a, quaternion b);
quaternion qc_add(quaternion a, complex b);
quaternion cq_add(complex a, quaternion b);

quaternion qq_sub(quaternion a, quaternion b);
quaternion qr_sub(quaternion a, real b);
quaternion rq_sub(real a, quaternion b);
quaternion qc_sub(quaternion a, complex b);
quaternion cq_sub(complex a, quaternion b);

quaternion qq_mul(quaternion a, quaternion b);
quaternion qr_mul(quaternion a, real b);
quaternion rq_mul(real a, quaternion b);
quaternion qc_mul(quaternion a, complex b);
quaternion cq_mul(complex a, quaternion b);

quaternion qr_div(quaternion a, real b);
quaternion q_inv(quaternion a);
quaternion qq_div(quaternion a, quaternion b);
quaternion rq_div(real a, quaternion b);
quaternion qc_div(quaternion a, complex b);
quaternion cq_div(complex a, quaternion b);

real qq_dot(quaternion a, quaternion b);
real qq_dist(quaternion a, quaternion b);

#ifdef OPENCL_INTEROP
quaternion_packed q_pack(quaternion a);
quaternion q_unpack(quaternion_packed a);
#endif // OPENCL_INTEROP

#ifdef __cplusplus
};
#endif // __cplusplus


#ifdef OPENCL_DEVICE

#else // OPENCL_DEVICE

struct quaternion {
    real x, y, z, w;

#ifdef __cplusplus
    static const quaternion i, j, k;

    quaternion();
    quaternion(real r, real i, real j, real k);
    quaternion(real r);
    quaternion(complex c);

    quaternion conj() const;
    real abs2() const;
    real abs() const;
    quaternion norm() const;
    quaternion inv() const;

    quaternion &operator+=(quaternion b);
    quaternion &operator+=(complex b);
    quaternion &operator+=(real b);
    quaternion &operator-=(quaternion b);
    quaternion &operator-=(complex b);
    quaternion &operator-=(real b);
    quaternion &operator*=(quaternion b);
    quaternion &operator*=(complex b);
    quaternion &operator*=(real b);
    quaternion &operator/=(quaternion b);
    quaternion &operator/=(complex b);
    quaternion &operator/=(real b);

#ifdef OPENCL_INTEROP
    quaternion_packed pack() const;
#endif // OPENCL_INTEROP

#endif // __cplusplus
};

#ifdef __cplusplus

quaternion operator+(quaternion a);
quaternion operator-(quaternion a);

quaternion operator+(quaternion a, quaternion b);
quaternion operator+(quaternion a, real b);
quaternion operator+(real a, quaternion b);
quaternion operator+(quaternion a, complex b);
quaternion operator+(complex a, quaternion b);

quaternion operator-(quaternion a, quaternion b);
quaternion operator-(quaternion a, real b);
quaternion operator-(real a, quaternion b);
quaternion operator-(quaternion a, complex b);
quaternion operator-(complex a, quaternion b);

quaternion operator*(quaternion a, quaternion b);
quaternion operator*(quaternion a, real b);
quaternion operator*(real a, quaternion b);
quaternion operator*(quaternion a, complex b);
quaternion operator*(complex a, quaternion b);

quaternion operator/(quaternion a, real b);
quaternion operator/(quaternion a, quaternion b);
quaternion operator/(real a, quaternion b);
quaternion operator/(quaternion a, complex b);
quaternion operator/(complex a, quaternion b);

real dot(quaternion a, quaternion b);

std::ostream &operator<<(std::ostream &s, quaternion c);

quaternion operator "" _j(long double v);
quaternion operator "" _j(unsigned long long v);
quaternion operator "" _k(long double v);
quaternion operator "" _k(unsigned long long v);

#endif // __cplusplus

#endif // OPENCL_DEVICE


#ifdef OPENCL_DEVICE

#include "quaternion.c"

#endif // OPENCL_DEVICE


#ifdef UNIT_TEST
#ifdef __cplusplus

class q_approx {
private:
    quaternion v;
public:
    q_approx(quaternion q);
    friend bool operator==(quaternion a, q_approx b);
    friend bool operator==(q_approx a, quaternion b);
    friend std::ostream &operator<<(std::ostream &s, q_approx a);
};

quaternion rand_q_normal(Rng &rng);
quaternion rand_q_nonzero(Rng &rng);
quaternion rand_q_unit(Rng &rng);

#endif // __cplusplus
#endif // UNIT_TEST
