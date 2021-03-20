#include "shapes.hh"


_ALLOW_UNUSED_PARAMETERS_
real sphereeu_detect(__global const void *shape, Context *context, real3 *normal, LightEu *light) {
    real3 pos = light->ray.start;
    real3 dir = light->ray.direction;
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
    light->ray.start = h;
    return e;
}

static real cubeeu_detect_nearest(real3 near, real3 *normal) {
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
real cubeeu_detect(__global const void *shape, Context *context, real3 *normal, LightEu *light) {
    const real3 cmax = MAKE(real3)(R1);
    const real3 cmin = MAKE(real3)(-R1);

    real3 inv_dir = R1/light->ray.direction;

    real3 vmin = (cmin - light->ray.start)*inv_dir;
    real3 vmax = (cmax - light->ray.start)*inv_dir;

    real3 near = fmin(vmin, vmax);
    real3 far = fmax(vmin, vmax);

    real3 norm_in = MAKE(real3)(R0);
    float dist_in = cubeeu_detect_nearest(near, &norm_in);
    norm_in *= -sign(light->ray.direction);

    real3 norm_out = MAKE(real3)(R0);
    float dist_out = -cubeeu_detect_nearest(-far, &norm_out);
    norm_out *= sign(light->ray.direction);

    if (dist_in < EPS*context->repeat || dist_in > dist_out) {
        return -R1;
    }

    light->ray.start += dist_in*light->ray.direction;
    *normal = norm_in;
    return dist_in;
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

class ShapeEuTest : public testing::Test {
protected:
    TestRng<real3> vrng = TestRng<real3>(0xBAAB);
};

TEST_F(ShapeEuTest, sphere) {
    Context ctx;
    ctx.repeat = false;
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        real3 start = vrng.normal(), dir = vrng.unit();
        LightEu light;
        light.ray = RayEu { start, dir };
        real3 normal;
        
        real dist = sphereeu_detect(nullptr, &ctx, &normal, &light);

        if (length(start) < 1 - EPS) {
            // TODO: Update on refraction
            ASSERT_EQ(dist, approx(-1));
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

TEST_F(ShapeEuTest, cube) {
    Context ctx;
    ctx.repeat = false;
    for (int i = 0; i < TEST_ATTEMPTS; ++i) {
        real3 start = vrng.normal(), dir = vrng.unit();
        LightEu light;
        light.ray = RayEu { start, dir };
        real3 normal;

        real dist = cubeeu_detect(nullptr, &ctx, &normal, &light);
        
        if (max_comp(fabs(start)) < 1 - EPS) {
            // TODO: Update on refraction
            ASSERT_EQ(dist, approx(-1));
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
