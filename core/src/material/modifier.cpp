#ifdef TEST_DEV

#include "modifier.hpp"

#include <rtest.hpp>
#include <vector>
#include <host/dyntype/vector.hpp>
#include <test/devtest.hpp>

using namespace rstd;


extern devtest::Target devtest_make_target();

rtest_module_(material_modifier) {
    rtest_(colored) {
        TestRng<real> rng(0xABBA);
        TestRng<real3> vrng(0xBAAB);

        devtest::Target target = devtest_make_target();
        auto queue = target.make_queue();

        auto mty = rs::Box<Colored>(Colored(rs::Box(Transparent()), rs::Box(dyn::Primitive<float3>())));
        auto darr = dyn::Vector<Material>::Instance(mty->clone());

        const int n = TEST_ATTEMPTS;
        std::vector<real3> idir(n), odir(n, real3(0.0)), norm(n);
        std::vector<float3> lcol(n), icol(n), ocol(n, float3(0.0f)), emi(n, float3(0.0f));
        std::vector<uint> seed(n);
        for (size_t i = 0; i < n; ++i) {
            float3 color = vrng.unit().convert<float>();
            Colored::Instance dmat = Colored::Instance(
                rs::Box<Transparent::Instance>(Transparent::Instance()),
                rs::Box<dyn::Primitive<float3>::Instance>(dyn::Primitive<float3>::Instance(color))
            );
            lcol[i] = color;

            norm[i] = vrng.unit();
            idir[i] = vrng.unit();
            if (dot(idir[i], norm[i]) > 0.0) {
                idir[i] = -idir[i];
            }
            icol[i] = vrng.unit().convert<float>();
            seed[i] = uint(((ulong_)1<<32)*rng.uniform());

            darr.append(rs::Box<Colored::Instance>(std::move(dmat)));
        }

        auto aty = darr.type();
        auto src = aty->source();
        auto kernel = devtest::KernelBuilder(target.device_id(), queue)
        .source("material_modifier_colored.cl", format_(
            "#include <{}>\n"
            "__kernel void interact(\n"
            "    __global uint *seed, __global const {} *materials,\n"
            "    __global const real3 *norm, __global const real3 *idir, __global real3 *odir,\n"
            "    __global const float3 *icol, __global float3 *ocol, __global float3 *emi\n"
            ") {{\n"
            "    int i = get_global_id(0);\n"
            "    Context ctx;\n"
            "    Rng rng;\n"
            "    rng.state = seed[i];\n"
            "    ctx.rng = &rng;\n"
            "    LightLocal light;\n"
            "    light.direction = idir[i];\n"
            "    light.base.intensity = icol[i];\n"
            "    float3 emission = (float3)(0.0f);\n"
            "    {}_interact(&(&materials->first)[i], &ctx, norm[i], &light, &emission);\n"
            "    odir[i] = light.direction;\n"
            "    ocol[i] = light.base.intensity;\n"
            "    emi[i] = emission;\n"
            "}}\n",
            src.name(),
            aty->name(),
            mty->prefix()
        ), std::move(src.files()))
        .build("interact").expect("Kernel build error");

        devtest::KernelRunner(queue, std::move(kernel))
        .run(n, seed, (dyn::Type::Instance*)&darr, norm, idir, odir, icol, ocol, emi).expect("Kernel run error");

        for(size_t i = 0; i < n; ++i) {
            assert_eq_(idir[i], dev_approx(odir[i]));
            assert_eq_(lcol[i], dev_approx(darr.items()[i].template downcast_ref<Colored::Instance>().unwrap()->data().value));
            assert_eq_(icol[i]*lcol[i], dev_approx(ocol[i]));
            assert_eq_(emi[i], dev_approx(float3(0.0f)));
        }
    }
    rtest_(emissive) {
        TestRng<real> rng(0xABBA);
        TestRng<real3> vrng(0xBAAB);

        devtest::Target target = devtest_make_target();
        auto queue = target.make_queue();

        auto mty = rs::Box<Emissive>(Emissive(rs::Box(Transparent()), rs::Box(dyn::Primitive<float3>())));
        auto darr = dyn::Vector<Material>::Instance(mty->clone());

        const int n = TEST_ATTEMPTS;
        std::vector<real3> idir(n), odir(n, real3(0.0)), norm(n);
        std::vector<float3> lcol(n), icol(n), ocol(n, float3(0.0f)), emi(n, float3(0.0f));
        std::vector<uint> seed(n);
        for (size_t i = 0; i < n; ++i) {
            float3 color = vrng.unit().convert<float>();
            Emissive::Instance dmat = Emissive::Instance(
                rs::Box<Transparent::Instance>(Transparent::Instance()),
                rs::Box<dyn::Primitive<float3>::Instance>(dyn::Primitive<float3>::Instance(color))
            );
            lcol[i] = color;

            norm[i] = vrng.unit();
            idir[i] = vrng.unit();
            if (dot(idir[i], norm[i]) > 0.0) {
                idir[i] = -idir[i];
            }
            icol[i] = vrng.unit().convert<float>();
            seed[i] = uint(((ulong_)1<<32)*rng.uniform());

            darr.append(rs::Box<Emissive::Instance>(std::move(dmat)));
        }

        auto aty = darr.type();
        auto src = aty->source();
        auto kernel = devtest::KernelBuilder(target.device_id(), queue)
        .source("material_modifier_emissive.cl", format_(
            "#include <{}>\n"
            "__kernel void interact(\n"
            "    __global uint *seed, __global const {} *materials,\n"
            "    __global const real3 *norm, __global const real3 *idir, __global real3 *odir,\n"
            "    __global const float3 *icol, __global float3 *ocol, __global float3 *emi\n"
            ") {{\n"
            "    int i = get_global_id(0);\n"
            "    Context ctx;\n"
            "    Rng rng;\n"
            "    rng.state = seed[i];\n"
            "    ctx.rng = &rng;\n"
            "    LightLocal light;\n"
            "    light.direction = idir[i];\n"
            "    light.base.intensity = icol[i];\n"
            "    float3 emission = (float3)(0.0f);\n"
            "    {}_interact(&(&materials->first)[i], &ctx, norm[i], &light, &emission);\n"
            "    odir[i] = light.direction;\n"
            "    ocol[i] = light.base.intensity;\n"
            "    emi[i] = emission;\n"
            "}}\n",
            src.name(),
            aty->name(),
            mty->prefix()
        ), std::move(src.files()))
        .build("interact").expect("Kernel build error");

        devtest::KernelRunner(queue, std::move(kernel))
        .run(n, seed, (dyn::Type::Instance*)&darr, norm, idir, odir, icol, ocol, emi).expect("Kernel run error");

        for(size_t i = 0; i < n; ++i) {
            assert_eq_(idir[i], dev_approx(odir[i]));
            assert_eq_(icol[i], dev_approx(ocol[i]));
            assert_eq_(lcol[i]*icol[i], dev_approx(emi[i]));
            assert_eq_(lcol[i], dev_approx(darr.items()[i].template downcast_ref<Emissive::Instance>().unwrap()->data().value));
        }
    }
}

#endif // TEST_DEV
