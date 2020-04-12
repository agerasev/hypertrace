#include "moebius.hpp"

#ifdef UNIT_TEST
#include <catch.hpp>
#include "test.hpp"

using namespace test;

TEST_CASE("Moebius transformation", "[moebius]") {
    Rng rng;

    SECTION("Complex coefficients") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Moebius m = rng.d<Moebius>().some();
            real a = rng.d<real>().normal();
            comp b = rng.d<comp>().normal();
            quat c = rng.d<quat>().normal();

            REQUIRE(m.apply(a) == approx((m.a()*a + m.b())/(m.c()*a + m.d())));
            REQUIRE(m.apply(b) == approx((m.a()*b + m.b())/(m.c()*b + m.d())));
            REQUIRE(m.apply(c) == approx((m.a()*c + m.b())/(m.c()*c + m.d())));
        }
    }
    SECTION("Chaining") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Moebius a(rng.d<comp2x2>().normalized());
            Moebius b(rng.d<comp2x2>().normalized());
            quat c = rng.d<quat>().normal();

            REQUIRE((a*b).apply(c) == approx(a.apply(b.apply(c))));
        }
    }
    
    SECTION("Complex derivation") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Moebius a = rng.d<Moebius>().some();
            comp p = rng.d<comp>().normal();
            comp v = rng.d<comp>().nonzero();
            
            REQUIRE(a.deriv(p) == approx((a.apply(p + EPS*v) - a.apply(p))/(EPS*v)));
        }
    }
    SECTION("Quaternion directional derivation") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Moebius a = rng.d<Moebius>().some();
            quat p = rng.d<quat>().normal();
            quat v = rng.d<quat>().nonzero();
            
            REQUIRE(a.deriv(p, v) == approx((a.apply(p + EPS*v) - a.apply(p))/EPS));
        }
    }
};
#endif // UNIT_TEST
