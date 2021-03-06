#ifdef UNITTEST

#include "../transparent.hh"

struct ColoredTransparent {
    // Transparent inner;
    color3 color;
};
__global const Transparent *colored_transparent__inner__gc(__global const ColoredTransparent *self) {
    return NULL;
}

#define $Self ColoredTransparent
#define $self colored_transparent
#define $Material Transparent
#define $material transparent
#include "../colored.inl"
#undef $Self
#undef $self
#undef $Material
#undef $material

struct EmissiveTransparent {
    // Transparent inner;
    color3 emission;
};
__global const Transparent *emissive_transparent__inner__gc(__global const EmissiveTransparent *self) {
    return NULL;
}

#define $Self EmissiveTransparent
#define $self emissive_transparent
#define $Material Transparent
#define $material transparent
#include "../emissive.inl"
#undef $Self
#undef $self
#undef $Material
#undef $material

#include <gtest/gtest.h>

class ModifierTest : public testing::Test {
protected:
    TestRng<real3> vrng = TestRng<real3>(0xDCAB);
    TestRng<color3> colrng = TestRng<color3>(0xDCBA);
};

TEST_F(ModifierTest, colored) {
    Context ctx;
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        real3 normal = vrng.unit();
        real3 dir = vrng.unit();
        color3 color = colrng.uniform();
        if (dot(normal, dir) > 0.0) {
            dir = -dir;
        }
        LightLocal light {
            LightBase {
                color,
                false
            },
            dir
        };
        float3 emission(0.0f);

        ColoredTransparent material;
        material.color = colrng.uniform();
        bool b = colored_transparent_interact(&material, &ctx, normal, &light, &emission);

        ASSERT_TRUE(b);
        ASSERT_EQ(emission, approx(float3(0.0f)));
        ASSERT_EQ(light.direction, approx(dir));
        ASSERT_EQ(light.base.intensity, approx(color * material.color));
    }
}

TEST_F(ModifierTest, emissive) {
    Context ctx;
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        real3 normal = vrng.unit();
        real3 dir = vrng.unit();
        color3 color = colrng.uniform();
        color3 base = colrng.uniform();
        if (dot(normal, dir) > 0.0) {
            dir = -dir;
        }
        LightLocal light {
            LightBase {
                color,
                false
            },
            dir
        };
        float3 emission = base;

        EmissiveTransparent material;
        material.emission = colrng.uniform();
        bool b = emissive_transparent_interact(&material, &ctx, normal, &light, &emission);

        ASSERT_TRUE(b);
        ASSERT_EQ(emission, approx(base + color * material.emission));
        ASSERT_EQ(light.direction, approx(dir));
        ASSERT_EQ(light.base.intensity, approx(color));
    }
}

#endif // UNITTEST
