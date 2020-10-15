#include "horosphere.hh"


real horosphere_detect(Context *context, HyDir *normal, LightHy *light) {
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


#ifdef TEST

#include <rtest.hpp>

#ifdef TEST_UNIT

rtest_module_(horosphere) {
    static_thread_local_(TestRng<real3>, vrng) {
        return TestRng<real3>(0x807A);
    }
    static_thread_local_(TestRngHyPos, hyrng) {
        return TestRngHyPos(0x807A);
    }

    rtest_(detect) {
        Context ctx;
        ctx.repeat = false;
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            quat start = hyrng->normal(), dir = quat(vrng->unit(), 0.0);
            LightHy light;
            light.ray = RayHy { start, dir };
            quat normal;
            
            real dist = horosphere_detect(&ctx, &normal, &light);

            if (light.ray.start.z > R1) {
                assert_(dist > -EPS);
            }
            if (dist > -EPS) {
                assert_eq_(light.ray.start.z, approx(1));
                assert_eq_(length(light.ray.direction), approx(1));

                assert_eq_(normal, approx(quat(0,0,1,0)));
            }
        }
    }
}

#endif // TEST_UNIT


#ifdef TEST_DEV

#include <rtest.hpp>

#include <vector>

#include <test/devtest.hpp>

extern devtest::Target devtest_make_target();

rtest_module_(horosphere) {
    rtest_(detect) {
        TestRng<real3> vrng(0xBAAB);
        TestRngHyPos hyrng(0xBAAB);

        devtest::Target target = devtest_make_target();
        auto queue = target.make_queue();
        auto kernel = devtest::KernelBuilder(target.device_id(), queue)
        .source("horosphere.cl", std::string(
            "#include <common/object/hyperbolic/horosphere.hh>\n"
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
            "    dist[i] = horosphere_detect(&ctx, &normal, &light);\n"
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
            if (start[i].z > R1) {
                assert_(dist[i] > -DEV_EPS);
            }
            if (dist[i] > -DEV_EPS) {
                assert_eq_(hit[i].z, dev_approx(1));
                assert_eq_(length(odir[i]), dev_approx(1));

                assert_eq_(norm[i], dev_approx(quat(0,0,1,0)));
            }
        }
    }
}

#endif // TEST_DEV

#endif // TEST
