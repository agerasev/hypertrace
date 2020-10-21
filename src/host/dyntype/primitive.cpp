#ifdef TEST_DEV

#include <vector>
#include <rtest.hpp>
#include <test/devtest.hpp>
#include "primitive.hpp"
#include <common/algebra/vector.hh>

extern devtest::Target devtest_make_target();

rtest_module_(dyntype_primitive) {
    rtest_(real_vector) {
        devtest::Target target = devtest_make_target();
        auto queue = target.make_queue();

        TestRng<real16> vrng(0xdeadbeef);
        Primitive<real16> dty;

        auto kernel = devtest::KernelBuilder(target.device_id(), queue)
        .source("dyntype_real16.cl", std::string(format_(
            "{}\n"
            "__kernel void unpack(__global const {} *input, __global float16 *output) {{\n"
            "    int i = get_global_id(0);\n"
            "    output[i] = input[i];\n"
            "}}\n",
            dty.source(),
            dty.name()
        )))
        .build("unpack").expect("Kernel build error");


        real16 v = vrng.normal();
        std::vector<real16> data{v};
        Primitive<real16>::Instance dval(v);

        devtest::KernelRunner(queue, std::move(kernel))
        .run(1, (Type::Instance*)&dval, data).expect("Kernel run error");

        assert_eq_(data[0], dev_approx(v));
    }
}

#endif // TEST_DEV
