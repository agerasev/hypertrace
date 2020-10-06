#include "hyperbolic.hh"


HyDir hy_origin() {
    return QJ;
}
EuDir hy_dir_to_local(HyDir, HyDir dir) {
    return dir.xyz;
}
HyDir hy_dir_from_local(HyDir, EuDir ldir) {
    return q_new(ldir, R0);
}

real hy_length(HyDir a) {
    return hy_distance(a, hy_origin());
}
real hy_distance(HyDir a, HyDir b) {
    real x = 1 + length2(a - b)/(2*a.z*b.z);
    return log(x + sqrt(x*x - 1));
}

HyPos hy_apply_pos(HyMap m, HyPos p) {
    return mo_apply_q(m, p);
}
HyDir hy_apply_dir(HyMap m, HyPos p, HyDir d) {
    return mo_deriv_q(m, p, d);
}

// Returns the direction of the line at point `dst_pos`
// when we know that the line at the point `src_pos` has direction of `src_dir`.
HyDir hy_dir_at(HyDir src_pos, HyDir src_dir, HyDir dst_pos) {
    quat p = src_pos, d = src_dir, h = dst_pos;
    return quat(
        h.z/p.z*d.x,
        h.z/p.z*d.y,
        d.z - length(p.xy - h.xy)/p.z*length(d.xy),
        R0
    );
}

HyMap hy_zshift(real l) {
    real l2 = l/2;
    real e = exp(l2);
    return mo_new(e*C1, C0, C0, C1/e);
}
HyMap hy_xshift(real l) {
    real l2 = l/2;
    real c = cosh(l2), s = sinh(l2);
    return mo_new(c*C1, s*C1, s*C1, c*C1);
}
HyMap hy_yshift(real l) {
    real l2 = l/(real)2;
    real c = cosh(l2), s = sinh(l2);
    return mo_new(c*C1, s*CI, -s*CI, c*C1);
}

HyMap hy_zrotate(real phi) {
    real c = cos(phi/2), s = sin(phi/2);
    return mo_new(c_new(c, s), C0, C0, c_new(c, -s));
}
HyMap hy_xrotate(real theta) {
    real c = cos(theta/2), s = sin(theta/2);
    return mo_new(c*C1, s*CI, s*CI, c*C1);
}
HyMap hy_yrotate(real theta) {
    real c = cos(theta/2), s = sin(theta/2);
    return mo_new(c*C1, -s*C1, s*C1, c*C1);
}

// Move to position at the horosphere.
HyMap hy_horosphere(comp pos) {
    return mo_new(C1, pos, C0, C1);
}

// Turns direction `dir` to *j*.
HyMap hy_look_to(HyDir dir) {
    // We look at the top (along the z axis).
	real phi = -atan2(dir.y, dir.x);
	real theta = -atan2(length(dir.xy), dir.z);
	return hy_chain(hy_yrotate(theta), hy_zrotate(phi));
}

// Rotatates point `pos` around the origin to make it lay on the z axis.
HyMap hy_look_at(HyDir pos) {
    // The origin is at *j* (z = 1).
	real phi = -atan2(pos.y, pos.x);
	real theta = -atan2(2*length(pos.xy), length2(pos) - 1);
	return hy_chain(hy_yrotate(theta), hy_zrotate(phi));
}

// Translates point `pos` to the origin preserving orientation
// relative to the line that connects `pos` to the origin.
HyMap hy_move_at(HyDir pos) {
    Moebius a = hy_look_at(pos);
    Moebius b = hy_zshift(-hy_length(pos));
    return mo_chain(mo_inverse(a), mo_chain(b, a));
}
HyMap hy_move_to(HyDir dir, real dist) {
    Moebius a = hy_look_to(dir);
    Moebius b = hy_zshift(-dist);
    return mo_chain(mo_inverse(a), mo_chain(b, a));
}


#ifdef TEST_CATCH

#include <catch.hpp>
#include <functional>

quat TestRngHyPos::normal() {
    return q_new(rng2.normal(), exp(rng.normal()), R0);
}

TEST_CASE("Hyperbolic geometry", "[hyperbolic]") {
    TestRng<real> rng;
    TestRng<comp> crng;
    TestRngHyPos hyrng;

    SECTION("Distance invariance") {
        std::vector<std::function<Moebius()>> elem = {
            [&]() { return hy_yrotate(2*PI*rng.uniform()); },
            [&]() { return hy_zrotate(2*PI*rng.uniform()); },
            [&]() { return hy_zshift(rng.normal()); }
        };

        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            quat a = hyrng.normal(), b = hyrng.normal();

            Moebius m = mo_identity();
            for (int j = 0; j < 8; ++j) {
                m = hy_chain(m, elem[floor(3*rng.uniform())]());
            }

            real dist_before = hy_distance(a, b);
            real dist_after = hy_distance(hy_apply_pos(m, a), hy_apply_pos(m, b));

            REQUIRE(dist_before == approx(dist_after));
        }
    }
    SECTION("Rotation of derivative") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            quat q = normalize(q_new(crng.normal(), R1, R0));
            real phi = -atan2(q.y, q.x);
            real theta = -atan2(length(q.xy), q.z);

            Moebius c = hy_chain(hy_yrotate(theta), hy_zrotate(phi));

            REQUIRE(hy_apply_dir(c, QJ, q) == approx(QJ));
        }
    }
    SECTION("Look at the point") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            quat q = hyrng.normal();
            quat p = hy_apply_pos(hy_look_at(q), q);

            REQUIRE(p.xy == approx(C0));
        }
    }
    SECTION("Move at the point") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            quat p = hyrng.normal(), q = hyrng.normal();

            Moebius a = hy_move_at(p);
            REQUIRE(hy_apply_pos(a, p) == approx(QJ));

            Moebius b = hy_chain(hy_inverse(hy_move_at(q)), a);
            REQUIRE(hy_apply_pos(b, p) == approx(q));
        }
    }
    SECTION("Rotation interpolation at small angles") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Moebius m = hy_yrotate(1e-3*PI*rng.uniform());
            int q = (int)floor(8*rng.uniform()) + 2;

            Moebius l = mo_identity();
            for (int i = 0; i < q; ++i) {
                l = mo_chain(l, m);
            }
            Moebius o = mo_pow(l, R1/q);
            REQUIRE(c22_det(o) == approx(C1));
            REQUIRE(o == approx(m));
        }
    }
    SECTION("Interpolation") {
        Moebius a = hy_xshift(R1);
        Moebius b = hy_yshift(R1);
        quat aq = hy_apply_pos(a, QJ), bq = hy_apply_pos(b, QJ);
        real d = hy_distance(aq, bq);
        int n = 10;
        for (int i = 0; i < n; ++i) {
            real t = real(i)/(n - 1);
            Moebius c = mo_chain(a, mo_pow(mo_chain(mo_inverse(a), b), t));
            quat cq = hy_apply_pos(c, QJ);
            REQUIRE(hy_distance(aq, cq)/d == approx(t).epsilon(0.01));
        }
    }
};
#endif // TEST_CATCH
