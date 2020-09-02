#include "tuple.hpp"

using namespace core;

#ifdef TEST_UNIT

#include <memory>
#include <catch.hpp>

TEST_CASE("Tuple", "[tuple]") {
    SECTION("Getters") {
        Tuple<bool, int, double> a(true, 1, 3.1415);
        REQUIRE(a.size() == 3);

        REQUIRE(a.get<0>() == true);
        REQUIRE(a.get<1>() == 1);
        REQUIRE(a.get<2>() == Approx(3.1415));
    }
    SECTION("Move") {
        Tuple<std::unique_ptr<int>, int> a(std::make_unique<int>(123), 456);
        REQUIRE(a.size() == 2);
        REQUIRE(*a.get<0>() == 123);
        REQUIRE(a.get<1>() == 456);

        auto b(std::move(a));
        REQUIRE(b.size() == 2);
        REQUIRE(*b.get<0>() == 123);
        REQUIRE(b.get<1>() == 456);
    }
}

#endif // TEST_UNIT
