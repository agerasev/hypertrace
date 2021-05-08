#include "sphere.hh"


_ALLOW_UNUSED_PARAMETERS_
real sphere_eu_detect(__global const void *shape, Context *context, real3 *normal, RayEu *ray) {
    real3 pos = ray->start;
    real3 dir = ray->direction;
    real b = -dot(dir, pos);
    real c = dot(pos, pos) - R1;
    real d = b*b - c;
    if (d < R0) {
        return -R1;
    }
    d = sqrt(d);
    real e = b - d;
    if (e < EPS*context->repeat) {
        return -R1;
    }
    real3 h = pos + dir*e;
    *normal = h;
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
    Context ctx;
    ctx.repeat = false;
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        real3 start = vrng.normal(), dir = vrng.unit();
        RayEu ray { start, dir };
        real3 normal;
        
        real dist = sphere_eu_detect(nullptr, &ctx, &normal, &ray);

        if (length(start) < 1 - EPS) {
            // TODO: Update on refraction
            ASSERT_EQ(dist, approx(-1));
        }
        if (length(start) > 1 + EPS) {
            real proj = -dot(start, dir);
            if (proj > EPS && length(start + proj*dir) < 1 - EPS) {
                ASSERT_EQ(length2(ray.start), approx(1));
                ASSERT_EQ(ray.start, approx(normal));
                ASSERT_TRUE(dist > -EPS);
            } else {
                ASSERT_EQ(dist, approx(-1));
            }
        }
    }
}

#endif // UNITTEST
