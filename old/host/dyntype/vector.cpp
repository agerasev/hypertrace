#ifdef TEST_DEV

#include <vector>
#include <rtest.hpp>
#include <test/devtest.hpp>
#include <common/algebra/vector.hh>
#include "primitive.hpp"
#include "vector.hpp"

using namespace dyn;


extern devtest::Target devtest_make_target();

rtest_module_(dyntype_vector) {
    rtest_(store_load_real16) {
        devtest::Target target = devtest_make_target();
        auto queue = target.make_queue();

        const size_t n = TEST_ATTEMPTS;
        TestRng<real16> vrng(0xdeadbeef);
        std::vector<real16> data(n), out(n, real16(0.0));
        std::vector<ulong_> vsize(1);
        Vector<>::Instance darr;
        const size_t m = n - n/8;
        for (size_t i = 0; i < m; ++i) {
            real16 v = vrng.normal();
            darr.append(InstanceBox(Primitive<real16>::Instance(v)));
            data[i] = v;
        }

        TypeBox dty = darr.type();
        Source src = dty->source();
        std::string lname = rs::to_lower(dty->name());
        auto kernel = devtest::KernelBuilder(target.device_id(), queue)
        .source("dyntype_vector_real16.cl", std::string(format_(
            "#include <{}>\n"
            "__kernel void unpack(__global const {} *darr, __global real16 *out, __global ulong *vsize) {{\n"
            "    int i = get_global_id(0);\n"
            "    if (i == 0) {{\n"
            "        *vsize = {}_size(darr);\n"
            "    }}\n"
            "    if (i < {}_size(darr)) {{\n"
            "        out[i] = *{}_get(darr, i);\n"
            "    }}\n"
            "}}\n",
            src.name(),
            dty->name(), lname, lname, lname
        )), src.into_files())
        .build("unpack").expect("Kernel build error");

        devtest::KernelRunner(queue, std::move(kernel))
        .run(n, (Type::Instance*)&darr, out, vsize).expect("Kernel run error");

        assert_eq_(darr.items().size(), m);
        assert_eq_(m, vsize[0]);
        for (size_t i = 0; i < m; ++i) {
            assert_eq_(out[i], dev_approx(data[i]));
            real16 v = darr.items()[i].template downcast<Primitive<real16>::Instance>().unwrap()->value;
            assert_eq_(v, dev_approx(data[i]));
        }
        for (size_t i = m; i < n; ++i) {
            assert_eq_(out[i], dev_approx(real16(0.0)));
        }
    }
    rtest_(store_load_uchar) {
        devtest::Target target = devtest_make_target();
        auto queue = target.make_queue();

        const size_t n = TEST_ATTEMPTS;
        TestRng<real> rng(0xdeadbeef);
        std::vector<uchar> data(n), out(n, 1);
        std::vector<ulong_> vsize(1);
        Vector<>::Instance darr;
        const size_t m = n - n/8;
        for (size_t i = 0; i < m; ++i) {
            uchar v = uchar(256*rng.uniform());
            darr.append(InstanceBox(Primitive<uchar>::Instance(v)));
            data[i] = v;
        }

        TypeBox dty = darr.type();
        Source src = dty->source();
        std::string lname = rs::to_lower(dty->name());
        auto kernel = devtest::KernelBuilder(target.device_id(), queue)
        .source("dyntype_vector_uchar.cl", std::string(format_(
            "#include <{}>\n"
            "__kernel void unpack(__global const {} *darr, __global uchar *out, __global ulong *vsize) {{\n"
            "    int i = get_global_id(0);\n"
            "    if (i == 0) {{\n"
            "        *vsize = {}_size(darr);\n"
            "    }}\n"
            "    if (i < {}_size(darr)) {{\n"
            "        out[i] = *{}_get(darr, i);\n"
            "    }}\n"
            "}}\n",
            src.name(),
            dty->name(), lname, lname, lname
        )), src.into_files())
        .build("unpack").expect("Kernel build error");

        devtest::KernelRunner(queue, std::move(kernel))
        .run(n, (Type::Instance*)&darr, out, vsize).expect("Kernel run error");

        assert_eq_(darr.items().size(), m);
        assert_eq_(m, vsize[0]);
        for (size_t i = 0; i < m; ++i) {
            assert_eq_(int(out[i]), int(data[i]));
            uchar v = darr.items()[i].template downcast<Primitive<uchar>::Instance>().unwrap()->value;
            assert_eq_(int(v), int(data[i]));
        }
        for (size_t i = m; i < n; ++i) {
            assert_eq_(int(out[i]), int(1));
        }
    }
}

#endif // TEST_DEV
