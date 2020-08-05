#include "euclidean.hh"
#include <common/algebra/rotation.hh>


eu_pos eu_origin() {
    return real3(0);
}

real eu_length(eu_pos a) {
    return length(a);
}
real eu_distance(eu_pos a, eu_pos b) {
    return length(a - b);
}

eu_pos eu_apply_pos(EuMap m, eu_pos p) {
    return aff3_apply_pos(m, p);
}
eu_dir eu_apply_dir(EuMap m, eu_pos, eu_dir d) {
    return aff3_apply_dir(m, d);
}

// Returns the direction of the line at point `dst_pos`
// when we know that the line at the point `src_pos` has direction of `src_dir`.
eu_dir eu_dir_at(eu_pos, eu_dir src_dir, eu_pos) {
    return src_dir;
}

EuMap eu_shift(eu_dir pos) {
    return aff3_from_ls(lin3_identity(), pos);
}
EuMap eu_rotate(eu_dir axis, real phi) {
    return rot3_to_linear(rot3_from_axis(axis, phi));
}

// Turns direction `dir` to *z*.
EuMap eu_look_to(eu_dir dir) {
    return lin3_look_to(dir);
}

// Rotates point `pos` around the origin to make it lay on the z axis.
EuMap eu_look_at(eu_pos pos) {
    return eu_look_to(normalize(pos));
}

// Translates point `pos` to the origin preserving orientation
// relative to the line that connects `pos` to the origin.
EuMap eu_move_at(eu_pos pos) {
    return aff3_from_ls(lin3_identity(), -pos);
}
EuMap eu_move_to(eu_dir dir, real dist) {
    return eu_move_at(dir*dist);
}


#ifdef UNIT_TEST

#include <catch.hpp>


TEST_CASE("Euclidean geometry", "[euclidean]") {
    TestRng<real3> vrng;
    TestRngRotation3 rot3rng;

    SECTION("Distance invariance") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            real3 a = vrng.normal(), b = vrng.normal();

            Affine3 m = aff3_from_ls(
                rot3_to_linear(rot3rng.uniform()),
                vrng.normal()
            );

            real dist_before = eu_distance(a, b);
            real dist_after = eu_distance(
                eu_apply_pos(m, a),
                eu_apply_pos(m, b)
            );

            REQUIRE(dist_before == approx(dist_after));
        }
    }
    SECTION("Look at the point") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            real3 q = vrng.normal();
            real3 p = eu_apply_pos(eu_look_at(q), q);

            REQUIRE(p.xy == approx(r2_new(R0, R0)));
        }
    }
    SECTION("Move at the point") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            real3 p = vrng.normal(), q = vrng.normal();

            Affine3 a = eu_move_at(p);
            REQUIRE(eu_apply_pos(a, p) == approx(eu_origin()));

            Affine3 b = eu_chain(eu_inverse(eu_move_at(q)), a);
            REQUIRE(eu_apply_pos(b, p) == approx(q));
        }
    }
};

#endif // UNIT_TEST
