#include "cube.hh"


real cube_eu_detect_nearest(real3 near, real3 *normal) {
    bool xy = near.x > near.y;
    bool yz = near.y > near.z;
    bool xz = near.x > near.z;
    real dist = R0;
    //*normal = MAKE(real3)(R0);
    if (xy && xz) {
        dist = near.x;
        normal->x = R1;
    } else if (yz) {
        dist = near.y;
        normal->y = R1;
    } else {
        dist = near.z;
        normal->z = R1;
    }
    return dist;
}

_ALLOW_UNUSED_PARAMETERS_
real cube_eu_detect(__global const Cube *shape, Context *context, real3 *normal, LightEu *light) {
    bool repeat = context_is_repeat(context);
    RayEu *ray = &light->ray;

    const real3 cmax = MAKE(real3)(R1);
    const real3 cmin = MAKE(real3)(-R1);

    real3 inv_dir = R1/ray->direction;

    real3 vmin = (cmin - ray->start)*inv_dir;
    real3 vmax = (cmax - ray->start)*inv_dir;

    real3 near = fmin(vmin, vmax);
    real3 far = fmax(vmin, vmax);

    real3 norm_in = MAKE(real3)(R0);
    float dist_in = cube_eu_detect_nearest(near, &norm_in);
    norm_in *= -sign(ray->direction);

    real3 norm_out = MAKE(real3)(R0);
    float dist_out = -cube_eu_detect_nearest(-far, &norm_out);
    norm_out *= sign(ray->direction);

    if (dist_in > dist_out) {
        return -R1;
    }

    real dist = dist_in;
    real3 norm = norm_in;
    real w = 2 * EPS * (2 * repeat - 1);
    if (dist < w) {
        dist = dist_out;
        norm = norm_out;
        if (dist < w) {
            return -R1;
        }
    }

    ray->start += dist * ray->direction;
    *normal = norm;
    return dist;
}


#ifdef UNITTEST

#include <gtest/gtest.h>

real max_comp(real3 v) {
    return max(max(v.x, v.y), v.z);
}
real max_comp_idx(real3 v) {
    if (v.y > v.x) {
        if (v.z > v.y) {
            return 2;
        } else {
            return 1;
        }
    } else {
        if (v.z > v.x) {
            return 2;
        } else {
            return 0;
        }
    }
}

class CubeEuTest : public testing::Test {
protected:
    TestRng<real3> vrng = TestRng<real3>(0xBAAB);
};

TEST_F(CubeEuTest, cube) {
    Context ctx = context_init();
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        real3 start = vrng.normal(), dir = vrng.unit();
        LightEu light { LightBase {}, RayEu { start, dir }};
        real3 normal;

        real dist = cube_eu_detect(nullptr, &ctx, &normal, &light);
        
        if (max_comp(fabs(start)) < 1 - EPS) {
            ASSERT_EQ(max_comp(fabs(light.ray.start)), approx(1).epsilon(sqrt(EPS)));
        }
        if (max_comp(fabs(start)) > 1 + EPS) {
            real proj = -dot(start, dir);
            bool sph = proj > EPS && length(start + proj*dir) < 1 - EPS;
            if (sph || dist > EPS) {
                ASSERT_EQ(max_comp(fabs(light.ray.start)), approx(1).epsilon(sqrt(EPS)));
                int idx = max_comp_idx(fabs(light.ray.start));
                ASSERT_EQ(normal[idx], approx(sign(light.ray.start[idx])));
                normal[idx] = 0;
                ASSERT_EQ(normal, approx(real3(0)));
                ASSERT_TRUE(dist > -EPS);
            }
        }
    }
}

#endif // UNITTEST
