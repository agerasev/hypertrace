#include "complex.hh"


comp c_conj(comp a) {
    return c_new(a.x, -a.y);
}
real c_abs2(comp a) {
    return dot(a, a);
}
real c_abs(comp a) {
    return length(a);
}
real c_norm_l1(comp a) {
    return fabs(a.x) + fabs(a.y);
}

comp c_mul(comp a, comp b) {
    return c_new(
        a.x*b.x - a.y*b.y,
        a.x*b.y + a.y*b.x
    );
}

comp c_inverse(comp a) {
    return c_conj(a)/c_abs2(a);
}

comp c_div(comp a, comp b) {
    return c_mul(a, c_inverse(b));
}

comp c_exp(comp p) {
    return exp(p.x)*c_new(cos(p.y), sin(p.y));
}
comp c_pow_r(comp a, real p) {
    real r = pow(c_abs2(a), p/2);
    real phi = p*atan2(a.y, a.x);
    return c_new(r*cos(phi), r*sin(phi));
}
comp c_sqrt(comp a) {
    real r = sqrt(c_abs(a));
    real phi = 0.5f*atan2(a.y, a.x);
    return c_new(r*cos(phi), r*sin(phi));
}


#ifdef UNITTEST

#include <gtest/gtest.h>

class ComplexTest : public testing::Test {
protected:
    TestRng<real> rng = TestRng<real>(0xbeef);
    TestRng<comp> crng = TestRng<comp>(0xbeef);
};

TEST_F(ComplexTest, constructor) {
    comp a = c_new(R0, R1);
    EXPECT_EQ(a.x, Approx(R0));
    EXPECT_EQ(a.y, Approx(R1));
}
TEST_F(ComplexTest, inversion) {
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        comp a = crng.nonzero();
        EXPECT_EQ(c_div(a, a), approx(C1));
    }
}
TEST_F(ComplexTest, square_root) {
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        comp a = crng.normal();
        comp b = c_sqrt(a);
        EXPECT_EQ(c_mul(b, b), approx(a));
    }
}
TEST_F(ComplexTest, power) {
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        comp a = crng.normal();
        int n = int(floor(2 + 10*rng.uniform()));
        comp b = c_pow_r(a, 1.0/n);
        comp c = C1;
        for (int i = 0; i < n; ++i) {
            c = c_mul(c, b);
        }
        EXPECT_EQ(c, approx(a));
    }
}
TEST_F(ComplexTest, norm) {
    EXPECT_EQ(c_norm_l1(c_new(-1, 2)), approx(3));
    EXPECT_EQ(length(c_new(3, -4)), approx(5));
}

#endif // UNITTEST
