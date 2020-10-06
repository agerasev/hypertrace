#include "affine.hpp"

#ifdef TEST_CATCH
#include <catch.hpp>
#include "test.hpp"

using namespace test;

TEST_CASE("Affine transformation", "[affine]") {
    Rng rng;

    SECTION("Chaining") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Affine<real, 3> a = rng.d<Affine<real, 3>>().some();
            Affine<real, 3> b = rng.d<Affine<real, 3>>().some();
            real3 c = rng.d<real3>().normal();

            REQUIRE((a*b).apply(c) == approx(a.apply(b.apply(c))));
        }
    }
    SECTION("Inversion") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Affine<real, 3> a = rng.d<Affine<real, 3>>().some();
            real3 x = rng.d<real3>().normal();
            
            REQUIRE((!a).apply(a.apply(x)) == approx(x));
            REQUIRE(a.apply((!a).apply(x)) == approx(x));
        }
    }
};
#endif // TEST_CATCH
