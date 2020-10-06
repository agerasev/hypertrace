#include "linear.hpp"

#ifdef TEST_CATCH
#include <catch.hpp>
#include "test.hpp"

using namespace test;

TEST_CASE("Linear transformation", "[linear]") {
    Rng rng;

    SECTION("Linearity") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Linear<real, 3> m = rng.d<Linear<real, 3>>().some();
            real a = rng.d<real>().normal();
            real3 x = rng.d<real3>().normal();

            REQUIRE((Linear<real, 3>(a*m.mat())).apply(x) == approx(m.apply(a*x)));
        }
    }
    SECTION("Chaining") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Linear<real, 3> a = rng.d<Linear<real, 3>>().some();
            Linear<real, 3> b = rng.d<Linear<real, 3>>().some();
            real3 c = rng.d<real3>().normal();

            REQUIRE((a*b).apply(c) == approx(a.apply(b.apply(c))));
        }
    }
    SECTION("Inversion") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            Linear<real, 3> a = rng.d<Linear<real, 3>>().some();
            real3 x = rng.d<real3>().normal();

            REQUIRE((a*!a).mat() == approx(Linear<real, 3>::identity().mat()));
            REQUIRE((!a*a).mat() == approx(Linear<real, 3>::identity().mat()));
            REQUIRE((!a).apply(a.apply(x)) == approx(x));
            REQUIRE(a.apply((!a).apply(x)) == approx(x));
        }
    }
    SECTION("Look to the direction") {
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            real3 d = rng.d<real3>().unit();
            Linear<real, 3> m = Linear<real, 3>::look_to(d);

            REQUIRE(m.apply(d) == approx(real3(0,0,1)));
        }
    }
};
#endif // TEST_CATCH
