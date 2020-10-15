#ifdef TEST_DEV

#include <rtest.hpp>

#include <common/algebra/real.hh>

#include <vector>

#include <test/devtest.hpp>

extern_lazy_static_(devtest::Selector, devtest_selector);

rtest_module_(linking) {
    rtest_(data_to_link) {
        for (devtest::Target target : *devtest_selector) {
            auto queue = target.make_queue();
            auto kernel = devtest::KernelBuilder(target.device_id(), queue)
            .source("data_to_link.cl", std::string(
                "__kernel void data_to_link(__global const int *data, __global ulong *link) {\n"
                "    int i = get_global_id(0);\n"
                "    link[i] = (ulong)&data[i];\n"
                "}\n"
            ))
            .build("data_to_link").unwrap();

            const int n = TEST_ATTEMPTS;
            std::vector<cl_int> data(n);
            std::vector<cl_ulong> link(n);
            for (size_t i = 0; i < n; ++i) {
                data[i] = cl_int(i);
            }
            cl::Buffer dbuf, lbuf;
            lbuf = cl::Buffer::create(*queue, n*sizeof(cl_ulong), true).unwrap();
            dbuf.store(*queue, data.data()).unwrap();

            kernel(*queue, n, dbuf, lbuf).unwrap();
            lbuf.load(*queue, link.data()).unwrap();

            println_("dbuf: {}", dbuf.raw());
            println_("lbuf: {}", lbuf.raw());
            for(size_t i = 0; i < n; ++i) {
                assert_eq_((size_t)dbuf.raw() + i*sizeof(cl_int), link[i]);
            }
            //panic_();
        }
    }
    rtest_(link_to_data) {
        for (devtest::Target target : *devtest_selector) {
            auto queue = target.make_queue();
            auto kernel = devtest::KernelBuilder(target.device_id(), queue)
            .source("link_to_data.cl", std::string(
                "__kernel void link_to_data(__global const ulong *link, __global int *data) {\n"
                "    int i = get_global_id(0);\n"
                "    *(__global int *)link[i] = i;\n"
                "}\n"
            ))
            .build("link_to_data").unwrap();

            const int n = TEST_ATTEMPTS;
            std::vector<cl_int> data(n);
            std::vector<cl_ulong> link(n);
            cl::Buffer dbuf, lbuf;
            dbuf = cl::Buffer::create(*queue, n*sizeof(cl_int), true).unwrap();
            for (size_t i = 0; i < n; ++i) {
                link[i] = (cl_ulong)((size_t)dbuf.raw() + i*sizeof(cl_int));
            }
            lbuf.store(*queue, link.data()).unwrap();

            kernel(*queue, n, lbuf, dbuf).unwrap();
            dbuf.load(*queue, data.data()).unwrap();

            println_("dbuf: {}", dbuf.raw());
            println_("lbuf: {}", lbuf.raw());
            for(size_t i = 0; i < n; ++i) {
                assert_eq_(i, data[i]);
            }
            //panic_();
        }
    }
}

#endif // TEST_DEV
