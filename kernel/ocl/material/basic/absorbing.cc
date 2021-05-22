#include "absorbing.hh"

_ALLOW_UNUSED_PARAMETERS_
bool absorbing_interact(__global const Absorbing *self, Context *context, real3 normal, LightLocal *light, float3 *emission) {
    return false;
}

#ifdef UNITTEST

#include <gtest/gtest.h>

// Monte Carlo confidence interval
static const real CONF = (real)3/sqrt(real(TEST_ATTEMPTS));

class MaterialTest : public testing::Test {
protected:
    TestRng<real3> vrng = TestRng<real3>(0xABCD);
};

TEST_F(MaterialTest, absorbing) {
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

        bool b = absorbing_interact(nullptr, &ctx, normal, &light, &emission);

        ASSERT_TRUE(!b);
        ASSERT_EQ(emission, approx(float3(0.0f)));
    }
}

#endif // UNITTEST
