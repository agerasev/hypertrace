#include "tuple.hpp"


#ifdef UNIT_TEST
#include <catch.hpp>
#include <test.hpp>

using namespace test;

TEST_CASE("Tuple", "[tuple]") {
    Rng rng(0x807A);

    SECTION("Union") {
        Tuple<bool, int, double> a;

        REQUIRE(a.size() == 3);
        a.elem<0>() = true;
        a.elem<1>() = 1;
        a.elem<2>() = 3.1415;
        REQUIRE(a.elem<0>() == true);
        REQUIRE(a.elem<1>() == 1);
        REQUIRE(a.elem<2>() == approx(3.1415));
    }
}

#endif
