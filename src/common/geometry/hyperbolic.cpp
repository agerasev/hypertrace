#include "hyperbolic.hpp"


real Hy::distance(quat a, quat b) {
    real x = 1 + length2(a - b)/(2*a[2]*b[2]);
    return math::log(x + math::sqrt(x*x - 1));
}

quat Hy::dir_at(quat src_pos, quat src_dir, quat dst_pos) {
    quat p = src_pos, d = src_dir, h = dst_pos;
    return quat(
        h[2]/p[2]*d[0],
        h[2]/p[2]*d[1],
        d[2] - length(p.re() - h.re())/p[2]*length(d.re()),
        (real)0
    );
}

Moebius Hy::zshift(real l) {
    real l2 = l/2;
    real e = math::exp(l2);
    return Moebius(e, 0, 0, 1/e);
}

Moebius Hy::xshift(real l) {
    real l2 = l/2;
    real c = math::cosh(l2), s = math::sinh(l2);
    return Moebius(c, s, s, c);
}

Moebius Hy::yshift(real l) {
    real l2 = l/(real)2;
    real c = math::cosh(l2), s = math::sinh(l2);
    return Moebius(c, s*1_i, -s*1_i, c);
}

Moebius Hy::zrotate(real phi) {
    real c = math::cos(phi/2), s = math::sin(phi/2);
    return Moebius(comp(c, s), 0, 0, comp(c, -s));
}

Moebius Hy::xrotate(real theta) {
    real c = math::cos(theta/2), s = math::sin(theta/2);
    return Moebius(c, -s, s, c);
}

Moebius Hy::yrotate(real theta) {
    real c = math::cos(theta/2), s = math::sin(theta/2);
    return Moebius(c, s*1_i, s*1_i, c);
}

Moebius Hy::horosphere(comp pos) {
    return Moebius(1, pos, 0, 1);
}

Moebius Hy::look_to(quat dir) {
	// We look at the top (along the z axis).
	real phi = -math::atan2(dir[1], dir[0]);
	real theta = -math::atan2(length(dir.re()), dir[2]);
	return xrotate(theta)*zrotate(phi);
}

Moebius Hy::look_at(quat pos) {
    // The origin is at *j* (z = 1).
	real phi = -math::atan2(pos[1], pos[0]);
	real theta = -math::atan2(2*length(pos.re()), length2(pos) - 1);
	return xrotate(theta)*zrotate(phi);
}

Moebius Hy::move_at(quat pos) {
    Moebius a = look_at(pos);
    Moebius b = zshift(-distance(1_j, pos));
    return !a*b*a;
}

Moebius Hy::move_to(quat dir, real dist) {
    Moebius a = look_to(dir);
    Moebius b = zshift(-dist);
    return !a*b*a;
}


#ifdef UNIT_TEST
#include <catch.hpp>
#include <functional>
#include <test.hpp>

using namespace test;

quat rand_pos(test::Rng &rng) {
    return quat(rng.d<comp>().normal(), math::exp(rng.d<real>().normal()), 0);
}

TEST_CASE("Hyperbolic geometry", "[hyperbolic]") {
    test::Rng rng;

    SECTION("Distance invariance") {
        std::vector<std::function<Moebius(Rng &)>> elem = {
            [](Rng &rng) { return Hy::xrotate(2*PI*rng.d<real>().uniform()); },
            [](Rng &rng) { return Hy::zrotate(2*PI*rng.d<real>().uniform()); },
            [](Rng &rng) { return Hy::zshift(rng.d<real>().normal()); }
        };

        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            quat a = rand_pos(rng), b = rand_pos(rng);

            Moebius m = Moebius::identity();
            for (int j = 0; j < 8; ++j) {
                m = m*elem[math::floor(3*rng.d<real>().uniform())](rng);
            }

            real dist_before = Hy::distance(a, b);
            real dist_after = Hy::distance(m.apply(a), m.apply(b));

            REQUIRE(dist_before == approx(dist_after));
        }
    }
    SECTION("Rotation of derivative") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            quat q = normalize(quat(rng.d<comp>().normal(), 1, 0));
            real phi = -math::atan2(q[1], q[0]);
            real theta = -math::atan2(length(q.re()), q[2]);

            Moebius c = Hy::xrotate(theta)*Hy::zrotate(phi);

            REQUIRE(c.deriv(1_j, q) == approx(1_j));
        }
    }
    SECTION("Look at the point") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            quat q = rand_pos(rng);
            quat p = Hy::look_at(q).apply(q);

            REQUIRE(p.re() == approx(0));
        }
    }
    SECTION("Move at the point") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            quat p = rand_pos(rng), q = rand_pos(rng);

            Moebius a = Hy::move_at(p);
            REQUIRE(a.apply(p) == approx(1_j));

            Moebius b = !Hy::move_at(q)*a;
            REQUIRE(b.apply(p) == approx(q));
        }
    }
    SECTION("Rotation interpolation at small angles") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Moebius m = Hy::xrotate(1e-3*PI*rng.d<real>().uniform());
            int q = (int)math::floor(8*rng.d<real>().uniform()) + 2;

            Moebius l = Moebius::identity();
            for (int i = 0; i < q; ++i) {
                l *= m;
            }
            Moebius o = pow(l, 1.0/q);
            REQUIRE(det(o.mat()) == approx(1));
            REQUIRE(o.mat() == approx(m.mat()));
        }
    }
    SECTION("Interpolation") {
        Moebius a = Hy::xshift(1.0);
        Moebius b = Hy::yshift(1.0);
        quat aq = a.apply(1_j), bq = b.apply(1_j);
        real d = Hy::distance(aq, bq);
        int n = 10;
        for (int i = 0; i < n; ++i) {
            real t = real(i)/(n - 1);
            Moebius c = a*pow(!a*b, t);
            quat cq = c.apply(1_j);
            REQUIRE(Hy::distance(aq, cq)/d == approx(t).epsilon(0.01));
        }
    }
};
#endif // UNIT_TEST
