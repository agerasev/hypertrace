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
            includer includer(
                "test.cl", std::list<std::string>{"src"},
                std::map<std::string, std::string>{std::make_pair("test.cl", std::string("\n\
                    #include <common/algebra/real.hh>\n\
                    __kernel void square(__global const real *ibuf, __global real *obuf) {\n\
                        int i = get_global_id(0);\n\
                        real x = ibuf[i];\n\
                        obuf[i] = x*x;\n\
                    }\n\
                "))},
                std::map<std::string, bool>{
                    std::make_pair("HOST", false),
                    std::make_pair("TEST_UNIT", false),
                    std::make_pair("TEST_DEV", false)
                }
            );
            assert(includer.include());
            println_(includer.log());
            std::fstream("build/real.cl", std::ios::out) << includer.data();

            auto prog_log = cl::Program::create(queue->context_ref(), target.device_id(), includer);
            println_(prog_log.get<1>());
            auto program = rstd::Rc<cl::Program>(prog_log.get<0>().expect("Program build error"));
            auto kernel = cl::Kernel::create(program, "square").expect("Kernel create error");

            const int n = 256;
            auto ibuf = cl::Buffer::create(*queue, n*sizeof(dev_type<real>)).unwrap();
            auto obuf = cl::Buffer::create(*queue, n*sizeof(dev_type<real>), true).unwrap();
            std::vector<dev_type<real>> host_buf(n);
            for (int i = 0; i < int(host_buf.size()); ++i) {
                real x = (real)i;
                dev_store(&host_buf[i], &x);
            }
            ibuf.store(*queue, host_buf.data()).expect("Buffer store error");
            kernel(*queue, n, ibuf, obuf).expect("Kernel run error");
            obuf.load(*queue, host_buf.data()).expect("Buffer load error");
            for(int i = 0; i < int(host_buf.size()); ++i) {
                real x;
                dev_load(&x, &host_buf[i]);
                assert_eq_(i*i, (int)x);
            }

        }
    }
}

#endif // TEST_DEV
