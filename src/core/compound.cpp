#ifdef TEST_UNIT

#include "prelude.hpp"

#include <catch.hpp>
#include <memory>

TEST_CASE("Compound", "[compound]") {
    SECTION("Tuple of option") {
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
