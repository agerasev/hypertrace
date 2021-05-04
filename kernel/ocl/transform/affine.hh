#pragma once

#include <algebra/matrix.hh>
#include "linear.hh"
#include "shift.hh"

typedef struct {
    real4x4 v;
} Affine3;

#define $Self Affine3
#define $self aff3
#define $elem real3
#include "interface.inl"
#undef $Self
#undef $self
#undef $elem

Affine3 aff3_new(Linear3 m, Shift3 v);
Affine3 aff3_from_raw(real4x4 v);

Linear3 aff3_linear(Affine3 m);
Shift3 aff3_shift(Affine3 m);

real4 aff3_apply4(Affine3 m, real4 v);

Affine3 aff3_interpolate(Affine3 a, Affine3 b, real t);

#ifdef HOST
template <>
struct Distance<Affine3> {
    static real distance(Affine3 a, Affine3 b) {
        return ::distance(a.v, b.v);
    }
};
#endif // HOST

#ifdef UNITTEST

class TestRngAffine3 {
private:
    TestRngReal3x3 lrng;
    TestRng<real3> srng;

public:
    inline TestRngAffine3() = default;
    inline explicit TestRngAffine3(uint32_t seed) : lrng(seed), srng(seed) {}

    Affine3 normal();
    Affine3 uniform();
    Affine3 invertible();
};

#endif // UNITTEST


#ifndef HOST
#include "affine.cc"
#endif // !HOST
