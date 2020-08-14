#include "prelude.hpp"

#ifdef TEST_UNIT

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
}

TEST_CASE("Result", "[result]") {
    SECTION("Create") {
        auto a = Result<int, float>::Ok(123);
        REQUIRE(a.is_ok());
        REQUIRE(a.unwrap() == 123);

        auto b = Result<int, float>::Err(3.1415f);
        REQUIRE(b.is_err());
        REQUIRE(b.unwrap_err() == Approx(3.1415f));
    }
    SECTION("Move") {
        std::unique_ptr<int> ptr = std::make_unique<int>(123);
        auto a = Result<std::unique_ptr<int>, std::string>::Ok(std::move(ptr));
        REQUIRE(*a.get() == 123);
        ptr = a.unwrap();
        REQUIRE(*ptr == 123);
    }
    SECTION("Destroy") {
        std::shared_ptr<int> ptr = std::make_shared<int>(123);
        REQUIRE(ptr.use_count() == 1);
        auto a = Result<std::shared_ptr<int>, std::string>::Ok(ptr);
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
}

#endif
