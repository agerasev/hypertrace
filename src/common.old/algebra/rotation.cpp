#include "rotation.hpp"


#ifdef TEST_UNIT
#include <catch.hpp>

using namespace test;

TEST_CASE("Rotation transformation", "[rotation]") {
    Rng rng(0x807A);

    SECTION("Rotation 2D") {
        SECTION("Mapping") {
            for (int k = 0; k < TEST_ATTEMPTS; ++k) {
                real angle = 2*PI*rng.d<real>().uniform();
                Rotation2 r(angle);
                real2 a = rng.d<real2>().normal();
                real2 b = r.apply(a);
                REQUIRE(length(a) == approx(length(b)));
                REQUIRE(dot(a, b)/length2(a) == approx(math::cos(angle)));
            }
        }
        SECTION("Chaining") {
            for (int k = 0; k < TEST_ATTEMPTS; ++k) {
                Rotation2 a = rng.d<Rotation2>().some();
                Rotation2 b = rng.d<Rotation2>().some();
                real2 v = rng.d<real2>().normal();
                REQUIRE((a*b).apply(v) == approx(a.apply(b.apply(v))));
            }
        }
        SECTION("Inversion") {
            for (int k = 0; k < TEST_ATTEMPTS; ++k) {
                Rotation2 r = rng.d<Rotation2>().some();
                REQUIRE((r*!r).comp() == approx(Rotation2::identity().comp()));
            }
        }
        SECTION("To linear") {
            for (int k = 0; k < TEST_ATTEMPTS; ++k) {
                Rotation2 a = rng.d<Rotation2>().some();
                Rotation2 b = rng.d<Rotation2>().some();
                real2 x = rng.d<real2>().normal();
                REQUIRE(a.to_linear().apply(x) == approx(a.apply(x)));
                REQUIRE((a.to_linear()*b.to_linear()).mat() == approx((a*b).to_linear().mat()));
            }
        }
    }
    SECTION("Rotation 3D") {
        SECTION("Mapping") {
            for (int k = 0; k < TEST_ATTEMPTS; ++k) {
                real3 axis = rng.d<real3>().unit();
                real angle = 2*PI*rng.d<real>().uniform();
                Rotation3 r(axis, angle);
                real3 a = rng.d<real3>().normal();
                real3 b = r.apply(a);
                REQUIRE(length(a) == approx(length(b)));
                a -= dot(a, axis)*axis;
                b -= dot(b, axis)*axis;
                real aa = length2(a);
                REQUIRE(dot(a, b)/aa == approx(math::cos(angle)));
                REQUIRE(cross(a, b)/aa == approx(axis*math::sin(angle)));
            }
        }
        SECTION("Chaining") {
            for (int k = 0; k < TEST_ATTEMPTS; ++k) {
                Rotation3 a = rng.d<Rotation3>().some();
                Rotation3 b = rng.d<Rotation3>().some();
                real3 v = rng.d<real3>().normal();
                REQUIRE((a*b).apply(v) == approx(a.apply(b.apply(v))));
            }
        }
        SECTION("Inversion") {
            for (int k = 0; k < TEST_ATTEMPTS; ++k) {
                Rotation3 r = rng.d<Rotation3>().some();
                REQUIRE((r*!r).quat() == approx(Rotation3::identity().quat()));
            }
        }
        SECTION("To linear") {
            for (int k = 0; k < TEST_ATTEMPTS; ++k) {
                Rotation3 a = rng.d<Rotation3>().some();
                Rotation3 b = rng.d<Rotation3>().some();
                real3 x = rng.d<real3>().normal();
                REQUIRE(a.to_linear().apply(x) == approx(a.apply(x)));
                REQUIRE((a.to_linear()*b.to_linear()).mat() == approx((a*b).to_linear().mat()));
            }
        }
    }
}

#endif

