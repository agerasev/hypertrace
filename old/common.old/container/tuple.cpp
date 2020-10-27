#include "tuple.hpp"


#ifdef TEST_UNIT

#include <catch.hpp>

TEST_CASE("Tuple", "[tuple]") {
    SECTION("Union") {
        Tuple<bool, int, double> a;

        REQUIRE(a.size() == 3);
        a.elem<0>() = true;
        a.elem<1>() = 1;
        a.elem<2>() = 3.1415;
        REQUIRE(a.elem<0>() == true);
        REQUIRE(a.elem<1>() == 1);
        REQUIRE(a.elem<2>() == Approx(3.1415));
    }
}

#endif
