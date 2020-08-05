#pragma once

#include "complex.hh"
#include "quaternion.hh"
#include "matrix.hh"


typedef comp2x2 Moebius;


#define mo_new c22_new
#define mo_identity c22_one

comp mo_apply_c(Moebius m, comp p);
quat mo_apply_q(Moebius m, quat p);

comp mo_deriv_c(Moebius m, comp p);
quat mo_deriv_q(Moebius m, quat p, quat v);

#define mo_inverse c22_inverse_n

#define mo_chain c22_dot

#define mo_eigen c22_eigen_n
#define mo_pow c22_pow_n

Moebius mo_interpolate(Moebius a, Moebius b, real t);

#define mo_mul c22_mul
#define mo_div c22_div

#define mo_norm_l1 c22_norm_l1
real mo_diff(Moebius a, Moebius b);
