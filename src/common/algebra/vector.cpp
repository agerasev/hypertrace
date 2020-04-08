#include "vector.hpp"


#ifdef UNIT_TEST
#include <catch.hpp>

TEST_CASE("Vector types", "[vector]") {
    SECTION("Contruction") {
        auto a = vector<int, 3>(-1);
        for (int i = 0; i < a.size(); ++i) {
            REQUIRE(a[i] == -1);
        }
        auto b = vector<int, 4>(0, 1, 2, 3);
        for (int i = 0; i < b.size(); ++i) {
            REQUIRE(b[i] == i);
        }
        /*
        auto c = vector<int, 5>(vector<int, 3>(0, vector<int, 2>(1, 2)), 3, 4);
        for (int i = 0; i < c.size(); ++i) {
            REQUIRE(c[i] == i);
        }
        */
    }
    SECTION("Compare") {
        REQUIRE(real4(1, 2, 3, 4) == approx(real4(1, 2, 3, 4)));
    }
};

#endif // UNIT_TEST
