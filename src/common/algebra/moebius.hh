#pragma once

#include "complex.hh"
#include "quaternion.hh"

// Enable this if you want to use transformations which `mo_det(..) != 1`
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


#ifdef OPENCL
#include "moebius.cc"
#endif // OPENCL
