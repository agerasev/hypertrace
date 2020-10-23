#include "plane.hh"


_ALLOW_UNUSED_PARAMETERS_
real planehy_detect(__global const void *shape, Context *context, HyDir *normal, LightHy *light) {
    // Line cannot intersect plane twice
    if (context->repeat) {
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
#ifdef TEST

#include <rtest.hpp>

#ifdef TEST_UNIT

rtest_module_(hyperbolic_plane) {
    static_thread_local_(TestRng<real3>, vrng) {
        return TestRng<real3>(0xBAAB);
    }
    static_thread_local_(TestRngHyPos, hyrng) {
        return TestRngHyPos(0xBAAB);
    }

    rtest_(detect) {
        Context ctx;
        ctx.repeat = false;
        size_t hits = 0;
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            quat start = hyrng->normal(), dir = quat(vrng->unit(), 0.0);
            LightHy light;
            light.ray = RayHy { start, dir };
            quat normal;
            
            real dist = planehy_detect(nullptr, &ctx, &normal, &light);

            if (dist > -EPS) {
                hits += 1;

                assert_eq_(length(light.ray.start), approx(1));
                assert_eq_(length(light.ray.direction), approx(1));

                assert_eq_(length(normal), approx(1));
                assert_eq_(normal, approx(light.ray.start));
            }
        }
        assert_(hits > 0);
    }
}

#endif // TEST_UNIT


#ifdef TEST_DEV

#include <rtest.hpp>
#include <vector>
#include <test/devtest.hpp>
#include <host/renderer.hpp>

using namespace rstd;

extern devtest::Target devtest_make_target();

rtest_module_(hyperbolic_plane) {
    rtest_(detect) {
        TestRng<real3> vrng(0xBAAB);
        TestRngHyPos hyrng(0xBAAB);

        devtest::Target target = devtest_make_target();
        auto queue = target.make_queue();
        auto kernel = devtest::KernelBuilder(target.device_id(), queue)
        .source("hyperbolic_plane.cl", std::string(
            "#include <common/object/hyperbolic/plane.hh>\n"
            "__kernel void detect(\n"
            "    __global const quat *start, __global const quat *idir,\n"
            "    __global quat *hit, __global quat *odir, __global quat *norm,\n"
            "    __global real *dist\n"
            ") {\n"
            "    int i = get_global_id(0);\n"
            "    Context ctx;\n"
            "    ctx.repeat = false;\n"
            "    LightHy light;\n"
            "    light.ray.start = start[i];\n"
            "    light.ray.direction = idir[i];\n"
            "    quat normal;\n"
            "    dist[i] = planehy_detect(NULL, &ctx, &normal, &light);\n"
            "    hit[i] = light.ray.start;\n"
            "    odir[i] = light.ray.direction;\n"
            "    norm[i] = normal;\n"
            "}\n"
        ))
        .build("detect").expect("Kernel build error");

        const int n = TEST_ATTEMPTS;
        std::vector<quat> start(n), idir(n), hit(n), odir(n), norm(n);
        std::vector<real> dist(n);
        for (size_t i = 0; i < n; ++i) {
            start[i] = hyrng.normal();
            idir[i] = quat(vrng.unit(), 0.0);
        }

        devtest::KernelRunner(queue, std::move(kernel))
        .run(n, start, idir, hit, odir, norm, dist).expect("Kernel run error");

        for(size_t i = 0; i < n; ++i) {
            if (dist[i] > -DEV_EPS) {
                assert_eq_(length(hit[i]), dev_approx(1));
                assert_eq_(length(odir[i]), dev_approx(1));

                assert_eq_(length(norm[i]), dev_approx(1));
                assert_eq_(norm[i], dev_approx(hit[i]));
            }
        }
    }
    rtest_(renderer) {
        devtest::Target target = devtest_make_target();
        Renderer<Hyperbolic> renderer(
            target.device_id(),
            target.context(),
            800, 600,
            Box(Plane<Hy>())
        );
        renderer.set_view(View<Hy> { mo_identity(), 1, 1, 1 });
        renderer.render();
    }
}

#endif // TEST_DEV

#endif // TEST
