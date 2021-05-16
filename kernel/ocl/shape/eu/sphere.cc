#include "sphere.hh"


_ALLOW_UNUSED_PARAMETERS_
real sphere_eu_detect(__global const void *shape, Context *context, real3 *normal, LightEu *light) {
    bool repeat = context_is_repeat(context);
    RayEu *ray = &light->ray;

    real3 pos = ray->start;
    real3 dir = ray->direction;
    real b = -dot(dir, pos);
    real3 g = pos + dir * b;
    real d = R1 - length2(g);
    if (d < R0) {
        return -R1;
    }
    d = sqrt(d);
    real f = R1;
    real e = b - d;
    real w = 2 * EPS * (2 * repeat - 1);
    if (e < w) {
        e = b + d;
        f = -R1;
        if (e < w) {
            return -R1;
        }
    }
    real3 h = normalize(pos + dir * e);
    *normal = h * f;
    ray->start = h;
    return e;
}


#ifdef UNITTEST

#include <gtest/gtest.h>

class SphereEuTest : public testing::Test {
protected:
    TestRng<real3> vrng = TestRng<real3>(0xBAAB);
};

TEST_F(SphereEuTest, detect) {
    Context ctx = context_init();
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        real3 start = vrng.normal(), dir = vrng.unit();
        LightEu light { LightBase {}, RayEu { start, dir } };
        real3 normal;

        real dist = sphere_eu_detect(nullptr, &ctx, &normal, &light);

        if (length(start) < 1 - EPS) {
            ASSERT_EQ(length(start + dist*dir), approx(1));
        }
        if (length(start) > 1 + EPS) {
            real proj = -dot(start, dir);
            if (proj > EPS && length(start + proj*dir) < 1 - EPS) {
                ASSERT_EQ(length2(light.ray.start), approx(1));
                ASSERT_EQ(light.ray.start, approx(normal));
                ASSERT_TRUE(dist > -EPS);
            } else {
                ASSERT_EQ(dist, approx(-1));
            }
        }
    }
}

#endif // UNITTEST
