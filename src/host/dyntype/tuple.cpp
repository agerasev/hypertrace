#ifdef TEST_DEV

#include <vector>
#include <rtest.hpp>
#include <test/devtest.hpp>
#include <common/algebra/vector.hh>
#include "primitive.hpp"
#include "array.hpp"
#include "tuple.hpp"

using namespace dyn;


extern devtest::Target devtest_make_target();

rtest_module_(dyntype_tuple) {
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

        Array::Instance darr;

        for (size_t i = 0; i < n; ++i) {
            real r = rng.normal();
            real3 r3 = rng3.normal();
            real16 r16 = rng16.normal();
            uint u = uint(((ulong_)1<<32)*rng.uniform());
            char b = uchar(256*rng.uniform());

            Tuple::Instance dtup;
            dtup.append(InstanceBox(Primitive<real>::Instance(r)));
            dtup.append(InstanceBox(Primitive<real3>::Instance(r3)));
            dtup.append(InstanceBox(Primitive<real16>::Instance(r16)));
            dtup.append(InstanceBox(Primitive<uint>::Instance(u)));
            dtup.append(InstanceBox(Primitive<uchar>::Instance(b)));
            if (i == 0) {
                dyn_offset = dtup.type_().offsets();
            }
            darr.append(InstanceBox(std::move(dtup)));

            datar[i] = r;
            datar3[i] = r3;
            datar16[i] = r16;
            datau[i] = u;
            datab[i] = b;
        }

        TypeBox darrty = darr.type();
        TypeBox dtupty = darr.item_type();
        auto kernel = devtest::KernelBuilder(target.device_id(), queue)
        .source("dyntype_tuple.cl", std::string(format_(
            "{}\n"
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
            darrty->source(),
            dtupty->name(),
            darrty->name()
        )))
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

            rstd::Box<Tuple::Instance> dtup = darr.items()[i].template downcast<Tuple::Instance>().unwrap();
            assert_eq_(dev_approx(datar[i]), dtup->fields()[0].template downcast<Primitive<real>::Instance>().unwrap()->value);
            assert_eq_(dev_approx(datar3[i]), dtup->fields()[1].template downcast<Primitive<real3>::Instance>().unwrap()->value);
            assert_eq_(dev_approx(datar16[i]), dtup->fields()[2].template downcast<Primitive<real16>::Instance>().unwrap()->value);
            assert_eq_(datau[i], dtup->fields()[3].template downcast<Primitive<uint>::Instance>().unwrap()->value);
            assert_eq_(int(datab[i]), int(dtup->fields()[4].template downcast<Primitive<uchar>::Instance>().unwrap()->value));
        }
    }
}

#endif // TEST_DEV
