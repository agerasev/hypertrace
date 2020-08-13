#include "tuple.hpp"

#ifdef TEST_UNIT

#include <catch.hpp>

TEST_CASE("Tuple", "[tuple]") {
    SECTION("Getters") {
        Tuple<bool, int, double> a(true, 1, 3.1415);
        REQUIRE(a.size() == 3);

        REQUIRE(a.get<0>() == true);
        REQUIRE(a.get<1>() == 1);
        REQUIRE(a.get<2>() == Approx(3.1415));
    }
}

#endif // TEST_UNIT
