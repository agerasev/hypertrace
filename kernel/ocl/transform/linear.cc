#include "linear.hh"

// Linear2

Linear2 lin2_identity() {
    return Linear2 { r22_one() };
}

real2 lin2_apply_pos(Linear2 m, real2 v) {
    return r22_dot_mv(m.v, v);
}
real2 lin2_apply_dir(Linear2 m, real2 p, real2 d) {
    return normalize(lin2_apply_pos(m, d));
}
real2 lin2_apply_normal(Linear2 m, real2 p, real2 d) {
    return normalize(r22_dot_mv(r22_transpose(r22_inverse(m.v)), d));
}

Linear2 lin2_chain(Linear2 a, Linear2 b) {
    return Linear2 { r22_dot(a.v, b.v) };
}
Linear2 lin2_inverse(Linear2 m) {
    return Linear2 { r22_inverse(m.v) };
}

void lin2_shf2_reorder(Linear2 *a, Shift2 *b) {
    b->v = lin2_apply_pos(*a, b->v);
}
void shf2_lin2_reorder(Shift2 *a, Linear2 *b) {
    a->v = lin2_apply_pos(lin2_inverse(*b), a->v);
}

// Linear3

Linear3 lin3_identity() {
    return Linear3 { r44_one() };
}

Linear3 lin3_look_to(real3 dir) {
    if (2*fabs(dir.z) < 1) {
        return lin3_look_to_up(dir, r3_new(0,0,1));
    } else {
        return lin3_look_to_up(dir, r3_new(0,1,0));
    }
}

Linear3 lin3_look_to_up(real3 dir, real3 up) {
    real3 right = normalize(cross(dir, up));
    real3 down = cross(dir, right);
    return Linear3 { MAKE(real3x3)(
        r4_new(right, R0),
        r4_new(down, R0),
        r4_new(dir, R0),
        r4_new(R0)
    ) };
}

real3 lin3_apply_pos(Linear3 m, real3 v) {
    return r33_dot_mv(m.v, v);
}
real3 lin3_apply_dir(Linear3 m, real3 p, real3 d) {
    return normalize(lin3_apply_pos(m, d));
}
real3 lin3_apply_normal(Linear3 m, real3 p, real3 d) {
    return normalize(r33_dot_mv(r33_transpose(r33_inverse(m.v)), d));
}

Linear3 lin3_chain(Linear3 a, Linear3 b) {
    return Linear3 { r33_dot(a.v, b.v) };
}
Linear3 lin3_inverse(Linear3 m) {
    return Linear3 { r33_inverse(m.v) };
}

void lin3_shf3_reorder(Linear3 *a, Shift3 *b) {
    b->v = lin3_apply_pos(*a, b->v);
}
void shf3_lin3_reorder(Shift3 *a, Linear3 *b) {
    a->v = lin3_apply_pos(lin3_inverse(*b), a->v);
}

#ifdef HOST

std::ostream &operator<<(std::ostream &o, Linear2 m) {
    return o << "Linear2 { " << m.v << " }";
}

std::ostream &operator<<(std::ostream &o, Linear3 m) {
    return o << "Linear3 { " << m.v << " }";
}

#endif // HOST

#ifdef UNITTEST

#include <gtest/gtest.h>

class LinearTest : public testing::Test {
protected:
    TestRng<real> rng = TestRng<real>(0xABBA);
    TestRng<real3> vrng = TestRng<real3>(0xBAAB);
    TestRngReal3x3 mrng = TestRngReal3x3(0xBEEB);
};

TEST_F(LinearTest, linearity) {
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        auto m = Linear3 { mrng.normal() };
        real a = rng.normal();
        real3 x = vrng.normal();

        ASSERT_EQ(lin3_apply_pos(Linear3 { a * m.v }, x), approx(lin3_apply_pos(m, a * x)));
        ASSERT_EQ(lin3_apply_pos(Linear3 { a * m.v }, x), approx(a * lin3_apply_pos(m, x)));
    }
}
TEST_F(LinearTest, chaining) {
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        auto a = Linear3 { mrng.normal() };
        auto b = Linear3 { mrng.normal() };
        real3 c = vrng.normal();

        ASSERT_EQ(lin3_chain(a, lin3_identity()), approx(a));
        ASSERT_EQ(lin3_chain(lin3_identity(), b), approx(b));
        ASSERT_EQ(lin3_apply_pos(lin3_chain(a, b), c), approx(lin3_apply_pos(a, lin3_apply_pos(b, c))));
    }
}
TEST_F(LinearTest, inversion) {
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        auto a = Linear3 { mrng.invertible() };
        real3 x = vrng.normal();

        ASSERT_EQ(lin3_chain(a, lin3_inverse(a)), approx(lin3_identity()));
        ASSERT_EQ(lin3_chain(lin3_inverse(a), a), approx(lin3_identity()));
        ASSERT_EQ(lin3_apply_pos(lin3_inverse(a), lin3_apply_pos(a, x)), approx(x));
        ASSERT_EQ(lin3_apply_pos(a, lin3_apply_pos(lin3_inverse(a), x)), approx(x));
    }
}
TEST_F(LinearTest, look_to_the_direction) {
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        real3 d = vrng.unit();
        Linear3 m = lin3_look_to(d);

        ASSERT_EQ(lin3_apply_pos(m, d), approx(r3_new(R0, R0, R1)));
    }
}

#endif // UNITTEST
