#pragma once

#include "linear.hh"


typedef real4x4 Affine3;


Affine3 aff3_identity();
Affine3 aff3_from_ls(Linear3 m, real3 v);

Linear3 aff3_get_linear(Affine3 m);
real3 aff3_get_shift(Affine3 m);

real4 aff3_apply4(Affine3 m, real4 v);
real3 aff3_apply_pos(Affine3 m, real3 p);
real3 aff3_apply_dir(Affine3 m, real3 d);

Affine3 aff3_chain(Affine3 a, Affine3 b);
Affine3 aff3_inverse(Affine3 m);

Affine3 aff3_interpolate(Affine3 a, Affine3 b, real t);


#ifdef TEST_CATCH

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

#endif // TEST_CATCH
