#ifdef TEST_DEV

#include <vector>
#include <rtest.hpp>
#include <test/devtest.hpp>
#include <common/algebra/vector.hh>
#include "primitive.hpp"
#include "vector.hpp"
#include "variant.hpp"

using namespace dyn;


extern devtest::Target devtest_make_target();

rtest_module_(dyntype_variant) {
    rtest_(store) {
        devtest::Target target = devtest_make_target();
        auto queue = target.make_queue();

        TestRng<real16> rng16;
        TestRng<real3> rng3;
        TestRng<real> rng;

        const size_t n = TEST_ATTEMPTS;

        std::vector<ulong_> idxs(n), outidxs(n);
        std::vector<real> datar(n), outr(n);
        std::vector<real3> datar3(n), outr3(n);
        std::vector<real16> datar16(n), outr16(n);
        std::vector<uint> datau(n), outu(n);
        std::vector<uchar> datab(n), outb(n);
        std::vector<ulong_> ssize(1);

        Variant<> dty;
        dty.append_vartype(TypeBox(Primitive<real>()));
        dty.append_vartype(TypeBox(Primitive<real3>()));
        dty.append_vartype(TypeBox(Primitive<real16>()));
        dty.append_vartype(TypeBox(Primitive<uint>()));
        dty.append_vartype(TypeBox(Primitive<uchar>()));
        
        Vector<Variant<>>::Instance darr;

        for (size_t i = 0; i < n; ++i) {
            size_t idx = int(5*rng.uniform());
            Variant<>::Instance dvar = dty.instance_();

            InstanceBox dprm;
            if (idx == 0) {
                real r = rng.normal();
                dprm = InstanceBox(Primitive<real>::Instance(r));
                datar[i] = r;
            } else if (idx == 1) {
                real3 r3 = rng3.normal();
                dprm = InstanceBox(Primitive<real3>::Instance(r3));
                datar3[i] = r3;
            } else if (idx == 2) {
                real16 r16 = rng16.normal();
                dprm = InstanceBox(Primitive<real16>::Instance(r16));
                datar16[i] = r16;
            } else if (idx == 3) {
                uint u = uint(((ulong_)1<<32)*rng.uniform());
                dprm = InstanceBox(Primitive<uint>::Instance(u));
                datau[i] = u;
            } else if (idx == 4) {
                char b = uchar(256*rng.uniform());
                dprm = InstanceBox(Primitive<uchar>::Instance(b));
                datab[i] = b;
            } else {
                panic_("Bad index: {}", idx);
            }
            
            dvar.set_value(idx, std::move(dprm));
            idxs[i] = idx;

            darr.append(rs::Box(std::move(dvar)));
        }

        TypeBox darrty = darr.type();
        Source src = darrty->source();
        auto kernel = devtest::KernelBuilder(target.device_id(), queue)
        .source("dyntype_variant_in.cl", std::string(format_(
            "#include <{}>\n"
            "#define MyVar {}\n"
            "__kernel void unpack(\n"
            "    __global const {} *darr,\n"
            "    __global ulong *index,\n"
            "    __global real *outr,\n"
            "    __global real3 *outr3,\n"
            "    __global real16 *outr16,\n"
            "    __global uint *outu,\n"
            "    __global uchar *outb,\n"
            "    __global ulong *ssize\n"
            ") {{\n"
            "    int i = get_global_id(0);\n"
            "    __global const MyVar *var = &(&darr->first)[i];\n"
            "    index[i] = var->index;\n"
            "    if (var->index == 0) {{\n"
            "        outr[i] = var->variants.variant0;\n"
            "    }} else if (var->index == 1) {{\n"
            "        outr3[i] = var->variants.variant1;\n"
            "    }} else if (var->index == 2) {{\n"
            "        outr16[i] = var->variants.variant2;\n"
            "    }} else if (var->index == 3) {{\n"
            "        outu[i] = var->variants.variant3;\n"
            "    }} else if (var->index == 4) {{\n"
            "        outb[i] = var->variants.variant4;\n"
            "    }}\n"
            "    if (i == 0) {{\n"
            "        *ssize = sizeof(MyVar);\n"
            "    }}\n"
            "}}\n",
            src.name(),
            dty.name(),
            darrty->name()
        )), src.into_files())
        .build("unpack").expect("Kernel build error");

        devtest::KernelRunner(queue, std::move(kernel))
        .run(n, (Type::Instance*)&darr, outidxs, outr, outr3, outr16, outu, outb, ssize).expect("Kernel run error");

        assert_eq_(dty.size().unwrap(), ssize[0]);

        for (size_t i = 0; i < n; ++i) {
            rs::Box<Variant<>::Instance> dvar = std::move(darr.items()[i]);
            size_t idx = idxs[i];
            assert_eq_(dvar->index(), idx);

            if (idx == 0) {
                assert_eq_(dev_approx(datar[i]), outr[i]);
                assert_eq_(dev_approx(datar[i]), dvar->value().template downcast<Primitive<real>::Instance>().unwrap()->value);
            } else if (idx == 1) {
                assert_eq_(dev_approx(datar3[i]), outr3[i]);
                assert_eq_(dev_approx(datar3[i]), dvar->value().template downcast<Primitive<real3>::Instance>().unwrap()->value);
            } else if (idx == 2) {
                assert_eq_(dev_approx(datar16[i]), outr16[i]);
                assert_eq_(dev_approx(datar16[i]), dvar->value().template downcast<Primitive<real16>::Instance>().unwrap()->value);
            } else if (idx == 3) {
                assert_eq_(datau[i], outu[i]);
                assert_eq_(dev_approx(datau[i]), dvar->value().template downcast<Primitive<uint>::Instance>().unwrap()->value);
            } else if (idx == 4) {
                assert_eq_(int(datab[i]), int(outb[i]));
                assert_eq_(dev_approx(datab[i]), dvar->value().template downcast<Primitive<uchar>::Instance>().unwrap()->value);
            } else {
                panic_("Bad index: {}", idx);
            }
        }
    }
    rtest_(load) {
        devtest::Target target = devtest_make_target();
        auto queue = target.make_queue();

        TestRng<real16> rng16;
        TestRng<real3> rng3;
        TestRng<real> rng;

        const size_t n = TEST_ATTEMPTS;

        std::vector<ulong_> idxs(n), inidxs;
        std::vector<real> datar(n), inr;
        std::vector<real3> datar3(n), inr3;
        std::vector<real16> datar16(n), inr16;
        std::vector<uint> datau(n), inu;
        std::vector<uchar> datab(n), inb;

        Variant<> dty;
        dty.append_vartype(TypeBox(Primitive<real>()));
        dty.append_vartype(TypeBox(Primitive<real3>()));
        dty.append_vartype(TypeBox(Primitive<real16>()));
        dty.append_vartype(TypeBox(Primitive<uint>()));
        dty.append_vartype(TypeBox(Primitive<uchar>()));
        
        Vector<Variant<>>::Instance darr;

        for (size_t i = 0; i < n; ++i) {
            size_t idx = int(5*rng.uniform());
            Variant<>::Instance dvar = dty.instance_();
            dvar.set_value(0, InstanceBox(Primitive<real>::Instance(0.0)));

            InstanceBox dprm;
            if (idx == 0) {
                real r = rng.normal();
                datar[i] = r;
            } else if (idx == 1) {
                real3 r3 = rng3.normal();
                datar3[i] = r3;
            } else if (idx == 2) {
                real16 r16 = rng16.normal();
                datar16[i] = r16;
            } else if (idx == 3) {
                uint u = uint(((ulong_)1<<32)*rng.uniform());
                datau[i] = u;
            } else if (idx == 4) {
                char b = uchar(256*rng.uniform());
                datab[i] = b;
            } else {
                panic_("Bad index: {}", idx);
            }
            
            idxs[i] = idx;
            darr.append(rs::Box(std::move(dvar)));
        }
        inidxs = idxs;
        inr = datar;
        inr3 = datar3;
        inr16 = datar16;
        inu = datau;
        inb = datab;

        TypeBox darrty = darr.type();
        Source src = darrty->source();
        auto kernel = devtest::KernelBuilder(target.device_id(), queue)
        .source("dyntype_variant_in.cl", std::string(format_(
            "#include <{}>\n"
            "#define MyVar {}\n"
            "__kernel void unpack(\n"
            "    __global {} *darr,\n"
            "    __global const ulong *index,\n"
            "    __global const real *inr,\n"
            "    __global const real3 *inr3,\n"
            "    __global const real16 *inr16,\n"
            "    __global const uint *inu,\n"
            "    __global const uchar *inb\n"
            ") {{\n"
            "    int i = get_global_id(0);\n"
            "    __global MyVar *var = &(&darr->first)[i];\n"
            "    var->index = index[i];\n"
            "    if (var->index == 0) {{\n"
            "        var->variants.variant0 = inr[i];\n"
            "    }} else if (var->index == 1) {{\n"
            "        var->variants.variant1 = inr3[i];\n"
            "    }} else if (var->index == 2) {{\n"
            "        var->variants.variant2 = inr16[i];\n"
            "    }} else if (var->index == 3) {{\n"
            "        var->variants.variant3 = inu[i];\n"
            "    }} else if (var->index == 4) {{\n"
            "        var->variants.variant4 = inb[i];\n"
            "    }}\n"
            "}}\n",
            src.name(),
            dty.name(),
            darrty->name()
        )), src.into_files())
        .build("unpack").expect("Kernel build error");

        devtest::KernelRunner(queue, std::move(kernel))
        .run(n, (Type::Instance*)&darr, inidxs, inr, inr3, inr16, inu, inb).expect("Kernel run error");

        for (size_t i = 0; i < n; ++i) {
            rs::Box<Variant<>::Instance> dvar = std::move(darr.items()[i]);
            size_t idx = idxs[i];
            assert_eq_(idx, dvar->index());

            if (idx == 0) {
                assert_eq_(dev_approx(datar[i]), dvar->value().template downcast<Primitive<real>::Instance>().unwrap()->value);
            } else if (idx == 1) {
                assert_eq_(dev_approx(datar3[i]), dvar->value().template downcast<Primitive<real3>::Instance>().unwrap()->value);
            } else if (idx == 2) {
                assert_eq_(dev_approx(datar16[i]), dvar->value().template downcast<Primitive<real16>::Instance>().unwrap()->value);
            } else if (idx == 3) {
                assert_eq_(dev_approx(datau[i]), dvar->value().template downcast<Primitive<uint>::Instance>().unwrap()->value);
            } else if (idx == 4) {
                assert_eq_(dev_approx(datab[i]), dvar->value().template downcast<Primitive<uchar>::Instance>().unwrap()->value);
            } else {
                panic_("Bad index: {}", idx);
            }
        }
    }
}

#endif // TEST_DEV
