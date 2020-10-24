#include "material.hh"

#include <common/algebra/linear.hh>


_ALLOW_UNUSED_PARAMETERS_
bool black_interact(__global const void *material, Context *context, real3 normal, LightLocal *light, float3 *emission) {
    return false;
}

_ALLOW_UNUSED_PARAMETERS_
bool transparent_interact(__global const void *material, Context *context, real3 normal, LightLocal *light, float3 *emission) {
    //light.face = !light.face;
    return true;
}

_ALLOW_UNUSED_PARAMETERS_
bool specular_interact(__global const void *material, Context *context, real3 normal, LightLocal *light, float3 *emission) {
    light->direction -= (2*dot(light->direction, normal))*normal;
    return true;
}

_ALLOW_UNUSED_PARAMETERS_
bool lambertian_interact(__global const void *material, Context *context, real3 normal, LightLocal *light, float3 *emission) {
    if (dot(normal, light->direction) > R0) {
        normal = -normal;
    }
    real3 rand = random_hemisphere_cosine(context->rng);
    // FIXME: Use Rotation3 instead of more general Linear3
    Linear3 rot = lin3_inverse(lin3_look_to(normal)); // Transpose
    light->direction = lin3_apply(rot, rand);
    //light->diffuse = true;
    return true;
}


#ifdef TEST

#include <rtest.hpp>

#ifdef TEST_UNIT

rtest_module_(material) {
    // Monte Carlo confidence interval
    static const real CONF = 3.0/sqrt(real(TEST_ATTEMPTS));

    static_thread_local_(TestRng<real3>, rng) {
        return TestRng<real3>(0xABBA);
    }
    static_thread_local_(TestRng<real3>, vrng) {
        return TestRng<real3>(0xBAAB);
    }

    rtest_(black) {
        Context ctx;
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            real3 normal = rng->unit();
            real3 dir = rng->unit();
            if (dot(normal, dir) > 0.0) {
                dir = -dir;
            }
            LightLocal light {
                LightBase {
                    float3(1.0f),
                    false
                },
                dir
            };
            float3 emission(0.0f);

            bool b = black_interact(nullptr, &ctx, normal, &light, &emission);

            assert_(!b);
            assert_eq_(emission, approx(float3(0.0f)));
        }
    }
    rtest_(transparent) {
        Context ctx;
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            real3 normal = rng->unit();
            real3 dir = rng->unit();
            if (dot(normal, dir) > 0.0) {
                dir = -dir;
            }
            LightLocal light {
                LightBase {
                    float3(1.0f),
                    false
                },
                dir
            };
            float3 emission(0.0f);

            bool b = transparent_interact(nullptr, &ctx, normal, &light, &emission);

            assert_(b);
            assert_eq_(emission, approx(float3(0.0f)));
            assert_eq_(light.direction, approx(dir));
        }
    }
    rtest_(specular) {
        Context ctx;
        for (int i = 0; i < TEST_ATTEMPTS; ++i) {
            real3 normal = rng->unit();
            real3 dir = rng->unit();
            if (dot(normal, dir) > 0.0) {
                dir = -dir;
            }
            LightLocal light {
                LightBase {
                    float3(1.0f),
                    false
                },
                dir
            };
            float3 emission(0.0f);

            bool b = specular_interact(nullptr, &ctx, normal, &light, &emission);

            assert_(b);
            assert_eq_(emission, approx(float3(0.0f)));
            assert_eq_(length(light.direction), approx(1.0));
            assert_eq_(normalize(light.direction - dir), approx(normal));
        }
    }
    rtest_(lambertian) {
        Context ctx;
        Rng devrng { 0xDEADBEEF };
        ctx.rng = &devrng;

        real3 sum = real3(R0);
        size_t P = 16, R = 16;
        DiskGrid grid(P, R);

        real3 normal = rng->unit();
        Linear3 loc = lin3_inverse(lin3_look_to(normal));
        float3 emission(0.0f);

        const size_t N = P*R*TEST_ATTEMPTS;
        for (size_t i = 0; i < N; ++i) {
            real3 dir = rng->unit();
            if (dot(normal, dir) > 0.0) {
                dir = -dir;
            }
            LightLocal light {
                LightBase {
                    float3(1.0f),
                    false
                },
                dir
            };

            bool b = lambertian_interact(nullptr, &ctx, normal, &light, &emission);
            real3 odir = light.direction;

            assert_(b);
            assert_eq_(length(odir), approx(1.0));
            assert_(dot(odir, normal) > 0.0);

            real3 ldir = lin3_apply(loc, odir);
            sum += ldir;
            grid[ldir.xy] += 1.0;
        }

        assert_eq_(emission, approx(float3(0.0f)));
        assert_eq_(sum.xy/N, approx(real2(R0)).epsilon(2*CONF));
        grid.assert_all(N, CONF);
    }
}

#endif // TEST_UNIT

/*
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
*/

#endif // TEST
