#define CATCH_CONFIG_MAIN
#include <catch.hpp>

#include <common/complex.h>
#include <common/quaternion.h>
#include <common/moebius.h>

TEST_CASE("Dummy test", "[dummy]") {
    REQUIRE(1 + 1 == 2);
};
