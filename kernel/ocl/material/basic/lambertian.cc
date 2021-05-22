#include "lambertian.hh"

#include <transform/rotation.hh>

_ALLOW_UNUSED_PARAMETERS_
bool lambertian_interact(__global const Lambertian *self, Context *context, real3 normal, LightLocal *light, float3 *emission) {
    real3 rand = random_hemisphere_cosine(&context->rng);
    Rotation3 rot = rot3_look_at(-normal);
    light->direction = rot3_apply_dir(rot, MAKE(real3)(R0), rand);
    return true;
}

#ifdef UNITTEST

#include <gtest/gtest.h>

// Monte Carlo confidence interval
static const real CONF = (real)3/sqrt(real(TEST_ATTEMPTS));

class MaterialTest : public testing::Test {
protected:
    TestRng<real3> vrng = TestRng<real3>(0xABCD);
};

TEST_F(MaterialTest, lambertian) {
    Context ctx;
    ctx.rng = Rng { 0xDEADBEEF };

    real3 sum = real3(R0);
    size_t P = 16, R = 16;
    DiskGrid grid(P, R);

    real3 normal = vrng.unit();
    Rotation3 loc = rot3_inverse(rot3_look_at(-normal));
    float3 emission(0.0f);

    const size_t N = P*R*TEST_ATTEMPTS;
    for (size_t i = 0; i < N; ++i) {
        real3 dir = vrng.unit();
        if (dot(normal, dir) > 0.0) {
            dir = -dir;
        }
        LightLocal light {
            LightBase {
                float3(1.0f),
                false
            },
            dir
        };

        bool b = lambertian_interact(nullptr, &ctx, normal, &light, &emission);
        real3 odir = light.direction;

        ASSERT_TRUE(b);
        ASSERT_EQ(length(odir), approx(1));
        ASSERT_GT(dot(odir, normal), R0);

        real3 ldir = rot3_apply_dir(loc, MAKE(real3)(R0), odir);
        sum += ldir;
        grid[ldir.xy] += 1.0;
    }

    ASSERT_EQ(emission, approx(float3(0.0f)));
    ASSERT_EQ(sum.xy/N, approx(real2(R0)).epsilon(2*CONF));
    ASSERT_TRUE(grid.check_all(N, CONF));
}

#endif // UNITTEST
