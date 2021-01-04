#include "rotation.hh"


Rotation2 rot2_identity() {
    return C1;
}
Rotation2 rot2_from_angle(real angle) {
    return c_new(cos(angle), sin(angle));
}
Rotation2 rot2_look_at(real2 pos) {
    return normalize(c_new(pos.x, pos.y));
}

real2 rot2_apply(Rotation2 m, real2 pos) {
    return c_mul(m, pos);
}
Rotation2 rot2_chain(Rotation2 a, Rotation2 b) {
    return c_mul(a, b);
}
Rotation2 rot2_inverse(Rotation2 m) {
    return c_inverse(m);
}
Linear2 rot2_to_linear(Rotation2 m) {
    return r22_transpose(r22_from_comp(m));
}


Rotation3 rot3_identity() {
    return Q1;
}
Rotation3 rot3_from_axis(real3 axis, real angle) {
    return q_new(cos(angle/2), sin(angle/2)*axis);
}

real3 rot3_apply(Rotation3 m, real3 p) {
    return q_mul(q_mul(m, q_new(R0, p)), q_conj(m)).yzw;
}
Rotation3 rot3_chain(Rotation3 a, Rotation3 b) {
    return q_mul(a, b);
}
Rotation3 rot3_inverse(Rotation3 m) {
    return q_conj(m);
}
Linear3 rot3_to_linear(Rotation3 m) {
    return r33_new(
        1 - 2*m.z*m.z - 2*m.w*m.w,
        2*m.y*m.z - 2*m.w*m.x,
        2*m.y*m.w + 2*m.z*m.x,
        R0,

        2*m.y*m.z + 2*m.w*m.x,
        1 - 2*m.y*m.y - 2*m.w*m.w,
        2*m.z*m.w - 2*m.y*m.x,
        R0,

        2*m.y*m.w - 2*m.z*m.x,
        2*m.z*m.w + 2*m.y*m.x,
        1 - 2*m.y*m.y - 2*m.z*m.z,
        R0,

        R0, R0, R0, R1
    );
}


#ifdef UNITTEST

Rotation2 TestRngRotation2::uniform() {
    return rot2_from_angle(2*PI*rng.uniform());
}

Rotation3 TestRngRotation3::uniform() {
    return rot3_from_axis(
        vrng.unit(),
        2*PI*rng.uniform()
    );
}

#include <gtest/gtest.h>

class Rotation2Test : public testing::Test {
protected:
    TestRng<real> rng = TestRng<real>(0x807A);
    TestRng<real2> r2rng = TestRng<real2>(0x807A);
    TestRngRotation2 rot2rng = TestRngRotation2(0x807A);
};

TEST_F(Rotation2Test, mapping) {
    for (int k = 0; k < TEST_ATTEMPTS; ++k) {
        real angle = 2*PI*rng.uniform();
        Rotation2 r = rot2_from_angle(angle);
        real2 a = r2rng.normal();
        real2 b = rot2_apply(r, a);
        EXPECT_EQ(length(a), approx(length(b)));
        EXPECT_EQ(dot(a, b)/length2(a), approx(cos(angle)));
    }
}
TEST_F(Rotation2Test, chaining) {
    for (int k = 0; k < TEST_ATTEMPTS; ++k) {
        Rotation2 a = rot2rng.uniform();
        Rotation2 b = rot2rng.uniform();
        real2 v = r2rng.normal();
        EXPECT_EQ(
            rot2_apply(rot2_chain(a, b), v), 
            approx(rot2_apply(a, rot2_apply(b, v)))
        );
    }
}
TEST_F(Rotation2Test, inversion) {
    for (int k = 0; k < TEST_ATTEMPTS; ++k) {
        Rotation2 r = rot2rng.uniform();
        EXPECT_EQ(rot2_chain(r, rot2_inverse(r)), approx(rot2_identity()));
    }
}
TEST_F(Rotation2Test, to_linear) {
    for (int k = 0; k < TEST_ATTEMPTS; ++k) {
        Rotation2 a = rot2rng.uniform();
        Rotation2 b = rot2rng.uniform();
        real2 x = r2rng.normal();
        EXPECT_EQ(lin2_apply(rot2_to_linear(a), x), approx(rot2_apply(a, x)));
        EXPECT_EQ(
            lin2_chain(rot2_to_linear(a), rot2_to_linear(b)),
            approx(rot2_to_linear(rot2_chain(a, b)))
        );
    }
}

class Rotation3Test : public testing::Test {
protected:
    TestRng<real> rng = TestRng<real>(0x807A);
    TestRng<real3> r3rng = TestRng<real3>(0x807A);
    TestRngRotation3 rot3rng = TestRngRotation3(0x807A);
};

TEST_F(Rotation3Test, mapping) {
    for (int k = 0; k < TEST_ATTEMPTS; ++k) {
        real3 axis = r3rng.unit();
        real angle = 2*PI*rng.uniform();
        Rotation3 r = rot3_from_axis(axis, angle);
        real3 a = r3rng.normal();
        real3 b = rot3_apply(r, a);
        EXPECT_EQ(length(a), approx(length(b)));
        a -= dot(a, axis)*axis;
        b -= dot(b, axis)*axis;
        real aa = length2(a);
        EXPECT_EQ(dot(a, b)/aa, approx(cos(angle)));
        EXPECT_EQ(cross(a, b)/aa, approx(axis*sin(angle)));
    }
}
TEST_F(Rotation3Test, chaining) {
    for (int k = 0; k < TEST_ATTEMPTS; ++k) {
        Rotation3 a = rot3rng.uniform();
        Rotation3 b = rot3rng.uniform();
        real3 v = r3rng.normal();
        EXPECT_EQ(
            rot3_apply(rot3_chain(a, b), v),
            approx(rot3_apply(a, rot3_apply(b, v)))
        );
    }
}
TEST_F(Rotation3Test, inversion) {
    for (int k = 0; k < TEST_ATTEMPTS; ++k) {
        Rotation3 r = rot3rng.uniform();
        EXPECT_EQ(rot3_chain(r, rot3_inverse(r)), approx(rot3_identity()));
    }
}
TEST_F(Rotation3Test, to_linear) {
    for (int k = 0; k < TEST_ATTEMPTS; ++k) {
        Rotation3 a = rot3rng.uniform();
        Rotation3 b = rot3rng.uniform();
        real3 x = r3rng.normal();
        EXPECT_EQ(lin3_apply(rot3_to_linear(a), x), approx(rot3_apply(a, x)));
        EXPECT_EQ(
            lin3_chain(rot3_to_linear(a), rot3_to_linear(b)),
            approx(rot3_to_linear(rot3_chain(a, b)))
        );
    }
}

#endif // TEST_UNIT
