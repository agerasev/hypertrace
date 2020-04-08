#include "complex.hpp"

/*
real c_fabs(complex a) {
    return fabs(a.x) + fabs(a.y);
}
*/

#ifdef UNIT_TEST
#include <catch.hpp>

#include <iostream>

comp CompTestRng::complex_normal() {
    return comp(normal(), normal());
}
comp CompTestRng::complex_unit() {
    real phi = 2*PI*uniform();
    return comp(cos(phi), sin(phi));
}
comp CompTestRng::complex_nonzero() {
    comp a;
    do {
        a = complex_normal();
    } while(abs2(a) < EPS);
    return a;
}

TEST_CASE("Complex numbers", "[complex]") {
    CompTestRng rng(0xbeef);

    SECTION("Constructor") {
        comp a(0.0, 1.0);
        REQUIRE(a.re() == Approx(0.0));
        REQUIRE(a.im() == Approx(1.0));
    }

    SECTION("Inversion") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            comp a = rng.complex_nonzero();
            REQUIRE(a/a == approx(1));
        }
    }
};
#endif // UNIT_TEST
