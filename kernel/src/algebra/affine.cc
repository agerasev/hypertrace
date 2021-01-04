#include "affine.hh"


Affine3 aff3_identity() {
    return r44_one();
}
Affine3 aff3_from_ls(Linear3 m, real3 v) {
    m.s3 = v.x;
    m.s7 = v.y;
    m.sb = v.z;
    return m;
}

Linear3 aff3_get_linear(Affine3 m) {
    return r44_clip_to_r33(m);
}
real3 aff3_get_shift(Affine3 m) {
    return r3_new(m.s3, m.s7, m.sb);
}

real4 aff3_apply4(Affine3 m, real4 v) {
    return r44_dot_mv(m, v);
}
real3 aff3_apply_pos(Affine3 m, real3 p) {
    return aff3_apply4(m, r4_new(p, R1)).xyz;
}
real3 aff3_apply_dir(Affine3 m, real3 d) {
    return aff3_apply4(m, r4_new(d, R0)).xyz;
}

Affine3 aff3_chain(Affine3 a, Affine3 b) {
    return r44_dot(a, b);
}
Affine3 aff3_inverse(Affine3 m) {
    Linear3 l = lin3_inverse(m); // aff3_get_linear(m)
    real3 s = -lin3_apply(l, aff3_get_shift(m));
    return aff3_from_ls(l, s);
}

Affine3 aff3_interpolate(Affine3 a, Affine3 b, real t) {
    // FIXME: Use matrix power operation
    return (R1 - t)*a + t*b;
}


#ifdef UNITTEST

Affine3 TestRngAffine3::normal() {
    return aff3_from_ls(lrng.normal(), srng.normal());
}
Affine3 TestRngAffine3::uniform() {
    return aff3_from_ls(lrng.uniform(), srng.uniform());
}
Affine3 TestRngAffine3::invertible() {
    return aff3_from_ls(lrng.invertible(), srng.normal());
}

#include <gtest/gtest.h>

class AffineTest : public testing::Test {
protected:
    TestRngAffine3 arng = TestRngAffine3(0xAFFE);
    TestRng<real3> vrng = TestRng<real3>(0xAFFE);
};

TEST_F(AffineTest, chaining) {
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        Affine3 a = arng.normal();
        Affine3 b = arng.normal();
        real3 c = vrng.normal();

        EXPECT_EQ(aff3_apply_pos(aff3_chain(a, b), c), approx(aff3_apply_pos(a, aff3_apply_pos(b, c))));
        EXPECT_EQ(aff3_apply_dir(aff3_chain(a, b), c), approx(aff3_apply_dir(a, aff3_apply_dir(b, c))));
    }
}
TEST_F(AffineTest, inversion) {
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        Affine3 a = arng.invertible();
        real3 x = vrng.normal();
        
        const real IEPS = sqrt(EPS);
        EXPECT_EQ(
            aff3_apply_pos(aff3_inverse(a), aff3_apply_pos(a, x)),
            approx(x).epsilon(IEPS)
        );
        EXPECT_EQ(
            aff3_apply_pos(a, aff3_apply_pos(aff3_inverse(a), x)),
            approx(x).epsilon(IEPS)
        );
        EXPECT_EQ(
            aff3_apply_dir(aff3_inverse(a), aff3_apply_dir(a, x)),
            approx(x).epsilon(IEPS)
        );
        EXPECT_EQ(
            aff3_apply_dir(a, aff3_apply_dir(aff3_inverse(a), x)),
            approx(x).epsilon(IEPS)
        );
    }
}

#endif // UNITTEST
