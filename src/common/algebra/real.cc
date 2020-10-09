#include "real.hh"


#ifdef TEST_UNIT

#include <rtest.hpp>

rtest_module_(real) {
    rtest_(approx) {
        assert_eq_(R1 + EPS/2, approx(R1));
    }
}

#endif // TEST_UNIT

#ifdef TEST_DEV

#include <rtest.hpp>

#include <vector>
#include <fstream>

#include <host/opencl/search.hpp>
#include <host/opencl/opencl.hpp>

#include <test/devtest.hpp>

extern_lazy_static_(devtest::Selector, devtest_selector);

rtest_module_(real) {
    rtest_(square) {
        for (devtest::Target target : *devtest_selector) {
            auto queue = target.make_queue();
            auto kernel = devtest::KernelBuilder(target.device_id(), queue)
            .source("real.cl", std::string(
                "#include <common/algebra/real.hh>\n"
                "__kernel void square(__global const real *ibuf, __global real *obuf) {\n"
                "    int i = get_global_id(0);\n"
                "    real x = ibuf[i];\n"
                "    obuf[i] = x*x;\n"
                "}\n"
            ))
            .build("square").unwrap();

            const int n = 256;
            std::vector<real> ibuf(n), obuf(n);
            for (size_t i = 0; i < ibuf.size(); ++i) {
                ibuf[i] = real(i);
            }

            devtest::KernelRunner(queue, std::move(kernel))
            .run(n, ibuf, obuf).expect("Kernel run error");

            for(size_t i = 0; i < obuf.size(); ++i) {
                assert_eq_(i*i, size_t(obuf[i]));
            }

        }
    }
}

#endif // TEST_DEV
