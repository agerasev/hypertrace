#include "real.hh"


#ifdef TEST_DEV

#include <vector>
#include <cassert>

#include <host/opencl/search.hpp>
#include <host/opencl/opencl.hpp>


void test_dev_real(
    cl_device_id device,
    cl_context context,
    cl_command_queue queue
) {
    c_includer includer(
        "test.cl", std::list<std::string>{"src"},
        std::map<std::string, std::string>{std::make_pair("test.cl", std::string(" \n\
            #include <common/algebra/real.hh> \n\
            __kernel void square(__global const real *ibuf, __global real *obuf) { \n\
                int i = get_global_id(0); \n\
                real x = ibuf[i]; \n\
                obuf[i] = x*x; \n\
            } \n\
        "))}
    );
    assert(includer.include());
    std::fstream("build/test/real.cl", std::ios::out) << includer.data();
    //std::cout << includer.data() << std::endl;

    cl::Program program(context, device, includer);
    cl::Kernel kernel(program, "square");

    const int n = 256;
    cl::Buffer ibuf(queue, n*sizeof(dev_type<real>)), obuf(queue, n*sizeof(dev_type<real>), true);
    std::vector<dev_type<real>> host_buf(n);
    for (int i = 0; i < int(host_buf.size()); ++i) {
        real x = (real)i;
        dev_store(&host_buf[i], &x);
    }
    ibuf.store(queue, host_buf.data());
    kernel.set_arg(0, ibuf);
    kernel.set_arg(1, obuf);
    kernel.run(queue, n);
    obuf.load(queue, host_buf.data());
    for(int i = 0; i < int(host_buf.size()); ++i) {
        real x;
        dev_load(&x, &host_buf[i]);
        assert(i*i == (int)x);
    }
}

#endif // TEST_DEV
