#ifdef TEST_DEV

#include <vector>
#include <rtest.hpp>
#include <test/devtest.hpp>
#include <common/algebra/vector.hh>
#include "primitive.hpp"
#include "array.hpp"

using namespace dyn;


extern devtest::Target devtest_make_target();

rtest_module_(dyntype_array) {
    rtest_(store_load) {
        devtest::Target target = devtest_make_target();
        auto queue = target.make_queue();

        const size_t n = TEST_ATTEMPTS;
        TestRng<real16> vrng(0xdeadbeef);
        std::vector<real16> data(n), out(n);
        Array<>::Instance darr;
        for (size_t i = 0; i < n; ++i) {
            real16 v = vrng.normal();
            darr.append(InstanceBox(Primitive<real16>::Instance(v)));
            data[i] = v;
        }

        TypeBox dty = darr.type();
        Source src = dty->source();
        auto kernel = devtest::KernelBuilder(target.device_id(), queue)
        .source("dyntype_array.cl", std::string(format_(
            "#include <{}>\n"
            "__kernel void unpack(__global const {} *darr, __global real16 *out) {{\n"
            "    int i = get_global_id(0);\n"
            "    out[i] = darr->items[i];\n"
            "}}\n",
            src.name(),
            dty->name()
        )), src.into_files())
        .build("unpack").expect("Kernel build error");

        devtest::KernelRunner(queue, std::move(kernel))
        .run(n, (Type::Instance*)&darr, out).expect("Kernel run error");

        for (size_t i = 0; i < n; ++i) {
            assert_eq_(out[i], dev_approx(data[i]));
            real16 v = darr.items()[i].template downcast<Primitive<real16>::Instance>().unwrap()->value;
            assert_eq_(v, dev_approx(data[i]));
        }
    }
}

#endif // TEST_DEV
