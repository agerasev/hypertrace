#ifdef TEST_DEV

#include <vector>
#include <rtest.hpp>
#include <test/devtest.hpp>
#include <common/algebra/vector.hh>
#include "primitive.hpp"
#include "array.hpp"
#include "struct.hpp"

using namespace dyn;


extern devtest::Target devtest_make_target();

rtest_module_(dyntype_struct) {
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

        Array<Struct<>>::Instance darr;

        for (size_t i = 0; i < n; ++i) {
            real r = rng.normal();
            real3 r3 = rng3.normal();
            real16 r16 = rng16.normal();
            uint u = uint(((ulong_)1<<32)*rng.uniform());
            char b = uchar(256*rng.uniform());

            Struct<>::Instance dinst;
            dinst.append("r", InstanceBox(Primitive<real>::Instance(r)));
            dinst.append("r3", InstanceBox(Primitive<real3>::Instance(r3)));
            dinst.append("r16", InstanceBox(Primitive<real16>::Instance(r16)));
            dinst.append("u", InstanceBox(Primitive<uint>::Instance(u)));
            dinst.append("b", InstanceBox(Primitive<uchar>::Instance(b)));
            if (i == 0) {
                dyn_offset = dinst.type_().offsets();
            }
            darr.append(rs::Box(std::move(dinst)));

            datar[i] = r;
            datar3[i] = r3;
            datar16[i] = r16;
            datau[i] = u;
            datab[i] = b;
        }

        TypeBox darrty = darr.type();
        TypeBox dty = darr.item_type();
        Source src = darrty->source();
        auto kernel = devtest::KernelBuilder(target.device_id(), queue)
        .source("dyntype_tuple.cl", std::string(format_(
            "#include <{}>\n"
            "#define MyStruct {}\n"
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
            "    __global const MyStruct *tup = &darr->items[i];\n"
            "    outr[i] =   tup->r;\n"
            "    outr3[i] =  tup->r3;\n"
            "    outr16[i] = tup->r16;\n"
            "    outu[i] =   tup->u;\n"
            "    outb[i] =   tup->b;\n"
            "    if (i == 0) {{\n"
            "        *ssize = sizeof(MyStruct);\n"
            "        offset[0] = offsetof(MyStruct, r);\n"
            "        offset[1] = offsetof(MyStruct, r3);\n"
            "        offset[2] = offsetof(MyStruct, r16);\n"
            "        offset[3] = offsetof(MyStruct, u);\n"
            "        offset[4] = offsetof(MyStruct, b);\n"
            "    }}\n"
            "}}\n",
            src.name(),
            dty->name(),
            darrty->name()
        )), src.into_files())
        .build("unpack").expect("Kernel build error");

        devtest::KernelRunner(queue, std::move(kernel))
        .run(n, (Type::Instance*)&darr, outr, outr3, outr16, outu, outb, ssize, offset).expect("Kernel run error");

        assert_eq_(dty->size().unwrap(), ssize[0]);
        for (int i = 0; i < 5; ++i) {
            assert_eq_(dyn_offset[i], offset[i]);
        }

        for (size_t i = 0; i < n; ++i) {
            assert_eq_(dev_approx(datar[i]), outr[i]);
            assert_eq_(dev_approx(datar3[i]), outr3[i]);
            assert_eq_(dev_approx(datar16[i]), outr16[i]);
            assert_eq_(datau[i], outu[i]);
            assert_eq_(int(datab[i]), int(outb[i]));

            rs::Box<Struct<>::Instance> dinst = std::move(darr.items()[i]);
            assert_eq_(dev_approx(datar[i]), dinst->fields()[0].template downcast<Primitive<real>::Instance>().unwrap()->value);
            assert_eq_(dev_approx(datar3[i]), dinst->fields()[1].template downcast<Primitive<real3>::Instance>().unwrap()->value);
            assert_eq_(dev_approx(datar16[i]), dinst->fields()[2].template downcast<Primitive<real16>::Instance>().unwrap()->value);
            assert_eq_(datau[i], dinst->fields()[3].template downcast<Primitive<uint>::Instance>().unwrap()->value);
            assert_eq_(int(datab[i]), int(dinst->fields()[4].template downcast<Primitive<uchar>::Instance>().unwrap()->value));
        }
    }
}

#endif // TEST_DEV
