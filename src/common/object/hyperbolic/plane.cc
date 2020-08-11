#include "plane.hh"


real detect(TraceContext &context, HyDir &normal, Light<Hy> &light) {
    // Line cannot intersect plane twice
    if (context.repeat) {
        return -R1;
    }

    quat p = light.ray.start, d = light.ray.direction;
    //real dxy = sqrt(d.x*d.x + d.y*d.y);
    // FIXME: check (dxy < EPS)

    real pd = dot(p, d);
    if (math::abs(pd) < EPS) {
        return -1_r;
    }
    real t = (1_r - length2(p))/(2*pd);
    if (t < 0_r) {
        return -1_r;
    }
    quat h(p.re() + d.re()*t, 0, 0);
    
    real pxy2 = length2(h.re());
    if (pxy2 > 1_r) {
        return -1_r;
    }
    h[2] = math::sqrt(1 - pxy2);

    light.ray.start = h;
    light.ray.direction = Hy::dir_at(p, d, h);
    normal = h;

    return Hy::distance(p, h);
}

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

#ifdef UNIT_TEST

#include <catch.hpp>

TEST_CASE("Hyperbolic Plane", "[hyperbolic.plane]") {
    Rng rng(0x807A);

    struct DummyCtx {
        bool repeat = false;
    } ctx;

    SECTION("Collision") {
        Plane h;
        int hits = 0;
        for (int i = 0; i < 64; ++i) {
            Ray<Hy> incoming(
                quat(rng.d<real2>().normal(), math::exp(rng.d<real>().normal()), 0),
                quat(rng.d<real3>().unit(), 0)
            );
            Light<Hy> light{incoming, float3(0)};
            quat normal;
            real dist = h.detect(ctx, normal, light);
            if (dist > 0_r) {
                hits += 1;

                REQUIRE(length(light.ray.start) == approx(1));
                REQUIRE(length(light.ray.direction) == approx(1));

                REQUIRE(length(normal) == approx(1));
                REQUIRE(normal == approx(light.ray.start));
            }
        }
        REQUIRE(hits > 0);
    }
}

#endif
