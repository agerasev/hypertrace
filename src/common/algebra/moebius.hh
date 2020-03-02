#pragma once

#include "complex.hh"
#include "quaternion.hh"

// Without this flag all Moebius transformations are assumed
// to be normalized (i.e. to have determinant equal to 1).
//#define MOEBIUS_DENORMALIZED

typedef struct Moebius {
    complex a, b, c, d;
} Moebius;

#ifdef OPENCL_INTEROP
typedef struct __attribute__ ((packed)) MoebiusPk {
    complex_pk a, b, c, d;
} MoebiusPk;
#endif // OPENCL_INTEROP


Moebius mo_new(complex a, complex b, complex c, complex d);
Moebius mo_identity();

quaternion mo_apply(Moebius m, quaternion p);
quaternion mo_deriv(Moebius m, quaternion p, quaternion v);

complex mo_det(Moebius m);
Moebius mo_normalize(Moebius m);
Moebius mo_inverse(Moebius m);

Moebius mo_chain(Moebius k, Moebius l);

void mo_eigen(Moebius m, Moebius *jordan, Moebius *vectors);
Moebius mo_pow(Moebius m, real p);

// These arithmetic operations may return denormalized result
// even without `MOEBIUS_DENORMALIZED` flag
Moebius mo_add(Moebius a, Moebius b);
Moebius mo_sub(Moebius a, Moebius b);
Moebius mo_mul(Moebius a, real b);
Moebius mo_div(Moebius a, real b);

real mo_fabs(Moebius m);
real mo_diff(Moebius a, Moebius b);

#ifdef OPENCL_INTEROP
MoebiusPk pack_moebius(Moebius m);
Moebius unpack_moebius(MoebiusPk p);
#define mo_pack pack_moebius
#define mo_unpack unpack_moebius
#endif // OPENCL_INTEROP


#ifndef OPENCL
#include <iostream>
std::ostream &operator<<(std::ostream &s, const Moebius &m);
#endif // OPENCL


#ifdef UNIT_TEST
Moebius random_moebius(TestRng &rng);
#endif // UNIT_TEST
