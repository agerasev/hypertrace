#include "euclidean.hh"
#include <transform/rotation.hh>


EuPos eu_origin() {
    return MAKE(real3)(R0);
}
EuDir eu_default_dir() {
    return MAKE(real3)(R0, R0, -R1);
}

real eu_length(EuPos a) {
    return length(a);
}
real eu_distance(EuPos a, EuPos b) {
    return distance(a, b);
}

// Returns the direction of the line at point `dst_pos`
// when we know that the line at the point `src_pos` has direction of `src_dir`.
_ALLOW_UNUSED_PARAMETERS_
EuDir eu_dir_at(EuPos src_pos, EuDir src_dir, EuPos dst_pos) {
    return src_dir;
}

EuMap eu_shift(EuDir pos) {
    return EuMap { -pos, rot3_identity() };
}
EuMap eu_rotate(EuDir axis, real phi) {
    return EuMap { shf3_identity(), rot3_from_axis(axis, phi) };
}

// Turns direction `dir` to *z*.
EuMap eu_look_to(EuDir dir) {
    /// FIXME: Implement look at for Rotation
    return hom3_identity(); //lin3_look_to(dir);
}

// Rotates point `pos` around the origin to make it lay on the z axis.
EuMap eu_look_at(EuPos pos) {
    return eu_look_to(normalize(pos));
}

// Translates point `pos` to the origin preserving orientation
// relative to the line that connects `pos` to the origin.
EuMap eu_move_at(EuPos pos) {
    return EuMap { pos, rot3_identity() };
}
EuMap eu_move_to(EuDir dir, real dist) {
    return eu_move_at(dir * dist);
}

EuMap eu_identity() {
    return hom3_identity();
}
EuPos eu_apply_pos(EuMap m, EuPos p) {
    return hom3_apply_pos(m, p);
}
EuDir eu_apply_dir(EuMap m, EuPos p, EuDir d) {
    return hom3_apply_dir(m, p, d);
}
EuDir eu_apply_normal(EuMap m, EuPos p, EuDir d) {
    return hom3_apply_normal(m, p, d);
}
EuMap eu_chain(EuMap a, EuMap b) {
    return hom3_chain(a, b);
}
EuMap eu_inverse(EuMap m) {
    return hom3_inverse(m);
}

#ifdef UNITTEST

#include <gtest/gtest.h>

class EuclideanTest : public testing::Test {
protected:
    TestRng<real3> vrng = TestRng<real3>();
    TestRngRotation3 rot3rng = TestRngRotation3();
};

TEST_F(EuclideanTest, distance_invariance) {
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        real3 a = vrng.normal(), b = vrng.normal();

        Homogenous3 m = {
            vrng.normal(),
            rot3rng.uniform(),
        };

        real dist_before = eu_distance(a, b);
        real dist_after = eu_distance(
            eu_apply_pos(m, a),
            eu_apply_pos(m, b)
        );

        ASSERT_EQ(dist_before, approx(dist_after));
    }
}
TEST_F(EuclideanTest, look_at_the_point) {
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        real3 q = vrng.normal();
        real3 p = eu_apply_pos(eu_look_at(q), q);

        ASSERT_EQ(p.xy, approx(r2_new(R0, R0)));
    }
}
TEST_F(EuclideanTest, move_at_the_point) {
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        real3 p = vrng.normal(), q = vrng.normal();

        EuMap a = eu_move_at(p);
        ASSERT_EQ(eu_apply_pos(a, p), approx(eu_origin()));

        EuMap b = eu_chain(eu_inverse(eu_move_at(q)), a);
        ASSERT_EQ(eu_apply_pos(b, p), approx(q));
    }
}

#endif // UNITTEST
