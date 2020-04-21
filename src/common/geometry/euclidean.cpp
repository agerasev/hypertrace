#include "euclidean.hpp"


real3 Eu::origin() {
    return real3(0);
}

real3 Eu::dir_to_local(real3, real3 dir) {
    return dir;
}
real3 Eu::dir_from_local(real3, real3 dir) {
    return dir;
}

real Eu::length(real3 a) {
    return ::length(a);
}

real Eu::distance(real3 a, real3 b) {
    return ::length(a - b);
}

real3 Eu::dir_at(real3, real3 src_dir, real3) {
    return src_dir;
}

Linear<real, 3> Eu::zshift(real l) {
    real l2 = l/2;
    real e = math::exp(l2);
    return Linear<real, 3>(e, 0, 0, 1/e);
}

Linear<real, 3> Eu::xshift(real l) {
    real l2 = l/2;
    real c = math::cosh(l2), s = math::sinh(l2);
    return Linear<real, 3>(c, s, s, c);
}

Linear<real, 3> Eu::yshift(real l) {
    real l2 = l/(real)2;
    real c = math::cosh(l2), s = math::sinh(l2);
    return Linear<real, 3>(c, s*1_i, -s*1_i, c);
}

Linear<real, 3> Eu::zrotate(real phi) {
    real c = math::cos(phi/2), s = math::sin(phi/2);
    return Linear<real, 3>(comp(c, s), 0, 0, comp(c, -s));
}

Linear<real, 3> Eu::xrotate(real theta) {
    real c = math::cos(theta/2), s = math::sin(theta/2);
    return Linear<real, 3>(c, -s, s, c);
}

Linear<real, 3> Eu::yrotate(real theta) {
    real c = math::cos(theta/2), s = math::sin(theta/2);
    return Linear<real, 3>(c, s*1_i, s*1_i, c);
}

Linear<real, 3> Eu::horosphere(comp pos) {
    return Linear<real, 3>(1, pos, 0, 1);
}

Linear<real, 3> Eu::look_to(real3 dir) {
	// We look at the top (along the z axis).
	real phi = -math::atan2(dir[1], dir[0]);
	real theta = -math::atan2(::length(dir.re()), dir[2]);
	return xrotate(theta)*zrotate(phi);
}

Linear<real, 3> Eu::look_at(real3 pos) {
    // The origin is at *j* (z = 1).
	real phi = -math::atan2(pos[1], pos[0]);
	real theta = -math::atan2(2*::length(pos.re()), ::length2(pos) - 1);
	return xrotate(theta)*zrotate(phi);
}

Linear<real, 3> Eu::move_at(real3 pos) {
    Linear<real, 3> a = look_at(pos);
    Linear<real, 3> b = zshift(-Eu::length(pos));
    return !a*b*a;
}

Linear<real, 3> Eu::move_to(real3 dir, real dist) {
    Linear<real, 3> a = look_to(dir);
    Linear<real, 3> b = zshift(-dist);
    return !a*b*a;
}


#ifdef UNIT_TEST
#include <catch.hpp>
#include <functional>
#include <test.hpp>

using namespace test;

real3 rand_pos(test::Rng &rng) {
    return real3(rng.d<comp>().normal(), math::exp(rng.d<real>().normal()), 0);
}

TEST_CASE("Hyperbolic geometry", "[hyperbolic]") {
    test::Rng rng;

    SECTION("Distance invariance") {
        std::vector<std::function<Linear<real, 3>(Rng &)>> elem = {
            [](Rng &rng) { return Eu::xrotate(2*PI*rng.d<real>().uniform()); },
            [](Rng &rng) { return Eu::zrotate(2*PI*rng.d<real>().uniform()); },
            [](Rng &rng) { return Eu::zshift(rng.d<real>().normal()); }
        };

        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            real3 a = rand_pos(rng), b = rand_pos(rng);

            Linear<real, 3> m = Linear<real, 3>::identity();
            for (int j = 0; j < 8; ++j) {
                m = m*elem[math::floor(3*rng.d<real>().uniform())](rng);
            }

            real dist_before = Eu::distance(a, b);
            real dist_after = Eu::distance(m.apply(a), m.apply(b));

            REQUIRE(dist_before == approx(dist_after));
        }
    }
    SECTION("Rotation of derivative") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            real3 q = normalize(real3(rng.d<comp>().normal(), 1, 0));
            real phi = -math::atan2(q[1], q[0]);
            real theta = -math::atan2(length(q.re()), q[2]);

            Linear<real, 3> c = Eu::xrotate(theta)*Eu::zrotate(phi);

            REQUIRE(c.deriv(1_j, q) == approx(1_j));
        }
    }
    SECTION("Look at the point") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            real3 q = rand_pos(rng);
            real3 p = Eu::look_at(q).apply(q);

            REQUIRE(p.re() == approx(0));
        }
    }
    SECTION("Move at the point") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            real3 p = rand_pos(rng), q = rand_pos(rng);

            Linear<real, 3> a = Eu::move_at(p);
            REQUIRE(a.apply(p) == approx(1_j));

            Linear<real, 3> b = !Eu::move_at(q)*a;
            REQUIRE(b.apply(p) == approx(q));
        }
    }
    SECTION("Rotation interpolation at small angles") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Linear<real, 3> m = Eu::xrotate(1e-3*PI*rng.d<real>().uniform());
            int q = (int)math::floor(8*rng.d<real>().uniform()) + 2;

            Linear<real, 3> l = Linear<real, 3>::identity();
            for (int i = 0; i < q; ++i) {
                l *= m;
            }
            Linear<real, 3> o = pow(l, 1.0/q);
            REQUIRE(det(o.mat()) == approx(1));
            REQUIRE(o.mat() == approx(m.mat()));
        }
    }
    SECTION("Interpolation") {
        Linear<real, 3> a = Eu::xshift(1.0);
        Linear<real, 3> b = Eu::yshift(1.0);
        real3 aq = a.apply(1_j), bq = b.apply(1_j);
        real d = Eu::distance(aq, bq);
        int n = 10;
        for (int i = 0; i < n; ++i) {
            real t = real(i)/(n - 1);
            Linear<real, 3> c = a*pow(!a*b, t);
            real3 cq = c.apply(1_j);
            REQUIRE(Eu::distance(aq, cq)/d == approx(t).epsilon(0.01));
        }
    }
};
#endif // UNIT_TEST
