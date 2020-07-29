#include "vector.hh"


#ifdef UNIT_TEST
#include <catch.hpp>

TEST_CASE("Vector types", "[vec]") {
    SECTION("Field alignment") {
        vector<int, 2> a2;
        a2[0] = 123456;
        REQUIRE(a2.x == 123456);
        a2.y = 654321;
        REQUIRE(a2[1] == 654321);

        vector<int, 4> a4;
        a4.yz = vector<int, 2>(1, 2);
        REQUIRE(a4[1] == 1);
        REQUIRE(a4[2] == 2);
    }
    SECTION("Contruction") {
        auto a = MAKE(int3)(-1);
        for (int i = 0; i < a.SIZE; ++i) {
            REQUIRE(a[i] == -1);
        }
        auto b = MAKE(int4)(0, 1, 2, 3);
        for (int i = 0; i < b.SIZE; ++i) {
            REQUIRE(b[i] == i);
        }
        auto c = int4(int3(0, int2(1, 2)), 3);
        for (int i = 0; i < c.SIZE; ++i) {
            REQUIRE(c[i] == i);
        }
    }
    SECTION("Compare") {
        REQUIRE(real4(1, 2, 3, 4) == approx(real4(1, 2, 3, 4)));
    }
    SECTION("Norm") {
        REQUIRE(length(real2(3, -4)) == approx(5));
    }
    SECTION("Abs") {
        REQUIRE(fabs(real4(1, -2, 3, -4)) == approx(real4(1, 2, 3, 4)));
    }
    SECTION("Fract") {
        real4 i;
        real4 p = fract(real4(1.1, -1.8, 3.3, -3.6), &i);
        REQUIRE(p == approx(real4(0.1, 0.2, 0.3, 0.4)));
        REQUIRE(i == approx(real4(1, -2, 3, -4)));
    }
};

#endif // UNIT_TEST
