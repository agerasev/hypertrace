#include "affine.hh"

Affine3 aff3_new(Linear3 l, Shift3 s) {
    Affine3 a = aff3_from_raw(l.v);
    a.v.s3 = s.v.x;
    a.v.s7 = s.v.y;
    a.v.sb = s.v.z;
    return a;
}

Affine3 aff3_from_raw(real4x4 v) {
    Affine3 a;
    a.v = v;
    return a;
}

Affine3 aff3_identity() {
    return aff3_from_raw(r44_one());
}

Linear3 aff3_linear(Affine3 m) {
    return lin3_new(r44_clip_to_r33(m.v));
}
Shift3 aff3_shift(Affine3 m) {
    return shf3_new(r3_new(m.v.s3, m.v.s7, m.v.sb));
}

real4 aff3_apply4(Affine3 m, real4 v) {
    return r44_dot_mv(m.v, v);
}

real3 aff3_apply_pos(Affine3 m, real3 p) {
    return aff3_apply4(m, r4_new(p, R1)).xyz;
}
real3 aff3_apply_dir(Affine3 m, real3 p, real3 d) {
    return lin3_apply_dir(aff3_linear(m), p, d);
}
real3 aff3_apply_normal(Affine3 m, real3 p, real3 d) {
    return lin3_apply_normal(aff3_linear(m), p, d);
}

Affine3 aff3_chain(Affine3 a, Affine3 b) {
    return aff3_from_raw(r44_dot(a.v, b.v));
}
Affine3 aff3_inverse(Affine3 m) {
    Linear3 l = lin3_inverse(aff3_linear(m));
    Shift3 s = shf3_inverse(aff3_shift(m));
    lin3_shf3_reorder(&l, &s);
    return aff3_new(l, s);
}

Affine3 aff3_interpolate(Affine3 a, Affine3 b, real t) {
    // FIXME: Use matrix power operation
    return aff3_from_raw((R1 - t) * a.v + t * b.v);
}

#ifdef UNITTEST

Affine3 TestRngAffine3::normal() {
    return aff3_new(lin3_new(lrng.normal()), shf3_new(srng.normal()));
}
Affine3 TestRngAffine3::uniform() {
    return aff3_new(lin3_new(lrng.uniform()), shf3_new(srng.uniform()));
}
Affine3 TestRngAffine3::invertible() {
    return aff3_new(lin3_new(lrng.invertible()), shf3_new(srng.normal()));
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
        real3 d = vrng.normal();

        ASSERT_EQ(aff3_apply_pos(aff3_chain(a, b), c), approx(aff3_apply_pos(a, aff3_apply_pos(b, c))));
        ASSERT_EQ(aff3_apply_dir(aff3_chain(a, b), c, d), approx(aff3_apply_dir(a, aff3_apply_pos(b, c), aff3_apply_dir(b, c, d))));
    }
}
TEST_F(AffineTest, inversion) {
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        Affine3 a = arng.invertible();
        real3 x = vrng.normal();
        
        const real IEPS = sqrt(EPS);
        ASSERT_EQ(
            aff3_apply_pos(aff3_inverse(a), aff3_apply_pos(a, x)),
            approx(x).epsilon(IEPS)
        );
        ASSERT_EQ(
            aff3_apply_pos(a, aff3_apply_pos(aff3_inverse(a), x)),
            approx(x).epsilon(IEPS)
        );
    }
}

#endif // UNITTEST
