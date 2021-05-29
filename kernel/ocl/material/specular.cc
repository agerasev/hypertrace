#include "specular.hh"

__allow_unused_params__
bool specular_interact(__global const Specular *self, Context *context, real3 normal, LightLocal *light, float3 *emission) {
    light->direction -= (2 * dot(light->direction, normal)) * normal;
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

TEST_F(MaterialTest, specular) {
    Context ctx;
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        real3 normal = vrng.unit();
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
        float3 emission(0.0f);

        bool b = specular_interact(nullptr, &ctx, normal, &light, &emission);

        ASSERT_TRUE(b);
        ASSERT_EQ(emission, approx(float3(0.0f)));
        ASSERT_EQ(length(light.direction), approx(1));
        ASSERT_EQ(normalize(light.direction - dir), approx(normal));
    }
}

#endif // UNITTEST
