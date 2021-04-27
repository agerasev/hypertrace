#include "rotation.hh"

// Rotation2

Rotation2 rot2_identity() {
    return Rotation2 { C1 };
}
Rotation2 rot2_from_angle(real angle) {
    return Rotation2 { c_new(cos(angle), sin(angle)) };
}
Rotation2 rot2_look_at(real2 pos) {
    return Rotation2 { normalize(c_new(pos.x, pos.y)) };
}

real2 rot2_apply_pos(Rotation2 m, real2 pos) {
    return c_mul(m.v, pos);
}
real2 rot2_apply_dir(Rotation2 m, real2 pos, real2 dir) {
    return rot2_apply_pos(m, dir);
}
real2 rot2_apply_pos_normal(Rotation2 m, real2 pos, real2 dir) {
    return rot2_apply_dir(m, pos, dir);
}

Rotation2 rot2_chain(Rotation2 a, Rotation2 b) {
    return Rotation2 { c_mul(a.v, b.v) };
}
Rotation2 rot2_inverse(Rotation2 m) {
    return Rotation2 { c_inverse(m.v) };
}

real rot2_distance(Rotation2 a, Rotation2 b) {
    return distance(a.v, b.v);
}

Linear2 rot2_to_linear(Rotation2 m) {
    return Linear2 { r22_transpose(r22_from_comp(m.v)) };
}

void rot2_shf2_reorder(Rotation2 *a, Shift2 *b) {
    b->v = rot2_apply_pos(*a, b->v);
}
void shf2_rot2_reorder(Shift2 *a, Rotation2 *b) {
    a->v = rot2_apply_pos(rot2_inverse(*b), a->v);
}

// Rotation3

Rotation3 rot3_identity() {
    return Rotation3 { Q1 };
}
Rotation3 rot3_from_axis(real3 axis, real angle) {
    return Rotation3 { q_new(cos(angle/2), sin(angle/2)*axis) };
}

real3 rot3_apply_pos(Rotation3 m, real3 pos) {
    return q_mul(q_mul(m.v, q_new(R0, pos)), q_conj(m.v)).yzw;
}
real3 rot3_apply_dir(Rotation3 m, real3 pos, real3 dir) {
    return rot3_apply_pos(m, dir);
}
real3 rot3_apply_pos_normal(Rotation3 m, real3 pos, real3 dir) {
    return rot3_apply_dir(m, pos, dir);
}

Rotation3 rot3_chain(Rotation3 a, Rotation3 b) {
    return Rotation3 { q_mul(a.v, b.v) };
}
Rotation3 rot3_inverse(Rotation3 m) {
    return Rotation3 { q_conj(m.v) };
}

real rot3_distance(Rotation3 a, Rotation3 b) {
    return distance(a.v, b.v);
}

Linear3 rot3_to_linear(Rotation3 m) {
    quat q = m.v;
    return Linear3 { r33_new(
        1 - 2*q.z*q.z - 2*q.w*q.w,
        2*q.y*q.z - 2*q.w*q.x,
        2*q.y*q.w + 2*q.z*q.x,
        R0,

        2*q.y*q.z + 2*q.w*q.x,
        1 - 2*q.y*q.y - 2*q.w*q.w,
        2*q.z*q.w - 2*q.y*q.x,
        R0,

        2*q.y*q.w - 2*q.z*q.x,
        2*q.z*q.w + 2*q.y*q.x,
        1 - 2*q.y*q.y - 2*q.z*q.z,
        R0,

        R0, R0, R0, R1
    ) };
}

void rot3_shf3_reorder(Rotation3 *a, Shift3 *b) {
    b->v = rot3_apply_pos(*a, b->v);
}
void shf3_rot3_reorder(Shift3 *a, Rotation3 *b) {
    a->v = rot3_apply_pos(rot3_inverse(*b), a->v);
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
        real2 b = rot2_apply_pos(r, a);
        ASSERT_EQ(length(a), approx(length(b)));
        ASSERT_EQ(dot(a, b)/length2(a), approx(cos(angle)));
    }
}
TEST_F(Rotation2Test, chaining) {
    for (int k = 0; k < TEST_ATTEMPTS; ++k) {
        Rotation2 a = rot2rng.uniform();
        Rotation2 b = rot2rng.uniform();
        real2 v = r2rng.normal();
        ASSERT_EQ(
            rot2_apply_pos(rot2_chain(a, b), v), 
            approx(rot2_apply_pos(a, rot2_apply_pos(b, v)))
        );
    }
}
TEST_F(Rotation2Test, inversion) {
    for (int k = 0; k < TEST_ATTEMPTS; ++k) {
        Rotation2 r = rot2rng.uniform();
        ASSERT_EQ(rot2_chain(r, rot2_inverse(r)), approx(rot2_identity()));
    }
}
TEST_F(Rotation2Test, to_linear) {
    for (int k = 0; k < TEST_ATTEMPTS; ++k) {
        Rotation2 a = rot2rng.uniform();
        Rotation2 b = rot2rng.uniform();
        real2 x = r2rng.normal();
        ASSERT_EQ(lin2_apply_pos(rot2_to_linear(a), x), approx(rot2_apply_pos(a, x)));
        ASSERT_EQ(
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
        real3 b = rot3_apply_pos(r, a);
        ASSERT_EQ(length(a), approx(length(b)));
        a -= dot(a, axis)*axis;
        b -= dot(b, axis)*axis;
        real aa = length2(a);
        ASSERT_EQ(dot(a, b)/aa, approx(cos(angle)));
        ASSERT_EQ(cross(a, b)/aa, approx(axis*sin(angle)));
    }
}
TEST_F(Rotation3Test, chaining) {
    for (int k = 0; k < TEST_ATTEMPTS; ++k) {
        Rotation3 a = rot3rng.uniform();
        Rotation3 b = rot3rng.uniform();
        real3 v = r3rng.normal();
        ASSERT_EQ(
            rot3_apply_pos(rot3_chain(a, b), v),
            approx(rot3_apply_pos(a, rot3_apply_pos(b, v)))
        );
    }
}
TEST_F(Rotation3Test, inversion) {
    for (int k = 0; k < TEST_ATTEMPTS; ++k) {
        Rotation3 r = rot3rng.uniform();
        ASSERT_EQ(rot3_chain(r, rot3_inverse(r)), approx(rot3_identity()));
    }
}
TEST_F(Rotation3Test, to_linear) {
    for (int k = 0; k < TEST_ATTEMPTS; ++k) {
        Rotation3 a = rot3rng.uniform();
        Rotation3 b = rot3rng.uniform();
        real3 x = r3rng.normal();
        ASSERT_EQ(lin3_apply_pos(rot3_to_linear(a), x), approx(rot3_apply_pos(a, x)));
        ASSERT_EQ(
            lin3_chain(rot3_to_linear(a), rot3_to_linear(b)),
            approx(rot3_to_linear(rot3_chain(a, b)))
        );
    }
}

#endif // TEST_UNIT
