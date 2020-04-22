#include "euclidean.hpp"
#include <algebra/rotation.hpp>


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

Affine<real, 3> Eu::shift(real3 pos) {
    return Affine<real, 3>(Linear<real, 3>::identity(), pos);
}

Affine<real, 3> Eu::rotate(real3 axis, real angle) {
    return Affine<real, 3>(Rotation<real, 3>(axis, angle).to_linear());
}

Affine<real, 3> Eu::look_to(real3 dir) {
	return Affine<real, 3>(Linear<real, 3>::look_to(dir));
}

Affine<real, 3> Eu::look_at(real3 pos) {
    return look_to(normalize(pos));
}

Affine<real, 3> Eu::move_at(real3 pos) {
    return Affine<real, 3>(Linear<real, 3>::identity(), -pos);
}

Affine<real, 3> Eu::move_to(real3 dir, real dist) {
    return move_at(dir*dist);
}


#ifdef UNIT_TEST
#include <catch.hpp>
#include <functional>
#include <test.hpp>

using namespace test;

static real3 rand_pos(test::Rng &rng) {
    return rng.d<real3>().normal();
}

TEST_CASE("Euclidean geometry", "[euclidean]") {
    test::Rng rng;

    SECTION("Distance invariance") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            real3 a = rand_pos(rng), b = rand_pos(rng);

            Affine<real, 3> m(
                rng.d<Rotation3>().some().to_linear(),
                rng.d<real3>().normal()
            );

            real dist_before = Eu::distance(a, b);
            real dist_after = Eu::distance(m.apply(a), m.apply(b));

            REQUIRE(dist_before == approx(dist_after));
        }
    }
    SECTION("Look at the point") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            real3 q = rand_pos(rng);
            real3 p = Eu::look_at(q).apply(q);

            REQUIRE(p.shuffle(0,1) == approx(real2(0,0)));
        }
    }
    SECTION("Move at the point") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            real3 p = rand_pos(rng), q = rand_pos(rng);

            Affine<real, 3> a = Eu::move_at(p);
            REQUIRE(a.apply(p) == approx(Eu::origin()));

            Affine<real, 3> b = !Eu::move_at(q)*a;
            REQUIRE(b.apply(p) == approx(q));
        }
    }
};
#endif // UNIT_TEST
