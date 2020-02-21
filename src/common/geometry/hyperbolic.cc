#include "hyperbolic.hh"

#ifndef OPENCL
#include <math.h>
#endif // OPENCL


real hy_distance(quaternion a, quaternion b) {
    real x = (real)1 + q_abs2(a - b)/((real)2*a.z*b.z);
    return log(x + sqrt(x*x - 1));
}

// Returns the direction of the line at point `dst_pos`
// when we know that the line at the point `src_pos` has direction of `src_dir`.
quaternion hy_dir_at(quaternion src_pos, quaternion src_dir, quaternion dst_pos) {
    quaternion p = src_pos, d = src_dir, h = dst_pos;
    return q_new(
        h.z/p.z*d.x,
        h.z/p.z*d.y,
        d.z - length(p.xy - h.xy)/p.z*length(d.xy),
        (real)0
    );
}

Moebius hy_zshift(real l) {
    real l2 = l/(real)2;
    real e = exp(l2);
    return mo_new(C1*e, C0, C0, C1/e);
}

Moebius hy_xshift(real l) {
    real l2 = l/(real)2;
    real c = cosh(l2), s = sinh(l2);
    return mo_new(c*C1, s*C1, s*C1, c*C1);
}

Moebius hy_yshift(real l) {
    real l2 = l/(real)2;
    real c = cosh(l2), s = sinh(l2);
    return mo_new(c*C1, s*CI, -s*CI, c*C1);
}

Moebius hy_zrotate(real phi) {
    real c = cos(phi/(real)2), s = sin(phi/(real)2);
    return mo_new(c_new(c, s), C0, C0, c_new(c, -s));
}

Moebius hy_xrotate(real theta) {
    real c = cos(theta/(real)2), s = sin(theta/(real)2);
    return mo_new(c*C1, -s*C1, s*C1, c*C1);
}

Moebius hy_yrotate(real theta) {
    real c = cos(theta/(real)2), s = sin(theta/(real)2);
    return mo_new(c*C1, s*CI, s*CI, c*C1);
}

// Turns direction `dir` to *j*.
Moebius hy_look_to(quaternion dir) {
	// We look at the top (along the z axis).
	real phi = -atan2(dir.y, dir.x);
	real theta = -atan2(length(dir.xy), dir.z);
	return mo_chain(hy_xrotate(theta), hy_zrotate(phi));
}

// Rotatates point `pos` around the origin to make it lay on the z axis.
Moebius hy_look_at(quaternion pos) {
    // The origin is at *j* (z = 1).
	real phi = -atan2(pos.y, pos.x);
	real theta = -atan2((real)2*length(pos.xy), q_abs2(pos) - (real)1);
	return mo_chain(hy_xrotate(theta), hy_zrotate(phi));
}

// Translates point `pos` to the origin preserving orientation
// relative to the line that connects `pos` to the origin.
Moebius hy_move_at(quaternion pos) {
    Moebius a = hy_look_at(pos);
    Moebius b = hy_zshift(-hy_distance(QJ, pos));
    return mo_chain(mo_inverse(a), mo_chain(b, a));
}


#ifdef UNIT_TEST
#include <catch.hpp>

#include <functional>

quaternion rand_hy_pos(Rng &rng) {
    return q_new(rand_c_normal(rng), exp(rng.normal()), 0);
}

TEST_CASE("Hyperbolic geometry", "[hyperbolic]") {
    Rng rng;

    SECTION("Distance invariance") {
        std::vector<std::function<Moebius(Rng &)>> elem = {
            [](Rng &rng) { return hy_xrotate(2*PI*rng.uniform()); },
            [](Rng &rng) { return hy_zrotate(2*PI*rng.uniform()); },
            [](Rng &rng) { return hy_zshift(rng.normal()); }
        };

        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            quaternion a = rand_hy_pos(rng), b = rand_hy_pos(rng);

            Moebius m = mo_identity();
            for (int j = 0; j < 8; ++j) {
                m = mo_chain(m, elem[floor(3*rng.uniform())](rng));
            }

            real dist_before = hy_distance(a, b);
            real dist_after = hy_distance(mo_apply(m, a), mo_apply(m, b));

            REQUIRE(dist_before == Approx(dist_after));
        }
    }
    SECTION("Rotation of derivative") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            quaternion q = normalize(q_new(rand_c_normal(rng), 1, 0));
            real phi = -atan2(q.y, q.x);
            real theta = -atan2(length(q.xy), q.z);

            Moebius c = mo_chain(hy_xrotate(theta), hy_zrotate(phi));

            REQUIRE(mo_deriv(c, QJ, q) == ApproxV(QJ));
        }
    }
    SECTION("Look at the point") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            quaternion q = rand_hy_pos(rng);
            quaternion p = mo_apply(hy_look_at(q), q);

            REQUIRE(p.xy == ApproxV(C0));
        }
    }
    SECTION("Move at the point") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            quaternion p = rand_hy_pos(rng), q = rand_hy_pos(rng);

            Moebius a = hy_move_at(p);
            REQUIRE(mo_apply(a, p) == ApproxV(QJ));

            Moebius b = mo_chain(mo_inverse(hy_move_at(q)), a);
            REQUIRE(mo_apply(b, p) == ApproxV(q));
        }
    }
};
#endif // UNIT_TEST
