#include "horosphere.hh"


_ALLOW_UNUSED_PARAMETERS_
real horosphere_detect(__global const void *shape, Context *context, HyDir *normal, LightHy *light) {
    quat p = light->ray.start, d = light->ray.direction;
    real dxy = length(d.xy);
    // FIXME: check (dxy < EPS)

    if (p.z < dxy) {
        return -R1;
    }
    
    real dt = sqrt(p.z*p.z - dxy*dxy);
    real t = p.z*d.z - dt;
    if (t < R0 + context->repeat*EPS) {
        t += 2*dt;
    }
    if (t < R0 + context->repeat*EPS) {
        return -R1;
    }

    t /= dxy*dxy;
    quat h = MAKE(quat)(p.xy + d.xy*t, 1, 0);

    light->ray.start = h;
    light->ray.direction = hy_dir_at(p, d, h);
    *normal = MAKE(quat)(0, 0, 1, 0);

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
    Context ctx;
    ctx.repeat = false;
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        quat start = hyrng.normal(), dir = quat(vrng.unit(), 0.0);
        LightHy light;
        light.ray = RayHy { start, dir };
        quat normal;
        
        real dist = horosphere_detect(nullptr, &ctx, &normal, &light);

        if (light.ray.start.z > R1) {
            ASSERT_TRUE(dist > -EPS);
        }
        if (dist > -EPS) {
            ASSERT_EQ(light.ray.start.z, approx(1));
            ASSERT_EQ(length(light.ray.direction), approx(1));

            ASSERT_EQ(normal, approx(quat(0,0,1,0)));
        }
    }
}

#endif // UNITTEST
