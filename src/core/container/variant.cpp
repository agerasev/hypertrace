#include "variant.hpp"

using namespace core;

#ifdef TEST_UNIT

#include <catch.hpp>
#include <memory>
#include "tuple.hpp"


TEST_CASE("Variant", "[variant]") {
    SECTION("Primitive") {
        auto a = Variant<bool, int, double>::create<1>(123);
        REQUIRE(a.size() == 3);

        REQUIRE(a.get<1>() == 123);
        a.get<1>() = -321;
        REQUIRE(a.take<1>() == -321);

        a.put<2>(3.1415);
        REQUIRE(a.get<2>() == Approx(3.1415));
        a.get<2>() = -2.71;
        REQUIRE(a.take<2>() == -2.71);
    }
    SECTION("Move") {
        std::unique_ptr<int> ptr = std::make_unique<int>(123);
        auto a = Variant<Tuple<>, std::unique_ptr<int>>::create<1>(std::move(ptr));
        //auto b = a;
        REQUIRE(*a.get<1>() == 123);
        ptr = a.take<1>();
        REQUIRE(*ptr == 123);
    }
    SECTION("Ctor Dtor") {
        std::shared_ptr<int> ptr = std::make_shared<int>(123);
        REQUIRE(ptr.use_count() == 1);
        {
            auto a = Variant<Tuple<>, std::shared_ptr<int>>::create<1>(ptr);
            REQUIRE(ptr.use_count() == 2);
            REQUIRE(*a.get<1>() == 123);
            REQUIRE(ptr.use_count() == 2);
        }
        REQUIRE(ptr.use_count() == 1);
    }
}

#endif
