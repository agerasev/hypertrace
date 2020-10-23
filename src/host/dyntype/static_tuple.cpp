#ifdef TEST_DEV

#include <vector>
#include <rtest.hpp>
#include <test/devtest.hpp>
#include <common/algebra/vector.hh>
#include "primitive.hpp"
#include "array.hpp"
#include "static_tuple.hpp"

using namespace dyn;


extern devtest::Target devtest_make_target();

rtest_module_(dyntype_static_tuple) {
    typedef StaticTuple<
        Primitive<real>,
        Primitive<real3>,
        Primitive<real16>,
        Primitive<uint>,
        Primitive<uchar>
    > TestStaticTuple;

    rtest_(store_load) {
        devtest::Target target = devtest_make_target();
        auto queue = target.make_queue();

        TestRng<real16> rng16;
        TestRng<real3> rng3;
        TestRng<real> rng;

        const size_t n = TEST_ATTEMPTS;

        std::vector<real> datar(n), outr(n);
        std::vector<real3> datar3(n), outr3(n);
        std::vector<real16> datar16(n), outr16(n);
        std::vector<uint> datau(n), outu(n);
        std::vector<uchar> datab(n), outb(n);

        std::vector<ulong_> ssize(1);
        std::vector<ulong_> dyn_offset, offset(5);

        Array<TestStaticTuple>::Instance darr;

        for (size_t i = 0; i < n; ++i) {
            real r = rng.normal();
            real3 r3 = rng3.normal();
            real16 r16 = rng16.normal();
            uint u = uint(((ulong_)1<<32)*rng.uniform());
            char b = uchar(256*rng.uniform());

            TestStaticTuple::Instance dtup = TestStaticTuple::Instance(
                rstd::Box(Primitive<real>::Instance(r)),
                rstd::Box(Primitive<real3>::Instance(r3)),
                rstd::Box(Primitive<real16>::Instance(r16)),
                rstd::Box(Primitive<uint>::Instance(u)),
                rstd::Box(Primitive<uchar>::Instance(b))
            );
            if (i == 0) {
                dyn_offset = dtup.type_().offsets();
            }
            darr.append(rstd::Box(std::move(dtup)));

            datar[i] = r;
            datar3[i] = r3;
            datar16[i] = r16;
            datau[i] = u;
            datab[i] = b;
        }

        TypeBox darrty = darr.type();
        TypeBox dtupty = darr.item_type();
        Source src = darrty->source();
        auto kernel = devtest::KernelBuilder(target.device_id(), queue)
        .source("dyntype_tuple.cl", std::string(format_(
            "#include <{}>\n"
            "#define MyTuple {}\n"
            "__kernel void unpack(\n"
            "    __global const {} *darr,\n"
            "    __global real *outr,\n"
            "    __global real3 *outr3,\n"
            "    __global real16 *outr16,\n"
            "    __global uint *outu,\n"
            "    __global uchar *outb,\n"
            "    __global ulong *ssize,\n"
            "    __global ulong *offset\n"
            ") {{\n"
            "    int i = get_global_id(0);\n"
            "    __global const MyTuple *tup = &darr->items[i];\n"
            "    outr[i] =   tup->field0;\n"
            "    outr3[i] =  tup->field1;\n"
            "    outr16[i] = tup->field2;\n"
            "    outu[i] =   tup->field3;\n"
            "    outb[i] =   tup->field4;\n"
            "    if (i == 0) {{\n"
            "        *ssize = sizeof(MyTuple);\n"
            "        offset[0] = offsetof(MyTuple, field0);\n"
            "        offset[1] = offsetof(MyTuple, field1);\n"
            "        offset[2] = offsetof(MyTuple, field2);\n"
            "        offset[3] = offsetof(MyTuple, field3);\n"
            "        offset[4] = offsetof(MyTuple, field4);\n"
            "    }}\n"
            "}}\n",
            src.name(),
            dtupty->name(),
            darrty->name()
        )), src.into_files())
        .build("unpack").expect("Kernel build error");

        devtest::KernelRunner(queue, std::move(kernel))
        .run(n, (Type::Instance*)&darr, outr, outr3, outr16, outu, outb, ssize, offset).expect("Kernel run error");

        assert_eq_(dtupty->size().unwrap(), ssize[0]);
        for (int i = 0; i < 5; ++i) {
            assert_eq_(dyn_offset[i], offset[i]);
        }

        for (size_t i = 0; i < n; ++i) {
            assert_eq_(dev_approx(datar[i]), outr[i]);
            assert_eq_(dev_approx(datar3[i]), outr3[i]);
            assert_eq_(dev_approx(datar16[i]), outr16[i]);
            assert_eq_(datau[i], outu[i]);
            assert_eq_(int(datab[i]), int(outb[i]));

            rstd::Box<TestStaticTuple::Instance> dtup = std::move(darr.items()[i]);
            assert_eq_(dev_approx(datar[i]), dtup->fields().template get<0>()->value);
            assert_eq_(dev_approx(datar3[i]), dtup->fields().template get<1>()->value);
            assert_eq_(dev_approx(datar16[i]), dtup->fields().template get<2>()->value);
            assert_eq_(datau[i], dtup->fields().template get<3>()->value);
            assert_eq_(int(datab[i]), int(dtup->fields().template get<4>()->value));
        }
    }
}

#endif // TEST_DEV
