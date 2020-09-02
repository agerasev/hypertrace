#include "option.hpp"

using namespace core;

#ifdef TEST_UNIT

#include "container/tuple.hpp"
#include <catch.hpp>
#include <memory>

TEST_CASE("Option", "[option]") {
    SECTION("Create") {
        auto a = Option<int>::Some(123);
        REQUIRE(a.is_some());
        REQUIRE(a.unwrap() == 123);

        auto b = Option<int>::None();
        REQUIRE(b.is_none());
    }
    SECTION("Move") {
        std::unique_ptr<int> ptr = std::make_unique<int>(123);
        auto a = Option<std::unique_ptr<int>>::Some(std::move(ptr));
        REQUIRE(*a.get() == 123);
        ptr = a.unwrap();
        REQUIRE(*ptr == 123);
    }
    SECTION("Destroy") {
        std::shared_ptr<int> ptr = std::make_shared<int>(123);
        REQUIRE(ptr.use_count() == 1);
        auto a = Option<std::shared_ptr<int>>::Some(ptr);
        REQUIRE(ptr.use_count() == 2);
        REQUIRE(*a.get() == 123);
        REQUIRE(ptr.use_count() == 2);
        {
            std::shared_ptr<int> cptr = a.unwrap();
            REQUIRE(*cptr == 123);
            REQUIRE(ptr.use_count() == 2);
        }
        REQUIRE(ptr.use_count() == 1);
    }
    SECTION("Tuple of non-copy option") {
        std::unique_ptr<int> ptr = std::make_unique<int>(123);
        Tuple<Option<std::unique_ptr<int>>, int> a(
            Option<std::unique_ptr<int>>::Some(std::move(ptr)),
            321
        );
        REQUIRE(*a.get<0>().get() == 123);
        REQUIRE(a.get<1>() == 321);
        //ptr = a.get<0>().unwrap();
        //REQUIRE(*ptr == 123);
    }
}

#endif // TEST_UNIT
