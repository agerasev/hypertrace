#include "traits.hpp"


#ifdef TEST_UNIT

#include <catch.hpp>


TEST_CASE("Traits", "[traits]") {
    SECTION("Any/all") {
        REQUIRE((any<false, false, true>()) == true);
        REQUIRE((any<false, false, false>()) == false);

        REQUIRE((all<true, true, true>()) == true);
        REQUIRE((all<true, false, true>()) == false);
    }
}

#endif // TEST_UNIT
