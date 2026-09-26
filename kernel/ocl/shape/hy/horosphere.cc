#include "horosphere.hh"


__allow_unused_params__
real horosphere_detect(__global const void *shape, Context *context, HyDir *normal, LightHy *light) {
    bool repeat = context_is_repeat(context);

    quat p = light->ray.start, d = light->ray.direction;
    real w = EPS * (2 * repeat - 1);
    real a = length2(d.xy);
    real t;
    if (a == R0) {
        // The geodesic is a vertical line, not a circle of infinite radius.
        t = (R1 - p.z) * (R1 + p.z) / (2 * p.z * d.z);
        if (t < w) {
            return -R1;
        }
    } else {
        real dxy = sqrt(a);
        if (p.z < dxy) {
            return -R1;
        }
        real dt = sqrt((p.z - dxy) * (p.z + dxy));
        real b = p.z * d.z;
        real q = b + copysign(dt, b);
        // The product of the roots is (1-p.z*p.z)/a. Recovering the
        // smaller root this way avoids cancellation for near-vertical rays.
        real first = q / a;
        real second = q == R0 ? R0 : (R1 - p.z) * (R1 + p.z) / q;
        t = fmin(first, second);
        if (t < w) {
            t = fmax(first, second);
            if (t < w) {
                return -R1;
            }
        }
    }

    quat h = make(quat)(p.xy + d.xy*t, 1, 0);

    light->ray.start = h;
    light->ray.direction = hy_dir_at(p, d, h);
    *normal = make(quat)(0, 0, -1, 0);

    return hy_distance(p, h);
}


#ifdef UNITTEST

#include <gtest/gtest.h>

class HorosphereTest : public testing::Test {
protected:
    TestRng<real3> vrng = TestRng<real3>(0x807A);
    TestRngHyPos hyrng = TestRngHyPos(0x807A);
};

TEST_F(HorosphereTest, detect) {
    Context ctx = context_init();
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        quat start = hyrng.normal(), dir = quat(vrng.unit(), 0.0);
        LightHy light;
        light.ray = RayHy { start, dir };
        quat normal;
        
        real dist = horosphere_detect(nullptr, &ctx, &normal, &light);

        if (start.z > R1 + EPS) {
            ASSERT_TRUE(dist > -EPS);
            ASSERT_EQ(normal, approx(quat(0,0,-1,0)));
        }
        if (dist > -EPS) {
            ASSERT_EQ(light.ray.start.z, approx(1));
            ASSERT_EQ(length(light.ray.direction), approx(1));   
            if (start.z < R1 - EPS) {
                ASSERT_EQ(normal, approx(quat(0,0,-1,0)));
            }         
        }
    }
}

#endif // UNITTEST
