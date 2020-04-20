#include "plane.hpp"


#ifdef UNIT_TEST
#include <catch.hpp>
#include <test.hpp>

using namespace test;
using namespace hyperbolic;


TEST_CASE("Hyperbolic Plane", "[hyperbolic.plane]") {
    Rng rng(0x807A);

    struct DummyCtx {
        bool repeat = false;
    } ctx;

    SECTION("Collision") {
        Plane h;
        int hits = 0;
        for (int i = 0; i < 64; ++i) {
            Ray<Hy> incoming(
                quat(rng.d<real2>().normal(), math::exp(rng.d<real>().normal()), 0),
                quat(rng.d<real3>().unit(), 0)
            );
            Light<Hy> light{incoming, float3(0)};
            quat normal;
            real dist = h.detect(ctx, normal, light);
            if (dist > 0_r) {
                hits += 1;

                REQUIRE(length(light.ray.start) == approx(1));
                REQUIRE(length(light.ray.direction) == approx(1));

                REQUIRE(length(normal) == approx(1));
                REQUIRE(normal == approx(light.ray.start));
            }
        }
        REQUIRE(hits > 0);
    }
}

#endif
