#include "material.hh"

#include <common/algebra/linear.hh>


_ALLOW_UNUSED_PARAMETERS_
bool absorbing_interact(__global const void *material, Context *context, real3 normal, LightLocal *light, float3 *emission) {
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

    rtest_(absorbing) {
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

            bool b = absorbing_interact(nullptr, &ctx, normal, &light, &emission);

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

#ifdef TEST_DEV

#include <rtest.hpp>
#include <vector>
#include <test/devtest.hpp>

using namespace rstd;

extern devtest::Target devtest_make_target();

rtest_module_(material) {
    rtest_(absorbing) {
        TestRng<real> rng(0xABBA);
        TestRng<real3> vrng(0xBAAB);

        devtest::Target target = devtest_make_target();
        auto queue = target.make_queue();

        auto mty = rstd::Box<Material>(Absorbing());
        auto src = mty->source();
        auto kernel = devtest::KernelBuilder(target.device_id(), queue)
        .source("material_absorbing.cl", format_(
            "#include <{}>\n"
            "__kernel void interact(\n"
            "    __global const real3 *norm, __global const real3 *idir,\n"
            "    __global real3 *odir, __global uchar *bounce,\n"
            "    __global uint *seed\n"
            ") {{\n"
            "    int i = get_global_id(0);\n"
            "    Context ctx;\n"
            "    Rng rng;\n"
            "    rng.state = seed[i];\n"
            "    ctx.rng = &rng;\n"
            "    LightLocal light;\n"
            "    light.direction = idir[i];\n"
            "    float3 emission = (float3)(0.0f);\n"
            "    bounce[i] = {}_interact(NULL, &ctx, norm[i], &light, &emission);\n"
            "    odir[i] = light.direction;\n"
            "}}\n",
            src.name(),
            mty->prefix()
        ), std::move(src.files()))
        .build("interact").expect("Kernel build error");

        const int n = TEST_ATTEMPTS;
        std::vector<real3> idir(n), odir(n), norm(n);
        std::vector<uchar> bounce(n);
        std::vector<uint> seed(n);
        for (size_t i = 0; i < n; ++i) {
            norm[i] = vrng.unit();
            idir[i] = vrng.unit();
            if (dot(idir[i], norm[i]) > 0.0) {
                idir[i] = -idir[i];
            }
            seed[i] = uint(((ulong_)1<<32)*rng.uniform());
        }

        devtest::KernelRunner(queue, std::move(kernel))
        .run(n, norm, idir, odir, bounce, seed).expect("Kernel run error");

        for(size_t i = 0; i < n; ++i) {
            assert_(!bool(bounce[i]));
        }
    }
    rtest_(transparent) {
        TestRng<real> rng(0xABBA);
        TestRng<real3> vrng(0xBAAB);

        devtest::Target target = devtest_make_target();
        auto queue = target.make_queue();

        auto mty = rstd::Box<Material>(Transparent());
        auto src = mty->source();
        auto kernel = devtest::KernelBuilder(target.device_id(), queue)
        .source("material_transparent.cl", format_(
            "#include <{}>\n"
            "__kernel void interact(\n"
            "    __global const real3 *norm, __global const real3 *idir,\n"
            "    __global real3 *odir, __global uchar *bounce,\n"
            "    __global uint *seed\n"
            ") {{\n"
            "    int i = get_global_id(0);\n"
            "    Context ctx;\n"
            "    Rng rng;\n"
            "    rng.state = seed[i];\n"
            "    ctx.rng = &rng;\n"
            "    LightLocal light;\n"
            "    light.direction = idir[i];\n"
            "    float3 emission = (float3)(0.0f);\n"
            "    bounce[i] = {}_interact(NULL, &ctx, norm[i], &light, &emission);\n"
            "    odir[i] = light.direction;\n"
            "}}\n",
            src.name(),
            mty->prefix()
        ), std::move(src.files()))
        .build("interact").expect("Kernel build error");

        const int n = TEST_ATTEMPTS;
        std::vector<real3> idir(n), odir(n), norm(n);
        std::vector<uchar> bounce(n);
        std::vector<uint> seed(n);
        for (size_t i = 0; i < n; ++i) {
            norm[i] = vrng.unit();
            idir[i] = vrng.unit();
            if (dot(idir[i], norm[i]) > 0.0) {
                idir[i] = -idir[i];
            }
            seed[i] = uint(((ulong_)1<<32)*rng.uniform());
        }

        devtest::KernelRunner(queue, std::move(kernel))
        .run(n, norm, idir, odir, bounce, seed).expect("Kernel run error");

        for(size_t i = 0; i < n; ++i) {
            assert_(bool(bounce[i]));
            assert_eq_(odir[i], dev_approx(idir[i]));
        }
    }
    rtest_(specular) {
        TestRng<real> rng(0xABBA);
        TestRng<real3> vrng(0xBAAB);

        devtest::Target target = devtest_make_target();
        auto queue = target.make_queue();

        auto mty = rstd::Box<Material>(Specular());
        auto src = mty->source();
        auto kernel = devtest::KernelBuilder(target.device_id(), queue)
        .source("material_specular.cl", format_(
            "#include <{}>\n"
            "__kernel void interact(\n"
            "    __global const real3 *norm, __global const real3 *idir,\n"
            "    __global real3 *odir, __global uchar *bounce,\n"
            "    __global uint *seed\n"
            ") {{\n"
            "    int i = get_global_id(0);\n"
            "    Context ctx;\n"
            "    Rng rng;\n"
            "    rng.state = seed[i];\n"
            "    ctx.rng = &rng;\n"
            "    LightLocal light;\n"
            "    light.direction = idir[i];\n"
            "    float3 emission = (float3)(0.0f);\n"
            "    bounce[i] = {}_interact(NULL, &ctx, norm[i], &light, &emission);\n"
            "    odir[i] = light.direction;\n"
            "}}\n",
            src.name(),
            mty->prefix()
        ), std::move(src.files()))
        .build("interact").expect("Kernel build error");

        const int n = TEST_ATTEMPTS;
        std::vector<real3> idir(n), odir(n), norm(n);
        std::vector<uchar> bounce(n);
        std::vector<uint> seed(n);
        for (size_t i = 0; i < n; ++i) {
            norm[i] = vrng.unit();
            idir[i] = vrng.unit();
            if (dot(idir[i], norm[i]) > 0.0) {
                idir[i] = -idir[i];
            }
            seed[i] = uint(((ulong_)1<<32)*rng.uniform());
        }

        devtest::KernelRunner(queue, std::move(kernel))
        .run(n, norm, idir, odir, bounce, seed).expect("Kernel run error");

        for(size_t i = 0; i < n; ++i) {
            assert_(bool(bounce[i]));
            assert_eq_(length(odir[i]), dev_approx(1.0));
            assert_eq_(normalize(odir[i] - idir[i]), dev_approx(norm[i]));
        }
    }
    rtest_(lambertian) {
        TestRng<real> rng(0xABBA);
        TestRng<real3> vrng(0xBAAB);

        devtest::Target target = devtest_make_target();
        auto queue = target.make_queue();

        auto mty = rstd::Box<Material>(Lambertian());
        auto src = mty->source();
        auto kernel = devtest::KernelBuilder(target.device_id(), queue)
        .source("material_lambertian.cl", format_(
            "#include <{}>\n"
            "__kernel void interact(\n"
            "    __global const real3 *norm, __global const real3 *idir,\n"
            "    __global real3 *odir, __global uchar *bounce,\n"
            "    __global uint *seed\n"
            ") {{\n"
            "    int i = get_global_id(0);\n"
            "    Context ctx;\n"
            "    Rng rng;\n"
            "    rng.state = seed[i];\n"
            "    ctx.rng = &rng;\n"
            "    LightLocal light;\n"
            "    light.direction = idir[i];\n"
            "    float3 emission = (float3)(0.0f);\n"
            "    bounce[i] = {}_interact(NULL, &ctx, norm[i], &light, &emission);\n"
            "    odir[i] = light.direction;\n"
            "}}\n",
            src.name(),
            mty->prefix()
        ), std::move(src.files()))
        .build("interact").expect("Kernel build error");

        const int n = TEST_ATTEMPTS;
        std::vector<real3> idir(n), odir(n), norm(n);
        std::vector<uchar> bounce(n);
        std::vector<uint> seed(n);
        for (size_t i = 0; i < n; ++i) {
            norm[i] = vrng.unit();
            idir[i] = vrng.unit();
            if (dot(idir[i], norm[i]) > 0.0) {
                idir[i] = -idir[i];
            }
            seed[i] = uint(((ulong_)1<<32)*rng.uniform());
        }

        devtest::KernelRunner(queue, std::move(kernel))
        .run(n, norm, idir, odir, bounce, seed).expect("Kernel run error");

        for(size_t i = 0; i < n; ++i) {
            assert_(bool(bounce[i]));
            assert_eq_(length(odir[i]), dev_approx(1.0));
            assert_(dot(odir[i], norm[i]) > -DEV_EPS);
        }
    }
}

#endif // TEST_DEV

#endif // TEST
