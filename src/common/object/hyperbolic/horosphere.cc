#include "horosphere.hpp"


#ifdef TEST_CATCH
#include <catch.hpp>
#include <test.hpp>

using namespace test;
using namespace hyperbolic;


TEST_CASE("Horosphere", "[hyperbolic.horosphere]") {
    Rng rng(0x807A);

    struct DummyCtx {
        bool repeat = false;
    } ctx;

    SECTION("Collision") {
        Horosphere h;
        for (int i = 0; i < 64; ++i) {
            Ray<Hy> incoming(
                quat(rng.d<real2>().normal(), math::exp(rng.d<real>().normal()), 0),
                quat(rng.d<real3>().unit(), 0)
            );
            Light<Hy> light{incoming, float3(0)};
            quat normal;
            real dist = h.detect(ctx, normal, light);
            if (incoming.start[2] > 1_r) {
                REQUIRE(dist > 0_r);
            }
            if (dist > 0_r) {
                REQUIRE(light.ray.start[2] == approx(1));
                REQUIRE(length(light.ray.direction) == approx(1));

                REQUIRE(normal == approx(quat(0,0,1,0)));
            }
        }
    }
}

#endif
