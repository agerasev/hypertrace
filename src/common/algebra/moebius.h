#pragma once

#ifdef __cplusplus
#include <iostream>
#endif // __cplusplus

#include "quaternion.h"


typedef struct Moebius Moebius;

#ifdef OPENCL_INTEROP
typedef struct MoebiusPacked MoebiusPacked;
#endif // OPENCL_INTEROP


#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void moebius_new(Moebius *o, complex a, complex b, complex c, complex d);

void moebius_new_identity(Moebius *o);
void moebius_new_zshift(Moebius *o, real l);
void moebius_new_xshift(Moebius *o, real l);
void moebius_new_yshift(Moebius *o, real l);
void moebius_new_zrotate(Moebius *o, real phi);
void moebius_new_xrotate(Moebius *o, real theta);
void moebius_new_yrotate(Moebius *o, real theta);
void moebius_new_lookat(Moebius *o, quaternion dir);

quaternion moebius_apply(const Moebius *m, quaternion p);
quaternion moebius_deriv(const Moebius *m, quaternion p, quaternion v);

complex moebius_det(const Moebius *m);
void moebius_inverse(Moebius *o, const Moebius *m);

void moebius_chain(Moebius *o, const Moebius *k, const Moebius *l);

#ifdef OPENCL_INTEROP
void moebius_pack(MoebiusPacked *p, const Moebius *u);
void moebius_unpack(Moebius *u, const MoebiusPacked *p);
#endif // OPENCL_INTEROP

#ifdef __cplusplus
};
#endif // __cplusplus


struct Moebius {
    complex a, b, c, d;

#ifdef __cplusplus
    Moebius();
    Moebius(complex a, complex b, complex c, complex d);

    quaternion apply(quaternion p) const;
    quaternion deriv(quaternion p, quaternion v) const;

    Moebius operator*(const Moebius &other) const;
    Moebius &operator*=(const Moebius &other);

    complex det() const;
    Moebius inverse() const;

    static Moebius identity();
    static Moebius zshift(real l);
    static Moebius xshift(real l);
    static Moebius yshift(real l);
    static Moebius zrotate(real phi);
    static Moebius xrotate(real theta);
    static Moebius yrotate(real theta);
    static Moebius lookat(quaternion dir);

#ifdef OPENCL_INTEROP
    MoebiusPacked pack() const;
#endif // OPENCL_INTEROP

#endif // __cplusplus
};


#ifdef OPENCL_INTEROP

struct __attribute__ ((packed)) MoebiusPacked {
    complex_packed a, b, c, d;

#ifdef __cplusplus
    Moebius unpack() const;
#endif // __cplusplus

};

#endif // OPENCL_INTEROP


#ifdef OPENCL_DEVICE

#include "moebius.c"

#endif // OPENCL_DEVICE


#ifdef __cplusplus

std::ostream &operator<<(std::ostream &s, const Moebius &m);

#endif // __cplusplus


#ifdef UNIT_TEST
#ifdef __cplusplus

Moebius random_moebius(Rng &rng);

#endif // __cplusplus
#endif // UNIT_TEST
