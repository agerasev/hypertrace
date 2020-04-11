#include "moebius.hpp"

#ifdef UNIT_TEST
#include <catch.hpp>
#include "test.hpp"

using namespace test;

TEST_CASE("Moebius transformation", "[moebius]") {
    Rng rng;

    SECTION("Complex coefficients") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Moebius<comp> m = rng.distrib<Moebius<comp>>().normal();
            real a = rng.distrib<real>().normal();
            comp b = rng.distrib<comp>().normal();
            quat c = rng.distrib<quat>().normal();

            REQUIRE(m.apply(a) == approx((m.a()*a + m.b())/(m.c()*a + m.d())));
            REQUIRE(m.apply(b) == approx((m.a()*b + m.b())/(m.c()*b + m.d())));
            REQUIRE(m.apply(c) == approx((m.a()*c + m.b())/(m.c()*c + m.d())));
        }
    }
    SECTION("Chaining") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Moebius<comp> a(rng.distrib<comp2x2>().normalized());
            Moebius<comp> b(rng.distrib<comp2x2>().normalized());
            quat c = rng.distrib<quat>().normal();

            REQUIRE((a*b).apply(c) == approx(a.apply(b.apply(c))));
        }
    }
    
    SECTION("Complex derivation") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Moebius<comp> a = rng.distrib<Moebius<comp>>().normal();
            comp p = rng.distrib<comp>().normal();
            comp v = rng.distrib<comp>().nonzero();
            
            REQUIRE(a.deriv(p) == approx((a.apply(p + EPS*v) - a.apply(p))/(EPS*v)));
        }
    }
    SECTION("Quaternion directional derivation") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Moebius<comp> a = rng.distrib<Moebius<comp>>().normal();
            quat p = rng.distrib<quat>().normal();
            quat v = rng.distrib<quat>().nonzero();
            
            REQUIRE(a.deriv(p, v) == approx((a.apply(p + EPS*v) - a.apply(p))/EPS));
        }
    }
};
#endif // UNIT_TEST
