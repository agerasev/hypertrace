#ifdef TEST_DEV

#include <vector>
#include <rtest.hpp>
#include <test/devtest.hpp>
#include "primitive.hpp"
#include <common/algebra/vector.hh>

using namespace dyn;


extern devtest::Target devtest_make_target();

rtest_module_(dyntype_primitive) {
    rtest_(store_load) {
        devtest::Target target = devtest_make_target();
        auto queue = target.make_queue();

        TestRng<real16> vrng(0xdeadbeef);
        real16 v = vrng.normal();
        std::vector<real16> out(1);
        Primitive<real16>::Instance dval(v);

        TypeBox dty = dval.type();
        auto kernel = devtest::KernelBuilder(target.device_id(), queue)
        .source("dyntype_primitive.cl", std::string(format_(
            "{}\n"
            "__kernel void unpack(__global const {} *dval, __global float16 *out) {{\n"
            "    int i = get_global_id(0);\n"
            "    out[i] = dval[i];\n"
            "}}\n",
            dty->source(),
            dty->name()
        )))
        .build("unpack").expect("Kernel build error");

        devtest::KernelRunner(queue, std::move(kernel))
        .run(1, (Type::Instance*)&dval, out).expect("Kernel run error");

        assert_eq_(out[0], dev_approx(v));
        assert_eq_(dval.value, dev_approx(v));
    }
}

#endif // TEST_DEV
