#include "differential.hpp"



#ifdef UNIT_TEST
#include <catch.hpp>

using namespace test;

TEST_CASE("Differential operators", "[differential]") {
    Rng rng(0xDEAF);

    SECTION("Gradient") {
        auto func = [](real3 p) { return length(p); };
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            real3 p = rng.d<real3>().normal();
            REQUIRE(gradient(func, p) == approx(normalize(p)));
        }
    }
}

#endif
