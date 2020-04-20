#include "horosphere.hpp"


/*
int material_no = 0;
bool border = false;

real br = horosphere->tiling.border_width/horosphere->tiling.cell_size;
if (horosphere->tiling.type == HOROSPHERE_TILING_SQUARE) {
    complex k, f;
    complex g = cache->pos.xy;
    f = fract(g/horosphere->tiling.cell_size, &k);

    if (f.x < br || f.x > 1 - br || f.y < br || f.y > 1 - br) {
        border = true;
    } else {
        int hx = mod((int)k.x, 2);
        int hy = mod((int)k.y, 2);
        material_no = 3*hy - 2*hx*hy + hx;
    }
} else if (horosphere->tiling.type == HOROSPHERE_TILING_HEXAGONAL) {
    real2 g = cache->pos.xy;
    real2 bx = make_real2(2/sqrt(3.0f), (real)0);
    real2 by = make_real2(-1/sqrt(3.0f), (real)1);
    real s = horosphere->tiling.cell_size*sqrt(3.0f)/2;
    real2 h = make_real2(dot(bx, g), dot(by, g))/s;
    int hx = (int)floor((floor(h.x) - floor(h.y))/3);
    int hy = (int)floor((floor(h.x + h.y) - hx)/2);

    h -= hx*make_real2(2.0f, -1.0f) + hy*make_real2(1.0f, 1.0f);
    if (fabs(h.x - 1) > 1 - br || fabs(h.y) > 1 - br || fabs(h.x + h.y - 1) > 1 - br) {
        border = true;
    } else {
        material_no = 2*hx + hy;
    }
} else {
    material_no = 0;
}

*hit_dir = cache->dir;
*normal = -QJ;

if (border) {
    *material = horosphere->tiling.border_material;
} else {
    *material = horosphere->materials[mod(material_no, horosphere->material_count)];
}
*/


#ifdef UNIT_TEST
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
            real dist = h.detect(ctx, light);
            if (incoming.start[2] > 1_r) {
                REQUIRE(dist > 0_r);
            }
            if (dist > 0_r) {
                REQUIRE(light.ray.start[2] == approx(1));
            }
        }
    }
}

#endif
