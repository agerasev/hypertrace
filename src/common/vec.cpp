#include "vec.hpp"


#ifdef UNIT_TEST
#include <catch.hpp>

TEST_CASE("Vector types", "[vec]") {
    SECTION("Field alignment") {
        vec<int, 2> a2;
        a2[0] = 123456;
        REQUIRE(a2.x == 123456);
        a2.y = 654321;
        REQUIRE(a2[1] == 654321);

        vec<int, 4> a4;
        a4.yz = vec<int, 2>(1, 2);
        REQUIRE(a4[1] == 1);
        REQUIRE(a4[2] == 2);
    }
    SECTION("Contruction") {
        auto v = vec<int, 4>(vec<int, 2>(0, 1), 2, 3);
        for (int i = 0; i < 4; ++i) {
            REQUIRE(v[i] == i);
        }
    }
};

#endif // UNIT_TEST
