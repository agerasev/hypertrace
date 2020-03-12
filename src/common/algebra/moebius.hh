#pragma once

#include "complex.hh"
#include "quaternion.hh"
#include "matrix.hh"

typedef complex2x2 Moebius;

#ifdef OPENCL_INTEROP
typedef complex2x2_pk MoebiusPk;
#endif // OPENCL_INTEROP


#define mo_new complex2x2_new
#define mo_identity complex2x2_one

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
