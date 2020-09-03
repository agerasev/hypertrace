#include "real.hh"


#ifdef TEST_DEV

#include <vector>
#include <fstream>
#include <cassert>

#include <host/opencl/search.hpp>
#include <host/opencl/opencl.hpp>


void test_dev_real(
    cl_device_id device,
    core::Rc<cl::Queue> queue
) {
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
    std::cout << includer.log() << std::endl;
    std::fstream("build/real.cl", std::ios::out) << includer.data();

    auto prog_log = cl::Program::create(queue->context_ref(), device, includer);
    std::cout << prog_log.get<1>() << std::endl;
    auto program = core::Rc<cl::Program>(prog_log.get<0>().unwrap());
    auto kernel = cl::Kernel::create(program, "square").unwrap();

    const int n = 256;
    auto ibuf = cl::Buffer::create(*queue, n*sizeof(dev_type<real>)).unwrap();
    auto obuf = cl::Buffer::create(*queue, n*sizeof(dev_type<real>), true).unwrap();
    std::vector<dev_type<real>> host_buf(n);
    for (int i = 0; i < int(host_buf.size()); ++i) {
        real x = (real)i;
        dev_store(&host_buf[i], &x);
    }
    ibuf.store(*queue, host_buf.data());
    kernel.set_arg(0, ibuf);
    kernel.set_arg(1, obuf);
    kernel.run(*queue, n);
    obuf.load(*queue, host_buf.data());
    for(int i = 0; i < int(host_buf.size()); ++i) {
        real x;
        dev_load(&x, &host_buf[i]);
        assert(i*i == (int)x);
    }
}

#endif // TEST_DEV
