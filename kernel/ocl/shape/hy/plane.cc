#include "plane.hh"


__allow_unused_params__
real plane_hy_detect(__global const Plane *shape, Context *context, HyDir *normal, LightHy *light) {
    // Line cannot intersect plane twice
    if (context_is_repeat(context)) {
        return -R1;
    }

    quat p = light->ray.start, d = light->ray.direction;

    real pd = dot(p, d);
    // The magnitude scales with position. A fixed epsilon would reject even
    // vertical intersections when p.z is small near the ideal boundary.
    if (pd == R0) {
        return -R1;
    }
    real s = R1 - length2(p);
    real t = s / (2 * pd);
    if (t < -EPS) {
        return -R1;
    }
    quat h = make(quat)(p.xy + d.xy * t, 0, 0);
    
    real pxy2 = length2(h.xy);
    // A point at z=0 is at infinity, not a finite intersection.
    if (pxy2 >= R1) {
        return -R1;
    }
    h.z = sqrt(R1 - pxy2);

    light->ray.start = h;
    light->ray.direction = hy_dir_at(p, d, h);
    *normal = -h;

    return hy_distance(p, h);
}

#ifdef UNITTEST

#include <gtest/gtest.h>

class PlaneHyTest : public testing::Test {
protected:
    TestRng<real3> vrng = TestRng<real3>(0xBAAB);
    TestRngHyPos hyrng = TestRngHyPos(0xBAAB);
};

TEST_F(PlaneHyTest, detect) {
    Context ctx = context_init();
    size_t hits = 0;
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        quat start = hyrng.normal(), dir = quat(vrng.unit(), 0.0);
        LightHy light;
        light.ray = RayHy { start, dir };
        quat normal;
        
        real dist = plane_hy_detect(nullptr, &ctx, &normal, &light);

        if (dist > -EPS) {
            hits += 1;

            ASSERT_EQ(length(light.ray.start), approx(1));
            ASSERT_EQ(length(light.ray.direction), approx(1));

            ASSERT_EQ(length(normal), approx(1));
            ASSERT_EQ(normal, approx(-light.ray.start));
        }
    }
    ASSERT_TRUE(hits > 0);
}

#endif // UNITTEST
