#include "plane.hh"


_ALLOW_UNUSED_PARAMETERS_
real plane_hy_detect(__global const void *shape, Context *context, HyDir *normal, LightHy *light) {
    // Line cannot intersect plane twice
    if (context_is_repeat(context)) {
        return -R1;
    }

    quat p = light->ray.start, d = light->ray.direction;
    //real dxy = sqrt(d.x*d.x + d.y*d.y);
    // FIXME: check (dxy < EPS)

    real pd = dot(p, d);
    if (fabs(pd) < EPS) {
        return -R1;
    }
    real t = (R1 - length2(p))/(2*pd);
    if (t < R0) {
        return -R1;
    }
    quat h = MAKE(quat)(p.xy + d.xy*t, 0, 0);
    
    real pxy2 = length2(h.xy);
    if (pxy2 > R1) {
        return -R1;
    }
    h.z = sqrt(1 - pxy2);

    light->ray.start = h;
    light->ray.direction = hy_dir_at(p, d, h);
    *normal = h;

    return hy_distance(p, h);
}

/*
bool tiled_plane_hy_interact(
    TraceContext *context, const Cache *cache,
    LightHy *light, color3 *luminance
) {
    const Mat *material;

    if (tiling == Tiling::PENTAGONAL || tiling == Tiling::PENTASTAR) {
        quat p = light.ray.start;

        bool w = false;
        //uint n = 0, b = 1;
        real Q = math::sqrt(math::cos(PI/4 + PI/5)/math::cos(PI/4 - PI/5));
        real T = math::sqrt(math::cos(PI/4 + PI/5)*math::cos(PI/4 - PI/5));
        real S = (math::cos(PI/4) - math::sin(PI/5))/T;
        real L = T/math::cos(PI/4);
        real K = L*(2*math::cos(PI/5) - 1/math::cos(PI/5));
        Q = math::log((1 + Q)/(1 - Q));
        S = math::log((1 + S)/(1 - S));

        if (p[0] < 0_r) {
            p[0] = -p[0];
            //n |= 1;
            w = !w;
        }
        if (p[1] < 0_r) {
            p[1] = -p[1];
            //n |= 2;
            w = !w;
        }
        //b *= 4;

        p = (Hy::xshift(-Q)*Hy::zrotate(-PI/4)).apply(p);
        bool e = false;
        for (int j = 0; j < 5; ++j) {
            bool a[3] = {false, false, false};
            for (int i = 0; i < 3 - e; ++i) {
                real o = 2*PI*(i - 1)/5;
                real2 d = real2(math::cos(o), math::sin(o));
                a[i] = (dot(d, p.re().vec()) < L);
            }
            a[2] = a[2] || e;
            int s = (int)a[0] + (int)a[1] + (int)a[2];
            //uint bb = b*(6 - 2*e);
            if (s == 3) {
                break;
            } else if (s == 2) {
                int i = (!a[1]) + 2*(!a[2]);
                real o = 2*PI*(i - 1)/5;
                p = (Hy::zrotate(-PI/5)*Hy::xshift(-2*S)*Hy::zrotate(-o)).apply(p);
                //n += b*(2*i + 1);
                e = true;
                w = !w;
            } else {
                int i = a[0];
                real o = PI*(2*i - 1)/5;
                p = (Hy::xshift(-2*Q)*Hy::zrotate(-o)).apply(p);
                //n += b*(2*(i + 1));
                e = false;
            }
            //b = bb;
        }
        
        const real br = border_width;
        bool bh = 0;
        for (int i = 0; i < 5; ++i) {
            real o = 2*PI*i/5;
            real2 d = real2(math::cos(o), math::sin(o));
            bh = bh || (dot(d, p.re().vec()) > (L - br*p[2]));
            if (tiling == Tiling::PENTASTAR) {
                real ps = K + dot(d, p.re().vec());
                w = w^(ps < 0);
                bh = bh || (math::abs(ps) < br*p[2]);
            }
        }
        if (bh == 0) {
            if (w || materials.size() < 2) {
                material = &materials[0];
            } else {
                material = &materials[1];
            }
        } else {
            material = &border_material;
        }
    } else {
        material = &materials[0];
    }
    
    real3 ln = Hy::dir_to_local(light.ray.start, cache.normal);
    LocalLight ll = light.get_local();
    bool bounce = material->interact(context, ln, ll, luminance);
    if (bounce) {
        light.set_local(ll);
    }
    return bounce;
}
*/
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
            ASSERT_EQ(normal, approx(light.ray.start));
        }
    }
    ASSERT_TRUE(hits > 0);
}

#endif // UNITTEST
