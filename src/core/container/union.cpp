#include "union.hpp"

using namespace core;

#ifdef TEST_UNIT

#include <catch.hpp>
#include <memory>
#include "tuple.hpp"


template <size_t P>
struct TestDispatch {
    static void call(bool mask[]) {
        mask[P] = true;
    }
};

TEST_CASE("Union", "[union]") {
    SECTION("Primitive") {
        auto a = Union<bool, int, double>::create<1>(123);
        REQUIRE(a.size() == 3);

        REQUIRE(a.get<1>() == 123);
        a.get<1>() = -321;
        REQUIRE(a.take<1>() == -321);
    }
    SECTION("Move") {
        std::unique_ptr<int> ptr = std::make_unique<int>(123);
        auto a = Union<Tuple<>, std::unique_ptr<int>>::create<1>(std::move(ptr));
        REQUIRE(*a.get<1>() == 123);
        ptr = a.take<1>();
        REQUIRE(*ptr == 123);
    }
    SECTION("Ctor Dtor") {
        std::shared_ptr<int> ptr = std::make_shared<int>(123);
        REQUIRE(ptr.use_count() == 1);
        auto a = Union<Tuple<>, std::shared_ptr<int>>::create<1>(copy(ptr));
        REQUIRE(ptr.use_count() == 2);
        REQUIRE(*a.get<1>() == 123);
        REQUIRE(ptr.use_count() == 2);
        {
            std::shared_ptr<int> cptr = a.take<1>();
            REQUIRE(*cptr == 123);
            REQUIRE(ptr.use_count() == 2);
        }
        REQUIRE(ptr.use_count() == 1);
    }
    SECTION("Dispatch") {
        bool mask[3] = {false, false, false};
        auto a = Union<bool, int, double>::create<1>(123);
        core::Dispatcher<TestDispatch, a.size()>::dispatch(1, mask);
        REQUIRE(mask[0] == false);
        REQUIRE(mask[1] == true);
        REQUIRE(mask[2] == false);
        a.destroy<1>();
    }
};

#endif
